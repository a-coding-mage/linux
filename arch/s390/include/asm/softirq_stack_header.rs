/* SPDX-License-Identifier: GPL-2.0-or-later */

// C dependencies:
// #include <asm/lowcore.h>
// #include <asm/stacktrace.h>

// Equivalent of CONFIG_SOFTIRQ_ON_OWN_STACK.  The original declaration is
// present only when this build-time configuration option is enabled.
#[cfg(feature = "CONFIG_SOFTIRQ_ON_OWN_STACK")]
#[inline(always)]
pub unsafe fn do_softirq_own_stack() {
    call_on_stack(0, (*get_lowcore()).async_stack, __do_softirq);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
