/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2008, Creative Technology Ltd. All Rights Reserved.
 *
 * @File	ctpcm.h
 *
 * @Brief
 * This file contains the definition of the pcm device functions.
 *
 * @Author	Liu Chun
 * @Date 	Mar 28 2008
 */

/* Depends on declarations from "ctatc.h". */

extern "C" {
    pub fn ct_alsa_pcm_create(
        atc: *mut ct_atc,
        device: CTALSADEVS,
        device_name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
