// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/alpha/kernel/core_irongate.c
 *
 * Based on code written by David A. Rusling (david.rusling@reo.mts.dec.com).
 *
 * Copyright (C) 1999 Alpha Processor, Inc.,
 * (David Daniel, Stig Telfer, Soohoon Lee)
 *
 * Code common to all IRONGATE core logic chips.
 */

// Dependencies supplied by the surrounding kernel implementation are intentionally external.

pub static mut IronECC: *mut igcsr32 = core::ptr::null_mut();

unsafe fn mk_conf_addr(
    pbus: *mut pci_bus,
    device_fn: libc::c_uint,
    where_: libc::c_int,
    pci_addr: *mut libc::c_ulong,
    type1: *mut libc::c_uchar,
) -> libc::c_int {
    let bus: u8 = (*pbus).number;
    *type1 = (bus != 0) as u8;
    let addr = ((bus as libc::c_ulong) << 16)
        | ((device_fn as libc::c_ulong) << 8)
        | (where_ as libc::c_ulong)
        | IRONGATE_CONF;
    *pci_addr = addr;
    0
}

unsafe fn irongate_read_config(
    bus: *mut pci_bus,
    devfn: libc::c_uint,
    where_: libc::c_int,
    size: libc::c_int,
    value: *mut u32,
) -> libc::c_int {
    let mut addr: libc::c_ulong = 0;
    let mut type1: libc::c_uchar = 0;
    if mk_conf_addr(bus, devfn, where_, &mut addr, &mut type1) != 0 {
        return PCIBIOS_DEVICE_NOT_FOUND;
    }
    match size {
        1 => *value = __kernel_ldbu(addr as *const u8) as u32,
        2 => *value = __kernel_ldwu(addr as *const u16) as u32,
        4 => *value = (addr as *const u32).read_volatile(),
        _ => {}
    }
    PCIBIOS_SUCCESSFUL
}

unsafe fn irongate_write_config(
    bus: *mut pci_bus,
    devfn: libc::c_uint,
    where_: libc::c_int,
    size: libc::c_int,
    value: u32,
) -> libc::c_int {
    let mut addr: libc::c_ulong = 0;
    let mut type1: libc::c_uchar = 0;
    if mk_conf_addr(bus, devfn, where_, &mut addr, &mut type1) != 0 {
        return PCIBIOS_DEVICE_NOT_FOUND;
    }
    match size {
        1 => { __kernel_stb(value, addr as *mut u8); mb(); let _ = __kernel_ldbu(addr as *const u8); }
        2 => { __kernel_stw(value, addr as *mut u16); mb(); let _ = __kernel_ldwu(addr as *const u16); }
        4 => { (addr as *mut u32).write_volatile(value); mb(); let _ = (addr as *const u32).read_volatile(); }
        _ => {}
    }
    PCIBIOS_SUCCESSFUL
}

pub static mut irongate_pci_ops: pci_ops = pci_ops {
    read: Some(irongate_read_config),
    write: Some(irongate_write_config),
};

pub unsafe fn irongate_pci_clr_err() -> libc::c_int {
    let mut nmi_ctl: u32 = 0;
    let mut irongate_jd: u32;
    loop {
        irongate_jd = IRONGATE0.read_volatile().stat_cmd;
        printk!("Iron stat_cmd %x\n", irongate_jd);
        IRONGATE0.write_volatile(IrongateRegs { stat_cmd: irongate_jd });
        mb();
        irongate_jd = IRONGATE0.read_volatile().stat_cmd;
        irongate_jd = IronECC.read_volatile();
        printk!("Iron ECC %x\n", irongate_jd);
        IronECC.write_volatile(irongate_jd);
        mb();
        irongate_jd = IronECC.read_volatile();
        nmi_ctl = inb(0x61) as u32;
        nmi_ctl |= 0x0c;
        outb(nmi_ctl as u8, 0x61);
        nmi_ctl &= !0x0c;
        outb(nmi_ctl as u8, 0x61);
        irongate_jd = IronECC.read_volatile();
        if irongate_jd & 0x300 == 0 { break; }
    }
    0
}

const IRONGATE_3GB: libc::c_ulong = 0xc0000000;

