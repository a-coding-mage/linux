// SPDX-License-Identifier: GPL-2.0
/* Rust translation of fname.c.  Kernel-provided types and functions are
 * intentionally referenced as external dependencies. */

const FSCRYPT_FNAME_MIN_MSG_LEN: usize = 16;

#[repr(C)]
pub struct fscrypt_nokey_name {
    pub dirhash: [u32; 2],
    pub bytes: [u8; 149],
    pub sha256: [u8; SHA256_DIGEST_SIZE],
}

const FSCRYPT_NOKEY_NAME_MAX: usize = 8 + 149 + SHA256_DIGEST_SIZE;
const FSCRYPT_NOKEY_NAME_MAX_ENCODED: usize = BASE64_CHARS!(FSCRYPT_NOKEY_NAME_MAX);

#[inline]
unsafe fn fscrypt_is_dot_dotdot(str_: *const qstr) -> bool {
    name_is_dot_dotdot((*str_).name, (*str_).len)
}

pub unsafe fn fscrypt_fname_encrypt(inode: *const inode, iname: *const qstr,
                                    out: *mut u8, olen: u32) -> i32 {
    let ci = fscrypt_get_inode_info_raw(inode);
    let tfm = (*ci).ci_enc_key.tfm;
    let mut req = SYNC_SKCIPHER_REQUEST_ON_STACK!(tfm);
    let mut iv = fscrypt_iv::default();
    let mut sg = scatterlist::default();
    if WARN_ON_ONCE!(olen < (*iname).len) { return -ENOBUFS; }
    core::ptr::copy_nonoverlapping((*iname).name, out, (*iname).len as usize);
    core::ptr::write_bytes(out.add((*iname).len as usize), 0,
                           (olen - (*iname).len) as usize);
    fscrypt_generate_iv(&mut iv, 0, ci);
    skcipher_request_set_callback(req, CRYPTO_TFM_REQ_MAY_BACKLOG | CRYPTO_TFM_REQ_MAY_SLEEP,
                                  core::ptr::null_mut(), core::ptr::null_mut());
    sg_init_one(&mut sg, out, olen);
    skcipher_request_set_crypt(req, &mut sg, &mut sg, olen, &mut iv);
    let err = crypto_skcipher_encrypt(req);
    if err != 0 { fscrypt_err(inode, "Filename encryption failed: %d", err); }
    err
}

unsafe fn fname_decrypt(inode: *const inode, iname: *const fscrypt_str,
                        oname: *mut fscrypt_str) -> i32 {
    let ci = fscrypt_get_inode_info_raw(inode);
    let tfm = (*ci).ci_enc_key.tfm;
    let mut req = SYNC_SKCIPHER_REQUEST_ON_STACK!(tfm);
    let mut iv = fscrypt_iv::default();
    let mut src_sg = scatterlist::default();
    let mut dst_sg = scatterlist::default();
    fscrypt_generate_iv(&mut iv, 0, ci);
    skcipher_request_set_callback(req, CRYPTO_TFM_REQ_MAY_BACKLOG | CRYPTO_TFM_REQ_MAY_SLEEP,
                                  core::ptr::null_mut(), core::ptr::null_mut());
    sg_init_one(&mut src_sg, (*iname).name, (*iname).len);
    sg_init_one(&mut dst_sg, (*oname).name, (*oname).len);
    skcipher_request_set_crypt(req, &mut src_sg, &mut dst_sg, (*iname).len, &mut iv);
    let err = crypto_skcipher_decrypt(req);
    if err != 0 { fscrypt_err(inode, "Filename decryption failed: %d", err); return err; }
    (*oname).len = strnlen((*oname).name, (*iname).len);
    0
}

