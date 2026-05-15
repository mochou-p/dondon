// mochou-p/dondon/src/main.rs

mod init;
mod utils;


fn main() {
    let Some((host, device, config)) = init::setup() else { return; };
}

