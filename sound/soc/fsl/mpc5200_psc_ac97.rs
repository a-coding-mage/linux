// SPDX-License-Identifier: GPL-2.0
//
// linux/sound/mpc5200-ac97.c -- AC97 support for the Freescale MPC52xx chip.
//
// Copyright (C) 2009 Jon Smirl, Digispeaker
// Author: Jon Smirl <jonsmirl@gmail.com>

// C dependencies:
// linux/module.h, linux/delay.h, linux/time.h
// sound/pcm.h, sound/pcm_params.h, sound/soc.h
// asm/time.h, asm/delay.h, asm/mpc52xx.h, asm/mpc52xx_psc.h
// "mpc5200_dma.h"

pub const DRV_NAME: &[u8] = b"mpc5200-psc-ac97\0";

/* ALSA only supports a single AC97 device so static is recommend here */
static mut psc_dma: *mut psc_dma = core::ptr::null_mut();

unsafe extern "C" {
    static psc_ac97_analog_ops: snd_soc_dai_ops;
    static psc_ac97_digital_ops: snd_soc_dai_ops;

    fn in_be16(addr: *const u16) -> u16;
    fn in_be32(addr: *const u32) -> u32;
    fn out_be16(addr: *mut u16, val: u16);
    fn out_be32(addr: *mut u32, val: u32);
    fn out_8(addr: *mut u8, val: u8);
    fn udelay(usecs: u64);
    fn usleep_range(min: u64, max: u64);
    fn pr_err(fmt: *const i8, ...);
    fn dev_dbg(dev: *mut device, fmt: *const i8, ...);
    fn dev_err(dev: *mut device, fmt: *const i8, ...);
    fn mpc5200_psc_ac97_gpio_reset(id: i32);
    fn snd_soc_dai_get_drvdata(cpu_dai: *mut snd_soc_dai) -> *mut psc_dma;
    fn to_psc_dma_stream(substream: *mut snd_pcm_substream, psc_dma: *mut psc_dma) -> *mut psc_dma_stream;
    fn params_period_size(params: *mut snd_pcm_hw_params) -> i32;
    fn params_period_bytes(params: *mut snd_pcm_hw_params) -> i32;
    fn params_periods(params: *mut snd_pcm_hw_params) -> i32;
    fn params_buffer_size(params: *mut snd_pcm_hw_params) -> i32;
    fn params_buffer_bytes(params: *mut snd_pcm_hw_params) -> i32;
    fn params_channels(params: *mut snd_pcm_hw_params) -> i32;
    fn params_rate(params: *mut snd_pcm_hw_params) -> i32;
    fn params_format(params: *mut snd_pcm_hw_params) -> i32;
    fn mpc5200_audio_dma_create(op: *mut platform_device) -> i32;
    fn mpc5200_audio_dma_destroy(op: *mut platform_device);
    fn snd_soc_set_ac97_ops(ops: *mut snd_ac97_bus_ops) -> i32;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: usize,
    ) -> i32;
    fn dev_get_drvdata(dev: *mut device) -> *mut psc_dma;
}

const ENODEV: i32 = 19;
const MPC52xx_PSC_SR_CMDSEND: u16 = 0;
const MPC52xx_PSC_SR_DATA_VAL: u16 = 0;
const MPC52xx_PSC_SICR_AWR: u32 = 0;
const MPC52xx_PSC_SICR_ACRB: u32 = 0;
const MPC52xx_PSC_TX_ENABLE: u8 = 0;
const MPC52xx_PSC_RX_ENABLE: u8 = 0;
const MPC52xx_PSC_SICR_SIM_AC97: u32 = 0;
const MPC52xx_PSC_SICR_ENAC97: u32 = 0;
const SNDRV_PCM_STREAM_CAPTURE: i32 = 0;
const SNDRV_PCM_TRIGGER_START: i32 = 0;
const SNDRV_PCM_TRIGGER_STOP: i32 = 0;
const SNDRV_PCM_RATE_8000_48000: u32 = 0;
const SNDRV_PCM_RATE_32000: u32 = 0;
const SNDRV_PCM_RATE_44100: u32 = 0;
const SNDRV_PCM_RATE_48000: u32 = 0;
const SNDRV_PCM_FMTBIT_S32_BE: u64 = 0;
const SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_BE: u64 = 0;

