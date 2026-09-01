// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2025 Cirrus Logic, Inc. and
//                    Cirrus Logic International Semiconductor Ltd.

/*
 * The MIPI SDCA specification is available for public downloads at
 * https://www.mipi.org/mipi-sdca-v1-0-download
 */

// C includes translated as external dependencies:
// linux/acpi.h, linux/device.h, linux/dev_printk.h, linux/dmi.h,
// linux/firmware.h, linux/module.h, linux/pci.h, linux/pm_runtime.h,
// linux/regmap.h, linux/sprintf.h, linux/soundwire/sdw.h,
// linux/soundwire/sdw_registers.h, sound/sdca.h, sound/sdca_fdl.h,
// sound/sdca_function.h, sound/sdca_interrupts.h, sound/sdca_ump.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const GFP_KERNEL: c_uint = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENOENT: c_int = 2;
const ETIMEDOUT: c_int = 110;

const DMI_SYS_VENDOR: c_int = 0;
const DMI_BOARD_VENDOR: c_int = 1;
const DMI_CHASSIS_VENDOR: c_int = 2;
const DMI_PRODUCT_SKU: c_int = 3;
const DMI_PRODUCT_NAME: c_int = 4;

const SDCA_MAX_INTERRUPTS: usize = 32;
const SDCA_ENTITY_TYPE_ENTITY_0: c_uint = 0;
const SDCA_ENTITY_TYPE_XU: c_uint = 3;
const SDCA_CTL_ENTITY_0_FUNCTION_ACTION: c_uint = 0;
const SDCA_CTL_ENTITY_0_RESET_FUNCTION_NOW: c_uint = 1;
const SDCA_CTL_XU_FDL_CURRENTOWNER: c_uint = 0;
const SDCA_CTL_XU_FDL_MESSAGEOFFSET: c_uint = 0;
const SDCA_CTL_XU_FDL_MESSAGELENGTH: c_uint = 0;
const SDCA_CTL_XU_FDL_SET_INDEX: c_uint = 0;
const SDCA_CTL_XU_FDL_STATUS: c_uint = 0;
const SDCA_FDL_SET_INDEX_NCOLS: c_uint = 0;
const SDCA_FDL_SET_INDEX_SET_NUMBER: c_uint = 0;
const SDCA_FDL_SET_INDEX_FILE_SET_ID: c_uint = 0;
const SDCA_CTL_XU_FDLD_NEEDS_SET: c_uint = 0;
const SDCA_CTL_XU_FDLD_MORE_FILES_OK: c_uint = 1;
const SDCA_CTL_XU_FDLD_FILE_OK: c_uint = 2;
const SDCA_CTL_XU_FDLD_COMPLETE: c_uint = 3;
const SDCA_CTL_XU_FDLD_REQ_RESET: c_uint = 1 << 8;
const SDCA_CTL_XU_FDLD_REQ_ABORT: c_uint = 1 << 9;
const SDCA_CTL_XU_FDLH_REQ_ABORT: c_int = 0;
const SDCA_CTL_XU_FDLH_FILE_AVAILABLE: c_int = 1;
const SDCA_CTL_XU_FDLH_MORE_FILES: c_int = 2;
const SDCA_CTL_XU_FDLH_COMPLETE: c_int = 3;
const SDCA_CTL_XU_FDLH_RESET_ACK: c_int = 4;
const SDCA_CTL_XU_FDLH_MASK: c_uint = 0xff;
const SDCA_XU_RESET_FUNCTION: c_uint = 0;

