use std::io;
use std::io::Write;
use std::path::Path;
use std::ffi::OsStr;

use native_dialog::FileDialog;

use image::io::Reader;
use image::{imageops, RgbaImage};

use imageproc::noise;

// Taking input
fn take_input(query: String) -> String {
    print!("{}", query);
    io::stdout().flush().unwrap();

    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("Failed");
    inp
}

fn enter_to_close() {
    print!("Press enter to close ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut String::new()).expect("Failed");
}

fn main() {
    println!("Choose an Image file: ");

    // File dialog for choosing a file
    let path = FileDialog::new()
        .set_location("~/Desktop")
        .add_filter("PNG", &["png"])
        .add_filter("JPEG", &["jpg", "jpeg"])
        .show_open_single_file()
        .unwrap();

    let path = match path {
        Some(path) => path,
        None => return,
    };

    println!("Opening image, please wait.");
    
    let mut img = Reader::open(&path).unwrap().decode().unwrap();
    let file_name = Path::new(&path).file_name().and_then(OsStr::to_str).unwrap().split(".").collect::<Vec<&str>>()[0];
    let file_extension = Path::new(&path).extension().and_then(OsStr::to_str).unwrap();
   
    let choice = take_input(String::from("What would you like to do with the file?: "));
    
    // Perform things here
    let result;

    match choice.as_str().trim_end() {
        "Resize" => {
            let width = take_input(String::from("Enter the width of the resize: ")).as_str().trim_end().parse::<u32>().unwrap();
            let height = take_input(String::from("Enter the height of the resize: ")).as_str().trim_end().parse::<u32>().unwrap();

            println!("Performing changes, please wait this might take a while.");
            
            result = imageops::resize(&img, width, height, imageops::FilterType::CatmullRom);
        },
        "Blur" => {
            let intensity = take_input(String::from("Enter the intensity to blur the image, example 5: "))
                .as_str().trim_end()
                .parse::<f32>().unwrap();
            
            println!("Performing changes, please wait this might take a while.");
            
            result = imageops::blur(&img, intensity);
        }
        "Flip" => {
            let pos = take_input(String::from("Would you like to flip the image horizontally or verically? Enter `H` or `V`: "));

            println!("Performing changes, please wait this might take a while.");
            
            match pos.as_str().trim() {
                "H" => { result = imageops::flip_horizontal(&img) },
                "V" => { result = imageops::flip_vertical(&img) },
                _ =>  {
                        println!("Invalid option, enter `H` or `V`");
                        enter_to_close();
                        return
                }
            }
        }
        "Tile" => {
            let mut idk = RgbaImage::new(750, 750);
            let resized_img = imageops::resize(&img, 150, 150, imageops::FilterType::CatmullRom);

            println!("Performing changes, please wait this might take a while.");

            imageops::tile(&mut idk, &resized_img);

            result = idk;
        }
        "Unsharpen" => {
            let intensity = take_input(String::from("Enter the intensity example 10: ")).as_str().trim_end().parse::<f32>().unwrap();
            let threshold = take_input(String::from("Enter the threshold example 15: ")).as_str().trim_end().parse::<i32>().unwrap();

            println!("Performing changes, please wait this might take a while.");
            
            result = imageops::unsharpen(&img, intensity, threshold);  
        }
        "Huerotate" => {
            let degrees = take_input(String::from("Enter the degress to rotate: ")).as_str().trim_end().parse::<i32>().unwrap();

            println!("Performing changes, please wait this might take a while.");

            result = imageops::huerotate(&img, degrees);
        }
        "Contrast" => {
            let value = take_input(String::from("Enter the contrast value: ")).as_str().trim_end().parse::<f32>().unwrap();

            println!("Performing changes, please wait this might take a while.");

            result = imageops::contrast(&img, value);
        }
        "Brighten" => {
            let value = take_input(String::from("Enter the brightness intensity: ")).as_str().trim_end().parse::<i32>().unwrap();

            println!("Performing changes, please wait this might take a while.");

            result = imageops::brighten(&img, value);
        }
        "Grayscale" => {
            println!("Performing changes, please wait this might take a while.");
            
            let gray_scaled = imageops::grayscale(&img);

            gray_scaled.save(format!("edited_{}.{}", file_name, file_extension)).unwrap();
            println!("Changes saved! Check the root directory of the executable.");
            enter_to_close();
            return
        }
        "InvertPixels" => {
            println!("Performing changes, please wait this might take a while.");

            imageops::invert(&mut img);
            img.save(format!("edited_{}.{}", file_name, file_extension)).unwrap();
            
            println!("Changes saved! Check the root directory of the executable.");
            enter_to_close();
            return
        }
        "Noise" => {
            let mean = take_input(String::from("Enter the mean, example 50: ")).as_str().trim_end().parse::<f64>().unwrap();
            let deviation = take_input(String::from("Enter the deviation, example 50: ")).as_str().trim_end().parse::<f64>().unwrap();
            let seed = take_input(String::from("Enter the seed, example 100: ")).as_str().trim_end().parse::<u64>().unwrap();

            println!("Performing changes, please wait this might take a while.");
            result = noise::gaussian_noise(img.as_rgba8().unwrap(), mean, deviation, seed);
        }
        _ => { println!("Invalid option!");  enter_to_close(); return  }
    };
    
    // I was actually using the save_with_format method instead of save but that didn't seem to work, so I'm just hardcoding the file extension.
    result.save(format!("edited_{}.{}", file_name, file_extension)).unwrap();
    println!("Changes saved!");
    enter_to_close();
}