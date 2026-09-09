// SPDX-License-Identifier: GPL-2.0-only
/*
 * Generic Exynos Bus frequency driver with DEVFREQ Framework
 *
 * Copyright (c) 2016 Samsung Electronics Co., Ltd.
 * Author : Chanwoo Choi <cw00.choi@samsung.com>
 *
 * This driver support Exynos Bus frequency feature by using
 * DEVFREQ framework and is based on drivers/devfreq/exynos/exynos4_bus.c.
 */

// C includes are supplied by the surrounding kernel Rust bindings.

const DEFAULT_SATURATION_RATIO: u32 = 40;

#[repr(C)]
struct ExynosBus {
    dev: *mut Device,
    icc_pdev: *mut PlatformDevice,
    devfreq: *mut Devfreq,
    edev: *mut *mut DevfreqEventDev,
    edev_count: u32,
    lock: Mutex,
    curr_freq: CULong,
    opp_token: CInt,
    clk: *mut Clk,
    ratio: u32,
}

/* Control the devfreq-event device to get the current state of bus. */
unsafe fn exynos_bus_enable_edev(bus: *mut ExynosBus) -> CInt {
    exynos_bus_edev_op(bus, devfreq_event_enable_edev)
}

unsafe fn exynos_bus_disable_edev(bus: *mut ExynosBus) -> CInt {
    exynos_bus_edev_op(bus, devfreq_event_disable_edev)
}

unsafe fn exynos_bus_set_event(bus: *mut ExynosBus) -> CInt {
    exynos_bus_edev_op(bus, devfreq_event_set_event)
}

unsafe fn exynos_bus_edev_op(
    bus: *mut ExynosBus,
    op: unsafe extern "C" fn(*mut DevfreqEventDev) -> CInt,
) -> CInt {
    let mut i: CInt = 0;
    while i < (*bus).edev_count as CInt {
        let event_dev = *(*bus).edev.add(i as usize);
        if !event_dev.is_null() {
            let ret = op(event_dev);
            if ret < 0 { return ret; }
        }
        i += 1;
    }
    0
}

unsafe fn exynos_bus_get_event(
    bus: *mut ExynosBus,
    edata: *mut DevfreqEventData,
) -> CInt {
    let mut event_data = DevfreqEventData::default();
    let mut load_count: CULong = 0;
    let mut total_count: CULong = 0;
    let mut ret: CInt = 0;
    let mut i: CInt = 0;
    while i < (*bus).edev_count as CInt {
        let event_dev = *(*bus).edev.add(i as usize);
        if !event_dev.is_null() {
            ret = devfreq_event_get_event(event_dev, &mut event_data);
            if ret < 0 { return ret; }
            if i == 0 || event_data.load_count > load_count {
                load_count = event_data.load_count;
                total_count = event_data.total_count;
            }
        }
        i += 1;
    }
    (*edata).load_count = load_count;
    (*edata).total_count = total_count;
    ret
}

unsafe extern "C" fn exynos_bus_target(
    dev: *mut Device, freq: *mut CULong, flags: U32,
) -> CInt {
    let bus = dev_get_drvdata(dev) as *mut ExynosBus;
    let new_opp = devfreq_recommended_opp(dev, freq, flags);
    if is_err(new_opp as *mut CVoid) {
        dev_err(dev, "failed to get recommended opp instance\n");
        return ptr_err(new_opp as *mut CVoid);
    }
    dev_pm_opp_put(new_opp);
    mutex_lock(&mut (*bus).lock);
    let ret = dev_pm_opp_set_rate(dev, *freq);
    if ret == 0 { (*bus).curr_freq = *freq; }
    mutex_unlock(&mut (*bus).lock);
    ret
}

