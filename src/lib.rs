#![no_std]

use core::arch::asm;

pub fn print(text: &str) {
    let print_addr = 0x1A10FF80;

    for byte in text.as_bytes() {
        unsafe {
            asm!(
                "sw	{1},0({0})",
                in(reg) print_addr,
                in(reg) *byte as u32,
            );
        }
    }
}

#[macro_export]
macro_rules! println {
    ($($arg:tt),*) => {{
        use ::print;
        $(
            print($arg);
            print(" ");
        )*
        print("\n");

    }};
}

#[macro_export]
macro_rules! print {
    ($($arg:tt),*) => {{
        $(
            print($arg);
            print(" ");
        )*

    }};
}

pub enum Format {
    Dec,
    Hex,
    Bin
}

#[macro_export]
macro_rules! print_nr {
    ($name:tt,$number:tt,$format:path) => {
       
            print($name);
            let mut buf = [0u8; 100];
            let number = match $format {
                Format::Hex => $number.numtoa_str(16, &mut buf),
                Format::Bin => $number.numtoa_str(2, &mut buf),
                _ => $number.numtoa_str(10, &mut buf),
            };
            
            print(" ");
            print(number);
            print("\n");        

    };
}