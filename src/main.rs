use std::io::Error;
use std::path::Path;
mod verilog_analysis;
mod verilog_synthesis;
use string_builder::Builder;

use crate::verilog_analysis::AnalysisData;

fn main() -> Result<(), Error> {
    // let path_to_read = Path::new("verilog/top.sv");
    let date : AnalysisData;
    date = verilog_analysis::verilog_analysis(Path::new("verilog/top.sv"));
    verilog_synthesis::synthesis(date);

    
    let mut out_file = Builder::default();
    // print!("{}", out_file.string());

    // out_file.append("abc");
    // out_file.append("def");
    // print!("{}", out_file.string().unwrap());

    Ok(())
}





