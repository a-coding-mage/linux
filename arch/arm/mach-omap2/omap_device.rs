// SPDX-License-Identifier: GPL-2.0-only
/*
 * omap_device implementation
 *
 * Copyright (C) 2009-2010 Nokia Corporation
 * Paul Walmsley, Kevin Hilman
 *
 * This code provides a consistent interface for OMAP device drivers
 * to control power management and interconnect properties of their
 * devices.
 */

// C dependencies supplied by the surrounding kernel translation.

static mut OMAP_DEVICE_FAIL_PM_DOMAIN: dev_pm_domain = dev_pm_domain { ops: dev_pm_ops { runtime_suspend: None, runtime_resume: None } };
static mut OMAP_DEVICE_PM_DOMAIN: dev_pm_domain = dev_pm_domain { ops: dev_pm_ops { runtime_suspend: None, runtime_resume: None } };

unsafe fn _add_clkdev(od: *mut omap_device, clk_alias: *const c_char, clk_name: *const c_char) {
    let mut r: *mut clk;
    let rc: c_int;
    if clk_alias.is_null() || clk_name.is_null() { return; }
    dev_dbg(&mut (*(*od).pdev).dev, "Creating %s -> %s\n", clk_alias, clk_name);
    r = clk_get_sys(dev_name(&mut (*(*od).pdev).dev), clk_alias);
    if !IS_ERR(r) { dev_dbg(&mut (*(*od).pdev).dev, "alias %s already exists\n", clk_alias); clk_put(r); return; }
    r = clk_get_sys(core::ptr::null(), clk_name);
    if IS_ERR(r) {
        let mut clkspec: of_phandle_args = core::mem::zeroed();
        clkspec.np = of_find_node_by_name(core::ptr::null(), clk_name);
        r = of_clk_get_from_provider(&mut clkspec);
        rc = clk_register_clkdev(r, clk_alias, dev_name(&mut (*(*od).pdev).dev));
    } else { rc = clk_add_alias(clk_alias, dev_name(&mut (*(*od).pdev).dev), clk_name, core::ptr::null()); }
    if rc != 0 {
        if rc == -ENODEV || rc == -ENOMEM { dev_err(&mut (*(*od).pdev).dev, "clkdev_alloc for %s failed\n", clk_alias); }
        else { dev_err(&mut (*(*od).pdev).dev, "clk_get for %s failed\n", clk_name); }
    }
}

unsafe fn _add_hwmod_clocks_clkdev(od: *mut omap_device, oh: *mut omap_hwmod) {
    _add_clkdev(od, c_str!("fck"), (*oh).main_clk);
    for i in 0..(*oh).opt_clks_cnt { _add_clkdev(od, (*oh).opt_clks.add(i).read().role, (*oh).opt_clks.add(i).read().clk); }
}

unsafe fn omap_device_build_from_dt(pdev: *mut platform_device) -> c_int {
    let node = (*pdev).dev.of_node;
    let mut res: resource = core::mem::zeroed();
    let mut oh_name: *const c_char = core::ptr::null();
    let oh_cnt = of_property_count_strings(node, c_str!("ti,hwmods"));
    if oh_cnt <= 0 { dev_dbg(&mut (*pdev).dev, "No 'hwmods' to build omap_device\n"); return -ENODEV; }
    let mut ret = 0; let mut device_active = false; let mut skip_pm_domain = false;
    ret = of_property_read_string_index(node, c_str!("ti,hwmods"), 0, &mut oh_name);
    if ret == 0 && (!strncmp(c_str!("dma_system"), oh_name, 10) || !strncmp(c_str!("dma"), oh_name, 3)) { skip_pm_domain = true; }
    if !skip_pm_domain && omap_hwmod_parse_module_range(core::ptr::null_mut(), node, &mut res) == 0 { return -ENODEV; }
    let hwmods = kzalloc_objs::<*mut omap_hwmod>(oh_cnt as usize);
    if hwmods.is_null() { ret = -ENOMEM; return ret; }
    for i in 0..oh_cnt as usize {
        of_property_read_string_index(node, c_str!("ti,hwmods"), i as c_int, &mut oh_name);
        let oh = omap_hwmod_lookup(oh_name);
        if oh.is_null() { dev_err(&mut (*pdev).dev, "Cannot lookup hwmod '%s'\n", oh_name); ret = -EINVAL; break; }
        *hwmods.add(i) = oh; if (*oh).flags & HWMOD_INIT_NO_IDLE != 0 { device_active = true; }
    }
    if ret == 0 {
        let od = omap_device_alloc(pdev, hwmods, oh_cnt);
        if IS_ERR(od) { dev_err(&mut (*pdev).dev, "Cannot allocate omap_device for :%s\n", oh_name); ret = PTR_ERR(od); }
        else {
            for i in 0..(*pdev).num_resources as usize { let r = (*pdev).resource.add(i); if (*r).name.is_null() { (*r).name = dev_name(&mut (*pdev).dev); } }
            if !skip_pm_domain { dev_pm_domain_set(&mut (*pdev).dev, &mut OMAP_DEVICE_PM_DOMAIN); if device_active { omap_device_enable(pdev); pm_runtime_set_active(&mut (*pdev).dev); } }
        }
    }
    kfree(hwmods); if ret != 0 { dev_pm_domain_set(&mut (*pdev).dev, &mut OMAP_DEVICE_FAIL_PM_DOMAIN); } ret
}

