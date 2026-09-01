// SPDX-License-Identifier: GPL-2.0
//
// Hitachi Audio Controller (AC97) support for SH7760/SH7780
//
// Copyright (c) 2007 Manuel Lauss <mano@roarinelk.homelinux.net>
//
// dont forget to set IPSEL/OMSEL register bits (in your board code) to
// enable HAC output pins!

/* BIG FAT FIXME: although the SH7760 has 2 independent AC97 units, only
 * the FIRST can be used since ASoC does not pass any information to the
 * ac97_read/write() functions regarding WHICH unit to use.  You'll have
 * to edit the code a bit to use the other AC97 unit.		--mlau
 */

// C dependencies:
// linux/init.h, linux/module.h, linux/platform_device.h, linux/interrupt.h,
// linux/wait.h, linux/delay.h, sound/core.h, sound/pcm.h, sound/ac97_codec.h,
// sound/initval.h, sound/soc.h

/* regs and bits */
const HACCR: usize = 0x08;
const HACCSAR: usize = 0x20;
const HACCSDR: usize = 0x24;
const HACPCML: usize = 0x28;
const HACPCMR: usize = 0x2C;
const HACTIER: usize = 0x50;
const HACTSR: usize = 0x54;
const HACRIER: usize = 0x58;
const HACRSR: usize = 0x5C;
const HACACR: usize = 0x60;

const CR_CR: libc::c_ulong = 1 << 15; /* "codec-ready" indicator */
const CR_CDRT: libc::c_ulong = 1 << 11; /* cold reset */
const CR_WMRT: libc::c_ulong = 1 << 10; /* warm reset */
const CR_B9: libc::c_ulong = 1 << 9; /* the mysterious "bit 9" */
const CR_ST: libc::c_ulong = 1 << 5; /* AC97 link start bit */

const CSAR_RD: libc::c_ulong = 1 << 19; /* AC97 data read bit */
const CSAR_WR: libc::c_ulong = 0;

const TSR_CMDAMT: libc::c_ulong = 1 << 31;
const TSR_CMDDMT: libc::c_ulong = 1 << 30;

const RSR_STARY: libc::c_ulong = 1 << 22;
const RSR_STDRY: libc::c_ulong = 1 << 21;

const ACR_DMARX16: libc::c_ulong = 1 << 30;
const ACR_DMATX16: libc::c_ulong = 1 << 29;
const ACR_TX12ATOM: libc::c_ulong = 1 << 26;
const ACR_DMARX20: libc::c_ulong = (1 << 24) | (1 << 22);
const ACR_DMATX20: libc::c_ulong = (1 << 23) | (1 << 21);

const CSDR_SHIFT: libc::c_uint = 4;
const CSDR_MASK: libc::c_ulong = 0xffff << CSDR_SHIFT;
const CSAR_SHIFT: libc::c_uint = 12;
const CSAR_MASK: libc::c_ulong = 0x7f << CSAR_SHIFT;

const AC97_WRITE_RETRY: libc::c_uint = 1;
const AC97_READ_RETRY: libc::c_uint = 5;

/* manual-suggested AC97 codec access timeouts (us) */
const TMO_E1: libc::c_uint = 500; /* 21 < E1 < 1000 */
const TMO_E2: libc::c_uint = 13; /* 13 < E2 */
const TMO_E3: libc::c_uint = 21; /* 21 < E3 */
const TMO_E4: libc::c_uint = 500; /* 21 < E4 < 1000 */

#[repr(C)]
struct hac_priv {
    mmio: libc::c_ulong, /* HAC base address */
}

// Original C selects this initializer with CONFIG_CPU_SUBTYPE_SH7760,
// CONFIG_CPU_SUBTYPE_SH7780, or emits #error for unsupported SuperH SoCs.
#[cfg(CONFIG_CPU_SUBTYPE_SH7760)]
static mut hac_cpu_data: [hac_priv; 2] = [
    hac_priv { mmio: 0xFE240000 },
    hac_priv { mmio: 0xFE250000 },
];

#[cfg(CONFIG_CPU_SUBTYPE_SH7780)]
static mut hac_cpu_data: [hac_priv; 1] = [hac_priv { mmio: 0xFFE40000 }];

extern "C" {
    fn udelay(usecs: libc::c_ulong);
    fn msleep(msecs: libc::c_uint);
    fn local_irq_disable();
    fn local_irq_enable();
    fn printk(fmt: *const libc::c_char, ...) -> libc::c_int;
    fn pr_debug(fmt: *const libc::c_char, ...);
    fn snd_soc_set_ac97_ops(ops: *mut snd_ac97_bus_ops) -> libc::c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: libc::c_int,
    ) -> libc::c_int;
}

