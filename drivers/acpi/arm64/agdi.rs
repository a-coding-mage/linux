// SPDX-License-Identifier: GPL-2.0-only
/*
 * This file implements handling of
 * Arm Generic Diagnostic Dump and Reset Interface table (AGDI)
 *
 * Copyright (c) 2022, Ampere Computing LLC
 */

// C dependencies supplied by the surrounding kernel translation unit.

#[repr(C)]
pub struct agdi_data {
    pub flags: u8, /* AGDI Signaling Mode */
    pub sdei_event: i32,
    pub gsiv: u32,
    pub use_nmi: bool,
    pub irq: i32,
}

unsafe extern "C" {
    fn nmi_panic(regs: *mut pt_regs, msg: *const core::ffi::c_char);
    fn sdei_event_register(event: i32, handler: unsafe extern "C" fn(u32, *mut pt_regs, *mut core::ffi::c_void) -> i32, arg: *mut platform_device) -> i32;
    fn sdei_event_enable(event: i32) -> i32;
    fn sdei_event_unregister(event: i32) -> i32;
    fn sdei_event_disable(event: i32) -> i32;
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn acpi_register_gsi(a: *mut core::ffi::c_void, gsiv: u32, trigger: u32, polarity: u32) -> i32;
    fn acpi_unregister_gsi(gsiv: u32);
    fn request_nmi(irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t, flags: usize, name: *const core::ffi::c_char, dev_id: *mut core::ffi::c_void) -> i32;
    fn enable_nmi(irq: i32);
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t, flags: usize, name: *const core::ffi::c_char, dev_id: *mut core::ffi::c_void) -> i32;
    fn enable_irq(irq: i32);
    fn free_nmi(irq: i32, dev_id: *mut core::ffi::c_void);
    fn free_irq(irq: i32, dev_id: *mut core::ffi::c_void);
    fn panic(msg: *const core::ffi::c_char) -> !;
    fn schedule();
    fn ERR_PTR(err: i32) -> *mut core::ffi::c_void;
    fn dev_get_platdata(dev: *mut device) -> *mut agdi_data;
    fn platform_device_register_data(parent: *mut core::ffi::c_void, name: *const core::ffi::c_char, id: i32, data: *mut agdi_data, size: usize) -> *mut platform_device;
    fn platform_device_unregister(pdev: *mut platform_device);
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
    fn acpi_get_table(sig: u32, instance: u32, table: *mut *mut acpi_table_header) -> acpi_status;
    fn acpi_put_table(table: *mut acpi_table_header);
}

#[repr(C)] pub struct pt_regs { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct acpi_table_header { _private: [u8; 0] }
#[repr(C)] pub struct acpi_table_agdi { pub flags: u8, pub gsiv: u32, pub sdei_event: i32 }
#[repr(C)] pub struct platform_driver { _private: [u8; 0] }
pub type irqreturn_t = i32;
pub type acpi_status = i32;

unsafe extern "C" fn agdi_sdei_handler(sdei_event: u32, regs: *mut pt_regs, _arg: *mut core::ffi::c_void) -> i32 {
    nmi_panic(regs, b"Arm Generic Diagnostic Dump and Reset SDEI event issued\0".as_ptr() as _);
    0
}

unsafe fn agdi_sdei_probe(pdev: *mut platform_device, adata: *mut agdi_data) -> i32 {
    let mut err = sdei_event_register((*adata).sdei_event, agdi_sdei_handler, pdev);
    if err != 0 {
        dev_err(&mut (*pdev).dev, b"Failed to register for SDEI event %d\n\0".as_ptr() as _, (*adata).sdei_event);
        return err;
    }
    err = sdei_event_enable((*adata).sdei_event);
    if err != 0 {
        sdei_event_unregister((*adata).sdei_event);
        dev_err(&mut (*pdev).dev, b"Failed to enable event %d\n\0".as_ptr() as _, (*adata).sdei_event);
        return err;
    }
    0
}

unsafe extern "C" fn agdi_interrupt_handler_nmi(_irq: i32, _dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    nmi_panic(core::ptr::null_mut(), b"Arm Generic Diagnostic Dump and Reset NMI Interrupt event issued\n\0".as_ptr() as _);
    1
}