#[repr(C)]
pub struct mutex;
#[repr(C)]
pub struct snd_ac97;
#[repr(C)]
pub struct snd_pcm_hw_params;
#[repr(C)]
pub struct snd_soc_dai;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_str {
    pub stream: i32,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub pstr: *mut snd_pcm_str,
}

#[repr(C)]
pub struct psc_dma_stream {
    pub ac97_slot_bits: u32,
}

#[repr(C)]
pub struct sr_csr_regs {
    pub status: u16,
}

#[repr(C)]
pub struct isr_imr_regs {
    pub imr: u16,
}

#[repr(C)]
pub struct mpc52xx_psc {
    pub sr_csr: sr_csr_regs,
    pub ac97_data: u32,
    pub ac97_cmd: u32,
    pub sicr: u32,
    pub command: u8,
    pub ac97_slots: u32,
    pub isr_imr: isr_imr_regs,
}

#[repr(C)]
pub struct psc_dma {
    pub mutex: mutex,
    pub psc_regs: *mut mpc52xx_psc,
    pub sicr: u32,
    pub dev: *mut device,
    pub id: i32,
    pub slots: u32,
    pub imr: u16,
}

#[repr(C)]
pub struct snd_ac97_bus_ops {
    pub read: Option<unsafe extern "C" fn(*mut snd_ac97, u16) -> u16>,
    pub write: Option<unsafe extern "C" fn(*mut snd_ac97, u16, u16)>,
    pub reset: Option<unsafe extern "C" fn(*mut snd_ac97)>,
    pub warm_reset: Option<unsafe extern "C" fn(*mut snd_ac97)>,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> i32>,
    pub hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> i32,
    >,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, i32, *mut snd_soc_dai) -> i32>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const i8,
    pub channels_min: u32,
    pub channels_max: u32,
    pub rates: u32,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const i8,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const u8,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const i8,
}

#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const i8,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub driver: platform_driver_inner,
}

macro_rules! spin_event_timeout {
    ($cond:expr, $timeout:expr, $delay:expr) => {{
        let mut __left = $timeout;
        while !($cond) && __left > 0 {
            let _ = $delay;
            __left -= 1;
        }
        if $cond { __left } else { 0 }
    }};
}

unsafe extern "C" fn psc_ac97_read(_ac97: *mut snd_ac97, reg: u16) -> u16 {
    let status: i32;
    let mut val: u32;

    // guard(mutex)(&psc_dma->mutex);

    /* Wait for command send status zero = ready */
    status = spin_event_timeout!(
        (in_be16(&(*(*psc_dma).psc_regs).sr_csr.status as *const u16) & MPC52xx_PSC_SR_CMDSEND) == 0,
        100,
        0,
    );
    if status == 0 {
        pr_err(c"timeout on ac97 bus (rdy)\n".as_ptr());
        return (-ENODEV) as u16;
    }

    /* Force clear the data valid bit */
    in_be32(&(*(*psc_dma).psc_regs).ac97_data as *const u32);

    /* Send the read */
    out_be32(
        &mut (*(*psc_dma).psc_regs).ac97_cmd as *mut u32,
        (1u32 << 31) | (((reg & 0x7f) as u32) << 24),
    );

    /* Wait for the answer */
    status = spin_event_timeout!(
        (in_be16(&(*(*psc_dma).psc_regs).sr_csr.status as *const u16) & MPC52xx_PSC_SR_DATA_VAL) != 0,
        100,
        0,
    );
    if status == 0 {
        pr_err(
            c"timeout on ac97 read (val) %x\n".as_ptr(),
            in_be16(&(*(*psc_dma).psc_regs).sr_csr.status as *const u16) as i32,
        );
        return (-ENODEV) as u16;
    }
    /* Get the data */
    val = in_be32(&(*(*psc_dma).psc_regs).ac97_data as *const u32);
    if ((val >> 24) & 0x7f) != reg as u32 {
        pr_err(c"reg echo error on ac97 read\n".as_ptr());
        return (-ENODEV) as u16;
    }
    val = (val >> 8) & 0xffff;

    val as u16
}

