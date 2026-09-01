// SPDX-License-Identifier: GPL-2.0
// dice-teac.c - a part of driver for DICE based devices
//
// Copyright (c) 2025 Takashi Sakamoto

// Translated from C source that included "dice.h"; external definitions are
// expected to be supplied by the surrounding repository.

extern "C" {
    fn snd_dice_transaction_read_tx(
        dice: *mut snd_dice,
        offset: u32,
        buf: *mut core::ffi::c_void,
        len: usize,
    ) -> i32;
    fn snd_dice_transaction_read_rx(
        dice: *mut snd_dice,
        offset: u32,
        buf: *mut core::ffi::c_void,
        len: usize,
    ) -> i32;
    fn be32_to_cpu(value: __be32) -> u32;
}

extern "C" {
    static TX_NUMBER: u32;
    static RX_NUMBER: u32;
    static SND_DICE_RATE_MODE_LOW: usize;
    static SND_DICE_RATE_MODE_MIDDLE: usize;
}

pub type __be32 = u32;

#[repr(C)]
pub struct snd_dice {
    pub tx_pcm_chs: [[u32; 3]; 2],
    pub tx_midi_ports: [u32; 2],
    pub rx_pcm_chs: [[u32; 3]; 2],
    pub rx_midi_ports: [u32; 2],
}

#[no_mangle]
pub unsafe extern "C" fn snd_dice_detect_teac_formats(dice: *mut snd_dice) -> i32 {
    let mut reg: __be32 = 0;
    let mut data: u32;
    let mut err: i32;

    err = snd_dice_transaction_read_tx(
        dice,
        TX_NUMBER,
        &mut reg as *mut __be32 as *mut core::ffi::c_void,
        core::mem::size_of_val(&reg),
    );
    if err < 0 {
        return err;
    }

    (*dice).tx_pcm_chs[0][SND_DICE_RATE_MODE_LOW] = 16;
    (*dice).tx_pcm_chs[0][SND_DICE_RATE_MODE_MIDDLE] = 16;
    (*dice).tx_midi_ports[0] = 1;

    data = be32_to_cpu(reg);
    if data > 1 {
        (*dice).tx_pcm_chs[1][SND_DICE_RATE_MODE_LOW] = 16;
        (*dice).tx_pcm_chs[1][SND_DICE_RATE_MODE_MIDDLE] = 16;
    }

    err = snd_dice_transaction_read_rx(
        dice,
        RX_NUMBER,
        &mut reg as *mut __be32 as *mut core::ffi::c_void,
        core::mem::size_of_val(&reg),
    );
    if err < 0 {
        return err;
    }

    (*dice).rx_pcm_chs[0][SND_DICE_RATE_MODE_LOW] = 16;
    (*dice).rx_pcm_chs[0][SND_DICE_RATE_MODE_MIDDLE] = 16;
    (*dice).rx_midi_ports[0] = 1;

    data = be32_to_cpu(reg);
    if data > 1 {
        (*dice).rx_pcm_chs[1][SND_DICE_RATE_MODE_LOW] = 16;
        (*dice).rx_pcm_chs[1][SND_DICE_RATE_MODE_MIDDLE] = 16;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
