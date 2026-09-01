// SPDX-License-Identifier: GPL-2.0-only
/*
 * DB1000/DB1500/DB1100 ASoC audio fabric support code.
 *
 * (c) 2011 Manuel Lauss <manuel.lauss@googlemail.com>
 *
 */

/* Dependencies from the original C includes:
 * linux/module.h, linux/moduleparam.h, linux/timer.h, linux/interrupt.h,
 * linux/platform_device.h, sound/core.h, sound/pcm.h, sound/soc.h,
 * asm/mach-au1x00/au1000.h, asm/mach-db1x00/bcsr.h, and "psc.h".
 */

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
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
    pub cpus: *mut snd_soc_dai_link_component,
    pub num_cpus: c_int,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_int,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: c_int,
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
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut module;
    static snd_soc_pm_ops: dev_pm_ops;

    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn platform_driver_register(driver: *mut platform_driver) -> c_int;
    fn platform_driver_unregister(driver: *mut platform_driver);
}

/*
 * SND_SOC_DAILINK_DEFS(hifi,
 *     DAILINK_COMP_ARRAY(COMP_CPU("alchemy-ac97c")),
 *     DAILINK_COMP_ARRAY(COMP_CODEC("ac97-codec", "ac97-hifi")),
 *     DAILINK_COMP_ARRAY(COMP_PLATFORM("alchemy-pcm-dma.0")));
 */
static mut hifi_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"alchemy-ac97c\0".as_ptr() as *const c_char,
    dai_name: core::ptr::null(),
}];

static mut hifi_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"ac97-codec\0".as_ptr() as *const c_char,
    dai_name: b"ac97-hifi\0".as_ptr() as *const c_char,
}];

static mut hifi_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"alchemy-pcm-dma.0\0".as_ptr() as *const c_char,
    dai_name: core::ptr::null(),
}];

static mut db1000_ac97_dai: snd_soc_dai_link = snd_soc_dai_link {
    name: b"AC97\0".as_ptr() as *const c_char,
    stream_name: b"AC97 HiFi\0".as_ptr() as *const c_char,
    cpus: unsafe { hifi_cpus.as_mut_ptr() },
    num_cpus: 1,
    codecs: unsafe { hifi_codecs.as_mut_ptr() },
    num_codecs: 1,
    platforms: unsafe { hifi_platforms.as_mut_ptr() },
    num_platforms: 1,
};

static mut db1000_ac97: snd_soc_card = snd_soc_card {
    name: b"DB1000_AC97\0".as_ptr() as *const c_char,
    owner: unsafe { THIS_MODULE },
    dai_link: unsafe { &raw mut db1000_ac97_dai },
    num_links: 1,
    dev: core::ptr::null_mut(),
};

unsafe extern "C" fn db1000_audio_probe(pdev: *mut platform_device) -> c_int {
    let card: *mut snd_soc_card = &raw mut db1000_ac97;
    unsafe {
        (*card).dev = &mut (*pdev).dev;
        devm_snd_soc_register_card(&mut (*pdev).dev, card)
    }
}

static mut db1000_audio_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"db1000-audio\0".as_ptr() as *const c_char,
        pm: unsafe { &snd_soc_pm_ops },
    },
    probe: Some(db1000_audio_probe),
};

/*
 * module_platform_driver(db1000_audio_driver);
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db1000_audio_init() -> c_int {
    unsafe { platform_driver_register(&raw mut db1000_audio_driver) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn db1000_audio_exit() {
    unsafe { platform_driver_unregister(&raw mut db1000_audio_driver) };
}

/*
 * MODULE_LICENSE("GPL");
 * MODULE_DESCRIPTION("DB1000/DB1500/DB1100 ASoC audio");
 * MODULE_AUTHOR("Manuel Lauss");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
