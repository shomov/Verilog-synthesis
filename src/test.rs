#[cfg(test)]
mod tests {
    use std::{path::Path, collections::HashMap};
    use crate::verilog_analysis::{self, AnalysisData};
    
    #[test]
    fn test_name() {
        let data : AnalysisData = verilog_analysis::verilog_analysis(Path::new("verilog/top.sv"));
        assert_eq!(data.module_name[0], "top");
    }

    #[test]
    fn test_inputs() {
        let data : AnalysisData = verilog_analysis::verilog_analysis(Path::new("verilog/top.sv"));
        let inputs = HashMap::from([
            ("clk".to_string(), 1),
            ("rst".to_string(), 1),
            ("a".to_string(), 2),
            ("b".to_string(), 2),
            ("d".to_string(), 1),
            ("e".to_string(), 1),
        ]);
        assert_eq!(data.inputs, inputs);      
    }

    #[test]
    fn test_outputs() {
        let data : AnalysisData = verilog_analysis::verilog_analysis(Path::new("verilog/top.sv"));
        let outputs = HashMap::from([
            ("f".to_string(), 1),
            ("c".to_string(), 2)
        ]);
        assert_eq!(data.outputs, outputs);      
    }





    // #[test]
    // #[should_panic]
    // fn test_any_panic() {
    //     divide_non_zero_result(1, 0);
    // }

    // #[test]
    // #[should_panic(expected = "Divide result is zero")]
    // fn test_specific_panic() {
    //     divide_non_zero_result(1, 10);
    // }
}