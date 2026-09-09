/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2013 Synopsys, Inc. (www.synopsys.com)
 */

// C header guard: __ARC_ASM_SHMPARAM_H

/* Handle up to 2 cache bins */
pub const SHMLBA: usize = 2 * PAGE_SIZE;

/* Enforce SHMLBA in shmat */
// C preprocessor marker: __ARCH_FORCE_SHMLBA
pub const __ARCH_FORCE_SHMLBA: bool = true;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
