// SPDX-License-Identifier: GPL-2.0
/*
 * AMD L3 cache_disable_{0,1} sysfs handling
 * Documentation/ABI/testing/sysfs-devices-system-cpu
 */

// C dependencies: linux/cacheinfo.h, linux/capability.h, linux/pci.h,
// linux/sysfs.h, asm/amd/nb.h, and "cpu.h".

unsafe fn amd_calc_l3_indices(nb: *mut amd_northbridge) {
    let l3 = &mut (*nb).l3_cache;
    let (mut sc0, mut sc1, sc2, sc3): (u32, u32, u32, u32);
    let mut val: u32 = 0;

    pci_read_config_dword((*nb).misc, 0x1c4, &mut val);

    l3.subcaches[0] = (!(val & BIT(0))) as u32;
    sc0 = l3.subcaches[0];
    l3.subcaches[1] = (!(val & BIT(4))) as u32;
    sc1 = l3.subcaches[1];

    if boot_cpu_data.x86 == 0x15 {
        sc0 = sc0.wrapping_add((!(val & BIT(1))) as u32);
        l3.subcaches[0] = sc0;
        sc1 = sc1.wrapping_add((!(val & BIT(5))) as u32);
        l3.subcaches[1] = sc1;
    }

    sc2 = (!(val & BIT(8))) as u32 + (!(val & BIT(9))) as u32;
    l3.subcaches[2] = sc2;
    sc3 = (!(val & BIT(12))) as u32 + (!(val & BIT(13))) as u32;
    l3.subcaches[3] = sc3;

    l3.indices = (max(max3(sc0, sc1, sc2), sc3) << 10).wrapping_sub(1);
}

unsafe fn amd_get_l3_disable_slot(nb: *mut amd_northbridge, slot: u32) -> i32 {
    let mut reg: u32 = 0;

    pci_read_config_dword((*nb).misc, 0x1bc + slot * 4, &mut reg);
    if reg & (3u32 << 30) != 0 {
        return (reg & 0xfff) as i32;
    }
    -1
}

unsafe fn show_cache_disable(ci: *mut cacheinfo, buf: *mut core::ffi::c_char, slot: u32) -> isize {
    let index = amd_get_l3_disable_slot((*ci).priv_, slot);
    if index >= 0 {
        return sysfs_emit(buf, b"%d\0".as_ptr() as *const _, index);
    }
    sysfs_emit(buf, b"FREE\n\0".as_ptr() as *const _)
}

unsafe extern "C" fn cache_disable_0_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut core::ffi::c_char) -> isize {
    let ci = dev_get_drvdata(dev) as *mut cacheinfo;
    show_cache_disable(ci, buf, 0)
}

unsafe extern "C" fn cache_disable_1_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut core::ffi::c_char) -> isize {
    let ci = dev_get_drvdata(dev) as *mut cacheinfo;
    show_cache_disable(ci, buf, 1)
}

unsafe fn amd_l3_disable_index(nb: *mut amd_northbridge, cpu: i32, slot: u32, mut idx: usize) {
    idx |= BIT(30) as usize;
    for i in 0..4 {
        let mut reg = idx | ((i as usize) << 20);
        if (*nb).l3_cache.subcaches[i] == 0 {
            continue;
        }
        pci_write_config_dword((*nb).misc, 0x1bc + slot * 4, reg as u32);
        wbinvd_on_cpu(cpu);
        reg |= BIT(31) as usize;
        pci_write_config_dword((*nb).misc, 0x1bc + slot * 4, reg as u32);
    }
}

unsafe fn amd_set_l3_disable_slot(nb: *mut amd_northbridge, cpu: i32, slot: u32, index: usize) -> i32 {
    let ret = amd_get_l3_disable_slot(nb, slot);
    if ret >= 0 { return -EEXIST; }
    if index > (*nb).l3_cache.indices as usize { return -EINVAL; }
    if index as i32 == amd_get_l3_disable_slot(nb, (!slot) & 1) { return -EEXIST; }
    amd_l3_disable_index(nb, cpu, slot, index);
    0
}

unsafe fn store_cache_disable(ci: *mut cacheinfo, buf: *const core::ffi::c_char, count: usize, slot: u32) -> isize {
    let nb = (*ci).priv_;
    let mut val: usize = 0;
    if !capable(CAP_SYS_ADMIN) { return -EPERM as isize; }
    let cpu = cpumask_first(&(*ci).shared_cpu_map);
    if kstrtoul(buf, 10, &mut val) < 0 { return -EINVAL as isize; }
    let err = amd_set_l3_disable_slot(nb, cpu, slot, val);
    if err != 0 {
        if err == -EEXIST { pr_warn(b"L3 slot %d in use/index already disabled!\n\0".as_ptr() as *const _, slot); }
        return err as isize;
    }
    count as isize
}

