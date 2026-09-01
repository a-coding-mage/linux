// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2014, Michael Ellerman, IBM Corp.
 */

// C dependencies: stdio.h, stdlib.h, and "ebb.h".
// The EBB helpers, SPR constants, and test macros are supplied externally.

extern "C" {
    fn ebb_is_supported() -> bool;
    fn mtspr(spr: u64, val: u64);
    fn mfspr(spr: u64) -> u64;
    fn test_harness(test: extern "C" fn() -> i32, name: *const i8) -> i32;

    static SPRN_BESCR: u64;
    static SPRN_EBBHR: u64;
}

/*
 * Test basic access to the EBB regs, they should be user accessible with no
 * kernel interaction required.
 */
#[no_mangle]
pub extern "C" fn reg_access() -> i32 {
    let mut val: u64;
    let mut expected: u64;

    unsafe {
        SKIP_IF!(!ebb_is_supported());

        expected = 0x8000000100000000_u64;
        mtspr(SPRN_BESCR, expected);
        val = mfspr(SPRN_BESCR);

        FAIL_IF!(val != expected);

        expected = 0x0000000001000000_u64;
        mtspr(SPRN_EBBHR, expected);
        val = mfspr(SPRN_EBBHR);

        FAIL_IF!(val != expected);
    }

    0
}

fn main() {
    unsafe {
        std::process::exit(test_harness(reg_access, b"reg_access\0".as_ptr() as *const i8));
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
