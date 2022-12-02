use same_file::Handle;
use std::fs::File;
use std::io::{Error};
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
mod verilog_analysis;


fn main() -> Result<(), Error> {
    let path_to_read = Path::new("verilog/top.sv");

    Handle::stdout()?;
    Handle::from_path(path_to_read)?;
    let mut module_name : String;
    let mut inputs: HashMap<String, i32> = HashMap::new();
    let mut outputs: HashMap<String, i32> = HashMap::new();
    let mut wires: HashMap<String, i32> = HashMap::new();
    let mut regs: HashMap<String, i32> = HashMap::new();
    let mut cont_assigns: HashMap<String, i32> = HashMap::new();
    let mut always_assigns: HashMap<String, String> = HashMap::new();

    
    let file = File::open(&path_to_read)?;
    let buf_file = BufReader::new(file);
    for line in buf_file.lines() {
        let mut token : Vec<String> = Vec::new();
        let liner = line.unwrap();
        token.push(liner);

        let premodule_name = verilog_analysis::get_module_name(&token[0]);
        if premodule_name != "".to_string() {
            module_name = premodule_name;
        }

        let (input, dimension) = verilog_analysis::get_inputs(&token[0]);
        if input != "".to_string() {
            inputs.insert(input, dimension);
        }

        let (output, dimension) = verilog_analysis::get_outputs(&token[0]);
        if output != "".to_string() {
            outputs.insert(output, dimension);
        }

        let (wire, dimension) = verilog_analysis::get_wires(&token[0]);
        if wire != "".to_string() {
            wires.insert(wire, dimension);
        }
        
        let (reg, dimension) = verilog_analysis::get_regs(&token[0]);
        if reg != "".to_string() {
            regs.insert(reg, dimension);
        }

        let (always_event, always_assign) = verilog_analysis::get_alwayses(&token[0]);
        always_assigns.insert(always_event, always_assign);

        let (pre_signals, pre_lut_cmd) = verilog_analysis::get_assign(&token[0]);
        if pre_signals != ""{
            cont_assigns.insert(pre_signals, pre_lut_cmd);
        }
    }

    Ok(())
}





