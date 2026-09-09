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
#[allow(non_upper_case_globals)]
pub static __start_fixup: *const u8 = core::ptr::null();

#[allow(non_upper_case_globals)]
pub static __stop_fixup: *const u8 = core::ptr::null();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
