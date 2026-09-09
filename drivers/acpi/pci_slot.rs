// SPDX-License-Identifier: GPL-2.0-only
/*
 * pci_slot.c - ACPI PCI Slot Driver
 *
 * Rust translation of the original implementation source.
 */

// Kernel headers and build-time configuration supplied by the surrounding
// kernel translation are intentionally not reproduced here.

const SLOT_NAME_SIZE: usize = 21;

static mut CHECK_STA_BEFORE_SUN: i32 = 0;

#[repr(C)]
struct AcpiPciSlot {
    pci_slot: *mut PciSlot,
    list: ListHead,
}

#[repr(C)]
struct ListHead {
    next: *mut ListHead,
    prev: *mut ListHead,
}

#[repr(C)]
struct AcpiBuffer {
    length: usize,
    pointer: *mut core::ffi::c_void,
}

#[repr(C)]
struct PciSlot {
    bus: *mut PciBus,
    number: i32,
}

#[repr(C)]
struct PciBus {
    bridge: *mut Device,
    number: u8,
    dev: Device,
}

#[repr(C)]
struct Device;
#[repr(C)]
struct DmiSystemId {
    callback: Option<unsafe extern "C" fn(*const DmiSystemId) -> i32>,
    ident: *const u8,
    matches: [DmiMatch; 2],
}
#[repr(C)]
struct DmiMatch;

type AcpiHandle = *mut core::ffi::c_void;
type AcpiStatus = i32;
type U64 = u64;
type U32 = u32;

const ACPI_ALLOCATE_BUFFER: usize = usize::MAX;
const ACPI_FULL_PATHNAME: u32 = 0;
const ACPI_TYPE_DEVICE: u32 = 1;
const ACPI_STA_DEVICE_PRESENT: u64 = 1;
const AE_OK: AcpiStatus = 0;