unsafe extern "C" fn psc_ac97_write(_ac97: *mut snd_ac97, reg: u16, val: u16) {
    let status: i32;

    // guard(mutex)(&psc_dma->mutex);

    /* Wait for command status zero = ready */
    status = spin_event_timeout!(
        (in_be16(&(*(*psc_dma).psc_regs).sr_csr.status as *const u16) & MPC52xx_PSC_SR_CMDSEND) == 0,
        100,
        0,
    );
    if status == 0 {
        pr_err(c"timeout on ac97 bus (write)\n".as_ptr());
        return;
    }
    /* Write data */
    out_be32(
        &mut (*(*psc_dma).psc_regs).ac97_cmd as *mut u32,
        (((reg & 0x7f) as u32) << 24) | ((val as u32) << 8),
    );
}

unsafe extern "C" fn psc_ac97_warm_reset(_ac97: *mut snd_ac97) {
    let regs: *mut mpc52xx_psc = (*psc_dma).psc_regs;

    // guard(mutex)(&psc_dma->mutex);

    out_be32(&mut (*regs).sicr as *mut u32, (*psc_dma).sicr | MPC52xx_PSC_SICR_AWR);
    udelay(3);
    out_be32(&mut (*regs).sicr as *mut u32, (*psc_dma).sicr);
}

unsafe extern "C" fn psc_ac97_cold_reset(ac97: *mut snd_ac97) {
    let regs: *mut mpc52xx_psc = (*psc_dma).psc_regs;

    // scoped_guard(mutex, &psc_dma->mutex) {
    dev_dbg((*psc_dma).dev, c"cold reset\n".as_ptr());

    mpc5200_psc_ac97_gpio_reset((*psc_dma).id);

    /* Notify the PSC that a reset has occurred */
    out_be32(&mut (*regs).sicr as *mut u32, (*psc_dma).sicr | MPC52xx_PSC_SICR_ACRB);

    /* Re-enable RX and TX */
    out_8(
        &mut (*regs).command as *mut u8,
        MPC52xx_PSC_TX_ENABLE | MPC52xx_PSC_RX_ENABLE,
    );
    // }

    usleep_range(1000, 2000);
    psc_ac97_warm_reset(ac97);
}

static mut psc_ac97_ops: snd_ac97_bus_ops = snd_ac97_bus_ops {
    read: Some(psc_ac97_read),
    write: Some(psc_ac97_write),
    reset: Some(psc_ac97_cold_reset),
    warm_reset: Some(psc_ac97_warm_reset),
};

unsafe extern "C" fn psc_ac97_hw_analog_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    cpu_dai: *mut snd_soc_dai,
) -> i32 {
    let psc_dma: *mut psc_dma = snd_soc_dai_get_drvdata(cpu_dai);
    let s: *mut psc_dma_stream = to_psc_dma_stream(substream, psc_dma);

    dev_dbg(
        (*psc_dma).dev,
        c"%s(substream=%p) p_size=%i p_bytes=%i periods=%i buffer_size=%i  buffer_bytes=%i channels=%i rate=%i format=%i\n".as_ptr(),
        c"psc_ac97_hw_analog_params".as_ptr(),
        substream,
        params_period_size(params),
        params_period_bytes(params),
        params_periods(params),
        params_buffer_size(params),
        params_buffer_bytes(params),
        params_channels(params),
        params_rate(params),
        params_format(params),
    );

    /* Determine the set of enable bits to turn on */
    (*s).ac97_slot_bits = if params_channels(params) == 1 { 0x100 } else { 0x300 };
    if (*(*substream).pstr).stream != SNDRV_PCM_STREAM_CAPTURE {
        (*s).ac97_slot_bits <<= 16;
    }
    0
}

