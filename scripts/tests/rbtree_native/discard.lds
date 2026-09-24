/* Freestanding panic=abort process: retain no unused exception/unwind metadata.
 * Official Rust target core carries these references even when our callers
 * abort. Never supply a fake rust_eh_personality implementation. */
SECTIONS { /DISCARD/ : { *(.discard*) *(.eh_frame*) *(.gcc_except_table*) *(.data.DW.ref.rust_eh_personality) } } INSERT AFTER .bss;
