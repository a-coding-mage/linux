// SPDX-License-Identifier: GPL-2.0-only
/*
 * DB1200/DB1300/DB1550 ASoC audio fabric support code.
 *
 * (c) 2008-2011 Manuel Lauss <manuel.lauss@googlemail.com>
 *
 */

/* C dependencies:
 * linux/module.h, linux/moduleparam.h, linux/timer.h, linux/interrupt.h,
 * linux/platform_device.h, sound/core.h, sound/pcm.h, sound/soc.h,
 * asm/mach-au1x00/au1000.h, asm/mach-au1x00/au1xxx_psc.h,
 * asm/mach-au1x00/au1xxx_dbdma.h, asm/mach-db1x00/bcsr.h,
 * ../codecs/wm8731.h, psc.h
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct platform_device_id {
    pub name: [c_char; 32],
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_driver,
    pub id_table: *const platform_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub dai_fmt: c_uint,
    pub ops: *const snd_soc_ops,
    pub cpus: *const snd_soc_dai_link_component,
    pub num_cpus: c_uint,
    pub codecs: *const snd_soc_dai_link_component,
    pub num_codecs: c_uint,
    pub platforms: *const snd_soc_dai_link_component,
    pub num_platforms: c_uint,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub owner: *mut module,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub dev: *mut device,
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut module;
    static snd_soc_pm_ops: dev_pm_ops;

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn platform_get_device_id(pdev: *mut platform_device) -> *const platform_device_id;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
}

const WM8731_SYSCLK_XTAL: c_int = 0;
const SND_SOC_CLOCK_IN: c_int = 0;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0;

const fn platform_name(name: &[u8]) -> [c_char; 32] {
    let mut out = [0 as c_char; 32];
    let mut i = 0;

    while i < name.len() && i < 32 {
        out[i] = name[i] as c_char;
        i += 1;
    }

    out
}

static db1200_pids: [platform_device_id; 7] = [
    platform_device_id {
        name: platform_name(b"db1200-ac97"),
        driver_data: 0,
    },
    platform_device_id {
        name: platform_name(b"db1200-i2s"),
        driver_data: 1,
    },
    platform_device_id {
        name: platform_name(b"db1300-ac97"),
        driver_data: 2,
    },
    platform_device_id {
        name: platform_name(b"db1300-i2s"),
        driver_data: 3,
    },
    platform_device_id {
        name: platform_name(b"db1550-ac97"),
        driver_data: 4,
    },
    platform_device_id {
        name: platform_name(b"db1550-i2s"),
        driver_data: 5,
    },
    platform_device_id {
        name: [0; 32],
        driver_data: 0,
    },
];

/* MODULE_DEVICE_TABLE(platform, db1200_pids); */

/*-------------------------  AC97 PART  ---------------------------*/

static db1200_ac97_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c"au1xpsc_ac97.1".as_ptr(),
    dai_name: core::ptr::null(),
}];

static db1200_ac97_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c"ac97-codec.1".as_ptr(),
    dai_name: c"ac97-hifi".as_ptr(),
}];

static db1200_ac97_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c"au1xpsc-pcm.1".as_ptr(),
    dai_name: core::ptr::null(),
}];

static mut db1200_ac97_dai: snd_soc_dai_link = snd_soc_dai_link {
    name: c"AC97".as_ptr(),
    stream_name: c"AC97 HiFi".as_ptr(),
    dai_fmt: 0,
    ops: core::ptr::null(),
    cpus: db1200_ac97_cpus.as_ptr(),
    num_cpus: 1,
    codecs: db1200_ac97_codecs.as_ptr(),
    num_codecs: 1,
    platforms: db1200_ac97_platforms.as_ptr(),
    num_platforms: 1,
};

static mut db1200_ac97_machine: snd_soc_card = snd_soc_card {
    name: c"DB1200_AC97".as_ptr(),
    owner: unsafe { THIS_MODULE },
    dai_link: unsafe { &raw mut db1200_ac97_dai },
    num_links: 1,
    dev: core::ptr::null_mut(),
};

