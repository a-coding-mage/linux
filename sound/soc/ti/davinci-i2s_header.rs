// SPDX-License-Identifier: GPL-2.0-only
/*
 * ALSA SoC I2S (McBSP) Audio Layer for TI DAVINCI processor
 *
 * Author:      Vladimir Barinov, <vbarinov@embeddedalley.com>
 * Copyright:   (C) 2007 MontaVista Software, Inc., <source@mvista.com>
 */

// C header guard: _DAVINCI_I2S_H

/* McBSP dividers */
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum davinci_mcbsp_div {
    DAVINCI_MCBSP_CLKGDV = 0, /* Sample rate generator divider */
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
