/* SPDX-License-Identifier: GPL-2.0-or-later */

// CONFIG_SOFTIRQ_ON_OWN_STACK is a build-time configuration condition.
#[cfg(CONFIG_SOFTIRQ_ON_OWN_STACK)]
unsafe extern "C" {
    pub fn do_softirq_own_stack();
}

#[cfg(not(CONFIG_SOFTIRQ_ON_OWN_STACK))]
unsafe extern "C" {
    fn __do_softirq();
}

#[cfg(not(CONFIG_SOFTIRQ_ON_OWN_STACK))]
#[inline]
pub unsafe fn do_softirq_own_stack() {
    unsafe {
        __do_softirq();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
