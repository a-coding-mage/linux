/*
 * S32C1I selftest.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2016 Cadence Design Systems Inc.
 */

// linux/init.h, linux/kernel.h, and asm/traps.h provide the external symbols
// and initialization annotations used below.

#[cfg(XCHAL_HAVE_S32C1I)]
static mut rcw_word: i32 = 0;
#[cfg(XCHAL_HAVE_S32C1I)]
static mut rcw_probe_pc: i32 = 0;
#[cfg(XCHAL_HAVE_S32C1I)]
static mut rcw_exc: i32 = 0;

#[cfg(XCHAL_HAVE_S32C1I)]
#[inline]
unsafe fn probed_compare_swap(v: *mut i32, cmp: i32, mut set: i32) -> i32 {
    let mut tmp: i32;
    core::arch::asm!(
        "movi {tmp}, 1f",
        "s32i {tmp}, {probe}, 0",
        "wsr {cmp}, scompare1",
        "1: s32c1i {set}, {v}, 0",
        tmp = lateout(reg) tmp,
        set = inout(reg) set,
        cmp = in(reg) cmp,
        v = in(reg) v,
        probe = in(reg) &raw mut rcw_probe_pc,
        options(preserves_flags)
    );
    set
}

#[cfg(XCHAL_HAVE_S32C1I)]
unsafe fn do_probed_exception(regs: *mut pt_regs) {
    if (*regs).pc == rcw_probe_pc {
        (*regs).pc += 3;
        rcw_exc = (*regs).exccause;
    } else {
        do_unhandled(regs);
    }
}

#[cfg(XCHAL_HAVE_S32C1I)]
unsafe fn check_s32c1i() -> i32 {
    let mut n: i32;
    let mut cause1: i32;
    let mut cause2: i32;
    let handbus: *mut core::ffi::c_void;
    let handdata: *mut core::ffi::c_void;
    let handaddr: *mut core::ffi::c_void;

    rcw_probe_pc = 0;
    handbus = trap_set_handler(EXCCAUSE_LOAD_STORE_ERROR, do_probed_exception);
    handdata = trap_set_handler(EXCCAUSE_LOAD_STORE_DATA_ERROR, do_probed_exception);
    handaddr = trap_set_handler(EXCCAUSE_LOAD_STORE_ADDR_ERROR, do_probed_exception);

    rcw_exc = 0;
    rcw_word = 1;
    n = probed_compare_swap(&raw mut rcw_word, 0, 2);
    cause1 = rcw_exc;

    if cause1 != 0 {
        if n != 2 || rcw_word != 1 {
            panic!("S32C1I exception error");
        }
    } else if rcw_word != 1 || n != 1 {
        panic!("S32C1I compare error");
    }

    rcw_exc = 0;
    rcw_word = 0x1234567;
    n = probed_compare_swap(&raw mut rcw_word, 0x1234567, 0xabcde);
    cause2 = rcw_exc;

    if cause2 != 0 {
        if n != 0xabcde || rcw_word != 0x1234567 {
            panic!("S32C1I exception error (b)");
        }
    } else if rcw_word != 0xabcde || n != 0x1234567 {
        panic!("S32C1I store error");
    }

    if cause1 != 0 || cause2 != 0 {
        pr_warn!("S32C1I took exception {}, {}\n", cause1, cause2);
        panic!("S32C1I exceptions not currently supported");
    }
    if cause1 != cause2 {
        panic!("inconsistent S32C1I exceptions");
    }

    trap_set_handler(EXCCAUSE_LOAD_STORE_ERROR, handbus);
    trap_set_handler(EXCCAUSE_LOAD_STORE_DATA_ERROR, handdata);
    trap_set_handler(EXCCAUSE_LOAD_STORE_ADDR_ERROR, handaddr);
    0
}

#[cfg(not(XCHAL_HAVE_S32C1I))]
unsafe fn check_s32c1i() -> i32 {
    pr_warn!("Processor configuration lacks atomic compare-and-swap support!\n");
    0
}

early_initcall!(check_s32c1i);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
