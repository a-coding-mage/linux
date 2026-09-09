// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/arch/alpha/kernel/core_wildfire.c
 *
 *  Wildfire support.
 *
 *  Copyright (C) 2000 Andrea Arcangeli <andrea@suse.de> SuSE
 */

// Dependencies supplied by the surrounding kernel translation unit.

pub const DEBUG_CONFIG: i32 = 0;
pub const DEBUG_DUMP_REGS: i32 = 0;
pub const DEBUG_DUMP_CONFIG: i32 = 1;
pub const QBB_MAP_EMPTY: u8 = 0xff;

pub static mut wildfire_hard_qbb_map: [u8; WILDFIRE_MAX_QBB as usize] = [0; WILDFIRE_MAX_QBB as usize];
pub static mut wildfire_soft_qbb_map: [u8; WILDFIRE_MAX_QBB as usize] = [0; WILDFIRE_MAX_QBB as usize];
pub static mut wildfire_hard_qbb_mask: c_ulong = 0;
pub static mut wildfire_soft_qbb_mask: c_ulong = 0;
pub static mut wildfire_gp_mask: c_ulong = 0;
pub static mut wildfire_hs_mask: c_ulong = 0;
pub static mut wildfire_iop_mask: c_ulong = 0;
pub static mut wildfire_ior_mask: c_ulong = 0;
pub static mut wildfire_pca_mask: c_ulong = 0;
pub static mut wildfire_cpu_mask: c_ulong = 0;
pub static mut wildfire_mem_mask: c_ulong = 0;

unsafe fn wildfire_init_hose(qbbno: i32, hoseno: i32) {
    let hose = alloc_pci_controller();
    (*hose).io_space = alloc_resource();
    (*hose).mem_space = alloc_resource();
    (*hose).sparse_mem_base = 0;
    (*hose).sparse_io_base = 0;
    (*hose).dense_mem_base = WILDFIRE_MEM(qbbno, hoseno);
    (*hose).dense_io_base = WILDFIRE_IO(qbbno, hoseno);
    (*hose).config_space_base = WILDFIRE_CONF(qbbno, hoseno);
    (*hose).index = (qbbno << 3) + hoseno;
    (*(*hose).io_space).start = WILDFIRE_IO(qbbno, hoseno) - WILDFIRE_IO_BIAS;
    (*(*hose).io_space).end = (*(*hose).io_space).start + WILDFIRE_IO_SPACE - 1;
    (*(*hose).io_space).name = pci_io_names[hoseno as usize];
    (*(*hose).io_space).flags = IORESOURCE_IO;
    (*(*hose).mem_space).start = WILDFIRE_MEM(qbbno, hoseno) - WILDFIRE_MEM_BIAS;
    (*(*hose).mem_space).end = (*(*hose).mem_space).start + 0xffff_ffff;
    (*(*hose).mem_space).name = pci_mem_names[hoseno as usize];
    (*(*hose).mem_space).flags = IORESOURCE_MEM;
    if request_resource(&mut ioport_resource, (*hose).io_space) < 0 { printk!(KERN_ERR "Failed to request IO on qbb %d hose %d\n", qbbno, hoseno); }
    if request_resource(&mut iomem_resource, (*hose).mem_space) < 0 { printk!(KERN_ERR "Failed to request MEM on qbb %d hose %d\n", qbbno, hoseno); }
    (*hose).sg_isa = iommu_arena_new(hose, 0x00800000, 0x00800000, SMP_CACHE_BYTES);
    (*hose).sg_pci = iommu_arena_new(hose, 0xc0000000, 0x08000000, SMP_CACHE_BYTES);
    let pci = WILDFIRE_pci(qbbno, hoseno);
    (*pci).pci_window[0].wbase.csr = (*(*hose).sg_isa).dma_base | 3;
    (*pci).pci_window[0].wmask.csr = ((*(*hose).sg_isa).size - 1) & 0xfff00000;
    (*pci).pci_window[0].tbase.csr = virt_to_phys((*(*hose).sg_isa).ptes);
    (*pci).pci_window[1].wbase.csr = 0x40000000 | 1;
    (*pci).pci_window[1].wmask.csr = (0x40000000 - 1) & 0xfff00000;
    (*pci).pci_window[1].tbase.csr = 0;
    (*pci).pci_window[2].wbase.csr = 0x80000000 | 1;
    (*pci).pci_window[2].wmask.csr = (0x40000000 - 1) & 0xfff00000;
    (*pci).pci_window[2].tbase.csr = 0x40000000;
    (*pci).pci_window[3].wbase.csr = (*(*hose).sg_pci).dma_base | 3;
    (*pci).pci_window[3].wmask.csr = ((*(*hose).sg_pci).size - 1) & 0xfff00000;
    (*pci).pci_window[3].tbase.csr = virt_to_phys((*(*hose).sg_pci).ptes);
    wildfire_pci_tbi(hose, 0, 0);
}