#[repr(C)]
pub struct device {
    pub parent: *mut device,
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct completion {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct delayed_work {
    pub work: work_struct,
}

#[repr(C)]
pub struct firmware {
    pub size: usize,
    pub data: *const u8,
}

#[repr(C)]
pub struct pci_dev {
    pub dev: device,
    pub subsystem_vendor: c_uint,
    pub subsystem_device: c_uint,
}

#[repr(C)]
pub struct acpi_sw_table_header {
    pub length: c_int,
}

#[repr(C)]
pub struct acpi_sw_table {
    pub header: acpi_sw_table_header,
    pub files: *mut acpi_sw_file,
}

#[repr(C)]
pub struct acpi_sw_file {
    pub file_length: c_int,
    pub vendor_id: c_uint,
    pub file_id: c_uint,
    pub file_version: c_uint,
    pub data: [u8; 0],
}

#[repr(C)]
pub struct sdca_function_desc {
    pub adr: c_uint,
}

#[repr(C)]
pub struct sdca_fdl_file {
    pub vendor_id: c_uint,
    pub file_id: c_uint,
    pub fdl_offset: c_uint,
}

#[repr(C)]
pub struct sdca_fdl_set {
    pub id: c_uint,
    pub num_files: c_int,
    pub files: *mut sdca_fdl_file,
}

#[repr(C)]
pub struct sdca_fdl_data {
    pub swft: *mut acpi_sw_table,
    pub num_sets: c_int,
    pub sets: *mut sdca_fdl_set,
}

#[repr(C)]
pub struct sdca_function_data {
    pub desc: *mut sdca_function_desc,
    pub reset_max_delay: c_uint,
    pub fdl_data: sdca_fdl_data,
}

#[repr(C)]
pub struct sdca_entity_xu {
    pub reset_mechanism: c_uint,
    pub max_delay: c_uint,
}

#[repr(C)]
pub struct sdca_entity {
    pub id: c_uint,
    pub type_: c_uint,
    pub xu: sdca_entity_xu,
}

#[repr(C)]
pub struct sdca_control {
    pub sel: c_uint,
}

#[repr(C)]
pub struct sdca_control_range {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fdl_state {
    pub timeout: delayed_work,
    pub begin: completion,
    pub done: completion,
    pub lock: mutex,
    pub interrupt: *mut sdca_interrupt,
    pub set: *mut sdca_fdl_set,
    pub file_index: c_int,
}

#[repr(C)]
pub struct sdca_interrupt {
    pub dev: *mut device,
    pub function: *mut sdca_function_data,
    pub device_regmap: *mut regmap,
    pub function_regmap: *mut regmap,
    pub entity: *mut sdca_entity,
    pub control: *mut sdca_control,
    pub priv_: *mut c_void,
    pub irq: c_uint,
}

#[repr(C)]
pub struct sdca_interrupt_info {
    pub irqs: [sdca_interrupt; SDCA_MAX_INTERRUPTS],
}

unsafe extern "C" {
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn msecs_to_jiffies(msecs: c_uint) -> c_ulong;
    fn wait_for_completion_timeout(x: *mut completion, timeout: c_ulong) -> c_ulong;
    fn disable_irq(irq: c_uint);
    fn sdca_ump_cancel_timeout(work: *mut delayed_work);
    fn sdca_ump_schedule_timeout(work: *mut delayed_work, delay: c_uint);
    fn dev_is_pci(dev: *mut device) -> bool;
    fn to_pci_dev(dev: *mut device) -> *mut pci_dev;
    fn kasprintf(gfp: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn kfree(ptr: *mut c_void);
    fn dmi_get_system_info(field: c_int) -> *const c_char;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn firmware_request_nowarn(fw: *mut *const firmware, name: *const c_char, dev: *mut device) -> c_int;
    fn release_firmware(fw: *const firmware);
    fn sdca_ump_write_message(
        dev: *mut device,
        device_regmap: *mut regmap,
        function_regmap: *mut regmap,
        function: *mut sdca_function_data,
        entity: *mut sdca_entity,
        offset_ctl: c_uint,
        offset: c_uint,
        length_ctl: c_uint,
        data: *const u8,
        length: usize,
    ) -> c_int;
    fn sdca_selector_find_range(
        dev: *mut device,
        xu: *mut sdca_entity,
        sel: c_uint,
        ncols: c_uint,
        index: c_uint,
    ) -> *mut sdca_control_range;
    fn sdca_range_search(range: *mut sdca_control_range, col: c_uint, val: c_uint, ret_col: c_uint) -> c_uint;
    fn pm_runtime_get(dev: *mut device) -> c_int;
    fn pm_runtime_put(dev: *mut device) -> c_int;
    fn complete(x: *mut completion);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn INIT_DELAYED_WORK(work: *mut delayed_work, func: unsafe extern "C" fn(*mut work_struct));
    fn init_completion(x: *mut completion);
    fn mutex_init(lock: *mut mutex);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
}

const fn SDW_SDCA_CTL(adr: c_uint, entity: c_uint, ctl: c_uint, instance: c_uint) -> c_uint {
    (adr << 20) | (entity << 12) | (ctl << 4) | instance
}

unsafe fn regmap_read_poll_timeout(
    map: *mut regmap,
    reg: c_uint,
    val: *mut c_uint,
    poll_us: c_uint,
    timeout_us: c_uint,
) -> c_int {
    let mut elapsed: c_uint = 0;

    while elapsed <= timeout_us {
        let ret = unsafe { regmap_read(map, reg, val) };
        if ret != 0 {
            return ret;
        }
        if unsafe { *val } == 0 {
            return 0;
        }
        elapsed = elapsed.wrapping_add(poll_us);
    }

    -ETIMEDOUT
}

unsafe fn acpi_add_ptr<T>(ptr: *mut acpi_sw_file, bytes: c_int) -> *mut T {
    unsafe { (ptr as *mut u8).add(bytes as usize) as *mut T }
}

unsafe fn acpi_sw_file_data_offset() -> usize {
    let uninit = core::mem::MaybeUninit::<acpi_sw_file>::uninit();
    let base = uninit.as_ptr();
    unsafe { (&raw const (*base).data as usize) - (base as usize) }
}

/**
 * sdca_reset_function - send an SDCA function reset
 * @dev: Device pointer for error messages.
 * @function: Pointer to the SDCA Function.
 * @regmap: Pointer to the SDCA Function regmap.
 *
 * Return: Zero on success or a negative error code.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sdca_reset_function(
    dev: *mut device,
    function: *mut sdca_function_data,
    regmap: *mut regmap,
) -> c_int {
    let reg: c_uint = SDW_SDCA_CTL(
        unsafe { (*(*function).desc).adr },
        SDCA_ENTITY_TYPE_ENTITY_0,
        SDCA_CTL_ENTITY_0_FUNCTION_ACTION,
        0,
    );
    let mut val: c_uint = 0;
    let poll_us: c_uint;
    let mut ret: c_int;

    ret = unsafe { regmap_write(regmap, reg, SDCA_CTL_ENTITY_0_RESET_FUNCTION_NOW) };
    if ret != 0 {
        return 0;
    }

    /*
     * Poll up to 16 times but no more than once per ms, these are just
     * arbitrarily selected values, so may be fine tuned in future.
     */
    poll_us = core::cmp::min(unsafe { (*function).reset_max_delay >> 4 }, 1000);

    ret = unsafe {
        regmap_read_poll_timeout(
            regmap,
            reg,
            &mut val,
            poll_us,
            (*function).reset_max_delay,
        )
    };
    if ret != 0 {
        unsafe { dev_err(dev, c"Failed waiting for function reset: %d\n".as_ptr(), ret) };
        return ret;
    }

    0
}
// EXPORT_SYMBOL_NS(sdca_reset_function, "SND_SOC_SDCA");

/**
 * sdca_fdl_sync - wait for a function to finish FDL
 * @dev: Device pointer for error messages.
 * @function: Pointer to the SDCA Function.
 * @info: Pointer to the SDCA interrupt info for this device.
 *
 * Return: Zero on success or a negative error code.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sdca_fdl_sync(
    dev: *mut device,
    function: *mut sdca_function_data,
    info: *mut sdca_interrupt_info,
) -> c_int {
    static fdl_retries: c_int = 6;
    let begin_timeout: c_ulong = unsafe { msecs_to_jiffies(100) };
    let done_timeout: c_ulong = unsafe { msecs_to_jiffies(4000) };
    let mut nfdl: c_int;
    let mut i: c_int = 0;

    while i < fdl_retries {
        nfdl = 0;

        let mut j: usize = 0;
        while j < SDCA_MAX_INTERRUPTS {
            let interrupt = unsafe { &mut (*info).irqs[j] as *mut sdca_interrupt };
            let fdl_state: *mut fdl_state;
            let mut time: c_ulong;

            if unsafe { (*interrupt).function != function }
                || unsafe { (*interrupt).entity.is_null() }
                || unsafe { (*interrupt).control.is_null() }
                || unsafe { (*(*interrupt).entity).type_ != SDCA_ENTITY_TYPE_XU }
                || unsafe { (*(*interrupt).control).sel != SDCA_CTL_XU_FDL_CURRENTOWNER }
            {
                j += 1;
                continue;
            }

            fdl_state = unsafe { (*interrupt).priv_ as *mut fdl_state };
            nfdl += 1;

            /*
             * Looking for timeout without any new FDL requests
             * to imply the device has completed initial
             * firmware setup. Alas the specification doesn't
             * have any mechanism to detect this.
             */
            time = unsafe { wait_for_completion_timeout(&mut (*fdl_state).begin, begin_timeout) };
            if time == 0 {
                unsafe { dev_dbg(dev, c"no new FDL starts\n".as_ptr()) };
                nfdl -= 1;
                j += 1;
                continue;
            }

            time = unsafe { wait_for_completion_timeout(&mut (*fdl_state).done, done_timeout) };
            if time == 0 {
                unsafe { dev_err(dev, c"timed out waiting for FDL to complete\n".as_ptr()) };
                break;
            }

            j += 1;
        }

        if j < SDCA_MAX_INTERRUPTS {
            break;
        }

        if nfdl == 0 {
            return 0;
        }

        i += 1;
    }

