use same_file::Handle;
use std::fs::File;
use std::io::{Error};
use std::path::Path;
mod verilog_analysis;


fn main() -> Result<(), Error> {
    let path_to_read = Path::new("verilog/top.sv");

    Handle::stdout()?;
    Handle::from_path(path_to_read)?;

    let module_name = verilog_analysis::get_module_name(File::open(&path_to_read)?);
    let inputs = verilog_analysis::get_inputs(File::open(&path_to_read)?);
    for pair in inputs.into_iter(){
        println!("{pair:?}");
    }
    println!("{}", module_name);
    

    Ok(())
}





