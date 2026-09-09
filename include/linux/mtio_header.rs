/* SPDX-License-Identifier: GPL-2.0 */

// Translated from Linux compatibility header.
// Dependencies supplied by the surrounding translation unit:
// `mtget`, `mtpos`, `in_compat_syscall`, `copy_to_user`, `put_user`, and
// `EFAULT`.

#[repr(C)]
pub struct mtget32 {
    pub mt_type: i32,
    pub mt_resid: i32,
    pub mt_dsreg: i32,
    pub mt_gstat: i32,
    pub mt_erreg: i32,
    pub mt_fileno: i32,
    pub mt_blkno: i32,
}

// _IOR('m', 2, struct mtget32)
pub const MTIOCGET32: usize = 0x801c6d02;

#[repr(C)]
pub struct mtpos32 {
    pub mt_blkno: i32,
}

// _IOR('m', 3, struct mtpos32)
pub const MTIOCPOS32: usize = 0x80046d03;

pub unsafe fn put_user_mtget(u: *mut core::ffi::c_void, k: *mut mtget) -> i32 {
    let k32 = mtget32 {
        mt_type: (*k).mt_type,
        mt_resid: (*k).mt_resid,
        mt_dsreg: (*k).mt_dsreg,
        mt_gstat: (*k).mt_gstat,
        mt_erreg: (*k).mt_erreg,
        mt_fileno: (*k).mt_fileno,
        mt_blkno: (*k).mt_blkno,
    };
    let ret: i32;

    if in_compat_syscall() {
        ret = copy_to_user(
            u,
            &k32 as *const mtget32 as *const core::ffi::c_void,
            core::mem::size_of::<mtget32>(),
        );
    } else {
        ret = copy_to_user(
            u,
            k as *const mtget as *const core::ffi::c_void,
            core::mem::size_of::<mtget>(),
        );
    }

    if ret != 0 { -EFAULT } else { 0 }
}

pub unsafe fn put_user_mtpos(u: *mut core::ffi::c_void, k: *mut mtpos) -> i32 {
    if in_compat_syscall() {
        put_user((*k).mt_blkno, u as *mut u32)
    } else {
        put_user((*k).mt_blkno, u as *mut libc::c_long)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
