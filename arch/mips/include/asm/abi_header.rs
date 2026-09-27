/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2005, 06 by Ralf Baechle (ralf@linux-mips.org)
 * Copyright (C) 2005 MIPS Technologies, Inc.
 */

// Translated from the C header _ASM_ABI_H.

#[repr(C)]
pub struct mips_abi {
    pub setup_frame: unsafe extern "C" fn(
        sig_return: *mut kernel::ffi::c_void,
        ksig: *mut crate::ksignal,
        regs: *mut crate::pt_regs,
        set: *mut crate::sigset_t,
    ) -> kernel::ffi::c_int,
    pub setup_rt_frame: unsafe extern "C" fn(
        sig_return: *mut kernel::ffi::c_void,
        ksig: *mut crate::ksignal,
        regs: *mut crate::pt_regs,
        set: *mut crate::sigset_t,
    ) -> kernel::ffi::c_int,
    pub restart: kernel::ffi::c_ulong,

    pub off_sc_fpregs: kernel::ffi::c_uint,
    pub off_sc_fpc_csr: kernel::ffi::c_uint,
    pub off_sc_used_math: kernel::ffi::c_uint,

    pub vdso: *mut crate::mips_vdso_image,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
