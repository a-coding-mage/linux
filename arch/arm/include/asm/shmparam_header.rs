/* SPDX-License-Identifier: GPL-2.0 */

/*
 * This should be the size of the virtually indexed cache/ways,
 * or page size, whichever is greater since the cache aliases
 * every size/ways bytes.
 */
pub const SHMLBA: usize = 4 * PAGE_SIZE;

/* Enforce SHMLBA in shmat. */
/* __ARCH_FORCE_SHMLBA */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
