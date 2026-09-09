// SPDX-License-Identifier: GPL-2.0
/*
 *    Filename: ks0108.c
 *     Version: 0.1.0
 * Description: ks0108 LCD Controller driver
 *     Depends: parport
 *
 *      Author: Copyright (C) Miguel Ojeda <ojeda@kernel.org>
 *        Date: 2006-10-31
 */

// pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// Kernel and parport dependencies are supplied externally.

const KS0108_NAME: &str = "ks0108";

// CONFIG_KS0108_PORT and CONFIG_KS0108_DELAY are supplied by the build.
static mut ks0108_port: u32 = CONFIG_KS0108_PORT;
static mut ks0108_delay: u32 = CONFIG_KS0108_DELAY;

#[repr(C)]
pub struct parport {
    pub base: u16,
}

#[repr(C)]
pub struct pardevice {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pardev_cb {
    pub flags: u32,
    _rest: [u8; 0],
}

#[repr(C)]
pub struct parport_driver {
    pub name: *const core::ffi::c_char,
    pub match_port: Option<unsafe extern "C" fn(*mut parport)>,
    pub detach: Option<unsafe extern "C" fn(*mut parport)>,
}

extern "C" {
    fn parport_write_data(port: *mut parport, byte: u8);
    fn parport_write_control(port: *mut parport, byte: u8);
    fn udelay(delay: u32);
    fn memset(dest: *mut core::ffi::c_void, value: i32, count: usize) -> *mut core::ffi::c_void;
    fn parport_register_dev_model(
        port: *mut parport,
        name: *const core::ffi::c_char,
        cb: *mut pardev_cb,
        index: i32,
    ) -> *mut pardevice;
    fn parport_claim(device: *mut pardevice) -> i32;
    fn parport_unregister_device(device: *mut pardevice);
    fn parport_release(device: *mut pardevice);
}

const PARPORT_DEV_EXCL: u32 = 1;

static mut ks0108_parport: *mut parport = core::ptr::null_mut();
static mut ks0108_pardevice: *mut pardevice = core::ptr::null_mut();

#[inline]
const fn bit(n: u32) -> u8 {
    1u8 << n
}

pub unsafe extern "C" fn ks0108_writedata(byte: u8) {
    parport_write_data(ks0108_parport, byte);
}

pub unsafe extern "C" fn ks0108_writecontrol(byte: u8) {
    udelay(ks0108_delay);
    parport_write_control(ks0108_parport, byte ^ (bit(0) | bit(1) | bit(3)));
}

pub unsafe extern "C" fn ks0108_displaystate(state: u8) {
    ks0108_writedata(
        (if state != 0 { bit(0) } else { 0 })
            | bit(1)
            | bit(2)
            | bit(3)
            | bit(4)
            | bit(5),
    );
}

pub unsafe extern "C" fn ks0108_startline(startline: u8) {
    ks0108_writedata(core::cmp::min(startline, 63) | bit(6) | bit(7));
}

pub unsafe extern "C" fn ks0108_address(address: u8) {
    ks0108_writedata(core::cmp::min(address, 63) | bit(6));
}

pub unsafe extern "C" fn ks0108_page(page: u8) {
    ks0108_writedata(core::cmp::min(page, 7) | bit(3) | bit(4) | bit(5) | bit(7));
}

static mut ks0108_inited: u8 = 0;

pub unsafe extern "C" fn ks0108_isinited() -> u8 {
    ks0108_inited
}

unsafe extern "C" fn ks0108_parport_attach(port: *mut parport) {
    let mut ks0108_cb: pardev_cb = core::mem::zeroed();

    if (*port).base as u32 != ks0108_port {
        return;
    }

    memset(
        &mut ks0108_cb as *mut pardev_cb as *mut core::ffi::c_void,
        0,
        core::mem::size_of::<pardev_cb>(),
    );
    ks0108_cb.flags = PARPORT_DEV_EXCL;
    ks0108_pardevice = parport_register_dev_model(
        port,
        KS0108_NAME.as_ptr() as *const core::ffi::c_char,
        &mut ks0108_cb,
        0,
    );
    if ks0108_pardevice.is_null() {
        // pr_err("ERROR: parport didn't register new device\n");
        return;
    }
    if parport_claim(ks0108_pardevice) != 0 {
        // pr_err("could not claim access to parport %i. Aborting.\n", ks0108_port);
        parport_unregister_device(ks0108_pardevice);
        ks0108_pardevice = core::ptr::null_mut();
        return;
    }

    ks0108_parport = port;
    ks0108_inited = 1;
}

unsafe extern "C" fn ks0108_parport_detach(port: *mut parport) {
    if (*port).base as u32 != ks0108_port {
        return;
    }

    if ks0108_pardevice.is_null() {
        // pr_err("%s: already unregistered.\n", KS0108_NAME);
        return;
    }

    parport_release(ks0108_pardevice);
    parport_unregister_device(ks0108_pardevice);
    ks0108_pardevice = core::ptr::null_mut();
    ks0108_parport = core::ptr::null_mut();
}

static mut ks0108_parport_driver: parport_driver = parport_driver {
    name: b"ks0108\0".as_ptr() as *const core::ffi::c_char,
    match_port: Some(ks0108_parport_attach),
    detach: Some(ks0108_parport_detach),
};

// module_parport_driver(ks0108_parport_driver);
// MODULE_LICENSE("GPL v2");
// MODULE_AUTHOR("Miguel Ojeda <ojeda@kernel.org>");
// MODULE_DESCRIPTION("ks0108 LCD Controller driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
