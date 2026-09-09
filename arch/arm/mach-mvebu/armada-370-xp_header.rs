/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Generic definitions for Marvell Armada_370_XP SoCs
 *
 * Copyright (C) 2012 Marvell
 *
 * Lior Amsalem <alior@marvell.com>
 * Gregory CLEMENT <gregory.clement@free-electrons.com>
 * Thomas Petazzoni <thomas.petazzoni@free-electrons.com>
 */

/* C build-time condition: CONFIG_SMP. */
#[cfg(feature = "CONFIG_SMP")]
unsafe extern "C" {
    pub fn armada_xp_secondary_startup();

    pub static armada_xp_smp_ops: smp_operations;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