unsafe fn wildfire_init_pca(qbbno: i32, pcano: i32) {
    if !WILDFIRE_PCA_EXISTS(qbbno, pcano) { return; }
    wildfire_init_hose(qbbno, (pcano << 1) + 0);
    wildfire_init_hose(qbbno, (pcano << 1) + 1);
}

unsafe fn wildfire_init_qbb(qbbno: i32) {
    if !WILDFIRE_QBB_EXISTS(qbbno) { return; }
    for pcano in 0..WILDFIRE_PCA_PER_QBB { wildfire_init_pca(qbbno, pcano); }
}

unsafe fn wildfire_hardware_probe() {
    let fast = WILDFIRE_fast_qsd();
    let mut temp = (*fast).qsd_whami.csr;
    let mut hard_qbb = (temp >> 8) & 7;
    let mut soft_qbb = (temp >> 4) & 7;
    wildfire_hard_qbb_mask = 1 << hard_qbb;
    wildfire_soft_qbb_mask = 1 << soft_qbb;
    wildfire_gp_mask = 0; wildfire_hs_mask = 0; wildfire_iop_mask = 0;
    wildfire_ior_mask = 0; wildfire_pca_mask = 0; wildfire_cpu_mask = 0; wildfire_mem_mask = 0;
    memset(wildfire_hard_qbb_map.as_mut_ptr(), QBB_MAP_EMPTY as i32, WILDFIRE_MAX_QBB);
    memset(wildfire_soft_qbb_map.as_mut_ptr(), QBB_MAP_EMPTY as i32, WILDFIRE_MAX_QBB);
    let mut qsa = WILDFIRE_qsa(soft_qbb);
    temp = (*qsa).qsa_qbb_id.csr;
    if temp & 0x40 != 0 { wildfire_hs_mask = 1; }
    if temp & 0x20 != 0 {
        let gp = WILDFIRE_gp(soft_qbb); temp = 0;
        for i in 0..4 { temp |= (*gp).gpa_qbb_map[i].csr << (i * 8); }
        for h in 0..WILDFIRE_MAX_QBB { if temp & 8 != 0 { soft_qbb = temp & 7; wildfire_hard_qbb_mask |= 1 << h; wildfire_soft_qbb_mask |= 1 << soft_qbb; } temp >>= 4; }
        wildfire_gp_mask = wildfire_soft_qbb_mask;
    }
    for s in 0..WILDFIRE_MAX_QBB {
        if !WILDFIRE_QBB_EXISTS(s) { continue; }
        let qsd = WILDFIRE_qsd(s); temp = (*qsd).qsd_whami.csr; hard_qbb = (temp >> 8) & 7;
        wildfire_hard_qbb_map[hard_qbb as usize] = s as u8; wildfire_soft_qbb_map[s as usize] = hard_qbb as u8;
        qsa = WILDFIRE_qsa(s); temp = (*qsa).qsa_qbb_pop[0].csr;
        wildfire_cpu_mask |= ((temp >> 0) & 0xf) << (s << 2); wildfire_mem_mask |= ((temp >> 4) & 0xf) << (s << 2);
        temp = (*qsa).qsa_qbb_pop[1].csr; wildfire_iop_mask |= 1 << s; wildfire_ior_mask |= ((temp >> 4) & 0xf) << (s << 2);
        if (*qsa).qsa_qbb_id.csr & 0x20 != 0 { wildfire_gp_mask |= 1 << s; }
        let iop = WILDFIRE_iop(s);
        for i in 0..WILDFIRE_PCA_PER_QBB { let ne = WILDFIRE_ne(s, i); let fe = WILDFIRE_fe(s, i); if ((*iop).iop_hose[i].init.csr & 1) == 1 && ((*ne).ne_what_am_i.csr & 0xf00000300UL) == 0x100000300UL && ((*fe).fe_what_am_i.csr & 0xf00000300UL) == 0x100000200UL { wildfire_pca_mask |= 1 << ((s << 2) + i); } }
    }
}

