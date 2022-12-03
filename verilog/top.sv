`timescale 1ns / 1ps
//////////////////////////////////////////////////////////////////////////////////
// Company: 
// Engineer: 
// 
// Create Date: 05.11.2022 22:25:59
// Design Name: 
// Module Name: top
// Project Name: 
// Target Devices: 
// Tool Versions: 
// Description: 
// 
// Dependencies: 
// 
// Revision:
// Revision 0.01 - File Created
// Additional Comments:
// 
//////////////////////////////////////////////////////////////////////////////////


module top(
        input clk,
        input wire rst,
        input [1:0] a,
        input [1:0] b,
        input d,
        input e,
        output f,
        output reg [1:0] c
    );
    always_ff @(posedge clk) begin
        c <= a + b;
    end
    assign f = e | d;
endmodule
