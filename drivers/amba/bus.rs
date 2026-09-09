// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/common/amba.c
 *
 *  Copyright (C) 2003 Deep Blue Solutions Ltd, All Rights Reserved.
 */
// Linux kernel includes are supplied by the surrounding translation unit.

/* called on periphid match and class 0x9 coresight device. */
unsafe fn amba_cs_uci_id_match(table: *const amba_id, dev: *mut amba_device) -> c_int {
    let mut ret: c_int = 0;
    let uci: *mut amba_cs_uci_id = (*table).data as *mut amba_cs_uci_id;

    /* no table data or zero mask - return match on periphid */
    if uci.is_null() || (*uci).devarch_mask == 0 {
        return 1;
    }

    /* test against read devtype and masked devarch value */
    ret = ((*dev).uci.devtype == (*uci).devtype
        && ((*dev).uci.devarch & (*uci).devarch_mask) == (*uci).devarch) as c_int;
    ret
}

unsafe fn amba_lookup(mut table: *const amba_id, dev: *mut amba_device) -> *const amba_id {
    while (*table).mask != 0 {
        if (((*dev).periphid & (*table).mask) == (*table).id)
            && ((*dev).cid != CORESIGHT_CID || amba_cs_uci_id_match(table, dev) != 0)
        {
            return table;
        }
        table = table.add(1);
    }
    core::ptr::null()
}

unsafe fn amba_get_enable_pclk(pcdev: *mut amba_device) -> c_int {
    let mut ret: c_int;
    (*pcdev).pclk = clk_get(&mut (*pcdev).dev, b"apb_pclk\0".as_ptr() as *const c_char);
    if IS_ERR((*pcdev).pclk) {
        return PTR_ERR((*pcdev).pclk);
    }
    ret = clk_prepare_enable((*pcdev).pclk);
    if ret != 0 {
        clk_put((*pcdev).pclk);
    }
    ret
}

unsafe fn amba_put_disable_pclk(pcdev: *mut amba_device) {
    clk_disable_unprepare((*pcdev).pclk);
    clk_put((*pcdev).pclk);
}

unsafe fn amba_id_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> isize {
    sprintf(buf, b"%08x\n\0".as_ptr() as *const c_char, (*(to_amba_device(dev))).periphid)
}

unsafe fn amba_resource_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> isize {
    let d = to_amba_device(dev);
    sprintf(buf, b"\t%016llx\t%016llx\t%016lx\n\0".as_ptr() as *const c_char,
        (*d).res.start as c_ulonglong, (*d).res.end as c_ulonglong, (*d).res.flags as c_ulong)
}

/* Attribute groups, device-model declarations, and macro-generated metadata. */
static mut AMBA_DEV_ATTRS: [*mut attribute; 3] = [
    &mut DEV_ATTR_ID_ATTR,
    &mut DEV_ATTR_RESOURCE_ATTR,
    core::ptr::null_mut(),
];

unsafe fn amba_read_periphid(dev: *mut amba_device) -> c_int {
    let mut rstc: *mut reset_control;
    let size: u32;
    let mut pid: u32;
    let mut cid: u32;
    let tmp: *mut c_void;
    let mut i: c_int;
    let mut ret = dev_pm_domain_attach(&mut (*dev).dev, PD_FLAG_ATTACH_POWER_ON);
    if ret != 0 { dev_dbg(&mut (*dev).dev, b"can't get PM domain: %d\n\0".as_ptr() as *const c_char, ret); return ret; }
    ret = amba_get_enable_pclk(dev);
    if ret != 0 { dev_dbg(&mut (*dev).dev, b"can't get pclk: %d\n\0".as_ptr() as *const c_char, ret); dev_pm_domain_detach(&mut (*dev).dev, true); return ret; }
    rstc = of_reset_control_array_get_optional_shared((*dev).dev.of_node);
    if IS_ERR(rstc) { ret = PTR_ERR(rstc); if ret != -EPROBE_DEFER { dev_err(&mut (*dev).dev, b"can't get reset: %d\n\0".as_ptr() as *const c_char, ret); } amba_put_disable_pclk(dev); dev_pm_domain_detach(&mut (*dev).dev, true); return ret; }
    reset_control_deassert(rstc); reset_control_put(rstc);
    size = resource_size(&(*dev).res);
    tmp = ioremap((*dev).res.start, size);
    if tmp.is_null() { amba_put_disable_pclk(dev); dev_pm_domain_detach(&mut (*dev).dev, true); return -ENOMEM; }
    pid = 0; i = 0; while i < 4 { pid |= (readl((tmp as *mut u8).add((size - 0x20 + 4 * i as u32) as usize)) & 255) << (i * 8); i += 1; }
    cid = 0; i = 0; while i < 4 { cid |= (readl((tmp as *mut u8).add((size - 0x10 + 4 * i as u32) as usize)) & 255) << (i * 8); i += 1; }
    if cid == CORESIGHT_CID { let csbase = (tmp as *mut u8).add((size - 4096) as usize); (*dev).uci.devarch = readl(csbase.add(UCI_REG_DEVARCH_OFFSET as usize)); (*dev).uci.devtype = readl(csbase.add(UCI_REG_DEVTYPE_OFFSET as usize)) & 0xff; }
    if cid == AMBA_CID || cid == CORESIGHT_CID { (*dev).periphid = pid; (*dev).cid = cid; }
    if (*dev).periphid == 0 { ret = -ENODEV; }
    iounmap(tmp); amba_put_disable_pclk(dev); dev_pm_domain_detach(&mut (*dev).dev, true); ret
}

