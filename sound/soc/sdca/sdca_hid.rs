// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)

/*
 * The MIPI SDCA specification is available for public downloads at
 * https://www.mipi.org/mipi-sdca-v1-0-download
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of_val;
use core::ptr;

type SizeT = usize;
type U8 = u8;

const EINVAL: c_int = 22;
const EIO: c_int = 5;
const ENODEV: c_int = 19;

const HID_MAX_DESCRIPTOR_SIZE: c_uint = 4096;
const HID_REQ_GET_REPORT: c_int = 0x01;
const HID_REQ_SET_REPORT: c_int = 0x09;
const HID_INPUT_REPORT: c_int = 0;
const BUS_SDW: c_uint = 0x1c;

const SDCA_CTL_HIDE_HIDTX_MESSAGEOFFSET: c_uint = 0;
const SDCA_CTL_HIDE_HIDTX_MESSAGELENGTH: c_uint = 0;

#[repr(C)]
pub struct device {
    pub parent: *mut device,
}

#[repr(C)]
pub struct hid_device {
    pub ll_driver: *const hid_ll_driver,
    pub dev: device,
    pub bus: c_uint,
    pub version: c_uint,
    pub phys: [c_char; 64],
    pub name: [c_char; 128],
    pub driver_data: *mut c_void,
}

#[repr(C)]
pub struct hid_ll_driver {
    pub parse: Option<unsafe extern "C" fn(*mut hid_device) -> c_int>,
    pub start: Option<unsafe extern "C" fn(*mut hid_device) -> c_int>,
    pub stop: Option<unsafe extern "C" fn(*mut hid_device)>,
    pub open: Option<unsafe extern "C" fn(*mut hid_device) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut hid_device)>,
    pub raw_request: Option<
        unsafe extern "C" fn(
            *mut hid_device,
            u8,
            *mut U8,
            SizeT,
            u8,
            c_int,
        ) -> c_int,
    >,
}

#[repr(C)]
pub struct sdca_interrupt {
    pub dev: *mut device,
    pub function: *mut sdca_function_data,
    pub priv_: *mut c_void,
    pub function_regmap: *mut c_void,
    pub device_regmap: *mut c_void,
    pub entity: c_uint,
    pub control: c_uint,
}

#[repr(C)]
pub struct sdca_function_data {
    pub hid: sdca_hid_data,
    pub desc: *mut sdca_function_desc,
}

#[repr(C)]
pub struct sdca_function_desc {
    pub name: *const c_char,
    pub adr: c_uint,
}

#[repr(C)]
pub struct sdca_hid_data {
    pub desc: sdca_hid_descriptor,
    pub report_desc: *mut U8,
}

#[repr(C)]
pub struct sdca_hid_descriptor {
    pub bcdHID: u16,
    pub rpt_desc: sdca_hid_report_descriptor,
}

#[repr(C)]
pub struct sdca_hid_report_descriptor {
    pub wDescriptorLength: u16,
}

unsafe extern "C" {
    fn hid_parse_report(hid: *mut hid_device, start: *mut U8, size: c_uint) -> c_int;
    fn hid_allocate_device() -> *mut hid_device;
    fn hid_add_device(hid: *mut hid_device) -> c_int;
    fn hid_destroy_device(hid: *mut hid_device);
    fn hid_input_report(
        hid: *mut hid_device,
        type_: c_uint,
        data: *mut c_void,
        size: c_int,
        interrupt: bool,
    ) -> c_int;

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_name(dev: *const device) -> *const c_char;
    fn strscpy(dest: *mut c_char, src: *const c_char) -> SizeT;
    fn snprintf(str: *mut c_char, size: SizeT, fmt: *const c_char, ...) -> c_int;

    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn kfree(ptr: *mut c_void);

    fn sdca_ump_get_owner_host(
        dev: *mut device,
        regmap: *mut c_void,
        function: *mut sdca_function_data,
        entity: c_uint,
        control: c_uint,
    ) -> c_int;
    fn sdca_ump_read_message(
        dev: *mut device,
        device_regmap: *mut c_void,
        function_regmap: *mut c_void,
        function: *mut sdca_function_data,
        entity: c_uint,
        message_offset: c_uint,
        message_length: c_uint,
        val: *mut *mut c_void,
    ) -> c_int;
    fn sdca_ump_set_owner_device(
        dev: *mut device,
        regmap: *mut c_void,
        function: *mut sdca_function_data,
        entity: c_uint,
        control: c_uint,
    ) -> c_int;
}

#[inline]
fn le16_to_cpu(x: u16) -> u16 {
    u16::from_le(x)
}

struct KfreePtr(*mut c_void);

impl Drop for KfreePtr {
    fn drop(&mut self) {
        unsafe {
            if !self.0.is_null() {
                kfree(self.0);
            }
        }
    }
}

unsafe extern "C" fn sdwhid_parse(hid: *mut hid_device) -> c_int {
    let function = unsafe { (*hid).driver_data as *mut sdca_function_data };
    let rsize: c_uint;
    let ret: c_int;

    rsize = unsafe {
        le16_to_cpu((*function).hid.desc.rpt_desc.wDescriptorLength) as c_uint
    };

    if rsize == 0 || rsize > HID_MAX_DESCRIPTOR_SIZE {
        unsafe {
            dev_err(
                &mut (*hid).dev,
                c"invalid size of report descriptor (%u)\n".as_ptr(),
                rsize,
            );
        }
        return -EINVAL;
    }

    ret = unsafe { hid_parse_report(hid, (*function).hid.report_desc, rsize) };

    if ret == 0 {
        return 0;
    }

    unsafe {
        dev_err(
            &mut (*hid).dev,
            c"parsing report descriptor failed\n".as_ptr(),
        );
    }
    ret
}

unsafe extern "C" fn sdwhid_start(_hid: *mut hid_device) -> c_int {
    0
}

unsafe extern "C" fn sdwhid_stop(_hid: *mut hid_device) {}

unsafe extern "C" fn sdwhid_raw_request(
    _hid: *mut hid_device,
    _reportnum: u8,
    _buf: *mut U8,
    _len: SizeT,
    _rtype: u8,
    reqtype: c_int,
) -> c_int {
    match reqtype {
        HID_REQ_GET_REPORT => {
            /* not implemented yet */
            0
        }
        HID_REQ_SET_REPORT => {
            /* not implemented yet */
            0
        }
        _ => -EIO,
    }
}

