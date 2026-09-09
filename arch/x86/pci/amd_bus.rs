// SPDX-License-Identifier: GPL-2.0
// External kernel declarations and types are supplied by the surrounding tree.

const AMD_NB_F0_NODE_ID: u32 = 0x60;
const AMD_NB_F0_UNIT_ID: u32 = 0x64;
const AMD_NB_F1_CONFIG_MAP_REG: u32 = 0xe0;
const RANGE_NUM: usize = 16;
const AMD_NB_F1_CONFIG_MAP_RANGES: usize = 4;
const ENABLE_CF8_EXT_CFG: u64 = 1u64 << 46;

#[repr(C)]
struct amd_hostbridge {
    bus: u32,
    slot: u32,
    device: u32,
}

/*
 * IMPORTANT NOTE:
 * hb_probes[] and early_root_info_init() is in maintenance mode.
 * It only supports K8, Fam10h, Fam11h, and Fam15h_00h-0fh .
 * Future processor will rely on information in ACPI.
 */
static mut hb_probes: [amd_hostbridge; 5] = [
    amd_hostbridge { bus: 0, slot: 0x18, device: 0x1100 }, // K8
    amd_hostbridge { bus: 0, slot: 0x18, device: 0x1200 }, // Family10h
    amd_hostbridge { bus: 0xff, slot: 0, device: 0x1200 }, // Family10h
    amd_hostbridge { bus: 0, slot: 0x18, device: 0x1300 }, // Family11h
    amd_hostbridge { bus: 0, slot: 0x18, device: 0x1600 }, // Family15h
];

unsafe fn find_pci_root_info(node: i32, link: i32) -> *mut pci_root_info {
    let mut info: *mut pci_root_info;
    list_for_each_entry!(info, pci_root_infos, list) {
        if (*info).node == node && (*info).link == link { return info; }
    }
    core::ptr::null_mut()
}

#[inline]
unsafe fn cap_resource(val: u64) -> resource_size_t {
    if val > RESOURCE_SIZE_MAX { RESOURCE_SIZE_MAX } else { val }
}

/**
 * early_root_info_init()
 * called before pcibios_scan_root and pci_scan_bus
 * fills the mp_bus_to_cpumask array based according
 * to the LDT Bus Number Registers found in the northbridge.
 */
