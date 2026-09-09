/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * OMAP2 CPU identification code
 *
 * Copyright (C) 2010 Kan-Ru Chen <kanru@0xlab.org>
 */

#[repr(C)]
pub struct omap_die_id {
    pub id_0: u32,
    pub id_1: u32,
    pub id_2: u32,
    pub id_3: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