unsafe fn _omap_device_enable_hwmods(od: *mut omap_device) -> c_int { let mut ret=0; for i in 0..(*od).hwmods_cnt as usize { ret |= omap_hwmod_enable(*(*od).hwmods.add(i)); } ret }
unsafe fn _omap_device_idle_hwmods(od: *mut omap_device) -> c_int { let mut ret=0; for i in 0..(*od).hwmods_cnt as usize { ret |= omap_hwmod_idle(*(*od).hwmods.add(i)); } ret }

unsafe fn omap_device_alloc(pdev: *mut platform_device, ohs: *mut *mut omap_hwmod, oh_cnt: c_int) -> *mut omap_device {
    let od = kzalloc_flex::<omap_device>(oh_cnt as usize); if od.is_null() { dev_err(&mut (*pdev).dev, "omap_device: build failed (%d)\n", -ENOMEM); return ERR_PTR(-ENOMEM); }
    (*od).hwmods_cnt=oh_cnt; core::ptr::copy_nonoverlapping(ohs, (*od).hwmods, oh_cnt as usize); (*od).pdev=pdev; (*pdev).archdata.od=od;
    for i in 0..oh_cnt as usize { let hwmod=*(*od).hwmods.add(i); (*hwmod).od=od; _add_hwmod_clocks_clkdev(od,hwmod); } od
}
unsafe fn omap_device_delete(od:*mut omap_device) { if !od.is_null() { (*(*od).pdev).archdata.od=core::ptr::null_mut(); kfree(od); } }

pub unsafe fn omap_device_enable(pdev:*mut platform_device)->c_int { let od=to_omap_device(pdev); if (*od)._state==OMAP_DEVICE_STATE_ENABLED { dev_warn(&mut (*pdev).dev,"omap_device: %s() called from invalid state %d\n",c_str!("omap_device_enable"),(*od)._state); return -EINVAL; } let ret=_omap_device_enable_hwmods(od); if ret==0 {(*od)._state=OMAP_DEVICE_STATE_ENABLED;} ret }
pub unsafe fn omap_device_idle(pdev:*mut platform_device)->c_int { let od=to_omap_device(pdev); if (*od)._state!=OMAP_DEVICE_STATE_ENABLED { dev_warn(&mut (*pdev).dev,"omap_device: %s() called from invalid state %d\n",c_str!("omap_device_idle"),(*od)._state); return -EINVAL; } let ret=_omap_device_idle_hwmods(od); if ret==0 {(*od)._state=OMAP_DEVICE_STATE_IDLE;} ret }
pub unsafe fn omap_device_assert_hardreset(pdev:*mut platform_device,name:*const c_char)->c_int { let od=to_omap_device(pdev); let mut ret=0; for i in 0..(*od).hwmods_cnt as usize { ret=omap_hwmod_assert_hardreset(*(*od).hwmods.add(i),name); if ret!=0 {break;} } ret }
pub unsafe fn omap_device_deassert_hardreset(pdev:*mut platform_device,name:*const c_char)->c_int { let od=to_omap_device(pdev); let mut ret=0; for i in 0..(*od).hwmods_cnt as usize { ret=omap_hwmod_deassert_hardreset(*(*od).hwmods.add(i),name); if ret!=0 {break;} } ret }

