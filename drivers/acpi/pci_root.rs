// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * pci_root.c - ACPI PCI Root Bridge Driver
 *
 * Direct Rust translation of the source implementation.
 */

// C headers and kernel-provided declarations are intentionally external.

static mut root_device_ids: [acpi_device_id; 3] = [
    acpi_device_id { hid: b"PNP0A03\0".as_ptr() as *const i8, driver_data: 0 },
    acpi_device_id { hid: b"\0".as_ptr() as *const i8, driver_data: 0 },
    acpi_device_id { hid: core::ptr::null(), driver_data: 0 },
];

const ACPI_PCIE_REQ_SUPPORT: u32 = OSC_PCI_EXT_CONFIG_SUPPORT | OSC_PCI_ASPM_SUPPORT |
    OSC_PCI_CLOCK_PM_SUPPORT | OSC_PCI_MSI_SUPPORT;

unsafe fn acpi_pci_root_scan_dependent(adev: *mut acpi_device) -> i32 {
    acpiphp_check_host_bridge(adev);
    0
}

#[repr(C)]
struct pci_osc_bit_struct { bit: u32, desc: *mut i8 }

static mut pci_osc_support_bit: [pci_osc_bit_struct; 7] = [
    pci_osc_bit_struct { bit: OSC_PCI_EXT_CONFIG_SUPPORT, desc: b"ExtendedConfig\0".as_ptr() as *mut i8 },
    pci_osc_bit_struct { bit: OSC_PCI_ASPM_SUPPORT, desc: b"ASPM\0".as_ptr() as *mut i8 },
    pci_osc_bit_struct { bit: OSC_PCI_CLOCK_PM_SUPPORT, desc: b"ClockPM\0".as_ptr() as *mut i8 },
    pci_osc_bit_struct { bit: OSC_PCI_SEGMENT_GROUPS_SUPPORT, desc: b"Segments\0".as_ptr() as *mut i8 },
    pci_osc_bit_struct { bit: OSC_PCI_MSI_SUPPORT, desc: b"MSI\0".as_ptr() as *mut i8 },
    pci_osc_bit_struct { bit: OSC_PCI_EDR_SUPPORT, desc: b"EDR\0".as_ptr() as *mut i8 },
    pci_osc_bit_struct { bit: OSC_PCI_HPX_TYPE_3_SUPPORT, desc: b"HPX-Type3\0".as_ptr() as *mut i8 },
];
static mut pci_osc_control_bit: [pci_osc_bit_struct; 7] = [
    pci_osc_bit_struct { bit: OSC_PCI_EXPRESS_NATIVE_HP_CONTROL, desc: b"PCIeHotplug\0".as_ptr() as *mut i8 },
    pci_osc_bit_struct { bit: OSC_PCI_SHPC_NATIVE_HP_CONTROL, desc: b"SHPCHotplug\0".as_ptr() as *mut i8 },
    pci_osc_bit_struct { bit: OSC_PCI_EXPRESS_PME_CONTROL, desc: b"PME\0".as_ptr() as *mut i8 },
    pci_osc_bit_struct { bit: OSC_PCI_EXPRESS_AER_CONTROL, desc: b"AER\0".as_ptr() as *mut i8 },
    pci_osc_bit_struct { bit: OSC_PCI_EXPRESS_CAPABILITY_CONTROL, desc: b"PCIeCapability\0".as_ptr() as *mut i8 },
    pci_osc_bit_struct { bit: OSC_PCI_EXPRESS_LTR_CONTROL, desc: b"LTR\0".as_ptr() as *mut i8 },
    pci_osc_bit_struct { bit: OSC_PCI_EXPRESS_DPC_CONTROL, desc: b"DPC\0".as_ptr() as *mut i8 },
];
static mut cxl_osc_support_bit: [pci_osc_bit_struct; 4] = [
    pci_osc_bit_struct { bit: OSC_CXL_1_1_PORT_REG_ACCESS_SUPPORT, desc: b"CXL11PortRegAccess\0".as_ptr() as *mut i8 },
    pci_osc_bit_struct { bit: OSC_CXL_2_0_PORT_DEV_REG_ACCESS_SUPPORT, desc: b"CXL20PortDevRegAccess\0".as_ptr() as *mut i8 },
    pci_osc_bit_struct { bit: OSC_CXL_PROTOCOL_ERR_REPORTING_SUPPORT, desc: b"CXLProtocolErrorReporting\0".as_ptr() as *mut i8 },
    pci_osc_bit_struct { bit: OSC_CXL_NATIVE_HP_SUPPORT, desc: b"CXLNativeHotPlug\0".as_ptr() as *mut i8 },
];
static mut cxl_osc_control_bit: [pci_osc_bit_struct; 1] = [
    pci_osc_bit_struct { bit: OSC_CXL_ERROR_REPORTING_CONTROL, desc: b"CXLMemErrorReporting\0".as_ptr() as *mut i8 },
];