#[repr(C)]
struct snd_ac97 {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_substream {
    stream: libc::c_int,
}

#[repr(C)]
struct snd_pcm_hw_params {
    msbits: libc::c_int,
}

#[repr(C)]
struct snd_soc_dai {
    id: libc::c_int,
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct platform_device {
    dev: device,
}

#[repr(C)]
struct snd_ac97_bus_ops {
    read: Option<unsafe extern "C" fn(*mut snd_ac97, libc::c_ushort) -> libc::c_ushort>,
    write: Option<unsafe extern "C" fn(*mut snd_ac97, libc::c_ushort, libc::c_ushort)>,
    reset: Option<unsafe extern "C" fn(*mut snd_ac97)>,
    warm_reset: Option<unsafe extern "C" fn(*mut snd_ac97)>,
}

#[repr(C)]
struct snd_soc_dai_ops {
    hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> libc::c_int,
    >,
}

#[repr(C)]
struct snd_soc_pcm_stream {
    rates: libc::c_uint,
    formats: libc::c_ulonglong,
    channels_min: libc::c_uint,
    channels_max: libc::c_uint,
}

#[repr(C)]
struct snd_soc_dai_driver {
    name: *const libc::c_char,
    id: libc::c_int,
    playback: snd_soc_pcm_stream,
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
}

#[repr(C)]
struct snd_soc_component_driver {
    name: *const libc::c_char,
    legacy_dai_naming: libc::c_uint,
}

#[repr(C)]
struct device_driver {
    name: *const libc::c_char,
}

#[repr(C)]
struct platform_driver {
    driver: device_driver,
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> libc::c_int>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

const SNDRV_PCM_STREAM_PLAYBACK: libc::c_int = 0;
const SNDRV_PCM_RATE_8000_192000: libc::c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: libc::c_ulonglong = 0;
const EINVAL: libc::c_int = 22;
const KERN_INFO: &[u8] = b"\0";

unsafe fn HACREG(hac: *mut hac_priv, reg: usize) -> *mut libc::c_ulong {
    ((*hac).mmio as usize + reg) as *mut libc::c_ulong
}

unsafe fn hacreg_read(hac: *mut hac_priv, reg: usize) -> libc::c_ulong {
    core::ptr::read_volatile(HACREG(hac, reg))
}

unsafe fn hacreg_write(hac: *mut hac_priv, reg: usize, val: libc::c_ulong) {
    core::ptr::write_volatile(HACREG(hac, reg), val);
}

unsafe fn hacreg_and(hac: *mut hac_priv, reg: usize, mask: libc::c_ulong) {
    let p = HACREG(hac, reg);
    core::ptr::write_volatile(p, core::ptr::read_volatile(p) & mask);
}

unsafe fn hacreg_or(hac: *mut hac_priv, reg: usize, bits: libc::c_ulong) {
    let p = HACREG(hac, reg);
    core::ptr::write_volatile(p, core::ptr::read_volatile(p) | bits);
}

/*
 * AC97 read/write flow as outlined in the SH7760 manual (pages 903-906)
 */
unsafe extern "C" fn hac_get_codec_data(
    hac: *mut hac_priv,
    r: libc::c_ushort,
    v: *mut libc::c_ushort,
) -> libc::c_int {
    let mut to1: libc::c_uint;
    let mut to2: libc::c_uint;
    let mut i: libc::c_uint;
    let mut adr: libc::c_ushort;

    i = AC97_READ_RETRY;
    while i != 0 {
        *v = 0;
        /* wait for HAC to receive something from the codec */
        to1 = TMO_E4;
        while to1 != 0 && (hacreg_read(hac, HACRSR) & RSR_STARY) == 0 {
            to1 = to1.wrapping_sub(1);
            udelay(1);
        }
        to2 = TMO_E4;
        while to2 != 0 && (hacreg_read(hac, HACRSR) & RSR_STDRY) == 0 {
            to2 = to2.wrapping_sub(1);
            udelay(1);
        }

        if to1 == 0 && to2 == 0 {
            return 0; /* codec comm is down */
        }

        adr = ((hacreg_read(hac, HACCSAR) & CSAR_MASK) >> CSAR_SHIFT) as libc::c_ushort;
        *v = ((hacreg_read(hac, HACCSDR) & CSDR_MASK) >> CSDR_SHIFT) as libc::c_ushort;

        hacreg_and(hac, HACRSR, !(RSR_STDRY | RSR_STARY));

        if r == adr {
            break;
        }

        /* manual says: wait at least 21 usec before retrying */
        udelay(21);
        i = i.wrapping_sub(1);
    }
    hacreg_and(hac, HACRSR, !(RSR_STDRY | RSR_STARY));
    i as libc::c_int
}

unsafe extern "C" fn hac_read_codec_aux(
    hac: *mut hac_priv,
    reg: libc::c_ushort,
) -> libc::c_ushort {
    let mut val: libc::c_ushort;
    let mut i: libc::c_uint;
    let mut to: libc::c_uint;

    i = AC97_READ_RETRY;
    while i != 0 {
        /* send_read_request */
        local_irq_disable();
        hacreg_and(hac, HACTSR, !TSR_CMDAMT);
        hacreg_write(hac, HACCSAR, ((reg as libc::c_ulong) << CSAR_SHIFT) | CSAR_RD);
        local_irq_enable();

        to = TMO_E3;
        while to != 0 && (hacreg_read(hac, HACTSR) & TSR_CMDAMT) == 0 {
            to = to.wrapping_sub(1);
            udelay(1);
        }

        hacreg_and(hac, HACTSR, !TSR_CMDAMT);
        val = 0;
        if hac_get_codec_data(hac, reg, &mut val) != 0 {
            break;
        }
        i = i.wrapping_sub(1);
    }

    if i != 0 {
        val
    } else {
        !0 as libc::c_ushort
    }
}

unsafe extern "C" fn hac_ac97_write(
    ac97: *mut snd_ac97,
    reg: libc::c_ushort,
    val: libc::c_ushort,
) {
    let unit_id: libc::c_int = 0; /* ac97->private_data */
    let hac: *mut hac_priv = &mut hac_cpu_data[unit_id as usize];
    let mut i: libc::c_uint;
    let mut to: libc::c_uint;
    let _ = ac97;
    /* write_codec_aux */
    i = AC97_WRITE_RETRY;
    while i != 0 {
        /* send_write_request */
        local_irq_disable();
        hacreg_and(hac, HACTSR, !(TSR_CMDDMT | TSR_CMDAMT));
        hacreg_write(hac, HACCSDR, (val as libc::c_ulong) << CSDR_SHIFT);
        hacreg_write(hac, HACCSAR, ((reg as libc::c_ulong) << CSAR_SHIFT) & !CSAR_RD);
        local_irq_enable();

        /* poll-wait for CMDAMT and CMDDMT */
        to = TMO_E1;
        while to != 0 && (hacreg_read(hac, HACTSR) & (TSR_CMDAMT | TSR_CMDDMT)) == 0 {
            to = to.wrapping_sub(1);
            udelay(1);
        }

        hacreg_and(hac, HACTSR, !(TSR_CMDAMT | TSR_CMDDMT));
        if to != 0 {
            break;
        }
        /* timeout, try again */
        i = i.wrapping_sub(1);
    }
}

unsafe extern "C" fn hac_ac97_read(
    ac97: *mut snd_ac97,
    reg: libc::c_ushort,
) -> libc::c_ushort {
    let unit_id: libc::c_int = 0; /* ac97->private_data */
    let hac: *mut hac_priv = &mut hac_cpu_data[unit_id as usize];
    let _ = ac97;
    hac_read_codec_aux(hac, reg)
}

unsafe extern "C" fn hac_ac97_warmrst(ac97: *mut snd_ac97) {
    let unit_id: libc::c_int = 0; /* ac97->private_data */
    let hac: *mut hac_priv = &mut hac_cpu_data[unit_id as usize];
    let mut tmo: libc::c_uint;
    let _ = ac97;

    hacreg_write(hac, HACCR, CR_WMRT | CR_ST | CR_B9);
    msleep(10);
    hacreg_write(hac, HACCR, CR_ST | CR_B9);
    tmo = 1000;
    while tmo > 0 && (hacreg_read(hac, HACCR) & CR_CR) == 0 {
        tmo = tmo.wrapping_sub(1);
        udelay(1);
    }

    if tmo == 0 {
        printk(
            concat!(core::str::from_utf8_unchecked(KERN_INFO), "hac: reset: AC97 link down!\n\0")
                .as_ptr() as *const libc::c_char,
        );
    }
    /* settings this bit lets us have a conversation with codec */
    hacreg_or(hac, HACACR, ACR_TX12ATOM);
}

unsafe extern "C" fn hac_ac97_coldrst(ac97: *mut snd_ac97) {
    let unit_id: libc::c_int = 0; /* ac97->private_data */
    let hac: *mut hac_priv;
    hac = &mut hac_cpu_data[unit_id as usize];

    hacreg_write(hac, HACCR, 0);
    hacreg_write(hac, HACCR, CR_CDRT | CR_ST | CR_B9);
    msleep(10);
    hac_ac97_warmrst(ac97);
}

static mut hac_ac97_ops: snd_ac97_bus_ops = snd_ac97_bus_ops {
    read: Some(hac_ac97_read),
    write: Some(hac_ac97_write),
    reset: Some(hac_ac97_coldrst),
    warm_reset: Some(hac_ac97_warmrst),
};

unsafe extern "C" fn hac_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> libc::c_int {
    let hac: *mut hac_priv = &mut hac_cpu_data[(*dai).id as usize];
    let d: libc::c_int = if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        0
    } else {
        1
    };

