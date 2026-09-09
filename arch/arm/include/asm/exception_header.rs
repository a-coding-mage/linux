/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Annotations for marking C functions as exception handlers.
 *
 * These should only be used for C functions that are called from the low
 * level exception entry code and not any intervening C code.
 */

// Dependency equivalent of: #include <linux/interrupt.h>

// C macro equivalent: __exception_irq_entry expands to the externally
// supplied __irq_entry annotation.
// Rust declarations using this header should apply __irq_entry directly.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
