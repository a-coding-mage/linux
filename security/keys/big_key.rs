// SPDX-License-Identifier: GPL-2.0-or-later
/* Large capacity key type
 *
 * Copyright (C) 2017-2020 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
 * Copyright (C) 2013 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// pr_fmt(fmt) "big_key: "fmt
// C dependencies: linux/init.h, linux/seq_file.h, linux/file.h,
// linux/shmem_fs.h, linux/err.h, linux/random.h, keys/user-type.h,
// keys/big_key-type.h, crypto/chacha20poly1305.h

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

type u8 = core::ffi::c_uchar;
type size_t = usize;
type ssize_t = isize;
type loff_t = i64;

const CHACHA20POLY1305_AUTHTAG_SIZE: size_t = 16;
const CHACHA20POLY1305_KEY_SIZE: size_t = 32;
const GFP_KERNEL: c_ulong = 0;
const EMPTY_VMA_FLAGS: c_ulong = 0;
const O_RDONLY: c_int = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EIO: c_int = 5;
const EBADMSG: c_int = 74;

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vfsmount {
    _private: [u8; 0],
}

#[repr(C)]
pub struct path {
    pub mnt: *mut vfsmount,
    pub dentry: *mut dentry,
}

#[repr(C)]
pub struct file {
    pub f_path: path,
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub union key_payload_data {
    pub data: [*mut c_void; 4],
}

#[repr(C)]
pub struct key_preparsed_payload {
    pub payload: key_payload_data,
    pub data: *const c_void,
    pub datalen: size_t,
    pub quotalen: size_t,
}

#[repr(C)]
pub struct key {
    pub payload: key_payload_data,
    pub description: *const c_char,
}

#[repr(C)]
pub struct key_type {
    pub name: *const c_char,
    pub preparse: Option<unsafe extern "C" fn(*mut key_preparsed_payload) -> c_int>,
    pub free_preparse: Option<unsafe extern "C" fn(*mut key_preparsed_payload)>,
    pub instantiate:
        Option<unsafe extern "C" fn(*mut key, *mut key_preparsed_payload) -> c_int>,
    pub revoke: Option<unsafe extern "C" fn(*mut key)>,
    pub destroy: Option<unsafe extern "C" fn(*mut key)>,
    pub describe: Option<unsafe extern "C" fn(*const key, *mut seq_file)>,
    pub read: Option<unsafe extern "C" fn(*const key, *mut c_char, size_t) -> c_long>,
    pub update: Option<unsafe extern "C" fn(*mut key, *mut key_preparsed_payload) -> c_int>,
}

/*
 * Layout of key payload words.
 */
#[repr(C)]
pub struct big_key_payload {
    pub data: *mut u8,
    pub path: path,
    pub length: size_t,
}

unsafe fn to_big_key_payload(payload: key_payload_data) -> *mut big_key_payload {
    unsafe { payload.data.as_ptr() as *mut big_key_payload }
}

/*
 * If the data is under this limit, there's no point creating a shm file to
 * hold it as the permanently resident metadata for the shmem fs will be at
 * least as large as the data.
 */
// Mirrors: sizeof(struct inode) + sizeof(struct dentry). The concrete layouts
// are supplied by external kernel headers in the original C translation unit.
const BIG_KEY_FILE_THRESHOLD: size_t =
    core::mem::size_of::<inode>() + core::mem::size_of::<dentry>();

unsafe extern "C" {
    fn generic_key_instantiate(key: *mut key, prep: *mut key_preparsed_payload) -> c_int;
    fn key_payload_reserve(key: *mut key, datalen: size_t) -> c_int;
    fn key_is_positive(key: *const key) -> bool;
    fn vfs_truncate(path: *mut path, length: loff_t) -> c_int;
    fn seq_puts(m: *mut seq_file, s: *const c_char) -> c_int;
    fn seq_printf(m: *mut seq_file, fmt: *const c_char, ...) -> c_int;
    fn register_key_type(ktype: *mut key_type) -> c_int;
    fn kvmalloc(size: size_t, flags: c_ulong) -> *mut u8;
    fn kvfree_sensitive(addr: *const c_void, len: size_t);
    fn kmalloc(size: size_t, flags: c_ulong) -> *mut c_void;
    fn kfree_sensitive(addr: *const c_void);
    fn get_random_bytes_wait(buf: *mut c_void, nbytes: c_int) -> c_int;
    fn chacha20poly1305_encrypt(
        dst: *mut u8,
        src: *const c_void,
        src_len: size_t,
        ad: *const c_void,
        ad_len: size_t,
        nonce: u64,
        key: *const u8,
    );
    fn chacha20poly1305_decrypt(
        dst: *mut u8,
        src: *const u8,
        src_len: size_t,
        ad: *const c_void,
        ad_len: size_t,
        nonce: u64,
        key: *const u8,
    ) -> bool;
    fn shmem_kernel_file_setup(
        name: *const c_char,
        size: loff_t,
        flags: c_ulong,
    ) -> *mut file;
    fn kernel_write(file: *mut file, buf: *const u8, count: size_t, pos: *mut loff_t) -> ssize_t;
    fn kernel_read(file: *mut file, buf: *mut u8, count: size_t, pos: *mut loff_t) -> ssize_t;
    fn path_get(path: *mut path);
    fn path_put(path: *mut path);
    fn fput(file: *mut file);
    fn dentry_open(path: *mut path, flags: c_int, cred: *const c_void) -> *mut file;
    fn current_cred() -> *const c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_long;
}