    match (*params).msbits {
        16 => {
            hacreg_or(hac, HACACR, if d != 0 { ACR_DMARX16 } else { ACR_DMATX16 });
            hacreg_and(hac, HACACR, if d != 0 { !ACR_DMARX20 } else { !ACR_DMATX20 });
        }
        20 => {
            hacreg_and(hac, HACACR, if d != 0 { !ACR_DMARX16 } else { !ACR_DMATX16 });
            hacreg_or(hac, HACACR, if d != 0 { ACR_DMARX20 } else { ACR_DMATX20 });
        }
        _ => {
            pr_debug(
                b"hac: invalid depth %d bit\n\0".as_ptr() as *const libc::c_char,
                (*params).msbits,
            );
            return -EINVAL;
        }
    }

    0
}

const AC97_RATES: libc::c_uint = SNDRV_PCM_RATE_8000_192000;

const AC97_FMTS: libc::c_ulonglong = SNDRV_PCM_FMTBIT_S16_LE;

static hac_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(hac_hw_params),
};

#[cfg(CONFIG_CPU_SUBTYPE_SH7760)]
static mut sh4_hac_dai: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: b"hac-dai.0\0".as_ptr() as *const libc::c_char,
        id: 0,
        playback: snd_soc_pcm_stream {
            rates: AC97_RATES,
            formats: AC97_FMTS,
            channels_min: 2,
            channels_max: 2,
        },
        capture: snd_soc_pcm_stream {
            rates: AC97_RATES,
            formats: AC97_FMTS,
            channels_min: 2,
            channels_max: 2,
        },
        ops: &hac_dai_ops,
    },
    snd_soc_dai_driver {
        name: b"hac-dai.1\0".as_ptr() as *const libc::c_char,
        id: 1,
        playback: snd_soc_pcm_stream {
            rates: AC97_RATES,
            formats: AC97_FMTS,
            channels_min: 2,
            channels_max: 2,
        },
        capture: snd_soc_pcm_stream {
            rates: AC97_RATES,
            formats: AC97_FMTS,
            channels_min: 2,
            channels_max: 2,
        },
        ops: &hac_dai_ops,
    },
];