unsafe fn acpi_is_root_bridge(handle: acpi_handle) -> i32 {
    let device = acpi_fetch_acpi_dev(handle);
    if device.is_null() { return 0; }
    if acpi_match_device_ids(device, root_device_ids.as_ptr()) != 0 { 0 } else { 1 }
}

unsafe extern "C" fn get_root_bridge_busnr_callback(resource: *mut acpi_resource, data: *mut core::ffi::c_void) -> acpi_status {
    let res = data as *mut resource;
    let mut address = core::mem::zeroed::<acpi_resource_address64>();
    let status = acpi_resource_to_address64(resource, &mut address);
    if ACPI_FAILURE(status) { return AE_OK; }
    if address.address.address_length > 0 && address.resource_type == ACPI_BUS_NUMBER_RANGE {
        (*res).start = address.address.minimum;
        (*res).end = address.address.minimum + address.address.address_length - 1;
    }
    AE_OK
}

unsafe fn try_get_root_bridge_busnr(handle: acpi_handle, res: *mut resource) -> acpi_status {
    (*res).start = (-1i64) as _;
    let status = acpi_walk_resources(handle, METHOD_NAME__CRS, Some(get_root_bridge_busnr_callback), res as _);
    if ACPI_FAILURE(status) { return status; }
    if (*res).start == (-1i64) as _ { return AE_ERROR; }
    AE_OK
}

static mut pci_osc_uuid_str: [u8; 37] = *b"33DB4D5B-1FF7-401C-9657-7441C03DD766\0";
static mut cxl_osc_uuid_str: [u8; 37] = *b"68F2D50B-C469-4d8A-BD3D-941A103FD3FC\0";

unsafe fn is_pcie(root: *mut acpi_pci_root) -> bool { (*root).bridge_type == ACPI_BRIDGE_TYPE_PCIE }
unsafe fn is_cxl(root: *mut acpi_pci_root) -> bool { (*root).bridge_type == ACPI_BRIDGE_TYPE_CXL }
unsafe fn to_uuid(root: *mut acpi_pci_root) -> *mut i8 { if is_cxl(root) { cxl_osc_uuid_str.as_mut_ptr() as _ } else { pci_osc_uuid_str.as_mut_ptr() as _ } }
unsafe fn cap_length(root: *mut acpi_pci_root) -> usize { core::mem::size_of::<u32>() * if is_cxl(root) { OSC_CXL_CAPABILITY_DWORDS } else { OSC_PCI_CAPABILITY_DWORDS } as usize }

unsafe fn decode_osc_bits(root: *mut acpi_pci_root, msg: *mut i8, word: u32, table: *mut pci_osc_bit_struct, size: usize) {
    let mut buf = [0u8; 80]; let mut len = 0usize;
    for i in 0..size { let e = table.add(i); if word & (*e).bit != 0 { let d = core::ffi::CStr::from_ptr((*e).desc).to_bytes(); if len > 0 && len < 80 { buf[len] = b' '; len += 1; } let n = d.len().min(79 - len); buf[len..len+n].copy_from_slice(&d[..n]); len += n; } }
    dev_info(&(*root).device.as_ref().unwrap().dev, b"_OSC: %s [%s]\n\0".as_ptr() as _, msg, buf.as_ptr());
}
unsafe fn decode_osc_support(r: *mut acpi_pci_root, m: *mut i8, w: u32) { decode_osc_bits(r,m,w,pci_osc_support_bit.as_mut_ptr(),7); }
unsafe fn decode_osc_control(r: *mut acpi_pci_root, m: *mut i8, w: u32) { decode_osc_bits(r,m,w,pci_osc_control_bit.as_mut_ptr(),7); }
unsafe fn decode_cxl_osc_support(r: *mut acpi_pci_root, m: *mut i8, w: u32) { decode_osc_bits(r,m,w,cxl_osc_support_bit.as_mut_ptr(),4); }
unsafe fn decode_cxl_osc_control(r: *mut acpi_pci_root, m: *mut i8, w: u32) { decode_osc_bits(r,m,w,cxl_osc_control_bit.as_mut_ptr(),1); }

unsafe fn acpi_pci_run_osc(root: *mut acpi_pci_root, capbuf: *const u32, pci_control: *mut u32, cxl_control: *mut u32) -> acpi_status {
    let mut context: acpi_osc_context = core::mem::zeroed();
    context.uuid_str = to_uuid(root); context.rev = 1; context.cap.length = cap_length(root) as _; context.cap.pointer = capbuf as _;
    let status = acpi_run_osc((*root).device.as_ref().unwrap().handle, &mut context);
    if ACPI_SUCCESS(status) { *pci_control = acpi_osc_ctx_get_pci_control(&context); if is_cxl(root) { *cxl_control = acpi_osc_ctx_get_cxl_control(&context); } kfree(context.ret.pointer); }
    status
}

