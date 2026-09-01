/* SPDX-License-Identifier: GPL-2.0 */

// C dependency intent: #include <time.h>

pub type cycles_t = clock_t;

unsafe extern "C" {
    pub fn clock() -> clock_t;
}

#[inline]
pub unsafe fn get_cycles() -> cycles_t {
    unsafe { clock() }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
