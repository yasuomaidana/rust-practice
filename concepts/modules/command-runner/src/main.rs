use std::process::Command;
fn main() {
    let output = Command::new("ls")
        .arg("-l")
        //.args(&["-a", "-h"])
        .output()
        .expect("failed to execute process");
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}
