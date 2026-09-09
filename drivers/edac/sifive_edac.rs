// SPDX-License-Identifier: GPL-2.0
/*
 * SiFive Platform EDAC Driver
 *
 * Copyright (C) 2018-2022 SiFive, Inc.
 *
 * This driver is partially based on octeon_edac-pc.c
 *
 */

// C dependencies supplied by the surrounding kernel build are intentionally
// left as external Rust declarations.

pub const DRVNAME: &[u8] = b"sifive_edac\0";

#[repr(C)]
pub struct notifier_block {
    pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, libc::c_ulong, *mut libc::c_void) -> libc::c_int>,
}

#[repr(C)]
pub struct edac_device_ctl_info {
    pub dev: *mut libc::c_void,
    pub mod_name: *const libc::c_char,
    pub ctl_name: *const libc::c_char,
    pub dev_name: *const libc::c_char,
}

#[repr(C)]
pub struct device;
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct sifive_edac_priv {
    pub notifier: notifier_block,
    pub dci: *mut edac_device_ctl_info,
}

pub const SIFIVE_CCACHE_ERR_TYPE_UE: libc::c_ulong = 0;
pub const SIFIVE_CCACHE_ERR_TYPE_CE: libc::c_ulong = 1;
pub const NOTIFY_OK: libc::c_int = 0;
pub const ENOMEM: libc::c_int = 12;
pub const ENXIO: libc::c_int = 6;

extern "C" {
    fn devm_kzalloc(dev: *mut device, size: usize, flags: libc::c_ulong) -> *mut libc::c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut libc::c_void);
    fn edac_device_alloc_ctl_info(a: libc::c_uint, b: *const libc::c_char, c: libc::c_uint,
                                  d: *const libc::c_char, e: libc::c_uint, f: libc::c_uint,
                                  g: libc::c_int) -> *mut edac_device_ctl_info;
    fn edac_device_alloc_index() -> libc::c_int;
    fn edac_device_handle_ue(dci: *mut edac_device_ctl_info, a: libc::c_uint, b: libc::c_uint, msg: *const libc::c_char);
    fn edac_device_handle_ce(dci: *mut edac_device_ctl_info, a: libc::c_uint, b: libc::c_uint, msg: *const libc::c_char);
    fn edac_device_add_device(dci: *mut edac_device_ctl_info) -> libc::c_int;
    fn edac_device_free_ctl_info(dci: *mut edac_device_ctl_info);
    fn edac_device_del_device(dev: *mut device);
    fn dev_name(dev: *mut device) -> *const libc::c_char;
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut libc::c_void;
    fn register_sifive_ccache_error_notifier(nb: *mut notifier_block);
    fn unregister_sifive_ccache_error_notifier(nb: *mut notifier_block);
    fn platform_device_register_simple(name: *const libc::c_char, id: libc::c_int, data: *mut libc::c_void, size: libc::c_uint) -> *mut platform_device;
    fn platform_device_unregister(pdev: *mut platform_device);
}

#[allow(non_upper_case_globals)]
static GFP_KERNEL: libc::c_ulong = 0;

/*
 * EDAC error callback
 *
 * @event: non-zero if unrecoverable.
 */
unsafe extern "C" fn ecc_err_event(this: *mut notifier_block, event: libc::c_ulong, ptr: *mut libc::c_void) -> libc::c_int {
    let msg = ptr as *const libc::c_char;
    // `notifier` is the first field, matching container_of(this, ...).
    let p = this as *mut sifive_edac_priv;

    if event == SIFIVE_CCACHE_ERR_TYPE_UE {
        edac_device_handle_ue((*p).dci, 0, 0, msg);
    } else if event == SIFIVE_CCACHE_ERR_TYPE_CE {
        edac_device_handle_ce((*p).dci, 0, 0, msg);
    }

    NOTIFY_OK
}

unsafe fn ecc_register(pdev: *mut platform_device) -> libc::c_int {
    let p = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<sifive_edac_priv>(), GFP_KERNEL) as *mut sifive_edac_priv;
    if p.is_null() {
        return -ENOMEM;
    }

    (*p).notifier.notifier_call = Some(ecc_err_event);
    platform_set_drvdata(pdev, p as *mut libc::c_void);

    (*p).dci = edac_device_alloc_ctl_info(0, b"sifive_ecc\0".as_ptr() as *const libc::c_char, 1,
                                          b"sifive_ecc\0".as_ptr() as *const libc::c_char, 1, 1,
                                          edac_device_alloc_index());
    if (*p).dci.is_null() {
        return -ENOMEM;
    }

    (*(*p).dci).dev = &mut (*pdev).dev as *mut device as *mut libc::c_void;
    (*(*p).dci).mod_name = b"Sifive ECC Manager\0".as_ptr() as *const libc::c_char;
    (*(*p).dci).ctl_name = dev_name(&mut (*pdev).dev);
    (*(*p).dci).dev_name = dev_name(&mut (*pdev).dev);

    if edac_device_add_device((*p).dci) != 0 {
        edac_device_free_ctl_info((*p).dci);
        return -ENXIO;
    }

    register_sifive_ccache_error_notifier(&mut (*p).notifier);
    0
}

unsafe fn ecc_unregister(pdev: *mut platform_device) -> libc::c_int {
    let p = platform_get_drvdata(pdev) as *mut sifive_edac_priv;
    unregister_sifive_ccache_error_notifier(&mut (*p).notifier);
    edac_device_del_device(&mut (*pdev).dev);
    edac_device_free_ctl_info((*p).dci);
    0
}

static mut sifive_pdev: *mut platform_device = core::ptr::null_mut();

unsafe extern "C" fn sifive_edac_init() -> libc::c_int {
    sifive_pdev = platform_device_register_simple(DRVNAME.as_ptr() as *const libc::c_char, 0, core::ptr::null_mut(), 0);
    if sifive_pdev.is_null() {
        return -1;
    }

    let ret = ecc_register(sifive_pdev);
    if ret != 0 {
        platform_device_unregister(sifive_pdev);
    }
    ret
}

unsafe extern "C" fn sifive_edac_exit() {
    ecc_unregister(sifive_pdev);
    platform_device_unregister(sifive_pdev);
}

// module_init(sifive_edac_init);
// module_exit(sifive_edac_exit);
// MODULE_AUTHOR("SiFive Inc.");
// MODULE_DESCRIPTION("SiFive platform EDAC driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
