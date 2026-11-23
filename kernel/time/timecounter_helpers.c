// SPDX-License-Identifier: GPL-2.0+
/*
 * Compile-time ties for the Rust time counter owner in
 * kernel/time/timecounter.rs, which mirrors these two structures.
 */
#include <linux/build_bug.h>
#include <linux/stddef.h>
#include <linux/timecounter.h>

static_assert(offsetof(struct cyclecounter, read) == 0);
static_assert(offsetof(struct cyclecounter, mask) == 8);
static_assert(offsetof(struct cyclecounter, mult) == 16);
static_assert(offsetof(struct cyclecounter, shift) == 20);
static_assert(sizeof(struct cyclecounter) == 24);
static_assert(offsetof(struct timecounter, cc) == 0);
static_assert(offsetof(struct timecounter, cycle_last) == 8);
static_assert(offsetof(struct timecounter, nsec) == 16);
static_assert(offsetof(struct timecounter, mask) == 24);
static_assert(offsetof(struct timecounter, frac) == 32);
static_assert(sizeof(struct timecounter) == 40);
