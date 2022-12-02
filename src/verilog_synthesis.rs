use chrono::{Timelike, Utc};
use string_builder::Builder;
use std::collections::HashMap;
use crate::verilog_analysis::AnalysisData;

pub fn synthesis(data : AnalysisData) {
    let mut out_file = Builder::default();
    out_file = set_header(out_file);
    out_file = set_module_name(out_file, data.module_name);
    out_file = set_io(out_file, data.inputs, data.outputs);





    print!("{}", out_file.string().unwrap());
}

fn set_header(mut out_file : Builder) -> Builder {
    let now = Utc::now();
    out_file.append(    
        "Garipova & Shomov Verilog HDL synthesis tool\n\r"
    );
    out_file.append(    
        format!(
            "Synthesis Time UTC {:02}:{:02}:{:02}\n\r",
            now.hour(),
            now.minute(),
            now.second()
        )
    );
    return out_file;
}

fn set_module_name(mut out_file : Builder, module_name : Vec<String>) -> Builder {
    out_file.append(
        "`timescale 1 ps / 1 ps\n\r\n\r"
    );
    out_file.append(
        "(* STRUCTURAL_NETLIST = \"yes\" *)\n\r"
    );
    out_file.append(
        format!(
            "module {}", module_name[0]
        )
    );
    return out_file;
}

fn set_io(mut out_file : Builder, inputs : HashMap<String, i32>, outputs : HashMap<String, i32>) -> Builder {
    out_file.append("(\n\r");
    for (port, _) in &inputs {
        out_file.append(
            format!("{},\n\r", port)
        );
    }
    let mut i = 0;
    for (port, _) in &outputs {
        if i != outputs.len()-1 {
            out_file.append(
                format!("{},\n\r", port)
            );
        }
        else {
            out_file.append(
                format!("{}\n\r);\n\r", port)
            );
        }
        i += 1;
    }
    for (port, dimension) in &inputs {
        if *dimension == 1{
            out_file.append(
                format!("input {};\n\r", port)
            );
        }
        else {
            out_file.append(
                format!("input [{dim}:0] {port};\n\r", dim = *dimension-1, port = port)
            );
        }  
    }
    for (port, dimension) in &outputs {
        if *dimension == 1{
            out_file.append(
                format!("output {};\n\r", port)
            );
        }
        else {
            out_file.append(
                format!("output [{dim}:0] {port};\n\r", dim = *dimension-1, port = port)
            );
        }
    }

    out_file.append("VCC VCC\n\r\t(.P(\\<const1> ));\n\r");
    /* */
    for (port, dimension) in &inputs {
        if *dimension == 1{
            out_file.append(
                format!("IBUF \\{port}_IBUF_inst \n\r\t(.I({port}), \n\r\t.O({port}_IBUF));\n\r", port=port)
            );
        }
        else {
            for i in 0..*dimension {
                out_file.append(
                    format!("IBUF \\{port}_IBUF[0]_inst \n\r\t(.I({port}[{dim}]), \n\r\t.O({port}_IBUF[{dim}]));\n\r", port=port, dim=i)
                );
            }
        }  
    }
    for (port, dimension) in &outputs {
        if *dimension == 1{
            out_file.append(
                format!("OBUF \\{port}_OBUF_inst \n\r\t(.I({port}), \n\r\t.O({port}_OBUF));\n\r", port=port)
            );
        }
        else {
            for i in 0..*dimension {
                out_file.append(
                    format!("OBUF \\{port}_OBUF[0]_inst \n\r\t(.I({port}[{dim}]), \n\r\t.O({port}_OBUF[{dim}]));\n\r", port=port, dim=i)
                );
            }
        } 
    }
    return out_file;
}





/*
fn set_wires(line_r : &String) -> (String, i32) {
}
fn set_regs(line_r : &String) -> (String, i32) {
}
fn set_assign(line_r : &String) -> (String, i32) {
}
fn set_alwayses(line_r : &String) -> (String, String) {
}
*/