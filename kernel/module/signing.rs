// SPDX-License-Identifier: GPL-2.0-or-later
/* Module signature checker
 *
 * Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// C dependencies supplied by the surrounding kernel translation.

// MODULE_PARAM_PREFIX is "module." for this implementation.
static mut sig_enforce: bool = cfg!(CONFIG_MODULE_SIG_FORCE);

extern "C" {
    static MODULE_SIGNATURE_MARKER: [u8; 0];

    fn mod_check_sig(ms: *const module_signature, len: usize, name: *const u8) -> i32;
    fn verify_pkcs7_signature(
        data: *const core::ffi::c_void,
        len: usize,
        sig: *const core::ffi::c_void,
        sig_len: usize,
        usage: i32,
        id_type: i32,
        view_content: *mut core::ffi::c_void,
        ctx: *mut core::ffi::c_void,
    ) -> i32;
    fn security_locked_down(what: i32) -> i32;
}

// These declarations are provided by other translated kernel files.
#[repr(C)]
pub struct module_signature {
    pub algo: u8,
    pub hash: u8,
    pub id_type: u8,
    pub signer_len: u8,
    pub sig_len: u32,
    pub key_id_len: u8,
    pub __pad: u8,
}

#[repr(C)]
pub struct load_info {
    pub hdr: *const core::ffi::c_void,
    pub len: usize,
    pub sig_ok: bool,
}

extern "C" {
    fn pr_devel(fmt: *const u8, ...);
    fn pr_notice(fmt: *const u8, ...);
}

pub const MODULE_INIT_IGNORE_MODVERSIONS: i32 = 1 << 0;
pub const MODULE_INIT_IGNORE_VERMAGIC: i32 = 1 << 1;
pub const VERIFY_USE_SECONDARY_KEYRING: i32 = 1;
pub const VERIFYING_MODULE_SIGNATURE: i32 = 0;
pub const LOCKDOWN_MODULE_SIGNATURE: i32 = 0;

pub const ENODATA: i32 = 61;
pub const ENOPKG: i32 = 65;
pub const ENOKEY: i32 = 126;
pub const EKEYREJECTED: i32 = 129;
pub const EBADMSG: i32 = 74;

pub unsafe extern "C" fn is_module_sig_enforced() -> bool {
    sig_enforce
}

pub unsafe extern "C" fn set_module_sig_enforced() {
    sig_enforce = true;
}

pub unsafe extern "C" fn mod_verify_sig(
    mod_: *const core::ffi::c_void,
    info: *mut load_info,
) -> i32 {
    let mut ms: module_signature = core::mem::zeroed();
    let mut sig_len: usize;
    let mut modlen = (*info).len;
    let mut ret: i32;

    pr_devel(b"==>%s(,%zu)\0".as_ptr(), b"mod_verify_sig\0".as_ptr(), modlen);

    if modlen <= core::mem::size_of::<module_signature>() {
        return -EBADMSG;
    }

    core::ptr::copy_nonoverlapping(
        (mod_ as *const u8).add(modlen - core::mem::size_of::<module_signature>()),
        &mut ms as *mut module_signature as *mut u8,
        core::mem::size_of::<module_signature>(),
    );

    ret = mod_check_sig(&ms, modlen, b"module\0".as_ptr());
    if ret != 0 {
        return ret;
    }

    sig_len = u32::from_be(ms.sig_len) as usize;
    modlen -= sig_len + core::mem::size_of::<module_signature>();
    (*info).len = modlen;

    verify_pkcs7_signature(
        mod_,
        modlen,
        (mod_ as *const u8).add(modlen) as *const core::ffi::c_void,
        sig_len,
        VERIFY_USE_SECONDARY_KEYRING,
        VERIFYING_MODULE_SIGNATURE,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
    )
}

pub unsafe extern "C" fn module_sig_check(info: *mut load_info, flags: i32) -> i32 {
    let mut err = -ENODATA;
    let markerlen = core::mem::size_of_val(&MODULE_SIGNATURE_MARKER) - 1;
    let reason: *const u8;
    let mod_ = (*info).hdr;
    let mangled_module = flags
        & (MODULE_INIT_IGNORE_MODVERSIONS | MODULE_INIT_IGNORE_VERMAGIC)
        != 0;

    /* Do not allow mangled modules as a module with version information
     * removed is no longer the module that was signed. */
    if !mangled_module && (*info).len > markerlen {
        let marker = (mod_ as *const u8).add((*info).len - markerlen);
        if core::slice::from_raw_parts(marker, markerlen)
            == core::slice::from_raw_parts(MODULE_SIGNATURE_MARKER.as_ptr(), markerlen)
        {
            /* We truncate the module to discard the signature */
            (*info).len -= markerlen;
            err = mod_verify_sig(mod_, info);
            if err == 0 {
                (*info).sig_ok = true;
                return 0;
            }
        }
    }

    /* We don't permit modules to be loaded into the trusted kernels
     * without a valid signature on them, but if we're not enforcing,
     * certain errors are non-fatal. */
    reason = match err {
        -ENODATA => b"unsigned module\0".as_ptr(),
        -ENOPKG => b"module with unsupported crypto\0".as_ptr(),
        -ENOKEY => b"module with unavailable key\0".as_ptr(),
        _ => return err,
    };

    if is_module_sig_enforced() {
        pr_notice(b"Loading of %s is rejected\n\0".as_ptr(), reason);
        return -EKEYREJECTED;
    }

    security_locked_down(LOCKDOWN_MODULE_SIGNATURE)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
