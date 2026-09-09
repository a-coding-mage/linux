// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/alpha/kernel/core_mcpcia.c
 *
 * Based on code written by David A Rusling (david.rusling@reo.mts.dec.com).
 * Code common to all MCbus-PCI Adaptor core logic chipsets
 */

// C headers and build-time DEBUG_CFG macro omitted; their supplied symbols
// remain external dependencies of this translation.

unsafe fn conf_read(addr: usize, _type1: u8, hose: *mut pci_controller) -> u32 {
    let mid = MCPCIA_HOSE2MID((*hose).index);
    let cpu = smp_processor_id();
    let mut flags: usize = 0;
    local_irq_save(&mut flags);
    let stat0 = core::ptr::read_volatile(MCPCIA_CAP_ERR(mid) as *const u32);
    core::ptr::write_volatile(MCPCIA_CAP_ERR(mid) as *mut u32, stat0);
    mb();
    core::ptr::read_volatile(MCPCIA_CAP_ERR(mid) as *const u32);
    mb();
    draina();
    *mcheck_expected(cpu) = 1;
    *mcheck_taken(cpu) = 0;
    *mcheck_extra(cpu) = mid;
    mb();
    let mut value = core::ptr::read_volatile(addr as *const u32);
    mb();
    mb();
    if *mcheck_taken(cpu) != 0 {
        *mcheck_taken(cpu) = 0;
        value = 0xffff_ffff;
        mb();
    }
    *mcheck_expected(cpu) = 0;
    mb();
    local_irq_restore(flags);
    value
}

unsafe fn conf_write(addr: usize, value: u32, _type1: u8, hose: *mut pci_controller) {
    let mid = MCPCIA_HOSE2MID((*hose).index);
    let cpu = smp_processor_id();
    let mut flags: usize = 0;
    local_irq_save(&mut flags);
    let stat0 = core::ptr::read_volatile(MCPCIA_CAP_ERR(mid) as *const u32);
    core::ptr::write_volatile(MCPCIA_CAP_ERR(mid) as *mut u32, stat0);
    mb();
    core::ptr::read_volatile(MCPCIA_CAP_ERR(mid) as *const u32);
    draina();
    *mcheck_expected(cpu) = 1;
    *mcheck_extra(cpu) = mid;
    mb();
    core::ptr::write_volatile(addr as *mut u32, value);
    mb();
    mb();
    core::ptr::read_volatile(MCPCIA_CAP_ERR(mid) as *const u32);
    *mcheck_expected(cpu) = 0;
    mb();
    local_irq_restore(flags);
}

unsafe fn mk_conf_addr(pbus: *mut pci_bus, devfn: u32, where_: i32,
                       hose: *mut pci_controller, pci_addr: *mut usize,
                       type1: *mut u8) -> i32 {
    let mut bus = (*pbus).number as usize;
    *type1 = 1;
    if (*pbus).parent.is_null() { bus = 0; }
    let addr = ((bus << 16) | ((devfn as usize) << 8) | (where_ as usize)) << 5
        | (*hose).config_space_base;
    *pci_addr = addr;
    0
}

unsafe fn mcpcia_read_config(bus: *mut pci_bus, devfn: u32, where_: i32,
                             size: i32, value: *mut u32) -> i32 {
    let hose = (*bus).sysdata as *mut pci_controller;
    let mut addr = 0usize;
    let mut type1 = 0u8;
    if mk_conf_addr(bus, devfn, where_, hose, &mut addr, &mut type1) != 0 {
        return PCIBIOS_DEVICE_NOT_FOUND;
    }
    addr |= (size - 1) as usize * 8;
    let w = conf_read(addr, type1, hose);
    match size {
        1 => *value = __kernel_extbl(w, (where_ & 3) as u32),
        2 => *value = __kernel_extwl(w, (where_ & 3) as u32),
        4 => *value = w,
        _ => {}
    }
    PCIBIOS_SUCCESSFUL
}

unsafe fn mcpcia_write_config(bus: *mut pci_bus, devfn: u32, where_: i32,
                              size: i32, mut value: u32) -> i32 {
    let hose = (*bus).sysdata as *mut pci_controller;
    let mut addr = 0usize;
    let mut type1 = 0u8;
    if mk_conf_addr(bus, devfn, where_, hose, &mut addr, &mut type1) != 0 {
        return PCIBIOS_DEVICE_NOT_FOUND;
    }
    addr |= (size - 1) as usize * 8;
    value = __kernel_insql(value, (where_ & 3) as u32);
    conf_write(addr, value, type1, hose);
    PCIBIOS_SUCCESSFUL
}

