// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// Copyright (c) 2022 Mediatek Corporation. All rights reserved.
//
// Author: Allen-KH Cheng <allen-kh.cheng@mediatek.com>
//         Tinghan Shen <tinghan.shen@mediatek.com>
//
// Hardware interface for mt8186 DSP code loader

// C dependencies:
// #include <sound/sof.h>
// #include "mt8186.h"
// #include "../../ops.h"

use core::ffi::c_void;

#[repr(C)]
pub struct snd_sof_dev {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn snd_sof_dsp_update_bits(
        sdev: *mut snd_sof_dev,
        bar: u32,
        offset: u32,
        mask: u32,
        value: u32,
    ) -> c_void;
    fn snd_sof_dsp_write(sdev: *mut snd_sof_dev, bar: u32, offset: u32, value: u32) -> c_void;
    fn udelay(usecs: u64) -> c_void;
}

unsafe extern "C" {
    static DSP_REG_BAR: u32;
    static DSP_SECREG_BAR: u32;
    static ADSP_HIFI_IO_CONFIG: u32;
    static RUNSTALL: u32;
    static ADSP_MBOX_IRQ_EN: u32;
    static DSP_MBOX0_IRQ_EN: u32;
    static DSP_MBOX1_IRQ_EN: u32;
    static ADSP_ALTVEC_C0: u32;
    static ADSP_ALTVECSEL: u32;
    static ADSP_ALTVECSEL_C0: u32;
    static ADSP_CFGREG_SW_RSTN: u32;
    static SW_RSTN_C0: u32;
    static SW_DBG_RSTN_C0: u32;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mt8186_sof_hifixdsp_boot_sequence(
    sdev: *mut snd_sof_dev,
    boot_addr: u32,
) {
    /* set RUNSTALL to stop core */
    unsafe {
        snd_sof_dsp_update_bits(
            sdev,
            DSP_REG_BAR,
            ADSP_HIFI_IO_CONFIG,
            RUNSTALL,
            RUNSTALL,
        );
    }

    /* enable mbox 0 & 1 IRQ */
    unsafe {
        snd_sof_dsp_update_bits(
            sdev,
            DSP_REG_BAR,
            ADSP_MBOX_IRQ_EN,
            DSP_MBOX0_IRQ_EN | DSP_MBOX1_IRQ_EN,
            DSP_MBOX0_IRQ_EN | DSP_MBOX1_IRQ_EN,
        );
    }

    /* set core boot address */
    unsafe {
        snd_sof_dsp_write(sdev, DSP_SECREG_BAR, ADSP_ALTVEC_C0, boot_addr);
        snd_sof_dsp_write(sdev, DSP_SECREG_BAR, ADSP_ALTVECSEL, ADSP_ALTVECSEL_C0);
    }

    /* assert core reset */
    unsafe {
        snd_sof_dsp_update_bits(
            sdev,
            DSP_REG_BAR,
            ADSP_CFGREG_SW_RSTN,
            SW_RSTN_C0 | SW_DBG_RSTN_C0,
            SW_RSTN_C0 | SW_DBG_RSTN_C0,
        );
    }

    /* hardware requirement */
    unsafe {
        udelay(1);
    }

    /* release core reset */
    unsafe {
        snd_sof_dsp_update_bits(
            sdev,
            DSP_REG_BAR,
            ADSP_CFGREG_SW_RSTN,
            SW_RSTN_C0 | SW_DBG_RSTN_C0,
            0,
        );
    }

    /* clear RUNSTALL (bit31) to start core */
    unsafe {
        snd_sof_dsp_update_bits(sdev, DSP_REG_BAR, ADSP_HIFI_IO_CONFIG, RUNSTALL, 0);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mt8186_sof_hifixdsp_shutdown(sdev: *mut snd_sof_dev) {
    /* set RUNSTALL to stop core */
    unsafe {
        snd_sof_dsp_update_bits(
            sdev,
            DSP_REG_BAR,
            ADSP_HIFI_IO_CONFIG,
            RUNSTALL,
            RUNSTALL,
        );
    }

    /* assert core reset */
    unsafe {
        snd_sof_dsp_update_bits(
            sdev,
            DSP_REG_BAR,
            ADSP_CFGREG_SW_RSTN,
            SW_RSTN_C0 | SW_DBG_RSTN_C0,
            SW_RSTN_C0 | SW_DBG_RSTN_C0,
        );
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
