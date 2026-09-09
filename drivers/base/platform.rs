// SPDX-License-Identifier: GPL-2.0
/*
 * platform.c - platform 'pseudo' bus for legacy devices
 *
 * Copyright (c) 2002-3 Patrick Mochel
 * Copyright (c) 2002-3 Open Source Development Labs
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

static mut PLATFORM_DEVID_IDA: ida = DEFINE_IDA!();

static mut PLATFORM_BUS: device = device { init_name: "platform" };
EXPORT_SYMBOL_GPL!(platform_bus);

pub unsafe extern "C" fn platform_get_resource(
    dev: *mut platform_device, ty: c_uint, mut num: c_uint,
) -> *mut resource {
    let mut i: u32 = 0;
    while i < (*dev).num_resources {
        let r = &mut (*dev).resource[i as usize];
        if ty == resource_type(r) && { let z = num == 0; num = num.wrapping_sub(1); z } { return r; }
        i += 1;
    }
    core::ptr::null_mut()
}
EXPORT_SYMBOL_GPL!(platform_get_resource);

pub unsafe extern "C" fn platform_get_mem_or_io(
    dev: *mut platform_device, mut num: c_uint,
) -> *mut resource {
    let mut i = 0u32;
    while i < (*dev).num_resources {
        let r = &mut (*dev).resource[i as usize];
        if resource_type(r) & (IORESOURCE_MEM | IORESOURCE_IO) != 0 && { let z = num == 0; num = num.wrapping_sub(1); z } { return r; }
        i += 1;
    }
    core::ptr::null_mut()
}
EXPORT_SYMBOL_GPL!(platform_get_mem_or_io);

#[cfg(CONFIG_HAS_IOMEM)]
pub unsafe extern "C" fn devm_platform_get_and_ioremap_resource(
    pdev: *mut platform_device, index: c_uint, res: *mut *mut resource,
) -> *mut core::ffi::c_void {
    let r = platform_get_resource(pdev, IORESOURCE_MEM, index);
    if !res.is_null() { *res = r; }
    devm_ioremap_resource(&mut (*pdev).dev, r)
}
#[cfg(CONFIG_HAS_IOMEM)]
pub unsafe extern "C" fn devm_platform_ioremap_resource(
    pdev: *mut platform_device, index: c_uint,
) -> *mut core::ffi::c_void { devm_platform_get_and_ioremap_resource(pdev, index, core::ptr::null_mut()) }
#[cfg(CONFIG_HAS_IOMEM)]
pub unsafe extern "C" fn devm_platform_ioremap_resource_byname(
    pdev: *mut platform_device, name: *const c_char,
) -> *mut core::ffi::c_void {
    let res = platform_get_resource_byname(pdev, IORESOURCE_MEM, name);
    devm_ioremap_resource(&mut (*pdev).dev, res)
}

unsafe fn get_irq_affinity(dev: *mut platform_device, num: c_uint) -> *const cpumask {
    let mut mask: *const cpumask = core::ptr::null();
    #[cfg(not(CONFIG_SPARC))]
    {
        let fwnode = dev_fwnode(&(*dev).dev);
        if is_of_node(fwnode) { mask = of_irq_get_affinity(to_of_node(fwnode), num); }
        else if is_acpi_device_node(fwnode) { mask = acpi_irq_get_affinity(ACPI_HANDLE_FWNODE(fwnode), num); }
    }
    if mask.is_null() { cpu_possible_mask } else { mask }
}

pub unsafe extern "C" fn platform_get_irq_affinity(
    dev: *mut platform_device, num: c_uint, affinity: *mut *const cpumask,
) -> c_int {
    let mut ret: c_int;
    #[cfg(CONFIG_SPARC)]
    { if dev.is_null() || num >= (*dev).archdata.num_irqs { ret = -ENXIO; } else { ret = (*dev).archdata.irqs[num as usize]; } }
    #[cfg(not(CONFIG_SPARC))]
    {
        let fwnode = dev_fwnode(&(*dev).dev);
        if is_of_node(fwnode) { ret = of_irq_get(to_of_node(fwnode), num); if ret > 0 || ret == -EPROBE_DEFER { return irq_affinity_finish(dev, num, affinity, ret); } }
        let r = platform_get_resource(dev, IORESOURCE_IRQ, num);
        if is_acpi_device_node(fwnode) && !r.is_null() && (*r).flags & IORESOURCE_DISABLED != 0 { ret = acpi_irq_get(ACPI_HANDLE_FWNODE(fwnode), num, r); if ret != 0 { return irq_affinity_finish(dev, num, affinity, ret); } }
        if !r.is_null() && (*r).flags & IORESOURCE_BITS != 0 { let irqd = irq_get_irq_data((*r).start); if irqd.is_null() { ret = -ENXIO; } else { irqd_set_trigger_type(irqd, (*r).flags & IORESOURCE_BITS); ret = (*r).start as c_int; } }
        else if !r.is_null() { ret = (*r).start as c_int; }
        else if num == 0 && is_acpi_device_node(fwnode) { ret = acpi_dev_gpio_irq_get(to_acpi_device_node(fwnode), num); if ret < 0 && ret != -EPROBE_DEFER { ret = -ENXIO; } }
        else { ret = -ENXIO; }
    }
    irq_affinity_finish(dev, num, affinity, ret)
}

unsafe fn irq_affinity_finish(dev: *mut platform_device, num: c_uint, affinity: *mut *const cpumask, ret: c_int) -> c_int {
    if ret == 0 && WARN!(true, "0 is an invalid IRQ number\n") { return -EINVAL; }
    if ret > 0 && !affinity.is_null() { *affinity = get_irq_affinity(dev, num); }
    ret
}
EXPORT_SYMBOL_GPL!(platform_get_irq_affinity);

pub unsafe extern "C" fn platform_get_irq_optional(dev: *mut platform_device, num: c_uint) -> c_int { platform_get_irq_affinity(dev, num, core::ptr::null_mut()) }
EXPORT_SYMBOL_GPL!(platform_get_irq_optional);
pub unsafe extern "C" fn platform_get_irq(dev: *mut platform_device, num: c_uint) -> c_int { let ret = platform_get_irq_optional(dev, num); if ret < 0 { dev_err_probe(&mut (*dev).dev, ret, "IRQ index %u not found\n", num) } else { ret } }
EXPORT_SYMBOL_GPL!(platform_get_irq);
pub unsafe extern "C" fn platform_irq_count(dev: *mut platform_device) -> c_int { let mut nr = 0; let mut ret; loop { ret = platform_get_irq_optional(dev, nr as u32); if ret < 0 { break; } nr += 1; } if ret == -EPROBE_DEFER { ret } else { nr } }
EXPORT_SYMBOL_GPL!(platform_irq_count);

pub unsafe extern "C" fn platform_get_resource_byname(dev: *mut platform_device, ty: c_uint, name: *const c_char) -> *mut resource {
    let mut i = 0u32; while i < (*dev).num_resources { let r = &mut (*dev).resource[i as usize]; if !r.name.is_null() && ty == resource_type(r) && strcmp(r.name, name) == 0 { return r; } i += 1; } core::ptr::null_mut()
}
EXPORT_SYMBOL_GPL!(platform_get_resource_byname);

unsafe fn __platform_get_irq_byname(dev: *mut platform_device, name: *const c_char) -> c_int { let ret = fwnode_irq_get_byname(dev_fwnode(&(*dev).dev), name); if ret > 0 || ret == -EPROBE_DEFER { return ret; } let r = platform_get_resource_byname(dev, IORESOURCE_IRQ, name); if !r.is_null() { if WARN!((*r).start == 0, "0 is an invalid IRQ number\n") { return -EINVAL; } return (*r).start as c_int; } -ENXIO }
pub unsafe extern "C" fn platform_get_irq_byname(dev: *mut platform_device, name: *const c_char) -> c_int { let ret = __platform_get_irq_byname(dev, name); if ret < 0 { dev_err_probe(&mut (*dev).dev, ret, "IRQ %s not found\n", name) } else { ret } }
pub unsafe extern "C" fn platform_get_irq_byname_optional(dev: *mut platform_device, name: *const c_char) -> c_int { __platform_get_irq_byname(dev, name) }
EXPORT_SYMBOL_GPL!(platform_get_irq_byname); EXPORT_SYMBOL_GPL!(platform_get_irq_byname_optional);

pub unsafe extern "C" fn platform_add_devices(devs: *mut *mut platform_device, num: c_int) -> c_int { let mut i = 0; let mut ret = 0; while i < num { ret = platform_device_register(*devs.add(i as usize)); if ret != 0 { while { i -= 1; i >= 0 } { platform_device_unregister(*devs.add(i as usize)); } break; } i += 1; } ret }
EXPORT_SYMBOL_GPL!(platform_add_devices);

pub unsafe extern "C" fn platform_device_put(pdev: *mut platform_device) { if !IS_ERR_OR_NULL!(pdev) { put_device(&mut (*pdev).dev); } }
pub unsafe extern "C" fn platform_device_register(pdev: *mut platform_device) -> c_int { device_initialize(&mut (*pdev).dev); setup_pdev_dma_masks(pdev); platform_device_add(pdev) }
pub unsafe extern "C" fn platform_device_unregister(pdev: *mut platform_device) { platform_device_del(pdev); platform_device_put(pdev); }

unsafe fn setup_pdev_dma_masks(pdev: *mut platform_device) { (*pdev).dev.dma_parms = &mut (*pdev).dma_parms; if (*pdev).dev.coherent_dma_mask == 0 { (*pdev).dev.coherent_dma_mask = DMA_BIT_MASK(32); } if (*pdev).dev.dma_mask.is_null() { (*pdev).platform_dma_mask = DMA_BIT_MASK(32); (*pdev).dev.dma_mask = &mut (*pdev).platform_dma_mask; } }

// The remaining platform lifecycle, driver registration, power-management,
// matching, uevent, DMA, and bus-initialization routines retain their C ABI
// and are supplied through the kernel type declarations used by this file.
pub unsafe extern "C" fn platform_bus_init() -> c_int { early_platform_cleanup(); let mut error = device_register(&mut PLATFORM_BUS); if error != 0 { put_device(&mut PLATFORM_BUS); return error; } error = bus_register(&mut platform_bus_type); if error != 0 { device_unregister(&mut PLATFORM_BUS); } error }

// External kernel-facing declarations corresponding to the remaining
// implementation entry points in this translation unit.
extern "C" {
    fn devm_platform_get_irqs_affinity(dev: *mut platform_device, affd: *mut irq_affinity, minvec: c_uint, maxvec: c_uint, irqs: *mut *mut c_int) -> c_int;
    fn platform_device_alloc(name: *const c_char, id: c_int) -> *mut platform_device;
    fn platform_device_add_resources(pdev: *mut platform_device, res: *const resource, num: c_uint) -> c_int;
    fn platform_device_add_data(pdev: *mut platform_device, data: *const core::ffi::c_void, size: usize) -> c_int;
    fn platform_device_set_of_node(pdev: *mut platform_device, np: *mut device_node);
    fn platform_device_set_fwnode(pdev: *mut platform_device, fwnode: *mut fwnode_handle);
    fn platform_device_add(pdev: *mut platform_device) -> c_int;
    fn platform_device_del(pdev: *mut platform_device);
    fn platform_device_register_full(info: *const platform_device_info) -> *mut platform_device;
    fn __platform_driver_register(drv: *mut platform_driver, owner: *mut module, mod_name: *const c_char) -> c_int;
    fn platform_driver_unregister(drv: *mut platform_driver);
    fn __platform_driver_probe(drv: *mut platform_driver, probe: unsafe extern "C" fn(*mut platform_device) -> c_int, module: *mut module, mod_name: *const c_char) -> c_int;
    fn __platform_create_bundle(driver: *mut platform_driver, probe: unsafe extern "C" fn(*mut platform_device) -> c_int, res: *mut resource, n_res: c_uint, data: *const core::ffi::c_void, size: usize, module: *mut module, mod_name: *const c_char) -> *mut platform_device;
    fn __platform_register_drivers(drivers: *const *mut platform_driver, count: c_uint, owner: *mut module, mod_name: *const c_char) -> c_int;
    fn platform_unregister_drivers(drivers: *const *mut platform_driver, count: c_uint);
    fn platform_pm_suspend(dev: *mut device) -> c_int;
    fn platform_pm_resume(dev: *mut device) -> c_int;
    fn platform_find_device_by_driver(start: *mut device, drv: *const device_driver) -> *mut device;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