static db1300_ac97_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c"au1xpsc_ac97.1".as_ptr(),
    dai_name: core::ptr::null(),
}];

static db1300_ac97_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c"wm9712-codec.1".as_ptr(),
    dai_name: c"wm9712-hifi".as_ptr(),
}];

static db1300_ac97_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c"au1xpsc-pcm.1".as_ptr(),
    dai_name: core::ptr::null(),
}];

static mut db1300_ac97_dai: snd_soc_dai_link = snd_soc_dai_link {
    name: c"AC97".as_ptr(),
    stream_name: c"AC97 HiFi".as_ptr(),
    dai_fmt: 0,
    ops: core::ptr::null(),
    cpus: db1300_ac97_cpus.as_ptr(),
    num_cpus: 1,
    codecs: db1300_ac97_codecs.as_ptr(),
    num_codecs: 1,
    platforms: db1300_ac97_platforms.as_ptr(),
    num_platforms: 1,
};

static mut db1300_ac97_machine: snd_soc_card = snd_soc_card {
    name: c"DB1300_AC97".as_ptr(),
    owner: unsafe { THIS_MODULE },
    dai_link: unsafe { &raw mut db1300_ac97_dai },
    num_links: 1,
    dev: core::ptr::null_mut(),
};

static mut db1550_ac97_machine: snd_soc_card = snd_soc_card {
    name: c"DB1550_AC97".as_ptr(),
    owner: unsafe { THIS_MODULE },
    dai_link: unsafe { &raw mut db1200_ac97_dai },
    num_links: 1,
    dev: core::ptr::null_mut(),
};

/*-------------------------  I2S PART  ---------------------------*/

unsafe extern "C" fn db1200_i2s_startup(substream: *mut snd_pcm_substream) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = unsafe { snd_soc_substream_to_rtd(substream) };
    let codec_dai: *mut snd_soc_dai = unsafe { snd_soc_rtd_to_codec(rtd, 0) };

    /* WM8731 has its own 12MHz crystal */
    unsafe {
        snd_soc_dai_set_sysclk(
            codec_dai,
            WM8731_SYSCLK_XTAL,
            12000000,
            SND_SOC_CLOCK_IN,
        );
    }

    0
}

static db1200_i2s_wm8731_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(db1200_i2s_startup),
};

static db1200_i2s_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c"au1xpsc_i2s.1".as_ptr(),
    dai_name: core::ptr::null(),
}];

static db1200_i2s_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c"wm8731.0-001b".as_ptr(),
    dai_name: c"wm8731-hifi".as_ptr(),
}];

static db1200_i2s_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c"au1xpsc-pcm.1".as_ptr(),
    dai_name: core::ptr::null(),
}];

static mut db1200_i2s_dai: snd_soc_dai_link = snd_soc_dai_link {
    name: c"WM8731".as_ptr(),
    stream_name: c"WM8731 PCM".as_ptr(),
    dai_fmt: SND_SOC_DAIFMT_LEFT_J | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP,
    ops: &db1200_i2s_wm8731_ops,
    cpus: db1200_i2s_cpus.as_ptr(),
    num_cpus: 1,
    codecs: db1200_i2s_codecs.as_ptr(),
    num_codecs: 1,
    platforms: db1200_i2s_platforms.as_ptr(),
    num_platforms: 1,
};

static mut db1200_i2s_machine: snd_soc_card = snd_soc_card {
    name: c"DB1200_I2S".as_ptr(),
    owner: unsafe { THIS_MODULE },
    dai_link: unsafe { &raw mut db1200_i2s_dai },
    num_links: 1,
    dev: core::ptr::null_mut(),
};

static db1300_i2s_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c"au1xpsc_i2s.2".as_ptr(),
    dai_name: core::ptr::null(),
}];

static db1300_i2s_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c"wm8731.0-001b".as_ptr(),
    dai_name: c"wm8731-hifi".as_ptr(),
}];

