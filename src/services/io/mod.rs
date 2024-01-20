use crate::models;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn read_data_or_create_new_data(file_path: &str) -> Vec<models::Item> {
    let f = File::open(file_path);
    match f {
        Ok(s) => {
            let buf_reader = BufReader::new(s);
            serde_json::from_reader(buf_reader).expect("deserialize faild")
        }
        Err(_) => {
            println!("{}", "cretae new file");
            Vec::new()
        }
    }
}

pub fn read_data_or_panic(file_path: &str) -> Vec<models::Item> {
    let file = File::open(file_path).expect("faild open file");
    let buf_reader = BufReader::new(file);
    let data: Vec<_> = serde_json::from_reader(buf_reader).expect("deserialize faild");

    if data.len() == 0 {
        panic!("やああ")
    }
    data
}

pub fn write_to_json(data: &Vec<models::Item>, file_path: &str) {
    let json_data = serde_json::to_string_pretty(data).expect("deserialize failed");
    let mut file = File::create(file_path).expect("failed write file");
    writeln!(file, "{}", json_data).expect("failed write");
    println!("{}", "書き込みおわた");
}
