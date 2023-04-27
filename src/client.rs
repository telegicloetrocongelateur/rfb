use crate::error;
use crate::io::*;
use crate::messages::*;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::net::SocketAddr;
use std::net::TcpStream;
use std::time::Duration;
pub struct Client {
    pixel_format: PixelFormat,
    version: Version,
}

impl Client {
    pub fn new(addr: &SocketAddr, timeout: Duration) -> Result<Self, crate::error::Error> {
        let stream = TcpStream::connect_timeout(addr, timeout)?;
        let mut reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);
        println!("Connected");
        let version = Version::decode_from(&mut reader)?;
        version.clone().encode_to(&mut writer)?;
        writer.flush()?;
        let security_type = SecurityType::None;

        let server_security_types = match version {
            Version::Rfb33 => {
                let server_security_type = SecurityType::decode_from(&mut reader)?;
                vec![server_security_type]
            }

            _ => Vec::<SecurityType>::decode_from(&mut reader)?,
        };

        //TODO: List of securities for client, get error message if version > 3.3
        if !server_security_types.contains(&security_type) {
            return Err(error::Error::IncompatibleSecurity);
        }
        security_type.encode_to(&mut writer)?;
        writer.flush()?;

        let security_result = SecurityResult::decode_from(&mut reader)?;
        if security_result != SecurityResult::Ok {
            if version == Version::Rfb38 {
                let reason = String::decode_from(&mut reader)?;
                //TODO: Handle reason
            }
            return Err(error::Error::HandshakeFailed);
        }

        ClientInit { shared: false }.encode_to(&mut writer)?;
        writer.flush()?;

        let server_init = ServerInit::decode_from(&mut reader)?;
        /*
            SetEncodings {
                encodings: vec![EncodingType::Raw],
            }
            .encode_to(&mut writer)?;
        writer.flush()?;
        */

        FramebufferUpdateRequest {
            incremental: false,
            x: 0,
            y: 0,
            width: 10,
            height: 10,
        }
        .encode_to(&mut writer)?;
        writer.flush()?;

        println!("Bonjour1");
        let framebuffer = FramebufferUpdate::decode_from(&mut reader)?;
        FramebufferUpdateRequest {
            incremental: true,
            x: 0,
            y: 0,
            width: 10,
            height: 10,
        }
        .encode_to(&mut writer)?;
        writer.flush()?;
        let framebuffer = FramebufferUpdate::decode_from(&mut reader)?;

        Ok(Self {
            pixel_format: server_init.pixel_format,
            version,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, SocketAddrV4};

    #[test]
    fn client() {
        use crate::client::Client;
        println!("Bonjour");
        Client::new(
            &SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(192, 168, 0, 14), 5900)),
            Duration::from_millis(1000),
        )
        .unwrap();

        panic!("Bonjour")
    }
}