static db1300_i2s_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c"au1xpsc-pcm.2".as_ptr(),
    dai_name: core::ptr::null(),
}];

static mut db1300_i2s_dai: snd_soc_dai_link = snd_soc_dai_link {
    name: c"WM8731".as_ptr(),
    stream_name: c"WM8731 PCM".as_ptr(),
    dai_fmt: SND_SOC_DAIFMT_LEFT_J | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP,
    ops: &db1200_i2s_wm8731_ops,
    cpus: db1300_i2s_cpus.as_ptr(),
    num_cpus: 1,
    codecs: db1300_i2s_codecs.as_ptr(),
    num_codecs: 1,
    platforms: db1300_i2s_platforms.as_ptr(),
    num_platforms: 1,
};

static mut db1300_i2s_machine: snd_soc_card = snd_soc_card {
    name: c"DB1300_I2S".as_ptr(),
    owner: unsafe { THIS_MODULE },
    dai_link: unsafe { &raw mut db1300_i2s_dai },
    num_links: 1,
    dev: core::ptr::null_mut(),
};

static db1550_i2s_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c"au1xpsc_i2s.3".as_ptr(),
    dai_name: core::ptr::null(),
}];

static db1550_i2s_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c"wm8731.0-001b".as_ptr(),
    dai_name: c"wm8731-hifi".as_ptr(),
}];

static db1550_i2s_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c"au1xpsc-pcm.3".as_ptr(),
    dai_name: core::ptr::null(),
}];

static mut db1550_i2s_dai: snd_soc_dai_link = snd_soc_dai_link {
    name: c"WM8731".as_ptr(),
    stream_name: c"WM8731 PCM".as_ptr(),
    dai_fmt: SND_SOC_DAIFMT_LEFT_J | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP,
    ops: &db1200_i2s_wm8731_ops,
    cpus: db1550_i2s_cpus.as_ptr(),
    num_cpus: 1,
    codecs: db1550_i2s_codecs.as_ptr(),
    num_codecs: 1,
    platforms: db1550_i2s_platforms.as_ptr(),
    num_platforms: 1,
};

static mut db1550_i2s_machine: snd_soc_card = snd_soc_card {
    name: c"DB1550_I2S".as_ptr(),
    owner: unsafe { THIS_MODULE },
    dai_link: unsafe { &raw mut db1550_i2s_dai },
    num_links: 1,
    dev: core::ptr::null_mut(),
};

/*-------------------------  COMMON PART  ---------------------------*/

static mut db1200_cards: [*mut snd_soc_card; 6] = unsafe {
    [
        &raw mut db1200_ac97_machine,
        &raw mut db1200_i2s_machine,
        &raw mut db1300_ac97_machine,
        &raw mut db1300_i2s_machine,
        &raw mut db1550_ac97_machine,
        &raw mut db1550_i2s_machine,
    ]
};

unsafe extern "C" fn db1200_audio_probe(pdev: *mut platform_device) -> c_int {
    let pid: *const platform_device_id = unsafe { platform_get_device_id(pdev) };
    let card: *mut snd_soc_card;

    card = unsafe { db1200_cards[(*pid).driver_data as usize] };
    unsafe {
        (*card).dev = &raw mut (*pdev).dev;
    }
    unsafe { devm_snd_soc_register_card(&raw mut (*pdev).dev, card) }
}

static mut db1200_audio_driver: platform_driver = platform_driver {
    driver: platform_driver_driver {
        name: c"db1200-ac97".as_ptr(),
        pm: unsafe { &snd_soc_pm_ops },
    },
    id_table: db1200_pids.as_ptr(),
    probe: Some(db1200_audio_probe),
};

/* module_platform_driver(db1200_audio_driver); */

/* MODULE_LICENSE("GPL"); */
/* MODULE_DESCRIPTION("DB1200/DB1300/DB1550 ASoC audio support"); */
/* MODULE_AUTHOR("Manuel Lauss"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