static mut SLOT_LIST: ListHead = ListHead { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut SLOT_LIST_LOCK: core::ffi::c_void = core::mem::MaybeUninit::uninit().assume_init();

extern "C" {
    fn acpi_get_name(handle: AcpiHandle, name_type: u32, buffer: *mut AcpiBuffer) -> AcpiStatus;
    fn acpi_evaluate_integer(handle: AcpiHandle, pathname: *const u8, args: *mut core::ffi::c_void, data: *mut u64) -> AcpiStatus;
    fn acpi_get_local_u64_address(handle: AcpiHandle, address: *mut u64) -> AcpiStatus;
    fn acpi_walk_namespace(ty: u32, handle: AcpiHandle, max_depth: u32, callback: unsafe extern "C" fn(AcpiHandle, U32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> AcpiStatus, pre: *mut core::ffi::c_void, context: *mut core::ffi::c_void, ret: *mut *mut core::ffi::c_void) -> AcpiStatus;
    fn pci_create_slot(bus: *mut PciBus, device: i32, name: *const u8, parent: *mut core::ffi::c_void) -> *mut PciSlot;
    fn pci_destroy_slot(slot: *mut PciSlot);
    fn dmi_check_system(table: *const DmiSystemId);
    fn kfree(ptr: *mut core::ffi::c_void);
    fn kmalloc(size: usize) -> *mut core::ffi::c_void;
    fn get_device(dev: *mut Device) -> *mut Device;
    fn put_device(dev: *mut Device);
    fn mutex_lock(lock: *mut core::ffi::c_void);
    fn mutex_unlock(lock: *mut core::ffi::c_void);
}

unsafe fn check_slot(handle: AcpiHandle, sun: *mut u64) -> i32 {
    let mut device: i32 = -1;
    let mut sta: u64 = 0;
    let mut adr: u64 = 0;
    let mut buffer = AcpiBuffer { length: ACPI_ALLOCATE_BUFFER, pointer: core::ptr::null_mut() };

    acpi_get_name(handle, ACPI_FULL_PATHNAME, &mut buffer);
    if CHECK_STA_BEFORE_SUN != 0 {
        let status = acpi_evaluate_integer(handle, b"_STA\0".as_ptr(), core::ptr::null_mut(), &mut sta);
        if status == AE_OK && (sta & ACPI_STA_DEVICE_PRESENT) == 0 {
            kfree(buffer.pointer);
            return device;
        }
    }
    if acpi_get_local_u64_address(handle, &mut adr) != AE_OK {
        kfree(buffer.pointer);
        return device;
    }
    if acpi_evaluate_integer(handle, b"_SUN\0".as_ptr(), core::ptr::null_mut(), sun) != AE_OK {
        kfree(buffer.pointer);
        return device;
    }
    device = ((adr >> 16) & 0xffff) as i32;
    kfree(buffer.pointer);
    device
}

unsafe extern "C" fn register_slot(handle: AcpiHandle, _lvl: U32, context: *mut core::ffi::c_void, _rv: *mut *mut core::ffi::c_void) -> AcpiStatus {
    let mut sun = 0u64;
    let device = check_slot(handle, &mut sun);
    if device < 0 { return AE_OK; }
    let pci_bus = context as *mut PciBus;
    // list_for_each_entry(slot, &slot_list, list)
    let mut slot: *mut AcpiPciSlot = core::ptr::null_mut();
    while !slot.is_null() {
        let pci_slot = (*slot).pci_slot;
        if (*pci_slot).bus == pci_bus && (*pci_slot).number == device { return AE_OK; }
        break;
    }
    slot = kmalloc(core::mem::size_of::<AcpiPciSlot>()) as *mut AcpiPciSlot;
    if slot.is_null() { return AE_OK; }
    let mut name = [0u8; SLOT_NAME_SIZE];
    let text = sun.to_string();
    let bytes = text.as_bytes();
    let n = core::cmp::min(bytes.len(), name.len() - 1);
    name[..n].copy_from_slice(&bytes[..n]);
    let pci_slot = pci_create_slot(pci_bus, device, name.as_ptr(), core::ptr::null_mut());
    if pci_slot.is_null() { kfree(slot.cast()); return AE_OK; }
    (*slot).pci_slot = pci_slot;
    get_device(&mut (*pci_bus).dev);
    AE_OK
}

pub unsafe extern "C" fn acpi_pci_slot_enumerate(bus: *mut PciBus) {
    let handle = (*bus).bridge as AcpiHandle;
    if !handle.is_null() {
        mutex_lock(&raw mut SLOT_LIST_LOCK);
        acpi_walk_namespace(ACPI_TYPE_DEVICE, handle, 1, register_slot, core::ptr::null_mut(), bus.cast(), core::ptr::null_mut());
        mutex_unlock(&raw mut SLOT_LIST_LOCK);
    }
}

pub unsafe extern "C" fn acpi_pci_slot_remove(bus: *mut PciBus) {
    mutex_lock(&raw mut SLOT_LIST_LOCK);
    // list_for_each_entry_safe(slot, tmp, &slot_list, list)
    let slot: *mut AcpiPciSlot = core::ptr::null_mut();
    let _tmp: *mut AcpiPciSlot = core::ptr::null_mut();
    if !slot.is_null() && (*slot).pci_slot.as_ref().unwrap().bus == bus {
        pci_destroy_slot((*slot).pci_slot);
        put_device(&mut (*bus).dev);
        kfree(slot.cast());
    }
    mutex_unlock(&raw mut SLOT_LIST_LOCK);
}

unsafe extern "C" fn do_sta_before_sun(_d: *const DmiSystemId) -> i32 {
    CHECK_STA_BEFORE_SUN = 1;
    0
}

#[used]
static ACPI_PCI_SLOT_DMI_TABLE: [DmiSystemId; 2] = [
    DmiSystemId { callback: Some(do_sta_before_sun), ident: b"Fujitsu PRIMEQUEST\0".as_ptr(), matches: [DmiMatch, DmiMatch] },
    DmiSystemId { callback: None, ident: core::ptr::null(), matches: [DmiMatch, DmiMatch] },
];

pub unsafe extern "C" fn acpi_pci_slot_init() {
    dmi_check_system(ACPI_PCI_SLOT_DMI_TABLE.as_ptr());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