pub unsafe fn __fscrypt_fname_encrypted_size(policy: *const fscrypt_policy,
                                             orig_len: u32, max_len: u32,
                                             encrypted_len_ret: *mut u32) -> bool {
    let padding = 4u32 << (fscrypt_policy_flags(policy) & FSCRYPT_POLICY_FLAGS_PAD_MASK);
    if orig_len > max_len { return false; }
    let mut encrypted_len = core::cmp::max(orig_len, FSCRYPT_FNAME_MIN_MSG_LEN as u32);
    encrypted_len = (encrypted_len + padding - 1) / padding * padding;
    *encrypted_len_ret = core::cmp::min(encrypted_len, max_len);
    true
}

pub unsafe fn fscrypt_fname_encrypted_size(inode: *const inode, orig_len: u32,
                                           max_len: u32, ret: *mut u32) -> bool {
    let ci = fscrypt_get_inode_info_raw(inode);
    __fscrypt_fname_encrypted_size(&(*ci).ci_policy, orig_len, max_len, ret)
}

pub unsafe fn fscrypt_fname_alloc_buffer(max_encrypted_len: u32,
                                         crypto_str: *mut fscrypt_str) -> i32 {
    let max_presented_len = core::cmp::max(FSCRYPT_NOKEY_NAME_MAX_ENCODED as u32, max_encrypted_len);
    (*crypto_str).name = kmalloc(max_presented_len + 1, GFP_NOFS);
    if (*crypto_str).name.is_null() { return -ENOMEM; }
    (*crypto_str).len = max_presented_len;
    0
}

pub unsafe fn fscrypt_fname_free_buffer(crypto_str: *mut fscrypt_str) {
    if crypto_str.is_null() { return; }
    kfree((*crypto_str).name);
    (*crypto_str).name = core::ptr::null_mut();
}

pub unsafe fn fscrypt_fname_disk_to_usr(inode: *const inode, hash: u32, minor_hash: u32,
                                        iname: *const fscrypt_str, oname: *mut fscrypt_str) -> i32 {
    let qname = FSTR_TO_QSTR!(iname);
    let mut nokey_name = fscrypt_nokey_name { dirhash: [0; 2], bytes: [0; 149], sha256: [0; SHA256_DIGEST_SIZE] };
    let size: usize;
    if fscrypt_is_dot_dotdot(&qname) {
        (*oname).name.write(b'.'); (*oname).name.add((*iname).len as usize - 1).write(b'.');
        (*oname).len = (*iname).len; return 0;
    }
    if (*iname).len < FSCRYPT_FNAME_MIN_MSG_LEN as u32 { return -EUCLEAN; }
    if fscrypt_has_encryption_key(inode) { return fname_decrypt(inode, iname, oname); }
    nokey_name.dirhash = [hash, minor_hash];
    if (*iname).len as usize <= nokey_name.bytes.len() {
        core::ptr::copy_nonoverlapping((*iname).name, nokey_name.bytes.as_mut_ptr(), (*iname).len as usize);
        size = 8 + (*iname).len as usize;
    } else {
        core::ptr::copy_nonoverlapping((*iname).name, nokey_name.bytes.as_mut_ptr(), nokey_name.bytes.len());
        sha256((*iname).name.add(nokey_name.bytes.len()), (*iname).len - nokey_name.bytes.len() as u32, nokey_name.sha256.as_mut_ptr());
        size = FSCRYPT_NOKEY_NAME_MAX;
    }
    (*oname).len = base64_encode(nokey_name.as_ptr() as *const u8, size, (*oname).name, false, BASE64_URLSAFE);
    0
}

pub unsafe fn fscrypt_fname_siphash(dir: *const inode, name: *const qstr) -> u64 {
    let ci = fscrypt_get_inode_info_raw(dir);
    WARN_ON_ONCE!(!(*ci).ci_dirhash_key_initialized);
    siphash((*name).name, (*name).len, &(*ci).ci_dirhash_key)
}