unsafe fn albacore_init_arch() {
    let memtop = max_low_pfn << PAGE_SHIFT;
    let mut pci_mem = (memtop + 0x1000000) & !0xffffff;
    let cpu = (hwrpb as *mut u8).add((*hwrpb).processor_offset as usize) as *mut percpu_struct;
    let pal_rev = (*cpu).pal_revision & 0xffff;
    let pal_var = ((*cpu).pal_revision >> 16) & 0xff;
    if alpha_using_srm && (pal_rev < 0x13e || (pal_rev == 0x13e && pal_var < 2)) {
        printk!(KERN_WARNING "WARNING! Upgrade to SRM A5.6-19 or later\n");
    }
    if pci_mem > IRONGATE_3GB { pci_mem = IRONGATE_3GB; }
    (*IRONGATE0).pci_mem = pci_mem;
    alpha_mv.min_mem_address = pci_mem;
    if memtop > pci_mem {
        memblock_reserve(pci_mem, memtop - pci_mem);
        printk!("irongate_init_arch: temporarily reserving region %08lx-%08lx for PCI\n", pci_mem, memtop - 1);
    }
}

unsafe fn irongate_setup_agp() {
    (*IRONGATE0).agpva &= !0xf;
    alpha_agpgart_size = 0;
}

pub unsafe fn irongate_init_arch() {
    let amd761 = ((*IRONGATE0).dev_vendor >> 16) > 0x7006;
    IronECC = if amd761 { &mut (*IRONGATE0).bacsr54_eccms761 } else { &mut (*IRONGATE0).dramms };
    irongate_pci_clr_err();
    if amd761 { albacore_init_arch(); }
    irongate_setup_agp();
    let hose = alloc_pci_controller();
    pci_isa_hose = hose;
    (*hose).io_space = &mut ioport_resource;
    (*hose).mem_space = &mut iomem_resource;
    (*hose).index = 0;
    (*hose).sparse_mem_base = 0;
    (*hose).sparse_io_base = 0;
    (*hose).dense_mem_base = (IRONGATE_MEM & 0xffffffffff) | 0x80000000000;
    (*hose).dense_io_base = (IRONGATE_IO & 0xffffffffff) | 0x80000000000;
    (*hose).sg_isa = core::ptr::null_mut();
    (*hose).sg_pci = core::ptr::null_mut();
    __direct_map_base = 0;
    __direct_map_size = 0xffffffff;
}

pub unsafe fn irongate_ioremap(addr: libc::c_ulong, size: libc::c_ulong) -> *mut core::ffi::c_void {
    if alpha_agpgart_size == 0 { return (addr + IRONGATE_MEM) as *mut _; }
    let gart_bus_addr = ((*IRONGATE0).bar0 as libc::c_ulong) & PCI_BASE_ADDRESS_MEM_MASK;
    if !(addr >= gart_bus_addr && addr + size - 1 < gart_bus_addr + alpha_agpgart_size) {
        return (addr + IRONGATE_MEM) as *mut _;
    }
    let mmio_regs = (((( (*IRONGATE0).bar1 as libc::c_ulong) & PCI_BASE_ADDRESS_MEM_MASK) + IRONGATE_MEM) as *mut u32);
    let gatt_pages = phys_to_virt(*mmio_regs.add(1)) as *mut u32;
    if addr & !PAGE_MASK != 0 { printk!("AGP ioremap failed... addr not page aligned (0x%lx)\n", addr); return (addr + IRONGATE_MEM) as *mut _; }
    let last = addr + size - 1;
    let size = PAGE_ALIGN(last) - addr;
    let area = get_vm_area(size, VM_IOREMAP);
    if area.is_null() { return core::ptr::null_mut(); }
    let mut baddr = addr;
    let mut vaddr = (*area).addr as libc::c_ulong;
    while baddr <= last {
        let cur_gatt = phys_to_virt((gatt_pages[(baddr >> 22) as usize] as libc::c_ulong) & !1) as *mut u32;
        let pte = *cur_gatt[((baddr & 0x003ff000) >> 12) as usize] & !1;
        if __alpha_remap_area_pages(vaddr, pte as libc::c_ulong, PAGE_SIZE, 0) != 0 { printk!("AGP ioremap: FAILED to map...\n"); vfree((*area).addr); return core::ptr::null_mut(); }
        baddr += PAGE_SIZE; vaddr += PAGE_SIZE;
    }
    flush_tlb_all();
    ((*area).addr as libc::c_ulong + (addr & !PAGE_MASK)) as *mut _
}

pub unsafe fn irongate_iounmap(xaddr: *mut core::ffi::c_void) {
    let addr = xaddr as libc::c_ulong;
    if ((addr as libc::c_long) >> 41) == -2 { return; }
    if addr != 0 { vfree((PAGE_MASK & addr) as *mut _); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
