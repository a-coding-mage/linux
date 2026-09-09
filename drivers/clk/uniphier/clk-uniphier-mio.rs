// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2016 Socionext Inc.
 *   Author: Masahiro Yamada <yamada.masahiro@socionext.com>
 */

// Translated from clk-uniphier-mio.c.  Types and clock-construction helpers
// are supplied by the surrounding Uniphier clock implementation.

macro_rules! UNIPHIER_MIO_CLK_SD_FIXED {
    () => {
        UNIPHIER_CLK_FACTOR!("sd-44m", -1, "sd-133m", 1, 3),
        UNIPHIER_CLK_FACTOR!("sd-33m", -1, "sd-200m", 1, 6),
        UNIPHIER_CLK_FACTOR!("sd-50m", -1, "sd-200m", 1, 4),
        UNIPHIER_CLK_FACTOR!("sd-67m", -1, "sd-200m", 1, 3),
        UNIPHIER_CLK_FACTOR!("sd-100m", -1, "sd-200m", 1, 2),
        UNIPHIER_CLK_FACTOR!("sd-40m", -1, "sd-200m", 1, 5),
        UNIPHIER_CLK_FACTOR!("sd-25m", -1, "sd-200m", 1, 8),
        UNIPHIER_CLK_FACTOR!("sd-22m", -1, "sd-133m", 1, 6),
    };
}

macro_rules! UNIPHIER_MIO_CLK_SD {
    ($idx:expr, $ch:expr) => {
        uniphier_clk_data {
            name: concat!("sd", $ch, "-sel"),
            r#type: UNIPHIER_CLK_TYPE_MUX,
            idx: -1,
            data: uniphier_clk_data_union::mux(uniphier_clk_mux_data {
                parent_names: [
                    "sd-44m", "sd-33m", "sd-50m", "sd-67m",
                    "sd-100m", "sd-40m", "sd-25m", "sd-22m",
                ],
                num_parents: 8,
                reg: 0x30 + 0x200 * ($ch),
                masks: [
                    0x00031000, 0x00031000, 0x00031000, 0x00031000,
                    0x00001300, 0x00001300, 0x00001300, 0x00001300,
                ],
                vals: [
                    0x00000000, 0x00010000, 0x00020000, 0x00030000,
                    0x00001000, 0x00001100, 0x00001200, 0x00001300,
                ],
            }),
        },
        UNIPHIER_CLK_GATE!(concat!("sd", $ch), $idx,
                               concat!("sd", $ch, "-sel"),
                               0x20 + 0x200 * ($ch), 8),
    };
}

macro_rules! UNIPHIER_MIO_CLK_USB2 {
    ($idx:expr, $ch:expr) => {
        UNIPHIER_CLK_GATE!(concat!("usb2", $ch), $idx, "usb2",
                           0x20 + 0x200 * ($ch), 28)
    };
}

macro_rules! UNIPHIER_MIO_CLK_USB2_PHY {
    ($idx:expr, $ch:expr) => {
        UNIPHIER_CLK_GATE!(concat!("usb2", $ch, "-phy"), $idx, "usb2",
                           0x20 + 0x200 * ($ch), 29)
    };
}

pub const uniphier_ld4_mio_clk_data: &[uniphier_clk_data] = &[
    UNIPHIER_MIO_CLK_SD_FIXED!(),
    UNIPHIER_MIO_CLK_SD!(0, 0),
    UNIPHIER_MIO_CLK_SD!(1, 1),
    UNIPHIER_MIO_CLK_SD!(2, 2),
    UNIPHIER_CLK_GATE!("miodmac", 7, core::ptr::null(), 0x20, 25),
    UNIPHIER_MIO_CLK_USB2!(8, 0),
    UNIPHIER_MIO_CLK_USB2!(9, 1),
    UNIPHIER_MIO_CLK_USB2!(10, 2),
    UNIPHIER_MIO_CLK_USB2_PHY!(12, 0),
    UNIPHIER_MIO_CLK_USB2_PHY!(13, 1),
    UNIPHIER_MIO_CLK_USB2_PHY!(14, 2),
    uniphier_clk_data { /* sentinel */ },
];

pub const uniphier_pro5_sd_clk_data: &[uniphier_clk_data] = &[
    UNIPHIER_MIO_CLK_SD_FIXED!(),
    UNIPHIER_MIO_CLK_SD!(0, 0),
    UNIPHIER_MIO_CLK_SD!(1, 1),
    uniphier_clk_data { /* sentinel */ },
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