#[cfg(not(CONFIG_CPU_SUBTYPE_SH7760))]
static mut sh4_hac_dai: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: b"hac-dai.0\0".as_ptr() as *const libc::c_char,
    id: 0,
    playback: snd_soc_pcm_stream {
        rates: AC97_RATES,
        formats: AC97_FMTS,
        channels_min: 2,
        channels_max: 2,
    },
    capture: snd_soc_pcm_stream {
        rates: AC97_RATES,
        formats: AC97_FMTS,
        channels_min: 2,
        channels_max: 2,
    },
    ops: &hac_dai_ops,
}];

static sh4_hac_component: snd_soc_component_driver = snd_soc_component_driver {
    name: b"sh4-hac\0".as_ptr() as *const libc::c_char,
    legacy_dai_naming: 1,
};

unsafe extern "C" fn hac_soc_platform_probe(pdev: *mut platform_device) -> libc::c_int {
    let mut ret: libc::c_int;

    ret = snd_soc_set_ac97_ops(&mut hac_ac97_ops);
    if ret != 0 {
        return ret;
    }

    devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &sh4_hac_component,
        sh4_hac_dai.as_mut_ptr(),
        sh4_hac_dai.len() as libc::c_int,
    )
}

unsafe extern "C" fn hac_soc_platform_remove(pdev: *mut platform_device) {
    let _ = pdev;
    snd_soc_set_ac97_ops(core::ptr::null_mut());
}

static mut hac_pcm_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"hac-pcm-audio\0".as_ptr() as *const libc::c_char,
    },
    probe: Some(hac_soc_platform_probe),
    remove: Some(hac_soc_platform_remove),
};

// module_platform_driver(hac_pcm_driver);

// MODULE_LICENSE("GPL v2");
// MODULE_DESCRIPTION("SuperH onchip HAC (AC97) audio driver");
// MODULE_AUTHOR("Manuel Lauss <mano@roarinelk.homelinux.net>");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
