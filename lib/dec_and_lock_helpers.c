// SPDX-License-Identifier: GPL-2.0
/*
 * Native leaves for the Rust decrement-and-lock owner: the configured
 * atomic_add_unless()/atomic_dec_and_test() inlines and the (raw) spinlock
 * and IRQ-state primitives. The decision sequence lives in
 * lib/dec_and_lock.rs.
 */
#include <linux/atomic.h>
#include <linux/spinlock.h>

bool lupos_dec_and_lock_add_unless_one(atomic_t *atomic);
bool lupos_dec_and_lock_dec_and_test(atomic_t *atomic);
void lupos_dec_and_lock_spin_lock(spinlock_t *lock);
void lupos_dec_and_lock_spin_unlock(spinlock_t *lock);
unsigned long lupos_dec_and_lock_spin_lock_irqsave(spinlock_t *lock);
void lupos_dec_and_lock_spin_unlock_irqrestore(spinlock_t *lock, unsigned long flags);
void lupos_dec_and_lock_raw_spin_lock(raw_spinlock_t *lock);
void lupos_dec_and_lock_raw_spin_unlock(raw_spinlock_t *lock);
unsigned long lupos_dec_and_lock_raw_spin_lock_irqsave(raw_spinlock_t *lock);
void lupos_dec_and_lock_raw_spin_unlock_irqrestore(raw_spinlock_t *lock, unsigned long flags);

/* Subtract 1 from counter unless that drops it to 0 (ie. it was 1). */
bool lupos_dec_and_lock_add_unless_one(atomic_t *atomic)
{
	return atomic_add_unless(atomic, -1, 1);
}

bool lupos_dec_and_lock_dec_and_test(atomic_t *atomic)
{
	return atomic_dec_and_test(atomic);
}

void lupos_dec_and_lock_spin_lock(spinlock_t *lock)
{
	spin_lock(lock);
}

void lupos_dec_and_lock_spin_unlock(spinlock_t *lock)
{
	spin_unlock(lock);
}

unsigned long lupos_dec_and_lock_spin_lock_irqsave(spinlock_t *lock)
{
	unsigned long flags;

	spin_lock_irqsave(lock, flags);
	return flags;
}

void lupos_dec_and_lock_spin_unlock_irqrestore(spinlock_t *lock, unsigned long flags)
{
	spin_unlock_irqrestore(lock, flags);
}

void lupos_dec_and_lock_raw_spin_lock(raw_spinlock_t *lock)
{
	raw_spin_lock(lock);
}

void lupos_dec_and_lock_raw_spin_unlock(raw_spinlock_t *lock)
{
	raw_spin_unlock(lock);
}

unsigned long lupos_dec_and_lock_raw_spin_lock_irqsave(raw_spinlock_t *lock)
{
	unsigned long flags;

	raw_spin_lock_irqsave(lock, flags);
	return flags;
}

void lupos_dec_and_lock_raw_spin_unlock_irqrestore(raw_spinlock_t *lock, unsigned long flags)
{
	raw_spin_unlock_irqrestore(lock, flags);
}
