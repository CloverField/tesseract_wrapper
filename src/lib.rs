use std::process::Command;

fn run_process_windows() {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
        .args(&["/C", "echo hello"])
        .output()
        .expect("failed to execute process")
    } else {
        Command::new("sh")
        .arg("-c")
        .arg("echo hello")
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
    fn test_shell() {
        run_process_windows();
    }
}
