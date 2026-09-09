// SPDX-License-Identifier: GPL-2.0
/* Translated from linux/arch/alpha/kernel/core_cia.c. */

// Architecture-specific declarations and constants are supplied by the
// surrounding Alpha kernel translation unit.

type Vip = *mut i32;

unsafe fn mk_conf_addr(bus_dev: *mut pci_bus, device_fn: u32, where_: i32,
                       pci_addr: *mut usize, type1: *mut u8) -> i32 {
    let bus = (*bus_dev).number as usize;
    *type1 = (bus != 0) as u8;
    *pci_addr = (bus << 16) | ((device_fn as usize) << 8) | where_ as usize;
    0
}

unsafe fn conf_read(addr: usize, type1: u8) -> u32 {
    let mut flags: usize = 0;
    let mut stat0: i32;
    let mut value: i32;
    let mut cia_cfg = 0i32;
    local_irq_save(&mut flags);
    stat0 = *(CIA_IOC_CIA_ERR as Vip); *(CIA_IOC_CIA_ERR as Vip) = stat0; mb(); *(CIA_IOC_CIA_ERR as Vip);
    if type1 != 0 { cia_cfg = *(CIA_IOC_CFG as Vip); *(CIA_IOC_CFG as Vip) = (cia_cfg & !3) | 1; mb(); *(CIA_IOC_CFG as Vip); }
    mb(); draina(); *mcheck_expected(0) = 1; *mcheck_taken(0) = 0; mb();
    value = *(addr as Vip); mb(); mb();
    if *mcheck_taken(0) != 0 { *mcheck_taken(0) = 0; value = -1; mb(); }
    *mcheck_expected(0) = 0; mb();
    if type1 != 0 { *(CIA_IOC_CFG as Vip) = cia_cfg; mb(); *(CIA_IOC_CFG as Vip); }
    local_irq_restore(flags);
    value as u32
}

unsafe fn conf_write(addr: usize, value: u32, type1: u8) {
    let mut flags: usize = 0; let mut stat0: i32; let mut cia_cfg = 0i32;
    local_irq_save(&mut flags);
    stat0 = *(CIA_IOC_CIA_ERR as Vip); *(CIA_IOC_CIA_ERR as Vip) = stat0; mb(); *(CIA_IOC_CIA_ERR as Vip);
    if type1 != 0 { cia_cfg = *(CIA_IOC_CFG as Vip); *(CIA_IOC_CFG as Vip) = (cia_cfg & !3) | 1; mb(); *(CIA_IOC_CFG as Vip); }
    mb(); draina(); *mcheck_expected(0) = 1; *mcheck_taken(0) = 0; mb();
    *(addr as Vip) = value as i32; mb(); *(addr as Vip); *mcheck_expected(0) = 0; mb();
    if type1 != 0 { *(CIA_IOC_CFG as Vip) = cia_cfg; mb(); *(CIA_IOC_CFG as Vip); }
    local_irq_restore(flags);
}

unsafe fn cia_read_config(bus: *mut pci_bus, devfn: u32, where_: i32, size: i32, value: *mut u32) -> i32 {
    let mut addr = 0usize; let mut pci_addr = 0usize; let mut type1 = 0u8;
    if mk_conf_addr(bus, devfn, where_, &mut pci_addr, &mut type1) != 0 { return PCIBIOS_DEVICE_NOT_FOUND; }
    let mask = ((size - 1) * 8) as usize;
    let shift = ((where_ & 3) * 8) as u32;
    addr = (pci_addr << 5) + mask + CIA_CONF as usize;
    *value = conf_read(addr, type1) >> shift; PCIBIOS_SUCCESSFUL
}

unsafe fn cia_write_config(bus: *mut pci_bus, devfn: u32, where_: i32, size: i32, value: u32) -> i32 {
    let mut addr = 0usize; let mut pci_addr = 0usize; let mut type1 = 0u8;
    if mk_conf_addr(bus, devfn, where_, &mut pci_addr, &mut type1) != 0 { return PCIBIOS_DEVICE_NOT_FOUND; }
    addr = (pci_addr << 5) + ((size - 1) * 8) as usize + CIA_CONF as usize;
    conf_write(addr, value << ((where_ & 3) * 8), type1); PCIBIOS_SUCCESSFUL
}

#[repr(C)]
pub struct pci_ops { pub read: unsafe fn(*mut pci_bus,u32,i32,i32,*mut u32)->i32, pub write: unsafe fn(*mut pci_bus,u32,i32,i32,u32)->i32 }
pub static mut cia_pci_ops: pci_ops = pci_ops { read: cia_read_config, write: cia_write_config };

