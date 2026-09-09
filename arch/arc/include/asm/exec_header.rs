/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

/* Align to 16b */
#[macro_export]
macro_rules! arch_align_stack {
    ($p:expr) => {
        (($p as usize) & !0xfusize)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
