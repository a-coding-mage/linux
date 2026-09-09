/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2021 Pengutronix, Ahmad Fatoum <kernel@pengutronix.de>
 */

// C header guard: __CAAM_TRUSTED_KEY_H

extern "C" {
    pub static mut trusted_key_caam_ops: trusted_key_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
