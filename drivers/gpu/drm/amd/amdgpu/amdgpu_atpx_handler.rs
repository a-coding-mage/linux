// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2010 Red Hat Inc.
 * Author : Dave Airlie <airlied@redhat.com>
 *
 * ATPX support for both Intel/ATI
 */

// External Linux, ACPI, PCI, VGA switcheroo, and amdgpu symbols are supplied
// by the surrounding kernel translation unit.

const AMDGPU_PX_QUIRK_FORCE_ATPX: u32 = 1 << 0;

#[repr(C)]
struct amdgpu_px_quirk {
    chip_vendor: u32,
    chip_device: u32,
    subsys_vendor: u32,
    subsys_device: u32,
    px_quirk_flags: u32,
}

#[repr(C)]
struct amdgpu_atpx_functions {
    px_params: bool,
    power_cntl: bool,
    disp_mux_cntl: bool,
    i2c_mux_cntl: bool,
    switch_start: bool,
    switch_end: bool,
    disp_connectors_mapping: bool,
    disp_detection_ports: bool,
}

#[repr(C)]
struct amdgpu_atpx {
    handle: acpi_handle,
    functions: amdgpu_atpx_functions,
    is_hybrid: bool,
    dgpu_req_power_for_displays: bool,
}

#[repr(C)]
struct amdgpu_atpx_priv {
    atpx_detected: bool,
    bridge_pm_usable: bool,
    quirks: c_uint,
    dhandle: acpi_handle,
    other_handle: acpi_handle,
    atpx: amdgpu_atpx,
}

static mut amdgpu_atpx_priv: amdgpu_atpx_priv = amdgpu_atpx_priv {
    atpx_detected: false,
    bridge_pm_usable: false,
    quirks: 0,
    dhandle: core::ptr::null_mut(),
    other_handle: core::ptr::null_mut(),
    atpx: amdgpu_atpx {
        handle: core::ptr::null_mut(),
        functions: amdgpu_atpx_functions {
            px_params: false,
            power_cntl: false,
            disp_mux_cntl: false,
            i2c_mux_cntl: false,
            switch_start: false,
            switch_end: false,
            disp_connectors_mapping: false,
            disp_detection_ports: false,
        },
        is_hybrid: false,
        dgpu_req_power_for_displays: false,
    },
};

#[repr(C, packed)]
struct atpx_verify_interface { size: u16, version: u16, function_bits: u32 }
#[repr(C, packed)]
struct atpx_px_params { size: u16, valid_flags: u32, flags: u32 }
#[repr(C, packed)]
struct atpx_power_control { size: u16, dgpu_state: u8 }
#[repr(C, packed)]
struct atpx_mux { size: u16, mux: u16 }

unsafe fn amdgpu_has_atpx() -> bool { amdgpu_atpx_priv.atpx_detected }
unsafe fn amdgpu_has_atpx_dgpu_power_cntl() -> bool { amdgpu_atpx_priv.atpx.functions.power_cntl }
unsafe fn amdgpu_is_atpx_hybrid() -> bool { amdgpu_atpx_priv.atpx.is_hybrid }

unsafe fn amdgpu_atpx_buffer_validate(obj: *const acpi_object, min_size: usize) -> bool {
    !obj.is_null() && (*obj).type_ == ACPI_TYPE_BUFFER && (*obj).buffer.length >= core::mem::size_of::<u16>() &&
        (*obj).buffer.length >= *( (*obj).buffer.pointer as *const u16) as usize &&
        *( (*obj).buffer.pointer as *const u16) as usize >= min_size
}

unsafe fn amdgpu_atpx_call(handle: acpi_handle, function: c_int, params: *mut acpi_buffer) -> *mut acpi_object {
    let mut args: [acpi_object; 2] = core::mem::zeroed();
    let mut arg = acpi_object_list { count: 2, pointer: args.as_mut_ptr() };
    let mut buffer = acpi_buffer { length: ACPI_ALLOCATE_BUFFER, pointer: core::ptr::null_mut() };
    args[0].type_ = ACPI_TYPE_INTEGER; args[0].integer.value = function as u64;
    if !params.is_null() {
        args[1].type_ = ACPI_TYPE_BUFFER; args[1].buffer.length = (*params).length; args[1].buffer.pointer = (*params).pointer;
    } else { args[1].type_ = ACPI_TYPE_INTEGER; args[1].integer.value = 0; }
    let status = acpi_evaluate_object(handle, core::ptr::null(), &mut arg, &mut buffer);
    if ACPI_FAILURE(status) && status != AE_NOT_FOUND { pr_err!("failed to evaluate ATPX got %s\n", acpi_format_exception(status)); kfree(buffer.pointer); return core::ptr::null_mut(); }
    buffer.pointer as *mut acpi_object
}