#[repr(C)]
pub struct pci_ops { pub read: unsafe fn(*mut pci_bus, u32, i32, i32, *mut u32) -> i32,
                     pub write: unsafe fn(*mut pci_bus, u32, i32, i32, u32) -> i32 }

pub static mut mcpcia_pci_ops: pci_ops = pci_ops { read: mcpcia_read_config, write: mcpcia_write_config };

pub unsafe fn mcpcia_pci_tbi(hose: *mut pci_controller, _start: dma_addr_t, _end: dma_addr_t) {
    wmb();
    core::ptr::write_volatile(MCPCIA_SG_TBIA(MCPCIA_HOSE2MID((*hose).index)) as *mut u32, 0);
    mb();
}

unsafe fn mcpcia_probe_hose(h: i32) -> bool {
    let cpu = smp_processor_id();
    let mid = MCPCIA_HOSE2MID(h);
    mb(); mb(); draina(); wrmces(7);
    *mcheck_expected(cpu) = 2; *mcheck_taken(cpu) = 0; *mcheck_extra(cpu) = mid; mb();
    let mut pci_rev = core::ptr::read_volatile(MCPCIA_REV(mid) as *const u32);
    mb(); mb();
    if *mcheck_taken(cpu) != 0 { *mcheck_taken(cpu) = 0; pci_rev = 0xffff_ffff; mb(); }
    *mcheck_expected(cpu) = 0; mb();
    (pci_rev >> 16) == PCI_CLASS_BRIDGE_HOST
}

unsafe fn mcpcia_new_hose(h: i32) {
    let mid = MCPCIA_HOSE2MID(h);
    let hose = alloc_pci_controller();
    if h == 0 { pci_isa_hose = hose; }
    let io = alloc_resource(); let mem = alloc_resource(); let hae_mem = alloc_resource();
    (*hose).io_space = io; (*hose).mem_space = hae_mem;
    (*hose).sparse_mem_base = MCPCIA_SPARSE(mid) - IDENT_ADDR;
    (*hose).dense_mem_base = MCPCIA_DENSE(mid) - IDENT_ADDR;
    (*hose).sparse_io_base = MCPCIA_IO(mid) - IDENT_ADDR;
    (*hose).dense_io_base = 0; (*hose).config_space_base = MCPCIA_CONF(mid); (*hose).index = h;
    (*io).start = MCPCIA_IO(mid) - MCPCIA_IO_BIAS; (*io).end = (*io).start + 0xffff;
    (*io).name = pci_io_names[h as usize]; (*io).flags = IORESOURCE_IO;
    (*mem).start = MCPCIA_DENSE(mid) - MCPCIA_MEM_BIAS; (*mem).end = (*mem).start + 0xffff_ffff;
    (*mem).name = pci_mem_names[h as usize]; (*mem).flags = IORESOURCE_MEM;
    (*hae_mem).start = (*mem).start; (*hae_mem).end = (*mem).start + MCPCIA_MEM_MASK;
    (*hae_mem).name = pci_hae0_name; (*hae_mem).flags = IORESOURCE_MEM;
    if request_resource(&mut ioport_resource, io) < 0 { printk("Failed to request IO on hose %d\n", h); }
    if request_resource(&mut iomem_resource, mem) < 0 { printk("Failed to request MEM on hose %d\n", h); }
    if request_resource(mem, hae_mem) < 0 { printk("Failed to request HAE_MEM on hose %d\n", h); }
}

unsafe fn mcpcia_pci_clr_err(mid: i32) {
    core::ptr::read_volatile(MCPCIA_CAP_ERR(mid) as *const u32);
    core::ptr::write_volatile(MCPCIA_CAP_ERR(mid) as *mut u32, 0xffff_ffff);
    mb(); core::ptr::read_volatile(MCPCIA_CAP_ERR(mid) as *const u32);
}

