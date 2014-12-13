extern crate xdg;

pub fn config_dir(name: &str) -> Path {
    let mut path: Path = xdg::get_config_home();
    path.push(name);
    path
}