unsafe extern "C" fn agdi_interrupt_handler_irq(_irq: i32, _dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    panic(b"Arm Generic Diagnostic Dump and Reset Interrupt event issued\n\0".as_ptr() as _)
}

unsafe fn agdi_interrupt_probe(pdev: *mut platform_device, adata: *mut agdi_data) -> i32 {
    let irq = acpi_register_gsi(core::ptr::null_mut(), (*adata).gsiv, 1, 1);
    if irq < 0 { dev_err(&mut (*pdev).dev, b"cannot register GSI#%d (%d)\n\0".as_ptr() as _, (*adata).gsiv, irq); return irq; }
    let irq_flags: usize = (1 << 0) | (1 << 1) | (1 << 2) | (1 << 3);
    let ret = request_nmi(irq, agdi_interrupt_handler_nmi, irq_flags, b"agdi_interrupt_nmi\0".as_ptr() as _, core::ptr::null_mut());
    if ret == 0 { enable_nmi(irq); (*adata).irq = irq; (*adata).use_nmi = true; return 0; }
    let ret = request_irq(irq, agdi_interrupt_handler_irq, irq_flags, b"agdi_interrupt_irq\0".as_ptr() as _, core::ptr::null_mut());
    if ret != 0 { dev_err(&mut (*pdev).dev, b"cannot register IRQ %d\n\0".as_ptr() as _, ret); acpi_unregister_gsi((*adata).gsiv); return ret; }
    enable_irq(irq); (*adata).irq = irq; 0
}

unsafe fn agdi_probe(pdev: *mut platform_device) -> i32 {
    let adata = dev_get_platdata(&mut (*pdev).dev);
    if adata.is_null() { return -22; }
    if (*adata).flags & 1 != 0 { agdi_interrupt_probe(pdev, adata) } else { agdi_sdei_probe(pdev, adata) }
}

unsafe fn agdi_sdei_remove(pdev: *mut platform_device, adata: *mut agdi_data) {
    let err = sdei_event_disable((*adata).sdei_event);
    if err != 0 { dev_err(&mut (*pdev).dev, b"Failed to disable sdei-event #%d (%pe)\n\0".as_ptr() as _, (*adata).sdei_event, ERR_PTR(err)); return; }
    let mut err = 0;
    for _i in 0..3 { err = sdei_event_unregister((*adata).sdei_event); if err != -115 { break; } schedule(); }
    if err != 0 { dev_err(&mut (*pdev).dev, b"Failed to unregister sdei-event #%d (%pe)\n\0".as_ptr() as _, (*adata).sdei_event, ERR_PTR(err)); }
}

unsafe fn agdi_interrupt_remove(_pdev: *mut platform_device, adata: *mut agdi_data) {
    if (*adata).irq == -1 { return; }
    if (*adata).use_nmi { free_nmi((*adata).irq, core::ptr::null_mut()); } else { free_irq((*adata).irq, core::ptr::null_mut()); }
    acpi_unregister_gsi((*adata).gsiv);
}

unsafe fn agdi_remove(pdev: *mut platform_device) {
    let adata = dev_get_platdata(&mut (*pdev).dev);
    if (*adata).flags & 1 != 0 { agdi_interrupt_remove(pdev, adata); } else { agdi_sdei_remove(pdev, adata); }
}

static mut agdi_driver: platform_driver = platform_driver { _private: [] };

pub unsafe fn acpi_agdi_init() {
    let mut agdi_table: *mut acpi_table_agdi = core::ptr::null_mut();
    let mut pdata = agdi_data { flags: 0, sdei_event: 0, gsiv: 0, use_nmi: false, irq: 0 };
    let status = acpi_get_table(0, 0, &mut agdi_table as *mut _ as *mut *mut acpi_table_header);
    if status != 0 { return; }
    if (*agdi_table).flags & 1 != 0 { pdata.gsiv = (*agdi_table).gsiv; } else { pdata.sdei_event = (*agdi_table).sdei_event; }
    pdata.irq = -1; pdata.flags = (*agdi_table).flags;
    let pdev = platform_device_register_data(core::ptr::null_mut(), b"agdi\0".as_ptr() as _, 0, &mut pdata, core::mem::size_of::<agdi_data>());
    if !pdev.is_null() && platform_driver_register(&mut agdi_driver) != 0 { platform_device_unregister(pdev); }
    acpi_put_table(agdi_table as *mut acpi_table_header);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
