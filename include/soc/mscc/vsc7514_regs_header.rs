/* SPDX-License-Identifier: (GPL-2.0 OR MIT) */
/*
 * Microsemi Ocelot Switch driver
 *
 * Copyright (c) 2021 Innovative Advantage Inc.
 */

// Dependency supplied by the original include:
// #include <soc/mscc/ocelot_vcap.h>

extern "C" {
    pub static mut vsc7514_vcap_props: [vcap_props; 0];

    pub static vsc7514_regfields: [reg_field; REGFIELD_MAX];

    pub static mut vsc7514_regmap: [*const u32; TARGET_MAX];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