unsafe extern "C" fn psc_ac97_hw_digital_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    cpu_dai: *mut snd_soc_dai,
) -> i32 {
    let psc_dma: *mut psc_dma = snd_soc_dai_get_drvdata(cpu_dai);

    dev_dbg(
        (*psc_dma).dev,
        c"%s(substream=%p)\n".as_ptr(),
        c"psc_ac97_hw_digital_params".as_ptr(),
        substream,
    );

    if params_channels(params) == 1 {
        out_be32(&mut (*(*psc_dma).psc_regs).ac97_slots as *mut u32, 0x01000000);
    } else {
        out_be32(&mut (*(*psc_dma).psc_regs).ac97_slots as *mut u32, 0x03000000);
    }

    0
}

unsafe extern "C" fn psc_ac97_trigger(
    substream: *mut snd_pcm_substream,
    cmd: i32,
    dai: *mut snd_soc_dai,
) -> i32 {
    let psc_dma: *mut psc_dma = snd_soc_dai_get_drvdata(dai);
    let s: *mut psc_dma_stream = to_psc_dma_stream(substream, psc_dma);

    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            dev_dbg(
                (*psc_dma).dev,
                c"AC97 START: stream=%i\n".as_ptr(),
                (*(*substream).pstr).stream,
            );

            /* Set the slot enable bits */
            (*psc_dma).slots |= (*s).ac97_slot_bits;
            out_be32(&mut (*(*psc_dma).psc_regs).ac97_slots as *mut u32, (*psc_dma).slots);
        }

        SNDRV_PCM_TRIGGER_STOP => {
            dev_dbg(
                (*psc_dma).dev,
                c"AC97 STOP: stream=%i\n".as_ptr(),
                (*(*substream).pstr).stream,
            );

            /* Clear the slot enable bits */
            (*psc_dma).slots &= !((*s).ac97_slot_bits);
            out_be32(&mut (*(*psc_dma).psc_regs).ac97_slots as *mut u32, (*psc_dma).slots);
        }
        _ => {}
    }
    0
}

unsafe extern "C" fn psc_ac97_probe(cpu_dai: *mut snd_soc_dai) -> i32 {
    let psc_dma: *mut psc_dma = snd_soc_dai_get_drvdata(cpu_dai);
    let regs: *mut mpc52xx_psc = (*psc_dma).psc_regs;

    /* Go */
    out_8(
        &mut (*regs).command as *mut u8,
        MPC52xx_PSC_TX_ENABLE | MPC52xx_PSC_RX_ENABLE,
    );
    0
}

/* ---------------------------------------------------------------------
 * ALSA SoC Bindings
 *
 * - Digital Audio Interface (DAI) template
 * - create/destroy dai hooks
 */

/**
 * psc_ac97_dai_template: template CPU Digital Audio Interface
 */
static psc_ac97_analog_ops_value: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(psc_ac97_probe),
    hw_params: Some(psc_ac97_hw_analog_params),
    trigger: Some(psc_ac97_trigger),
};

static psc_ac97_digital_ops_value: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: None,
    hw_params: Some(psc_ac97_hw_digital_params),
    trigger: None,
};