unsafe fn amdgpu_atpx_parse_functions(f: *mut amdgpu_atpx_functions, mask: u32) {
    (*f).px_params = mask & ATPX_GET_PX_PARAMETERS_SUPPORTED != 0;
    (*f).power_cntl = mask & ATPX_POWER_CONTROL_SUPPORTED != 0;
    (*f).disp_mux_cntl = mask & ATPX_DISPLAY_MUX_CONTROL_SUPPORTED != 0;
    (*f).i2c_mux_cntl = mask & ATPX_I2C_MUX_CONTROL_SUPPORTED != 0;
    (*f).switch_start = mask & ATPX_GRAPHICS_DEVICE_SWITCH_START_NOTIFICATION_SUPPORTED != 0;
    (*f).switch_end = mask & ATPX_GRAPHICS_DEVICE_SWITCH_END_NOTIFICATION_SUPPORTED != 0;
    (*f).disp_connectors_mapping = mask & ATPX_GET_DISPLAY_CONNECTORS_MAPPING_SUPPORTED != 0;
    (*f).disp_detection_ports = mask & ATPX_GET_DISPLAY_DETECTION_PORTS_SUPPORTED != 0;
}

unsafe fn amdgpu_atpx_validate(atpx: *mut amdgpu_atpx) -> c_int {
    let mut valid_bits = 0u32;
    if (*atpx).functions.px_params {
        let info = amdgpu_atpx_call((*atpx).handle, ATPX_FUNCTION_GET_PX_PARAMETERS, core::ptr::null_mut());
        if info.is_null() { return -EIO; }
        let output_size = core::mem::size_of::<atpx_px_params>();
        if !amdgpu_atpx_buffer_validate(info, output_size) { pr_err!("Invalid ATPX GET_PX_PARAMETERS response\n"); kfree(info as *mut _); return -EINVAL; }
        let mut output: atpx_px_params = core::mem::zeroed();
        let size = core::cmp::min(output_size, *( (*info).buffer.pointer as *const u16) as usize);
        core::ptr::copy_nonoverlapping((*info).buffer.pointer, &mut output as *mut _ as *mut _, size);
        valid_bits = output.flags & output.valid_flags; kfree(info as *mut _);
    }
    if valid_bits & ATPX_SEPARATE_MUX_FOR_I2C != 0 { (*atpx).functions.i2c_mux_cntl = true; (*atpx).functions.disp_mux_cntl = true; }
    if valid_bits & (ATPX_CRT1_RGB_SIGNAL_MUXED | ATPX_TV_SIGNAL_MUXED | ATPX_DFP_SIGNAL_MUXED) != 0 { (*atpx).functions.disp_mux_cntl = true; }
    if valid_bits & (ATPX_DYNAMIC_PX_SUPPORTED | ATPX_DYNAMIC_DGPU_POWER_OFF_SUPPORTED) != 0 { (*atpx).functions.power_cntl = true; }
    (*atpx).is_hybrid = false;
    if valid_bits & ATPX_MS_HYBRID_GFX_SUPPORTED != 0 {
        if amdgpu_atpx_priv.quirks & AMDGPU_PX_QUIRK_FORCE_ATPX != 0 { pr_warn!("ATPX Hybrid Graphics, forcing to ATPX\n"); (*atpx).functions.power_cntl = true; }
        else { pr_notice!("ATPX Hybrid Graphics\n"); (*atpx).functions.power_cntl = !amdgpu_atpx_priv.bridge_pm_usable; (*atpx).is_hybrid = true; }
    }
    (*atpx).dgpu_req_power_for_displays = valid_bits & ATPX_DGPU_REQ_POWER_FOR_DISPLAYS != 0;
    0
}

unsafe fn amdgpu_atpx_verify_interface(atpx: *mut amdgpu_atpx) -> c_int {
    let info = amdgpu_atpx_call((*atpx).handle, ATPX_FUNCTION_VERIFY_INTERFACE, core::ptr::null_mut());
    if info.is_null() { return -EIO; }
    let mut err = 0;
    let output_size = core::mem::size_of::<atpx_verify_interface>();
    if !amdgpu_atpx_buffer_validate(info, output_size) { pr_err!("Invalid ATPX VERIFY_INTERFACE response\n"); err = -EINVAL; }
    else { let mut output: atpx_verify_interface = core::mem::zeroed(); let size = core::cmp::min(output_size, *( (*info).buffer.pointer as *const u16) as usize); core::ptr::copy_nonoverlapping((*info).buffer.pointer, &mut output as *mut _ as *mut _, size); pr_notice!("ATPX version %u, functions 0x%08x\n", output.version, output.function_bits); amdgpu_atpx_parse_functions(&mut (*atpx).functions, output.function_bits); }
    kfree(info as *mut _); err
}

