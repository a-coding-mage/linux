// SPDX-License-Identifier: GPL-2.0+
//
// soc-devres.c  --  ALSA SoC Audio Layer devres functions
//
// Copyright (C) 2013 Linaro Ltd

// C dependencies: <linux/module.h>, <linux/moduleparam.h>, <sound/soc.h>,
// <sound/dmaengine_pcm.h>

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component_driver {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_card {
    pub devres_dev: *mut device,
}

#[repr(C)]
pub struct snd_dmaengine_pcm_config {
    _private: [u8; 0],
}

type DevresRelease = unsafe extern "C" fn(*mut device, *mut core::ffi::c_void);

const GFP_KERNEL: u32 = 0;
const ENOMEM: i32 = 12;

unsafe extern "C" {
    fn devres_alloc(
        release: DevresRelease,
        size: usize,
        gfp: u32,
    ) -> *mut core::ffi::c_void;
    fn devres_add(dev: *mut device, res: *mut core::ffi::c_void);
    fn devres_free(res: *mut core::ffi::c_void);

    fn snd_soc_unregister_component_by_driver(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
    );
    fn snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn snd_soc_register_card(card: *mut snd_soc_card) -> core::ffi::c_int;

    fn snd_dmaengine_pcm_unregister(dev: *mut device);
    fn snd_dmaengine_pcm_register(
        dev: *mut device,
        config: *const snd_dmaengine_pcm_config,
        flags: core::ffi::c_uint,
    ) -> core::ffi::c_int;
}

unsafe extern "C" fn devm_component_release(
    dev: *mut device,
    res: *mut core::ffi::c_void,
) {
    let cmpnt_drv = res as *mut *const snd_soc_component_driver;

    unsafe {
        snd_soc_unregister_component_by_driver(dev, *cmpnt_drv);
    }
}

/**
 * devm_snd_soc_register_component - resource managed component registration
 * @dev: Device used to manage component
 * @cmpnt_drv: Component driver
 * @dai_drv: DAI driver
 * @num_dai: Number of DAIs to register
 *
 * Register a component with automatic unregistration when the device is
 * unregistered.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn devm_snd_soc_register_component(
    dev: *mut device,
    cmpnt_drv: *const snd_soc_component_driver,
    dai_drv: *mut snd_soc_dai_driver,
    num_dai: core::ffi::c_int,
) -> core::ffi::c_int {
    let ptr: *mut *const snd_soc_component_driver;
    let ret: core::ffi::c_int;

    unsafe {
        ptr = devres_alloc(
            devm_component_release,
            core::mem::size_of::<*const snd_soc_component_driver>(),
            GFP_KERNEL,
        ) as *mut *const snd_soc_component_driver;
        if ptr.is_null() {
            return -ENOMEM;
        }

        ret = snd_soc_register_component(dev, cmpnt_drv, dai_drv, num_dai);
        if ret == 0 {
            *ptr = cmpnt_drv;
            devres_add(dev, ptr as *mut core::ffi::c_void);
        } else {
            devres_free(ptr as *mut core::ffi::c_void);
        }
    }

    ret
}
// EXPORT_SYMBOL_GPL(devm_snd_soc_register_component);

/**
 * devm_snd_soc_register_card - resource managed card registration
 * @dev: Device used to manage card
 * @card: Card to register
 *
 * Register a card with automatic unregistration when the device is
 * unregistered.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn devm_snd_soc_register_card(
    dev: *mut device,
    card: *mut snd_soc_card,
) -> core::ffi::c_int {
    unsafe {
        (*card).devres_dev = dev;
        snd_soc_register_card(card)
    }
}
// EXPORT_SYMBOL_GPL(devm_snd_soc_register_card);

// Original C condition: #ifdef CONFIG_SND_SOC_GENERIC_DMAENGINE_PCM

unsafe extern "C" fn devm_dmaengine_pcm_release(
    _dev: *mut device,
    res: *mut core::ffi::c_void,
) {
    unsafe {
        snd_dmaengine_pcm_unregister(*(res as *mut *mut device));
    }
}

/**
 * devm_snd_dmaengine_pcm_register - resource managed dmaengine PCM registration
 * @dev: The parent device for the PCM device
 * @config: Platform specific PCM configuration
 * @flags: Platform specific quirks
 *
 * Register a dmaengine based PCM device with automatic unregistration when the
 * device is unregistered.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn devm_snd_dmaengine_pcm_register(
    dev: *mut device,
    config: *const snd_dmaengine_pcm_config,
    flags: core::ffi::c_uint,
) -> core::ffi::c_int {
    let ptr: *mut *mut device;
    let ret: core::ffi::c_int;

    unsafe {
        ptr = devres_alloc(
            devm_dmaengine_pcm_release,
            core::mem::size_of::<*mut device>(),
            GFP_KERNEL,
        ) as *mut *mut device;
        if ptr.is_null() {
            return -ENOMEM;
        }

        ret = snd_dmaengine_pcm_register(dev, config, flags);
        if ret == 0 {
            *ptr = dev;
            devres_add(dev, ptr as *mut core::ffi::c_void);
        } else {
            devres_free(ptr as *mut core::ffi::c_void);
        }
    }

    ret
}
// EXPORT_SYMBOL_GPL(devm_snd_dmaengine_pcm_register);

// Original C condition end: #endif

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