unsafe extern "C" fn exynos_bus_get_dev_status(
    dev: *mut Device, stat: *mut DevfreqDevStatus,
) -> CInt {
    let bus = dev_get_drvdata(dev) as *mut ExynosBus;
    let mut edata = DevfreqEventData::default();
    (*stat).current_frequency = (*bus).curr_freq;
    let ret = exynos_bus_get_event(bus, &mut edata);
    if ret < 0 {
        dev_err(dev, "failed to get event from devfreq-event devices\n");
        (*stat).total_time = 0;
        (*stat).busy_time = 0;
    } else {
        (*stat).busy_time = (edata.load_count * 100) / (*bus).ratio as CULong;
        (*stat).total_time = edata.total_count;
        dev_dbg(dev, "Usage of devfreq-event : %lu/%lu\n", (*stat).busy_time, (*stat).total_time);
    }
    let ret = exynos_bus_set_event(bus);
    if ret < 0 {
        dev_err(dev, "failed to set event to devfreq-event devices\n");
        return ret;
    }
    ret
}

unsafe extern "C" fn exynos_bus_exit(dev: *mut Device) {
    let bus = dev_get_drvdata(dev) as *mut ExynosBus;
    if exynos_bus_disable_edev(bus) < 0 { dev_warn(dev, "failed to disable the devfreq-event devices\n"); }
    platform_device_unregister((*bus).icc_pdev);
    dev_pm_opp_of_remove_table(dev);
    dev_pm_opp_put_regulators((*bus).opp_token);
}

unsafe extern "C" fn exynos_bus_passive_exit(dev: *mut Device) {
    let bus = dev_get_drvdata(dev) as *mut ExynosBus;
    platform_device_unregister((*bus).icc_pdev);
    dev_pm_opp_of_remove_table(dev);
}

// The remaining file-local routines retain the kernel driver's direct control flow.
// Their kernel API types and declarations are supplied externally.
unsafe fn exynos_bus_parent_parse_of(np: *mut DeviceNode, bus: *mut ExynosBus) -> CInt {
    let dev = (*bus).dev;
    let supplies = [b"vdd\0".as_ptr() as *const CChar, core::ptr::null()];
    let mut ret = dev_pm_opp_set_regulators(dev, supplies.as_ptr());
    if ret < 0 { dev_err(dev, "failed to set regulators %d\n", ret); return ret; }
    (*bus).opp_token = ret;
    let count = devfreq_event_get_edev_count(dev, b"devfreq-events\0".as_ptr() as *const CChar);
    if count < 0 { dev_err(dev, "failed to get the count of devfreq-event dev\n"); dev_pm_opp_put_regulators((*bus).opp_token); return count; }
    (*bus).edev_count = count as u32;
    let size = core::mem::size_of::<*mut DevfreqEventDev>() * count as usize;
    (*bus).edev = devm_kzalloc(dev, size, GFP_KERNEL) as *mut *mut DevfreqEventDev;
    if (*bus).edev.is_null() { dev_pm_opp_put_regulators((*bus).opp_token); return -ENOMEM; }
    let mut i = 0;
    while i < count {
        *(*bus).edev.add(i as usize) = devfreq_event_get_edev_by_phandle(dev, b"devfreq-events\0".as_ptr() as *const CChar, i);
        if is_err(*(*bus).edev.add(i as usize) as *mut CVoid) { dev_pm_opp_put_regulators((*bus).opp_token); return -EPROBE_DEFER; }
        i += 1;
    }
    if of_property_read_u32(np, b"exynos,saturation-ratio\0".as_ptr() as *const CChar, &mut (*bus).ratio) != 0 { (*bus).ratio = DEFAULT_SATURATION_RATIO; }
    0
}

unsafe fn exynos_bus_parse_of(bus: *mut ExynosBus) -> CInt {
    let dev = (*bus).dev;
    (*bus).clk = devm_clk_get_enabled(dev, b"bus\0".as_ptr() as *const CChar);
    if is_err((*bus).clk as *mut CVoid) { return dev_err_probe(dev, ptr_err((*bus).clk as *mut CVoid), b"failed to get bus clock\0".as_ptr() as *const CChar); }
    let mut ret = dev_pm_opp_of_add_table(dev);
    if ret < 0 { dev_err(dev, b"failed to get OPP table\0".as_ptr() as *const CChar); return ret; }
    let mut rate = clk_get_rate((*bus).clk);
    let opp = devfreq_recommended_opp(dev, &mut rate, 0);
    if is_err(opp as *mut CVoid) { ret = ptr_err(opp as *mut CVoid); dev_pm_opp_of_remove_table(dev); return ret; }
    (*bus).curr_freq = dev_pm_opp_get_freq(opp);
    dev_pm_opp_put(opp);
    0
}

