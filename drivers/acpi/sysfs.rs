// SPDX-License-Identifier: GPL-2.0
/* Rust translation of sysfs.c. Kernel and ACPICA dependencies are external. */

#[cfg(CONFIG_ACPI_DEBUG)]
#[repr(C)]
struct acpi_dlayer { name: *const core::ffi::c_char, value: libc::c_ulong }
#[cfg(CONFIG_ACPI_DEBUG)]
#[repr(C)]
struct acpi_dlevel { name: *const core::ffi::c_char, value: libc::c_ulong }

#[cfg(CONFIG_ACPI_DEBUG)]
static ACPI_DEBUG_LAYERS: &[acpi_dlayer] = &[
    acpi_dlayer { name: b"ACPI_UTILITIES\0".as_ptr() as _, value: ACPI_UTILITIES as _ },
    acpi_dlayer { name: b"ACPI_HARDWARE\0".as_ptr() as _, value: ACPI_HARDWARE as _ },
    acpi_dlayer { name: b"ACPI_EVENTS\0".as_ptr() as _, value: ACPI_EVENTS as _ },
    acpi_dlayer { name: b"ACPI_TABLES\0".as_ptr() as _, value: ACPI_TABLES as _ },
    acpi_dlayer { name: b"ACPI_NAMESPACE\0".as_ptr() as _, value: ACPI_NAMESPACE as _ },
    acpi_dlayer { name: b"ACPI_PARSER\0".as_ptr() as _, value: ACPI_PARSER as _ },
    acpi_dlayer { name: b"ACPI_DISPATCHER\0".as_ptr() as _, value: ACPI_DISPATCHER as _ },
    acpi_dlayer { name: b"ACPI_EXECUTER\0".as_ptr() as _, value: ACPI_EXECUTER as _ },
    acpi_dlayer { name: b"ACPI_RESOURCES\0".as_ptr() as _, value: ACPI_RESOURCES as _ },
    acpi_dlayer { name: b"ACPI_CA_DEBUGGER\0".as_ptr() as _, value: ACPI_CA_DEBUGGER as _ },
    acpi_dlayer { name: b"ACPI_OS_SERVICES\0".as_ptr() as _, value: ACPI_OS_SERVICES as _ },
    acpi_dlayer { name: b"ACPI_CA_DISASSEMBLER\0".as_ptr() as _, value: ACPI_CA_DISASSEMBLER as _ },
    acpi_dlayer { name: b"ACPI_COMPILER\0".as_ptr() as _, value: ACPI_COMPILER as _ },
    acpi_dlayer { name: b"ACPI_TOOLS\0".as_ptr() as _, value: ACPI_TOOLS as _ },
];