unsafe extern "C" fn sdwhid_open(_hid: *mut hid_device) -> c_int {
    0
}

unsafe extern "C" fn sdwhid_close(_hid: *mut hid_device) {}

static SDW_HID_DRIVER: hid_ll_driver = hid_ll_driver {
    parse: Some(sdwhid_parse),
    start: Some(sdwhid_start),
    stop: Some(sdwhid_stop),
    open: Some(sdwhid_open),
    close: Some(sdwhid_close),
    raw_request: Some(sdwhid_raw_request),
};

/**
 * sdca_add_hid_device - create a new SDCA HID device
 * @interrupt: Pointer to the SDCA interrupt information structure.
 *
 * Return: Zero on success, and a negative error code on failure.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sdca_add_hid_device(interrupt: *mut sdca_interrupt) -> c_int {
    let dev = unsafe { (*interrupt).dev };
    let function = unsafe { (*interrupt).function };
    let hid: *mut hid_device;
    let ret: c_int;

    hid = unsafe { hid_allocate_device() };
    if unsafe { IS_ERR(hid as *const c_void) } {
        return unsafe { PTR_ERR(hid as *const c_void) };
    }

    unsafe {
        (*hid).ll_driver = &SDW_HID_DRIVER;

        (*hid).dev.parent = dev;
        (*hid).bus = BUS_SDW;
        (*hid).version = le16_to_cpu((*function).hid.desc.bcdHID) as c_uint;

        strscpy((*hid).phys.as_mut_ptr(), dev_name(dev));
        snprintf(
            (*hid).name.as_mut_ptr(),
            size_of_val(&(*hid).name),
            c"SDCA %s:%02x".as_ptr(),
            (*(*function).desc).name,
            (*(*function).desc).adr,
        );

        (*hid).driver_data = function as *mut c_void;
    }

    ret = unsafe { hid_add_device(hid) };
    if ret != 0 && ret != -ENODEV {
        unsafe {
            dev_err(dev, c"can't add hid device: %d\n".as_ptr(), ret);
            hid_destroy_device(hid);
        }
        return ret;
    }

    unsafe {
        (*interrupt).priv_ = hid as *mut c_void;
    }

    0
}

/* EXPORT_SYMBOL_NS(sdca_add_hid_device, "SND_SOC_SDCA"); */

/**
 * sdca_destroy_hid_device - destroy the HID device
 * @interrupt: Pointer to the SDCA interrupt information structure.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sdca_destroy_hid_device(interrupt: *mut sdca_interrupt) {
    let hid = unsafe { (*interrupt).priv_ as *mut hid_device };

    unsafe {
        hid_destroy_device(hid);
    }
}

/* EXPORT_SYMBOL_NS(sdca_destroy_hid_device, "SND_SOC_SDCA"); */

/**
 * sdca_hid_process_report - read a HID event from the device and report
 * @interrupt: Pointer to the SDCA interrupt information structure.
 *
 * Return: Zero on success, and a negative error code on failure.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sdca_hid_process_report(interrupt: *mut sdca_interrupt) -> c_int {
    let dev = unsafe { (*interrupt).dev };
    let hid = unsafe { (*interrupt).priv_ as *mut hid_device };
    let mut val = KfreePtr(ptr::null_mut());
    let len: c_int;
    let mut ret: c_int;

    ret = unsafe {
        sdca_ump_get_owner_host(
            dev,
            (*interrupt).function_regmap,
            (*interrupt).function,
            (*interrupt).entity,
            (*interrupt).control,
        )
    };
    if ret != 0 {
        return ret;
    }

    len = unsafe {
        sdca_ump_read_message(
            dev,
            (*interrupt).device_regmap,
            (*interrupt).function_regmap,
            (*interrupt).function,
            (*interrupt).entity,
            SDCA_CTL_HIDE_HIDTX_MESSAGEOFFSET,
            SDCA_CTL_HIDE_HIDTX_MESSAGELENGTH,
            &mut val.0,
        )
    };
    if len < 0 {
        return len;
    }

    ret = unsafe {
        sdca_ump_set_owner_device(
            dev,
            (*interrupt).function_regmap,
            (*interrupt).function,
            (*interrupt).entity,
            (*interrupt).control,
        )
    };
    if ret != 0 {
        return ret;
    }

    ret = unsafe { hid_input_report(hid, HID_INPUT_REPORT, val.0, len, true) };
    if ret < 0 {
        unsafe {
            dev_err(dev, c"failed to report hid event: %d\n".as_ptr(), ret);
        }
        return ret;
    }

    0
}

/* EXPORT_SYMBOL_NS(sdca_hid_process_report, "SND_SOC_SDCA"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