    if i >= fdl_retries {
        unsafe { dev_err(dev, c"too many FDL requests\n".as_ptr()) };
    }

    let mut j: usize = 0;
    while j < SDCA_MAX_INTERRUPTS {
        let interrupt = unsafe { &mut (*info).irqs[j] as *mut sdca_interrupt };
        let fdl_state: *mut fdl_state;

        if unsafe { (*interrupt).function != function }
            || unsafe { (*interrupt).entity.is_null() }
            || unsafe { (*interrupt).control.is_null() }
            || unsafe { (*(*interrupt).entity).type_ != SDCA_ENTITY_TYPE_XU }
            || unsafe { (*(*interrupt).control).sel != SDCA_CTL_XU_FDL_CURRENTOWNER }
        {
            j += 1;
            continue;
        }

        unsafe { disable_irq((*interrupt).irq) };

        fdl_state = unsafe { (*interrupt).priv_ as *mut fdl_state };

        unsafe { sdca_ump_cancel_timeout(&mut (*fdl_state).timeout) };
        j += 1;
    }

    -ETIMEDOUT
}
// EXPORT_SYMBOL_NS_GPL(sdca_fdl_sync, "SND_SOC_SDCA");

unsafe fn fdl_get_sku_filename(dev: *mut device, fdl_file: *mut sdca_fdl_file) -> *mut c_char {
    let mut parent = dev;
    let mut product_vendor: *const c_char;
    let mut product_sku: *const c_char;

    /*
     * Try to find pci_dev manually because the card may not be ready to be
     * used for snd_soc_card_get_pci_ssid yet
     */
    while !parent.is_null() {
        if unsafe { dev_is_pci(parent) } {
            let pci_dev = unsafe { to_pci_dev(parent) };

            return unsafe {
                kasprintf(
                    GFP_KERNEL,
                    c"sdca/%x/%x/%x/%x.bin".as_ptr(),
                    (*fdl_file).vendor_id,
                    (*pci_dev).subsystem_vendor,
                    (*pci_dev).subsystem_device,
                    (*fdl_file).file_id,
                )
            };
        } else {
            parent = unsafe { (*parent).parent };
        }
    }

    product_vendor = unsafe { dmi_get_system_info(DMI_SYS_VENDOR) };
    if product_vendor.is_null()
        || unsafe { strcmp(product_vendor, c"Default string".as_ptr()) } == 0
    {
        product_vendor = unsafe { dmi_get_system_info(DMI_BOARD_VENDOR) };
    }
    if product_vendor.is_null()
        || unsafe { strcmp(product_vendor, c"Default string".as_ptr()) } == 0
    {
        product_vendor = unsafe { dmi_get_system_info(DMI_CHASSIS_VENDOR) };
    }
    if product_vendor.is_null() {
        product_vendor = c"unknown".as_ptr();
    }

    product_sku = unsafe { dmi_get_system_info(DMI_PRODUCT_SKU) };
    if product_sku.is_null()
        || unsafe { strcmp(product_sku, c"Default string".as_ptr()) } == 0
    {
        product_sku = unsafe { dmi_get_system_info(DMI_PRODUCT_NAME) };
    }
    if product_sku.is_null() {
        product_sku = c"unknown".as_ptr();
    }

    unsafe {
        kasprintf(
            GFP_KERNEL,
            c"sdca/%x/%s/%s/%x.bin".as_ptr(),
            (*fdl_file).vendor_id,
            product_vendor,
            product_sku,
            (*fdl_file).file_id,
        )
    }
}

