// SPDX-License-Identifier: GPL-2.0-only
/*
 * Shared support code for AMD K8 northbridges and derivatives.
 * Copyright 2006 Andi Kleen, SUSE Labs.
 */

// C includes and build-time configuration are supplied by the surrounding kernel.

static mut flush_words: *mut u32 = core::ptr::null_mut();

static amd_nb_misc_ids: [pci_device_id; 9] = [
    pci_device_id { vendor: PCI_VENDOR_ID_AMD, device: PCI_DEVICE_ID_AMD_K8_NB_MISC },
    pci_device_id { vendor: PCI_VENDOR_ID_AMD, device: PCI_DEVICE_ID_AMD_10H_NB_MISC },
    pci_device_id { vendor: PCI_VENDOR_ID_AMD, device: PCI_DEVICE_ID_AMD_15H_NB_F3 },
    pci_device_id { vendor: PCI_VENDOR_ID_AMD, device: PCI_DEVICE_ID_AMD_15H_M10H_F3 },
    pci_device_id { vendor: PCI_VENDOR_ID_AMD, device: PCI_DEVICE_ID_AMD_15H_M30H_NB_F3 },
    pci_device_id { vendor: PCI_VENDOR_ID_AMD, device: PCI_DEVICE_ID_AMD_15H_M60H_NB_F3 },
    pci_device_id { vendor: PCI_VENDOR_ID_AMD, device: PCI_DEVICE_ID_AMD_16H_NB_F3 },
    pci_device_id { vendor: PCI_VENDOR_ID_AMD, device: PCI_DEVICE_ID_AMD_16H_M30H_NB_F3 },
    pci_device_id { vendor: 0, device: 0 },
];

#[used]
pub static amd_nb_bus_dev_ranges: [amd_nb_bus_dev_range; 4] = [
    amd_nb_bus_dev_range { bus_start: 0x00, bus_end: 0x18, devfn_end: 0x20 },
    amd_nb_bus_dev_range { bus_start: 0xff, bus_end: 0x00, devfn_end: 0x20 },
    amd_nb_bus_dev_range { bus_start: 0xfe, bus_end: 0x00, devfn_end: 0x20 },
    amd_nb_bus_dev_range { bus_start: 0, bus_end: 0, devfn_end: 0 },
];

static mut amd_northbridges: amd_northbridge_info = amd_northbridge_info { num: 0, flags: 0, nb: core::ptr::null_mut() };

pub unsafe fn amd_nb_num() -> u16 { amd_northbridges.num }

pub unsafe fn amd_nb_has_feature(feature: u32) -> bool { (amd_northbridges.flags & feature) == feature }

pub unsafe fn node_to_amd_nb(node: i32) -> *mut amd_northbridge {
    if node < amd_northbridges.num as i32 { amd_northbridges.nb.add(node as usize) } else { core::ptr::null_mut() }
}

unsafe fn amd_cache_northbridges() -> i32 {
    if amd_northbridges.num != 0 { return 0; }
    amd_northbridges.num = amd_num_nodes();
    let nb = kzalloc_objs::<amd_northbridge>(amd_northbridges.num as usize);
    if nb.is_null() { return -ENOMEM; }
    amd_northbridges.nb = nb;
    for i in 0..amd_northbridges.num {
        (*node_to_amd_nb(i as i32)).misc = amd_node_get_func(i, 3);
        if (*node_to_amd_nb(i as i32)).misc.is_null() {
            amd_northbridges.num = 0; kfree(nb); return -ENODEV;
        }
        (*node_to_amd_nb(i as i32)).link = amd_node_get_func(i, 4);
    }
    if amd_gart_present() { amd_northbridges.flags |= AMD_NB_GART; }
    if !cpuid_amd_hygon_has_l3_cache() { return 0; }
    if boot_cpu_data.x86 == 0x10 && boot_cpu_data.x86_model >= 0x8 &&
       (boot_cpu_data.x86_model > 0x9 || boot_cpu_data.x86_stepping >= 0x1) {
        amd_northbridges.flags |= AMD_NB_L3_INDEX_DISABLE;
    }
    if boot_cpu_data.x86 == 0x15 { amd_northbridges.flags |= AMD_NB_L3_INDEX_DISABLE; }
    if boot_cpu_data.x86 == 0x15 { amd_northbridges.flags |= AMD_NB_L3_PARTITIONING; }
    0
}

/* Ignores subdevice/subvendor but as far as I can figure out they're useless anyways. */
pub unsafe fn early_is_amd_nb(mut device: u32) -> bool {
    let vendor = device & 0xffff;
    if boot_cpu_data.x86_vendor != X86_VENDOR_AMD && boot_cpu_data.x86_vendor != X86_VENDOR_HYGON { return false; }
    if cpu_feature_enabled(X86_FEATURE_ZEN) { return false; }
    device >>= 16;
    for id in amd_nb_misc_ids.iter() {
        if id.vendor == 0 { break; }
        if vendor == id.vendor && device == id.device { return true; }
    }
    false
}

