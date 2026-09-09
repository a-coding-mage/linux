// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/alpha/kernel/pci.c
 *
 * Rust source-level translation of the original implementation.
 */

// C includes and declarations supplied by the surrounding kernel are external
// dependencies of this translation.

pub static PCI_IO_NAMES: [&'static str; 8] = [
    "PCI IO bus 0", "PCI IO bus 1", "PCI IO bus 2", "PCI IO bus 3",
    "PCI IO bus 4", "PCI IO bus 5", "PCI IO bus 6", "PCI IO bus 7",
];
pub static PCI_MEM_NAMES: [&'static str; 8] = [
    "PCI mem bus 0", "PCI mem bus 1", "PCI mem bus 2", "PCI mem bus 3",
    "PCI mem bus 4", "PCI mem bus 5", "PCI mem bus 6", "PCI mem bus 7",
];
pub static PCI_HAE0_NAME: &[u8] = b"HAE0\0";

pub static mut hose_head: *mut pci_controller = core::ptr::null_mut();
pub static mut hose_tail: *mut *mut pci_controller = unsafe { &raw mut hose_head };
pub static mut pci_isa_hose: *mut pci_controller = core::ptr::null_mut();

unsafe fn quirk_isa_bridge(dev: *mut pci_dev) {
    (*dev).class = PCI_CLASS_BRIDGE_ISA << 8;
}
// DECLARE_PCI_FIXUP_HEADER(PCI_VENDOR_ID_INTEL, PCI_DEVICE_ID_INTEL_82378,
//                          quirk_isa_bridge);

unsafe fn quirk_cypress(dev: *mut pci_dev) {
    if (*dev).class >> 8 == PCI_CLASS_STORAGE_IDE {
        (*dev).resource.add(2).as_mut().unwrap().start = 0;
        (*dev).resource.add(3).as_mut().unwrap().start = 0;
        (*dev).resource.add(2).as_mut().unwrap().end = 0;
        (*dev).resource.add(3).as_mut().unwrap().end = 0;
        (*dev).resource.add(2).as_mut().unwrap().flags = 0;
        (*dev).resource.add(3).as_mut().unwrap().flags = 0;
        if PCI_FUNC((*dev).devfn) == 2 {
            (*dev).resource.add(0).as_mut().unwrap().start = 0x170;
            (*dev).resource.add(0).as_mut().unwrap().end = 0x177;
            (*dev).resource.add(1).as_mut().unwrap().start = 0x376;
            (*dev).resource.add(1).as_mut().unwrap().end = 0x376;
        }
    }
    if (*dev).class >> 8 == PCI_CLASS_BRIDGE_ISA {
        if __direct_map_base + __direct_map_size >= 0xfff00000 {
            __direct_map_size = 0xfff00000 - __direct_map_base;
        } else {
            let hose = (*dev).sysdata as *mut pci_controller;
            let pci = (*hose).sg_pci;
            if !pci.is_null() && (*pci).dma_base + (*pci).size >= 0xfff00000 {
                (*pci).size = 0xfff00000 - (*pci).dma_base;
            }
        }
    }
}
// DECLARE_PCI_FIXUP_HEADER(PCI_VENDOR_ID_CONTAQ, PCI_DEVICE_ID_CONTAQ_82C693,
//                          quirk_cypress);

unsafe fn pcibios_fixup_final(dev: *mut pci_dev) {
    let class = (*dev).class >> 8;
    if class == PCI_CLASS_BRIDGE_ISA || class == PCI_CLASS_BRIDGE_EISA {
        (*dev).dma_mask = MAX_ISA_DMA_ADDRESS - 1;
        isa_bridge = dev;
    }
}
// DECLARE_PCI_FIXUP_FINAL(PCI_ANY_ID, PCI_ANY_ID, pcibios_fixup_final);

