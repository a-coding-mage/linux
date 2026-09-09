// SPDX-License-Identifier: GPL-2.0-only

// Dependencies corresponding to <linux/bug.h>, <linux/export.h>, and
// <linux/irqflags.h> are supplied by the surrounding build.

extern "C" {
    fn instrumentation_begin();
    fn instrumentation_end();
}

// noinstr void warn_bogus_irq_restore(void)
pub unsafe fn warn_bogus_irq_restore() {
    instrumentation_begin();
    // WARN_ONCE(1, "raw_local_irq_restore() called with IRQs enabled\n");
    instrumentation_end();
}

// EXPORT_SYMBOL(warn_bogus_irq_restore);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