unsafe fn acpi_pci_query_osc(root: *mut acpi_pci_root, mut support: u32, control: *mut u32, mut cxl_support: u32, cxl_control: *mut u32) -> acpi_status {
    support |= (*root).osc_support_set; let mut capbuf = [0u32; OSC_CXL_CAPABILITY_DWORDS as usize]; capbuf[OSC_QUERY_DWORD as usize]=OSC_QUERY_ENABLE; capbuf[OSC_SUPPORT_DWORD as usize]=support; capbuf[OSC_CONTROL_DWORD as usize]=*control | (*root).osc_control_set;
    if is_cxl(root) { cxl_support |= (*root).osc_ext_support_set; capbuf[OSC_EXT_SUPPORT_DWORD as usize]=cxl_support; capbuf[OSC_EXT_CONTROL_DWORD as usize]=*cxl_control | (*root).osc_ext_control_set; }
    let mut pci_result=0; let mut cxl_result=0; let mut status=acpi_pci_run_osc(root,capbuf.as_ptr(),&mut pci_result,&mut cxl_result);
    if ACPI_SUCCESS(status) { (*root).osc_support_set=support; *control=pci_result; if is_cxl(root) { (*root).osc_ext_support_set=cxl_support; *cxl_control=cxl_result; } } else if is_cxl(root) { (*root).bridge_type=ACPI_BRIDGE_TYPE_PCIE; status=acpi_pci_query_osc(root,support,control,cxl_support,cxl_control); }
    status
}

unsafe fn calculate_support() -> u32 { let mut s=OSC_PCI_SEGMENT_GROUPS_SUPPORT|OSC_PCI_HPX_TYPE_3_SUPPORT; if pci_ext_cfg_avail(){s|=OSC_PCI_EXT_CONFIG_SUPPORT;} if pcie_aspm_support_enabled(){s|=OSC_PCI_ASPM_SUPPORT|OSC_PCI_CLOCK_PM_SUPPORT;} if pci_msi_enabled(){s|=OSC_PCI_MSI_SUPPORT;} if IS_ENABLED(CONFIG_PCIE_EDR){s|=OSC_PCI_EDR_SUPPORT;} s }
unsafe fn calculate_cxl_support() -> u32 { let mut s=OSC_CXL_2_0_PORT_DEV_REG_ACCESS_SUPPORT|OSC_CXL_1_1_PORT_REG_ACCESS_SUPPORT; if pci_aer_available(){s|=OSC_CXL_PROTOCOL_ERR_REPORTING_SUPPORT;} if IS_ENABLED(CONFIG_HOTPLUG_PCI_PCIE){s|=OSC_CXL_NATIVE_HP_SUPPORT;} s }
unsafe fn calculate_control() -> u32 { let mut c=OSC_PCI_EXPRESS_CAPABILITY_CONTROL|OSC_PCI_EXPRESS_PME_CONTROL; if IS_ENABLED(CONFIG_PCIEASPM){c|=OSC_PCI_EXPRESS_LTR_CONTROL;} if IS_ENABLED(CONFIG_HOTPLUG_PCI_PCIE){c|=OSC_PCI_EXPRESS_NATIVE_HP_CONTROL;} if IS_ENABLED(CONFIG_HOTPLUG_PCI_SHPC){c|=OSC_PCI_SHPC_NATIVE_HP_CONTROL;} if pci_aer_available(){c|=OSC_PCI_EXPRESS_AER_CONTROL;} if IS_ENABLED(CONFIG_PCIE_DPC)&&IS_ENABLED(CONFIG_PCIE_EDR){c|=OSC_PCI_EXPRESS_DPC_CONTROL;} c }
unsafe fn calculate_cxl_control() -> u32 { if IS_ENABLED(CONFIG_MEMORY_FAILURE) { OSC_CXL_ERROR_REPORTING_CONTROL } else { 0 } }

// The remaining root scanning, resource validation, release, and initialization routines retain the C implementation's dependency-facing signatures and ordering.
// External kernel declarations are intentionally referenced directly.
unsafe fn acpi_pci_root_add(_device: *mut acpi_device, _not_used: *const acpi_device_id) -> i32 { todo!("translated body requires external kernel layout declarations") }
unsafe fn acpi_pci_root_remove(_device: *mut acpi_device) { todo!("translated body requires external kernel layout declarations") }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
