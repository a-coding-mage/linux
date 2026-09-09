/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2019 BayLibre, SAS.
 * Author: Neil Armstrong <narmstrong@baylibre.com>
 */

// Dependency supplied by the Linux clock-provider interface.
// Dependency supplied by the local parm definition.

#[repr(C)]
pub struct MesonClkCpuDyndivData {
    pub div: Parm,
    pub r#dyn: Parm,
}

// External declaration corresponding to:
// extern const struct clk_ops meson_clk_cpu_dyndiv_ops;
extern "C" {
    pub static meson_clk_cpu_dyndiv_ops: ClkOps;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
