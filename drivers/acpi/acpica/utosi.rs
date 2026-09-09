// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/*
 * Module Name: utosi - Support for the _OSI predefined control method
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 */

// Dependencies supplied by the ACPICA translation unit are intentionally not
// redefined here.

/*
 * ACPICA policy for new _OSI strings:
 *
 * It is the stated policy of ACPICA that new _OSI strings will be integrated
 * into this module as soon as possible after they are defined. It is strongly
 * recommended that all ACPICA hosts mirror this policy and integrate any
 * changes to this module as soon as possible. There are several historical
 * reasons behind this policy:
 *
 * 1) New BIOSs tend to test only the case where the host responds TRUE to
 *    the latest version of Windows, which would respond to the latest/newest
 *    _OSI string. Not responding TRUE to the latest version of Windows will
 *    risk executing untested code paths throughout the DSDT and SSDTs.
 *
 * 2) If a new _OSI string is recognized only after a significant delay, this
 *    has the potential to cause problems on existing working machines because
 *    of the possibility that a new and different path through the ASL code
 *    will be executed.
 *
 * 3) New _OSI strings are tending to come out about once per year. A delay
 *    in recognizing a new string for a significant amount of time risks the
 *    release of another string which only compounds the initial problem.
 */

/* Strings supported by the _OSI predefined control method. */
static mut acpi_default_supported_interfaces: [acpi_interface_info; 30] = [
    acpi_interface_info { name: b"Windows 2000\0".as_ptr() as acpi_string, next: core::ptr::null_mut(), flags: 0, value: ACPI_OSI_WIN_2000 },
    acpi_interface_info { name: b"Windows 2001\0".as_ptr() as acpi_string, next: core::ptr::null_mut(), flags: 0, value: ACPI_OSI_WIN_XP },
    acpi_interface_info { name: b"Windows 2001 SP1\0".as_ptr() as acpi_string, next: core::ptr::null_mut(), flags: 0, value: ACPI_OSI_WIN_XP_SP1 },
    acpi_interface_info { name: b"Windows 2001.1\0".as_ptr() as acpi_string, next: core::ptr::null_mut(), flags: 0, value: ACPI_OSI_WINSRV_2003 },
    acpi_interface_info { name: b"Windows 2001 SP2\0".as_ptr() as acpi_string, next: core::ptr::null_mut(), flags: 0, value: ACPI_OSI_WIN_XP_SP2 },
    acpi_interface_info { name: b"Windows 2001.1 SP1\0".as_ptr() as acpi_string, next: core::ptr::null_mut(), flags: 0, value: ACPI_OSI_WINSRV_2003_SP1 },
    acpi_interface_info { name: b"Windows 2006\0".as_ptr() as acpi_string, next: core::ptr::null_mut(), flags: 0, value: ACPI_OSI_WIN_VISTA },
    acpi_interface_info { name: b"Windows 2006.1\0".as_ptr() as acpi_string, next: core::ptr::null_mut(), flags: 0, value: ACPI_OSI_WINSRV_2008 },
    acpi_interface_info { name: b"Windows 2006 SP1\0".as_ptr() as acpi_string, next: core::ptr::null_mut(), flags: 0, value: ACPI_OSI_WIN_VISTA_SP1 },
    acpi_interface_info { name: b"Windows 2006 SP2\0".as_ptr() as acpi_string, next: core::ptr::null_mut(), flags: 0, value: ACPI_OSI_WIN_VISTA_SP2 },
    acpi_interface_info { name: b"Windows 2009\0".as_ptr() as acpi_string, next: core::ptr::null_mut(), flags: 0, value: ACPI_OSI_WIN_7 },
    acpi_interface_info { name: b"Windows 2012\0".as_ptr() as acpi_string, next: core::ptr::null_mut(), flags: 0, value: ACPI_OSI_WIN_8 },
    acpi_interface_info { name: b"Windows 2013\0".as_ptr() as acpi_string, next: core::ptr::null_mut(), flags: 0, value: ACPI_OSI_WIN_8_1 },
    acpi_interface_info { name: b"Windows 2015\0".as_ptr() as acpi_string, next: core::ptr::null_mut(), flags: 0, value: ACPI_OSI_WIN_10 },
    acpi_interface_info { name: b"Windows 2016\0".as_ptr() as acpi_string, next: core::ptr::null_mut(), flags: 0, value: ACPI_OSI_WIN_10_RS1 },
    acpi_interface_info { name: b"Windows 2017\0".as_ptr() as acpi_string, next: core::ptr::null_mut(), flags: 0, value: ACPI_OSI_WIN_10_RS2 },
    acpi_interface_info { name: b"Windows 2017.2\0".as_ptr() as acpi_string, next: core::ptr::null_mut(), flags: 0, value: ACPI_OSI_WIN_10_RS3 },
    acpi_interface_info { name: b"Windows 2018\0".as_ptr() as acpi_string, next: core::ptr::null_mut(), flags: 0, value: ACPI_OSI_WIN_10_RS4 },
    acpi_interface_info { name: b"Windows 2018.2\0".as_ptr() as acpi_string, next: core::ptr::null_mut(), flags: 0, value: ACPI_OSI_WIN_10_RS5 },
    acpi_interface_info { name: b"Windows 2019\0".as_ptr() as acpi_string, next: core::ptr::null_mut(), flags: 0, value: ACPI_OSI_WIN_10_19H1 },
    acpi_interface_info { name: b"Windows 2020\0".as_ptr() as acpi_string, next: core::ptr::null_mut(), flags: 0, value: ACPI_OSI_WIN_10_20H1 },
    acpi_interface_info { name: b"Windows 2021\0".as_ptr() as acpi_string, next: core::ptr::null_mut(), flags: 0, value: ACPI_OSI_WIN_11 },
    acpi_interface_info { name: b"Windows 2022\0".as_ptr() as acpi_string, next: core::ptr::null_mut(), flags: 0, value: ACPI_OSI_WIN_11_22H2 },
    acpi_interface_info { name: b"Extended Address Space Descriptor\0".as_ptr() as acpi_string, next: core::ptr::null_mut(), flags: ACPI_OSI_FEATURE, value: 0 },
    acpi_interface_info { name: b"Module Device\0".as_ptr() as acpi_string, next: core::ptr::null_mut(), flags: ACPI_OSI_OPTIONAL_FEATURE, value: 0 },
    acpi_interface_info { name: b"Processor Device\0".as_ptr() as acpi_string, next: core::ptr::null_mut(), flags: ACPI_OSI_OPTIONAL_FEATURE, value: 0 },
    acpi_interface_info { name: b"3.0 Thermal Model\0".as_ptr() as acpi_string, next: core::ptr::null_mut(), flags: ACPI_OSI_OPTIONAL_FEATURE, value: 0 },
    acpi_interface_info { name: b"3.0 _SCP Extensions\0".as_ptr() as acpi_string, next: core::ptr::null_mut(), flags: ACPI_OSI_OPTIONAL_FEATURE, value: 0 },
    acpi_interface_info { name: b"Processor Aggregator Device\0".as_ptr() as acpi_string, next: core::ptr::null_mut(), flags: ACPI_OSI_OPTIONAL_FEATURE, value: 0 },
    acpi_interface_info { name: b"DisplayMux\0".as_ptr() as acpi_string, next: core::ptr::null_mut(), flags: ACPI_OSI_OPTIONAL_FEATURE, value: 0 },
];

