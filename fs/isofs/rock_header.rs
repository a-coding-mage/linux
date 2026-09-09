/* SPDX-License-Identifier: GPL-2.0 */
/*
 * These structs are used by the system-use-sharing protocol, in which the
 * Rock Ridge extensions are embedded.  It is quite possible that other
 * extensions are present on the disk, and this is fine as long as they
 * all use SUSP
 */

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct SU_SP_s {
    pub magic: [u8; 2],
    pub skip: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SU_CE_s {
    pub extent: [u8; 8],
    pub offset: [u8; 8],
    pub size: [u8; 8],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct SU_ER_s {
    pub len_id: u8,
    pub len_des: u8,
    pub len_src: u8,
    pub ext_ver: u8,
    // Flexible array member: storage follows this declaration.
    pub data: [u8; 0],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct RR_RR_s {
    pub flags: [u8; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RR_PX_s {
    pub mode: [u8; 8],
    pub n_links: [u8; 8],
    pub uid: [u8; 8],
    pub gid: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RR_PN_s {
    pub dev_high: [u8; 8],
    pub dev_low: [u8; 8],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct SL_component {
    pub flags: u8,
    pub len: u8,
    // Flexible array member counted by len; storage follows this declaration.
    pub text: [u8; 0],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct RR_SL_s {
    pub flags: u8,
    pub link: SL_component,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct RR_NM_s {
    pub flags: u8,
    // Flexible array member: storage follows this declaration.
    pub name: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RR_CL_s {
    pub location: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RR_PL_s {
    pub location: [u8; 8],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct RR_TF_s {
    pub flags: u8,
    // Flexible array member: storage follows this declaration.
    pub data: [u8; 0],
}

/* Linux-specific extension for transparent decompression */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RR_ZF_s {
    pub algorithm: [u8; 2],
    pub parms: [u8; 2],
    pub real_size: [u8; 8],
}

/*
 * These are the bits and their meanings for flags in the TF structure.
 */
pub const TF_CREATE: u8 = 1;
pub const TF_MODIFY: u8 = 2;
pub const TF_ACCESS: u8 = 4;
pub const TF_ATTRIBUTES: u8 = 8;
pub const TF_BACKUP: u8 = 16;
pub const TF_EXPIRATION: u8 = 32;
pub const TF_EFFECTIVE: u8 = 64;
pub const TF_LONG_FORM: u8 = 128;

#[repr(C)]
#[derive(Copy, Clone)]
pub union rock_ridge_u {
    pub SP: SU_SP_s,
    pub CE: SU_CE_s,
    pub ER: SU_ER_s,
    pub RR: RR_RR_s,
    pub PX: RR_PX_s,
    pub PN: RR_PN_s,
    pub SL: RR_SL_s,
    pub NM: RR_NM_s,
    pub CL: RR_CL_s,
    pub PL: RR_PL_s,
    pub TF: RR_TF_s,
    pub ZF: RR_ZF_s,
}

#[repr(C)]
pub struct rock_ridge {
    pub signature: [u8; 2],
    pub len: u8,
    pub version: u8,
    pub u: rock_ridge_u,
}

pub const RR_PX: u8 = 1; // POSIX attributes
pub const RR_PN: u8 = 2; // POSIX devices
pub const RR_SL: u8 = 4; // Symbolic link
pub const RR_NM: u8 = 8; // Alternate Name
pub const RR_CL: u8 = 16; // Child link
pub const RR_PL: u8 = 32; // Parent link
pub const RR_RE: u8 = 64; // Relocation directory
pub const RR_TF: u8 = 128; // Timestamps

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