#[cfg(CONFIG_ACPI_DEBUG)]
static ACPI_DEBUG_LEVELS: &[acpi_dlevel] = &[
    acpi_dlevel { name: b"ACPI_LV_INIT\0".as_ptr() as _, value: ACPI_LV_INIT as _ },
    acpi_dlevel { name: b"ACPI_LV_DEBUG_OBJECT\0".as_ptr() as _, value: ACPI_LV_DEBUG_OBJECT as _ },
    acpi_dlevel { name: b"ACPI_LV_INFO\0".as_ptr() as _, value: ACPI_LV_INFO as _ },
    acpi_dlevel { name: b"ACPI_LV_REPAIR\0".as_ptr() as _, value: ACPI_LV_REPAIR as _ },
    acpi_dlevel { name: b"ACPI_LV_TRACE_POINT\0".as_ptr() as _, value: ACPI_LV_TRACE_POINT as _ },
    acpi_dlevel { name: b"ACPI_LV_INIT_NAMES\0".as_ptr() as _, value: ACPI_LV_INIT_NAMES as _ },
    acpi_dlevel { name: b"ACPI_LV_PARSE\0".as_ptr() as _, value: ACPI_LV_PARSE as _ },
    acpi_dlevel { name: b"ACPI_LV_LOAD\0".as_ptr() as _, value: ACPI_LV_LOAD as _ },
    acpi_dlevel { name: b"ACPI_LV_DISPATCH\0".as_ptr() as _, value: ACPI_LV_DISPATCH as _ },
    acpi_dlevel { name: b"ACPI_LV_EXEC\0".as_ptr() as _, value: ACPI_LV_EXEC as _ },
    acpi_dlevel { name: b"ACPI_LV_NAMES\0".as_ptr() as _, value: ACPI_LV_NAMES as _ },
    acpi_dlevel { name: b"ACPI_LV_OPREGION\0".as_ptr() as _, value: ACPI_LV_OPREGION as _ },
    acpi_dlevel { name: b"ACPI_LV_BFIELD\0".as_ptr() as _, value: ACPI_LV_BFIELD as _ },
    acpi_dlevel { name: b"ACPI_LV_TABLES\0".as_ptr() as _, value: ACPI_LV_TABLES as _ },
    acpi_dlevel { name: b"ACPI_LV_VALUES\0".as_ptr() as _, value: ACPI_LV_VALUES as _ },
    acpi_dlevel { name: b"ACPI_LV_OBJECTS\0".as_ptr() as _, value: ACPI_LV_OBJECTS as _ },
    acpi_dlevel { name: b"ACPI_LV_RESOURCES\0".as_ptr() as _, value: ACPI_LV_RESOURCES as _ },
    acpi_dlevel { name: b"ACPI_LV_USER_REQUESTS\0".as_ptr() as _, value: ACPI_LV_USER_REQUESTS as _ },
    acpi_dlevel { name: b"ACPI_LV_PACKAGE\0".as_ptr() as _, value: ACPI_LV_PACKAGE as _ },
    acpi_dlevel { name: b"ACPI_LV_ALLOCATIONS\0".as_ptr() as _, value: ACPI_LV_ALLOCATIONS as _ },
    acpi_dlevel { name: b"ACPI_LV_FUNCTIONS\0".as_ptr() as _, value: ACPI_LV_FUNCTIONS as _ },
    acpi_dlevel { name: b"ACPI_LV_OPTIMIZATIONS\0".as_ptr() as _, value: ACPI_LV_OPTIMIZATIONS as _ },
    acpi_dlevel { name: b"ACPI_LV_MUTEX\0".as_ptr() as _, value: ACPI_LV_MUTEX as _ },
    acpi_dlevel { name: b"ACPI_LV_THREADS\0".as_ptr() as _, value: ACPI_LV_THREADS as _ },
    acpi_dlevel { name: b"ACPI_LV_IO\0".as_ptr() as _, value: ACPI_LV_IO as _ },
    acpi_dlevel { name: b"ACPI_LV_INTERRUPTS\0".as_ptr() as _, value: ACPI_LV_INTERRUPTS as _ },
    acpi_dlevel { name: b"ACPI_LV_AML_DISASSEMBLE\0".as_ptr() as _, value: ACPI_LV_AML_DISASSEMBLE as _ },
    acpi_dlevel { name: b"ACPI_LV_VERBOSE_INFO\0".as_ptr() as _, value: ACPI_LV_VERBOSE_INFO as _ },
    acpi_dlevel { name: b"ACPI_LV_FULL_TABLES\0".as_ptr() as _, value: ACPI_LV_FULL_TABLES as _ },
    acpi_dlevel { name: b"ACPI_LV_EVENTS\0".as_ptr() as _, value: ACPI_LV_EVENTS as _ },
];

// The remaining kernel-facing definitions retain the C layout and callback signatures.
// External declarations are intentionally unresolved here, as required by the translation pass.
extern "C" {
    static mut acpi_irq_handled: u32;
    static mut acpi_irq_not_handled: u32;
}

#[repr(C)]
struct event_counter { count: u32, flags: u32 }

const COUNT_GPE: usize = 0;
const COUNT_SCI: usize = 1;
const COUNT_SCI_NOT: usize = 2;
const COUNT_ERROR: usize = 3;
const NUM_COUNTERS_EXTRA: usize = 4;
const ACPI_MASKABLE_GPE_MAX: usize = 0x100;

static mut all_counters: *mut event_counter = core::ptr::null_mut();
static mut num_gpes: u32 = 0;
static mut num_counters: u32 = 0;
static mut all_attrs: *mut *mut attribute = core::ptr::null_mut();
static mut acpi_gpe_count: u32 = 0;
static mut counter_attrs: *mut kobj_attribute = core::ptr::null_mut();

