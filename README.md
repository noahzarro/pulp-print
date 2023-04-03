# `pulp-print`

This crate offers macros that ease printing in the QuestaSim simulation of the controlPULP IP.

It offers the `#[print]` and `#[println]` macros for printing of plain text.
E.g. `println!("Hello World")`.

Furthermore it offers `#[print_nr]` and `#[print_nr_only]` to print numbers and optional text. It also allows to set the format of the number. Either binary, hex, or decimal. The conversion from a number ot a string happens without the usage of the heap.

E.g. `let i=3; print_nr!("Nr of iterations:", i, Format::Dec);`