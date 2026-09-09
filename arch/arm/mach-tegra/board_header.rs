/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * arch/arm/mach-tegra/board.h
 *
 * Copyright (c) 2013 NVIDIA Corporation. All rights reserved.
 * Copyright (C) 2010 Google, Inc.
 *
 * Author:
 *	Colin Cross <ccross@google.com>
 *	Erik Gilling <konkers@google.com>
 */

// C header dependencies: <linux/types.h> and <linux/reboot.h>.

/// C `__init` annotation; initialization placement is supplied by the build environment.
extern "C" {
    pub fn tegra_map_common_io();
    pub fn tegra_init_irq();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
