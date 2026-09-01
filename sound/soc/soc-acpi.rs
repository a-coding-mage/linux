// SPDX-License-Identifier: GPL-2.0
//
// soc-apci.c - support for ACPI enumeration.
//
// Copyright (c) 2013-15, Intel Corporation.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

pub type bool_ = bool;
pub type u8_ = u8;
pub type u32_ = u32;
pub type u64_ = u64;
pub type acpi_handle = *mut c_void;
pub type acpi_status = c_uint;

pub const ACPI_ID_LEN: usize = 16;
pub const ACPI_ALLOCATE_BUFFER: usize = !0usize;
pub const ACPI_TYPE_PACKAGE: c_uint = 4;
pub const AE_OK: acpi_status = 0;
pub const AE_CTRL_TERMINATE: acpi_status = 0x0085;

pub const SDW_DISCO_LINK_ID_MASK: u64 = 0xF000000000000000;
pub const SDW_VERSION_MASK: u64 = 0x0F00000000000000;
pub const SDW_MFG_ID_MASK: u64 = 0x00FFFF0000000000;
pub const SDW_PART_ID_MASK: u64 = 0x000000FFFF000000;

#[repr(C)]
pub struct acpi_device_status {
    pub present: c_uint,
    pub enabled: c_uint,
    pub show_in_ui: c_uint,
    pub functional: c_uint,
    pub battery_present: c_uint,
}

#[repr(C)]
pub struct acpi_device {
    pub status: acpi_device_status,
}

#[repr(C)]
pub struct acpi_buffer {
    pub length: usize,
    pub pointer: *mut c_void,
}

#[repr(C)]
pub struct acpi_object_package {
    pub count: u32_,
    pub elements: *mut acpi_object,
}

#[repr(C)]
pub union acpi_object_data {
    pub package: core::mem::ManuallyDrop<acpi_object_package>,
}

#[repr(C)]
pub struct acpi_object {
    pub type_: c_uint,
    pub data: acpi_object_data,
}

impl acpi_object {
    unsafe fn package_count(&self) -> u32_ {
        unsafe { self.data.package.count }
    }
}

#[repr(C)]
pub struct snd_soc_acpi_codecs {
    pub num_codecs: c_int,
    pub codecs: *const *const c_char,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub id: [c_char; ACPI_ID_LEN],
    pub comp_ids: *const snd_soc_acpi_codecs,
    pub machine_quirk:
        Option<unsafe extern "C" fn(*mut snd_soc_acpi_mach) -> *mut snd_soc_acpi_mach>,
    pub quirk_data: *mut c_void,
}

#[repr(C)]
pub struct snd_soc_acpi_package_context {
    pub name: *const c_char,
    pub length: u32_,
    pub format: *const c_char,
    pub state: *mut c_void,
    pub data_valid: bool,
}

#[repr(C)]
pub struct snd_soc_acpi_endpoint_adr {
    pub adr: u64_,
}

