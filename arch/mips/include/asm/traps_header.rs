/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Trap handling definitions.
 *
 * Copyright (C) 2002, 2003  Maciej W. Rozycki
 */

/* Possible status responses for a board_be_handler backend. */
pub const MIPS_BE_DISCARD: i32 = 0; /* return with no action */
pub const MIPS_BE_FIXUP: i32 = 1;   /* return to the fixup code */
pub const MIPS_BE_FATAL: i32 = 2;   /* treat as an unrecoverable error */

/* These types are supplied by the surrounding kernel translation. */
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    pub static mut board_be_init: Option<unsafe extern "C" fn()>;
    pub fn mips_set_be_handler(
        handler: Option<unsafe extern "C" fn(reg: *mut pt_regs, is_fixup: c_int) -> c_int>,
    );

    pub static mut board_nmi_handler_setup: Option<unsafe extern "C" fn()>;
    pub static mut board_ejtag_handler_setup: Option<unsafe extern "C" fn()>;
    pub static mut board_bind_eic_interrupt: Option<unsafe extern "C" fn(irq: c_int, regset: c_int)>;
    pub static mut board_ebase_setup: Option<unsafe extern "C" fn()>;
    pub static mut board_cache_error_setup: Option<unsafe extern "C" fn()>;

    pub fn register_nmi_notifier(nb: *mut notifier_block) -> c_int;
    pub fn reserve_exception_space(addr: phys_addr_t, size: c_ulong);
    pub static mut except_vec_nmi: [c_char; 0];

    pub fn do_ade(regs: *mut pt_regs);
    pub fn do_be(regs: *mut pt_regs);
    pub fn do_ov(regs: *mut pt_regs);
    pub fn do_fpe(regs: *mut pt_regs, fcr31: c_ulong);
    pub fn do_bp(regs: *mut pt_regs);
    pub fn do_tr(regs: *mut pt_regs);
    pub fn do_ri(regs: *mut pt_regs);
    pub fn do_cpu(regs: *mut pt_regs);
    pub fn do_msa_fpe(regs: *mut pt_regs, msacsr: c_uint);
    pub fn do_msa(regs: *mut pt_regs);
    pub fn do_mdmx(regs: *mut pt_regs);
    pub fn do_watch(regs: *mut pt_regs);
    pub fn do_mcheck(regs: *mut pt_regs);
    pub fn do_mt(regs: *mut pt_regs);
    pub fn do_dsp(regs: *mut pt_regs);
    pub fn do_reserved(regs: *mut pt_regs);
    pub fn do_ftlb();
    pub fn do_gsexc(regs: *mut pt_regs, diag1: u32);
    pub fn do_daddi_ov(regs: *mut pt_regs);
    pub fn do_page_fault(regs: *mut pt_regs, write: c_ulong, address: c_ulong);

    pub fn cache_parity_error();
    pub fn ejtag_exception_handler(regs: *mut pt_regs);
    pub fn nmi_exception_handler(regs: *mut pt_regs) -> !;
}

/* Opaque types and aliases supplied by other translated headers. */
#[allow(non_camel_case_types)]
pub enum pt_regs {}
#[allow(non_camel_case_types)]
pub enum notifier_block {}
pub type phys_addr_t = usize;

pub const VECTORSPACING: usize = 0x100; /* for EI/VI mode */

/* C macro nmi_notifier(fn, pri), represented as a Rust macro. */
#[macro_export]
macro_rules! nmi_notifier {
    ($fn_name:ident, $pri:expr) => {{
        static mut FN_NB: $crate::notifier_block = $crate::notifier_block {};
        let _ = (&mut FN_NB, $fn_name, $pri);
        unsafe { $crate::register_nmi_notifier(&mut FN_NB) }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
