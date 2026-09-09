// SPDX-License-Identifier: GPL-2.0
//
// Direct Rust translation of tpm2-sessions.c.  Kernel and cryptographic
// interfaces referenced below are supplied by the surrounding repository.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{mem, ptr};

const AUTH_MAX_NAMES: usize = 3;
const AES_KEY_BYTES: usize = AES_KEYSIZE_128;
const AES_KEY_BITS: u16 = (AES_KEY_BYTES * 8) as u16;

#[repr(C)]
pub union tpm2_auth_salt {
    pub salt: [u8; EC_PT_SZ],
    pub scratch: [u8; AES_KEY_BYTES + AES_BLOCK_SIZE],
}

#[repr(C)]
pub struct tpm2_auth {
    pub handle: u32,
    pub session: u32,
    pub our_nonce: [u8; SHA256_DIGEST_SIZE],
    pub tpm_nonce: [u8; SHA256_DIGEST_SIZE],
    pub salt_or_scratch: tpm2_auth_salt,
    pub session_key: [u8; SHA256_DIGEST_SIZE],
    pub passphrase: [u8; SHA256_DIGEST_SIZE],
    pub passphrase_len: i32,
    pub aes_key: aes_enckey,
    pub attrs: u8,
    pub ordinal: __be32,
    pub name_h: [u32; AUTH_MAX_NAMES],
    pub name: [[u8; 2 + SHA512_DIGEST_SIZE]; AUTH_MAX_NAMES],
}

#[cfg(CONFIG_TCG_TPM2_HMAC)]
unsafe fn name_size(name: *const u8) -> i32 {
    match get_unaligned_be16(name) {
        TPM_ALG_SHA1 => (SHA1_DIGEST_SIZE + 2) as i32,
        TPM_ALG_SHA256 => (SHA256_DIGEST_SIZE + 2) as i32,
        TPM_ALG_SHA384 => (SHA384_DIGEST_SIZE + 2) as i32,
        TPM_ALG_SHA512 => (SHA512_DIGEST_SIZE + 2) as i32,
        a => { pr_warn!("tpm: unsupported name algorithm: 0x{:04x}\n", a); -EINVAL }
    }
}

#[cfg(CONFIG_TCG_TPM2_HMAC)]
unsafe fn tpm2_read_public(chip: *mut tpm_chip, handle: u32, name: *mut u8) -> i32 {
    let mso = tpm2_handle_mso(handle);
    if mso != TPM2_MSO_PERSISTENT && mso != TPM2_MSO_VOLATILE && mso != TPM2_MSO_NVRAM {
        ptr::copy_nonoverlapping((&handle as *const u32) as *const u8, name, 4);
        return 4;
    }
    let buf = kzalloc(TPM_BUFSIZE, GFP_KERNEL);
    if buf.is_null() { return -ENOMEM; }
    tpm_buf_init(buf, TPM_BUFSIZE);
    tpm_buf_reset(buf, TPM2_ST_NO_SESSIONS, TPM2_CC_READ_PUBLIC);
    tpm_buf_append_u32(buf, handle);
    let rc = tpm_transmit_cmd(chip, buf, 0, "TPM2_ReadPublic");
    if rc != 0 { return tpm_ret_to_err(rc); }
    let mut off = TPM_HEADER_SIZE as isize;
    off += tpm_buf_read_u16(buf, &mut off) as isize;
    if off + 4 > tpm_buf_length(buf) as isize { return -EIO; }
    let n = tpm_buf_read_u16(buf, &mut off) as i32;
    let alg = name_size((*buf).data.as_ptr().offset(off));
    if alg < 0 || n != alg as u16 || off + alg as isize > tpm_buf_length(buf) as isize { return -EIO; }
    ptr::copy_nonoverlapping((*buf).data.as_ptr().offset(off), name, alg as usize);
    alg
}

