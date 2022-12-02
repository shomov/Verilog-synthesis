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
            let module: Vec<&str> = regex.find_iter(&line_r).map(|x| x.as_str()).collect();
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

pub fn get_wires(file : File) -> HashMap<String, i32> {
    let wire_regex = Regex::new(r"^(wire?\s+(\[[0-9]:[0-9]?\])?\s*(\w+),?)+;").unwrap();
    let mut wires: HashMap<String, i32> = HashMap::new();
    let buf_file = BufReader::new(file);
    
    for (num, line) in buf_file.lines().enumerate() {
        let line_r = line.unwrap();
        if wire_regex.is_match(&line_r.to_string()) {
            let wire_name: Vec<&str> = Regex::new(r"([a-z]+,?;?)$").unwrap().find_iter(&line_r).map(|x| x.as_str()).collect();

            if Regex::new(r"(\[[0-9]:[0-9]?\])").unwrap().is_match(&line_r.to_string()) {
                let dimension: Vec<&str> = Regex::new(r"\d").unwrap().find_iter(&line_r).map(|x| x.as_str()).collect();
                wires.insert(
                    wire_name[0].to_string().replace(",", ""),
                    dimension[0].parse::<i32>().unwrap() + 1
                );
            }
            else {
                wires.insert(
                    wire_name[0].to_string().replace(",", ""),
                    1
                );
            }
        }
    }
    return wires;
}

pub fn get_regs(file : File) -> HashMap<String, i32> {
    let reg_regex = Regex::new(r"^(reg?\s+(\[[0-9]:[0-9]?\])?\s*(\w+),?)+;").unwrap();
    let mut regs: HashMap<String, i32> = HashMap::new();
    let buf_file = BufReader::new(file);
    
    for (num, line) in buf_file.lines().enumerate() {
        let line_r = line.unwrap();
        if reg_regex.is_match(&line_r.to_string()) {
            let reg_name: Vec<&str> = Regex::new(r"([a-z]+,?;?)$").unwrap().find_iter(&line_r).map(|x| x.as_str()).collect();

            if Regex::new(r"(\[[0-9]:[0-9]?\])").unwrap().is_match(&line_r.to_string()) {
                let dimension: Vec<&str> = Regex::new(r"\d").unwrap().find_iter(&line_r).map(|x| x.as_str()).collect();
                regs.insert(
                    reg_name[0].to_string().replace(",", ""),
                    dimension[0].parse::<i32>().unwrap() + 1
                );
            }
            else {
                regs.insert(
                    reg_name[0].to_string().replace(",", ""),
                    1
                );
            }
        }
    }
    return regs;
}

pub fn get_assign(file : File) -> HashMap<String, i32> {
    let assign_regex = Regex::new(r"^\s+(assign ((\w+ = \w+\s+[&|+-]+\s+\w+)|(\w+ [&|+-]+= \w+));\s?)").unwrap();
    let mut cont_assignments: HashMap<String, i32> = HashMap::new();
    let buf_file = BufReader::new(file);
    
    for (num, line) in buf_file.lines().enumerate() {
        let line_r = line.unwrap();
        if assign_regex.is_match(&line_r.to_string()) {
            let signals: Vec<&str> = Regex::new(r"\w+ = \w+\s+[&|+-]\s+\w+").unwrap().find_iter(&line_r).map(|x| x.as_str()).collect();
            let operation: Vec<&str> = Regex::new(r"[&|+-]+").unwrap().find_iter(&line_r).map(|x| x.as_str()).collect();
            let lut_cmd: i32 = match operation[0] {
                "&&" => 6,
                "||" => 5,
                "&" => 4,
                "|" => 3,
                "+" => 2,
                "-" => 1,
                _   => 0
            };
            cont_assignments.insert(
                signals[0].to_string(),
                lut_cmd
            );
        }
    }
    return cont_assignments;
}

pub fn get_alwayses(file : File) -> HashMap<String, i32> {
    let always_regex = Regex::new(r"^\s*(always(_ff)? @\((pos|neg)edge \w+\) begin\s*)").unwrap();
    let mut alw_assignments: HashMap<String, i32> = HashMap::new();
    let buf_file = BufReader::new(file);
    
    let mut always_detect:bool = false;

    for (num, line) in buf_file.lines().enumerate() {
        let line_r = line.unwrap();
        if always_regex.is_match(&line_r.to_string()) {
            always_detect = true;
        }
        else if always_detect && Regex::new(r"^\s*end\s*").unwrap().is_match(&line_r.to_string()){
            always_detect = false;
        }
        else if always_detect{
            if Regex::new(r"^\s*((\w+ <?= \w+\s+[&|+-]+\s+\w+;\s?)|(\w+ [&|+-]+= \w+))").unwrap().is_match(&line_r.to_string()) {
                let signals: Vec<&str> = Regex::new(r"\w+ <?= \w+\s+[&|+-]\s+\w+").unwrap().find_iter(&line_r).map(|x| x.as_str()).collect();
                let operation: Vec<&str> = Regex::new(r"[&|+-]+").unwrap().find_iter(&line_r).map(|x| x.as_str()).collect();
                let lut_cmd: i32 = match operation[0] {
                    "&&" => 6,
                    "||" => 5,
                    "&" => 4,
                    "|" => 3,
                    "+" => 2,
                    "-" => 1,
                    _   => 0
                };
                alw_assignments.insert(
                    signals[0].to_string(),
                    lut_cmd
                );
            }
        }
    }
    return alw_assignments;
}
