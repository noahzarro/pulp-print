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

#[macro_export]
macro_rules! print_nr {
    ($name:tt,$number:tt) => {{
        {
            use numtoa::NumToA; 
            print($name);
            let mut buf = [0u8; 100];
            let number = number.numtoa_str(16, &mut buf);
            print($number);
            print("\n");
            
        }

    }};
}