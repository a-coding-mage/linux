// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  hdac_i915.c - routines for sync between HD-A core and i915 display driver
 */

/* C includes translated as external dependencies:
 * <linux/init.h>, <linux/module.h>, <linux/pci.h>, <sound/core.h>,
 * <sound/hdaudio.h>, <sound/hda_i915.h>, <sound/hda_register.h>,
 * <video/nomodeset.h>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const PCI_VENDOR_ID_INTEL: c_uint = 0x8086;
const PCI_ANY_ID: c_uint = !0;
const I915_COMPONENT_AUDIO: c_int = 1;
const HSW_EM4: c_uint = 0;
const HSW_EM5: c_uint = 0;
const ENODEV: c_int = 19;
const EPROBE_DEFER: c_int = 517;

static mut gpu_bind: c_int = -1;
/* module_param(gpu_bind, int, 0644); */
/* MODULE_PARM_DESC(gpu_bind, "Whether to bind sound component to GPU "
 *                            "(1=always, 0=never, -1=on nomodeset(default))");
 */

#[repr(C)]
pub struct device {
    pub driver: *mut device_driver,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
}

#[repr(C)]
pub struct pci_bus {
    pub parent: *mut pci_bus,
}

#[repr(C)]
pub struct pci_dev {
    pub bus: *mut pci_bus,
    pub vendor: c_uint,
}

#[repr(C)]
pub struct hdac_bus {
    pub audio_component: *mut drm_audio_component,
    pub dev: *mut device,
}

#[repr(C)]
pub struct drm_audio_component {
    pub dev: *mut device,
    pub ops: *mut drm_audio_component_ops,
}

#[repr(C)]
pub struct drm_audio_component_ops {
    pub get_cdclk_freq: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
pub struct i915_audio_component {
    pub base: drm_audio_component,
}

#[repr(C)]
pub struct pci_device_id {
    pub vendor: c_uint,
    pub device: c_uint,
    pub subvendor: c_uint,
    pub subdevice: c_uint,
    pub class: c_uint,
    pub class_mask: c_uint,
    pub driver_data: usize,
}

unsafe extern "C" {
    fn to_pci_dev(dev: *mut device) -> *mut pci_dev;
    fn HDA_CONTROLLER_IS_HSW(pci: *mut pci_dev) -> bool;
    fn snd_hdac_chip_writew(bus: *mut hdac_bus, reg: c_uint, value: c_uint);
    fn dev_is_pci(dev: *mut device) -> bool;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn video_firmware_drivers_only() -> bool;
    fn pci_is_display(dev: *mut pci_dev) -> bool;
    fn pci_match_id(ids: *const pci_device_id, dev: *mut pci_dev) -> *const pci_device_id;
    fn pci_dev_put(dev: *mut pci_dev);
    fn pci_get_device(vendor: c_uint, device: c_uint, from: *mut pci_dev) -> *mut pci_dev;
    fn snd_hdac_acomp_init(
        bus: *mut hdac_bus,
        match_master: *mut c_void,
        match_component: Option<unsafe extern "C" fn(*mut device, c_int, *mut c_void) -> c_int>,
        extra_size: usize,
    ) -> c_int;
    fn snd_hdac_acomp_exit(bus: *mut hdac_bus);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
}

const fn PCI_DEVICE(vendor: c_uint, device: c_uint) -> pci_device_id {
    pci_device_id {
        vendor,
        device,
        subvendor: PCI_ANY_ID,
        subdevice: PCI_ANY_ID,
        class: 0,
        class_mask: 0,
        driver_data: 0,
    }
}

/**
 * snd_hdac_i915_set_bclk - Reprogram BCLK for HSW/BDW
 * @bus: HDA core bus
 *
 * Intel HSW/BDW display HDA controller is in GPU. Both its power and link BCLK
 * depends on GPU. Two Extended Mode registers EM4 (M value) and EM5 (N Value)
 * are used to convert CDClk (Core Display Clock) to 24MHz BCLK:
 * BCLK = CDCLK * M / N
 * The values will be lost when the display power well is disabled and need to
 * be restored to avoid abnormal playback speed.
 *
 * Call this function at initializing and changing power well, as well as
 * at ELD notifier for the hotplug.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_i915_set_bclk(bus: *mut hdac_bus) {
    let acomp: *mut drm_audio_component = unsafe { (*bus).audio_component };
    let pci: *mut pci_dev = unsafe { to_pci_dev((*bus).dev) };
    let cdclk_freq: c_int;
    let bclk_m: c_uint;
    let bclk_n: c_uint;

    if acomp.is_null()
        || unsafe { (*acomp).ops.is_null() }
        || unsafe { (*(*acomp).ops).get_cdclk_freq.is_none() }
    {
        return; /* only for i915 binding */
    }
    if unsafe { !HDA_CONTROLLER_IS_HSW(pci) } {
        return; /* only HSW/BDW */
    }

    cdclk_freq = unsafe { ((*(*acomp).ops).get_cdclk_freq.unwrap())((*acomp).dev) };
    match cdclk_freq {
        337500 => {
            bclk_m = 16;
            bclk_n = 225;
        }

        540000 => {
            bclk_m = 4;
            bclk_n = 90;
        }

        675000 => {
            bclk_m = 8;
            bclk_n = 225;
        }

        450000 | _ => {
            /* default CDCLK 450MHz */
            bclk_m = 4;
            bclk_n = 75;
        }
    }

    unsafe {
        snd_hdac_chip_writew(bus, HSW_EM4, bclk_m);
        snd_hdac_chip_writew(bus, HSW_EM5, bclk_n);
    }
}
/* EXPORT_SYMBOL_GPL(snd_hdac_i915_set_bclk); */

