// SPDX-License-Identifier: GPL-2.0
/*
 * drivers/base/power/common.c - Common device power management code.
 *
 * Copyright (C) 2011 Rafael J. Wysocki <rjw@sisk.pl>, Renesas Electronics Corp.
 */
// C dependencies supplied by the surrounding kernel translation unit.

pub unsafe fn dev_pm_get_subsys_data(dev: *mut device) -> i32 {
    let mut psd: *mut pm_subsys_data = kzalloc_obj::<pm_subsys_data>();
    if psd.is_null() {
        return -ENOMEM;
    }

    spin_lock_irq(&mut (*dev).power.lock);

    if !(*dev).power.subsys_data.is_null() {
        (*(*dev).power.subsys_data).refcount += 1;
    } else {
        spin_lock_init(&mut (*psd).lock);
        (*psd).refcount = 1;
        (*dev).power.subsys_data = psd;
        pm_clk_init(dev);
        psd = core::ptr::null_mut();
    }

    spin_unlock_irq(&mut (*dev).power.lock);

    /* kfree() verifies that its argument is nonzero. */
    kfree(psd);

    0
}

pub unsafe fn dev_pm_put_subsys_data(dev: *mut device) {
    let mut psd: *mut pm_subsys_data;

    spin_lock_irq(&mut (*dev).power.lock);

    psd = dev_to_psd(dev);
    if psd.is_null() {
        spin_unlock_irq(&mut (*dev).power.lock);
        kfree(psd);
        return;
    }

    (*psd).refcount -= 1;
    if (*psd).refcount == 0 {
        (*dev).power.subsys_data = core::ptr::null_mut();
    } else {
        psd = core::ptr::null_mut();
    }

    spin_unlock_irq(&mut (*dev).power.lock);
    kfree(psd);
}

pub unsafe fn dev_pm_domain_attach(dev: *mut device, flags: u32) -> i32 {
    let mut ret: i32;

    if !(*dev).pm_domain.is_null() {
        return 0;
    }

    ret = acpi_dev_pm_attach(dev, (flags & PD_FLAG_ATTACH_POWER_ON) != 0);
    if ret == 0 {
        ret = genpd_dev_pm_attach(dev);
    }

    if !(*dev).pm_domain.is_null() {
        (*dev).power.detach_power_off = (flags & PD_FLAG_DETACH_POWER_OFF) != 0;
    }

    if ret < 0 { ret } else { 0 }
}

pub unsafe fn dev_pm_domain_attach_by_id(dev: *mut device, index: u32) -> *mut device {
    if !(*dev).pm_domain.is_null() {
        return ERR_PTR(-EEXIST);
    }
    genpd_dev_pm_attach_by_id(dev, index)
}

pub unsafe fn dev_pm_domain_attach_by_name(dev: *mut device, name: *const i8) -> *mut device {
    if !(*dev).pm_domain.is_null() {
        return ERR_PTR(-EEXIST);
    }
    genpd_dev_pm_attach_by_name(dev, name)
}

