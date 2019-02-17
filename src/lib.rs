use std::process::Command;

fn run_tesseract(){
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

fn run_tesseract_with_args(args: &[&str]){
    let mut arg_string = "".to_owned();
    for s in args {
        arg_string.push_str(s);
    }

    //number of args in array matter
    //will have to clean up the output file in the end
    //inputs in order
    //image, extention, lang, config, nice, return_bytes

    //lets check the length of the array first
    if args.len() == 6 {
        // means we have everything
    } else if args.len() == 5 {

    } else if args.len() == 4 {

    } else if args.len() == 3 {
        
    }

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
        let arg_array = ["--list-langs"];
        run_tesseract_with_args(&arg_array);
    }
}
