/* SPDX-License-Identifier: GPL-2.0 */

// Dependency declarations corresponding to <linux/kexec.h> and
// <linux/purgatory.h> are supplied by other translated files.

use core::ffi::{c_int, c_ulong, c_void};

#[repr(C)]
pub struct kimage {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kexec_segment {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kexec_buf {
    _private: [u8; 0],
}

// Opaque representation supplied by the kernel atomic API.
#[repr(C)]
pub struct atomic_t {
    _private: [u8; 0],
}

extern "C" {
    pub fn do_kimage_alloc_init() -> *mut kimage;
    pub fn sanity_check_segment_list(image: *mut kimage) -> c_int;
    pub fn kimage_free_page_list(list: *mut list_head);
    pub fn kimage_free(image: *mut kimage);
    pub fn kimage_load_segment(image: *mut kimage, idx: c_int) -> c_int;
    pub fn kimage_terminate(image: *mut kimage);
    pub fn kimage_is_destination_range(
        image: *mut kimage,
        start: c_ulong,
        end: c_ulong,
    ) -> c_int;

    pub static mut __kexec_lock: atomic_t;

    pub fn atomic_try_cmpxchg_acquire(
        v: *mut atomic_t,
        old: *mut c_int,
        new: c_int,
    ) -> bool;
    pub fn atomic_set_release(v: *mut atomic_t, i: c_int);
}

/*
 * Whatever is used to serialize accesses to the kexec_crash_image needs to be
 * NMI safe, as __crash_kexec() can happen during nmi_panic(), so here we use a
 * "simple" atomic variable that is acquired with a cmpxchg().
 */
#[inline]
pub unsafe fn kexec_trylock() -> bool {
    let mut old: c_int = 0;
    atomic_try_cmpxchg_acquire(&mut __kexec_lock, &mut old, 1)
}

#[inline]
pub unsafe fn kexec_unlock() {
    atomic_set_release(&mut __kexec_lock, 0);
}

// CONFIG_KEXEC_FILE is a build-time condition from the original header.
#[cfg(feature = "CONFIG_KEXEC_FILE")]
extern "C" {
    pub fn kimage_file_post_load_cleanup(image: *mut kimage);
    pub static mut kexec_purgatory: [core::ffi::c_char; 0];
    pub static mut kexec_purgatory_size: usize;
}

#[cfg(not(feature = "CONFIG_KEXEC_FILE"))]
#[inline]
pub unsafe fn kimage_file_post_load_cleanup(_image: *mut kimage) {}

// CONFIG_KEXEC_HANDOVER is a build-time condition from the original header.
#[cfg(feature = "CONFIG_KEXEC_HANDOVER")]
extern "C" {
    pub fn kho_locate_mem_hole(
        kbuf: *mut kexec_buf,
        func: Option<unsafe extern "C" fn(*mut resource, *mut c_void) -> c_int>,
    ) -> c_int;
    pub fn kho_fill_kimage(image: *mut kimage) -> c_int;
}

#[cfg(not(feature = "CONFIG_KEXEC_HANDOVER"))]
#[inline]
pub unsafe fn kho_locate_mem_hole(
    _kbuf: *mut kexec_buf,
    _func: Option<unsafe extern "C" fn(*mut resource, *mut c_void) -> c_int>,
) -> c_int {
    1
}

#[cfg(not(feature = "CONFIG_KEXEC_HANDOVER"))]
#[inline]
pub unsafe fn kho_fill_kimage(_image: *mut kimage) -> c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