pub unsafe fn wildfire_init_arch() { ioport_resource.end = !0; wildfire_hardware_probe(); for qbbno in 0..WILDFIRE_MAX_QBB { wildfire_init_qbb(qbbno); } __direct_map_base = 0x40000000; __direct_map_size = 0x80000000; }
pub unsafe fn wildfire_machine_check(vector: c_ulong, la_ptr: c_ulong) { mb(); mb(); draina(); wrmces(7); mb(); process_mcheck_info(vector, la_ptr, "WILDFIRE", mcheck_expected(smp_processor_id())); }
pub unsafe fn wildfire_kill_arch(_mode: i32) {}
pub unsafe fn wildfire_pci_tbi(hose: *mut pci_controller, _start: dma_addr_t, _end: dma_addr_t) { let qbbno = (*hose).index >> 3; let hoseno = (*hose).index & 7; let pci = WILDFIRE_pci(qbbno, hoseno); mb(); let _ = (*pci).pci_flush_tlb.csr; }

unsafe fn mk_conf_addr(pbus: *mut pci_bus, device_fn: c_uint, where_: i32, pci_addr: *mut c_ulong, type1: *mut u8) -> i32 { let hose = (*pbus).sysdata; let mut bus = (*pbus).number; if (*pbus).parent.is_null() { bus = 0; } *type1 = (bus != 0) as u8; *pci_addr = ((bus as c_ulong) << 16) | ((device_fn as c_ulong) << 8) | where_ as c_ulong | (*hose).config_space_base; 0 }
unsafe fn wildfire_read_config(bus: *mut pci_bus, devfn: c_uint, where_: i32, size: i32, value: *mut u32) -> i32 { let mut addr=0; let mut type1=0; if mk_conf_addr(bus,devfn,where_,&mut addr,&mut type1)!=0{return PCIBIOS_DEVICE_NOT_FOUND;} match size {1=>*value=__kernel_ldbu(addr as *const u8) as u32,2=>*value=__kernel_ldwu(addr as *const u16) as u32,4=>*value=*(addr as *const u32),_=>{}} PCIBIOS_SUCCESSFUL }
unsafe fn wildfire_write_config(bus: *mut pci_bus, devfn: c_uint, where_: i32, size: i32, value: u32) -> i32 { let mut addr=0; let mut type1=0; if mk_conf_addr(bus,devfn,where_,&mut addr,&mut type1)!=0{return PCIBIOS_DEVICE_NOT_FOUND;} match size {1=>{__kernel_stb(value,addr as *mut u8);mb();let _=__kernel_ldbu(addr as *const u8)},2=>{__kernel_stw(value,addr as *mut u16);mb();let _=__kernel_ldwu(addr as *const u16)},4=>{*(addr as *mut u32)=value;mb();let _=*(addr as *const u32)},_=>{}} PCIBIOS_SUCCESSFUL }

pub static mut wildfire_pci_ops: pci_ops = pci_ops { read: wildfire_read_config, write: wildfire_write_config };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
