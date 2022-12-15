Редакция 1.0-0-preview от 06.11.2022

[![Rust](https://github.com/shomov/Verilog-synthesis/actions/workflows/rust.yml/badge.svg?branch=develop)](https://github.com/shomov/Verilog-synthesis/actions/workflows/rust.yml)

# [RUS|EN] Синтез Verilog HDL | Verilog HDL synthesis

## [RUS] Синтез Verilog HDL

Задачей проекта является разработка программы по простому RTL-синтезу аппаратных описаний Verilog HDL в netlist, близкий к Xilinx Vivado Synthesis 2022.

### Функциональные требования

* Типы портов: input, output
* Типы данных: wire, reg
* Размерность данных: однобитные, многобитные 
* Присваивания: непрерывное, блокирующее и неблокирующее 
* Поведенческий блок always
* Конструкция if..else
* Побитовые и редуцированные операции (AND, OR, NOT, XOR, XNOR)
* Логические операции (AND, OR, NOT)
* Арифметические операции
* Исключения

### Запуск проекта
#### Без использования докера
   
 1. Склонируйте репозиторий:

        git clone https://github.com/shomov/Verilog-synthesis.git

 2. Подключите все зависимости: 
   
        cargo build 

 3. Запустите исполняемый файл


#### С использованием докера:
1. Создание докер-образа с помощью dockerfile

        docker build -t verilog-synthesis .

2. Запуск докер-образа
   
        docker run -p 8080:8080 verilog-synthesis 

### Пример работы сервиса

<details ><summary>Загружаемый файл</summary>
  
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
    endmodule
  
  
</details>

<details ><summary>Результат</summary>

    //Garipova & Shomov Verilog HDL synthesis tool
    //Synthesis Time UTC 09:55:59
    `timescale 1 ps / 1 ps

    (* STRUCTURAL_NETLIST = "yes" *)
    module top(
    e,
    d,
    rst,
    a,
    clk,
    b,
    c,
    f
    );
    input e;
    input d;
    input rst;
    input [1:0] a;
    input clk;
    input [1:0] b;
    output [1:0] c;
    output f;
    wire \<const1> ;
    wire e;
    wire e_IBUF;
    wire d;
    wire d_IBUF;
    wire rst;
    wire rst_IBUF;
    wire [1:0] a;
    wire [1:0] a_IBUF;
    wire clk;
    wire clk_IBUF;
    wire [1:0] b;
    wire [1:0] b_IBUF;
    wire [1:0] c;
    wire [1:0] c_OBUF;
    wire f;
    wire f_OBUF;
    VCC VCC
        (.P(\<const1> ));
    IBUF \e_IBUF_inst 
        (.I(e), 
        .O(e_IBUF));
    IBUF \d_IBUF_inst 
        (.I(d), 
        .O(d_IBUF));
    IBUF \rst_IBUF_inst 
        (.I(rst), 
        .O(rst_IBUF));
    IBUF \a_IBUF[0]_inst 
        (.I(a[0]), 
        .O(a_IBUF[0]));
    IBUF \a_IBUF[0]_inst 
        (.I(a[1]), 
        .O(a_IBUF[1]));
    IBUF \clk_IBUF_inst 
        (.I(clk), 
        .O(clk_IBUF));
    IBUF \b_IBUF[0]_inst 
        (.I(b[0]), 
        .O(b_IBUF[0]));
    IBUF \b_IBUF[0]_inst 
        (.I(b[1]), 
        .O(b_IBUF[1]));
    OBUF \c_OBUF[0]_inst 
        (.I(c[0]), 
        .O(c_OBUF[0]));
    OBUF \c_OBUF[0]_inst 
        (.I(c[1]), 
        .O(c_OBUF[1]));
    OBUF \f_OBUF_inst 
        (.I(f), 
        .O(f_OBUF));
    wire [2:0]p_1_in;
    LUT2 #(
        .INIT(4'h2)) 
        \c[0]_i_1 
        (.I0(a_IBUF[0]),
        .I1(b_IBUF[0]),
        .O(p_1_in[0]));
        (* SOFT_HLUTNM = "soft_lutpair0" *) 
    LUT4 #(
        .INIT(16'h2)) 
        \c[1]_i_1 
        (.I0(a_IBUF[0]),
        .I1(b_IBUF[0]),
        .I2(b_IBUF[1]),
        .I3(a_IBUF[1]),
        .O(p_1_in[1]));



</details>

 


### Политика лицензирования 

Apache License, Version 2.0

### Правообладатели

Гарипова Гульзира garipova.gz@mail.ru<br>
Михаил Шомов m@shomov.spb.ru

<hr>

## [EN] Verilog HDL synthesis

The task of the project is to develop a program for simple RTL synthesis of Verilog HDL hardware descriptions to netlist, close to Xilinx Vivado Synthesis 2022.

### Functional requirements

* Ports types: input, output
* Data types: wire, reg
* Data dimension: single-bit, multi-bit
* Assignments: continuous, blocking and nonblocking
* Procedural block always
* if..else construction
* Bitwise and reduction operators (AND, OR, NOT, XOR, XNOR)
* Logical operators (AND, OR, NOT)
* Arithmetic operators (+, -)
* Exceptions 


### Launch 
#### Without docker
   
 1. Clone repository:

        git clone https://github.com/shomov/Verilog-synthesis.git

 2. Include all dependencies : 
   
        cargo build 

 3. Run 


#### With docker
1. Build docker image with dockerfile

        docker build -t verilog-synthesis .

2. Run 
   
        docker run -P -it verilog-synthesis 


### Licensing Policy

Apache License, Version 2.0

### Copyright holders

Garipova Gulzira garipova.gz@mail.ru<br>
Mikhail Shomov m@shomov.spb.ru
