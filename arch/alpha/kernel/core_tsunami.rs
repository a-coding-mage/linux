// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/alpha/kernel/core_tsunami.c
 *
 * Based on code written by David A. Rusling (david.rusling@reo.mts.dec.com).
 * Code common to all TSUNAMI core logic chips.
 */

// C includes and architecture-provided symbols are supplied by other files.

#[repr(C)]
pub struct SavedConfig {
    pub wsba: [c_ulong; 4],
    pub wsm: [c_ulong; 4],
    pub tba: [c_ulong; 4],
}

#[no_mangle]
pub static mut saved_config: [SavedConfig; 2] = unsafe { core::mem::zeroed() };

// DEBUG_CONFIG is 0 in the original source; DBG_CFG therefore has no effect.

unsafe fn mk_conf_addr(
    pbus: *mut pci_bus,
    device_fn: c_uint,
    where_: c_int,
    pci_addr: *mut c_ulong,
    type1: *mut c_uchar,
) -> c_int {
    let hose = (*pbus).sysdata as *mut pci_controller;
    let mut bus = (*pbus).number as u8;

    // No parent means peer PCI bus.
    if (*pbus).parent.is_null() {
        bus = 0;
    }
    *type1 = (bus != 0) as u8;
    let mut addr = ((bus as c_ulong) << 16)
        | ((device_fn as c_ulong) << 8)
        | (where_ as c_ulong);
    addr |= (*hose).config_space_base;
    *pci_addr = addr;
    0
}

