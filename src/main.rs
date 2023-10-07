#![feature(iter_next_chunk)]

fn usage(bin: &str) {
    const BIN: &str = env!("CARGO_BIN_NAME");
    const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    eprintln!("Please specify an input and output path.");
    eprintln!("Usage: {bin} <input> <output>");
    eprintln!();
    eprintln!("{BIN} {VERSION} by {AUTHORS}, 2023.");
}

fn main() {
    let mut args = std::env::args();
    let bin = args.next().unwrap_or("dith".to_string());
    let Ok([input, output]) = args.next_chunk() else {
        usage(&bin);
        std::process::exit(1);
    };
    let img = image::open(input).unwrap().to_luma32f();
    let width = img.width() as usize;
    let height = img.height() as usize;
    let mut pixels: Vec<_> = img.pixels().map(|&image::Luma([v])| v).collect();

    for y in 0..height {
        for x in 0..width {
            let old = pixels[y * width + x];
            let new = if old > 0.5 { 1.0 } else { 0.0 };
            pixels[y * width + x] = new;
            let error = old - new;

            // [        *    7/16 ]
            // [ 3/16  5/16  1/16 ]
            const NEIGHBOURS: [(isize, usize, f32); 4] = [
                (1, 0, 7.0 / 16.0),
                (-1, 1, 3.0 / 16.0),
                (0, 1, 5.0 / 16.0),
                (1, 1, 1.0 / 16.0),
            ];
            for (dx, dy, debt) in NEIGHBOURS {
                let x = x.saturating_add_signed(dx);
                let y = y + dy;
                if x >= width || y >= height {
                    continue;
                }
                let px = pixels[y * width + x];
                pixels[y * width + x] = px + error * debt;
            }
        }
    }

    let pixels: Vec<_> = pixels.iter().map(|v| (v * 255.0) as u8).collect();
    let img = image::GrayImage::from_vec(width as u32, height as u32, pixels).unwrap();
    img.save(output).unwrap();
}