pub unsafe fn pcibios_align_resource(
    data: *mut core::ffi::c_void, res: *const resource, _empty_res: *const resource,
    size: resource_size_t, align: resource_size_t,
) -> resource_size_t {
    let dev = data as *mut pci_dev;
    let hose = (*dev).sysdata as *mut pci_controller;
    let mut start = (*res).start;
    if (*res).flags & IORESOURCE_IO != 0 {
        if start - (*(*hose).io_space).start < PCIBIOS_MIN_IO {
            start = PCIBIOS_MIN_IO + (*(*hose).io_space).start;
        }
        if start & 0x300 != 0 { start = (start + 0x3ff) & !0x3ff; }
    } else if (*res).flags & IORESOURCE_MEM != 0 {
        if start - (*(*hose).mem_space).start < PCIBIOS_MIN_MEM {
            start = PCIBIOS_MIN_MEM + (*(*hose).mem_space).start;
        }
        let alignto = core::cmp::max(0x1000, align);
        start = ALIGN(start, alignto);
        if (*hose).sparse_mem_base != 0 && size <= 7 * 16 * MB {
            if ((start / (16 * MB)) & 7) == 0 {
                start &= !(128 * MB - 1); start += 16 * MB;
                start = ALIGN(start, alignto);
            }
            if start / (128 * MB) != (start + size - 1) / (128 * MB) {
                start &= !(128 * MB - 1); start += (128 + 16) * MB;
                start = ALIGN(start, alignto);
            }
        }
    }
    start
}

const KB: resource_size_t = 1024;
const MB: resource_size_t = 1024 * KB;
const GB: resource_size_t = 1024 * MB;

pub unsafe fn pcibios_init() -> i32 {
    if let Some(init) = alpha_mv.init_pci { init(); }
    0
}
// subsys_initcall(pcibios_init);

#[cfg(ALPHA_RESTORE_SRM_SETUP)]
struct pdev_srm_saved_conf { next: *mut pdev_srm_saved_conf, dev: *mut pci_dev }
#[cfg(ALPHA_RESTORE_SRM_SETUP)]
static mut srm_saved_configs: *mut pdev_srm_saved_conf = core::ptr::null_mut();

#[cfg(ALPHA_RESTORE_SRM_SETUP)]
unsafe fn pdev_save_srm_config(dev: *mut pci_dev) {
    static mut printed: i32 = 0;
    if !alpha_using_srm || pci_has_flag(PCI_PROBE_ONLY) { return; }
    if printed == 0 { printk(KERN_INFO, "pci: enabling save/restore of SRM state\0" as *const _); printed = 1; }
    let tmp = kmalloc_obj::<pdev_srm_saved_conf>();
    if tmp.is_null() { printk(KERN_ERR, "%s: kmalloc() failed!\0" as *const _, __func__); return; }
    (*tmp).next = srm_saved_configs; (*tmp).dev = dev; pci_save_state(dev); srm_saved_configs = tmp;
}
#[cfg(not(ALPHA_RESTORE_SRM_SETUP))]
unsafe fn pdev_save_srm_config(_dev: *mut pci_dev) {}

pub unsafe fn pci_restore_srm_config() {
    if pci_has_flag(PCI_PROBE_ONLY) { return; }
    #[cfg(ALPHA_RESTORE_SRM_SETUP)]
    for mut tmp in core::iter::successors(Some(srm_saved_configs), |p| if p.is_null() { None } else { Some((**p).next) }) {
        if !tmp.is_null() { pci_restore_state((*tmp).dev); }
    }
}

pub unsafe fn pcibios_fixup_bus(bus: *mut pci_bus) {
    let dev = (*bus).self_;
    if pci_has_flag(PCI_PROBE_ONLY) && !dev.is_null() && (*dev).class >> 8 == PCI_CLASS_BRIDGE_PCI { pci_read_bridge_bases(bus); }
    list_for_each_entry!(dev, &(*bus).devices, bus_list) { pdev_save_srm_config(dev); }
}

pub unsafe fn pcibios_set_master(dev: *mut pci_dev) {
    let mut lat: u8 = 0; pci_read_config_byte(dev, PCI_LATENCY_TIMER, &mut lat);
    if lat >= 16 { return; }
    printk!("PCI: Setting latency timer of device %s to 64\0", pci_name(dev));
    pci_write_config_byte(dev, PCI_LATENCY_TIMER, 64);
}

pub unsafe fn pcibios_claim_one_bus(b: *mut pci_bus) {
    let mut dev: *mut pci_dev = core::ptr::null_mut();
    let mut child_bus: *mut pci_bus = core::ptr::null_mut();
    list_for_each_entry!(dev, &(*b).devices, bus_list) {
        let mut r: *mut resource = core::ptr::null_mut(); let mut i: i32 = 0;
        pci_dev_for_each_resource!(dev, r, i) {
            if !(*r).parent.is_null() || (*r).start == 0 || (*r).flags == 0 { continue; }
            if pci_has_flag(PCI_PROBE_ONLY) || (*r).flags & IORESOURCE_PCI_FIXED != 0 {
                if pci_claim_resource(dev, i) == 0 { continue; }
                pci_claim_bridge_resource(dev, i);
            }
        }
    }
    list_for_each_entry!(child_bus, &(*b).children, node) { pcibios_claim_one_bus(child_bus); }
}

