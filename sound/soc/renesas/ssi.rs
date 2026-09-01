// SPDX-License-Identifier: GPL-2.0
//
// Serial Sound Interface (I2S) support for SH7760/SH7780
//
// Copyright (c) 2007 Manuel Lauss <mano@roarinelk.homelinux.net>
//
// dont forget to set IPSEL/OMSEL register bits (in your board code) to
// enable SSI output pins!

/*
 * LIMITATIONS:
 *	The SSI unit has only one physical data line, so full duplex is
 *	impossible.  This can be remedied  on the  SH7760 by  using the
 *	other SSI unit for recording; however the SH7780 has only 1 SSI
 *	unit, and its pins are shared with the AC97 unit,  among others.
 *
 * FEATURES:
 *	The SSI features "compressed mode": in this mode it continuously
 *	streams PCM data over the I2S lines and uses LRCK as a handshake
 *	signal.  Can be used to send compressed data (AC3/DTS) to a DSP.
 *	The number of bits sent over the wire in a frame can be adjusted
 *	and can be independent from the actual sample bit depth. This is
 *	useful to support TDM mode codecs like the AD1939 which have a
 *	fixed TDM slot size, regardless of sample resolution.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong};
use core::ptr::{read_volatile, write_volatile};

const SSICR: c_ulong = 0x00;
const SSISR: c_ulong = 0x04;

const CR_DMAEN: c_ulong = 1 << 28;
const CR_CHNL_SHIFT: c_ulong = 22;
const CR_CHNL_MASK: c_ulong = 3 << CR_CHNL_SHIFT;
const CR_DWL_SHIFT: c_ulong = 19;
const CR_DWL_MASK: c_ulong = 7 << CR_DWL_SHIFT;
const CR_SWL_SHIFT: c_ulong = 16;
const CR_SWL_MASK: c_ulong = 7 << CR_SWL_SHIFT;
const CR_SCK_MASTER: c_ulong = 1 << 15; /* bitclock master bit */
const CR_SWS_MASTER: c_ulong = 1 << 14; /* wordselect master bit */
const CR_SCKP: c_ulong = 1 << 13; /* I2Sclock polarity */
const CR_SWSP: c_ulong = 1 << 12; /* LRCK polarity */
const CR_SPDP: c_ulong = 1 << 11;
const CR_SDTA: c_ulong = 1 << 10; /* i2s alignment (msb/lsb) */
const CR_PDTA: c_ulong = 1 << 9; /* fifo data alignment */
const CR_DEL: c_ulong = 1 << 8; /* delay data by 1 i2sclk */
const CR_BREN: c_ulong = 1 << 7; /* clock gating in burst mode */
const CR_CKDIV_SHIFT: c_ulong = 4;
const CR_CKDIV_MASK: c_ulong = 7 << CR_CKDIV_SHIFT; /* bitclock divider */
const CR_MUTE: c_ulong = 1 << 3; /* SSI mute */
const CR_CPEN: c_ulong = 1 << 2; /* compressed mode */
const CR_TRMD: c_ulong = 1 << 1; /* transmit/receive select */
const CR_EN: c_ulong = 1 << 0; /* enable SSI */

#[repr(C)]
struct ssi_priv {
    mmio: c_ulong,
    sysclk: c_ulong,
    inuse: c_int,
}

#[cfg(CONFIG_CPU_SUBTYPE_SH7760)]
static mut ssi_cpu_data: [ssi_priv; 2] = [
    ssi_priv {
        mmio: 0xFE680000,
        sysclk: 0,
        inuse: 0,
    },
    ssi_priv {
        mmio: 0xFE690000,
        sysclk: 0,
        inuse: 0,
    },
];

#[cfg(CONFIG_CPU_SUBTYPE_SH7780)]
static mut ssi_cpu_data: [ssi_priv; 1] = [ssi_priv {
    mmio: 0xFFE70000,
    sysclk: 0,
    inuse: 0,
}];

// C source emits #error "Unsupported SuperH SoC" when neither CPU subtype is configured.

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    pub msbits: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub id: c_int,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int,
    >,
    pub shutdown:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub trigger: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int,
    >,
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub set_sysclk:
        Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_clkdiv: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub rates: c_uint,
    pub formats: c_ulong,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub legacy_dai_naming: c_uint,
}

#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const c_char,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

unsafe extern "C" {
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn pr_debug(fmt: *const c_char, ...);
}

unsafe fn ssireg_read(ssi: *mut ssi_priv, reg: c_ulong) -> c_ulong {
    read_volatile(((*ssi).mmio + reg) as *const c_ulong)
}

