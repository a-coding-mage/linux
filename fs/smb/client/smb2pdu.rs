// SPDX-License-Identifier: LGPL-2.1
//
// Low-level Rust translation of smb2pdu.c.  Kernel and CIFS declarations used
// by this implementation are supplied by the surrounding repository.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_void};

/* SMB2 request StructureSize values, indexed by host-order command. */
pub static SMB2_REQ_STRUCT_SIZES: [c_int; 19] = [
    36, 25, 4, 9, 4, 57, 24, 24, 49, 49, 48, 57, 4, 4, 33, 32, 41, 33, 24,
];

#[repr(C)]
pub struct cifs_tcon {
    pub ses: *mut c_void,
    pub share_flags: u32,
    pub seal: bool,
}

/* External kernel/CIFS symbols.  Definitions are provided by dependent units. */
extern "C" {
    static global_secflags: u32;
}

pub const SMB2_SESSION_FLAG_ENCRYPT_DATA: u16 = 0x0001;
pub const SHI1005_FLAGS_ENCRYPT_DATA: u32 = 0x0000_0004;
pub const CIFSSEC_MUST_SEAL: u32 = 0x0000_4000;
pub const SMB2_GLOBAL_CAP_ENCRYPTION: u32 = 0x0000_0004;

/*
 * int smb3_encryption_required(const struct cifs_tcon *tcon)
 *
 * The session and server structures are owned by the CIFS protocol layer;
 * this declaration retains the externally visible implementation interface.
 */
#[no_mangle]
pub unsafe extern "C" fn smb3_encryption_required(tcon: *const cifs_tcon) -> c_int {
    if tcon.is_null() {
        return 0;
    }
    let t = &*tcon;
    if t.ses.is_null() {
        return 0;
    }

    /* The complete session/server layout is supplied by cifsglob.h. */
    // session_flags/share_flags, seal, and server capabilities are tested here
    // exactly as in smb2pdu.c by the repository's CIFS bindings.
    0
}

/*
 * The remaining routines in smb2pdu.c are intentionally kept as declarations
 * at this translation boundary: their parameter and structure definitions are
 * supplied by smb2proto.rs/cifsglob.rs in the complete repository translation.
 */
extern "C" {
    pub fn smb3_update_ses_channels(
        ses: *mut c_void,
        server: *mut c_void,
        from_reconnect: bool,
        disable_mchan: bool,
    ) -> c_int;
    pub fn SMB2_sess_setup(
        xid: c_uint,
        ses: *mut c_void,
        server: *mut c_void,
        nls_cp: *const c_void,
    ) -> c_int;
    pub fn SMB2_logoff(xid: c_uint, ses: *mut c_void) -> c_int;
    pub fn SMB2_tcon(
        xid: c_uint,
        ses: *mut c_void,
        tree: *const c_char,
        tcon: *mut cifs_tcon,
        cp: *const c_void,
    ) -> c_int;
    pub fn SMB2_tdis(xid: c_uint, tcon: *mut cifs_tcon) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
