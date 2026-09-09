/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * tw9910 Driver header
 *
 * Copyright (C) 2008 Renesas Solutions Corp.
 * Kuninori Morimoto <morimoto.kuninori@renesas.com>
 *
 * Based on ov772x.h
 *
 * Copyright (C) Kuninori Morimoto <morimoto.kuninori@renesas.com>
 */

/* MPOUT (multi-purpose output) pin functions */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum tw9910_mpout_pin {
    TW9910_MPO_VLOSS,
    TW9910_MPO_HLOCK,
    TW9910_MPO_SLOCK,
    TW9910_MPO_VLOCK,
    TW9910_MPO_MONO,
    TW9910_MPO_DET50,
    TW9910_MPO_FIELD,
    TW9910_MPO_RTCO,
}

/**
 * struct tw9910_video_info - tw9910 driver interface structure
 * @buswidth:        Parallel data bus width (8 or 16).
 * @mpout:           Selected function of MPOUT (multi-purpose output) pin.
 *                   See enum tw9910_mpout_pin
 */
#[repr(C)]
pub struct tw9910_video_info {
    pub buswidth: ::core::ffi::c_ulong,
    pub mpout: tw9910_mpout_pin,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