unsafe fn fdl_load_file(
    interrupt: *mut sdca_interrupt,
    set: *mut sdca_fdl_set,
    file_index: c_int,
) -> c_int {
    let dev = unsafe { (*interrupt).dev };
    let fdl_data = unsafe { &mut (*(*interrupt).function).fdl_data as *mut sdca_fdl_data };
    let mut swf: *mut acpi_sw_file = ptr::null_mut();
    let mut tmp: *mut acpi_sw_file;
    let fdl_file: *mut sdca_fdl_file;
    let mut disk_filename: *mut c_char;
    let mut ret: c_int;
    let mut i: c_int;

    if set.is_null() {
        unsafe { dev_err(dev, c"request to load SWF with no set\n".as_ptr()) };
        return -EINVAL;
    }

    fdl_file = unsafe { (*set).files.add(file_index as usize) };

    if unsafe { !(*fdl_data).swft.is_null() } {
        tmp = unsafe { (*(*fdl_data).swft).files };
        i = 0;
        while i < unsafe { (*(*fdl_data).swft).header.length } {
            if unsafe { (*tmp).vendor_id == (*fdl_file).vendor_id && (*tmp).file_id == (*fdl_file).file_id } {
                unsafe {
                    dev_dbg(
                        dev,
                        c"located SWF in ACPI: %x-%x-%x\n".as_ptr(),
                        (*tmp).vendor_id,
                        (*tmp).file_id,
                        (*tmp).file_version,
                    )
                };
                swf = tmp;
                break;
            }

            i += unsafe { (*tmp).file_length };
            tmp = unsafe { acpi_add_ptr::<acpi_sw_file>(tmp, (*tmp).file_length) };
        }
    }

    disk_filename = unsafe { fdl_get_sku_filename(dev, fdl_file) };
    if disk_filename.is_null() {
        return -ENOMEM;
    }

    unsafe { dev_dbg(dev, c"FDL disk filename: %s\n".as_ptr(), disk_filename) };

    let mut firmware_ptr: *const firmware = ptr::null();
    ret = unsafe { firmware_request_nowarn(&mut firmware_ptr, disk_filename, dev) };
    unsafe { kfree(disk_filename as *mut c_void) };
    if ret != 0 {
        disk_filename = unsafe {
            kasprintf(
                GFP_KERNEL,
                c"sdca/%x/%x.bin".as_ptr(),
                (*fdl_file).vendor_id,
                (*fdl_file).file_id,
            )
        };
        if disk_filename.is_null() {
            return -ENOMEM;
        }

        unsafe { dev_dbg(dev, c"FDL disk filename: %s\n".as_ptr(), disk_filename) };

        ret = unsafe { firmware_request_nowarn(&mut firmware_ptr, disk_filename, dev) };
        unsafe { kfree(disk_filename as *mut c_void) };
    }

    if ret == 0 {
        tmp = unsafe { (*firmware_ptr).data as *mut acpi_sw_file };

        if unsafe { (*firmware_ptr).size < size_of::<acpi_sw_file>() || (*tmp).file_length as usize != (*firmware_ptr).size } {
            unsafe { dev_err(dev, c"bad disk SWF size\n".as_ptr()) };
        } else if swf.is_null() || unsafe { (*swf).file_version <= (*tmp).file_version } {
            unsafe { dev_dbg(dev, c"using SWF from disk\n".as_ptr()) };
            swf = tmp;
        }
    }

    if swf.is_null() {
        unsafe { dev_err(dev, c"failed to locate SWF\n".as_ptr()) };
        if !firmware_ptr.is_null() {
            unsafe { release_firmware(firmware_ptr) };
        }
        return -ENOENT;
    }

    unsafe {
        dev_info(
            dev,
            c"loading SWF: %x-%x-%x\n".as_ptr(),
            (*swf).vendor_id,
            (*swf).file_id,
            (*swf).file_version,
        )
    };

    ret = unsafe {
        sdca_ump_write_message(
            dev,
            (*interrupt).device_regmap,
            (*interrupt).function_regmap,
            (*interrupt).function,
            (*interrupt).entity,
            SDCA_CTL_XU_FDL_MESSAGEOFFSET,
            (*fdl_file).fdl_offset,
            SDCA_CTL_XU_FDL_MESSAGELENGTH,
            (&raw const (*swf).data) as *const u8,
            ((*swf).file_length as usize).wrapping_sub(acpi_sw_file_data_offset()),
        )
    };

    if !firmware_ptr.is_null() {
        unsafe { release_firmware(firmware_ptr) };
    }

    ret
}

