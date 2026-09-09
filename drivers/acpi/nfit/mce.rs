// SPDX-License-Identifier: GPL-2.0-only
/*
 * NFIT - Machine Check Handler
 *
 * Copyright(c) 2013-2016 Intel Corporation. All rights reserved.
 */

// Kernel dependencies supplied by the surrounding NFIT implementation.

extern "C" {
    static mut acpi_desc_lock: mutex;
    static mut acpi_descs: list_head;
    static mut nfit_mce_dec: notifier_block;

    fn mce_is_memory_error(mce: *mut mce) -> bool;
    fn mce_is_correctable(mce: *mut mce) -> bool;
    fn mce_usable_address(mce: *mut mce) -> bool;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn nfit_spa_type(spa: *mut acpi_nfit_system_address) -> i32;
    fn nvdimm_bus_add_badrange(bus: *mut nvdimm_bus, address: u64, length: u64);
    fn nvdimm_region_notify(region: *mut nvdimm_region, event: i32);
    fn acpi_nfit_ars_rescan(desc: *mut acpi_nfit_desc, scrub: i32);
    fn mce_register_decode_chain(nb: *mut notifier_block);
    fn mce_unregister_decode_chain(nb: *mut notifier_block);
}

#[repr(C)]
pub struct notifier_block {
    pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, usize, *mut core::ffi::c_void) -> i32>,
    pub priority: i32,
}

#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct nvdimm_bus { _private: [u8; 0] }
#[repr(C)] pub struct nvdimm_region { _private: [u8; 0] }

#[repr(C)]
pub struct mce {
    pub addr: u64,
    pub misc: u64,
    pub kflags: u64,
}

#[repr(C)]
pub struct acpi_nfit_system_address {
    pub address: u64,
    pub length: u64,
    pub range_index: u16,
}

#[repr(C)]
pub struct nfit_spa {
    pub list: list_head,
    pub spa: *mut acpi_nfit_system_address,
    pub nd_region: *mut nvdimm_region,
}

#[repr(C)]
pub struct acpi_nfit_desc {
    pub list: list_head,
    pub dev: *mut device,
    pub init_mutex: mutex,
    pub spas: list_head,
    pub nvdimm_bus: *mut nvdimm_bus,
    pub scrub_mode: i32,
}

const NOTIFY_DONE: i32 = 0x0000;
const NFIT_SPA_PM: i32 = 1;
const NVDIMM_REVALIDATE_POISON: i32 = 1;
const HW_ERROR_SCRUB_ON: i32 = 1;
const MCE_HANDLED_NFIT: u64 = 1;
const MCE_PRIO_NFIT: i32 = 0;

#[inline]
unsafe fn mci_misc_addr_lsb(misc: u64) -> u32 {
    ((misc >> 6) & 0x3f) as u32
}

unsafe extern "C" fn nfit_handle_mce(
    _nb: *mut notifier_block,
    _val: usize,
    data: *mut core::ffi::c_void,
) -> i32 {
    let mce = data as *mut mce;
    let mut acpi_desc: *mut acpi_nfit_desc;
    let mut nfit_spa: *mut nfit_spa;

    if !mce_is_memory_error(mce) || mce_is_correctable(mce) {
        return NOTIFY_DONE;
    }

    if !mce_usable_address(mce) {
        return NOTIFY_DONE;
    }

    mutex_lock(&mut acpi_desc_lock);
    // list_for_each_entry(acpi_desc, &acpi_descs, list)
    acpi_desc = list_entry_first::<acpi_nfit_desc>(&mut acpi_descs, 0);
    while !acpi_desc.is_null() {
        let align: u64 = 1u64 << mci_misc_addr_lsb((*mce).misc);
        let dev = (*acpi_desc).dev;
        let mut found_match = 0;

        mutex_lock(&mut (*acpi_desc).init_mutex);
        // list_for_each_entry(nfit_spa, &acpi_desc->spas, list)
        nfit_spa = list_entry_first::<nfit_spa>(&mut (*acpi_desc).spas, 0);
        while !nfit_spa.is_null() {
            let spa = (*nfit_spa).spa;
            if nfit_spa_type(spa) != NFIT_SPA_PM {
                nfit_spa = list_entry_next::<nfit_spa>(nfit_spa, 0);
                continue;
            }
            if (*spa).address > (*mce).addr {
                nfit_spa = list_entry_next::<nfit_spa>(nfit_spa, 0);
                continue;
            }
            if (*spa).address.wrapping_add((*spa).length).wrapping_sub(1) < (*mce).addr {
                nfit_spa = list_entry_next::<nfit_spa>(nfit_spa, 0);
                continue;
            }
            found_match = 1;
            // dev_dbg(dev, "addr in SPA %d (0x%llx, 0x%llx)\n", ...)
            break;
        }
        mutex_unlock(&mut (*acpi_desc).init_mutex);

        if found_match == 0 {
            acpi_desc = list_entry_next::<acpi_nfit_desc>(acpi_desc, 0);
            continue;
        }

        nvdimm_bus_add_badrange((*acpi_desc).nvdimm_bus, (*mce).addr & !(align - 1), align);
        nvdimm_region_notify((*nfit_spa).nd_region, NVDIMM_REVALIDATE_POISON);
        if (*acpi_desc).scrub_mode == HW_ERROR_SCRUB_ON {
            acpi_nfit_ars_rescan(acpi_desc, 0);
        }
        (*mce).kflags |= MCE_HANDLED_NFIT;
        break;
    }

    mutex_unlock(&mut acpi_desc_lock);
    NOTIFY_DONE
}

// The list helpers correspond to the kernel list_for_each_entry macros.
extern "C" {
    fn list_entry_first<T>(head: *mut list_head, member: usize) -> *mut T;
    fn list_entry_next<T>(entry: *mut T, member: usize) -> *mut T;
}

#[no_mangle]
pub static mut nfit_mce_dec: notifier_block = notifier_block {
    notifier_call: Some(nfit_handle_mce),
    priority: MCE_PRIO_NFIT,
};

#[no_mangle]
pub unsafe extern "C" fn nfit_mce_register() {
    mce_register_decode_chain(&mut nfit_mce_dec);
}

#[no_mangle]
pub unsafe extern "C" fn nfit_mce_unregister() {
    mce_unregister_decode_chain(&mut nfit_mce_dec);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
