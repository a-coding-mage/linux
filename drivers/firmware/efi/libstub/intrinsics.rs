// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding EFI stub environment are intentionally
// left as external symbols, matching the original C translation unit.

#[cfg(feature = "CONFIG_KASAN")]
extern "C" {
    pub fn __memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    pub fn __memmove(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, count: usize) -> *mut core::ffi::c_void;
    pub fn __memset(s: *mut core::ffi::c_void, c: core::ffi::c_int, count: usize) -> *mut core::ffi::c_void;
}

unsafe fn efistub_memmove(dst: *mut u8, src: *const u8, len: usize) -> *mut core::ffi::c_void {
    if src > dst as *const u8 || (dst as *const u8) >= src.add(len) {
        for i in 0..len {
            *dst.add(i) = *src.add(i);
        }
    } else {
        let mut i = len as isize - 1;
        while i >= 0 {
            *dst.offset(i) = *src.offset(i);
            i -= 1;
        }
    }

    dst as *mut core::ffi::c_void
}

unsafe fn efistub_memset(dst: *mut core::ffi::c_void, c: core::ffi::c_int, mut len: usize) -> *mut core::ffi::c_void {
    let mut d = dst as *mut u8;
    while len != 0 {
        *d = c as u8;
        d = d.add(1);
        len -= 1;
    }

    dst
}

pub unsafe fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: usize) -> *mut core::ffi::c_void {
    if efi_table_attr(efi_system_table, boottime).is_null() {
        return efistub_memmove(dst as *mut u8, src as *const u8, len);
    }

    efi_bs_call(copy_mem, dst, src, len);
    dst
}

pub use memcpy as memmove;

pub unsafe fn memset(dst: *mut core::ffi::c_void, c: core::ffi::c_int, len: usize) -> *mut core::ffi::c_void {
    if efi_table_attr(efi_system_table, boottime).is_null() {
        return efistub_memset(dst, c, len);
    }

    efi_bs_call(set_mem, dst, len, c & u8::MAX as core::ffi::c_int);
    dst
}

/// memcmp - Compare two areas of memory
/// @cs: One area of memory
/// @ct: Another area of memory
/// @count: The size of the area.
pub unsafe fn memcmp(cs: *const core::ffi::c_void, ct: *const core::ffi::c_void, mut count: usize) -> core::ffi::c_int {
    let mut su1 = cs as *const u8;
    let mut su2 = ct as *const u8;
    let mut res: core::ffi::c_int = 0;

    while count > 0 {
        res = *su1 as core::ffi::c_int - *su2 as core::ffi::c_int;
        if res != 0 {
            break;
        }
        su1 = su1.add(1);
        su2 = su2.add(1);
        count -= 1;
    }
    res
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