#[inline]
fn unlikely(x: c_int) -> bool {
    x != 0
}

/*
 * big_key defined keys take an arbitrary string as the description and an
 * arbitrary blob of data as the payload
 */
#[unsafe(no_mangle)]
pub static mut key_type_big_key: key_type = key_type {
    name: c"big_key".as_ptr(),
    preparse: Some(big_key_preparse),
    free_preparse: Some(big_key_free_preparse),
    instantiate: Some(generic_key_instantiate),
    revoke: Some(big_key_revoke),
    destroy: Some(big_key_destroy),
    describe: Some(big_key_describe),
    read: Some(big_key_read),
    update: Some(big_key_update),
};

/*
 * Preparse a big key
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn big_key_preparse(prep: *mut key_preparsed_payload) -> c_int {
    unsafe {
        let payload = to_big_key_payload((*prep).payload);
        let mut file: *mut file;
        let mut buf: *mut u8;
        let mut enckey: *mut u8;
        let written: ssize_t;
        let datalen: size_t = (*prep).datalen;
        let enclen: size_t = datalen.wrapping_add(CHACHA20POLY1305_AUTHTAG_SIZE);
        let mut ret: c_int;

        // BUILD_BUG_ON(sizeof(*payload) != sizeof(prep->payload.data));

        if datalen == 0 || datalen > 1024 * 1024 || (*prep).data.is_null() {
            return -EINVAL;
        }

        /* Set an arbitrary quota */
        (*prep).quotalen = 16;

        (*payload).length = datalen;

        if datalen > BIG_KEY_FILE_THRESHOLD {
            /* Create a shmem file to store the data in.  This will permit the data
             * to be swapped out if needed.
             *
             * File content is stored encrypted with randomly generated key.
             * Since the key is random for each file, we can set the nonce
             * to zero, provided we never define a ->update() call.
             */
            let mut pos: loff_t = 0;

            buf = kvmalloc(enclen, GFP_KERNEL);
            if buf.is_null() {
                return -ENOMEM;
            }

            /* generate random key */
            enckey = kmalloc(CHACHA20POLY1305_KEY_SIZE, GFP_KERNEL) as *mut u8;
            if enckey.is_null() {
                ret = -ENOMEM;
                goto_error(buf, enclen, ret)
            } else {
                ret = get_random_bytes_wait(enckey as *mut c_void, CHACHA20POLY1305_KEY_SIZE as c_int);
                if unlikely(ret) {
                    kfree_sensitive(enckey as *const c_void);
                    kvfree_sensitive(buf as *const c_void, enclen);
                    return ret;
                }

                /* encrypt data */
                chacha20poly1305_encrypt(
                    buf,
                    (*prep).data,
                    datalen,
                    core::ptr::null(),
                    0,
                    0,
                    enckey,
                );

                /* save aligned data to file */
                file = shmem_kernel_file_setup(c"".as_ptr(), enclen as loff_t, EMPTY_VMA_FLAGS);
                if IS_ERR(file as *const c_void) {
                    ret = PTR_ERR(file as *const c_void) as c_int;
                    kfree_sensitive(enckey as *const c_void);
                    kvfree_sensitive(buf as *const c_void, enclen);
                    return ret;
                }

                written = kernel_write(file, buf, enclen, &mut pos);
                if written != enclen as ssize_t {
                    ret = written as c_int;
                    if written >= 0 {
                        ret = -EIO;
                    }
                    fput(file);
                    kfree_sensitive(enckey as *const c_void);
                    kvfree_sensitive(buf as *const c_void, enclen);
                    return ret;
                }

                /* Pin the mount and dentry to the key so that we can open it again
                 * later
                 */
                (*payload).data = enckey;
                (*payload).path = (*file).f_path;
                path_get(&mut (*payload).path);
                fput(file);
                kvfree_sensitive(buf as *const c_void, enclen);
            }
        } else {
            /* Just store the data in a buffer */
            let data = kmalloc(datalen, GFP_KERNEL);

            if data.is_null() {
                return -ENOMEM;
            }

            (*payload).data = data as *mut u8;
            memcpy(data, (*prep).data, (*prep).datalen);
        }
        return 0;
    }
}

unsafe fn goto_error(buf: *mut u8, enclen: size_t, ret: c_int) -> c_int {
    unsafe {
        kvfree_sensitive(buf as *const c_void, enclen);
        ret
    }
}

