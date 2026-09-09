/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
pub enum cd_types {
    ESDHC_CD_NONE,       /* no CD, neither controller nor gpio */
    ESDHC_CD_CONTROLLER, /* mmc controller internal CD */
    ESDHC_CD_PERMANENT,  /* no CD, card permanently wired to host */
}

#[repr(C)]
pub struct mcf_esdhc_platform_data {
    pub max_bus_width: i32,
    pub cd_type: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
