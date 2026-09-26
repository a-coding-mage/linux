// Linker-script section declaration translated from the source header.
//
// SECTIONS {
//     .m68k_fixup 0 : {
//         __start_fixup = .;
//         *(.m68k_fixup)
//         __stop_fixup = .;
//     }
// }

// The section placement, input-section collection, and linker-defined symbols
// above are supplied by the linker and therefore have no direct Rust item
// equivalent. Preserve their externally visible names for dependent code.
// __start_fixup: linker-script symbol, defined by the SECTIONS fragment above.

// __stop_fixup: linker-script symbol, defined by the SECTIONS fragment above.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