unsafe fn ssireg_write(ssi: *mut ssi_priv, reg: c_ulong, val: c_ulong) {
    write_volatile(((*ssi).mmio + reg) as *mut c_ulong, val);
}

/*
 * track usage of the SSI; it is simplex-only so prevent attempts of
 * concurrent playback + capture. FIXME: any locking required?
 */
unsafe extern "C" fn ssi_startup(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let ssi = &mut ssi_cpu_data[(*dai).id as usize] as *mut ssi_priv;
    if (*ssi).inuse != 0 {
        pr_debug(c"ssi: already in use!\n".as_ptr());
        return -EBUSY;
    } else {
        (*ssi).inuse = 1;
    }
    0
}

unsafe extern "C" fn ssi_shutdown(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let ssi = &mut ssi_cpu_data[(*dai).id as usize] as *mut ssi_priv;

    (*ssi).inuse = 0;
}

unsafe extern "C" fn ssi_trigger(
    _substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let ssi = &mut ssi_cpu_data[(*dai).id as usize] as *mut ssi_priv;

    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            let val = ssireg_read(ssi, SSICR) | CR_DMAEN | CR_EN;
            ssireg_write(ssi, SSICR, val);
        }
        SNDRV_PCM_TRIGGER_STOP => {
            let val = ssireg_read(ssi, SSICR) & !(CR_DMAEN | CR_EN);
            ssireg_write(ssi, SSICR, val);
        }
        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn ssi_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let ssi = &mut ssi_cpu_data[(*dai).id as usize] as *mut ssi_priv;
    let mut ssicr: c_ulong = ssireg_read(ssi, SSICR);
    let mut bits: c_uint;
    let channels: c_uint;
    let swl: c_uint;
    let recv: c_uint;
    let mut i: c_uint;

    channels = params_channels(params);
    bits = (*params).msbits;
    recv = if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        0
    } else {
        1
    };

    pr_debug(c"ssi_hw_params() enter\nssicr was    %08lx\n".as_ptr(), ssicr);
    pr_debug(c"bits: %u channels: %u\n".as_ptr(), bits, channels);

    ssicr &= !(CR_TRMD | CR_CHNL_MASK | CR_DWL_MASK | CR_PDTA | CR_SWL_MASK);

    /* direction (send/receive) */
    if recv == 0 {
        ssicr |= CR_TRMD; /* transmit */
    }

    /* channels */
    if channels < 2 || channels > 8 || (channels & 1) != 0 {
        pr_debug(c"ssi: invalid number of channels\n".as_ptr());
        return -EINVAL;
    }
    ssicr |= (((channels >> 1) - 1) as c_ulong) << CR_CHNL_SHIFT;

    /* DATA WORD LENGTH (DWL): databits in audio sample */
    i = 0;
    match bits {
        32 => {
            i += 1;
            i += 1;
            i += 1;
            i += 1;
            i += 1;
            i += 1;
            ssicr |= (i as c_ulong) << CR_DWL_SHIFT;
        }
        24 => {
            i += 1;
            i += 1;
            i += 1;
            i += 1;
            i += 1;
            ssicr |= (i as c_ulong) << CR_DWL_SHIFT;
        }
        22 => {
            i += 1;
            i += 1;
            i += 1;
            i += 1;
            ssicr |= (i as c_ulong) << CR_DWL_SHIFT;
        }
        20 => {
            i += 1;
            i += 1;
            i += 1;
            ssicr |= (i as c_ulong) << CR_DWL_SHIFT;
        }
        18 => {
            i += 1;
            i += 1;
            ssicr |= (i as c_ulong) << CR_DWL_SHIFT;
        }
        16 => {
            i += 1;
            ssicr |= (i as c_ulong) << CR_DWL_SHIFT;
        }
        8 => {}
        _ => {
            pr_debug(c"ssi: invalid sample width\n".as_ptr());
            return -EINVAL;
        }
    }

    /*
     * SYSTEM WORD LENGTH: size in bits of half a frame over the I2S
     * wires. This is usually bits_per_sample x channels/2;  i.e. in
     * Stereo mode  the SWL equals DWL.  SWL can  be bigger than the
     * product of (channels_per_slot x samplebits), e.g.  for codecs
     * like the AD1939 which  only accept 32bit wide TDM slots.  For
     * "standard" I2S operation we set SWL = chans / 2 * DWL here.
     * Waiting for ASoC to get TDM support ;-)
     */
    if bits > 16 && bits <= 24 {
        bits = 24; /* these are padded by the SSI */
        /*ssicr |= CR_PDTA;*/ /* cpu/data endianness ? */
    }
    i = 0;
    swl = (bits * channels) / 2;
    match swl {
        256 => {
            i += 1;
            i += 1;
            i += 1;
            i += 1;
            i += 1;
            i += 1;
            ssicr |= (i as c_ulong) << CR_SWL_SHIFT;
        }
        128 => {
            i += 1;
            i += 1;
            i += 1;
            i += 1;
            i += 1;
            ssicr |= (i as c_ulong) << CR_SWL_SHIFT;
        }
        64 => {
            i += 1;
            i += 1;
            i += 1;
            i += 1;
            ssicr |= (i as c_ulong) << CR_SWL_SHIFT;
        }
        48 => {
            i += 1;
            i += 1;
            i += 1;
            ssicr |= (i as c_ulong) << CR_SWL_SHIFT;
        }
        32 => {
            i += 1;
            i += 1;
            ssicr |= (i as c_ulong) << CR_SWL_SHIFT;
        }
        16 => {
            i += 1;
            ssicr |= (i as c_ulong) << CR_SWL_SHIFT;
        }
        8 => {}
        _ => {
            pr_debug(c"ssi: invalid system word length computed\n".as_ptr());
            return -EINVAL;
        }
    }

    ssireg_write(ssi, SSICR, ssicr);

    pr_debug(c"ssi_hw_params() leave\nssicr is now %08lx\n".as_ptr(), ssicr);
    0
}

