// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2025 Google LLC
 * Author: Vincent Donnefort <vdonnefort@google.com>
 */

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    fn __arch_counter_get_cntvct() -> u64;
    fn smp_load_acquire(ptr: *const u64) -> u64;
    fn smp_store_release(ptr: *mut u64, value: u64);
}

#[repr(C)]
struct ClockDataBank {
    mult: u32,
    shift: u32,
    epoch_ns: u64,
    epoch_cyc: u64,
    cyc_overflow64: u64,
}

#[repr(C)]
struct ClockData {
    data: [ClockDataBank; 2],
    cur: u64,
}

static mut trace_clock_data: ClockData = ClockData {
    data: [
        ClockDataBank {
            mult: 0,
            shift: 0,
            epoch_ns: 0,
            epoch_cyc: 0,
            cyc_overflow64: 0,
        },
        ClockDataBank {
            mult: 0,
            shift: 0,
            epoch_ns: 0,
            epoch_cyc: 0,
            cyc_overflow64: 0,
        },
    ],
    cur: 0,
};

unsafe fn __clock_mult_uint128(cyc: u64, mult: u32, shift: u32) -> u64 {
    let mut ns = (cyc as u128).wrapping_mul(mult as u128);

    ns >>= shift;

    ns as u64
}

/* Does not guarantee no reader on the modified bank. */
#[no_mangle]
pub unsafe extern "C" fn trace_hyp_clock_update(
    mult: u32,
    shift: u32,
    epoch_ns: u64,
    epoch_cyc: u64,
) {
    let clock: *mut ClockData = &raw mut trace_clock_data;
    let bank = (*clock).cur ^ 1;
    let bank_index = bank as usize;

    if mult == 0 || shift >= 64 {
        return;
    }

    (*clock).data[bank_index].mult = mult;
    (*clock).data[bank_index].shift = shift;
    (*clock).data[bank_index].epoch_ns = epoch_ns;
    (*clock).data[bank_index].epoch_cyc = epoch_cyc;
    (*clock).data[bank_index].cyc_overflow64 = u64::MAX / mult as u64;

    smp_store_release(&raw mut (*clock).cur, bank);
}

/* Use untrusted host data */
#[no_mangle]
pub unsafe extern "C" fn trace_hyp_clock() -> u64 {
    let clock: *mut ClockData = &raw mut trace_clock_data;
    let bank = smp_load_acquire(&raw const (*clock).cur);
    let bank_index = bank as usize;
    let cyc: u64;
    let ns: u64;

    cyc = __arch_counter_get_cntvct() - (*clock).data[bank_index].epoch_cyc;

    if cyc < (*clock).data[bank_index].cyc_overflow64 {
        ns = cyc * (*clock).data[bank_index].mult as u64
            >> (*clock).data[bank_index].shift;
    } else {
        ns = __clock_mult_uint128(
            cyc,
            (*clock).data[bank_index].mult,
            (*clock).data[bank_index].shift,
        );
    }

    ns + (*clock).data[bank_index].epoch_ns
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