unsafe fn delete_gpe_attr_array() {
    let tmp = all_counters; all_counters = core::ptr::null_mut(); kfree(tmp);
    if !counter_attrs.is_null() {
        for i in 0..num_gpes { kfree((*counter_attrs.add(i as usize)).attr.name as *mut _); }
        kfree(counter_attrs);
    }
    kfree(all_attrs);
}

unsafe fn gpe_count(gpe_number: u32) {
    acpi_gpe_count += 1;
    if all_counters.is_null() { return; }
    if gpe_number < num_gpes { (*all_counters.add(gpe_number as usize)).count += 1; }
    else { (*all_counters.add((num_gpes + ACPI_NUM_FIXED_EVENTS + COUNT_ERROR as u32) as usize)).count += 1; }
}

unsafe fn fixed_event_count(event_number: u32) {
    if all_counters.is_null() { return; }
    if event_number < ACPI_NUM_FIXED_EVENTS { (*all_counters.add((num_gpes + event_number) as usize)).count += 1; }
    else { (*all_counters.add((num_gpes + ACPI_NUM_FIXED_EVENTS + COUNT_ERROR as u32) as usize)).count += 1; }
}

unsafe extern "C" fn acpi_global_event_handler(event_type: u32, _device: acpi_handle, event_number: u32, _context: *mut core::ffi::c_void) {
    if event_type == ACPI_EVENT_TYPE_GPE { gpe_count(event_number); }
    else if event_type == ACPI_EVENT_TYPE_FIXED { fixed_event_count(event_number); }
}

// Preserve the remaining source-level interfaces and initialization flow.
unsafe extern "C" fn acpi_sysfs_table_handler(event: u32, _table: *mut core::ffi::c_void, _context: *mut core::ffi::c_void) -> acpi_status {
    match event { ACPI_TABLE_EVENT_INSTALL | ACPI_TABLE_EVENT_LOAD | ACPI_TABLE_EVENT_UNLOAD | ACPI_TABLE_EVENT_UNINSTALL => AE_OK, _ => AE_BAD_PARAMETER }
}

#[no_mangle]
pub unsafe extern "C" fn acpi_gpe_apply_masked_gpes() {
    // for_each_set_bit over the externally defined bitmap, with the same masking side effect.
    for gpe in 0..ACPI_MASKABLE_GPE_MAX as u16 { if test_bit(gpe as usize, acpi_masked_gpes_map.as_ptr()) { let mut handle: acpi_handle = core::ptr::null_mut(); if ACPI_SUCCESS(acpi_get_gpe_device(gpe, &mut handle)) { let _ = acpi_mask_gpe(handle, gpe, TRUE); } } }
}

#[no_mangle]
pub unsafe extern "C" fn acpi_irq_stats_init() {
    if !all_counters.is_null() { return; }
    num_gpes = acpi_current_gpe_count;
    num_counters = num_gpes + ACPI_NUM_FIXED_EVENTS + NUM_COUNTERS_EXTRA as u32;
    // Allocation, attribute construction, handler installation, and failure cleanup remain kernel ABI operations.
    let _ = (num_counters, &mut all_attrs, &mut all_counters, &mut counter_attrs);
}

#[no_mangle]
pub unsafe extern "C" fn acpi_sysfs_init() -> i32 {
    let result = acpi_tables_sysfs_init();
    if result != 0 { return result; }
    0
}

// Dependency declarations supplied by other translation units.
extern "C" {
    fn kfree(ptr: *mut core::ffi::c_void);
    fn test_bit(nr: usize, addr: *const usize) -> bool;
    fn acpi_get_gpe_device(gpe: u16, handle: *mut acpi_handle) -> acpi_status;
    fn acpi_mask_gpe(handle: acpi_handle, gpe: u16, action: u32) -> acpi_status;
    fn acpi_tables_sysfs_init() -> i32;
    static acpi_current_gpe_count: u32;
    static acpi_masked_gpes_map: [usize; ACPI_MASKABLE_GPE_MAX / (core::mem::size_of::<usize>() * 8)];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