pub unsafe fn fscrypt_setup_filename(dir: *mut inode, iname: *const qstr,
                                     lookup: i32, fname: *mut fscrypt_name) -> i32 {
    let mut ret: i32;
    core::ptr::write_bytes(fname as *mut u8, 0, core::mem::size_of::<fscrypt_name>());
    (*fname).usr_fname = iname;
    if !IS_ENCRYPTED!(dir) || fscrypt_is_dot_dotdot(iname) {
        (*fname).disk_name.name = (*iname).name as *mut u8;
        (*fname).disk_name.len = (*iname).len; return 0;
    }
    ret = fscrypt_get_encryption_info(dir, lookup);
    if ret != 0 { return ret; }
    if fscrypt_has_encryption_key(dir) {
        if !fscrypt_fname_encrypted_size(dir, (*iname).len, NAME_MAX, &mut (*fname).crypto_buf.len) { return -ENAMETOOLONG; }
        (*fname).crypto_buf.name = kmalloc((*fname).crypto_buf.len, GFP_NOFS);
        if (*fname).crypto_buf.name.is_null() { return -ENOMEM; }
        ret = fscrypt_fname_encrypt(dir, iname, (*fname).crypto_buf.name, (*fname).crypto_buf.len);
        if ret != 0 { kfree((*fname).crypto_buf.name); return ret; }
        (*fname).disk_name = (*fname).crypto_buf; return 0;
    }
    if lookup == 0 { return -ENOKEY; }
    (*fname).is_nokey_name = true;
    if (*iname).len as usize > FSCRYPT_NOKEY_NAME_MAX_ENCODED { return -ENOENT; }
    (*fname).crypto_buf.name = kmalloc(FSCRYPT_NOKEY_NAME_MAX, GFP_KERNEL);
    if (*fname).crypto_buf.name.is_null() { return -ENOMEM; }
    ret = base64_decode((*iname).name, (*iname).len, (*fname).crypto_buf.name, false, BASE64_URLSAFE) as i32;
    if ret < 9 || (ret as usize > 8 + 149 && ret as usize != FSCRYPT_NOKEY_NAME_MAX) { kfree((*fname).crypto_buf.name); return -ENOENT; }
    (*fname).crypto_buf.len = ret as u32;
    let nk = (*fname).crypto_buf.name as *const fscrypt_nokey_name;
    (*fname).hash = (*nk).dirhash[0]; (*fname).minor_hash = (*nk).dirhash[1];
    if ret as usize != FSCRYPT_NOKEY_NAME_MAX { (*fname).disk_name.name = (*nk).bytes.as_ptr() as *mut u8; (*fname).disk_name.len = ret as u32 - 8; }
    0
}

pub unsafe fn fscrypt_match_name(fname: *const fscrypt_name, de_name: *const u8, de_name_len: u32) -> bool {
    let nk = (*fname).crypto_buf.name as *const fscrypt_nokey_name;
    if !(*fname).disk_name.name.is_null() {
        return de_name_len == (*fname).disk_name.len && memcmp(de_name, (*fname).disk_name.name, de_name_len) == 0;
    }
    if de_name_len <= 149 || memcmp(de_name, (*nk).bytes.as_ptr(), 149) != 0 { return false; }
    let mut digest = [0u8; SHA256_DIGEST_SIZE];
    sha256(de_name.add(149), de_name_len - 149, digest.as_mut_ptr());
    memcmp(digest.as_ptr(), (*nk).sha256.as_ptr(), SHA256_DIGEST_SIZE as u32) == 0
}

pub unsafe fn fscrypt_d_revalidate(dir: *mut inode, _name: *const qstr,
                                    dentry: *mut dentry, flags: u32) -> i32 {
    if (*dentry).d_flags & DCACHE_NOKEY_NAME == 0 { return 1; }
    if flags & LOOKUP_RCU != 0 { return -ECHILD; }
    let err = fscrypt_get_encryption_info(dir, true);
    if err < 0 { return err; }
    if fscrypt_has_encryption_key(dir) { 0 } else { 1 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
