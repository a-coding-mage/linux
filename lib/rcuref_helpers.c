// SPDX-License-Identifier: GPL-2.0-only
/*
 * Native leaves for the Rust rcuref owner: the release cmpxchg and the two
 * WARN_ONCE() sites. The slow-path zone handling lives in lib/rcuref.rs.
 */
#include <linux/atomic.h>
#include <linux/bug.h>
#include <linux/build_bug.h>
#include <linux/rcuref.h>

/* lib/rcuref.rs spells the zone markers. */
static_assert(RCUREF_MAXREF == 0x7FFFFFFFU && RCUREF_SATURATED == 0xA0000000U &&
	      RCUREF_RELEASED == 0xC0000000U && RCUREF_DEAD == 0xE0000000U &&
	      RCUREF_NOREF == 0xFFFFFFFFU);

bool lupos_rcuref_try_cmpxchg_release(atomic_t *v, int *old, int new);
void lupos_rcuref_warn_saturated(void);
void lupos_rcuref_warn_imbalanced_put(void);

bool lupos_rcuref_try_cmpxchg_release(atomic_t *v, int *old, int new)
{
	return atomic_try_cmpxchg_release(v, old, new);
}

void lupos_rcuref_warn_saturated(void)
{
	WARN_ONCE(1, "rcuref saturated - leaking memory");
}

void lupos_rcuref_warn_imbalanced_put(void)
{
	WARN_ONCE(1, "rcuref - imbalanced put()");
}
