use regex::Regex;
use chrono::{Timelike, Utc};
use string_builder::Builder;
use std::collections::HashMap;
use crate::verilog_analysis::AnalysisData;

pub fn synthesis(data : AnalysisData) -> Builder {
    let mut out_file = Builder::default();
    out_file = set_header(out_file);
    out_file = set_module_name(out_file, data.module_name);
    out_file = set_io(out_file, data.inputs.clone(), data.outputs.clone());
    out_file = set_wires(out_file, data.inputs.clone(), data.outputs.clone());
    out_file = set_alwayses(out_file, data.always_assigns, data.inputs, data.outputs);

    return out_file;
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
    return out_file;
}

fn set_wires(mut out_file : Builder, inputs : HashMap<String, i32>, outputs : HashMap<String, i32>) -> Builder {
    out_file.append("wire \\<const1> ;\n\r");
    for (port, dimension) in &inputs {
        if *dimension == 1{
            out_file.append(
                format!("wire {};\n\r", port)
            );
            out_file.append(
                format!("wire {}_IBUF;\n\r", port)
            );
        }
        else {
            out_file.append(
                format!("wire [{dim}:0] {port};\n\r", dim = *dimension-1, port = port)
            );
            out_file.append(
                format!("wire [{dim}:0] {port}_IBUF;\n\r", dim = *dimension-1, port = port)
            );
        }   
    }
    for (port, dimension) in &outputs {
        if *dimension == 1{
            out_file.append(
                format!("wire {};\n\r", port)
            );
            out_file.append(
                format!("wire {}_OBUF;\n\r", port)
            );
        }
        else {
            out_file.append(
                format!("wire [{dim}:0] {port};\n\r", dim = *dimension-1, port = port)
            );
            out_file.append(
                format!("wire [{dim}:0] {port}_OBUF;\n\r", dim = *dimension-1, port = port)
            );
        }   
    }

    
    out_file.append("VCC VCC\n\r\t(.P(\\<const1> ));\n\r");
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

fn set_alwayses(
        mut out_file : Builder, 
        always : HashMap<String, String>, 
        inputs : HashMap<String, i32>, 
        outputs : HashMap<String, i32>
    ) -> Builder {
        let mut alw_cnt = 0;
        for (expression, event) in &always {
            alw_cnt += 1;
            let signals: Vec<&str> = Regex::new(r"\w+").unwrap().find_iter(&expression).map(|x| x.as_str()).collect();
            let mut signal : HashMap<String, i32> = HashMap::new();
            for i in signals.clone() {
                if inputs.contains_key(i) {
                    signal.insert(i.to_string(), *inputs.get(i).unwrap());
                }
                else if outputs.contains_key(i) {
                    signal.insert(i.to_string(), *outputs.get(i).unwrap());
                }
            }
            let operation: Vec<&str> = Regex::new(r"[&|+-]+").unwrap().find_iter(&expression).map(|x| x.as_str()).collect();
            let lut_cmd: i32 = match operation[0] {
                "&&" => 6,
                "||" => 5,
                "&" => 4,
                "|" => 3,
                "+" => 2,
                "-" => 1,
                _   => 0
            };
            if lut_cmd == 1 || lut_cmd == 2 {
                let max_dim = signal.values().max().unwrap();
                for x in (0..*max_dim).step_by(2) {
                    out_file.append(
                        format!("wire [{dim}:0]p_{cnt}_in;\n\r", dim = max_dim, cnt = alw_cnt)
                    );
                    out_file.append(
                        format!("LUT2 #(
\t.INIT(4'h{lut_cmd})) 
\t\\{out}[{curr}]_i_{currinc} 
\t(.I0({in1}_IBUF[0]),
\t.I1({in2}_IBUF[0]),
\t.O(p_{cnt}_in[{curr}]));
\t(* SOFT_HLUTNM = \"soft_lutpair0\" *) 
LUT4 #(
\t.INIT(16'h{lut_cmd})) 
\t\\{out}[{currinc}]_i_{currinc} 
\t(.I0({in1}_IBUF[{curr}]),
\t.I1({in2}_IBUF[{curr}]),
\t.I2({in2}_IBUF[{currinc}]),
\t.I3({in1}_IBUF[{currinc}]),
\t.O(p_{cnt}_in[{currinc}]));\n\r", out = signals[0], in1 = signals[1], in2 = signals[2], curr = x, currinc = x+1, cnt = alw_cnt, lut_cmd = lut_cmd)
                    )
                }
            }
            else {
                let max_dim = signal.values().max().unwrap();
                for x in (0..*max_dim).step_by(2) {
                    out_file.append(
                        format!("wire [{dim}:0]p_{cnt}_in;\n\r", dim = max_dim, cnt = alw_cnt)
                    );
                    out_file.append(
                        format!("
LUT4 #(
\t.INIT(16'h{lut_cmd})) 
\t\\{out}[{currinc}]_i_1 
\t(.I0({in1}_IBUF[{curr}]),
\t.I1({in2}_IBUF[{curr}]),
\t.I2({in2}_IBUF[{currinc}]),
\t.I3({in1}_IBUF[{currinc}]),
\t.O(p_{cnt}_in[{currinc}]));\n\r", out = signals[0], in1 = signals[1], in2 = signals[2], curr = x, currinc = x+1, cnt = alw_cnt, lut_cmd = lut_cmd)
                    )
                }
                
            }
            print!("{}!!{}", expression, event);
        }

    
    return out_file;
}



