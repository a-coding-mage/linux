/*
 * arch/sparc/kernel/ksyms.c: Sparc specific ksyms support.
 *
 * Copyright (C) 1996 David S. Miller (davem@caip.rutgers.edu)
 * Copyright (C) 1996 Eddie C. Dost (ecd@skynet.be)
 */

// C dependencies: <linux/init.h>, <linux/export.h>

/* This is needed only for drivers/sbus/char/openprom.c */
extern "C" {
    pub static mut saved_command_line: *mut core::ffi::c_char;
}

// Equivalent of EXPORT_SYMBOL(saved_command_line): export the external
// symbol through the kernel's symbol-export mechanism supplied by the build.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
