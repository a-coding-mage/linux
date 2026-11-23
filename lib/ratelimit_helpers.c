// SPDX-License-Identifier: GPL-2.0-only
/*
 * Native leaves for the Rust ___ratelimit() owner: lock/IRQ, atomic, warning,
 * deferred-printk and miss-counter primitives that are macros or inlines in C.
 * The rate-limit decision itself lives in lib/ratelimit.rs.
 */
#include <linux/atomic.h>
#include <linux/build_bug.h>
#include <linux/printk.h>
#include <linux/ratelimit.h>
#include <linux/spinlock.h>

/* lib/ratelimit.rs spells these flag values; keep them tied to the header. */
static_assert(RATELIMIT_MSG_ON_RELEASE == (1U << 0));
static_assert(RATELIMIT_INITIALIZED == (1U << 1));

bool lupos_ratelimit_trylock_irqsave(struct ratelimit_state *rs, unsigned long *flags);
void lupos_ratelimit_unlock_irqrestore(struct ratelimit_state *rs, unsigned long flags);
int lupos_ratelimit_atomic_read(const atomic_t *v);
void lupos_ratelimit_atomic_set(atomic_t *v, int i);
int lupos_ratelimit_atomic_dec_return(atomic_t *v);
void lupos_ratelimit_warn_uninitialized(int interval, int burst);
void lupos_ratelimit_report_suppressed(const char *func, int missed);
int lupos_ratelimit_reset_miss(struct ratelimit_state *rs);
void lupos_ratelimit_inc_miss(struct ratelimit_state *rs);

bool lupos_ratelimit_trylock_irqsave(struct ratelimit_state *rs, unsigned long *flags)
{
	unsigned long f;
	bool locked = raw_spin_trylock_irqsave(&rs->lock, f);

	if (locked)
		*flags = f;
	return locked;
}

void lupos_ratelimit_unlock_irqrestore(struct ratelimit_state *rs, unsigned long flags)
{
	raw_spin_unlock_irqrestore(&rs->lock, flags);
}

int lupos_ratelimit_atomic_read(const atomic_t *v)
{
	return atomic_read(v);
}

void lupos_ratelimit_atomic_set(atomic_t *v, int i)
{
	atomic_set(v, i);
}

int lupos_ratelimit_atomic_dec_return(atomic_t *v)
{
	return atomic_dec_return(v);
}

void lupos_ratelimit_warn_uninitialized(int interval, int burst)
{
	WARN_ONCE(interval < 0 || burst < 0, "Negative interval (%d) or burst (%d): Uninitialized ratelimit_state structure?\n", interval, burst);
}

void lupos_ratelimit_report_suppressed(const char *func, int missed)
{
	printk_deferred(KERN_WARNING "%s: %d callbacks suppressed\n", func, missed);
}

int lupos_ratelimit_reset_miss(struct ratelimit_state *rs)
{
	return ratelimit_state_reset_miss(rs);
}

void lupos_ratelimit_inc_miss(struct ratelimit_state *rs)
{
	ratelimit_state_inc_miss(rs);
}
