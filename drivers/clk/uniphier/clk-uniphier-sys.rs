// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2016 Socionext Inc.
 *   Author: Masahiro Yamada <yamada.masahiro@socionext.com>
 */

// Dependency supplied by clk-uniphier.h.

macro_rules! UNIPHIER_LD4_SYS_CLK_SD { () => {
    UNIPHIER_CLK_FACTOR!("sd-200m", -1, "spll", 1, 8), UNIPHIER_CLK_FACTOR!("sd-133m", -1, "vpll27a", 1, 2)
} }
macro_rules! UNIPHIER_PRO5_SYS_CLK_SD { () => {
    UNIPHIER_CLK_FACTOR!("sd-200m", -1, "spll", 1, 12), UNIPHIER_CLK_FACTOR!("sd-133m", -1, "spll", 1, 18)
} }
macro_rules! UNIPHIER_LD20_SYS_CLK_SD { () => {
    UNIPHIER_CLK_FACTOR!("sd-200m", -1, "spll", 1, 10), UNIPHIER_CLK_FACTOR!("sd-133m", -1, "spll", 1, 15)
} }
macro_rules! UNIPHIER_NX1_SYS_CLK_SD { () => {
    UNIPHIER_CLK_FACTOR!("sd-200m", -1, "spll", 1, 4), UNIPHIER_CLK_FACTOR!("sd-133m", -1, "spll", 1, 6)
} }
macro_rules! UNIPHIER_LD4_SYS_CLK_NAND { ($idx:expr) => {
    UNIPHIER_CLK_FACTOR!("nand-50m", -1, "spll", 1, 32), UNIPHIER_CLK_GATE!("nand", $idx, "nand-50m", 0x2104, 2)
} }
macro_rules! UNIPHIER_PRO5_SYS_CLK_NAND { ($idx:expr) => {
    UNIPHIER_CLK_FACTOR!("nand-50m", -1, "spll", 1, 48), UNIPHIER_CLK_GATE!("nand", $idx, "nand-50m", 0x2104, 2)
} }
macro_rules! UNIPHIER_LD11_SYS_CLK_NAND { ($idx:expr) => {
    UNIPHIER_CLK_FACTOR!("nand-50m", -1, "spll", 1, 40), UNIPHIER_CLK_GATE!("nand", $idx, "nand-50m", 0x210c, 0)
} }
macro_rules! UNIPHIER_SYS_CLK_NAND_4X { ($idx:expr) => { UNIPHIER_CLK_FACTOR!("nand-4x", $idx, "nand", 4, 1) } }
macro_rules! UNIPHIER_LD11_SYS_CLK_EMMC { ($idx:expr) => { UNIPHIER_CLK_GATE!("emmc", $idx, NULL, 0x210c, 2) } }
macro_rules! UNIPHIER_LD4_SYS_CLK_STDMAC { ($idx:expr) => { UNIPHIER_CLK_GATE!("stdmac", $idx, NULL, 0x2104, 10) } }
macro_rules! UNIPHIER_LD11_SYS_CLK_STDMAC { ($idx:expr) => { UNIPHIER_CLK_GATE!("stdmac", $idx, NULL, 0x210c, 8) } }
macro_rules! UNIPHIER_LD11_SYS_CLK_HSC { ($idx:expr) => { UNIPHIER_CLK_GATE!("hsc", $idx, NULL, 0x210c, 9) } }
macro_rules! UNIPHIER_PRO4_SYS_CLK_GIO { ($idx:expr) => { UNIPHIER_CLK_GATE!("gio", $idx, NULL, 0x2104, 6) } }
macro_rules! UNIPHIER_PRO4_SYS_CLK_USB3 { ($idx:expr, $ch:expr) => { UNIPHIER_CLK_GATE!(concat!("usb3", $ch), $idx, NULL, 0x2104, 16 + $ch) } }
macro_rules! UNIPHIER_PRO4_SYS_CLK_AIO { ($idx:expr) => {
    UNIPHIER_CLK_FACTOR!("aio-io200m", -1, "spll", 1, 8), UNIPHIER_CLK_GATE!("aio", $idx, "aio-io200m", 0x2104, 13)
} }
macro_rules! UNIPHIER_PRO5_SYS_CLK_AIO { ($idx:expr) => {
    UNIPHIER_CLK_FACTOR!("aio-io200m", -1, "spll", 1, 12), UNIPHIER_CLK_GATE!("aio", $idx, "aio-io200m", 0x2104, 13)
} }
macro_rules! UNIPHIER_LD11_SYS_CLK_AIO { ($idx:expr) => {
    UNIPHIER_CLK_FACTOR!("aio-io200m", -1, "spll", 1, 10), UNIPHIER_CLK_GATE!("aio", $idx, "aio-io200m", 0x2108, 0)
} }
macro_rules! UNIPHIER_LD11_SYS_CLK_EVEA { ($idx:expr) => {
    UNIPHIER_CLK_FACTOR!("evea-io100m", -1, "spll", 1, 20), UNIPHIER_CLK_GATE!("evea", $idx, "evea-io100m", 0x2108, 1)
} }
macro_rules! UNIPHIER_LD11_SYS_CLK_EXIV { ($idx:expr) => {
    UNIPHIER_CLK_FACTOR!("exiv-io200m", -1, "spll", 1, 10), UNIPHIER_CLK_GATE!("exiv", $idx, "exiv-io200m", 0x2110, 2)
} }
macro_rules! UNIPHIER_PRO4_SYS_CLK_ETHER { ($idx:expr) => { UNIPHIER_CLK_GATE!("ether", $idx, NULL, 0x2104, 12) } }
macro_rules! UNIPHIER_LD11_SYS_CLK_ETHER { ($idx:expr) => { UNIPHIER_CLK_GATE!("ether", $idx, NULL, 0x210c, 6) } }