static mut psc_ac97_dai: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: c"mpc5200-psc-ac97.0".as_ptr(),
        playback: snd_soc_pcm_stream {
            stream_name: c"AC97 Playback".as_ptr(),
            channels_min: 1,
            channels_max: 6,
            rates: SNDRV_PCM_RATE_8000_48000,
            formats: SNDRV_PCM_FMTBIT_S32_BE,
        },
        capture: snd_soc_pcm_stream {
            stream_name: c"AC97 Capture".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000_48000,
            formats: SNDRV_PCM_FMTBIT_S32_BE,
        },
        ops: &psc_ac97_analog_ops_value as *const snd_soc_dai_ops,
    },
    snd_soc_dai_driver {
        name: c"mpc5200-psc-ac97.1".as_ptr(),
        playback: snd_soc_pcm_stream {
            stream_name: c"AC97 SPDIF".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000,
            formats: SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_BE,
        },
        capture: snd_soc_pcm_stream {
            stream_name: core::ptr::null(),
            channels_min: 0,
            channels_max: 0,
            rates: 0,
            formats: 0,
        },
        ops: &psc_ac97_digital_ops_value as *const snd_soc_dai_ops,
    },
];

static psc_ac97_component: snd_soc_component_driver = snd_soc_component_driver {
    name: DRV_NAME.as_ptr(),
};

/* ---------------------------------------------------------------------
 * OF platform bus binding code:
 * - Probe/remove operations
 * - OF device match table
 */
unsafe extern "C" fn psc_ac97_of_probe(op: *mut platform_device) -> i32 {
    let mut rc: i32;
    let regs: *mut mpc52xx_psc;

    rc = mpc5200_audio_dma_create(op);
    if rc != 0 {
        return rc;
    }

    rc = snd_soc_set_ac97_ops(&mut psc_ac97_ops as *mut snd_ac97_bus_ops);
    if rc != 0 {
        dev_err(&mut (*op).dev as *mut device, c"Failed to set AC'97 ops: %d\n".as_ptr(), rc);
        return rc;
    }

    rc = devm_snd_soc_register_component(
        &mut (*op).dev as *mut device,
        &psc_ac97_component as *const snd_soc_component_driver,
        psc_ac97_dai.as_mut_ptr(),
        psc_ac97_dai.len(),
    );
    if rc != 0 {
        dev_err(&mut (*op).dev as *mut device, c"Failed to register DAI\n".as_ptr());
        return rc;
    }

    psc_dma = dev_get_drvdata(&mut (*op).dev as *mut device);
    regs = (*psc_dma).psc_regs;

    (*psc_dma).imr = 0;
    out_be16(&mut (*(*psc_dma).psc_regs).isr_imr.imr as *mut u16, (*psc_dma).imr);

    /* Configure the serial interface mode to AC97 */
    (*psc_dma).sicr = MPC52xx_PSC_SICR_SIM_AC97 | MPC52xx_PSC_SICR_ENAC97;
    out_be32(&mut (*regs).sicr as *mut u32, (*psc_dma).sicr);

    /* No slots active */
    out_be32(&mut (*regs).ac97_slots as *mut u32, 0x00000000);

    0
}

unsafe extern "C" fn psc_ac97_of_remove(op: *mut platform_device) {
    mpc5200_audio_dma_destroy(op);
}

/* Match table for of_platform binding */
static psc_ac97_match: [of_device_id; 3] = [
    of_device_id {
        compatible: c"fsl,mpc5200-psc-ac97".as_ptr(),
    },
    of_device_id {
        compatible: c"fsl,mpc5200b-psc-ac97".as_ptr(),
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, psc_ac97_match);

static mut psc_ac97_driver: platform_driver = platform_driver {
    probe: Some(psc_ac97_of_probe),
    remove: Some(psc_ac97_of_remove),
    driver: platform_driver_inner {
        name: c"mpc5200-psc-ac97".as_ptr(),
        of_match_table: psc_ac97_match.as_ptr(),
    },
};

// module_platform_driver(psc_ac97_driver);

// MODULE_AUTHOR("Jon Smirl <jonsmirl@gmail.com>");
// MODULE_DESCRIPTION("mpc5200 AC97 module");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