unsafe fn tsunami_read_config(
    bus: *mut pci_bus,
    devfn: c_uint,
    where_: c_int,
    size: c_int,
    value: *mut u32,
) -> c_int {
    let mut addr: c_ulong = 0;
    let mut type1: c_uchar = 0;
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

unsafe fn tsunami_write_config(
    bus: *mut pci_bus,
    devfn: c_uint,
    where_: c_int,
    size: c_int,
    value: u32,
) -> c_int {
    let mut addr: c_ulong = 0;
    let mut type1: c_uchar = 0;
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

#[repr(C)]
pub struct pci_ops { pub read: unsafe fn(*mut pci_bus, c_uint, c_int, c_int, *mut u32) -> c_int, pub write: unsafe fn(*mut pci_bus, c_uint, c_int, c_int, u32) -> c_int }

#[no_mangle]
pub static mut tsunami_pci_ops: pci_ops = pci_ops { read: tsunami_read_config, write: tsunami_write_config };

#[no_mangle]
pub unsafe extern "C" fn tsunami_pci_tbi(hose: *mut pci_controller, start: dma_addr_t, end: dma_addr_t) {
    let pchip = if (*hose).index != 0 { TSUNAMI_pchip1 } else { TSUNAMI_pchip0 };
    let mut csr: *mut c_ulong = &mut (*pchip).tlbia.csr;
    if ((start ^ end) & 0xffff0000) == 0 { csr = &mut (*pchip).tlbiv.csr; }
    let value = (start & 0xffff0000) >> 12;
    *csr = value;
    mb();
    let _ = *csr;
}

#[cfg(NXM_MACHINE_CHECKS_ON_TSUNAMI)]
unsafe fn tsunami_probe_read(vaddr: *volatile c_ulong) -> c_long {
    let cpu = smp_processor_id();
    let s = swpipl(IPL_MCHECK - 1);
    mcheck_taken(cpu) = 0;
    mcheck_expected(cpu) = 1;
    mb();
    let dont_care = *vaddr;
    draina();
    mcheck_expected(cpu) = 0;
    let result = (!mcheck_taken(cpu)) as c_long;
    mcheck_taken(cpu) = 0;
    setipl(s);
    printk(b"dont_care == 0x%lx\n\0".as_ptr() as _, dont_care);
    result
}

#[cfg(not(NXM_MACHINE_CHECKS_ON_TSUNAMI))]
unsafe fn tsunami_probe_read(_addr: *volatile c_ulong) -> c_long { 1 }

#[cfg(NXM_MACHINE_CHECKS_ON_TSUNAMI)]
unsafe fn tsunami_probe_write(vaddr: *mut c_ulong) -> c_long {
    let mut result: c_long = 1;
    (*TSUNAMI_cchip).misc.csr |= 1 << 28;
    let contents = *vaddr;
    *vaddr = 0;
    draina();
    if (*TSUNAMI_cchip).misc.csr & (1 << 28) != 0 {
        let source = ((*TSUNAMI_cchip).misc.csr >> 29) & 7;
        (*TSUNAMI_cchip).misc.csr |= 1 << 28;
        result = 0;
        printk(b"tsunami_probe_write: unit %d at 0x%016lx\n\0".as_ptr() as _, source, vaddr as c_ulong);
    }
    if result != 0 { *vaddr = contents; }
    result
}

unsafe fn tsunami_init_one_pchip(pchip: *mut tsunami_pchip, index: c_int) {
    if tsunami_probe_read(&mut (*pchip).pctl.csr) == 0 { return; }
    let hose = alloc_pci_controller();
    if index == 0 { pci_isa_hose = hose; }
    (*hose).io_space = alloc_resource();
    (*hose).mem_space = alloc_resource();
    (*hose).sparse_mem_base = 0;
    (*hose).sparse_io_base = 0;
    (*hose).dense_mem_base = (TSUNAMI_MEM(index) & 0xffffffffff) | 0x80000000000;
    (*hose).dense_io_base = (TSUNAMI_IO(index) & 0xffffffffff) | 0x80000000000;
    (*hose).config_space_base = TSUNAMI_CONF(index);
    (*hose).index = index;
    (*hose).io_space.as_mut().unwrap().start = TSUNAMI_IO(index) - TSUNAMI_IO_BIAS;
    (*hose).io_space.as_mut().unwrap().end = (*hose).io_space.as_ref().unwrap().start + TSUNAMI_IO_SPACE - 1;
    (*hose).io_space.as_mut().unwrap().name = pci_io_names[index as usize];
    (*hose).io_space.as_mut().unwrap().flags = IORESOURCE_IO;
    (*hose).mem_space.as_mut().unwrap().start = TSUNAMI_MEM(index) - TSUNAMI_MEM_BIAS;
    (*hose).mem_space.as_mut().unwrap().end = (*hose).mem_space.as_ref().unwrap().start + 0xffffffff;
    (*hose).mem_space.as_mut().unwrap().name = pci_mem_names[index as usize];
    (*hose).mem_space.as_mut().unwrap().flags = IORESOURCE_MEM;
    if request_resource(&mut ioport_resource, (*hose).io_space) < 0 { printk(b"Failed to request IO on hose %d\n\0".as_ptr() as _, index); }
    if request_resource(&mut iomem_resource, (*hose).mem_space) < 0 { printk(b"Failed to request MEM on hose %d\n\0".as_ptr() as _, index); }

    for i in 0..4 { saved_config[index as usize].wsba[i] = (*pchip).wsba[i].csr; saved_config[index as usize].wsm[i] = (*pchip).wsm[i].csr; saved_config[index as usize].tba[i] = (*pchip).tba[i].csr; }
    (*hose).sg_isa = iommu_arena_new(hose, 0x00800000, 0x00800000, SMP_CACHE_BYTES);
    (*hose).sg_isa.as_mut().unwrap().align_entry = 4;
    (*hose).sg_pci = iommu_arena_new(hose, 0x40000000, size_for_memory(0x40000000), SMP_CACHE_BYTES);
    (*hose).sg_pci.as_mut().unwrap().align_entry = 4;
    __direct_map_base = 0x80000000; __direct_map_size = 0x80000000;
    (*pchip).wsba[0].csr = (*hose).sg_isa.as_ref().unwrap().dma_base | 3;
    (*pchip).wsm[0].csr = ((*hose).sg_isa.as_ref().unwrap().size - 1) & 0xfff00000;
    (*pchip).tba[0].csr = virt_to_phys((*hose).sg_isa.as_ref().unwrap().ptes);
    (*pchip).wsba[1].csr = (*hose).sg_pci.as_ref().unwrap().dma_base | 3;
    (*pchip).wsm[1].csr = ((*hose).sg_pci.as_ref().unwrap().size - 1) & 0xfff00000;
    (*pchip).tba[1].csr = virt_to_phys((*hose).sg_pci.as_ref().unwrap().ptes);
    (*pchip).wsba[2].csr = 0x80000000 | 1; (*pchip).wsm[2].csr = (0x80000000 - 1) & 0xfff00000; (*pchip).tba[2].csr = 0; (*pchip).wsba[3].csr = 0;
    (*pchip).pctl.csr |= pctl_m_mwin;
    tsunami_pci_tbi(hose, 0, !0);
}

#[no_mangle]
pub unsafe extern "C" fn tsunami_ioportmap(addr: c_ulong) -> *mut c_void { FIXUP_IOADDR_VGA(addr); (addr + TSUNAMI_IO_BIAS) as *mut c_void }
#[no_mangle]
pub unsafe extern "C" fn tsunami_ioremap(addr: c_ulong, _size: c_ulong) -> *mut c_void { FIXUP_MEMADDR_VGA(addr); (addr + TSUNAMI_MEM_BIAS) as *mut c_void }

#[no_mangle]
pub unsafe extern "C" fn tsunami_init_arch() {
    ioport_resource.end = !0;
    tsunami_init_one_pchip(TSUNAMI_pchip0, 0);
    if (*TSUNAMI_cchip).csc.csr & (1 << 14) != 0 { tsunami_init_one_pchip(TSUNAMI_pchip1, 1); }
    find_console_vga_hose();
}

unsafe fn tsunami_kill_one_pchip(pchip: *mut tsunami_pchip, index: c_int) {
    for i in 0..4 { (*pchip).wsba[i].csr = saved_config[index as usize].wsba[i]; (*pchip).wsm[i].csr = saved_config[index as usize].wsm[i]; (*pchip).tba[i].csr = saved_config[index as usize].tba[i]; }
}

#[no_mangle]
pub unsafe extern "C" fn tsunami_kill_arch(_mode: c_int) { tsunami_kill_one_pchip(TSUNAMI_pchip0, 0); if (*TSUNAMI_cchip).csc.csr & (1 << 14) != 0 { tsunami_kill_one_pchip(TSUNAMI_pchip1, 1); } }

unsafe fn tsunami_pci_clr_err_1(pchip: *mut tsunami_pchip) { let _ = (*pchip).perror.csr; (*pchip).perror.csr = 0x040; mb(); let _ = (*pchip).perror.csr; }
unsafe fn tsunami_pci_clr_err() { tsunami_pci_clr_err_1(TSUNAMI_pchip0); if (*TSUNAMI_cchip).csc.csr & (1 << 14) != 0 { tsunami_pci_clr_err_1(TSUNAMI_pchip1); } }

#[no_mangle]
pub unsafe extern "C" fn tsunami_machine_check(vector: c_ulong, la_ptr: c_ulong) {
    mb(); mb(); draina(); tsunami_pci_clr_err(); wrmces(0x7); mb();
    process_mcheck_info(vector, la_ptr, b"TSUNAMI\0".as_ptr() as _, mcheck_expected(smp_processor_id()));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
