use clap::{Arg, Command};

fn main() {
    let matches = Command::new("lsblk")
        .about("A tool to list information about block devices")
        .arg(
            Arg::new("device")
                .about("Specifies the device")
                .required(false)
                .takes_value(true),
        )
        .get_matches();

    println!("{:?}", matches);
}
