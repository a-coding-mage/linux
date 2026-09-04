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

// Forward declaration for pcm_runtime (from external dependencies)
pub struct pcm_runtime;

// External opaque types from <linux/usb.h> and <sound/core.h>
pub struct usb_device;
pub struct snd_card;

#[repr(C)]
pub struct hiface_chip {
    pub dev: *mut usb_device,
    pub card: *mut snd_card,
    pub pcm: *mut pcm_runtime,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