unsafe fn exynos_bus_profile_init(bus: *mut ExynosBus, profile: *mut DevfreqDevProfile) -> CInt {
    let dev = (*bus).dev;
    (*profile).polling_ms = 50;
    (*profile).target = Some(exynos_bus_target);
    (*profile).get_dev_status = Some(exynos_bus_get_dev_status);
    (*profile).exit = Some(exynos_bus_exit);
    let data = devm_kzalloc(dev, core::mem::size_of::<DevfreqSimpleOndemandData>(), GFP_KERNEL) as *mut DevfreqSimpleOndemandData;
    if data.is_null() { return -ENOMEM; }
    (*data).upthreshold = 40; (*data).downdifferential = 5;
    (*bus).devfreq = devm_devfreq_add_device(dev, profile, b"simple_ondemand\0".as_ptr() as *const CChar, data as *mut CVoid);
    if is_err((*bus).devfreq as *mut CVoid) { return ptr_err((*bus).devfreq as *mut CVoid); }
    let mut ret = devm_devfreq_register_opp_notifier(dev, (*bus).devfreq);
    if ret < 0 { return ret; }
    ret = exynos_bus_enable_edev(bus);
    if ret < 0 { return ret; }
    ret = exynos_bus_set_event(bus);
    if ret < 0 { let _ = exynos_bus_disable_edev(bus); }
    ret
}

unsafe fn exynos_bus_profile_init_passive(bus: *mut ExynosBus, profile: *mut DevfreqDevProfile) -> CInt {
    let dev = (*bus).dev;
    (*profile).target = Some(exynos_bus_target); (*profile).exit = Some(exynos_bus_passive_exit);
    let parent = devfreq_get_devfreq_by_phandle(dev, b"devfreq\0".as_ptr() as *const CChar, 0);
    if is_err(parent as *mut CVoid) { return -EPROBE_DEFER; }
    let data = devm_kzalloc(dev, core::mem::size_of::<DevfreqPassiveData>(), GFP_KERNEL) as *mut DevfreqPassiveData;
    if data.is_null() { return -ENOMEM; }
    (*data).parent = parent;
    (*bus).devfreq = devm_devfreq_add_device(dev, profile, b"passive\0".as_ptr() as *const CChar, data as *mut CVoid);
    if is_err((*bus).devfreq as *mut CVoid) { return ptr_err((*bus).devfreq as *mut CVoid); }
    0
}

unsafe extern "C" fn exynos_bus_probe(pdev: *mut PlatformDevice) -> CInt {
    let dev = platform_device_dev(pdev); let np = (*dev).of_node;
    if np.is_null() { return -EINVAL; }
    let bus = devm_kzalloc(dev, core::mem::size_of::<ExynosBus>(), GFP_KERNEL) as *mut ExynosBus;
    if bus.is_null() { return -ENOMEM; }
    mutex_init(&mut (*bus).lock); (*bus).dev = dev; platform_set_drvdata(pdev, bus as *mut CVoid);
    let profile = devm_kzalloc(dev, core::mem::size_of::<DevfreqDevProfile>(), GFP_KERNEL) as *mut DevfreqDevProfile;
    if profile.is_null() { return -ENOMEM; }
    let node = of_parse_phandle(np, b"devfreq\0".as_ptr() as *const CChar, 0);
    let mut ret;
    if !node.is_null() { of_node_put(node); ret = exynos_bus_parse_of(bus); }
    else { ret = exynos_bus_parent_parse_of(np, bus); if ret >= 0 { ret = exynos_bus_parse_of(bus); } }
    if ret < 0 { dev_pm_opp_of_remove_table(dev); dev_pm_opp_put_regulators((*bus).opp_token); return ret; }
    if !node.is_null() { ret = exynos_bus_profile_init_passive(bus, profile); } else { ret = exynos_bus_profile_init(bus, profile); }
    if ret < 0 { dev_pm_opp_of_remove_table(dev); dev_pm_opp_put_regulators((*bus).opp_token); }
    ret
}

