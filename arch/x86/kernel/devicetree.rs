// SPDX-License-Identifier: GPL-2.0
/*
 * Architecture specific OF callbacks.
 */

// Kernel dependencies supplied by other translation units are intentionally
// referenced here rather than reimplemented.

#[no_mangle]
pub static mut initial_dtb: u64 = 0;
#[no_mangle]
pub static mut cmd_line: [core::ffi::c_char; COMMAND_LINE_SIZE] = [0; COMMAND_LINE_SIZE];
#[no_mangle]
pub static mut of_ioapic: i32 = 0;

pub unsafe fn add_dtb(data: u64) {
    initial_dtb = data + core::mem::offset_of!(setup_data, data) as u64;
}

/* CE4100 ids. Will be moved to machine_device_initcall() once we have it. */
static mut ce4100_ids: [of_device_id; 4] = [
    of_device_id { compatible: b"intel,ce4100-cp\0".as_ptr() as *const _ },
    of_device_id { compatible: b"isa\0".as_ptr() as *const _ },
    of_device_id { compatible: b"pci\0".as_ptr() as *const _ },
    of_device_id { compatible: core::ptr::null() },
];

unsafe fn add_bus_probe() -> i32 {
    if !of_have_populated_dt() {
        return 0;
    }
    of_platform_bus_probe(core::ptr::null_mut(), ce4100_ids.as_mut_ptr(), core::ptr::null_mut())
}

#[cfg(feature = "CONFIG_PCI")]
pub unsafe fn pcibios_get_phb_of_node(bus: *mut pci_bus) -> *mut device_node {
    let mut np: *mut device_node = core::ptr::null_mut();
    for_each_node_by_type!(np, b"pci\0".as_ptr() as *const _) {
        let mut bus_min: u32 = 0;
        let prop = of_get_property(np, b"bus-range\0".as_ptr() as *const _, core::ptr::null_mut());
        if prop.is_null() { continue; }
        bus_min = be32_to_cpup(prop as *const u32);
        if (*bus).number == bus_min { return np; }
    }
    core::ptr::null_mut()
}

#[cfg(feature = "CONFIG_PCI")]
unsafe fn x86_of_pci_irq_enable(dev: *mut pci_dev) -> i32 {
    let mut pin: u8 = 0;
    let ret = pci_read_config_byte(dev, PCI_INTERRUPT_PIN, &mut pin);
    if ret != 0 { return pcibios_err_to_errno(ret); }
    if pin == 0 { return 0; }
    let virq = of_irq_parse_and_map_pci(dev, 0, 0);
    if virq == 0 { return -EINVAL; }
    (*dev).irq = virq;
    0
}

#[cfg(feature = "CONFIG_PCI")]
unsafe fn x86_of_pci_irq_disable(_dev: *mut pci_dev) {}

#[cfg(feature = "CONFIG_PCI")]
pub unsafe fn x86_of_pci_init() {
    pcibios_enable_irq = Some(x86_of_pci_irq_enable);
    pcibios_disable_irq = Some(x86_of_pci_irq_disable);
}

unsafe fn dtb_setup_hpet() {
    #[cfg(feature = "CONFIG_HPET_TIMER")]
    {
        let dn = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), b"intel,ce4100-hpet\0".as_ptr() as *const _);
        if dn.is_null() { return; }
        let mut r = core::mem::zeroed::<resource>();
        let ret = of_address_to_resource(dn, 0, &mut r);
        if ret != 0 { WARN_ON!(true); return; }
        hpet_address = r.start;
    }
}

#[cfg(all(feature = "CONFIG_X86_64", feature = "CONFIG_SMP"))]
const WAKEUP_MAILBOX_SIZE: u64 = 0x1000;
#[cfg(all(feature = "CONFIG_X86_64", feature = "CONFIG_SMP"))]
const WAKEUP_MAILBOX_ALIGN: u64 = 0x1000;

unsafe fn dtb_wakeup_mailbox_setup() {
    #[cfg(all(feature = "CONFIG_X86_64", feature = "CONFIG_SMP"))]
    {
        let node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), b"intel,wakeup-mailbox\0".as_ptr() as *const _);
        if node.is_null() { return; }
        let mut res = core::mem::zeroed::<resource>();
        if of_address_to_resource(node, 0, &mut res) != 0 { of_node_put(node); return; }
        if res.start & (WAKEUP_MAILBOX_ALIGN - 1) != 0 { of_node_put(node); return; }
        if res.end - res.start + 1 != WAKEUP_MAILBOX_SIZE { of_node_put(node); return; }
        cpu_hotplug_disable_offlining();
        acpi_setup_mp_wakeup_mailbox(res.start);
        of_node_put(node);
    }
    #[cfg(not(all(feature = "CONFIG_X86_64", feature = "CONFIG_SMP")))]
    { let _ = -EOPNOTSUPP; }
}

#[cfg(feature = "CONFIG_X86_LOCAL_APIC")]
unsafe fn dtb_cpu_setup() {
    let mut dn: *mut device_node = core::ptr::null_mut();
    for_each_of_cpu_node!(dn) {
        let apic_id = of_get_cpu_hwid(dn, 0);
        if apic_id == !0u32 { pr_warn!(b"%pOF: missing local APIC ID\n", dn); continue; }
        topology_register_apic(apic_id, CPU_ACPIID_INVALID, true);
        set_apicid_to_node(apic_id, of_node_to_nid(dn));
    }
}

