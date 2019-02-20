extern crate image;

use std::collections::HashMap;
use std::process::Command;
use std::path::PathBuf;
use std::fs;

fn run_tesseract() {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "tesseract"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("tesseract")
            .output()
            .expect("failed to execute process")
    };
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(output.status.success());
}

pub fn run_tesseract_get_result(image: image::DynamicImage, lang: &str, config: &str, nice: &str) {
    //first thing need to save the image
    //this now needs to be passed to tesseract.
    let mut arg_map = HashMap::new();

    arg_map.insert("lang", lang);
    arg_map.insert("config", config);
    arg_map.insert("nice", nice);
    run_tesseract_with_args(image, &arg_map);
}

fn run_tesseract_with_args(image: image::DynamicImage, args: &HashMap<&str, &str>) {
    image.save("temp.png").expect("Unable to save image");
    //let srcdir = PathBuf::from("temp.png");
    //let path = fs::canonicalize(&srcdir).expect("unable to find path");
    let path = PathBuf::from("temp.png");
    //println!("{:?}", path);
    let mut arg_string = "".to_owned();
    arg_string.push_str(path.to_str().expect("Invaid Path")); //inputfilename
    arg_string.push_str(" "); 
    arg_string.push_str("temp.png_out");// output filename
    arg_string.push_str(" "); 
    arg_string.push_str("-l");
    arg_string.push_str(" "); 
    arg_string.push_str(args.get("lang").expect("Unable to find arg")); //lang
    arg_string.push_str(" "); 
    arg_string.push_str(args.get("config").expect("Unable to find arg")); //config
    //println!("{:?}", arg_string);
    // 'input_filename': input_filename,
    // 'output_filename_base': temp_name + '_out',

    // 'config': config,
    // 'nice': nice

    //number of args in array matter
    //will have to clean up the output file in the end
    //inputs in order
    //image, extention, lang, config, nice, return_bytes

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", ("tesseract ".to_owned() + &arg_string).as_str()])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(("tesseract ".to_owned() + &arg_string).as_str())
            .output()
            .expect("failed to execute process")
    };
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(output.status.success());

    let contents = fs::read_to_string("temp.png_out.txt").expect("unable to read file");
    println!("Output: {}", contents);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_tessract() {
        run_tesseract();
    }

    #[test]
    fn test_run_tesseract_with_args() {
        let img = image::open("C:\\Users\\blaze\\Source\\Rust\\projects\\tesseract_wrapper\\test.png").expect("unable to open image");
        run_tesseract_get_result(img, "eng", "", "");
    }
}