#[repr(C)]
pub struct snd_soc_acpi_link_adr {
    pub adr_d: *const snd_soc_acpi_endpoint_adr,
    pub num_adr: c_int,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_bus {
    pub link_id: c_uint,
}

#[repr(C)]
pub struct sdw_slave_id {
    pub mfg_id: c_uint,
    pub part_id: c_uint,
    pub sdw_version: c_uint,
    pub unique_id: c_uint,
}

#[repr(C)]
pub struct sdw_slave {
    pub bus: *mut sdw_bus,
    pub id: sdw_slave_id,
}

#[repr(C)]
pub struct sdw_peripherals {
    pub num_peripherals: c_int,
    pub array: *mut *mut sdw_slave,
}

unsafe extern "C" {
    fn acpi_dev_present(hid: *const c_char, uid: *const c_char, hrv: c_int) -> bool;
    fn acpi_fetch_acpi_dev(handle: acpi_handle) -> *mut acpi_device;
    fn acpi_evaluate_object_typed(
        handle: acpi_handle,
        pathname: *const c_char,
        external_params: *mut c_void,
        return_buffer: *mut acpi_buffer,
        return_type: c_uint,
    ) -> acpi_status;
    fn acpi_extract_package(
        package: *mut acpi_object,
        format: *const c_char,
        buffer: *mut c_void,
    ) -> acpi_status;
    fn acpi_get_devices(
        hid: *const u8_,
        user_function: Option<
            unsafe extern "C" fn(acpi_handle, u32_, *mut c_void, *mut *mut c_void) -> acpi_status,
        >,
        context: *mut c_void,
        ret: *mut *mut c_void,
    ) -> acpi_status;
    fn kfree(ptr: *mut c_void);
    fn strscpy(dest: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

#[inline]
fn ACPI_FAILURE(status: acpi_status) -> bool {
    (status & 0x8000_0000) != 0
}

#[inline]
fn SDW_CODEC_ADR_MASK(adr: u64_) -> u64_ {
    adr & (SDW_DISCO_LINK_ID_MASK | SDW_VERSION_MASK | SDW_MFG_ID_MASK | SDW_PART_ID_MASK)
}

#[inline]
fn SDW_DISCO_LINK_ID(adr: u64_) -> c_uint {
    ((adr & SDW_DISCO_LINK_ID_MASK) >> 60) as c_uint
}

#[inline]
fn SDW_VERSION(adr: u64_) -> c_uint {
    ((adr & SDW_VERSION_MASK) >> 56) as c_uint
}

#[inline]
fn SDW_MFG_ID(adr: u64_) -> c_uint {
    ((adr & SDW_MFG_ID_MASK) >> 40) as c_uint
}

#[inline]
fn SDW_PART_ID(adr: u64_) -> c_uint {
    ((adr & SDW_PART_ID_MASK) >> 24) as c_uint
}

#[inline]
fn SDW_UNIQUE_ID(adr: u64_) -> c_uint {
    (adr & 0xF) as c_uint
}

unsafe fn snd_soc_acpi_id_present(machine: *mut snd_soc_acpi_mach) -> bool {
    let comp_ids = unsafe { (*machine).comp_ids };
    let mut i: c_int;

    if unsafe { (*machine).id[0] } != 0 {
        if unsafe { acpi_dev_present((*machine).id.as_ptr(), ptr::null(), -1) } {
            return true;
        }
    }

    if !comp_ids.is_null() {
        i = 0;
        while i < unsafe { (*comp_ids).num_codecs } {
            let codec = unsafe { *(*comp_ids).codecs.add(i as usize) };

            if unsafe { acpi_dev_present(codec, ptr::null(), -1) } {
                unsafe {
                    strscpy((*machine).id.as_mut_ptr(), codec, ACPI_ID_LEN);
                }
                return true;
            }
            i += 1;
        }
    }

    false
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_acpi_find_machine(
    machines: *mut snd_soc_acpi_mach,
) -> *mut snd_soc_acpi_mach {
    let mut mach: *mut snd_soc_acpi_mach;
    let mut mach_alt: *mut snd_soc_acpi_mach;

    mach = machines;
    while unsafe { (*mach).id[0] } != 0 || !unsafe { (*mach).comp_ids }.is_null() {
        if unsafe { snd_soc_acpi_id_present(mach) } {
            if unsafe { (*mach).machine_quirk.is_some() } {
                mach_alt = unsafe { ((*mach).machine_quirk.unwrap())(mach) };
                if mach_alt.is_null() {
                    mach = unsafe { mach.add(1) };
                    continue; /* not full match, ignore */
                }
                mach = mach_alt;
            }

            return mach;
        }
        mach = unsafe { mach.add(1) };
    }
    ptr::null_mut()
}

/* EXPORT_SYMBOL_GPL(snd_soc_acpi_find_machine); */

unsafe extern "C" fn snd_soc_acpi_find_package(
    handle: acpi_handle,
    _level: u32_,
    context: *mut c_void,
    _ret: *mut *mut c_void,
) -> acpi_status {
    let adev = unsafe { acpi_fetch_acpi_dev(handle) };
    let mut status: acpi_status;
    let pkg_ctx = context as *mut snd_soc_acpi_package_context;

    unsafe {
        (*pkg_ctx).data_valid = false;
    }

    if !adev.is_null()
        && unsafe { (*adev).status.present != 0 }
        && unsafe { (*adev).status.functional != 0 }
    {
        let mut buffer = acpi_buffer {
            length: ACPI_ALLOCATE_BUFFER,
            pointer: ptr::null_mut(),
        };
        let mut myobj: *mut acpi_object = ptr::null_mut();

        status = unsafe {
            acpi_evaluate_object_typed(
                handle,
                (*pkg_ctx).name,
                ptr::null_mut(),
                &mut buffer,
                ACPI_TYPE_PACKAGE,
            )
        };
        if ACPI_FAILURE(status) {
            return AE_OK;
        }

        myobj = buffer.pointer as *mut acpi_object;
        if myobj.is_null() || unsafe { (*myobj).package_count() != (*pkg_ctx).length } {
            unsafe {
                kfree(buffer.pointer);
            }
            return AE_OK;
        }

        status = unsafe { acpi_extract_package(myobj, (*pkg_ctx).format, (*pkg_ctx).state) };
        if ACPI_FAILURE(status) {
            unsafe {
                kfree(buffer.pointer);
            }
            return AE_OK;
        }

        unsafe {
            kfree(buffer.pointer);
            (*pkg_ctx).data_valid = true;
        }
        return AE_CTRL_TERMINATE;
    }

    AE_OK
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_acpi_find_package_from_hid(
    hid: *const u8_,
    ctx: *mut snd_soc_acpi_package_context,
) -> bool {
    let status: acpi_status;

    status = unsafe {
        acpi_get_devices(
            hid,
            Some(snd_soc_acpi_find_package),
            ctx as *mut c_void,
            ptr::null_mut(),
        )
    };

    if ACPI_FAILURE(status) || unsafe { !(*ctx).data_valid } {
        return false;
    }

    true
}

/* EXPORT_SYMBOL_GPL(snd_soc_acpi_find_package_from_hid); */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_acpi_codec_list(arg: *mut c_void) -> *mut snd_soc_acpi_mach {
    let mach = arg as *mut snd_soc_acpi_mach;
    let codec_list = unsafe { (*mach).quirk_data } as *mut snd_soc_acpi_codecs;
    let mut i: c_int;

    if unsafe { (*mach).quirk_data.is_null() } {
        return mach;
    }

    i = 0;
    while i < unsafe { (*codec_list).num_codecs } {
        let codec = unsafe { *(*codec_list).codecs.add(i as usize) };

        if unsafe { !acpi_dev_present(codec, ptr::null(), -1) } {
            return ptr::null_mut();
        }
        i += 1;
    }

    mach
}

/* EXPORT_SYMBOL_GPL(snd_soc_acpi_codec_list); */

/* Check if all Slaves defined on the link can be found */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_acpi_sdw_link_slaves_found(
    dev: *mut device,
    link: *const snd_soc_acpi_link_adr,
    peripherals: *mut sdw_peripherals,
) -> bool {
    let mut part_id: c_uint;
    let mut link_id: c_uint;
    let mut unique_id: c_uint;
    let mut mfg_id: c_uint;
    let mut version: c_uint;
    let mut i: c_int;
    let mut j: c_int;
    let mut k: c_int;

    i = 0;
    while i < unsafe { (*link).num_adr } {
        let adr = unsafe { (*(*link).adr_d.add(i as usize)).adr };
        let mut reported_part_count: c_int = 0;

        mfg_id = SDW_MFG_ID(adr);
        part_id = SDW_PART_ID(adr);
        link_id = SDW_DISCO_LINK_ID(adr);
        version = SDW_VERSION(adr);

        j = 0;
        while j < unsafe { (*peripherals).num_peripherals } {
            let peripheral = unsafe { *(*peripherals).array.add(j as usize) };

            /* find out how many identical parts were reported on that link */
            if unsafe { (*(*peripheral).bus).link_id == link_id }
                && unsafe { (*peripheral).id.part_id == part_id }
                && unsafe { (*peripheral).id.mfg_id == mfg_id }
                && unsafe { (*peripheral).id.sdw_version == version }
            {
                reported_part_count += 1;
            }
            j += 1;
        }

        j = 0;
        while j < unsafe { (*peripherals).num_peripherals } {
            let peripheral = unsafe { *(*peripherals).array.add(j as usize) };
            let mut expected_part_count: c_int = 0;

            if unsafe { (*(*peripheral).bus).link_id != link_id }
                || unsafe { (*peripheral).id.part_id != part_id }
                || unsafe { (*peripheral).id.mfg_id != mfg_id }
                || unsafe { (*peripheral).id.sdw_version != version }
            {
                j += 1;
                continue;
            }

            /* find out how many identical parts are expected */
            k = 0;
            while k < unsafe { (*link).num_adr } {
                let adr2 = unsafe { (*(*link).adr_d.add(k as usize)).adr };

                if SDW_CODEC_ADR_MASK(adr2) == SDW_CODEC_ADR_MASK(adr) {
                    expected_part_count += 1;
                }
                k += 1;
            }

            if reported_part_count == expected_part_count {
                /*
                 * we have to check unique id
                 * if there is more than one
                 * Slave on the link
                 */
                unique_id = SDW_UNIQUE_ID(adr);
                if reported_part_count == 1
                    || unsafe { (*peripheral).id.unique_id == unique_id }
                {
                    unsafe {
                        dev_dbg(
                            dev,
                            b"found part_id %#x at link %d\n\0".as_ptr() as *const c_char,
                            part_id,
                            link_id,
                        );
                    }
                    break;
                }
            } else {
                unsafe {
                    dev_dbg(
                        dev,
                        b"part_id %#x reported %d expected %d on link %d, skipping\n\0".as_ptr()
                            as *const c_char,
                        part_id,
                        reported_part_count,
                        expected_part_count,
                        link_id,
                    );
                }
            }
            j += 1;
        }
        if j == unsafe { (*peripherals).num_peripherals } {
            unsafe {
                dev_dbg(
                    dev,
                    b"Slave part_id %#x not found\n\0".as_ptr() as *const c_char,
                    part_id,
                );
            }
            return false;
        }
        i += 1;
    }
    true
}

/* EXPORT_SYMBOL_GPL(snd_soc_acpi_sdw_link_slaves_found); */

/* MODULE_LICENSE("GPL v2"); */
/* MODULE_DESCRIPTION("ALSA SoC ACPI module"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