pub unsafe fn acpi_ut_initialize_interfaces() -> acpi_status {
    let status = acpi_os_acquire_mutex(acpi_gbl_osi_mutex, ACPI_WAIT_FOREVER);
    if ACPI_FAILURE(status) { return status; }
    acpi_gbl_supported_interfaces = &mut acpi_default_supported_interfaces[0];
    for i in 0..(acpi_default_supported_interfaces.len() - 1) {
        acpi_default_supported_interfaces[i].next = &mut acpi_default_supported_interfaces[i + 1];
    }
    acpi_os_release_mutex(acpi_gbl_osi_mutex);
    AE_OK
}

pub unsafe fn acpi_ut_interface_terminate() -> acpi_status {
    let status = acpi_os_acquire_mutex(acpi_gbl_osi_mutex, ACPI_WAIT_FOREVER);
    if ACPI_FAILURE(status) { return status; }
    let mut next_interface = acpi_gbl_supported_interfaces;
    while !next_interface.is_null() {
        acpi_gbl_supported_interfaces = (*next_interface).next;
        if (*next_interface).flags & ACPI_OSI_DYNAMIC != 0 {
            ACPI_FREE((*next_interface).name);
            ACPI_FREE(next_interface);
        } else if (*next_interface).flags & ACPI_OSI_DEFAULT_INVALID != 0 {
            (*next_interface).flags |= ACPI_OSI_INVALID;
        } else {
            (*next_interface).flags &= !ACPI_OSI_INVALID;
        }
        next_interface = acpi_gbl_supported_interfaces;
    }
    acpi_os_release_mutex(acpi_gbl_osi_mutex);
    AE_OK
}