unsafe fn early_root_info_init() -> i32 {
    let mut i: i32;
    let mut bus: u32 = 0;
    let mut slot: u32 = 0;
    let mut node: i32;
    let mut link: i32;
    let mut def_node: i32;
    let mut def_link: i32;
    let mut info: *mut pci_root_info;
    let mut reg: u32;
    let mut start: u64;
    let mut end: u64;
    let mut range: [range; RANGE_NUM] = core::mem::zeroed();
    let mut val: u64;
    let mut address: u32;
    let mut found: bool;
    let mut fam10h_mmconf_res: resource = core::mem::zeroed();
    let mut fam10h_mmconf: *mut resource;
    let mut fam10h_mmconf_start: u64;
    let mut fam10h_mmconf_end: u64;

    if !early_pci_allowed() { return -1; }
    found = false;
    i = 0;
    while (i as usize) < hb_probes.len() {
        let id: u32;
        let device: u16;
        let vendor: u16;
        bus = hb_probes[i as usize].bus;
        slot = hb_probes[i as usize].slot;
        id = read_pci_config(bus, slot, 0, PCI_VENDOR_ID);
        vendor = (id & 0xffff) as u16;
        device = ((id >> 16) & 0xffff) as u16;
        if vendor != PCI_VENDOR_ID_AMD && vendor != PCI_VENDOR_ID_HYGON { i += 1; continue; }
        if hb_probes[i as usize].device == device as u32 { found = true; break; }
        i += 1;
    }
    if !found { return 0; }

    /* We extract node numbers here to work around BIOSes that don't supply _PXM. */
    i = 0;
    while (i as usize) < AMD_NB_F1_CONFIG_MAP_RANGES {
        reg = read_pci_config(bus, slot, 1, AMD_NB_F1_CONFIG_MAP_REG + ((i as u32) << 2));
        if (reg & 7) == 3 {
            let min_bus = ((reg >> 16) & 0xff) as i32;
            let max_bus = ((reg >> 24) & 0xff) as i32;
            node = ((reg >> 4) & 7) as i32;
            link = ((reg >> 8) & 3) as i32;
            alloc_pci_root_info(min_bus, max_bus, node, link);
        }
        i += 1;
    }
    if boot_cpu_data.x86 > 0x11 { return 0; }
    reg = read_pci_config(bus, slot, 0, AMD_NB_F0_NODE_ID); def_node = ((reg >> 8) & 7) as i32;
    reg = read_pci_config(bus, slot, 0, AMD_NB_F0_UNIT_ID); def_link = ((reg >> 8) & 3) as i32;
    core::ptr::write_bytes(range.as_mut_ptr(), 0, RANGE_NUM);
    add_range(range.as_mut_ptr(), RANGE_NUM, 0, 0, 0xffff + 1);
    for j in 0..4 {
        reg = read_pci_config(bus, slot, 1, 0xc0 + ((j as u32) << 3)); if (reg & 3) == 0 { continue; }
        start = (reg & 0xfff000) as u64; reg = read_pci_config(bus, slot, 1, 0xc4 + ((j as u32) << 3));
        node = (reg & 7) as i32; link = ((reg >> 4) & 3) as i32; end = ((reg & 0xfff000) | 0xfff) as u64;
        info = find_pci_root_info(node, link); if info.is_null() { continue; }
        if end > 0xffff { end = 0xffff; }
        update_res(info, start, end, IORESOURCE_IO, 1); subtract_range(range.as_mut_ptr(), RANGE_NUM, start, end + 1);
    }
    info = find_pci_root_info(def_node, def_link);
    if !info.is_null() { for j in 0..RANGE_NUM { if range[j].end != 0 { update_res(info, range[j].start, range[j].end - 1, IORESOURCE_IO, 1); } } }

    core::ptr::write_bytes(range.as_mut_ptr(), 0, RANGE_NUM);
    end = cap_resource((0xfdu64 << 32) - 1); end += 1; add_range(range.as_mut_ptr(), RANGE_NUM, 0, 0, end);
    address = MSR_K8_TOP_MEM1; rdmsrq(address, &mut val); end = val & 0xffffff800000;
    if end < (1u64 << 32) { subtract_range(range.as_mut_ptr(), RANGE_NUM, 0, end); }
    fam10h_mmconf = amd_get_mmconfig_range(&mut fam10h_mmconf_res);
    if !fam10h_mmconf.is_null() { fam10h_mmconf_start = (*fam10h_mmconf).start; fam10h_mmconf_end = (*fam10h_mmconf).end; subtract_range(range.as_mut_ptr(), RANGE_NUM, fam10h_mmconf_start, fam10h_mmconf_end + 1); }
    else { fam10h_mmconf_start = 0; fam10h_mmconf_end = 0; }
    for j in 0..8 {
        reg = read_pci_config(bus, slot, 1, 0x80 + ((j as u32) << 3)); if (reg & 3) == 0 { continue; }
        start = ((reg & 0xffffff00) as u64) << 8; reg = read_pci_config(bus, slot, 1, 0x84 + ((j as u32) << 3));
        node = (reg & 7) as i32; link = ((reg >> 4) & 3) as i32; end = (((reg & 0xffffff00) as u64) << 8) | 0xffff;
        info = find_pci_root_info(node, link); if info.is_null() { continue; }
        if fam10h_mmconf_end != 0 {
            let mut endx = 0u64;
            if start >= fam10h_mmconf_start && start <= fam10h_mmconf_end { start = fam10h_mmconf_end + 1; }
            if end >= fam10h_mmconf_start && end <= fam10h_mmconf_end { end = fam10h_mmconf_start - 1; }
            if start < fam10h_mmconf_start && end > fam10h_mmconf_end { endx = fam10h_mmconf_start - 1; update_res(info, start, endx, IORESOURCE_MEM, 0); subtract_range(range.as_mut_ptr(), RANGE_NUM, start, endx + 1); start = fam10h_mmconf_end + 1; }
            if start > end { continue; }
        }
        update_res(info, cap_resource(start), cap_resource(end), IORESOURCE_MEM, 1); subtract_range(range.as_mut_ptr(), RANGE_NUM, start, end + 1);
    }
    address = MSR_AMD64_SYSCFG; rdmsrq(address, &mut val);
    if val & (1u64 << 21) != 0 { address = MSR_K8_TOP_MEM2; rdmsrq(address, &mut val); end = val & 0xffffff800000; subtract_range(range.as_mut_ptr(), RANGE_NUM, 1u64 << 32, end); }
    info = find_pci_root_info(def_node, def_link);
    if !info.is_null() { for j in 0..RANGE_NUM { if range[j].end != 0 { update_res(info, cap_resource(range[j].start), cap_resource(range[j].end - 1), IORESOURCE_MEM, 1); } } }
    list_for_each_entry!(info, pci_root_infos, list) {
        let busnum = (*info).busn.start;
        let mut root_res: *mut pci_root_res;
        list_for_each_entry!(root_res, (*info).resources, list) {
            let _ = (busnum, (*root_res).res);
        }
    }
    0
}

unsafe fn amd_bus_cpu_online(_cpu: u32) -> i32 {
    let mut reg: u64 = 0; rdmsrq(MSR_AMD64_NB_CFG, &mut reg);
    if reg & ENABLE_CF8_EXT_CFG == 0 { wrmsrq(MSR_AMD64_NB_CFG, reg | ENABLE_CF8_EXT_CFG); }
    0
}

unsafe fn pci_enable_pci_io_ecs() {
    #[cfg(CONFIG_AMD_NB)]
    {
        let mut n = 0u32; let mut i = 0usize;
        while n == 0 && amd_nb_bus_dev_ranges[i].dev_limit != 0 {
            let bus = amd_nb_bus_dev_ranges[i].bus; let mut slot = amd_nb_bus_dev_ranges[i].dev_base; let limit = amd_nb_bus_dev_ranges[i].dev_limit;
            while slot < limit { let mut val = read_pci_config(bus, slot, 3, 0); if early_is_amd_nb(val) { val = read_pci_config(bus, slot, 3, 0x8c); if val & (ENABLE_CF8_EXT_CFG >> 32) == 0 { write_pci_config(bus, slot, 3, 0x8c, val | (ENABLE_CF8_EXT_CFG >> 32)); } n += 1; } slot += 1; }
            i += 1;
        }
    }
}

unsafe fn pci_io_ecs_init() -> i32 {
    if boot_cpu_data.x86 < 0x10 { return 0; }
    if early_pci_allowed() { pci_enable_pci_io_ecs(); }
    let ret = cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, c"pci/amd_bus:online", amd_bus_cpu_online, None);
    WARN_ON(ret < 0); pci_probe |= PCI_HAS_IO_ECS; 0
}

unsafe fn amd_postcore_init() -> i32 {
    if boot_cpu_data.x86_vendor != X86_VENDOR_AMD && boot_cpu_data.x86_vendor != X86_VENDOR_HYGON { return 0; }
    early_root_info_init(); pci_io_ecs_init(); 0
}

// postcore_initcall(amd_postcore_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