/* returns true if the devices can be connected for audio */
unsafe fn connectivity_check(i915: *mut pci_dev, hdac: *mut pci_dev) -> bool {
    let mut bus_a: *mut pci_bus = unsafe { (*i915).bus };
    let mut bus_b: *mut pci_bus = unsafe { (*hdac).bus };

    /* directly connected on the same bus */
    if bus_a == bus_b {
        return true;
    }

    bus_a = unsafe { (*bus_a).parent };
    bus_b = unsafe { (*bus_b).parent };

    /* connected via parent bus (may be NULL!) */
    if bus_a == bus_b {
        return true;
    }

    if bus_a.is_null() || bus_b.is_null() {
        return false;
    }

    /*
     * on i915 discrete GPUs with embedded HDA audio, the two
     * devices are connected via 2nd level PCI bridge
     */
    bus_a = unsafe { (*bus_a).parent };
    bus_b = unsafe { (*bus_b).parent };
    if !bus_a.is_null() && bus_a == bus_b {
        return true;
    }

    false
}

unsafe extern "C" fn i915_component_master_match(
    dev: *mut device,
    subcomponent: c_int,
    data: *mut c_void,
) -> c_int {
    let hdac_pci: *mut pci_dev;
    let i915_pci: *mut pci_dev;
    let bus: *mut hdac_bus = data as *mut hdac_bus;

    if unsafe { !dev_is_pci(dev) } {
        return 0;
    }

    hdac_pci = unsafe { to_pci_dev((*bus).dev) };
    i915_pci = unsafe { to_pci_dev(dev) };

    if (unsafe { strcmp((*(*dev).driver).name, c"i915".as_ptr()) } == 0
        || unsafe { strcmp((*(*dev).driver).name, c"xe".as_ptr()) } == 0)
        && subcomponent == I915_COMPONENT_AUDIO
        && unsafe { connectivity_check(i915_pci, hdac_pci) }
    {
        return 1;
    }

    0
}