unsafe fn pcibios_claim_console_setup() {
    let mut b: *mut pci_bus = core::ptr::null_mut();
    list_for_each_entry!(b, &pci_root_buses, node) { pcibios_claim_one_bus(b); }
}

pub unsafe fn common_init_pci() {
    let mut hose = hose_head; let mut next_busno: i32 = 0; let mut need_domain_info = 0;
    while !hose.is_null() {
        let sg_base = if !(*hose).sg_pci.is_null() { (*(*hose).sg_pci).dma_base } else { !0u32 };
        let pci_mem_end = core::cmp::min(__direct_map_base as u32, sg_base) - 1;
        let end = (*(*hose).mem_space).start + pci_mem_end as resource_size_t;
        if (*(*hose).mem_space).end > end { (*(*hose).mem_space).end = end; }
        let mut resources = list_head::default(); INIT_LIST_HEAD(&mut resources);
        pci_add_resource_offset(&mut resources, (*hose).io_space, (*(*hose).io_space).start);
        pci_add_resource_offset(&mut resources, (*hose).mem_space, (*(*hose).mem_space).start);
        let bridge = pci_alloc_host_bridge(0); if bridge.is_null() { hose = (*hose).next; continue; }
        list_splice_init(&mut resources, &mut (*bridge).windows); (*bridge).dev.parent = core::ptr::null_mut();
        (*bridge).sysdata = hose as *mut _; (*bridge).busnr = next_busno; (*bridge).ops = alpha_mv.pci_ops;
        (*bridge).swizzle_irq = alpha_mv.pci_swizzle; (*bridge).map_irq = alpha_mv.pci_map_irq;
        if pci_scan_root_bus_bridge(bridge) != 0 { pci_free_host_bridge(bridge); hose = (*hose).next; continue; }
        (*hose).bus = (*bridge).bus; (*hose).need_domain_info = need_domain_info;
        next_busno = (*(*hose).bus).busn_res.end + 1; if next_busno > 224 { next_busno = 0; need_domain_info = 1; }
        hose = (*hose).next;
    }
    pcibios_claim_console_setup(); pci_assign_unassigned_resources(); hose = hose_head;
    while !hose.is_null() { if !(*hose).bus.is_null() { pci_bus_add_devices((*hose).bus); } hose = (*hose).next; }
}

pub unsafe fn alloc_pci_controller() -> *mut pci_controller {
    let hose = memblock_alloc_or_panic(core::mem::size_of::<pci_controller>(), SMP_CACHE_BYTES) as *mut pci_controller;
    (*hose_tail) = hose; hose_tail = &mut (*hose).next; hose
}
pub unsafe fn alloc_resource() -> *mut resource { memblock_alloc_or_panic(core::mem::size_of::<resource>(), SMP_CACHE_BYTES) as *mut resource }

pub unsafe fn pciconfig_iobase(which: i64, bus: u64, dfn: u64) -> i64 {
    let mut hose: *mut pci_controller = core::ptr::null_mut();
    if which & IOBASE_FROM_HOSE != 0 { hose = hose_head; while !hose.is_null() && (*hose).index != bus { hose = (*hose).next; } if hose.is_null() { return -ENODEV; } }
    else if bus == 0 && dfn == 0 { hose = pci_isa_hose; }
    else { let dev = pci_get_domain_bus_and_slot(0, bus, dfn); if dev.is_null() { return -ENODEV; } hose = (*dev).sysdata as *mut _; pci_dev_put(dev); }
    match which & !IOBASE_FROM_HOSE { IOBASE_HOSE => (*hose).index as i64, IOBASE_SPARSE_MEM => (*hose).sparse_mem_base as i64, IOBASE_DENSE_MEM => (*hose).dense_mem_base as i64, IOBASE_SPARSE_IO => (*hose).sparse_io_base as i64, IOBASE_DENSE_IO => (*hose).dense_io_base as i64, IOBASE_ROOT_BUS => (*(*hose).bus).number as i64, _ => -EOPNOTSUPP }
}

pub unsafe fn pci_iounmap(_dev: *mut pci_dev, addr: *mut core::ffi::c_void) { if __is_mmio(addr) { iounmap(addr); } }
pub static mut isa_bridge: *mut pci_dev = core::ptr::null_mut();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
