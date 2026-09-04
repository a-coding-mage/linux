// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Linux driver for M2Tech hiFace compatible devices
 *
 * Copyright 2012-2013 (C) M2TECH S.r.l and Amarula Solutions B.V.
 *
 * Authors:  Michael Trimarchi <michael@amarulasolutions.com>
 *           Antonio Ospite <ao2@amarulasolutions.com>
 *
 * The driver is based on the work done in TerraTec DMX 6Fire USB
 */

pub struct hiface_chip;

extern "C" {
    pub fn hiface_pcm_init(chip: *mut hiface_chip, extra_freq: u8) -> i32;
    pub fn hiface_pcm_abort(chip: *mut hiface_chip);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
