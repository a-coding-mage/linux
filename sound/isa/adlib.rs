// SPDX-License-Identifier: GPL-2.0-only
/*
 * AdLib FM card driver.
 */

// C dependencies: linux/kernel.h, linux/module.h, linux/isa.h,
// sound/core.h, sound/initval.h, sound/opl3.h

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

const CRD_NAME: &[u8] = b"AdLib FM\0";
const DEV_NAME: &[u8] = b"adlib\0";

// MODULE_DESCRIPTION(CRD_NAME);
// MODULE_AUTHOR("Rene Herman");
// MODULE_LICENSE("GPL");

unsafe extern "C" {
    static THIS_MODULE: *mut module;

    static SNDRV_CARDS: usize;
    static SNDRV_DEFAULT_IDX: [c_int; 0];
    static SNDRV_DEFAULT_STR: [*mut c_char; 0];
    static SNDRV_DEFAULT_ENABLE: [bool; 0];
    static SNDRV_DEFAULT_PORT: [c_long; 0];
    static SNDRV_AUTO_PORT: c_long;

    static EBUSY: c_int;
    static OPL3_HW_AUTO: c_int;

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_devm_card_new(
        dev: *mut device,
        idx: c_int,
        xid: *mut c_char,
        module: *mut module,
        extra_size: c_int,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn devm_request_region(
        dev: *mut device,
        start: c_long,
        n: c_int,
        name: *const c_char,
    ) -> *mut c_void;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn snd_opl3_create(
        card: *mut snd_card,
        l_port: c_long,
        r_port: c_long,
        hardware: c_int,
        integrated: c_int,
        ropl3: *mut *mut snd_opl3,
    ) -> c_int;
    fn snd_opl3_hwdep_new(
        opl3: *mut snd_opl3,
        device: c_int,
        seq_device: c_int,
        rhwdep: *mut *mut c_void,
    ) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_opl3 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub private_data: *mut c_void,
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub longname: [c_char; 80],
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
}

#[repr(C)]
pub struct isa_driver {
    pub match_: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    pub probe: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    pub driver: device_driver,
}

static mut index: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IDX;
static mut id: [*mut c_char; SNDRV_CARDS] = SNDRV_DEFAULT_STR;
static mut enable: [bool; SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE;
static mut port: [c_long; SNDRV_CARDS] = SNDRV_DEFAULT_PORT;

// module_param_array(index, int, NULL, 0444);
// MODULE_PARM_DESC(index, "Index value for " CRD_NAME " soundcard.");
// module_param_array(id, charp, NULL, 0444);
// MODULE_PARM_DESC(id, "ID string for " CRD_NAME " soundcard.");
// module_param_array(enable, bool, NULL, 0444);
// MODULE_PARM_DESC(enable, "Enable " CRD_NAME " soundcard.");
// module_param_hw_array(port, long, ioport, NULL, 0444);
// MODULE_PARM_DESC(port, "Port # for " CRD_NAME " driver.");

unsafe extern "C" fn snd_adlib_match(dev: *mut device, n: c_uint) -> c_int {
    if !enable[n as usize] {
        return 0;
    }

    if port[n as usize] == SNDRV_AUTO_PORT {
        dev_err(dev, c"please specify port\n".as_ptr());
        return 0;
    }
    1
}

unsafe extern "C" fn snd_adlib_probe(dev: *mut device, n: c_uint) -> c_int {
    let mut card: *mut snd_card = core::ptr::null_mut();
    let mut opl3: *mut snd_opl3 = core::ptr::null_mut();
    let mut error: c_int;

    error = snd_devm_card_new(
        dev,
        index[n as usize],
        id[n as usize],
        THIS_MODULE,
        0,
        &mut card,
    );
    if error < 0 {
        dev_err(dev, c"could not create card\n".as_ptr());
        return error;
    }

    (*card).private_data = devm_request_region(dev, port[n as usize], 4, CRD_NAME.as_ptr().cast());
    if (*card).private_data.is_null() {
        dev_err(dev, c"could not grab ports\n".as_ptr());
        return -EBUSY;
    }

    strscpy((*card).driver.as_mut_ptr(), DEV_NAME.as_ptr().cast());
    strscpy((*card).shortname.as_mut_ptr(), CRD_NAME.as_ptr().cast());
    sprintf(
        (*card).longname.as_mut_ptr(),
        c"AdLib FM at %#lx".as_ptr(),
        port[n as usize],
    );

    error = snd_opl3_create(
        card,
        port[n as usize],
        port[n as usize].wrapping_add(2),
        OPL3_HW_AUTO,
        1,
        &mut opl3,
    );
    if error < 0 {
        dev_err(dev, c"could not create OPL\n".as_ptr());
        return error;
    }

    error = snd_opl3_hwdep_new(opl3, 0, 0, core::ptr::null_mut());
    if error < 0 {
        dev_err(dev, c"could not create FM\n".as_ptr());
        return error;
    }

    error = snd_card_register(card);
    if error < 0 {
        dev_err(dev, c"could not register card\n".as_ptr());
        return error;
    }

    dev_set_drvdata(dev, card.cast());
    0
}

static mut snd_adlib_driver: isa_driver = isa_driver {
    match_: Some(snd_adlib_match),
    probe: Some(snd_adlib_probe),

    driver: device_driver {
        name: DEV_NAME.as_ptr().cast(),
    },
};

// module_isa_driver(snd_adlib_driver, SNDRV_CARDS);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
