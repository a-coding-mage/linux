// SPDX-License-Identifier: GPL-2.0

#include <linux/seqlock.h>

/* Expose the typed _Generic primitive without changing its lockdep, KCSAN,
 * memory ordering or preemption behavior. Rust owns the protected data. */
__rust_helper void
rust_helper_write_seqcount_spinlock_begin(seqcount_spinlock_t *sequence)
{
	write_seqcount_begin(sequence);
}

__rust_helper void
rust_helper_write_seqcount_spinlock_end(seqcount_spinlock_t *sequence)
{
	write_seqcount_end(sequence);
}
