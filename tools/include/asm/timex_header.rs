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
