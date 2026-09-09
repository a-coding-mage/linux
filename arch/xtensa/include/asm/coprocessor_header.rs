/*
 * include/asm-xtensa/coprocessor.h
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2003 - 2007 Tensilica Inc.
 */

// C dependencies: variant/tie.h, asm/core.h, and asm/types.h.
// The assembler-only xchal/save/load macros are intentionally retained as
// comments because they have no Rust executable equivalent.

/*
 * XTENSA_HAVE_COPROCESSOR(x) returns 1 if coprocessor x is configured.
 * XTENSA_HAVE_IO_PORT(x) returns 1 if io-port x is configured.
 */

#[inline]
pub const fn xtensa_have_coprocessor(x: u32, xchal_cp_mask: u32, xchal_cp_port_mask: u32) -> u32 {
    (xchal_cp_mask ^ xchal_cp_port_mask) & (1u32 << x)
}

#[inline]
pub const fn xtensa_have_coprocessors(xchal_cp_mask: u32, xchal_cp_port_mask: u32) -> u32 {
    xchal_cp_mask ^ xchal_cp_port_mask
}

#[inline]
pub const fn xtensa_have_io_port(x: u32, xchal_cp_port_mask: u32) -> u32 {
    xchal_cp_port_mask & (1u32 << x)
}

#[inline]
pub const fn xtensa_have_io_ports(xchal_cp_port_mask: u32) -> u32 {
    xchal_cp_port_mask
}

/*
 * Additional registers.
 * We define three types of additional registers:
 *  ext: extra registers that are used by the compiler
 *  cpn: optional registers that can be used by a user application
 *  cpX: coprocessor registers that can only be used if the corresponding
 *       CPENABLE bit is set.
 *
 * The C XCHAL_*_SA_LIST macros expand these structures' fields from the
 * target-specific variant headers. Their expansions are external to this
 * isolated header, so the Rust declarations remain opaque and repr(C).
 */

#[repr(C)]
pub struct XtregsOpt {
    _private: [u8; 0],
}
pub type xtregs_opt_t = XtregsOpt;

#[repr(C)]
pub struct XtregsUser {
    _private: [u8; 0],
}
pub type xtregs_user_t = XtregsUser;

// These types are present when XTENSA_HAVE_COPROCESSORS is nonzero.
#[repr(C)]
pub struct XtregsCp0 { _private: [u8; 0] }
pub type xtregs_cp0_t = XtregsCp0;
#[repr(C)]
pub struct XtregsCp1 { _private: [u8; 0] }
pub type xtregs_cp1_t = XtregsCp1;
#[repr(C)]
pub struct XtregsCp2 { _private: [u8; 0] }
pub type xtregs_cp2_t = XtregsCp2;
#[repr(C)]
pub struct XtregsCp3 { _private: [u8; 0] }
pub type xtregs_cp3_t = XtregsCp3;
#[repr(C)]
pub struct XtregsCp4 { _private: [u8; 0] }
pub type xtregs_cp4_t = XtregsCp4;
#[repr(C)]
pub struct XtregsCp5 { _private: [u8; 0] }
pub type xtregs_cp5_t = XtregsCp5;
#[repr(C)]
pub struct XtregsCp6 { _private: [u8; 0] }
pub type xtregs_cp6_t = XtregsCp6;
#[repr(C)]
pub struct XtregsCp7 { _private: [u8; 0] }
pub type xtregs_cp7_t = XtregsCp7;

#[repr(C)]
pub struct thread_info {
    _private: [u8; 0],
}

extern "C" {
    pub fn coprocessor_flush(ti: *mut thread_info, cp_index: ::core::ffi::c_int);
    pub fn coprocessor_release_all(ti: *mut thread_info);
    pub fn coprocessor_flush_all(ti: *mut thread_info);
    pub fn coprocessor_flush_release_all(ti: *mut thread_info);
    pub fn local_coprocessors_flush_release_all();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