unsafe fn fdl_get_set(interrupt: *mut sdca_interrupt) -> *mut sdca_fdl_set {
    let dev = unsafe { (*interrupt).dev };
    let fdl_data = unsafe { &mut (*(*interrupt).function).fdl_data as *mut sdca_fdl_data };
    let xu = unsafe { (*interrupt).entity };
    let range: *mut sdca_control_range;
    let mut val: c_uint = 0;
    let mut i: c_int;
    let ret: c_int;

    ret = unsafe {
        regmap_read(
            (*interrupt).function_regmap,
            SDW_SDCA_CTL(
                (*(*(*interrupt).function).desc).adr,
                (*xu).id,
                SDCA_CTL_XU_FDL_SET_INDEX,
                0,
            ),
            &mut val,
        )
    };
    if ret < 0 {
        unsafe { dev_err(dev, c"failed to read FDL set index: %d\n".as_ptr(), ret) };
        return ptr::null_mut();
    }

    range = unsafe {
        sdca_selector_find_range(
            dev,
            xu,
            SDCA_CTL_XU_FDL_SET_INDEX,
            SDCA_FDL_SET_INDEX_NCOLS,
            0,
        )
    };

    val = unsafe {
        sdca_range_search(
            range,
            SDCA_FDL_SET_INDEX_SET_NUMBER,
            val,
            SDCA_FDL_SET_INDEX_FILE_SET_ID,
        )
    };

    i = 0;
    while i < unsafe { (*fdl_data).num_sets } {
        if unsafe { (*(*fdl_data).sets.add(i as usize)).id == val } {
            return unsafe { (*fdl_data).sets.add(i as usize) };
        }
        i += 1;
    }

    unsafe { dev_err(dev, c"invalid fileset id: %d\n".as_ptr(), val) };
    ptr::null_mut()
}