/* check whether Intel graphics is present and reachable */
unsafe fn i915_gfx_present(hdac_pci: *mut pci_dev) -> c_int {
    /* List of known platforms with no i915 support. */
    static DENYLIST: [pci_device_id; 16] = [
        /* CNL */
        pci_device_id { class: 0x030000, class_mask: 0xff0000, ..PCI_DEVICE(PCI_VENDOR_ID_INTEL, 0x5a40) },
        pci_device_id { class: 0x030000, class_mask: 0xff0000, ..PCI_DEVICE(PCI_VENDOR_ID_INTEL, 0x5a41) },
        pci_device_id { class: 0x030000, class_mask: 0xff0000, ..PCI_DEVICE(PCI_VENDOR_ID_INTEL, 0x5a42) },
        pci_device_id { class: 0x030000, class_mask: 0xff0000, ..PCI_DEVICE(PCI_VENDOR_ID_INTEL, 0x5a44) },
        pci_device_id { class: 0x030000, class_mask: 0xff0000, ..PCI_DEVICE(PCI_VENDOR_ID_INTEL, 0x5a49) },
        pci_device_id { class: 0x030000, class_mask: 0xff0000, ..PCI_DEVICE(PCI_VENDOR_ID_INTEL, 0x5a4a) },
        pci_device_id { class: 0x030000, class_mask: 0xff0000, ..PCI_DEVICE(PCI_VENDOR_ID_INTEL, 0x5a4c) },
        pci_device_id { class: 0x030000, class_mask: 0xff0000, ..PCI_DEVICE(PCI_VENDOR_ID_INTEL, 0x5a50) },
        pci_device_id { class: 0x030000, class_mask: 0xff0000, ..PCI_DEVICE(PCI_VENDOR_ID_INTEL, 0x5a51) },
        pci_device_id { class: 0x030000, class_mask: 0xff0000, ..PCI_DEVICE(PCI_VENDOR_ID_INTEL, 0x5a52) },
        pci_device_id { class: 0x030000, class_mask: 0xff0000, ..PCI_DEVICE(PCI_VENDOR_ID_INTEL, 0x5a54) },
        pci_device_id { class: 0x030000, class_mask: 0xff0000, ..PCI_DEVICE(PCI_VENDOR_ID_INTEL, 0x5a59) },
        pci_device_id { class: 0x030000, class_mask: 0xff0000, ..PCI_DEVICE(PCI_VENDOR_ID_INTEL, 0x5a5a) },
        pci_device_id { class: 0x030000, class_mask: 0xff0000, ..PCI_DEVICE(PCI_VENDOR_ID_INTEL, 0x5a5c) },
        /* LKF */
        pci_device_id { class: 0x030000, class_mask: 0xff0000, ..PCI_DEVICE(PCI_VENDOR_ID_INTEL, 0x9840) },
        pci_device_id {
            vendor: 0,
            device: 0,
            subvendor: 0,
            subdevice: 0,
            class: 0,
            class_mask: 0,
            driver_data: 0,
        },
    ];
    let mut display_dev: *mut pci_dev = ptr::null_mut();

    if unsafe { gpu_bind == 0 } || (unsafe { gpu_bind < 0 } && unsafe { video_firmware_drivers_only() }) {
        return false as c_int;
    }

    loop {
        display_dev = unsafe { pci_get_device(PCI_ANY_ID, PCI_ANY_ID, display_dev) };
        if display_dev.is_null() {
            break;
        }

        if unsafe { (*display_dev).vendor != PCI_VENDOR_ID_INTEL } || unsafe { !pci_is_display(display_dev) } {
            continue;
        }

        if unsafe { !pci_match_id(DENYLIST.as_ptr(), display_dev).is_null() } {
            continue;
        }

        if unsafe { connectivity_check(display_dev, hdac_pci) } {
            unsafe { pci_dev_put(display_dev) };
            return true as c_int;
        }
    }

    false as c_int
}

/**
 * snd_hdac_i915_init - Initialize i915 audio component
 * @bus: HDA core bus
 *
 * This function is supposed to be used only by a HD-audio controller
 * driver that needs the interaction with i915 graphics.
 *
 * This function initializes and sets up the audio component to communicate
 * with i915 graphics driver.
 *
 * Returns zero for success or a negative error code.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_i915_init(bus: *mut hdac_bus) -> c_int {
    let mut acomp: *mut drm_audio_component;
    let err: c_int;

    if unsafe { i915_gfx_present(to_pci_dev((*bus).dev)) } == 0 {
        return -ENODEV;
    }

    err = unsafe {
        snd_hdac_acomp_init(
            bus,
            ptr::null_mut(),
            Some(i915_component_master_match),
            size_of::<i915_audio_component>() - size_of::<drm_audio_component>(),
        )
    };
    if err < 0 {
        return err;
    }
    acomp = unsafe { (*bus).audio_component };
    if acomp.is_null() {
        return -ENODEV;
    }
    if unsafe { (*acomp).ops.is_null() } {
        unsafe { snd_hdac_acomp_exit(bus) };
        return unsafe {
            dev_err_probe(
                (*bus).dev,
                -EPROBE_DEFER,
                c"couldn't bind with audio component\n".as_ptr(),
            )
        };
    }
    0
}
/* EXPORT_SYMBOL_GPL(snd_hdac_i915_init); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
