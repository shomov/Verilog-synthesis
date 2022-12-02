use std::io::Error;
use std::path::Path;
mod verilog_analysis;


fn main() -> Result<(), Error> {
    // let path_to_read = Path::new("verilog/top.sv");
    let res = verilog_analysis::verilog_analysis(Path::new("verilog/top.sv"));
    

    Ok(())
}





