/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2021, NVIDIA Corporation. All rights reserved.
 */

// Dependency supplied by the translated Linux device definitions:
// #include <linux/device.h>

// Equivalent of CONFIG_TEGRA_HOST1X_CONTEXT_BUS.
#[cfg(CONFIG_TEGRA_HOST1X_CONTEXT_BUS)]
extern "C" {
    pub static host1x_context_device_bus_type: bus_type;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
