/* SPDX-License-Identifier: GPL-2.0-only OR BSD-2-Clause */
/*
 * Copyright (C) 2023 Sophgo Ltd.
 *
 * Author: Inochi Amaoto <inochiama@outlook.com>
 */

// C header guard: _DT_BINDINGS_PINCTRL_CV18XX_H

pub const PIN_MUX_INVALD: u32 = 0xff;

pub const fn PINMUX2(pin: u32, mux: u32, mux2: u32) -> u32 {
    (pin & 0xffff) | ((mux & 0xff) << 16) | ((mux2 & 0xff) << 24)
}

pub const fn PINMUX(pin: u32, mux: u32) -> u32 {
    PINMUX2(pin, mux, PIN_MUX_INVALD)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