// The remaining device-model glue is represented with the same externally supplied
// kernel types and functions; declarations are kept source-level and non-owning.
unsafe fn amba_match(dev: *mut device, drv: *const device_driver) -> c_int { let pcdev = to_amba_device(dev); let pcdrv = to_amba_driver(drv); mutex_lock(&mut (*pcdev).periphid_lock); if (*pcdev).periphid == 0 { if amba_read_periphid(pcdev) != 0 { mutex_unlock(&mut (*pcdev).periphid_lock); return -EPROBE_DEFER; } dev_set_uevent_suppress(dev, false); kobject_uevent(&mut (*dev).kobj, KOBJ_ADD); } mutex_unlock(&mut (*pcdev).periphid_lock); let ret = device_match_driver_override(dev, drv); if ret >= 0 { ret } else { (!amba_lookup((*pcdrv).id_table, pcdev).is_null()) as c_int } }

unsafe fn amba_uevent(dev: *const device, env: *mut kobj_uevent_env) -> c_int { let pcdev = to_amba_device(dev as *mut device); let mut ret = add_uevent_var(env, b"AMBA_ID=%08x\0".as_ptr() as *const c_char, (*pcdev).periphid); if ret != 0 { return ret; } ret = add_uevent_var(env, b"MODALIAS=amba:d%08X\0".as_ptr() as *const c_char, (*pcdev).periphid); ret }

// Remaining declarations retain the C implementation's externally visible API.
pub unsafe fn dev_is_amba(dev: *const device) -> bool { (*dev).bus == &amba_bustype }

unsafe fn of_amba_device_decode_irq(dev: *mut amba_device) -> c_int {
    let node = (*dev).dev.of_node;
    if IS_ENABLED(CONFIG_OF_IRQ) && !node.is_null() {
        let mut i = 0;
        while i < AMBA_NR_IRQS {
            let mut irq = of_irq_get(node, i);
            if irq < 0 { if irq == -EPROBE_DEFER { return irq; } irq = 0; }
            (*dev).irq[i as usize] = irq; i += 1;
        }
    }
    0
}

unsafe fn amba_probe(dev: *mut device) -> c_int {
    let pcdev = to_amba_device(dev); let pcdrv = to_amba_driver((*dev).driver);
    let id = amba_lookup((*pcdrv).id_table, pcdev); let mut ret;
    loop {
        ret = of_amba_device_decode_irq(pcdev); if ret != 0 { break; }
        ret = of_clk_set_defaults((*dev).of_node, false); if ret < 0 { break; }
        ret = dev_pm_domain_attach(dev, PD_FLAG_ATTACH_POWER_ON | PD_FLAG_DETACH_POWER_OFF); if ret != 0 { break; }
        ret = amba_get_enable_pclk(pcdev); if ret != 0 { break; }
        pm_runtime_get_noresume(dev); pm_runtime_set_active(dev); pm_runtime_enable(dev);
        ret = ((*pcdrv).probe)(pcdev, id); if ret == 0 { break; }
        pm_runtime_disable(dev); pm_runtime_set_suspended(dev); pm_runtime_put_noidle(dev); amba_put_disable_pclk(pcdev); break;
    }
    ret
}

