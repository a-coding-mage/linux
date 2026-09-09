/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Rockchip General Register Files definitions
 */

/* Rockchip DDRTYPE defines */
#[repr(i32)]
pub enum RockchipDdrtype {
    ROCKCHIP_DDRTYPE_DDR3 = 3,
    ROCKCHIP_DDRTYPE_LPDDR2 = 5,
    ROCKCHIP_DDRTYPE_LPDDR3 = 6,
    ROCKCHIP_DDRTYPE_LPDDR4 = 7,
    ROCKCHIP_DDRTYPE_LPDDR4X = 8,
    ROCKCHIP_DDRTYPE_LPDDR5 = 9,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
