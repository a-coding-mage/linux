// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// Copyright (c) 2021 Mediatek Corporation. All rights reserved.
//
// Author: YC Hung <yc.hung@mediatek.com>
//
// Hardware interface for mt8195 DSP code loader

// C dependencies:
// #include <sound/sof.h>
// #include "mt8195.h"
// #include "../../ops.h"

use crate::{
    snd_sof_dev, snd_sof_dsp_update_bits, snd_sof_dsp_write, udelay, ADSP_BRESET_SW,
    ADSP_DRESET_SW, ADSP_RUNSTALL, DSP_ALTRESETVEC, DSP_PDEBUGBUS0, DSP_REG_BAR, DSP_RESET_SW,
    PDEBUG_ENABLE, STATVECTOR_SEL,
};

#[no_mangle]
pub unsafe extern "C" fn sof_hifixdsp_boot_sequence(sdev: *mut snd_sof_dev, boot_addr: u32) {
    /* ADSP bootup base */
    unsafe {
        snd_sof_dsp_write(sdev, DSP_REG_BAR, DSP_ALTRESETVEC, boot_addr);
    }

    /* pull high RunStall (set bit3 to 1) */
    unsafe {
        snd_sof_dsp_update_bits(
            sdev,
            DSP_REG_BAR,
            DSP_RESET_SW,
            ADSP_RUNSTALL,
            ADSP_RUNSTALL,
        );
    }

    /* pull high StatVectorSel to use AltResetVec (set bit4 to 1) */
    unsafe {
        snd_sof_dsp_update_bits(
            sdev,
            DSP_REG_BAR,
            DSP_RESET_SW,
            STATVECTOR_SEL,
            STATVECTOR_SEL,
        );
    }

    /* toggle  DReset & BReset */
    /* pull high DReset & BReset */
    unsafe {
        snd_sof_dsp_update_bits(
            sdev,
            DSP_REG_BAR,
            DSP_RESET_SW,
            ADSP_BRESET_SW | ADSP_DRESET_SW,
            ADSP_BRESET_SW | ADSP_DRESET_SW,
        );
    }

    /* delay 10 DSP cycles at 26M about 1us by IP vendor's suggestion */
    unsafe {
        udelay(1);
    }

    /* pull low DReset & BReset */
    unsafe {
        snd_sof_dsp_update_bits(
            sdev,
            DSP_REG_BAR,
            DSP_RESET_SW,
            ADSP_BRESET_SW | ADSP_DRESET_SW,
            0,
        );
    }

    /* Enable PDebug */
    unsafe {
        snd_sof_dsp_update_bits(
            sdev,
            DSP_REG_BAR,
            DSP_PDEBUGBUS0,
            PDEBUG_ENABLE,
            PDEBUG_ENABLE,
        );
    }

    /* release RunStall (set bit3 to 0) */
    unsafe {
        snd_sof_dsp_update_bits(sdev, DSP_REG_BAR, DSP_RESET_SW, ADSP_RUNSTALL, 0);
    }
}

#[no_mangle]
pub unsafe extern "C" fn sof_hifixdsp_shutdown(sdev: *mut snd_sof_dev) {
    /* RUN_STALL pull high again to reset */
    unsafe {
        snd_sof_dsp_update_bits(
            sdev,
            DSP_REG_BAR,
            DSP_RESET_SW,
            ADSP_RUNSTALL,
            ADSP_RUNSTALL,
        );
    }

    /* pull high DReset & BReset */
    unsafe {
        snd_sof_dsp_update_bits(
            sdev,
            DSP_REG_BAR,
            DSP_RESET_SW,
            ADSP_BRESET_SW | ADSP_DRESET_SW,
            ADSP_BRESET_SW | ADSP_DRESET_SW,
        );
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
