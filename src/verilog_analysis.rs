use std::fs::File;
use regex::Regex;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};


pub fn get_module_name(file : File) -> String {
    let buf_file = BufReader::new(file);
    for (num, line) in buf_file.lines().enumerate() {
        let line_r = line.unwrap();
        if line_r.contains("module") {
            let regex = Regex::new(r"\bmodule\s+(\w+)").unwrap();
            let module: Vec<&str> = (regex.find_iter(&line_r).map(|x| x.as_str()).collect());
            return (module[0]).to_string();
        }
    }
    return "!".to_string();
}
pub fn get_inputs(file : File) -> HashMap<String, i32> {
    let input_regex = Regex::new(r"(input( reg|wire)?\s+(\[[0-9]:[0-9]?\])?\s*(\w+),?)").unwrap();
    let mut inputs: HashMap<String, i32> = HashMap::new();
    let buf_file = BufReader::new(file);
    
    for (num, line) in buf_file.lines().enumerate() {
        let line_r = line.unwrap();
        if input_regex.is_match(&line_r.to_string()) {
            let io_name: Vec<&str> = Regex::new(r"([a-z]+,?)$").unwrap().find_iter(&line_r).map(|x| x.as_str()).collect();

            if Regex::new(r"(\[[0-9]:[0-9]?\])").unwrap().is_match(&line_r.to_string()) {
                let dimension: Vec<&str> = Regex::new(r"\d").unwrap().find_iter(&line_r).map(|x| x.as_str()).collect();
                inputs.insert(
                    io_name[0].to_string().replace(",", ""),
                    dimension[0].parse::<i32>().unwrap() + 1
                );
            }
            else {
                inputs.insert(
                    io_name[0].to_string().replace(",", ""),
                    1
                );
            }
        }
    }
    return inputs;
}
