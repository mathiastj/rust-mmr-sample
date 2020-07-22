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

    // let bounds = parse_pair(&args[2], 'x')
    //     .expect("error parsing image dimensions");
    // let upper_left = parse_complex(&args[3])
    //     .expect("error parsing upper left corner point");
    // let lower_right = parse_complex(&args[4])
    //     .expect("error parsing lower right corner point");

    // let mut pixels = vec![0; bounds.0 * bounds.1];

    // // Scope of slicing up `pixels` into horizontal bands.
    // {
    //     let bands: Vec<(usize, &mut [u8])> = pixels
    //         .chunks_mut(bounds.0)
    //         .enumerate()
    //         .collect();

    //     bands.into_par_iter()
    //         .weight_max()
    //         .for_each(|(i, band)| {
    //             let top = i;
    //             let band_bounds = (bounds.0, 1);
    //             let band_upper_left = pixel_to_point(bounds, (0, top),
    //                                                  upper_left, lower_right);
    //             let band_lower_right = pixel_to_point(bounds, (bounds.0, top + 1),
    //                                                   upper_left, lower_right);
    //             render(band, band_bounds, band_upper_left, band_lower_right);
    //         });
    // }

    // write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
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

use std::fs;

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