pub unsafe fn cia_pci_tbi(_hose: *mut pci_controller, _start: dma_addr_t, _end: dma_addr_t) {
    wmb(); *(CIA_IOC_PCI_TBIA as Vip) = 3; mb(); *(CIA_IOC_PCI_TBIA as Vip);
}

const CIA_BROKEN_TBIA_BASE: usize = 0x30000000;
const CIA_BROKEN_TBIA_SIZE: usize = 1024;

unsafe fn cia_pci_tbi_try2(_hose: *mut pci_controller, _start: dma_addr_t, _end: dma_addr_t) {
    mb(); let ctrl = *(CIA_IOC_CIA_CTRL as Vip); *(CIA_IOC_CIA_CTRL as Vip) = ctrl | CIA_CTRL_PCI_LOOP_EN; mb(); *(CIA_IOC_CIA_CTRL as Vip); mb();
    let bus_addr = cia_ioremap(CIA_BROKEN_TBIA_BASE, 32768 * 4);
    cia_readl(bus_addr); cia_readl(bus_addr.add(0x8000)); cia_readl(bus_addr.add(0x10000)); cia_readl(bus_addr.add(0x18000)); cia_iounmap(bus_addr);
    mb(); *(CIA_IOC_CIA_CTRL as Vip) = ctrl; mb(); *(CIA_IOC_CIA_CTRL as Vip); mb();
}

unsafe fn cia_prepare_tbia_workaround(window: i32) {
    let ppte = memblock_alloc_or_panic(CIA_BROKEN_TBIA_SIZE, 32768) as *mut usize;
    let pte = (virt_to_phys(ppte as *mut _) >> (PAGE_SHIFT - 1)) | 1;
    for i in 0..CIA_BROKEN_TBIA_SIZE / core::mem::size_of::<usize>() { *ppte.add(i) = pte; }
    *(CIA_IOC_PCI_Wn_BASE(window) as Vip) = (CIA_BROKEN_TBIA_BASE | 3) as i32;
    *(CIA_IOC_PCI_Wn_MASK(window) as Vip) = ((CIA_BROKEN_TBIA_SIZE * 1024 - 1) & 0xfff00000) as i32;
    *(CIA_IOC_PCI_Tn_BASE(window) as Vip) = (virt_to_phys(ppte as *mut _) >> 2) as i32;
}

pub unsafe fn cia_init_arch() { do_init_arch(0); }
pub unsafe fn pyxis_init_arch() {
    let mut cc0: u32; let mut cc1: u32; let pyxis_cc = *(PYXIS_RT_COUNT as *mut u32);
    core::arch::asm!("rpcc {0}", out(reg) cc0); while (*(PYXIS_RT_COUNT as *mut u32)).wrapping_sub(pyxis_cc) < 4096 {}
    core::arch::asm!("rpcc {0}", out(reg) cc1); cc1 = cc1.wrapping_sub(cc0);
    (*hwrpb).cycle_freq = ((cc1 >> 11) as u64 * 100000000 / 3) as _; hwrpb_update_checksum(hwrpb); do_init_arch(1);
}