/*
 * Clear preparsement.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn big_key_free_preparse(prep: *mut key_preparsed_payload) {
    unsafe {
        let payload = to_big_key_payload((*prep).payload);

        if (*prep).datalen > BIG_KEY_FILE_THRESHOLD {
            path_put(&mut (*payload).path);
        }
        kfree_sensitive((*payload).data as *const c_void);
    }
}

/*
 * dispose of the links from a revoked keyring
 * - called with the key sem write-locked
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn big_key_revoke(key: *mut key) {
    unsafe {
        let payload = to_big_key_payload((*key).payload);

        /* clear the quota */
        key_payload_reserve(key, 0);
        if key_is_positive(key) && (*payload).length > BIG_KEY_FILE_THRESHOLD {
            vfs_truncate(&mut (*payload).path, 0);
        }
    }
}

/*
 * dispose of the data dangling from the corpse of a big_key key
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn big_key_destroy(key: *mut key) {
    unsafe {
        let payload = to_big_key_payload((*key).payload);

        if (*payload).length > BIG_KEY_FILE_THRESHOLD {
            path_put(&mut (*payload).path);
            (*payload).path.mnt = core::ptr::null_mut();
            (*payload).path.dentry = core::ptr::null_mut();
        }
        kfree_sensitive((*payload).data as *const c_void);
        (*payload).data = core::ptr::null_mut();
    }
}

/*
 * Update a big key
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn big_key_update(
    key: *mut key,
    prep: *mut key_preparsed_payload,
) -> c_int {
    unsafe {
        let mut ret: c_int;

        ret = key_payload_reserve(key, (*prep).datalen);
        if ret < 0 {
            return ret;
        }

        if key_is_positive(key) {
            big_key_destroy(key);
        }

        return generic_key_instantiate(key, prep);
    }
}

/*
 * describe the big_key key
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn big_key_describe(key: *const key, m: *mut seq_file) {
    unsafe {
        let payload = to_big_key_payload((*key).payload);

        seq_puts(m, (*key).description);

        if key_is_positive(key) {
            seq_printf(
                m,
                c": %zu [%s]".as_ptr(),
                (*payload).length,
                if (*payload).length > BIG_KEY_FILE_THRESHOLD {
                    c"file".as_ptr()
                } else {
                    c"buff".as_ptr()
                },
            );
        }
    }
}

/*
 * read the key data
 * - the key's semaphore is read-locked
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn big_key_read(
    key: *const key,
    buffer: *mut c_char,
    buflen: size_t,
) -> c_long {
    unsafe {
        let payload = to_big_key_payload((*key).payload);
        let datalen: size_t = (*payload).length;
        let mut ret: c_long;

        if buffer.is_null() || buflen < datalen {
            return datalen as c_long;
        }

        if datalen > BIG_KEY_FILE_THRESHOLD {
            let mut file: *mut file;
            let mut buf: *mut u8;
            let enckey: *mut u8 = (*payload).data;
            let enclen: size_t = datalen.wrapping_add(CHACHA20POLY1305_AUTHTAG_SIZE);
            let mut pos: loff_t = 0;

            buf = kvmalloc(enclen, GFP_KERNEL);
            if buf.is_null() {
                return -ENOMEM as c_long;
            }

            file = dentry_open(&mut (*payload).path, O_RDONLY, current_cred());
            if IS_ERR(file as *const c_void) {
                ret = PTR_ERR(file as *const c_void);
                kvfree_sensitive(buf as *const c_void, enclen);
                return ret;
            }

            /* read file to kernel and decrypt */
            ret = kernel_read(file, buf, enclen, &mut pos) as c_long;
            if ret != enclen as c_long {
                if ret >= 0 {
                    ret = -EIO as c_long;
                }
                fput(file);
                kvfree_sensitive(buf as *const c_void, enclen);
                return ret;
            }

            ret = if chacha20poly1305_decrypt(
                buf,
                buf,
                enclen,
                core::ptr::null(),
                0,
                0,
                enckey,
            ) {
                0
            } else {
                -EBADMSG as c_long
            };
            if ret != 0 {
                fput(file);
                kvfree_sensitive(buf as *const c_void, enclen);
                return ret;
            }

            ret = datalen as c_long;

            /* copy out decrypted data */
            memcpy(buffer as *mut c_void, buf as *const c_void, datalen);

            fput(file);
            kvfree_sensitive(buf as *const c_void, enclen);
        } else {
            ret = datalen as c_long;
            memcpy(
                buffer as *mut c_void,
                (*payload).data as *const c_void,
                datalen,
            );
        }

        return ret;
    }
}

/*
 * Register key type
 */
unsafe extern "C" fn big_key_init() -> c_int {
    unsafe { register_key_type(&raw mut key_type_big_key) }
}

// late_initcall(big_key_init);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
