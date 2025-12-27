mod interface;
mod server_interface;
mod utility;

use crate::interface::parse_args;
use crate::utility::Config;

fn main() {
    let config = Config::new("config.txt");
    parse_args(&config);
}