pub unsafe fn amd_get_mmconfig_range(res: *mut resource) -> *mut resource {
    if boot_cpu_data.x86_vendor != X86_VENDOR_AMD && boot_cpu_data.x86_vendor != X86_VENDOR_HYGON { return core::ptr::null_mut(); }
    let mut msr = 0u64;
    if boot_cpu_data.x86 < 0x10 || rdmsrq_safe(MSR_FAM10H_MMIO_CONF_BASE, &mut msr) != 0 { return core::ptr::null_mut(); }
    if msr & FAM10H_MMIO_CONF_ENABLE == 0 { return core::ptr::null_mut(); }
    let base = msr & (FAM10H_MMIO_CONF_BASE_MASK << FAM10H_MMIO_CONF_BASE_SHIFT);
    let bits = (msr >> FAM10H_MMIO_CONF_BUSRANGE_SHIFT) & FAM10H_MMIO_CONF_BUSRANGE_MASK;
    (*res).flags = IORESOURCE_MEM; (*res).start = base; (*res).end = base + (1u64 << (bits + 20)) - 1; res
}

pub unsafe fn amd_get_subcaches(cpu: i32) -> i32 {
    let link = (*node_to_amd_nb(topology_amd_node_id(cpu))).link; let mut mask = 0u32;
    if !amd_nb_has_feature(AMD_NB_L3_PARTITIONING) { return 0; }
    pci_read_config_dword(link, 0x1d4, &mut mask);
    ((mask >> (4 * cpu_data(cpu).topo.core_id)) & 0xf) as i32
}

pub unsafe fn amd_set_subcaches(cpu: i32, mut mask: u64) -> i32 {
    static mut reset: u32 = 0; static mut ban: u32 = 0;
    let nb = node_to_amd_nb(topology_amd_node_id(cpu)); let mut reg = 0u32;
    if !amd_nb_has_feature(AMD_NB_L3_PARTITIONING) || mask > 0xf { return -EINVAL; }
    if reset == 0 { pci_read_config_dword((*nb).link, 0x1d4, &mut reset); pci_read_config_dword((*nb).misc, 0x1b8, &mut ban); ban &= 0x180000; }
    if mask != 0xf { pci_read_config_dword((*nb).misc, 0x1b8, &mut reg); pci_write_config_dword((*nb).misc, 0x1b8, reg & !0x180000); }
    let cuid = cpu_data(cpu).topo.core_id; mask <<= 4 * cuid; mask |= (0xf ^ (1 << cuid)) << 26;
    pci_write_config_dword((*nb).link, 0x1d4, mask as u32); pci_read_config_dword((*nb).link, 0x1d4, &mut reg);
    if reg == reset { pci_read_config_dword((*nb).misc, 0x1b8, &mut reg); reg &= !0x180000; pci_write_config_dword((*nb).misc, 0x1b8, reg | ban); } 0
}

unsafe fn amd_cache_gart() {
    if !amd_nb_has_feature(AMD_NB_GART) { return; }
    flush_words = kmalloc_array(amd_northbridges.num as usize, core::mem::size_of::<u32>(), GFP_KERNEL);
    if flush_words.is_null() { amd_northbridges.flags &= !AMD_NB_GART; pr_notice!("Cannot initialize GART flush words, GART support disabled\n"); return; }
    for i in 0..amd_northbridges.num { pci_read_config_dword((*node_to_amd_nb(i as i32)).misc, 0x9c, &mut *flush_words.add(i as usize)); }
}

pub unsafe fn amd_flush_garts() {
    if !amd_nb_has_feature(AMD_NB_GART) { return; }
    static mut gart_lock: spinlock_t = spinlock_t::new(); let mut flags = 0ul; let mut flushed = 0;
    spin_lock_irqsave(&mut gart_lock, &mut flags);
    for i in 0..amd_northbridges.num { pci_write_config_dword((*node_to_amd_nb(i as i32)).misc, 0x9c, *flush_words.add(i as usize) | 1); flushed += 1; }
    for i in 0..amd_northbridges.num { loop { let mut w=0; pci_read_config_dword((*node_to_amd_nb(i as i32)).misc, 0x9c, &mut w); if w & 1 == 0 { break; } cpu_relax(); } }
    spin_unlock_irqrestore(&mut gart_lock, flags); if flushed == 0 { pr_notice!("nothing to flush?\n"); }
}

unsafe fn __fix_erratum_688(_info: *mut core::ffi::c_void) { msr_set_bit(0xC0011021, 3); msr_set_bit(0xC0011021, 14); }

unsafe fn fix_erratum_688() {
    if boot_cpu_data.x86 != 0x14 || amd_northbridges.num == 0 { return; }
    let f4 = (*node_to_amd_nb(0)).link; if f4.is_null() { return; } let mut val=0;
    if pci_read_config_dword(f4, 0x164, &mut val) != 0 || val & (1<<2) != 0 { return; }
    on_each_cpu(__fix_erratum_688, core::ptr::null_mut(), 0); pr_info!("x86/cpu/AMD: CPU erratum 688 worked around\n");
}

unsafe fn init_amd_nbs() -> i32 {
    if boot_cpu_data.x86_vendor != X86_VENDOR_AMD && boot_cpu_data.x86_vendor != X86_VENDOR_HYGON { return 0; }
    amd_cache_northbridges(); amd_cache_gart(); fix_erratum_688(); 0
}

// fs_initcall(init_amd_nbs); The initcall registration is supplied by the kernel build.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