unsafe extern "C" fn ssi_set_sysclk(
    cpu_dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let ssi = &mut ssi_cpu_data[(*cpu_dai).id as usize] as *mut ssi_priv;

    (*ssi).sysclk = freq as c_ulong;

    0
}

/*
 * This divider is used to generate the SSI_SCK (I2S bitclock) from the
 * clock at the HAC_BIT_CLK ("oversampling clock") pin.
 */
unsafe extern "C" fn ssi_set_clkdiv(
    dai: *mut snd_soc_dai,
    _did: c_int,
    div: c_int,
) -> c_int {
    let ssi = &mut ssi_cpu_data[(*dai).id as usize] as *mut ssi_priv;
    let ssicr: c_ulong;
    let mut i: c_int;

    i = 0;
    ssicr = ssireg_read(ssi, SSICR) & !CR_CKDIV_MASK;
    match div {
        16 => {
            i += 1;
            i += 1;
            i += 1;
            i += 1;
            ssireg_write(ssi, SSICR, ssicr | ((i as c_ulong) << CR_CKDIV_SHIFT));
        }
        8 => {
            i += 1;
            i += 1;
            i += 1;
            ssireg_write(ssi, SSICR, ssicr | ((i as c_ulong) << CR_CKDIV_SHIFT));
        }
        4 => {
            i += 1;
            i += 1;
            ssireg_write(ssi, SSICR, ssicr | ((i as c_ulong) << CR_CKDIV_SHIFT));
        }
        2 => {
            i += 1;
            ssireg_write(ssi, SSICR, ssicr | ((i as c_ulong) << CR_CKDIV_SHIFT));
        }
        1 => {}
        _ => {
            pr_debug(c"ssi: invalid sck divider %d\n".as_ptr(), div);
            return -EINVAL;
        }
    }

    0
}

unsafe extern "C" fn ssi_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let ssi = &mut ssi_cpu_data[(*dai).id as usize] as *mut ssi_priv;
    let mut ssicr: c_ulong = ssireg_read(ssi, SSICR);

    pr_debug(c"ssi_set_fmt()\nssicr was    0x%08lx\n".as_ptr(), ssicr);

    ssicr &= !(CR_DEL
        | CR_PDTA
        | CR_BREN
        | CR_SWSP
        | CR_SCKP
        | CR_SWS_MASTER
        | CR_SCK_MASTER);

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {}
        SND_SOC_DAIFMT_RIGHT_J => {
            ssicr |= CR_DEL | CR_PDTA;
        }
        SND_SOC_DAIFMT_LEFT_J => {
            ssicr |= CR_DEL;
        }
        _ => {
            pr_debug(c"ssi: unsupported format\n".as_ptr());
            return -EINVAL;
        }
    }

    match fmt & SND_SOC_DAIFMT_CLOCK_MASK {
        SND_SOC_DAIFMT_CONT => {}
        SND_SOC_DAIFMT_GATED => {
            ssicr |= CR_BREN;
        }
        _ => {}
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {
            ssicr |= CR_SCKP; /* sample data at low clkedge */
        }
        SND_SOC_DAIFMT_NB_IF => {
            ssicr |= CR_SCKP | CR_SWSP;
        }
        SND_SOC_DAIFMT_IB_NF => {}
        SND_SOC_DAIFMT_IB_IF => {
            ssicr |= CR_SWSP; /* word select starts low */
        }
        _ => {
            pr_debug(c"ssi: invalid inversion\n".as_ptr());
            return -EINVAL;
        }
    }

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BC_FC => {}
        SND_SOC_DAIFMT_BP_FC => {
            ssicr |= CR_SCK_MASTER;
        }
        SND_SOC_DAIFMT_BC_FP => {
            ssicr |= CR_SWS_MASTER;
        }
        SND_SOC_DAIFMT_BP_FP => {
            ssicr |= CR_SWS_MASTER | CR_SCK_MASTER;
        }
        _ => {
            pr_debug(c"ssi: invalid master/secondary configuration\n".as_ptr());
            return -EINVAL;
        }
    }

    ssireg_write(ssi, SSICR, ssicr);
    pr_debug(c"ssi_set_fmt() leave\nssicr is now 0x%08lx\n".as_ptr(), ssicr);

    0
}

