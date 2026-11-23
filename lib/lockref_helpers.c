// SPDX-License-Identifier: GPL-2.0
/*
 * Native leaves for the Rust lockref owner: the architecture's unlocked
 * spinlock value test, the relaxed try_cmpxchg64() and the spinlock
 * primitives. The CMPXCHG_LOOP() bodies live in lib/lockref.rs.
 */
#include <linux/build_bug.h>
#include <linux/lockref.h>
#include <linux/spinlock.h>
#include <linux/stddef.h>

/* lib/lockref.rs relies on the configured cmpxchg layout. */
static_assert(USE_CMPXCHG_LOCKREF);
static_assert(sizeof(struct lockref) == 8);
static_assert(__LOCKREF_DEAD_VAL == -128);

bool lupos_lockref_value_unlocked(u64 lock_count);
bool lupos_lockref_try_cmpxchg64_relaxed(u64 *ptr, u64 *old, u64 new);
void lupos_lockref_lock(struct lockref *lockref);
void lupos_lockref_unlock(struct lockref *lockref);
void lupos_lockref_assert_locked(struct lockref *lockref);

/* arch_spin_value_unlocked(old.lock.rlock.raw_lock) of a snapshot. */
bool lupos_lockref_value_unlocked(u64 lock_count)
{
	struct lockref old;

	old.lock_count = lock_count;
	return arch_spin_value_unlocked(old.lock.rlock.raw_lock);
}

bool lupos_lockref_try_cmpxchg64_relaxed(u64 *ptr, u64 *old, u64 new)
{
	return try_cmpxchg64_relaxed(ptr, old, new);
}

void lupos_lockref_lock(struct lockref *lockref)
{
	spin_lock(&lockref->lock);
}

void lupos_lockref_unlock(struct lockref *lockref)
{
	spin_unlock(&lockref->lock);
}

void lupos_lockref_assert_locked(struct lockref *lockref)
{
	assert_spin_locked(&lockref->lock);
}