pub unsafe fn acpi_ut_install_interface(interface_name: acpi_string) -> acpi_status {
    let interface_info = ACPI_ALLOCATE_ZEROED(core::mem::size_of::<acpi_interface_info>()) as *mut acpi_interface_info;
    if interface_info.is_null() { return AE_NO_MEMORY; }
    let len = strlen(interface_name);
    (*interface_info).name = ACPI_ALLOCATE_ZEROED(len + 1) as acpi_string;
    if (*interface_info).name.is_null() { ACPI_FREE(interface_info); return AE_NO_MEMORY; }
    strcpy((*interface_info).name, interface_name);
    (*interface_info).flags = ACPI_OSI_DYNAMIC;
    (*interface_info).next = acpi_gbl_supported_interfaces;
    acpi_gbl_supported_interfaces = interface_info;
    AE_OK
}

pub unsafe fn acpi_ut_remove_interface(interface_name: acpi_string) -> acpi_status {
    let mut previous_interface = acpi_gbl_supported_interfaces;
    let mut next_interface = acpi_gbl_supported_interfaces;
    while !next_interface.is_null() {
        if strcmp(interface_name, (*next_interface).name) == 0 {
            if (*next_interface).flags & ACPI_OSI_DYNAMIC != 0 {
                if previous_interface == next_interface { acpi_gbl_supported_interfaces = (*next_interface).next; }
                else { (*previous_interface).next = (*next_interface).next; }
                ACPI_FREE((*next_interface).name);
                ACPI_FREE(next_interface);
            } else {
                if (*next_interface).flags & ACPI_OSI_INVALID != 0 { return AE_NOT_EXIST; }
                (*next_interface).flags |= ACPI_OSI_INVALID;
            }
            return AE_OK;
        }
        previous_interface = next_interface;
        next_interface = (*next_interface).next;
    }
    AE_NOT_EXIST
}

pub unsafe fn acpi_ut_update_interfaces(action: u8) -> acpi_status {
    let mut next_interface = acpi_gbl_supported_interfaces;
    while !next_interface.is_null() {
        if (((*next_interface).flags & ACPI_OSI_FEATURE != 0) && (action & ACPI_FEATURE_STRINGS != 0)) ||
           (((*next_interface).flags & ACPI_OSI_FEATURE == 0) && (action & ACPI_VENDOR_STRINGS != 0)) {
            if action & ACPI_DISABLE_INTERFACES != 0 { (*next_interface).flags |= ACPI_OSI_INVALID; }
            else { (*next_interface).flags &= !ACPI_OSI_INVALID; }
        }
        next_interface = (*next_interface).next;
    }
    AE_OK
}

pub unsafe fn acpi_ut_get_interface(interface_name: acpi_string) -> *mut acpi_interface_info {
    let mut next_interface = acpi_gbl_supported_interfaces;
    while !next_interface.is_null() {
        if strcmp(interface_name, (*next_interface).name) == 0 { return next_interface; }
        next_interface = (*next_interface).next;
    }
    core::ptr::null_mut()
}

pub unsafe fn acpi_ut_osi_implementation(walk_state: *mut acpi_walk_state) -> acpi_status {
    let string_desc = (*walk_state).arguments[0].object;
    if string_desc.is_null() || (*string_desc).common.type_ != ACPI_TYPE_STRING { return AE_TYPE; }
    let return_desc = acpi_ut_create_internal_object(ACPI_TYPE_INTEGER);
    if return_desc.is_null() { return AE_NO_MEMORY; }
    let mut return_value: u64 = 0;
    let status = acpi_os_acquire_mutex(acpi_gbl_osi_mutex, ACPI_WAIT_FOREVER);
    if ACPI_FAILURE(status) { acpi_ut_remove_reference(return_desc); return status; }
    let interface_info = acpi_ut_get_interface((*string_desc).string.pointer);
    if !interface_info.is_null() && (*interface_info).flags & ACPI_OSI_INVALID == 0 {
        if (*interface_info).value > acpi_gbl_osi_data { acpi_gbl_osi_data = (*interface_info).value; }
        return_value = ACPI_UINT64_MAX;
    }
    acpi_os_release_mutex(acpi_gbl_osi_mutex);
    let interface_handler = acpi_gbl_interface_handler;
    if let Some(handler) = interface_handler {
        if handler((*string_desc).string.pointer, return_value as u32) { return_value = ACPI_UINT64_MAX; }
    }
    (*return_desc).integer.value = return_value;
    (*walk_state).return_desc = return_desc;
    AE_OK
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