unsafe fn fdl_end(interrupt: *mut sdca_interrupt) {
    let fdl_state = unsafe { (*interrupt).priv_ as *mut fdl_state };

    if unsafe { (*fdl_state).set.is_null() } {
        return;
    }

    unsafe { (*fdl_state).set = ptr::null_mut() };

    unsafe { pm_runtime_put((*interrupt).dev) };
    unsafe { complete(&mut (*fdl_state).done) };

    unsafe { dev_dbg((*interrupt).dev, c"completed FDL process\n".as_ptr()) };
}

unsafe extern "C" fn sdca_fdl_timeout_work(work: *mut work_struct) {
    let fdl_state = work as *mut fdl_state;
    let interrupt = unsafe { (*fdl_state).interrupt };
    let dev = unsafe { (*interrupt).dev };

    unsafe { dev_err(dev, c"FDL transaction timed out\n".as_ptr()) };

    unsafe { mutex_lock(&mut (*fdl_state).lock) };
    unsafe {
        fdl_end(interrupt);
        sdca_reset_function(dev, (*interrupt).function, (*interrupt).function_regmap);
        mutex_unlock(&mut (*fdl_state).lock);
    }
}

unsafe fn fdl_status_process(interrupt: *mut sdca_interrupt, status: c_uint) -> c_int {
    let fdl_state = unsafe { (*interrupt).priv_ as *mut fdl_state };
    let ret: c_int;

    match status {
        SDCA_CTL_XU_FDLD_NEEDS_SET => {
            unsafe { dev_dbg((*interrupt).dev, c"starting FDL process...\n".as_ptr()) };

            unsafe { pm_runtime_get((*interrupt).dev) };
            unsafe { complete(&mut (*fdl_state).begin) };

            unsafe { (*fdl_state).file_index = 0 };
            unsafe { (*fdl_state).set = fdl_get_set(interrupt) };

            ret = unsafe { fdl_load_file(interrupt, (*fdl_state).set, (*fdl_state).file_index) };
            if ret != 0 {
                unsafe { fdl_end(interrupt) };
                return SDCA_CTL_XU_FDLH_REQ_ABORT;
            }

            SDCA_CTL_XU_FDLH_FILE_AVAILABLE
        }
        SDCA_CTL_XU_FDLD_MORE_FILES_OK => {
            ret = unsafe { fdl_load_file(interrupt, (*fdl_state).set, (*fdl_state).file_index) };
            if ret != 0 {
                unsafe { fdl_end(interrupt) };
                return SDCA_CTL_XU_FDLH_REQ_ABORT;
            }

            SDCA_CTL_XU_FDLH_FILE_AVAILABLE
        }
        SDCA_CTL_XU_FDLD_FILE_OK => {
            if unsafe { (*fdl_state).set.is_null() } {
                unsafe { fdl_end(interrupt) };
                return SDCA_CTL_XU_FDLH_REQ_ABORT;
            }

            unsafe { (*fdl_state).file_index += 1 };

            if unsafe { (*fdl_state).file_index < (*(*fdl_state).set).num_files } {
                return SDCA_CTL_XU_FDLH_MORE_FILES;
            }

            unsafe { fdl_end(interrupt) };
            SDCA_CTL_XU_FDLH_COMPLETE
        }
        SDCA_CTL_XU_FDLD_COMPLETE => {
            unsafe { fdl_end(interrupt) };
            SDCA_CTL_XU_FDLH_COMPLETE
        }
        _ => {
            unsafe { fdl_end(interrupt) };

            if (status & SDCA_CTL_XU_FDLD_REQ_RESET) != 0 {
                SDCA_CTL_XU_FDLH_RESET_ACK
            } else if (status & SDCA_CTL_XU_FDLD_REQ_ABORT) != 0 {
                SDCA_CTL_XU_FDLH_COMPLETE
            } else {
                unsafe { dev_err((*interrupt).dev, c"invalid FDL status: %x\n".as_ptr(), status) };
                -EINVAL
            }
        }
    }
}

