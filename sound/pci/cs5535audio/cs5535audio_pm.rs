// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Power management for audio on multifunction CS5535 companion device
 * Copyright (C) Jaya Kumar
 */

// C includes translated as external dependencies:
// linux/init.h, linux/pci.h, linux/delay.h, sound/core.h, sound/control.h,
// sound/initval.h, sound/asoundef.h, sound/pcm.h, sound/ac97_codec.h,
// "cs5535audio.h"

use core::ffi::{c_char, c_int, c_void};

type u32 = u32;

#[repr(C)]
pub struct device {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub private_data: *mut cs5535audio,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_ac97 {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream_ops {
    pub prepare: Option<unsafe extern "C" fn(substream: *mut snd_pcm_substream) -> c_int>,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub ops: *mut snd_pcm_substream_ops,
}

#[repr(C)]
pub struct cs5535audio_dma_ops {
    pub read_prd: Option<unsafe extern "C" fn(cs5535au: *mut cs5535audio) -> u32>,
    pub setup_prd: Option<unsafe extern "C" fn(cs5535au: *mut cs5535audio, prd: u32)>,
}

#[repr(C)]
pub struct cs5535audio_dma {
    pub substream: *mut snd_pcm_substream,
    pub ops: *mut cs5535audio_dma_ops,
    pub saved_prd: u32,
}

#[repr(C)]
pub struct cs5535audio {
    pub card: *mut snd_card,
    pub ac97: *mut snd_ac97,
    pub dmas: [cs5535audio_dma; NUM_CS5535AUDIO_DMAS],
}

extern "C" {
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_power_change_state(card: *mut snd_card, state: c_int) -> c_int;
    fn snd_ac97_suspend(ac97: *mut snd_ac97);
    fn snd_ac97_resume(ac97: *mut snd_ac97);
    fn cs_writel(cs5535au: *mut cs5535audio, reg: u32, val: u32);
    fn cs_readl(cs5535au: *mut cs5535audio, reg: u32) -> u32;
    fn udelay(usecs: u32);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

extern "C" {
    static ACC_CODEC_CNTL: u32;
    static ACC_CODEC_CNTL_LNK_SHUTDOWN: u32;
    static ACC_CODEC_CNTL_LNK_WRM_RST: u32;
    static ACC_CODEC_STATUS: u32;
    static PRM_RDY_STS: u32;
    static SNDRV_CTL_POWER_D3hot: c_int;
    static SNDRV_CTL_POWER_D0: c_int;
}

const NUM_CS5535AUDIO_DMAS: usize = 0; // Provided by cs5535audio.h in the complete build.

unsafe fn snd_cs5535audio_stop_hardware(cs5535au: *mut cs5535audio) {
    /*
    we depend on snd_ac97_suspend to tell the
    AC97 codec to shutdown. the amd spec suggests
    that the LNK_SHUTDOWN be done at the same time
    that the codec power-down is issued. instead,
    we do it just after rather than at the same
    time. excluding codec specific build_ops->suspend
    ac97 powerdown hits:
    0x8000 EAPD
    0x4000 Headphone amplifier
    0x0300 ADC & DAC
    0x0400 Analog Mixer powerdown (Vref on)
    I am not sure if this is the best that we can do.
    The remainder to be investigated are:
    - analog mixer (vref off) 0x0800
    - AC-link powerdown 0x1000
    - codec internal clock 0x2000
    */

    /* set LNK_SHUTDOWN to shutdown AC link */
    cs_writel(cs5535au, ACC_CODEC_CNTL, ACC_CODEC_CNTL_LNK_SHUTDOWN);
}

unsafe fn snd_cs5535audio_suspend(dev: *mut device) -> c_int {
    let card = dev_get_drvdata(dev) as *mut snd_card;
    let cs5535au = (*card).private_data;
    let mut i: c_int;

    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
    snd_ac97_suspend((*cs5535au).ac97);
    i = 0;
    while i < NUM_CS5535AUDIO_DMAS as c_int {
        let dma = &mut (*cs5535au).dmas[i as usize] as *mut cs5535audio_dma;
        if !dma.is_null() && !(*dma).substream.is_null() {
            (*dma).saved_prd = ((*(*dma).ops).read_prd.unwrap())(cs5535au);
        }
        i += 1;
    }
    /* save important regs, then disable aclink in hw */
    snd_cs5535audio_stop_hardware(cs5535au);
    0
}

unsafe fn snd_cs5535audio_resume(dev: *mut device) -> c_int {
    let card = dev_get_drvdata(dev) as *mut snd_card;
    let cs5535au = (*card).private_data;
    let mut tmp: u32;
    let mut timeout: c_int;
    let mut i: c_int;

    /* set LNK_WRM_RST to reset AC link */
    cs_writel(cs5535au, ACC_CODEC_CNTL, ACC_CODEC_CNTL_LNK_WRM_RST);

    timeout = 50;
    loop {
        tmp = cs_readl(cs5535au, ACC_CODEC_STATUS);
        if tmp & PRM_RDY_STS != 0 {
            break;
        }
        udelay(1);
        timeout -= 1;
        if timeout == 0 {
            break;
        }
    }

    if timeout == 0 {
        dev_err(
            (*(*cs5535au).card).dev,
            b"Failure getting AC Link ready\n\0".as_ptr() as *const c_char,
        );
    }

    /* set up rate regs, dma. actual initiation is done in trig */
    i = 0;
    while i < NUM_CS5535AUDIO_DMAS as c_int {
        let dma = &mut (*cs5535au).dmas[i as usize] as *mut cs5535audio_dma;
        if !dma.is_null() && !(*dma).substream.is_null() {
            ((*(*(*dma).substream).ops).prepare.unwrap())((*dma).substream);
            ((*(*dma).ops).setup_prd.unwrap())(cs5535au, (*dma).saved_prd);
        }
        i += 1;
    }

    /* we depend on ac97 to perform the codec power up */
    snd_ac97_resume((*cs5535au).ac97);
    snd_power_change_state(card, SNDRV_CTL_POWER_D0);

    0
}

// SIMPLE_DEV_PM_OPS(snd_cs5535audio_pm, snd_cs5535audio_suspend, snd_cs5535audio_resume);
// The Linux C macro creates device PM ops wiring for the suspend/resume callbacks.

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
