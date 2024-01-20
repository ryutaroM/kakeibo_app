use kaibo_app::services;
use std::io;
const FILE_PATH: &str = "store/data.json";
fn main() {
    let mut service_type = String::new();
    println!("{}", "実行したい内容を入力してください");
    io::stdin().read_line(&mut service_type).unwrap();
    let service_type: u8 = service_type.trim().parse().unwrap();
    services::validate::InputValidater::validate_service_type(service_type);

    if service_type == 0 {
        services::register::run(FILE_PATH)
    } else if service_type == 1 {
        services::summarize::run(FILE_PATH)
    }
}