unsafe fn do_init_arch(is_pyxis: i32) {
    let mut hose: *mut pci_controller; let mut temp: i32; let mut cia_rev: i32; let mut tbia_window = 1;
    cia_rev = *(CIA_IOC_CIA_REV as Vip) & CIA_REV_MASK;
    printk("pci: cia revision %d%s\0", cia_rev, if is_pyxis != 0 { " (pyxis)\0" } else { "\0" });
    if alpha_using_srm != 0 { cia_save_srm_settings(is_pyxis); }
    temp = *(CIA_IOC_ERR_MASK as Vip); temp &= !(CIA_ERR_CPU_PE | CIA_ERR_MEM_NEM | CIA_ERR_PA_PTE_INV | CIA_ERR_RCVD_MAS_ABT | CIA_ERR_RCVD_TAR_ABT); *(CIA_IOC_ERR_MASK as Vip) = temp;
    temp = *(CIA_IOC_CIA_ERR as Vip); *(CIA_IOC_CIA_ERR as Vip) = temp;
    temp = *(CIA_IOC_CIA_CTRL as Vip); temp |= CIA_CTRL_FILL_ERR_EN | CIA_CTRL_MCHK_ERR_EN; *(CIA_IOC_CIA_CTRL as Vip) = temp; *(CIA_IOC_CFG as Vip) = 0; *(CIA_IOC_HAE_MEM as Vip) = 0; *(CIA_IOC_HAE_IO as Vip) = 0;
    if is_pyxis != 0 { temp = *(CIA_IOC_CIA_CNFG as Vip); temp |= CIA_CNFG_IOA_BWEN | CIA_CNFG_PCI_MWEN; *(CIA_IOC_CIA_CNFG as Vip) = temp; }
    mb(); *(CIA_IOC_CIA_REV as Vip);
    pci_isa_hose = hose = alloc_pci_controller(); (*hose).io_space = &mut ioport_resource; (*hose).mem_space = &mut iomem_resource; (*hose).index = 0;
    (*hose).sparse_mem_base = if is_pyxis == 0 { CIA_SPARSE_MEM - IDENT_ADDR } else { 0 }; (*hose).dense_mem_base = if is_pyxis == 0 { CIA_DENSE_MEM - IDENT_ADDR } else { CIA_BW_MEM - IDENT_ADDR }; (*hose).sparse_io_base = if is_pyxis == 0 { CIA_IO - IDENT_ADDR } else { 0 }; (*hose).dense_io_base = if is_pyxis != 0 { CIA_BW_IO - IDENT_ADDR } else { 0 };
    (*hose).sg_pci = core::ptr::null_mut(); (*hose).sg_isa = iommu_arena_new(hose, 0x00800000, 0x00800000, 32768);
    __direct_map_base = 0x80000000; __direct_map_size = 0x80000000;
    *(CIA_IOC_PCI_W0_BASE as Vip) = ((*hose).sg_isa).as_ref().unwrap().dma_base as i32 | 3; *(CIA_IOC_PCI_W0_MASK as Vip) = (((*hose).sg_isa).as_ref().unwrap().size - 1) as i32 & 0xfff00000; *(CIA_IOC_PCI_T0_BASE as Vip) = (virt_to_phys((*hose).sg_isa.as_ref().unwrap().ptes) >> 2) as i32;
    *(CIA_IOC_PCI_W2_BASE as Vip) = __direct_map_base as i32 | 1; *(CIA_IOC_PCI_W2_MASK as Vip) = (__direct_map_size as i32 - 1) & 0xfff00000; *(CIA_IOC_PCI_T2_BASE as Vip) = 0;
    if is_pyxis != 0 { *(CIA_IOC_PCI_W3_BASE as Vip) = 0; } else if cia_rev == 1 { *(CIA_IOC_PCI_W1_BASE as Vip) = 0; tbia_window = 3; } else if max_low_pfn > (0x100000000usize >> PAGE_SHIFT) { *(CIA_IOC_PCI_W3_BASE as Vip) = 0; } else { *(CIA_IOC_PCI_W3_BASE as Vip) = 9; *(CIA_IOC_PCI_W3_MASK as Vip) = 0xfff00000u32 as i32; *(CIA_IOC_PCI_T3_BASE as Vip) = 0; alpha_mv.pci_dac_offset = 0x200000000; *(CIA_IOC_PCI_W_DAC as Vip) = (alpha_mv.pci_dac_offset >> 32) as i32; }
    cia_prepare_tbia_workaround(tbia_window);
}

pub unsafe fn cia_kill_arch(_mode: i32) { if alpha_using_srm != 0 { cia_restore_srm_settings(); } }
pub unsafe fn cia_init_pci() { verify_tb_operation(); common_init_pci(); }

unsafe fn cia_pci_clr_err() { let jd = *(CIA_IOC_CIA_ERR as Vip); *(CIA_IOC_CIA_ERR as Vip) = jd; mb(); *(CIA_IOC_CIA_ERR as Vip); }

unsafe fn cia_decode_mchk(la_ptr: usize) -> i32 {
    let com = la_ptr as *mut el_common; let cia = (la_ptr + (*com).sys_offset as usize) as *mut el_CIA_sysdata_mcheck;
    if (*cia).cia_err & CIA_ERR_VALID == 0 { return 0; }
    1
}

pub unsafe fn cia_machine_check(vector: usize, la_ptr: usize) {
    mb(); mb(); draina(); cia_pci_clr_err(); wrmces(rdmces()); mb();
    let mut expected = *mcheck_expected(0); if expected == 0 && vector == 0x660 { expected = cia_decode_mchk(la_ptr); }
    process_mcheck_info(vector, la_ptr, "CIA\0", expected);
}

// External kernel types, symbols, register definitions, and helper functions
// intentionally remain unresolved here; they are supplied by other files.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