unsafe fn mcpcia_startup_hose(hose: *mut pci_controller) {
    let mid = MCPCIA_HOSE2MID((*hose).index);
    mcpcia_pci_clr_err(mid);
    let mut tmp = core::ptr::read_volatile(MCPCIA_CAP_ERR(mid) as *const u32) | 0x0006;
    core::ptr::write_volatile(MCPCIA_CAP_ERR(mid) as *mut u32, tmp); mb();
    tmp = core::ptr::read_volatile(MCPCIA_CAP_ERR(mid) as *const u32);
    (*hose).sg_isa = iommu_arena_new(hose, 0x0080_0000, 0x0080_0000, SMP_CACHE_BYTES);
    (*hose).sg_pci = iommu_arena_new(hose, 0x4000_0000, size_for_memory(0x4000_0000), SMP_CACHE_BYTES);
    __direct_map_base = 0x8000_0000; __direct_map_size = 0x8000_0000;
    core::ptr::write_volatile(MCPCIA_W0_BASE(mid) as *mut u32, (*(*hose).sg_isa).dma_base | 3);
    core::ptr::write_volatile(MCPCIA_W0_MASK(mid) as *mut u32, ((*(*hose).sg_isa).size - 1) & 0xfff0_0000);
    core::ptr::write_volatile(MCPCIA_T0_BASE(mid) as *mut u32, virt_to_phys((*(*hose).sg_isa).ptes) >> 8);
    core::ptr::write_volatile(MCPCIA_W1_BASE(mid) as *mut u32, (*(*hose).sg_pci).dma_base | 3);
    core::ptr::write_volatile(MCPCIA_W1_MASK(mid) as *mut u32, ((*(*hose).sg_pci).size - 1) & 0xfff0_0000);
    core::ptr::write_volatile(MCPCIA_T1_BASE(mid) as *mut u32, virt_to_phys((*(*hose).sg_pci).ptes) >> 8);
    core::ptr::write_volatile(MCPCIA_W2_BASE(mid) as *mut u32, __direct_map_base | 1);
    core::ptr::write_volatile(MCPCIA_W2_MASK(mid) as *mut u32, (__direct_map_size - 1) & 0xfff0_0000);
    core::ptr::write_volatile(MCPCIA_T2_BASE(mid) as *mut u32, 0);
    core::ptr::write_volatile(MCPCIA_W3_BASE(mid) as *mut u32, 0);
    mcpcia_pci_tbi(hose, 0, !0); core::ptr::write_volatile(MCPCIA_HBASE(mid) as *mut u32, 0); mb();
    core::ptr::write_volatile(MCPCIA_HAE_MEM(mid) as *mut u32, 0); mb(); core::ptr::read_volatile(MCPCIA_HAE_MEM(mid) as *const u32);
    core::ptr::write_volatile(MCPCIA_HAE_IO(mid) as *mut u32, 0); mb(); core::ptr::read_volatile(MCPCIA_HAE_IO(mid) as *const u32);
}

pub unsafe fn mcpcia_init_arch() { ioport_resource.end = !0; mcpcia_new_hose(0); }

pub unsafe fn mcpcia_init_hoses() {
    let mut hose_count = 0; let mut h = 0;
    while h < MCPCIA_MAX_HOSES { if mcpcia_probe_hose(h) { if h != 0 { mcpcia_new_hose(h); } hose_count += 1; } h += 1; }
    printk("mcpcia_init_hoses: found %d hoses\n", hose_count);
    let mut hose = hose_head; while !hose.is_null() { mcpcia_startup_hose(hose); hose = (*hose).next; }
}

#[repr(C)] struct IOD_subpacket { base: usize, whoami: u32, rsvd1: u32, pci_rev: u32, cap_ctrl: u32, hae_mem: u32, hae_io: u32, int_ctl: u32, int_reg: u32, int_mask0: u32, int_mask1: u32, mc_err0: u32, mc_err1: u32, cap_err: u32, rsvd2: u32, pci_err1: u32, mdpa_stat: u32, mdpa_syn: u32, mdpb_stat: u32, mdpb_syn: u32, rsvd3: u32, rsvd4: u32, rsvd5: u32 }