unsafe fn amba_remove(dev: *mut device) { let pcdev = to_amba_device(dev); let drv = to_amba_driver((*dev).driver); pm_runtime_get_sync(dev); if !(*drv).remove.is_none() { ((*drv).remove.unwrap())(pcdev); } pm_runtime_put_noidle(dev); pm_runtime_disable(dev); pm_runtime_set_suspended(dev); pm_runtime_put_noidle(dev); amba_put_disable_pclk(pcdev); }
unsafe fn amba_shutdown(dev: *mut device) { if (*dev).driver.is_null() { return; } let drv = to_amba_driver((*dev).driver); if !(*drv).shutdown.is_none() { ((*drv).shutdown.unwrap())(to_amba_device(dev)); } }
unsafe fn amba_dma_configure(dev: *mut device) -> c_int { let drv = to_amba_driver((*dev).driver); let mut ret = 0; if !(*dev).of_node.is_null() { ret = of_dma_configure(dev, (*dev).of_node, true); } else if has_acpi_companion(dev) { ret = acpi_dma_configure(dev, acpi_get_dma_attr(to_acpi_device_node((*dev).fwnode))); } if ret == 0 && !(*dev).driver.is_null() && !(*drv).driver_managed_dma { ret = iommu_device_use_default_domain(dev); if ret != 0 { arch_teardown_dma_ops(dev); } } ret }
unsafe fn amba_dma_cleanup(dev: *mut device) { let drv = to_amba_driver((*dev).driver); if !(*drv).driver_managed_dma { iommu_device_unuse_default_domain(dev); } }

unsafe fn amba_init() -> c_int { bus_register(&amba_bustype) }
unsafe fn amba_proxy_probe(_adev: *mut amba_device, _id: *const amba_id) -> c_int { WARN(1, b"Stub driver should never match any device.\n\0".as_ptr() as *const c_char); -ENODEV }
static AMBA_STUB_DRV_IDS: [amba_id; 1] = [amba_id { id: 0, mask: 0, data: core::ptr::null_mut() }];
unsafe fn amba_stub_drv_init() -> c_int { if !IS_ENABLED(CONFIG_MODULES) { return 0; } __amba_driver_register(&mut amba_proxy_drv, core::ptr::null_mut()) }

pub unsafe fn __amba_driver_register(drv: *mut amba_driver, owner: *mut module) -> c_int { if (*drv).probe.is_none() { return -EINVAL; } (*drv).drv.owner = owner; (*drv).drv.bus = &amba_bustype; driver_register(&mut (*drv).drv) }
pub unsafe fn amba_driver_unregister(drv: *mut amba_driver) { driver_unregister(&mut (*drv).drv); }
unsafe fn amba_device_release(dev: *mut device) { let d = to_amba_device(dev); fwnode_handle_put(dev_fwnode(&mut (*d).dev)); if !(*d).res.parent.is_null() { release_resource(&mut (*d).res); } mutex_destroy(&mut (*d).periphid_lock); kfree(d as *mut c_void); }
pub unsafe fn amba_device_add(dev: *mut amba_device, parent: *mut resource) -> c_int { fwnode_handle_get(dev_fwnode(&mut (*dev).dev)); let mut ret = request_resource(parent, &mut (*dev).res); if ret != 0 { return ret; } if (*dev).periphid == 0 && amba_read_periphid(dev) != 0 { dev_set_uevent_suppress(&mut (*dev).dev, true); } ret = device_add(&mut (*dev).dev); if ret != 0 { release_resource(&mut (*dev).res); } ret }
unsafe fn amba_device_initialize(dev: *mut amba_device, name: *const c_char) { device_initialize(&mut (*dev).dev); if !name.is_null() { dev_set_name(&mut (*dev).dev, b"%s\0".as_ptr() as *const c_char, name); } (*dev).dev.release = Some(amba_device_release); (*dev).dev.bus = &amba_bustype; (*dev).dev.dma_mask = &mut (*dev).dev.coherent_dma_mask; (*dev).dev.dma_parms = &mut (*dev).dma_parms; (*dev).res.name = dev_name(&mut (*dev).dev); mutex_init(&mut (*dev).periphid_lock); }
pub unsafe fn amba_device_alloc(name: *const c_char, base: resource_size_t, size: usize) -> *mut amba_device { let dev = kzalloc_obj::<amba_device>(); if !dev.is_null() { amba_device_initialize(dev, name); (*dev).res.start = base; (*dev).res.end = base + size as resource_size_t - 1; (*dev).res.flags = IORESOURCE_MEM; } dev }
pub unsafe fn amba_device_register(dev: *mut amba_device, parent: *mut resource) -> c_int { amba_device_initialize(dev, (*dev).dev.init_name); (*dev).dev.init_name = core::ptr::null(); amba_device_add(dev, parent) }
pub unsafe fn amba_device_put(dev: *mut amba_device) { put_device(&mut (*dev).dev); }
pub unsafe fn amba_device_unregister(dev: *mut amba_device) { device_unregister(&mut (*dev).dev); }
pub unsafe fn amba_request_regions(dev: *mut amba_device, mut name: *const c_char) -> c_int { if name.is_null() { name = (*(*dev).dev.driver).name; } if request_mem_region((*dev).res.start, resource_size(&(*dev).res), name).is_null() { -EBUSY } else { 0 } }
pub unsafe fn amba_release_regions(dev: *mut amba_device) { release_mem_region((*dev).res.start, resource_size(&(*dev).res)); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