unsafe extern "C" fn exynos_bus_shutdown(pdev: *mut PlatformDevice) { let bus = dev_get_drvdata(platform_device_dev(pdev)) as *mut ExynosBus; devfreq_suspend_device((*bus).devfreq); }
unsafe extern "C" fn exynos_bus_resume(dev: *mut Device) -> CInt { exynos_bus_enable_edev(dev_get_drvdata(dev) as *mut ExynosBus) }
unsafe extern "C" fn exynos_bus_suspend(dev: *mut Device) -> CInt { exynos_bus_disable_edev(dev_get_drvdata(dev) as *mut ExynosBus) }

// External kernel declarations intentionally remain unresolved.
type CInt = i32; type U32 = u32; type CULong = usize; type CChar = i8; type CVoid = core::ffi::c_void;
struct Device; struct PlatformDevice; struct Devfreq; struct DevfreqEventDev; struct Mutex; struct Clk; struct DeviceNode;
#[repr(C)] #[derive(Default)] struct DevfreqEventData { load_count: CULong, total_count: CULong }
#[repr(C)] struct DevfreqDevStatus { current_frequency: CULong, busy_time: CULong, total_time: CULong }
extern "C" {
    fn dev_get_drvdata(*mut Device) -> *mut CVoid; fn dev_err(*mut Device, *const str, ...); fn dev_warn(*mut Device, *const str, ...); fn dev_dbg(*mut Device, *const str, ...);
    fn devfreq_event_enable_edev(*mut DevfreqEventDev) -> CInt; fn devfreq_event_disable_edev(*mut DevfreqEventDev) -> CInt; fn devfreq_event_set_event(*mut DevfreqEventDev) -> CInt; fn devfreq_event_get_event(*mut DevfreqEventDev, *mut DevfreqEventData) -> CInt;
    fn dev_pm_opp_put(*mut DevPmOpp); fn dev_pm_opp_set_rate(*mut Device, CULong) -> CInt; fn dev_pm_opp_set_regulators(*mut Device, *const *const CChar) -> CInt; fn dev_pm_opp_put_regulators(CInt); fn dev_pm_opp_of_remove_table(*mut Device);
    fn devfreq_recommended_opp(*mut Device, *mut CULong, U32) -> *mut DevPmOpp; fn is_err(*mut CVoid) -> bool; fn ptr_err(*mut CVoid) -> CInt;
    fn mutex_lock(*mut Mutex); fn mutex_unlock(*mut Mutex); fn devm_kzalloc(*mut Device, usize, U32) -> *mut CVoid; fn devfreq_event_get_edev_count(*mut Device,*const CChar)->CInt; fn devfreq_event_get_edev_by_phandle(*mut Device,*const CChar,CInt)->*mut DevfreqEventDev; fn of_property_read_u32(*mut DeviceNode,*const CChar,*mut U32)->CInt;
    fn platform_device_unregister(*mut PlatformDevice);
}
struct DevPmOpp;
struct DevfreqDevProfile { polling_ms: u32, target: Option<unsafe extern "C" fn(*mut Device,*mut CULong,U32)->CInt>, get_dev_status: Option<unsafe extern "C" fn(*mut Device,*mut DevfreqDevStatus)->CInt>, exit: Option<unsafe extern "C" fn(*mut Device)> }
struct DevfreqSimpleOndemandData { upthreshold: u32, downdifferential: u32 }
struct DevfreqPassiveData { parent: *mut Devfreq }
const GFP_KERNEL: U32 = 0; const ENOMEM: CInt = 12; const EINVAL: CInt = 22; const EPROBE_DEFER: CInt = 517;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
