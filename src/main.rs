fn usage() {
    println!("Da first");
    println!("Version {}", get_version());
}

fn get_version() -> u16 {
    1000
}

fn main() {
    usage();
}
