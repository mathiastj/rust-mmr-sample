use std::fs;
use std::io::Write;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        writeln!(std::io::stderr(),
                 "Usage: rust-mmr-sample INPUT_FILE OUTPUT_FILE")
            .unwrap();
        writeln!(std::io::stderr(),
                 "Example: {} input.csv output.png",
                 args[0])
            .unwrap();
        std::process::exit(1);
    }

    let file_contents = read_file(&args[1]).expect("Failed to read file");

    let mmr_history = parse_mmr(file_contents);

    assert!(mmr_history.len() != 0);

    let padding = 0;
    let max = mmr_history.iter().max();
    let upper_y = match max {
        None => {
            println!("No max");
            std::process::exit(1);
        },
        Some(v) => v + padding
    };
    let min = mmr_history.iter().min();
    let lower_y = match min {
        None => {
            println!("No min");
            std::process::exit(1);
        },
        Some(v) => v - padding
    };

    let bounds = (mmr_history.len(), (upper_y-lower_y+1) as usize);
    println!("{:?}", bounds);

    let mut pixels = vec![255; bounds.0 as usize * bounds.1];

    render(&mut *pixels, bounds, lower_y, mmr_history);

    write_image(&args[2], &pixels, bounds).expect("error writing PNG file");
}

use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;

/// Write the buffer `pixels`, whose dimensions are given by `bounds`, to the
/// file named `filename`.
fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize))
    -> Result<(), std::io::Error>
{
    let output = File::create(filename)?;

    let encoder = PNGEncoder::new(output);
    encoder.encode(&pixels,
                   bounds.0 as u32, bounds.1 as u32,
                   ColorType::Gray(8))?;

    Ok(())
}

/// Read a file as a string
fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let contents = fs::read_to_string(filename);

    match contents {
        Ok(v) => {println!("{}",v); return Ok(v);},
        Err(e) => {println!("error reading file: {}", e); return Err(e);},
    }
}

fn parse_mmr(contents: String) -> Vec<isize> {
    let games = contents.lines();

    let mut mmr_history = Vec::new();
    for (i, game) in games.enumerate() {
        if i == 0 {
            // Skip the header in the file
            continue;
        }
        let data_points = game.split(",");
        let first_four_entries = data_points.take(4).collect::<Vec<_>>();
        println!("{:?}", first_four_entries);
        let mmr_entry = first_four_entries[3].replace("\"","").parse();
        match mmr_entry {
            Ok(v) => mmr_history.push(v),
            Err(e) => eprintln!("Could not parse mmr string to integer: {}", e),
        }
    };
    println!("{:?}",mmr_history);
    mmr_history
}

fn render(pixels: &mut [u8], bounds: (usize, usize), lower_y: isize, mmr_history: Vec<isize>) {
    for (i, mmr_entry) in mmr_history.iter().enumerate() {
        let mmr_to_pixel_bounds = mmr_entry - lower_y;
        for n in 0..=mmr_to_pixel_bounds {
            let mmr_in_pixel = bounds.0 * (bounds.1 - (n+1) as usize) + i;
            println!("n: {}, pixel: {}, bounds: {}", n, mmr_in_pixel, mmr_to_pixel_bounds);
            pixels[mmr_in_pixel] = 0;
        }
    }
}