fn main() {
    // hello world in bf
    //cargo run ">++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]<++.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-]<+."

    let args = std::env::args()
        .collect::<Vec<_>>();

    if args.len() != 2 {
        println!("");
    }

    let syntax = &args[1];
    let mut tab_level: usize = 1;

    let mut psuedo_code = String::from("#include <stdio.h>\r\n#include <stdlib.h>\r\n#include <string.h>\r\n#include <inttypes.h>\r\n\r\nint main()\r\n{\r\n\tunsigned char buffer[30000];\r\n\tmemset((void*)buffer, 0, sizeof(unsigned char) * 30000);\r\n\tuintptr_t data_ptr = 0;\r\n\r\n\t");

    //https://en.wikipedia.org/wiki/Brainfuck#Language_design
    for (index, instruction) in syntax.chars().enumerate() {
        let tab_string = "\t".repeat(tab_level);
        match instruction {
            '>' => psuedo_code.push_str(&format!("data_ptr++;\r\n{}", tab_string)),
            '<' => psuedo_code.push_str(&format!("data_ptr--;\r\n{}", tab_string)),
            '+' => psuedo_code.push_str(&format!("buffer[data_ptr]++;\r\n{}", tab_string)),
            '-' => psuedo_code.push_str(&format!("buffer[data_ptr]--;\r\n{}", tab_string)),
            '.' => psuedo_code.push_str(&format!("putchar(buffer[data_ptr]);\r\n{}", tab_string)),
            ',' => psuedo_code.push_str(&format!("buffer[data_ptr] = getchar();\r\n{}", tab_string)),
            '[' => {
                tab_level += 1;
                let tab_string = "\t".repeat(tab_level);
                psuedo_code.push_str(&format!("while (buffer[data_ptr] != 0) {{\r\n\r{}", tab_string));
            },
            ']' => {
                tab_level -= 1;
                let tab_string = "\t".repeat(tab_level);
                psuedo_code.push_str(&format!("\r{}}}\r\n\r\n{}", tab_string, tab_string));
            }
            invalid_instruction => panic!("invalid instruction was passed in at index {}: '{}'", index, invalid_instruction)
        }
    };

    psuedo_code.push_str("\r\n}");

    std::fs::write("translated.c", psuedo_code)
        .expect("written code to file");

    println!("Successfully converted to C code.");
}