unsafe extern "C" fn cache_disable_0_store(dev: *mut device, _attr: *mut device_attribute, buf: *const core::ffi::c_char, count: usize) -> isize {
    store_cache_disable(dev_get_drvdata(dev) as *mut cacheinfo, buf, count, 0)
}
unsafe extern "C" fn cache_disable_1_store(dev: *mut device, _attr: *mut device_attribute, buf: *const core::ffi::c_char, count: usize) -> isize {
    store_cache_disable(dev_get_drvdata(dev) as *mut cacheinfo, buf, count, 1)
}

unsafe extern "C" fn subcaches_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut core::ffi::c_char) -> isize {
    let ci = dev_get_drvdata(dev) as *mut cacheinfo;
    let cpu = cpumask_first(&(*ci).shared_cpu_map);
    sysfs_emit(buf, b"%x\n\0".as_ptr() as *const _, amd_get_subcaches(cpu))
}

unsafe extern "C" fn subcaches_store(dev: *mut device, _attr: *mut device_attribute, buf: *const core::ffi::c_char, count: usize) -> isize {
    let ci = dev_get_drvdata(dev) as *mut cacheinfo;
    let cpu = cpumask_first(&(*ci).shared_cpu_map);
    let mut val = 0usize;
    if !capable(CAP_SYS_ADMIN) { return -EPERM as isize; }
    if kstrtoul(buf, 16, &mut val) < 0 { return -EINVAL as isize; }
    if amd_set_subcaches(cpu, val) != 0 { return -EINVAL as isize; }
    count as isize
}

// DEVICE_ATTR_RW declarations and attribute-group wiring are supplied by the kernel bindings.
static mut cache_private_group: attribute_group = attribute_group { is_visible: Some(cache_private_attrs_is_visible), attrs: core::ptr::null_mut() };

unsafe extern "C" fn cache_private_attrs_is_visible(kobj: *mut kobject, attr: *mut attribute, _unused: i32) -> umode_t {
    let dev = kobj_to_dev(kobj);
    let ci = dev_get_drvdata(dev) as *mut cacheinfo;
    let mode = (*attr).mode;
    if (*ci).priv_.is_null() { return 0; }
    if attr == &mut dev_attr_subcaches.attr && amd_nb_has_feature(AMD_NB_L3_PARTITIONING) != 0 { return mode; }
    if (attr == &mut dev_attr_cache_disable_0.attr || attr == &mut dev_attr_cache_disable_1.attr) && amd_nb_has_feature(AMD_NB_L3_INDEX_DISABLE) != 0 { return mode; }
    0
}

unsafe fn init_amd_l3_attrs() {
    static mut amd_l3_attrs: *mut *mut attribute = core::ptr::null_mut();
    let mut n = 1usize;
    if !amd_l3_attrs.is_null() { return; }
    if amd_nb_has_feature(AMD_NB_L3_INDEX_DISABLE) != 0 { n += 2; }
    if amd_nb_has_feature(AMD_NB_L3_PARTITIONING) != 0 { n += 1; }
    amd_l3_attrs = kzalloc_objs(n);
    if amd_l3_attrs.is_null() { return; }
    n = 0;
    if amd_nb_has_feature(AMD_NB_L3_INDEX_DISABLE) != 0 { *amd_l3_attrs.add(n) = &mut dev_attr_cache_disable_0.attr; n += 1; *amd_l3_attrs.add(n) = &mut dev_attr_cache_disable_1.attr; n += 1; }
    if amd_nb_has_feature(AMD_NB_L3_PARTITIONING) != 0 { *amd_l3_attrs.add(n) = &mut dev_attr_subcaches.attr; }
    cache_private_group.attrs = amd_l3_attrs;
}

unsafe fn cache_get_priv_group(ci: *mut cacheinfo) -> *const attribute_group {
    let nb = (*ci).priv_;
    if (*ci).level < 3 || nb.is_null() { return core::ptr::null(); }
    if (*nb).l3_cache.indices != 0 { init_amd_l3_attrs(); }
    &cache_private_group
}

unsafe fn amd_init_l3_cache(index: i32) -> *mut amd_northbridge {
    if index < 3 { return core::ptr::null_mut(); }
    let node = topology_amd_node_id(smp_processor_id());
    let nb = node_to_amd_nb(node);
    if !nb.is_null() && (*nb).l3_cache.indices == 0 { amd_calc_l3_indices(nb); }
    nb
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
