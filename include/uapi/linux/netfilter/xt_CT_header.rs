/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency supplied by the surrounding kernel bindings.
pub struct nf_conn;

pub const XT_CT_NOTRACK: u32 = 1 << 0;
pub const XT_CT_NOTRACK_ALIAS: u32 = 1 << 1;
pub const XT_CT_ZONE_DIR_ORIG: u32 = 1 << 2;
pub const XT_CT_ZONE_DIR_REPL: u32 = 1 << 3;
pub const XT_CT_ZONE_MARK: u32 = 1 << 4;

pub const XT_CT_MASK: u32 = XT_CT_NOTRACK
    | XT_CT_NOTRACK_ALIAS
    | XT_CT_ZONE_DIR_ORIG
    | XT_CT_ZONE_DIR_REPL
    | XT_CT_ZONE_MARK;

#[repr(C, align(8))]
pub struct xt_ct_target_info {
    pub flags: u16,
    pub zone: u16,
    pub ct_events: u32,
    pub exp_events: u32,
    pub helper: [core::ffi::c_char; 16],

    /* Used internally by the kernel */
    pub ct: *mut nf_conn,
}

#[repr(C, align(8))]
pub struct xt_ct_target_info_v1 {
    pub flags: u16,
    pub zone: u16,
    pub ct_events: u32,
    pub exp_events: u32,
    pub helper: [core::ffi::c_char; 16],
    pub timeout: [core::ffi::c_char; 32],

    /* Used internally by the kernel */
    pub ct: *mut nf_conn,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