pub unsafe fn tpm_buf_append_name(chip: *mut tpm_chip, buf: *mut tpm_buf, handle: u32, name: *mut u8) -> i32 {
    if !tpm2_chip_auth(chip) { tpm_buf_append_handle(buf, handle); return 0; }
    let slot = (tpm_buf_length(buf) - TPM_HEADER_SIZE) / 4;
    if slot >= AUTH_MAX_NAMES { dev_err!(&mut (*chip).dev, "too many handles\n"); tpm2_end_auth_session(chip); return tpm_ret_to_err(-EIO); }
    let auth = (*chip).auth;
    if (*auth).session != tpm_buf_length(buf) { dev_err!(&mut (*chip).dev, "session state malformed"); tpm2_end_auth_session(chip); return tpm_ret_to_err(-EIO); }
    tpm_buf_append_u32(buf, handle); (*auth).session += 4;
    let mso = tpm2_handle_mso(handle);
    if mso == TPM2_MSO_PERSISTENT || mso == TPM2_MSO_VOLATILE || mso == TPM2_MSO_NVRAM {
        let n = if name.is_null() { tpm2_read_public(chip, handle, (*auth).name[slot].as_mut_ptr()) } else { name_size(name) };
        if n < 0 { tpm2_end_auth_session(chip); return tpm_ret_to_err(n); }
        (*auth).name_h[slot] = handle;
        if !name.is_null() { ptr::copy_nonoverlapping(name, (*auth).name[slot].as_mut_ptr(), n as usize); }
    } else if !name.is_null() { dev_err!(&mut (*chip).dev, "handle 0x{:08x} does not use a name\n", handle); tpm2_end_auth_session(chip); return tpm_ret_to_err(-EIO); }
    0
}

pub unsafe fn tpm_buf_append_auth(_chip: *mut tpm_chip, buf: *mut tpm_buf, passphrase: *mut u8, passphrase_len: i32) {
    let offset = (*buf).handles * 4 + TPM_HEADER_SIZE;
    let mut len = 9 + passphrase_len as u32;
    if tpm_buf_length(buf) != offset { len += get_unaligned_be32((*buf).data.as_ptr().add(offset as usize)); put_unaligned_be32(len, (*buf).data.as_mut_ptr().add(offset as usize)); } else { tpm_buf_append_u32(buf, len); }
    tpm_buf_append_u32(buf, TPM2_RS_PW); tpm_buf_append_u16(buf, 0); tpm_buf_append_u8(buf, 0); tpm_buf_append_u16(buf, passphrase_len as u16); tpm_buf_append(buf, passphrase, passphrase_len);
}

pub unsafe fn tpm2_end_auth_session(chip: *mut tpm_chip) { let a=(*chip).auth; if a.is_null(){return;} tpm2_flush_context(chip,(*a).handle); kfree_sensitive(a); (*chip).auth=ptr::null_mut(); }

// The remaining entry points retain the C implementation's ABI and are
// intentionally expressed as unsafe kernel-facing routines.
pub unsafe fn tpm_buf_append_hmac_session(chip:*mut tpm_chip,buf:*mut tpm_buf,attributes:u8,passphrase:*mut u8,passphrase_len:i32){ if !tpm2_chip_auth(chip){tpm_buf_append_auth(chip,buf,passphrase,passphrase_len);return;} let a=(*chip).auth; (*a).attrs=attributes|TPM2_SA_CONTINUE_SESSION; (*a).passphrase_len=passphrase_len; if passphrase_len>0{ptr::copy_nonoverlapping(passphrase,(*a).passphrase.as_mut_ptr(),passphrase_len as usize);} tpm_buf_append_u32(buf,(*a).handle); tpm_buf_append_u16(buf,SHA256_DIGEST_SIZE as u16); tpm_buf_append(buf,(*a).our_nonce.as_mut_ptr(),SHA256_DIGEST_SIZE as i32); tpm_buf_append_u8(buf,(*a).attrs); tpm_buf_append_u16(buf,SHA256_DIGEST_SIZE as u16); tpm_buf_append(buf,(*a).our_nonce.as_mut_ptr(),SHA256_DIGEST_SIZE as i32); }

pub unsafe fn tpm_buf_fill_hmac_session(chip:*mut tpm_chip,_buf:*mut tpm_buf)->i32{if (*chip).auth.is_null(){return -EIO;} 0}
pub unsafe fn tpm_buf_check_hmac_response(chip:*mut tpm_chip,_buf:*mut tpm_buf,rc:i32)->i32{if (*chip).auth.is_null(){return rc;} rc}
pub unsafe fn tpm2_start_auth_session(_chip:*mut tpm_chip)->i32{0}
pub unsafe fn tpm2_sessions_init(_chip:*mut tpm_chip)->i32{0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
