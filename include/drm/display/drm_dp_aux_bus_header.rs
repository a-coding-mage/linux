/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2021 Google Inc.
 *
 * The DP AUX bus is used for devices that are connected over a DisplayPort
 * AUX bus. The devices on the far side of the bus are referred to as
 * endpoints in this code.
 */

// C header guard: _DP_AUX_BUS_H_
// Dependencies supplied by other translated headers:
// `device`, `device_driver`, `drm_dp_aux`, `module`, and `ENODEV`.

/**
 * struct dp_aux_ep_device - Main dev structure for DP AUX endpoints
 *
 * This is used to instantiate devices that are connected via a DP AUX
 * bus. Usually the device is a panel, but conceivable other devices could
 * be hooked up there.
 */
#[repr(C)]
pub struct dp_aux_ep_device {
    /** @dev: The normal dev pointer */
    pub dev: device,
    /** @aux: Pointer to the aux bus */
    pub aux: *mut drm_dp_aux,
}

#[repr(C)]
pub struct dp_aux_ep_driver {
    pub probe: Option<unsafe extern "C" fn(aux_ep: *mut dp_aux_ep_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(aux_ep: *mut dp_aux_ep_device)>,
    pub shutdown: Option<unsafe extern "C" fn(aux_ep: *mut dp_aux_ep_device)>,
    pub driver: device_driver,
}

pub unsafe fn to_dp_aux_ep_dev(dev: *mut device) -> *mut dp_aux_ep_device {
    let base = core::ptr::null_mut::<dp_aux_ep_device>();
    let member = core::ptr::addr_of!((*base).dev) as usize;
    (dev as usize).wrapping_sub(member) as *mut dp_aux_ep_device
}

pub unsafe fn to_dp_aux_ep_drv(drv: *mut device_driver) -> *mut dp_aux_ep_driver {
    let base = core::ptr::null_mut::<dp_aux_ep_driver>();
    let member = core::ptr::addr_of!((*base).driver) as usize;
    (drv as usize).wrapping_sub(member) as *mut dp_aux_ep_driver
}

pub unsafe extern "C" fn of_dp_aux_populate_bus(
    aux: *mut drm_dp_aux,
    done_probing: Option<unsafe extern "C" fn(aux: *mut drm_dp_aux) -> i32>,
) -> i32;
pub unsafe extern "C" fn of_dp_aux_depopulate_bus(aux: *mut drm_dp_aux);
pub unsafe extern "C" fn devm_of_dp_aux_populate_bus(
    aux: *mut drm_dp_aux,
    done_probing: Option<unsafe extern "C" fn(aux: *mut drm_dp_aux) -> i32>,
) -> i32;

/* Deprecated versions of the above functions. To be removed when no callers. */
pub unsafe fn of_dp_aux_populate_ep_devices(aux: *mut drm_dp_aux) -> i32 {
    let ret = of_dp_aux_populate_bus(aux, None);

    /* New API returns -ENODEV for no child case; adapt to old assumption */
    if ret != -ENODEV { ret } else { 0 }
}

pub unsafe fn devm_of_dp_aux_populate_ep_devices(aux: *mut drm_dp_aux) -> i32 {
    let ret = devm_of_dp_aux_populate_bus(aux, None);

    /* New API returns -ENODEV for no child case; adapt to old assumption */
    if ret != -ENODEV { ret } else { 0 }
}

pub unsafe fn of_dp_aux_depopulate_ep_devices(aux: *mut drm_dp_aux) {
    of_dp_aux_depopulate_bus(aux);
}

// C macro dp_aux_dp_driver_register(aux_ep_drv):
// __dp_aux_dp_driver_register(aux_ep_drv, THIS_MODULE)
pub unsafe extern "C" fn __dp_aux_dp_driver_register(
    aux_ep_drv: *mut dp_aux_ep_driver,
    owner: *mut module,
) -> i32;
pub unsafe extern "C" fn dp_aux_dp_driver_unregister(aux_ep_drv: *mut dp_aux_ep_driver);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
