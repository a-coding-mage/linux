// SPDX-License-Identifier: GPL-2.0
/*
 * Native leaves for the Rust errseq owner: the fully ordered cmpxchg() and
 * the bad-error warning. Sequence logic lives in lib/errseq.rs.
 */
#include <linux/atomic.h>
#include <linux/bug.h>
#include <linux/build_bug.h>
#include <linux/err.h>
#include <linux/errseq.h>
#include <linux/log2.h>

/* lib/errseq.rs derives these from MAX_ERRNO as errseq.c does. */
static_assert(MAX_ERRNO == 4095);
static_assert(ilog2(MAX_ERRNO) + 1 == 12);
static_assert(sizeof(errseq_t) == 4 && (errseq_t)-1 > 0);

errseq_t lupos_errseq_cmpxchg(errseq_t *eseq, errseq_t old, errseq_t new);
void lupos_errseq_warn_bad_err(int err);

errseq_t lupos_errseq_cmpxchg(errseq_t *eseq, errseq_t old, errseq_t new)
{
	return cmpxchg(eseq, old, new);
}

/* errseq_set(): err must be between -1 and -MAX_ERRNO. */
void lupos_errseq_warn_bad_err(int err)
{
	WARN(1, "err = %d\n", err);
}
