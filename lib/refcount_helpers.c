// SPDX-License-Identifier: GPL-2.0
/*
 * Native leaves for the Rust refcount owner: the release cmpxchg, the
 * refcount_dec_and_test() inline, mutex/spinlock primitives and one
 * WARN_ONCE() per original REFCOUNT_WARN() site, each with its own once
 * state. The saturation and decrement policy lives in lib/refcount.rs.
 */
#include <linux/bug.h>
#include <linux/build_bug.h>
#include <linux/mutex.h>
#include <linux/refcount.h>
#include <linux/spinlock.h>

/* lib/refcount.rs spells these values. */
static_assert(REFCOUNT_SATURATED == INT_MIN / 2);
static_assert(REFCOUNT_ADD_NOT_ZERO_OVF == 0 && REFCOUNT_ADD_OVF == 1 &&
	      REFCOUNT_ADD_UAF == 2 && REFCOUNT_SUB_UAF == 3 &&
	      REFCOUNT_DEC_LEAK == 4);

#define REFCOUNT_WARN(str)	WARN_ONCE(1, "refcount_t: " str ".\n")

bool lupos_refcount_try_cmpxchg_release(atomic_t *v, int *old, int new);
bool lupos_refcount_dec_and_test(refcount_t *r);
void lupos_refcount_mutex_lock(struct mutex *lock);
void lupos_refcount_spin_lock(spinlock_t *lock);
void lupos_refcount_spin_unlock(spinlock_t *lock);
unsigned long lupos_refcount_spin_lock_irqsave(spinlock_t *lock);
void lupos_refcount_spin_unlock_irqrestore(spinlock_t *lock, unsigned long flags);
void lupos_refcount_warn_add_not_zero_ovf(void);
void lupos_refcount_warn_add_ovf(void);
void lupos_refcount_warn_add_uaf(void);
void lupos_refcount_warn_sub_uaf(void);
void lupos_refcount_warn_dec_leak(void);
void lupos_refcount_warn_unknown(void);
void lupos_refcount_warn_dec_not_one_underflow(void);

bool lupos_refcount_try_cmpxchg_release(atomic_t *v, int *old, int new)
{
	return atomic_try_cmpxchg_release(v, old, new);
}

bool lupos_refcount_dec_and_test(refcount_t *r)
{
	return refcount_dec_and_test(r);
}

void lupos_refcount_mutex_lock(struct mutex *lock)
{
	mutex_lock(lock);
}

void lupos_refcount_spin_lock(spinlock_t *lock)
{
	spin_lock(lock);
}

void lupos_refcount_spin_unlock(spinlock_t *lock)
{
	spin_unlock(lock);
}

unsigned long lupos_refcount_spin_lock_irqsave(spinlock_t *lock)
{
	unsigned long flags;

	spin_lock_irqsave(lock, flags);
	return flags;
}

void lupos_refcount_spin_unlock_irqrestore(spinlock_t *lock, unsigned long flags)
{
	spin_unlock_irqrestore(lock, flags);
}

void lupos_refcount_warn_add_not_zero_ovf(void)
{
	REFCOUNT_WARN("saturated; leaking memory");
}

void lupos_refcount_warn_add_ovf(void)
{
	REFCOUNT_WARN("saturated; leaking memory");
}

void lupos_refcount_warn_add_uaf(void)
{
	REFCOUNT_WARN("addition on 0; use-after-free");
}

void lupos_refcount_warn_sub_uaf(void)
{
	REFCOUNT_WARN("underflow; use-after-free");
}

void lupos_refcount_warn_dec_leak(void)
{
	REFCOUNT_WARN("decrement hit 0; leaking memory");
}

void lupos_refcount_warn_unknown(void)
{
	REFCOUNT_WARN("unknown saturation event!?");
}

/* refcount_dec_not_one(): WARN_ONCE(new > val, ...). */
void lupos_refcount_warn_dec_not_one_underflow(void)
{
	WARN_ONCE(1, "refcount_t: underflow; use-after-free.\n");
}