/* the SSI depends on an external clocksource (at HAC_BIT_CLK) even in
 * Master mode,  so really this is board specific;  the SSI can do any
 * rate with the right bitclk and divider settings.
 */
const SSI_RATES: c_uint = SNDRV_PCM_RATE_8000_192000;

/* the SSI can do 8-32 bit samples, with 8 possible channels */
const SSI_FMTS: c_ulong = SNDRV_PCM_FMTBIT_S8
    | SNDRV_PCM_FMTBIT_U8
    | SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_U16_LE
    | SNDRV_PCM_FMTBIT_S20_3LE
    | SNDRV_PCM_FMTBIT_U20_3LE
    | SNDRV_PCM_FMTBIT_S24_3LE
    | SNDRV_PCM_FMTBIT_U24_3LE
    | SNDRV_PCM_FMTBIT_S32_LE
    | SNDRV_PCM_FMTBIT_U32_LE;

static ssi_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(ssi_startup),
    shutdown: Some(ssi_shutdown),
    trigger: Some(ssi_trigger),
    hw_params: Some(ssi_hw_params),
    set_sysclk: Some(ssi_set_sysclk),
    set_clkdiv: Some(ssi_set_clkdiv),
    set_fmt: Some(ssi_set_fmt),
};

#[cfg(CONFIG_CPU_SUBTYPE_SH7760)]
static mut sh4_ssi_dai: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: c"ssi-dai.0".as_ptr(),
        playback: snd_soc_pcm_stream {
            rates: SSI_RATES,
            formats: SSI_FMTS,
            channels_min: 2,
            channels_max: 8,
        },
        capture: snd_soc_pcm_stream {
            rates: SSI_RATES,
            formats: SSI_FMTS,
            channels_min: 2,
            channels_max: 8,
        },
        ops: &ssi_dai_ops,
    },
    snd_soc_dai_driver {
        name: c"ssi-dai.1".as_ptr(),
        playback: snd_soc_pcm_stream {
            rates: SSI_RATES,
            formats: SSI_FMTS,
            channels_min: 2,
            channels_max: 8,
        },
        capture: snd_soc_pcm_stream {
            rates: SSI_RATES,
            formats: SSI_FMTS,
            channels_min: 2,
            channels_max: 8,
        },
        ops: &ssi_dai_ops,
    },
];

#[cfg(not(CONFIG_CPU_SUBTYPE_SH7760))]
static mut sh4_ssi_dai: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: c"ssi-dai.0".as_ptr(),
    playback: snd_soc_pcm_stream {
        rates: SSI_RATES,
        formats: SSI_FMTS,
        channels_min: 2,
        channels_max: 8,
    },
    capture: snd_soc_pcm_stream {
        rates: SSI_RATES,
        formats: SSI_FMTS,
        channels_min: 2,
        channels_max: 8,
    },
    ops: &ssi_dai_ops,
}];

static sh4_ssi_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c"sh4-ssi".as_ptr(),
    legacy_dai_naming: 1,
};

unsafe extern "C" fn sh4_soc_dai_probe(pdev: *mut platform_device) -> c_int {
    devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &sh4_ssi_component,
        sh4_ssi_dai.as_mut_ptr(),
        sh4_ssi_dai.len() as c_int,
    )
}

static mut sh4_ssi_driver: platform_driver = platform_driver {
    driver: platform_driver_inner {
        name: c"sh4-ssi-dai".as_ptr(),
    },

    probe: Some(sh4_soc_dai_probe),
};

// module_platform_driver(sh4_ssi_driver);
// MODULE_LICENSE("GPL v2");
// MODULE_DESCRIPTION("SuperH onchip SSI (I2S) audio driver");
// MODULE_AUTHOR("Manuel Lauss <mano@roarinelk.homelinux.net>");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
