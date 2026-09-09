/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 *
 * Vineetg: May 16th, 2008
 *  - Current macro is now implemented as "global register" r25
 */

/* C header guard: _ASM_ARC_CURRENT_H */

/* The original declaration is excluded when compiling assembly. */

#[cfg(feature = "CONFIG_ARC_CURR_IN_REG")]
extern "C" {
    /* C: register struct task_struct *curr_arc asm("gp"); */
    pub static mut curr_arc: *mut task_struct;
}

#[cfg(feature = "CONFIG_ARC_CURR_IN_REG")]
#[macro_export]
macro_rules! current {
    () => {
        $crate::curr_arc
    };
}

/*
 * When CONFIG_ARC_CURR_IN_REG is disabled, the C header includes
 * <asm-generic/current.h>; its externally supplied `current` definition is
 * intentionally left as a dependency of this translation.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