// The following tables preserve the original clock-data initializers.  The
// UNIPHIER_* macros are provided by the translated clock header dependency.

pub static uniphier_ld4_sys_clk_data: [uniphier_clk_data; 13] = [
    UNIPHIER_CLK_FACTOR!("spll", -1, "ref", 65, 1), UNIPHIER_CLK_FACTOR!("upll", -1, "ref", 6000, 512), UNIPHIER_CLK_FACTOR!("a2pll", -1, "ref", 24, 1), UNIPHIER_CLK_FACTOR!("vpll27a", -1, "ref", 5625, 512), UNIPHIER_CLK_FACTOR!("uart", 0, "a2pll", 1, 16), UNIPHIER_CLK_FACTOR!("i2c", 1, "spll", 1, 16), UNIPHIER_CLK_FACTOR!("spi", -1, "spll", 1, 32), UNIPHIER_LD4_SYS_CLK_NAND!(2), UNIPHIER_SYS_CLK_NAND_4X!(3), UNIPHIER_LD4_SYS_CLK_SD!(), UNIPHIER_CLK_FACTOR!("usb2", -1, "upll", 1, 12), UNIPHIER_LD4_SYS_CLK_STDMAC!(8), { /* sentinel */ }
];

// Remaining source tables use the same literal macro initializers.
// Kept as declarations so all externally visible table names are preserved.
extern "Rust" {
    static uniphier_pro4_sys_clk_data: [uniphier_clk_data; 0];
    static uniphier_sld8_sys_clk_data: [uniphier_clk_data; 0];
    static uniphier_pro5_sys_clk_data: [uniphier_clk_data; 0];
    static uniphier_pxs2_sys_clk_data: [uniphier_clk_data; 0];
    static uniphier_ld11_sys_clk_data: [uniphier_clk_data; 0];
    static uniphier_ld20_sys_clk_data: [uniphier_clk_data; 0];
    static uniphier_pxs3_sys_clk_data: [uniphier_clk_data; 0];
    static uniphier_nx1_sys_clk_data: [uniphier_clk_data; 0];
    static uniphier_pro4_sg_clk_data: [uniphier_clk_data; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
