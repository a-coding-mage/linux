/* SPDX-License-Identifier: GPL-2.0-or-later */

// C header guard: _ASM_POWERPC_DELAY_H
// The original declarations are available only when __KERNEL__ is defined.

/*
 * Copyright 1996, Paul Mackerras.
 * Copyright (C) 2009 Freescale Semiconductor, Inc. All rights reserved.
 *
 * PPC64 Support added by Dave Engebretsen, Todd Inglett, Mike Corrigan,
 * Anton Blanchard.
 */

unsafe extern "C" {
    pub fn __delay(loops: ::core::ffi::c_ulong);
    pub fn udelay(usecs: ::core::ffi::c_ulong);
}

/*
 * On shared processor machines the generic implementation of mdelay can
 * result in large errors. While each iteration of the loop inside mdelay
 * is supposed to take 1ms, the hypervisor could sleep our partition for
 * longer (eg 10ms). With the right timing these errors can add up.
 *
 * Since there is no 32bit overflow issue on 64bit kernels, just call
 * udelay directly.
 */
#[cfg(CONFIG_PPC64)]
#[macro_export]
macro_rules! mdelay {
    ($n:expr) => {
        unsafe { $crate::udelay(($n) * 1000) }
    };
}

/**
 * spin_event_timeout - spin until a condition gets true or a timeout elapses
 * @condition: a C expression to evalate
 * @timeout: timeout, in microseconds
 * @delay: the number of microseconds to delay between each evaluation of
 *         @condition
 *
 * The process spins until the condition evaluates to true (non-zero) or the
 * timeout elapses.  The return value of this macro is the value of
 * @condition when the loop terminates. This allows you to determine the cause
 * of the loop terminates.  If the return value is zero, then you know a
 * timeout has occurred.
 *
 * This primary purpose of this macro is to poll on a hardware register
 * until a status bit changes.  The timeout ensures that the loop still
 * terminates even if the bit never changes.  The delay is for devices that
 * need a delay in between successive reads.
 *
 * gcc will optimize out the if-statement if @delay is a constant.
 */
#[macro_export]
macro_rules! spin_event_timeout {
    ($condition:expr, $timeout:expr, $delay:expr) => {{
        let mut __ret;
        let __loops = tb_ticks_per_usec * $timeout;
        let __start = mftb();

        if $delay != 0 {
            while {
                __ret = $condition;
                !__ret && (tb_ticks_since(__start) <= __loops)
            } {
                unsafe { udelay($delay) };
            }
        } else {
            spin_begin();
            while {
                __ret = $condition;
                !__ret && (tb_ticks_since(__start) <= __loops)
            } {
                spin_cpu_relax();
            }
            spin_end();
        }
        if !__ret {
            __ret = $condition;
        }
        __ret
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