unsafe fn amdgpu_atpx_set_discrete_state(atpx: *mut amdgpu_atpx, state: u8) -> c_int {
    if (*atpx).functions.power_cntl { let mut input = atpx_power_control { size: 3, dgpu_state: state }; let mut params = acpi_buffer { length: input.size as usize, pointer: &mut input as *mut _ as *mut _ }; let info = amdgpu_atpx_call((*atpx).handle, ATPX_FUNCTION_POWER_CONTROL, &mut params); if info.is_null() { return -EIO; } kfree(info as *mut _); if state == 0 { msleep(200); } } 0
}

unsafe fn amdgpu_atpx_switch_disp_mux(atpx: *mut amdgpu_atpx, mux_id: u16) -> c_int { amdgpu_atpx_switch_mux(atpx, mux_id, ATPX_DISPLAY_MUX_CONTROL, (*atpx).functions.disp_mux_cntl) }
unsafe fn amdgpu_atpx_switch_i2c_mux(atpx: *mut amdgpu_atpx, mux_id: u16) -> c_int { amdgpu_atpx_switch_mux(atpx, mux_id, ATPX_I2C_MUX_CONTROL, (*atpx).functions.i2c_mux_cntl) }
unsafe fn amdgpu_atpx_switch_start(atpx: *mut amdgpu_atpx, mux_id: u16) -> c_int { amdgpu_atpx_switch_mux(atpx, mux_id, ATPX_GRAPHICS_DEVICE_SWITCH_START_NOTIFICATION, (*atpx).functions.switch_start) }
unsafe fn amdgpu_atpx_switch_end(atpx: *mut amdgpu_atpx, mux_id: u16) -> c_int { amdgpu_atpx_switch_mux(atpx, mux_id, ATPX_GRAPHICS_DEVICE_SWITCH_END_NOTIFICATION, (*atpx).functions.switch_end) }

unsafe fn amdgpu_atpx_switch_mux(atpx: *mut amdgpu_atpx, mux_id: u16, function: c_int, enabled: bool) -> c_int {
    if enabled { let mut input = atpx_mux { size: 4, mux: mux_id }; let mut params = acpi_buffer { length: input.size as usize, pointer: &mut input as *mut _ as *mut _ }; let info = amdgpu_atpx_call((*atpx).handle, function, &mut params); if info.is_null() { return -EIO; } kfree(info as *mut _); } 0
}

unsafe fn amdgpu_atpx_switchto(id: vga_switcheroo_client_id) -> c_int { let gpu_id = if id == VGA_SWITCHEROO_IGD { ATPX_INTEGRATED_GPU } else { ATPX_DISCRETE_GPU }; amdgpu_atpx_switch_start(&mut amdgpu_atpx_priv.atpx, gpu_id); amdgpu_atpx_switch_disp_mux(&mut amdgpu_atpx_priv.atpx, gpu_id); amdgpu_atpx_switch_i2c_mux(&mut amdgpu_atpx_priv.atpx, gpu_id); amdgpu_atpx_switch_end(&mut amdgpu_atpx_priv.atpx, gpu_id); 0 }
unsafe fn amdgpu_atpx_power_state(id: vga_switcheroo_client_id, state: vga_switcheroo_state) -> c_int { if id != VGA_SWITCHEROO_IGD { amdgpu_atpx_set_discrete_state(&mut amdgpu_atpx_priv.atpx, state as u8); } 0 }

unsafe fn amdgpu_atpx_pci_probe_handle(pdev: *mut pci_dev) -> bool { let dhandle = ACPI_HANDLE!(&mut (*pdev).dev); if dhandle.is_null() { return false; } let mut atpx_handle = core::ptr::null_mut(); let status = acpi_get_handle(dhandle, b"ATPX\0".as_ptr() as *const _, &mut atpx_handle); if ACPI_FAILURE(status) { amdgpu_atpx_priv.other_handle = dhandle; return false; } amdgpu_atpx_priv.dhandle = dhandle; amdgpu_atpx_priv.atpx.handle = atpx_handle; true }
unsafe fn amdgpu_atpx_init() -> c_int { let r = amdgpu_atpx_verify_interface(&mut amdgpu_atpx_priv.atpx); if r != 0 { return r; } amdgpu_atpx_validate(&mut amdgpu_atpx_priv.atpx) }
unsafe fn amdgpu_atpx_get_client_id(pdev: *mut pci_dev) -> vga_switcheroo_client_id { if amdgpu_atpx_priv.dhandle == ACPI_HANDLE!(&mut (*pdev).dev) { VGA_SWITCHEROO_IGD } else { VGA_SWITCHEROO_DIS } }

