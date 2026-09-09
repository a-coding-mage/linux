/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding translation unit:
// linux/jump_label.h, linux/atomic.h, asm/page.h, asm/processor.h,
// linux/compiler.h, asm/bitops.h, asm/qspinlock.h, and asm/qrwlock.h.

/*
 * Your basic SMP spinlocks, allowing only a single CPU anywhere
 *
 * Simple spin lock operations.  There are two variants, one clears IRQ's
 * on the local processor, one does not.
 *
 * These are fair FIFO ticket locks, which support up to 2^16 CPUs.
 *
 * (the type definitions are in asm/spinlock_types.h)
 */

/* How long a lock should spin before we consider blocking */
pub const SPIN_THRESHOLD: i32 = 1 << 15;

/*
 * Read-write spinlocks, allowing multiple readers
 * but only one writer.
 *
 * NOTE! it is quite common to have readers in interrupts
 * but no interrupt writers. For those circumstances we
 * can "mix" irq-safe locks - any writer needs to get a
 * irq-safe write-lock, but readers can get non-irqsafe
 * read-locks.
 *
 * On x86, we implement read-write locks using the generic qrwlock with
 * x86 specific optimization.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