// CONFIG_PM / CONFIG_SUSPEND blocks are preserved as external-dependent kernel callbacks.
#[cfg(feature = "CONFIG_PM")]
unsafe fn _od_runtime_suspend(dev: *mut device) -> c_int { let pdev=to_platform_device(dev); let ret=pm_generic_runtime_suspend(dev); if ret!=0 {ret} else {omap_device_idle(pdev)} }
#[cfg(feature = "CONFIG_PM")]
unsafe fn _od_runtime_resume(dev: *mut device) -> c_int { let pdev=to_platform_device(dev); let ret=omap_device_enable(pdev); if ret!=0 {dev_err(dev,"use pm_runtime_put_sync_suspend() in driver?\n"); ret} else {pm_generic_runtime_resume(dev)} }
#[cfg(feature = "CONFIG_PM")]
unsafe fn _od_fail_runtime_suspend(dev:*mut device)->c_int { dev_warn(dev,"%s: FIXME: missing hwmod/omap_dev info\n",c_str!("_od_fail_runtime_suspend")); -ENODEV }
#[cfg(feature = "CONFIG_PM")]
unsafe fn _od_fail_runtime_resume(dev:*mut device)->c_int { dev_warn(dev,"%s: FIXME: missing hwmod/omap_dev info\n",c_str!("_od_fail_runtime_resume")); -ENODEV }

#[cfg(feature = "CONFIG_SUSPEND")]
unsafe fn _od_suspend_noirq(dev:*mut device)->c_int { let pdev=to_platform_device(dev); let od=to_omap_device(pdev); if (*od)._driver_status!=BUS_NOTIFY_BOUND_DRIVER{return 0;} let ret=pm_generic_suspend_noirq(dev); if ret==0 && !pm_runtime_status_suspended(dev) && pm_generic_runtime_suspend(dev)==0 {omap_device_idle(pdev); (*od).flags|=OMAP_DEVICE_SUSPENDED;} ret }
#[cfg(feature = "CONFIG_SUSPEND")]
unsafe fn _od_resume_noirq(dev:*mut device)->c_int { let pdev=to_platform_device(dev); let od=to_omap_device(pdev); if (*od).flags&OMAP_DEVICE_SUSPENDED!=0 {(*od).flags&=!OMAP_DEVICE_SUSPENDED; omap_device_enable(pdev); pm_generic_runtime_resume(dev);} pm_generic_resume_noirq(dev) }

unsafe fn _omap_device_notifier_call(_nb:*mut notifier_block,event:c_ulong,dev:*mut c_void)->c_int {
    let pdev=to_platform_device(dev as *mut device); let od;
    match event {
        BUS_NOTIFY_REMOVED_DEVICE => { if !(*pdev).archdata.od.is_null(){omap_device_delete((*pdev).archdata.od);} }
        BUS_NOTIFY_UNBOUND_DRIVER => { od=to_omap_device(pdev); if !od.is_null()&&(*od)._state==OMAP_DEVICE_STATE_ENABLED {dev_info(dev,"enabled after unload, idling\n"); if omap_device_idle(pdev)!=0 {dev_err(dev,"failed to idle\n");}} }
        BUS_NOTIFY_BIND_DRIVER => { od=to_omap_device(pdev); if !od.is_null(){(*od)._driver_status=BUS_NOTIFY_BIND_DRIVER; if (*od)._state==OMAP_DEVICE_STATE_ENABLED&&pm_runtime_status_suspended(dev){pm_runtime_set_active(dev);}} }
        BUS_NOTIFY_ADD_DEVICE => {if !(*pdev).dev.of_node.is_null(){omap_device_build_from_dt(pdev);}}
        _ => {od=to_omap_device(pdev); if !od.is_null(){(*od)._driver_status=event;}}
    } NOTIFY_DONE
}

unsafe fn omap_device_late_idle(dev:*mut device,_data:*mut c_void)->c_int { let pdev=to_platform_device(dev); let od=to_omap_device(pdev); if od.is_null(){return 0;} for i in 0..(*od).hwmods_cnt as usize {if (*(*od).hwmods.add(i)).flags&HWMOD_INIT_NO_IDLE!=0{return 0;}} if (*od)._driver_status!=BUS_NOTIFY_BOUND_DRIVER&&(*od)._driver_status!=BUS_NOTIFY_BIND_DRIVER&&(*od)._state==OMAP_DEVICE_STATE_ENABLED {dev_warn(dev,"%s: enabled but no driver.  Idling\n",c_str!("omap_device_late_idle")); omap_device_idle(pdev);} 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
