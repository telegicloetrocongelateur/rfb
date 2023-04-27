#[cfg(test)]
mod tests {
    use std::{
        io::{BufRead, BufReader, BufWriter, Write},
        net::TcpListener,
    };

    use crate::{
        io::{DecodeFrom, Encode, EncodeTo},
        messages::{
            ClientInit, EncodingType, FramebufferUpdate, FramebufferUpdateRequest, PixelFormat,
            Rectangle, SecurityResult, SecurityType, ServerInit, SetEncodings, Version,
        },
    };

    #[test]
    fn server() -> Result<(), crate::Error> {
        let (stream, addr) = TcpListener::bind("0.0.0.0:5900")?.accept()?;

        let writer = &mut BufWriter::new(&stream);
        let reader = &mut BufReader::new(&stream);

        Version::Rfb38.encode_to(writer)?;
        writer.flush()?;
        assert_eq!(Version::Rfb38, Version::decode_from(reader)?);
        vec![SecurityType::None].encode_to(writer)?;
        writer.flush()?;
        assert_eq!(SecurityType::None, SecurityType::decode_from(reader)?);
        SecurityResult::Ok.encode_to(writer)?;
        writer.flush();
        ClientInit::decode_from(reader)?;
        ServerInit {
            pixel_format: PixelFormat {
                bits_per_pixel: 32,
                depth: 24,
                big_endian_flag: false,
                true_colour_flag: true,
                red_max: 255,
                green_max: 255,
                blue_max: 255,
                red_shift: 16,
                green_shift: 8,
                blue_shift: 0,
            },
            framebuffer_width: 500,
            framebuffer_height: 500,
            name: "BONOJOUR".to_string(),
        }
        .encode_to(writer)?;
        writer.flush();
        SetEncodings::decode_from(reader)?;

        FramebufferUpdateRequest::decode_from(reader)?;
        println!("{:?}", writer.buffer());
        for i in (0..1000).step_by(10) {
            std::thread::sleep_ms(1);
            FramebufferUpdate {
                rectangles: vec![Rectangle {
                    x: 0,
                    y: 0,
                    width: 500,
                    height: 500,
                    encoding_type: EncodingType::Raw,
                    pixels: (0..500 * 500 * 4).map(|x| (x + i % 256) as u8).collect(),
                }],
            }
            .encode_to(writer)?;
            writer.flush()?;
        }
        panic!("BONOJOIUJOPIUOPIHJIOUHOIUH");

        Ok(())
    }
}
