/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright (C) 2016 Robert Jarzmik <robert.jarzmik@free.fr>
 */

// Translated from the AC97 controller header. Linux device/list definitions
// and configuration symbols are supplied by the surrounding kernel bindings.

pub const AC97_BUS_MAX_CODECS: usize = 4;
pub const AC97_SLOTS_AVAILABLE_ALL: u16 = 0xf;

#[repr(C)]
pub struct ac97_controller_ops {
    pub reset: Option<unsafe extern "C" fn(adrv: *mut ac97_controller)>,
    pub warm_reset: Option<unsafe extern "C" fn(adrv: *mut ac97_controller)>,
    pub write: Option<unsafe extern "C" fn(
        adrv: *mut ac97_controller,
        slot: i32,
        reg: u16,
        val: u16,
    ) -> i32>,
    pub read: Option<unsafe extern "C" fn(
        adrv: *mut ac97_controller,
        slot: i32,
        reg: u16,
    ) -> i32>,
}

#[repr(C)]
pub struct ac97_controller {
    pub ops: *const ac97_controller_ops,
    pub controllers: list_head,
    pub adap: device,
    pub nr: i32,
    pub slots_available: u16,
    pub parent: *mut device,
    pub codecs: [*mut ac97_codec_device; AC97_BUS_MAX_CODECS],
    pub codecs_pdata: [*mut core::ffi::c_void; AC97_BUS_MAX_CODECS],
}

// External types supplied by the included Linux headers.
#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ac97_codec_device {
    _private: [u8; 0],
}

// CONFIG_AC97_BUS_NEW selects the external registration implementation.
#[cfg(CONFIG_AC97_BUS_NEW)]
extern "C" {
    pub fn snd_ac97_controller_register(
        ops: *const ac97_controller_ops,
        dev: *mut device,
        slots_available: u16,
    ) -> *mut ac97_controller;
    pub fn snd_ac97_controller_unregister(ac97_ctrl: *mut ac97_controller);
}

// When CONFIG_AC97_BUS_NEW is disabled, registration returns ERR_PTR(-ENODEV)
// and unregister is a no-op. ENODEV is supplied by the surrounding bindings.
#[cfg(not(CONFIG_AC97_BUS_NEW))]
pub unsafe fn snd_ac97_controller_register(
    _ops: *const ac97_controller_ops,
    _dev: *mut device,
    _slots_available: u16,
) -> *mut ac97_controller {
    (-ENODEV as isize) as *mut ac97_controller
}

#[cfg(not(CONFIG_AC97_BUS_NEW))]
pub unsafe fn snd_ac97_controller_unregister(_ac97_ctrl: *mut ac97_controller) {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
