use std::io::{self, stdin, BufReader, Read};
use std::path::Path;

use clap::Parser;
use colorsys::Rgb;
use image::RgbaImage;

#[derive(Parser)]
struct Args {
    #[arg(short = 'i')]
    /// Image to read. If not provided, Clorox will try to read from stdin
    image: Option<String>,
    #[arg(short = 'f')]
    /// Choose color. Can be `rgb`, `hsl`, or `hex`. Defaults to `hex`
    format: Option<String>,
    /// Location of the pixel you want to get the color from.
    location: String,
}

fn read(path: &Path) -> Result<RgbaImage, String> {
    if let Some("-") = path.to_str() {
        let mut reader = BufReader::new(stdin());
        let mut buffer = Vec::new();

        if let Err(e) = reader.read_to_end(&mut buffer) {
            return Err(format!("{e}"));
        }

        return match image::load_from_memory(&buffer) {
            Ok(img) => Ok(img.into_rgba8()),
            Err(e) => return Err(format!("{e}")),
        };
    }

    let img_buf = match image::io::Reader::open(path) {
        Ok(img) => img,
        Err(e) => return Err(format!("{e}")),
    };

    match img_buf.decode() {
        Ok(img) => Ok(img.into_rgba8()),
        Err(e) => Err(format!("{e}")),
    }
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let loc: Vec<u32> = args
        .location
        .split(&['x', ',', 'X'][..])
        .into_iter()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut img_path: String = "-".to_string();

    if args.image != None {
        img_path = args.image.unwrap();
    }

    let file = read(Path::new(&img_path)).expect("Not found");
    let raw_pixel = file.get_pixel(loc[0], loc[1]);

    let parsed_pixel = Rgb::from(&(
        i16::from(raw_pixel[0]),
        i16::from(raw_pixel[1]),
        i16::from(raw_pixel[2]),
    ));

    match args.format.unwrap_or_default().as_str() {
        "css" | "rgb" => println!("{}", parsed_pixel.to_css_string()),
        "hex" | _ => println!("{}", parsed_pixel.to_hex_string()),
    }

    Ok(())
}
