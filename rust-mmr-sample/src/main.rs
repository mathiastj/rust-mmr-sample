use std::fs;
use std::io::Write;

/// Takes a csv file with mmr on the fourth column and transforms it into a greyscale image
/// The image is as wide as the number of matches in the input file and the height is the mmr relative to each other
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

    // Read mmr from csv file
    let file_contents = read_file(&args[1]).expect("Failed to read file");
    let mmr_history = parse_mmr(file_contents);

    assert!(mmr_history.len() != 0);

    // Determine max and min mmr which will be top and bottom of the image
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

    // Make bounds for an image which is as wide as the number of matches and where height is the difference in max and min (+1) and optional padding
    let bounds = (mmr_history.len(), (upper_y-lower_y+1) as usize);
    println!("{:?}", bounds);

    // Create a vector which will hold the data for the image, initially make all the pixels white
    let mut pixels = vec![255; bounds.0 as usize * bounds.1];

    // Render the image
    render(&mut *pixels, bounds, lower_y, mmr_history);

    // Write the image to file
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

/// Read a file at location `filename` as a string
fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let contents = fs::read_to_string(filename);

    match contents {
        Ok(v) => {println!("{}",v); return Ok(v);},
        Err(e) => {println!("error reading file: {}", e); return Err(e);},
    }
}

/// Hackishly parse a stringified csv file `contents` containing mmr in the fourth column and return a vector with mmr history
fn parse_mmr(contents: String) -> Vec<isize> {
    let games = contents.lines();

    let mut mmr_history = Vec::new();
    for (i, game) in games.enumerate() {
        if i == 0 {
            // Skip the header of the csv file
            continue;
        }
        let data_points = game.split(",");
        let first_four_entries = data_points.take(4).collect::<Vec<_>>();
        println!("{:?}", first_four_entries);
        // Remove string quotations and parse the rest as a number
        let mmr_entry = first_four_entries[3].replace("\"","").parse();
        match mmr_entry {
            Ok(v) => mmr_history.push(v),
            Err(e) => eprintln!("Could not parse mmr string to integer: {}", e),
        }
    };
    println!("{:?}",mmr_history);
    mmr_history
}

/// Make a basic bar chart in `pixels` based on the data in `mmr_history`, `bounds` is the (x,y) dimensions of the bar chart, `lower_y` is the minimum value on the y-axis
/// The bar chart will be black bars based on the mmr on a white background
fn render(pixels: &mut [u8], bounds: (usize, usize), lower_y: isize, mmr_history: Vec<isize>) {
    for (i, mmr_entry) in mmr_history.iter().enumerate() {
        // Calculate how much higher than minimum the entry is
        let mmr_to_pixel_bounds = mmr_entry - lower_y;
        // Color each pixel in the bar black in the column
        for n in 0..=mmr_to_pixel_bounds {
            // Find the pixel in the column
            let mmr_in_pixel = bounds.0 * (bounds.1 - (n+1) as usize) + i;
            println!("n: {}, pixel: {}, bounds: {}", n, mmr_in_pixel, mmr_to_pixel_bounds);
            // Make the pixel black
            pixels[mmr_in_pixel] = 0;
        }
    }
}