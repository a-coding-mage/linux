// Dependency intent: declarations from "misc.h" are supplied by other files.

/* This might be accessed before .bss is cleared, so use .data instead. */
#[link_section = ".data"]
pub static mut early_serial_base: i32 = 0;

// The implementation from "../early_serial_console.c" is supplied by the
// surrounding source tree and is intentionally not expanded in this isolated
// translation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
