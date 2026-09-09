// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Module Name: hwpci - Obtain PCI bus, device, and function numbers

const PCI_CFG_HEADER_TYPE_REG: u32 = 0x0E;
const PCI_CFG_PRIMARY_BUS_NUMBER_REG: u32 = 0x18;
const PCI_CFG_SECONDARY_BUS_NUMBER_REG: u32 = 0x19;
const PCI_HEADER_TYPE_MASK: u64 = 0x7F;
const PCI_TYPE_BRIDGE: u64 = 0x01;
const PCI_TYPE_CARDBUS_BRIDGE: u64 = 0x02;

#[repr(C)]
pub struct acpi_pci_device {
    pub device: acpi_handle,
    pub next: *mut acpi_pci_device,
}

unsafe extern "C" {
    fn acpi_get_parent(object: acpi_handle, out: *mut acpi_handle) -> acpi_status;
    fn acpi_get_type(object: acpi_handle, out: *mut acpi_object_type) -> acpi_status;
    fn acpi_ut_evaluate_numeric_object(
        name: *const core::ffi::c_char,
        object: acpi_handle,
        value: *mut u64,
    ) -> acpi_status;
    fn acpi_os_read_pci_configuration(
        pci_id: *mut acpi_pci_id,
        reg: u32,
        value: *mut u64,
        width: u32,
    ) -> acpi_status;
    fn ACPI_ALLOCATE(size: usize) -> *mut core::ffi::c_void;
    fn ACPI_FREE(ptr: *mut core::ffi::c_void);
}

unsafe fn acpi_hw_delete_pci_list(mut list_head: *mut acpi_pci_device) {
    while !list_head.is_null() {
        let previous = list_head;
        list_head = (*previous).next;
        ACPI_FREE(previous.cast());
    }
}

unsafe fn acpi_hw_build_pci_list(
    root_pci_device: acpi_handle,
    pci_region: acpi_handle,
    return_list_head: *mut *mut acpi_pci_device,
) -> acpi_status {
    let mut current_device = pci_region;
    let mut parent_device: acpi_handle = core::ptr::null_mut();
    *return_list_head = core::ptr::null_mut();

    loop {
        let status = acpi_get_parent(current_device, &mut parent_device);
        if ACPI_FAILURE(status) {
            acpi_hw_delete_pci_list(*return_list_head);
            return status;
        }
        if parent_device == root_pci_device {
            return AE_OK;
        }

        let list_element = ACPI_ALLOCATE(core::mem::size_of::<acpi_pci_device>())
            as *mut acpi_pci_device;
        if list_element.is_null() {
            acpi_hw_delete_pci_list(*return_list_head);
            return AE_NO_MEMORY;
        }
        (*list_element).next = *return_list_head;
        (*list_element).device = parent_device;
        *return_list_head = list_element;
        current_device = parent_device;
    }
}

unsafe fn acpi_hw_get_pci_device_info(
    pci_id: *mut acpi_pci_id,
    pci_device: acpi_handle,
    bus_number: *mut u16,
    is_bridge: *mut u8,
) -> acpi_status {
    let mut object_type: acpi_object_type = 0;
    let mut return_value: u64 = 0;
    let mut pci_value: u64 = 0;

    let mut status = acpi_get_type(pci_device, &mut object_type);
    if ACPI_FAILURE(status) { return status; }
    if object_type != ACPI_TYPE_DEVICE { return AE_OK; }

    status = acpi_ut_evaluate_numeric_object(
        METHOD_NAME__ADR.as_ptr() as *const core::ffi::c_char,
        pci_device, &mut return_value);
    if ACPI_FAILURE(status) { return AE_OK; }

    (*pci_id).device = ACPI_HIWORD(ACPI_LODWORD(return_value));
    (*pci_id).function = ACPI_LOWORD(ACPI_LODWORD(return_value));
    if *is_bridge != 0 { (*pci_id).bus = *bus_number; }

    *is_bridge = FALSE;
    status = acpi_os_read_pci_configuration(
        pci_id, PCI_CFG_HEADER_TYPE_REG, &mut pci_value, 8);
    if ACPI_FAILURE(status) { return status; }
    pci_value &= PCI_HEADER_TYPE_MASK;
    if pci_value != PCI_TYPE_BRIDGE && pci_value != PCI_TYPE_CARDBUS_BRIDGE {
        return AE_OK;
    }

    status = acpi_os_read_pci_configuration(
        pci_id, PCI_CFG_PRIMARY_BUS_NUMBER_REG, &mut pci_value, 8);
    if ACPI_FAILURE(status) { return status; }
    *is_bridge = TRUE;
    (*pci_id).bus = pci_value as u16;

    status = acpi_os_read_pci_configuration(
        pci_id, PCI_CFG_SECONDARY_BUS_NUMBER_REG, &mut pci_value, 8);
    if ACPI_FAILURE(status) { return status; }
    *bus_number = pci_value as u16;
    AE_OK
}

unsafe fn acpi_hw_process_pci_list(
    pci_id: *mut acpi_pci_id,
    mut info: *mut acpi_pci_device,
) -> acpi_status {
    let mut status = AE_OK;
    let mut bus_number = (*pci_id).bus;
    let mut is_bridge = TRUE;
    while !info.is_null() {
        status = acpi_hw_get_pci_device_info(
            pci_id, (*info).device, &mut bus_number, &mut is_bridge);
        if ACPI_FAILURE(status) { return status; }
        info = (*info).next;
    }
    AE_OK
}

pub unsafe fn acpi_hw_derive_pci_id(
    pci_id: *mut acpi_pci_id,
    root_pci_device: acpi_handle,
    pci_region: acpi_handle,
) -> acpi_status {
    if pci_id.is_null() { return AE_BAD_PARAMETER; }
    let mut list_head: *mut acpi_pci_device = core::ptr::null_mut();
    let mut status = acpi_hw_build_pci_list(
        root_pci_device, pci_region, &mut list_head);
    if ACPI_SUCCESS(status) {
        status = acpi_hw_process_pci_list(pci_id, list_head);
        acpi_hw_delete_pci_list(list_head);
    }
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