#[cfg(feature = "CONFIG_X86_LOCAL_APIC")]
unsafe fn dtb_lapic_setup() {
    let dn = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), b"intel,ce4100-lapic\0".as_ptr() as *const _);
    let mut r = core::mem::zeroed::<resource>();
    let mut lapic_addr = APIC_DEFAULT_PHYS_BASE;
    if !dn.is_null() { let ret = of_address_to_resource(dn, 0, &mut r); if WARN_ON!(ret != 0) { return; } lapic_addr = r.start; }
    if !boot_cpu_has(X86_FEATURE_APIC) { if !apic_force_enable(lapic_addr) { return; } } else { register_lapic_address(lapic_addr); }
    smp_found_config = 1;
    pic_mode = !of_property_read_bool(dn, b"intel,virtual-wire-mode\0".as_ptr() as *const _);
    pr_info!(b"%s compatibility mode.\n", if pic_mode { b"IMCR and PIC\0".as_ptr() } else { b"Virtual Wire\0".as_ptr() });
}

#[cfg(feature = "CONFIG_X86_IO_APIC")]
static mut ioapic_id: u32 = 0;

#[repr(C)]
struct of_ioapic_type { out_type: u32, is_level: u32, active_low: u32 }

#[cfg(feature = "CONFIG_X86_IO_APIC")]
static mut of_ioapic_type: [of_ioapic_type; 4] = [
    of_ioapic_type { out_type: IRQ_TYPE_EDGE_FALLING, is_level: 0, active_low: 1 },
    of_ioapic_type { out_type: IRQ_TYPE_LEVEL_HIGH, is_level: 1, active_low: 0 },
    of_ioapic_type { out_type: IRQ_TYPE_LEVEL_LOW, is_level: 1, active_low: 1 },
    of_ioapic_type { out_type: IRQ_TYPE_EDGE_RISING, is_level: 0, active_low: 0 },
];

#[cfg(feature = "CONFIG_X86_IO_APIC")]
unsafe fn dt_irqdomain_alloc(domain: *mut irq_domain, virq: u32, nr_irqs: u32, arg: *mut core::ffi::c_void) -> i32 {
    let fwspec = arg as *mut irq_fwspec;
    if WARN_ON!((*fwspec).param_count < 2) { return -EINVAL; }
    let type_index = (*fwspec).param[1] as usize;
    if type_index >= of_ioapic_type.len() { return -EINVAL; }
    let it = &of_ioapic_type[type_index];
    let mut tmp = core::mem::zeroed::<irq_alloc_info>();
    ioapic_set_alloc_attr(&mut tmp, NUMA_NO_NODE, it.is_level, it.active_low);
    tmp.devid = mpc_ioapic_id(mp_irqdomain_ioapic_idx(domain));
    tmp.ioapic.pin = (*fwspec).param[0];
    mp_irqdomain_alloc(domain, virq, nr_irqs, &mut tmp)
}

#[cfg(feature = "CONFIG_X86_IO_APIC")]
unsafe fn dtb_add_ioapic(dn: *mut device_node) {
    let mut r = core::mem::zeroed::<resource>();
    let mut cfg = ioapic_domain_cfg { type_: IOAPIC_DOMAIN_DYNAMIC, ops: &ioapic_irq_domain_ops, dev: dn };
    if of_address_to_resource(dn, 0, &mut r) != 0 { pr_err!(b"Can't obtain address from device node %pOF.\n", dn); return; }
    mp_register_ioapic({ ioapic_id += 1; ioapic_id }, r.start, gsi_top, &mut cfg);
}

unsafe fn dtb_ioapic_setup() {
    #[cfg(feature = "CONFIG_X86_IO_APIC")]
    {
        let mut dn: *mut device_node = core::ptr::null_mut();
        for_each_compatible_node!(dn, core::ptr::null_mut(), b"intel,ce4100-ioapic\0".as_ptr() as *const _) { dtb_add_ioapic(dn); }
        if nr_ioapics != 0 { of_ioapic = 1; return; }
        pr_err!(b"Error: No information about IO-APIC in OF.\n");
    }
}

unsafe fn dtb_apic_setup() {
    #[cfg(feature = "CONFIG_X86_LOCAL_APIC")] { dtb_lapic_setup(); dtb_cpu_setup(); }
    dtb_ioapic_setup();
}

unsafe fn x86_dtb_parse_smp_config() {
    if !of_have_populated_dt() { return; }
    dtb_setup_hpet(); dtb_apic_setup(); dtb_wakeup_mailbox_setup();
}

pub unsafe fn x86_flattree_get_config() {
    #[cfg(feature = "CONFIG_OF_EARLY_FLATTREE")]
    {
        let mut map_len: u32;
        let mut dt: *mut core::ffi::c_void;
        if initial_dtb != 0 {
            map_len = core::cmp::max(PAGE_SIZE - (initial_dtb & !PAGE_MASK), 128) as u32;
            dt = early_memremap(initial_dtb, map_len as usize);
            let size = fdt_totalsize(dt) as u32;
            if map_len < size { early_memunmap(dt, map_len as usize); dt = early_memremap(initial_dtb, size as usize); map_len = size; }
            early_init_dt_verify(dt, __pa(dt));
        }
        unflatten_and_copy_device_tree();
        if initial_dtb != 0 { early_memunmap(dt, map_len as usize); }
    }
    if acpi_disabled && of_have_populated_dt() { x86_init.mpparse.parse_smp_cfg = Some(x86_dtb_parse_smp_config); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
