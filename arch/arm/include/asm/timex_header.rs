/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/include/asm/timex.h
 *
 *  Copyright (C) 1997,1998 Russell King
 *
 *  Architecture Specific TIME specifications
 */

pub type cycles_t = usize;

// Temporary workaround until timex.h is cleaned up
unsafe extern "C" {
    pub fn delay_read_timer(t: *mut usize) -> bool;
}

#[inline]
pub unsafe fn get_cycles() -> cycles_t {
    let mut c: cycles_t = 0;
    if delay_read_timer(&mut c as *mut cycles_t) {
        c
    } else {
        0
    }
}

#[inline]
pub unsafe fn random_get_entropy() -> usize {
    let cycles = get_cycles();
    if cycles != 0 {
        cycles
    } else {
        random_get_entropy_fallback()
    }
}

unsafe extern "C" {
    pub fn random_get_entropy_fallback() -> usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
