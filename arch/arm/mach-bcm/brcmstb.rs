// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2013-2014 Broadcom Corporation

// Translated from the C implementation. The original includes provide the
// kernel initialization and machine-description interfaces used below.

/*
 * Storage for debug-macro.S's state.
 *
 * This must be in .data not .bss so that it gets initialized each time the
 * kernel is loaded. The data is declared here rather than debug-macro.S so
 * that multiple inclusions of debug-macro.S point at the same data.
 */
pub static mut brcmstb_uart_config: [u32; 3] = [
	/* Debug UART initialization required */
	1,
	/* Debug UART physical address */
	0,
	/* Debug UART virtual address */
	0,
];

static brcmstb_match: [&'static core::ffi::c_char; 3] = [
	b"brcm,bcm7445\0".as_ptr() as *const core::ffi::c_char,
	b"brcm,brcmstb\0".as_ptr() as *const core::ffi::c_char,
	core::ptr::null(),
];

// DT_MACHINE_START(BRCMSTB, "Broadcom STB (Flattened Device Tree)")
//     .dt_compat = brcmstb_match,
// MACHINE_END
// The macro-defined machine descriptor is represented by the following
// external interface; its concrete definition is supplied by the kernel.
extern "C" {
	static BRCMSTB: core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