pub unsafe fn dev_pm_domain_attach_list(
    dev: *mut device,
    data: *const dev_pm_domain_attach_data,
    list: *mut *mut dev_pm_domain_list,
) -> i32 {
    let np = (*dev).of_node;
    let mut pds: *mut dev_pm_domain_list;
    let mut pd_dev: *mut device = core::ptr::null_mut();
    let mut ret: i32;
    let mut i: i32;
    let mut num_pds: i32 = 0;
    let mut by_id = true;
    let pd_flags = if !data.is_null() { (*data).pd_flags } else { 0 };
    let mut link_flags = if pd_flags & PD_FLAG_NO_DEV_LINK != 0 { 0 } else {
        DL_FLAG_STATELESS | DL_FLAG_PM_RUNTIME
    };

    if !(*dev).pm_domain.is_null() { return -EEXIST; }
    if np.is_null() { return 0; }

    if !data.is_null() && !(*data).pd_names.is_null() {
        num_pds = (*data).num_pd_names;
        by_id = false;
    } else {
        num_pds = of_count_phandle_with_args(np, b"power-domains\0".as_ptr() as *const i8,
                                             b"#power-domain-cells\0".as_ptr() as *const i8);
    }
    if num_pds <= 0 { return 0; }

    pds = kzalloc_obj::<dev_pm_domain_list>();
    if pds.is_null() { return -ENOMEM; }
    let size = core::mem::size_of::<*mut device>() + core::mem::size_of::<*mut device_link>() +
               core::mem::size_of::<u32>();
    (*pds).pd_devs = kcalloc(num_pds as usize, size, GFP_KERNEL) as *mut *mut device;
    if (*pds).pd_devs.is_null() { kfree(pds); return -ENOMEM; }
    (*pds).pd_links = ((*pds).pd_devs as *mut u8).add((num_pds as usize) * core::mem::size_of::<*mut device>()) as *mut *mut device_link;
    (*pds).opp_tokens = ((*pds).pd_links as *mut u8).add((num_pds as usize) * core::mem::size_of::<*mut device_link>()) as *mut u32;

    if link_flags != 0 && pd_flags & PD_FLAG_DEV_LINK_ON != 0 { link_flags |= DL_FLAG_RPM_ACTIVE; }

    i = 0;
    while i < num_pds {
        pd_dev = if by_id { dev_pm_domain_attach_by_id(dev, i as u32) }
                 else { dev_pm_domain_attach_by_name(dev, *(*data).pd_names.add(i as usize)) };
        if IS_ERR_OR_NULL(pd_dev) {
            ret = if !pd_dev.is_null() { PTR_ERR(pd_dev) } else { -ENODEV };
            goto_err_attach!(pds, i, ret);
        }
        if pd_flags & PD_FLAG_REQUIRED_OPP != 0 {
            let config = dev_pm_opp_config { required_dev: pd_dev, required_dev_index: i as u32 };
            ret = dev_pm_opp_set_config(dev, &config);
            if ret < 0 { goto_err_link!(pds, i, pd_dev, ret); }
            *(*pds).opp_tokens.add(i as usize) = ret as u32;
        }
        if link_flags != 0 {
            let link = device_link_add(dev, pd_dev, link_flags);
            if link.is_null() { ret = -ENODEV; goto_err_link!(pds, i, pd_dev, ret); }
            *(*pds).pd_links.add(i as usize) = link;
        }
        *(*pds).pd_devs.add(i as usize) = pd_dev;
        i += 1;
    }
    (*pds).num_pds = num_pds;
    *list = pds;
    num_pds
}

unsafe fn devm_pm_domain_detach_list(_list: *mut core::ffi::c_void) {
    dev_pm_domain_detach_list(_list as *mut dev_pm_domain_list);
}

pub unsafe fn devm_pm_domain_attach_list(
    dev: *mut device,
    data: *const dev_pm_domain_attach_data,
    list: *mut *mut dev_pm_domain_list,
) -> i32 {
    let num_pds = dev_pm_domain_attach_list(dev, data, list);
    if num_pds <= 0 { return num_pds; }
    let ret = devm_add_action_or_reset(dev, devm_pm_domain_detach_list, *list as *mut core::ffi::c_void);
    if ret != 0 { return ret; }
    num_pds
}

pub unsafe fn dev_pm_domain_detach_list(list: *mut dev_pm_domain_list) {
    if list.is_null() { return; }
    let mut i = 0;
    while i < (*list).num_pds {
        dev_pm_opp_clear_config(*(*list).opp_tokens.add(i as usize));
        let link = *(*list).pd_links.add(i as usize);
        if !link.is_null() { device_link_del(link); }
        dev_pm_domain_detach(*(*list).pd_devs.add(i as usize), true);
        i += 1;
    }
    kfree((*list).pd_devs);
    kfree(list);
}

pub unsafe fn dev_pm_domain_detach(dev: *mut device, power_off: bool) {
    if !(*dev).pm_domain.is_null() && !(*(*dev).pm_domain).detach.is_none() {
        ((*(*dev).pm_domain).detach.unwrap())(dev, power_off);
    }
}

pub unsafe fn dev_pm_domain_start(dev: *mut device) -> i32 {
    if !(*dev).pm_domain.is_null() && !(*(*dev).pm_domain).start.is_none() {
        return ((*(*dev).pm_domain).start.unwrap())(dev);
    }
    0
}

pub unsafe fn dev_pm_domain_set(dev: *mut device, pd: *mut dev_pm_domain) {
    if (*dev).pm_domain == pd { return; }
    WARN(!pd.is_null() && device_is_bound(dev), b"PM domains can only be changed for unbound devices\0".as_ptr() as *const i8);
    (*dev).pm_domain = pd;
    device_pm_check_callbacks(dev);
}

pub unsafe fn dev_pm_domain_set_performance_state(dev: *mut device, state: u32) -> i32 {
    if !(*dev).pm_domain.is_null() && !(*(*dev).pm_domain).set_performance_state.is_none() {
        return ((*(*dev).pm_domain).set_performance_state.unwrap())(dev, state);
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