/**
 * sdca_fdl_process - Process the FDL state machine
 * @interrupt: SDCA interrupt structure
 *
 * Based on section 13.2.5 Flow Diagram for File Download, Host side.
 *
 * Return: Zero on success or a negative error code.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sdca_fdl_process(interrupt: *mut sdca_interrupt) -> c_int {
    let dev = unsafe { (*interrupt).dev };
    let xu = unsafe { &mut (*(*interrupt).entity).xu as *mut sdca_entity_xu };
    let fdl_state = unsafe { (*interrupt).priv_ as *mut fdl_state };
    let mut reg: c_uint;
    let mut status: c_uint = 0;
    let response: c_int;
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
        unsafe { sdca_reset_function(dev, (*interrupt).function, (*interrupt).function_regmap) };
        return ret;
    }

    unsafe { sdca_ump_cancel_timeout(&mut (*fdl_state).timeout) };

    unsafe { mutex_lock(&mut (*fdl_state).lock) };

    reg = unsafe {
        SDW_SDCA_CTL(
            (*(*(*interrupt).function).desc).adr,
            (*(*interrupt).entity).id,
            SDCA_CTL_XU_FDL_STATUS,
            0,
        )
    };
    ret = unsafe { regmap_read((*interrupt).function_regmap, reg, &mut status) };
    if ret < 0 {
        unsafe {
            dev_err(dev, c"failed to read FDL status: %d\n".as_ptr(), ret);
            mutex_unlock(&mut (*fdl_state).lock);
        }
        return ret;
    }

    unsafe { dev_dbg(dev, c"FDL status: %#x\n".as_ptr(), status) };

    ret = unsafe { fdl_status_process(interrupt, status) };
    if ret < 0 {
        unsafe {
            mutex_unlock(&mut (*fdl_state).lock);
            sdca_reset_function(dev, (*interrupt).function, (*interrupt).function_regmap);
        }
        return ret;
    }

    response = ret;

    unsafe { dev_dbg(dev, c"FDL response: %#x\n".as_ptr(), response) };

    ret = unsafe {
        regmap_write(
            (*interrupt).function_regmap,
            reg,
            (response as c_uint) | (status & !SDCA_CTL_XU_FDLH_MASK),
        )
    };
    if ret < 0 {
        unsafe {
            dev_err(dev, c"failed to set FDL status signal: %d\n".as_ptr(), ret);
            mutex_unlock(&mut (*fdl_state).lock);
        }
        return ret;
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
        unsafe { mutex_unlock(&mut (*fdl_state).lock) };
        return ret;
    }

    match response {
        SDCA_CTL_XU_FDLH_RESET_ACK => {
            unsafe { dev_dbg(dev, c"FDL request reset\n".as_ptr()) };

            match unsafe { (*xu).reset_mechanism } {
                SDCA_XU_RESET_FUNCTION => {}
                _ => unsafe { dev_warn(dev, c"Requested reset mechanism not implemented\n".as_ptr()) },
            }
            unsafe {
                mutex_unlock(&mut (*fdl_state).lock);
                sdca_reset_function(dev, (*interrupt).function, (*interrupt).function_regmap);
            }
            ret
        }
        SDCA_CTL_XU_FDLH_COMPLETE => {
            if (status & SDCA_CTL_XU_FDLD_REQ_ABORT) != 0 || status == SDCA_CTL_XU_FDLD_COMPLETE {
                unsafe { mutex_unlock(&mut (*fdl_state).lock) };
                0
            } else {
                unsafe {
                    sdca_ump_schedule_timeout(&mut (*fdl_state).timeout, (*xu).max_delay);
                    mutex_unlock(&mut (*fdl_state).lock);
                }
                0
            }
        }
        _ => {
            unsafe {
                sdca_ump_schedule_timeout(&mut (*fdl_state).timeout, (*xu).max_delay);
                mutex_unlock(&mut (*fdl_state).lock);
            }
            0
        }
    }
}
// EXPORT_SYMBOL_NS_GPL(sdca_fdl_process, "SND_SOC_SDCA");

unsafe extern "C" {
    fn sdca_ump_get_owner_host(
        dev: *mut device,
        function_regmap: *mut regmap,
        function: *mut sdca_function_data,
        entity: *mut sdca_entity,
        control: *mut sdca_control,
    ) -> c_int;
    fn sdca_ump_set_owner_device(
        dev: *mut device,
        function_regmap: *mut regmap,
        function: *mut sdca_function_data,
        entity: *mut sdca_entity,
        control: *mut sdca_control,
    ) -> c_int;
}

/**
 * sdca_fdl_alloc_state - allocate state for an FDL interrupt
 * @interrupt: SDCA interrupt structure.
 *
 * Return: Zero on success or a negative error code.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sdca_fdl_alloc_state(interrupt: *mut sdca_interrupt) -> c_int {
    let fdl_state: *mut fdl_state;

    fdl_state = unsafe { kzalloc(size_of::<fdl_state>(), GFP_KERNEL) as *mut fdl_state };
    if fdl_state.is_null() {
        return -ENOMEM;
    }

    unsafe { INIT_DELAYED_WORK(&mut (*fdl_state).timeout, sdca_fdl_timeout_work) };
    unsafe { init_completion(&mut (*fdl_state).begin) };
    unsafe { init_completion(&mut (*fdl_state).done) };
    unsafe { mutex_init(&mut (*fdl_state).lock) };
    unsafe { (*fdl_state).interrupt = interrupt };

    unsafe { (*interrupt).priv_ = fdl_state as *mut c_void };

    0
}
// EXPORT_SYMBOL_NS_GPL(sdca_fdl_alloc_state, "SND_SOC_SDCA");

/**
 * sdca_fdl_free_state - free state for an FDL interrupt
 * @interrupt: SDCA interrupt structure.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sdca_fdl_free_state(interrupt: *mut sdca_interrupt) {
    unsafe { kfree((*interrupt).priv_) };
}
// EXPORT_SYMBOL_NS_GPL(sdca_fdl_free_state, "SND_SOC_SDCA");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
