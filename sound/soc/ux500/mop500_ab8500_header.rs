// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) ST-Ericsson SA 2012
 *
 * Author: Ola Lilja <ola.o.lilja@stericsson.com>
 *         for ST-Ericsson.
 */

// C header dependencies: struct snd_soc_ops, struct snd_soc_pcm_runtime,
// and struct snd_soc_card are supplied by other files.

#[repr(C)]
pub struct snd_soc_ops {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_card {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub static mop500_ab8500_ops: [snd_soc_ops; 0];

    pub fn mop500_ab8500_machine_init(rtd: *mut snd_soc_pcm_runtime) -> core::ffi::c_int;
    pub fn mop500_ab8500_remove(card: *mut snd_soc_card);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
