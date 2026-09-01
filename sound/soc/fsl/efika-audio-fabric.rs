// SPDX-License-Identifier: GPL-2.0
/*
 * Efika driver for the PSC of the Freescale MPC52xx
 * configured as AC97 interface
 *
 * Copyright 2008 Jon Smirl, Digispeaker
 * Author: Jon Smirl <jonsmirl@gmail.com>
 */

// C dependencies:
// linux/init.h, linux/module.h, linux/interrupt.h, linux/device.h,
// linux/delay.h, linux/of.h, linux/platform_device.h, linux/dma-mapping.h,
// sound/core.h, sound/pcm.h, sound/pcm_params.h, sound/initval.h, sound/soc.h

pub const DRV_NAME: &[u8] = b"efika-audio-fabric\0";

extern "C" {
    static mut THIS_MODULE: *mut module;

    fn of_machine_is_compatible(compat: *const core::ffi::c_char) -> core::ffi::c_int;
    fn platform_device_alloc(
        name: *const core::ffi::c_char,
        id: core::ffi::c_int,
    ) -> *mut platform_device;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut core::ffi::c_void);
    fn platform_device_add(pdev: *mut platform_device) -> core::ffi::c_int;
    fn platform_device_put(pdev: *mut platform_device);
    fn pr_err(fmt: *const core::ffi::c_char, ...);
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const core::ffi::c_char,
    pub stream_name: *const core::ffi::c_char,
    // Fields generated in C by SND_SOC_DAILINK_REG(...) are supplied externally.
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const core::ffi::c_char,
    pub owner: *mut module,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: core::ffi::c_int,
}

extern "C" {
    static mut analog: snd_soc_dai_link_component_definition;
    static mut iec958: snd_soc_dai_link_component_definition;
}

#[repr(C)]
pub struct snd_soc_dai_link_component_definition {
    _private: [u8; 0],
}

// SND_SOC_DAILINK_DEFS(analog,
//     DAILINK_COMP_ARRAY(COMP_CPU("mpc5200-psc-ac97.0")),
//     DAILINK_COMP_ARRAY(COMP_CODEC("stac9766-codec",
//                                   "stac9766-hifi-analog")),
//     DAILINK_COMP_ARRAY(COMP_PLATFORM("mpc5200-pcm-audio")));
//
// SND_SOC_DAILINK_DEFS(iec958,
//     DAILINK_COMP_ARRAY(COMP_CPU("mpc5200-psc-ac97.1")),
//     DAILINK_COMP_ARRAY(COMP_CODEC("stac9766-codec",
//                                   "stac9766-hifi-IEC958")),
//     DAILINK_COMP_ARRAY(COMP_PLATFORM("mpc5200-pcm-audio")));

#[no_mangle]
pub static mut efika_fabric_dai: [snd_soc_dai_link; 2] = [
    snd_soc_dai_link {
        name: b"AC97\0".as_ptr() as *const core::ffi::c_char,
        stream_name: b"AC97 Analog\0".as_ptr() as *const core::ffi::c_char,
        // SND_SOC_DAILINK_REG(analog),
    },
    snd_soc_dai_link {
        name: b"AC97\0".as_ptr() as *const core::ffi::c_char,
        stream_name: b"AC97 IEC958\0".as_ptr() as *const core::ffi::c_char,
        // SND_SOC_DAILINK_REG(iec958),
    },
];

#[no_mangle]
pub static mut card: snd_soc_card = unsafe {
    snd_soc_card {
        name: b"Efika\0".as_ptr() as *const core::ffi::c_char,
        owner: THIS_MODULE,
        dai_link: efika_fabric_dai.as_mut_ptr(),
        num_links: efika_fabric_dai.len() as core::ffi::c_int,
    }
};

pub const ENODEV: core::ffi::c_int = 19;

#[no_mangle]
pub unsafe extern "C" fn efika_fabric_init() -> core::ffi::c_int {
    let mut pdev: *mut platform_device;
    let rc: core::ffi::c_int;

    if of_machine_is_compatible(b"bplan,efika\0".as_ptr() as *const core::ffi::c_char) == 0 {
        return -ENODEV;
    }

    pdev = platform_device_alloc(b"soc-audio\0".as_ptr() as *const core::ffi::c_char, 1);
    if pdev.is_null() {
        pr_err(
            b"efika_fabric_init: platform_device_alloc() failed\n\0".as_ptr()
                as *const core::ffi::c_char,
        );
        return -ENODEV;
    }

    platform_set_drvdata(pdev, &mut card as *mut snd_soc_card as *mut core::ffi::c_void);

    rc = platform_device_add(pdev);
    if rc != 0 {
        pr_err(
            b"efika_fabric_init: platform_device_add() failed\n\0".as_ptr()
                as *const core::ffi::c_char,
        );
        platform_device_put(pdev);
        return -ENODEV;
    }
    return 0;
}

// module_init(efika_fabric_init);

// MODULE_AUTHOR("Jon Smirl <jonsmirl@gmail.com>");
// MODULE_DESCRIPTION(DRV_NAME ": mpc5200 Efika fabric driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
