/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright IBM Corp. 1999, 2009
 *
 * Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>
 */

// Dependencies supplied by the surrounding kernel translation unit:
// facility definitions, alternative patching, lowcore access, min/max,
// string operations, types, and preemption control.

pub const MAX_FACILITY_BIT: usize = core::mem::size_of::<[u64; 16]>() * 8;

extern "C" {
    pub static mut stfle_fac_list: [u64; 16];
}

#[inline]
pub unsafe fn __set_facility(nr: usize, facilities: *mut core::ffi::c_void) {
    let ptr = facilities as *mut u8;

    if nr >= MAX_FACILITY_BIT {
        return;
    }
    *ptr.add(nr >> 3) |= 0x80u8 >> (nr & 7);
}

#[inline]
pub unsafe fn __clear_facility(nr: usize, facilities: *mut core::ffi::c_void) {
    let ptr = facilities as *mut u8;

    if nr >= MAX_FACILITY_BIT {
        return;
    }
    *ptr.add(nr >> 3) &= !(0x80u8 >> (nr & 7));
}

#[inline(always)]
pub unsafe fn __test_facility(nr: usize, facilities: *mut core::ffi::c_void) -> bool {
    if nr >= MAX_FACILITY_BIT {
        return false;
    }
    let ptr = (facilities as *mut u8).add(nr >> 3);
    (*ptr & (0x80u8 >> (nr & 7))) != 0
}

/*
 * __test_facility_constant() generates a single instruction branch. If the
 * tested facility is available (likely) the branch is patched into a nop.
 *
 * Do not use this function unless you know what you are doing. All users are
 * supposed to use test_facility() which will do the right thing.
 */
#[inline(always)]
pub unsafe fn __test_facility_constant(_nr: usize) -> bool {
    // C asm-goto and the ALTERNATIVE patching machinery are supplied by the
    // s390 kernel build and have no file-local Rust equivalent.
    unimplemented!("s390 ALTERNATIVE asm-goto facility test")
}

/*
 * The test_facility function uses the bit ordering where the MSB is bit 0.
 * That makes it easier to query facility bits with the bit number as
 * documented in the Principles of Operation.
 */
#[inline(always)]
pub unsafe fn test_facility(nr: usize) -> bool {
    let facilities_als: [usize; 0] = [];

    // __is_defined(__DECOMPRESSOR) and __builtin_constant_p(nr) are
    // build-time C conditions; callers may provide the decompressor-specific
    // equivalent when integrating this translation.
    if nr < core::mem::size_of_val(&facilities_als) * 8 {
        if __test_facility(nr, facilities_als.as_ptr() as *mut core::ffi::c_void) {
            return true;
        }
        return __test_facility_constant(nr);
    }
    __test_facility(nr, core::ptr::addr_of_mut!(stfle_fac_list) as *mut core::ffi::c_void)
}

#[inline]
pub unsafe fn __stfle_asm(fac_list: *mut u64, size: i32) -> usize {
    let mut reg0 = (size - 1) as usize;

    core::arch::asm!(
        "lgr 0,{reg0}",
        ".insn s,0xb2b00000,{list}", // stfle
        "lgr {reg0},0",
        reg0 = inout(reg) reg0,
        list = inout(reg) *fac_list,
        options(preserves_flags)
    );
    reg0
}

/**
 * stfle - Store facility list extended
 * @fac_list: array where facility list can be stored
 * @size: size of passed in array in double words
 */
#[inline]
pub unsafe fn __stfle(fac_list: *mut u64, size: i32) {
    let mut nr: usize;
    let stfl_fac_list: u32;

    // The STFL lowcore store and get_lowcore() are architecture-specific
    // operations supplied by the surrounding s390 translation.
    core::arch::asm!("stfl 0(0)", options(nostack));
    stfl_fac_list = *(core::ptr::null::<u32>());
    core::ptr::copy_nonoverlapping(
        &stfl_fac_list as *const u32 as *const u8,
        fac_list as *mut u8,
        4,
    );
    nr = 4; /* bytes stored by stfl */
    if stfl_fac_list & 0x01000000 != 0 {
        /* More facility bits available with stfle */
        nr = __stfle_asm(fac_list, size);
        nr = core::cmp::min((nr + 1) * 8, (size as usize) * 8);
    }
    core::ptr::write_bytes((fac_list as *mut u8).add(nr), 0, (size as usize) * 8 - nr);
}

#[inline]
pub unsafe fn stfle(fac_list: *mut u64, size: i32) {
    preempt_disable();
    __stfle(fac_list, size);
    preempt_enable();
}

extern "C" {
    pub fn preempt_disable();
    pub fn preempt_enable();
    pub fn stfle_size() -> u32;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