unsafe fn mcpcia_print_uncorrectable(logout: *mut el_MCPCIA_uncorrected_frame_mcheck) { let frame = &(*logout).procdata; let mut i = 0; while i < 24 { printk("  paltmp[%d-%d] = %16lx %16lx\n", i, i+1, frame.paltemp[i], frame.paltemp[i+1]); i += 2; } i = 0; while i < 8 { printk("  shadow[%d-%d] = %16lx %16lx\n", i, i+1, frame.shadow[i], frame.shadow[i+1]); i += 2; } printk("  Addr of excepting instruction  = %16lx\n", frame.exc_addr); printk("  Summary of arithmetic traps    = %16lx\n", frame.exc_sum); printk("  Exception mask                 = %16lx\n", frame.exc_mask); printk("  Base address for PALcode       = %16lx\n", frame.pal_base); printk("  Interrupt Status Reg           = %16lx\n", frame.isr); printk("  CURRENT SETUP OF EV5 IBOX      = %16lx\n", frame.icsr); printk("  I-CACHE Reg %s parity error   = %16lx\n", if frame.ic_perr_stat & 0x800 != 0 { "Data" } else { "Tag" }, frame.ic_perr_stat); printk("  D-CACHE error Reg              = %16lx\n", frame.dc_perr_stat); if frame.dc_perr_stat & 2 != 0 { match frame.dc_perr_stat & 0x03c { 8 => printk("    Data error in bank 1\n"), 4 => printk("    Data error in bank 0\n"), 20 => printk("    Tag error in bank 1\n"), 10 => printk("    Tag error in bank 0\n"), _ => {} } } printk("  Effective VA                   = %16lx\n", frame.va); printk("  Reason for D-stream            = %16lx\n", frame.mm_stat); printk("  EV5 SCache address             = %16lx\n", frame.sc_addr); printk("  EV5 SCache TAG/Data parity     = %16lx\n", frame.sc_stat); printk("  EV5 BC_TAG_ADDR                = %16lx\n", frame.bc_tag_addr); printk("  EV5 EI_ADDR: Phys addr of Xfer = %16lx\n", frame.ei_addr); printk("  Fill Syndrome                  = %16lx\n", frame.fill_syndrome); printk("  EI_STAT reg                    = %16lx\n", frame.ei_stat); printk("  LD_LOCK                        = %16lx\n", frame.ld_lock); }

unsafe fn mcpcia_print_system_area(la_ptr: usize) { let frame = &*(la_ptr as *const el_common); let mut iodpp = (la_ptr + frame.sys_offset as usize) as *mut IOD_subpacket; let mut hose = hose_head; while !hose.is_null() { printk("IOD %d Register Subpacket - Bridge Base Address %16lx\n", (*hose).index, (*iodpp).base); printk("  WHOAMI      = %8x\n", (*iodpp).whoami); printk("  PCI_REV     = %8x\n", (*iodpp).pci_rev); printk("  CAP_CTRL    = %8x\n", (*iodpp).cap_ctrl); printk("  HAE_MEM     = %8x\n", (*iodpp).hae_mem); printk("  HAE_IO      = %8x\n", (*iodpp).hae_io); printk("  INT_CTL     = %8x\n", (*iodpp).int_ctl); printk("  INT_REG     = %8x\n", (*iodpp).int_reg); printk("  INT_MASK0   = %8x\n", (*iodpp).int_mask0); printk("  INT_MASK1   = %8x\n", (*iodpp).int_mask1); printk("  MC_ERR0     = %8x\n", (*iodpp).mc_err0); printk("  MC_ERR1     = %8x\n", (*iodpp).mc_err1); printk("  CAP_ERR     = %8x\n", (*iodpp).cap_err); printk("  PCI_ERR1    = %8x\n", (*iodpp).pci_err1); printk("  MDPA_STAT   = %8x\n", (*iodpp).mdpa_stat); printk("  MDPA_SYN    = %8x\n", (*iodpp).mdpa_syn); printk("  MDPB_STAT   = %8x\n", (*iodpp).mdpb_stat); printk("  MDPB_SYN    = %8x\n", (*iodpp).mdpb_syn); hose = (*hose).next; iodpp = iodpp.add(1); } }

pub unsafe fn mcpcia_machine_check(vector: usize, la_ptr: usize) { let mchk_logout = la_ptr as *mut el_MCPCIA_uncorrected_frame_mcheck; let cpu = smp_processor_id(); let expected = *mcheck_expected(cpu); mb(); mb(); draina(); match expected { 0 => { let mut hose = hose_head; while !hose.is_null() { mcpcia_pci_clr_err(MCPCIA_HOSE2MID((*hose).index)); hose = (*hose).next; } }, 1 => mcpcia_pci_clr_err(*mcheck_extra(cpu)), _ => {} } wrmces(7); mb(); process_mcheck_info(vector, la_ptr, "MCPCIA", expected != 0); if !expected && vector != 0x620 && vector != 0x630 { mcpcia_print_uncorrectable(mchk_logout); mcpcia_print_system_area(la_ptr); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
