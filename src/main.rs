use png::HasParameters;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;

fn bin_to_png(pixel: u8) -> [u8; 4] {
    let cell1 = pixel & 0b11;
    let cell2 = (pixel & 0b1100) >> 2;
    let cell3 = (pixel & 0b110000) >> 4;
    let cell4 = (pixel & 0b11000000) >> 6;

    let png: [u8; 4] = [cell4, cell3, cell2, cell1];

    let png = png.map(|x| match x {
        0 => 0,
        1 => 128,
        2 => 255,
        _ => 128,
    });

    png
}

fn main() {
    let f = File::open("betaflight.mcm").unwrap();
    let b = BufReader::new(f);

    let mut pixels: Vec<u8> = Vec::new();
    let mut data: Vec<u8> = Vec::new();

    for l in b.lines().skip(1) {
        pixels.push(u8::from_str_radix(&l.unwrap(), 2).unwrap());
    }

    let range = (0..16384).step_by(64);

    for c in range {
        for i in &pixels[c..c + 54] {
            let buf: [u8; 4] = bin_to_png(*i);

            buf.iter().for_each(|x| data.push(*x));
        }

        let filename = format!("output/{}.png", c / 64);
        let png_file = File::create(&filename).unwrap();
        let w = BufWriter::new(png_file);

        let mut encoder = png::Encoder::new(w, 12, 18);

        encoder
            .set(png::ColorType::Grayscale)
            .set(png::BitDepth::Eight);

        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(&data).unwrap();
        data.clear();
    }
}
