/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2020, Sandipan Das, IBM Corp.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

/*
 * C dependencies removed from executable Rust:
 * - <sys/mman.h>
 * - "reg.h"
 * - "utils.h"
 */

pub type size_t = usize;
#[repr(C)]
pub struct siginfo_t {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn mfspr(reg: c_ulong) -> c_ulong;
    fn set_amr(amr: c_ulong);
    fn syscall(num: c_long, ...) -> c_long;
    fn using_hash_mmu(hash_mmu: *mut bool) -> c_int;
}

/*
 * Older versions of libc use the Intel-specific access rights.
 * Hence, override the definitions as they might be incorrect.
 */
pub const PKEY_DISABLE_ACCESS: c_ulong = 0x3;
pub const PKEY_DISABLE_WRITE: c_ulong = 0x2;
pub const PKEY_DISABLE_EXECUTE: c_ulong = 0x4;
pub const PKEY_UNRESTRICTED: c_ulong = 0x0;

/* Older versions of libc do not define this */
pub const SEGV_PKUERR: c_int = 4;

pub const SI_PKEY_OFFSET: usize = 0x20;

pub const __NR_pkey_mprotect: c_long = 386;
pub const __NR_pkey_alloc: c_long = 384;
pub const __NR_pkey_free: c_long = 385;

pub const NT_PPC_PKEY: c_ulong = 0x110;

pub const PKEY_BITS_PER_PKEY: c_ulong = 2;
pub const NR_PKEYS: c_ulong = 32;
pub const PKEY_BITS_MASK: c_ulong = (1u64 << PKEY_BITS_PER_PKEY) as c_ulong - 1;

pub const AMR_BITS_PER_PKEY: c_ulong = 2;
pub const PKEY_REG_BITS: c_ulong = core::mem::size_of::<u64>() as c_ulong * 8;

/* SPRN_AMR is supplied by "reg.h" in the original C source. */

pub const fn pkeyshift(pkey: c_ulong) -> c_ulong {
    PKEY_REG_BITS - ((pkey + 1) * AMR_BITS_PER_PKEY)
}

#[inline]
pub unsafe fn pkeyreg_get() -> c_ulong {
    unsafe { mfspr(SPRN_AMR) }
}

#[inline]
pub unsafe fn pkeyreg_set(amr: c_ulong) {
    unsafe { set_amr(amr) };
}

pub unsafe fn pkey_set_rights(pkey: c_int, rights: c_ulong) {
    let mut amr: c_ulong;
    let shift: c_ulong;

    shift = (NR_PKEYS - pkey as c_ulong - 1) * PKEY_BITS_PER_PKEY;
    amr = unsafe { pkeyreg_get() };
    amr &= !(PKEY_BITS_MASK << shift);
    amr |= (rights & PKEY_BITS_MASK) << shift;
    unsafe { pkeyreg_set(amr) };
}

pub unsafe fn sys_pkey_mprotect(addr: *mut c_void, len: size_t, prot: c_int, pkey: c_int) -> c_int {
    unsafe { syscall(__NR_pkey_mprotect, addr, len, prot, pkey) as c_int }
}

pub unsafe fn sys_pkey_alloc(flags: c_ulong, rights: c_ulong) -> c_int {
    unsafe { syscall(__NR_pkey_alloc, flags, rights) as c_int }
}

pub unsafe fn sys_pkey_free(pkey: c_int) -> c_int {
    unsafe { syscall(__NR_pkey_free, pkey) as c_int }
}

pub unsafe fn pkeys_unsupported() -> c_int {
    let mut hash_mmu: bool = false;
    let pkey: c_int;

    /* Protection keys are currently supported on Hash MMU only */
    FAIL_IF!(unsafe { using_hash_mmu(&mut hash_mmu as *mut bool) });
    SKIP_IF!(!hash_mmu);

    /* Check if the system call is supported */
    pkey = unsafe { sys_pkey_alloc(0, PKEY_UNRESTRICTED) };
    SKIP_IF!(pkey < 0);
    unsafe { sys_pkey_free(pkey) };

    0
}

pub unsafe fn siginfo_pkey(si: *mut siginfo_t) -> c_int {
    /*
     * In older versions of libc, siginfo_t does not have si_pkey as
     * a member.
     *
     * Original C condition:
     * #ifdef si_pkey
     *     return si->si_pkey;
     * #else
     *     return *((int *)(((char *) si) + SI_PKEY_OFFSET));
     * #endif
     */
    unsafe { *((si as *mut c_char).add(SI_PKEY_OFFSET) as *mut c_int) }
}

pub unsafe fn pkey_rights(r: c_ulong) -> *mut c_char {
    static mut BUF: [c_char; 4] = [b'r' as c_char, b'w' as c_char, b'x' as c_char, 0];
    let amr_bits: c_uint;

    if (r & PKEY_DISABLE_EXECUTE) != 0 {
        unsafe {
            BUF[2] = b'-' as c_char;
        }
    }
    amr_bits = (r & PKEY_BITS_MASK) as c_uint;
    if (amr_bits & PKEY_DISABLE_WRITE as c_uint) != 0 {
        unsafe {
            BUF[1] = b'-' as c_char;
        }
    }
    if (amr_bits & PKEY_DISABLE_ACCESS as c_uint & !(PKEY_DISABLE_WRITE as c_uint)) != 0 {
        unsafe {
            BUF[0] = b'-' as c_char;
        }
    }
    unsafe { BUF.as_mut_ptr() }
}

pub unsafe fn next_pkey_rights(mut rights: c_ulong) -> c_ulong {
    if rights == PKEY_DISABLE_ACCESS {
        return PKEY_DISABLE_EXECUTE;
    } else if rights == (PKEY_DISABLE_ACCESS | PKEY_DISABLE_EXECUTE) {
        return 0;
    }

    if (rights & PKEY_BITS_MASK) == 0 {
        rights |= PKEY_DISABLE_WRITE;
    } else if (rights & PKEY_BITS_MASK) == PKEY_DISABLE_WRITE {
        rights |= PKEY_DISABLE_ACCESS;
    }

    rights
}
