/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/mach-sti/smp.h
 *
 * Copyright (C) 2013 STMicroelectronics (R&D) Limited.
 *		http://www.st.com
 */

// The `smp_operations` type is supplied by an external dependency.
unsafe extern "C" {
    pub static sti_smp_ops: smp_operations;

    pub fn sti_secondary_startup();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
