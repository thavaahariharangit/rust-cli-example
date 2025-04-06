use clap::{Arg, Command};

fn main() {
    let matches = Command::new("lsblk")
        .about("A tool to list information about block devices")
        .arg(
            Arg::new("device")
                .help("Specifies the device") // Updated method
                .required(false), // Optional argument
        )
        .get_matches();

    println!("{:?}", matches);
}
