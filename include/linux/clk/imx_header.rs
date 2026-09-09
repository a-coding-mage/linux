/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020 Freescale Semiconductor, Inc.
 *
 * Author: Lee Jones <lee.jones@linaro.org>
 */

// Dependency intent: <linux/types.h>

extern "C" {
    pub fn imx6sl_set_wait_clk(enter: bool);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
