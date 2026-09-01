/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright(c) 2024 Intel Corporation
 *
 * Authors: Cezary Rojewski <cezary.rojewski@intel.com>
 *          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
 */

/* Depends on C header: <sound/pcm.h> */

unsafe extern "C" {
    pub fn avs_period_elapsed(substream: *mut snd_pcm_substream);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
