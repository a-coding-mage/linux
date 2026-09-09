/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2024 Christophe Leroy <christophe.leroy@csgroup.eu>, CS GROUP France
 */

// C dependency: <asm/vdso_datapage.h>
// The build provides `vdso_rng_data` and `__NR_getrandom`.

/// Equivalent to the PowerPC `do_syscall_3` inline function.
#[inline(always)]
pub unsafe fn do_syscall_3(
    _r0: usize,
    _r3: usize,
    _r4: usize,
    _r5: usize,
) -> i32 {
    let mut r0 = _r0;
    let mut r3 = _r3;
    let mut r4 = _r4;
    let mut r5 = _r5;
    let mut ret: i32;

    core::arch::asm!(
        "sc",
        "bns+ 1f",
        "neg {ret}, {ret}",
        "1:",
        ret = lateout("r3") ret,
        inout("r0") r0,
        inout("r3") r3,
        inout("r4") r4,
        inout("r5") r5,
        lateout("r6") _,
        lateout("r7") _,
        lateout("r8") _,
        lateout("r9") _,
        lateout("r10") _,
        lateout("r11") _,
        lateout("r12") _,
        options(nostack),
    );

    ret
}

/**
 * getrandom_syscall - Invoke the getrandom() syscall.
 * @buffer:      Destination buffer to fill with random bytes.
 * @len:         Size of @buffer in bytes.
 * @flags:       Zero or more GRND_* flags.
 * Returns:      The number of bytes written to @buffer, or a negative value indicating an error.
 */
#[inline(always)]
pub unsafe fn getrandom_syscall(
    buffer: *mut core::ffi::c_void,
    len: usize,
    flags: u32,
) -> isize {
    do_syscall_3(
        __NR_getrandom as usize,
        buffer as usize,
        len,
        flags as usize,
    ) as isize
}

#[inline(always)]
pub unsafe fn __arch_get_vdso_u_rng_data() -> *const vdso_rng_data {
    let data: *const vdso_rng_data;

    core::arch::asm!(
        "bcl 20, 31, .+4",
        "0: mflr {data}",
        "addis {data}, {data}, (vdso_u_rng_data - 0b)@ha",
        "addi {data}, {data}, (vdso_u_rng_data - 0b)@l",
        data = lateout(reg) data,
        clobber_abi("C"),
    );

    data
}

pub use __arch_get_vdso_u_rng_data as __arch_get_vdso_u_rng_data;

extern "C" {
    pub fn __c_kernel_getrandom(
        buffer: *mut core::ffi::c_void,
        len: usize,
        flags: u32,
        opaque_state: *mut core::ffi::c_void,
        opaque_len: usize,
    ) -> isize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
