/* SPDX-License-Identifier: GPL-2.0
 *
 * include/asm-sh/machvec.h
 *
 * Copyright 2000 Stuart Menefy (stuart.menefy@st.com)
 */

// C header dependencies: <linux/types.h>, <linux/time.h>, and
// <generated/machtypes.h> provide the surrounding platform definitions.

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct sh_machine_vector {
    pub mv_setup: Option<unsafe extern "C" fn(cmdline_p: *mut *mut c_char)>,
    pub mv_name: *const c_char,

    pub mv_irq_demux: Option<unsafe extern "C" fn(irq: c_int) -> c_int>,
    pub mv_init_irq: Option<unsafe extern "C" fn()>,

    pub mv_clk_init: Option<unsafe extern "C" fn() -> c_int>,
    pub mv_mode_pins: Option<unsafe extern "C" fn() -> c_int>,

    pub mv_mem_init: Option<unsafe extern "C" fn()>,
    pub mv_mem_reserve: Option<unsafe extern "C" fn()>,
}

extern "C" {
    pub static mut sh_mv: sh_machine_vector;
}

#[macro_export]
macro_rules! get_system_type {
    () => {
        unsafe { $crate::sh_mv.mv_name }
    };
}

// C build-time attributes: __used __section(".machvec.init")
// Apply the corresponding target-specific Rust/linker attributes at use sites.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
