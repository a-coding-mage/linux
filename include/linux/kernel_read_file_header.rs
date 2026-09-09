/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the corresponding kernel headers. */
use core::ffi::c_char;

/* This is a list of *what* is being read, not *how* nor *where*. */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum kernel_read_file_id {
    READING_UNKNOWN,
    READING_FIRMWARE,
    READING_MODULE,
    READING_KEXEC_IMAGE,
    READING_KEXEC_INITRAMFS,
    READING_POLICY,
    READING_X509_CERTIFICATE,
    READING_MODULE_COMPRESSED,
    READING_MAX_ID,
}

pub static kernel_read_file_str: [*const c_char; 9] = [
    b"unknown\0".as_ptr() as *const c_char,
    b"firmware\0".as_ptr() as *const c_char,
    b"kernel-module\0".as_ptr() as *const c_char,
    b"kexec-image\0".as_ptr() as *const c_char,
    b"kexec-initramfs\0".as_ptr() as *const c_char,
    b"security-policy\0".as_ptr() as *const c_char,
    b"x509-certificate\0".as_ptr() as *const c_char,
    b"kernel-module-compressed\0".as_ptr() as *const c_char,
    b"\0".as_ptr() as *const c_char,
];

#[inline]
pub unsafe fn kernel_read_file_id_str(id: kernel_read_file_id) -> *const c_char {
    if (id as u32) >= kernel_read_file_id::READING_MAX_ID as u32 {
        return kernel_read_file_str[kernel_read_file_id::READING_UNKNOWN as usize];
    }

    kernel_read_file_str[id as usize]
}

extern "C" {
    pub fn kernel_read_file(
        file: *mut crate::file,
        offset: crate::loff_t,
        buf: *mut *mut core::ffi::c_void,
        buf_size: usize,
        file_size: *mut usize,
        id: kernel_read_file_id,
    ) -> crate::ssize_t;

    pub fn kernel_read_file_from_path(
        path: *const c_char,
        offset: crate::loff_t,
        buf: *mut *mut core::ffi::c_void,
        buf_size: usize,
        file_size: *mut usize,
        id: kernel_read_file_id,
    ) -> crate::ssize_t;

    pub fn kernel_read_file_from_path_initns(
        path: *const c_char,
        offset: crate::loff_t,
        buf: *mut *mut core::ffi::c_void,
        buf_size: usize,
        file_size: *mut usize,
        id: kernel_read_file_id,
    ) -> crate::ssize_t;

    pub fn kernel_read_file_from_fd(
        fd: i32,
        offset: crate::loff_t,
        buf: *mut *mut core::ffi::c_void,
        buf_size: usize,
        file_size: *mut usize,
        id: kernel_read_file_id,
    ) -> crate::ssize_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