static amdgpu_atpx_handler: vga_switcheroo_handler = vga_switcheroo_handler { switchto: Some(amdgpu_atpx_switchto), power_state: Some(amdgpu_atpx_power_state), get_client_id: Some(amdgpu_atpx_get_client_id) };

static amdgpu_px_quirk_list: &[amdgpu_px_quirk] = &[
    amdgpu_px_quirk { chip_vendor: 0x1002, chip_device: 0x6900, subsys_vendor: 0x1002, subsys_device: 0x0124, px_quirk_flags: AMDGPU_PX_QUIRK_FORCE_ATPX },
    amdgpu_px_quirk { chip_vendor: 0x1002, chip_device: 0x6900, subsys_vendor: 0x1028, subsys_device: 0x0812, px_quirk_flags: AMDGPU_PX_QUIRK_FORCE_ATPX },
    amdgpu_px_quirk { chip_vendor: 0x1002, chip_device: 0x6900, subsys_vendor: 0x1028, subsys_device: 0x0813, px_quirk_flags: AMDGPU_PX_QUIRK_FORCE_ATPX },
    amdgpu_px_quirk { chip_vendor: 0x1002, chip_device: 0x699f, subsys_vendor: 0x1028, subsys_device: 0x0814, px_quirk_flags: AMDGPU_PX_QUIRK_FORCE_ATPX },
    amdgpu_px_quirk { chip_vendor: 0x1002, chip_device: 0x6900, subsys_vendor: 0x1025, subsys_device: 0x125A, px_quirk_flags: AMDGPU_PX_QUIRK_FORCE_ATPX },
    amdgpu_px_quirk { chip_vendor: 0x1002, chip_device: 0x6900, subsys_vendor: 0x17AA, subsys_device: 0x3806, px_quirk_flags: AMDGPU_PX_QUIRK_FORCE_ATPX },
    amdgpu_px_quirk { chip_vendor: 0, chip_device: 0, subsys_vendor: 0, subsys_device: 0, px_quirk_flags: 0 },
];

unsafe fn amdgpu_atpx_get_quirks(pdev: *mut pci_dev) { for p in amdgpu_px_quirk_list { if p.chip_device == 0 { break; } if (*pdev).vendor == p.chip_vendor && (*pdev).device == p.chip_device && (*pdev).subsystem_vendor == p.subsys_vendor && (*pdev).subsystem_device == p.subsys_device { amdgpu_atpx_priv.quirks |= p.px_quirk_flags; break; } } }

unsafe fn amdgpu_atpx_detect() -> bool {
    let mut acpi_method_name = [0i8; 255]; let mut buffer = acpi_buffer { length: acpi_method_name.len(), pointer: acpi_method_name.as_mut_ptr() as *mut _ }; let mut pdev = core::ptr::null_mut(); let mut has_atpx = false; let mut vga_count = 0; let mut d3_supported = false;
    while { pdev = pci_get_class(PCI_CLASS_DISPLAY_VGA << 8, pdev); !pdev.is_null() } { vga_count += 1; has_atpx |= amdgpu_atpx_pci_probe_handle(pdev); let parent = pci_upstream_bridge(pdev); d3_supported |= !parent.is_null() && (*parent).bridge_d3; amdgpu_atpx_get_quirks(pdev); }
    while { pdev = pci_get_class(PCI_CLASS_DISPLAY_OTHER << 8, pdev); !pdev.is_null() } { vga_count += 1; has_atpx |= amdgpu_atpx_pci_probe_handle(pdev); let parent = pci_upstream_bridge(pdev); d3_supported |= !parent.is_null() && (*parent).bridge_d3; amdgpu_atpx_get_quirks(pdev); }
    if has_atpx && vga_count == 2 { acpi_get_name(amdgpu_atpx_priv.atpx.handle, ACPI_FULL_PATHNAME, &mut buffer); pr_info!("vga_switcheroo: detected switching method %s handle\n", acpi_method_name.as_ptr()); amdgpu_atpx_priv.atpx_detected = true; amdgpu_atpx_priv.bridge_pm_usable = d3_supported; amdgpu_atpx_init(); return true; } false
}

pub unsafe fn amdgpu_register_atpx_handler() { if !amdgpu_atpx_detect() { return; } vga_switcheroo_register_handler(&amdgpu_atpx_handler, 0); }
pub unsafe fn amdgpu_unregister_atpx_handler() { vga_switcheroo_unregister_handler(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
