/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 ARM Ltd.
 */

/*
 * For IPC syscalls from compat tasks, we need to use the legacy 16k
 * alignment value. Since we don't have aliasing D-caches, the rest of
 * the time we can safely use PAGE_SIZE.
 */
pub const COMPAT_SHMLBA: usize = 4 * PAGE_SIZE;

/* Dependency supplied by asm-generic/shmparam.h. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
