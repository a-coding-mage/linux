// SPDX-License-Identifier: GPL-2.0
/* Translation of linux/arch/alpha/kernel/core_marvel.c. */

/* External kernel declarations, types, constants, and architecture macros are
 * supplied by the surrounding kernel translation unit. */

static mut IO7_HEAD: *mut io7 = core::ptr::null_mut();

unsafe fn read_ev7_csr(pe: i32, offset: usize) -> usize {
    let ev7csr = EV7_CSR_KERN(pe, offset);
    mb(); let q = (*ev7csr).csr; mb(); q
}

unsafe fn write_ev7_csr(pe: i32, offset: usize, q: usize) {
    let ev7csr = EV7_CSR_KERN(pe, offset);
    mb(); (*ev7csr).csr = q; mb();
}

unsafe fn mk_resource_name(pe: i32, port: i32, s: *const i8) -> *mut i8 {
    let mut tmp = [0i8; 80];
    let sz = scnprintf(tmp.as_mut_ptr(), tmp.len(), b"PCI %s PE %d PORT %d\0".as_ptr() as _, s, pe, port) + 1;
    let name = memblock_alloc_or_panic(sz, SMP_CACHE_BYTES) as *mut i8;
    strscpy(name, tmp.as_ptr(), sz); name
}

#[inline]
pub unsafe fn marvel_next_io7(prev: *mut io7) -> *mut io7 {
    if !prev.is_null() { (*prev).next } else { IO7_HEAD }
}

pub unsafe fn marvel_find_io7(pe: i32) -> *mut io7 {
    let mut p = IO7_HEAD;
    while !p.is_null() && (*p).pe != pe { p = (*p).next; }
    p
}

unsafe fn alloc_io7(pe: u32) -> *mut io7 {
    if !marvel_find_io7(pe as i32).is_null() { printk(KERN_WARNING, b"IO7 at PE %d already allocated!\0".as_ptr() as _, pe); return core::ptr::null_mut(); }
    let p = memblock_alloc_or_panic(core::mem::size_of::<io7>(), SMP_CACHE_BYTES) as *mut io7;
    (*p).pe = pe; raw_spin_lock_init(&mut (*p).irq_lock);
    for h in 0..4 { (*p).ports[h].io7 = p; (*p).ports[h].port = h as i32; (*p).ports[h].enabled = 0; }
    if IO7_HEAD.is_null() { IO7_HEAD = p; }
    else if (*IO7_HEAD).pe > (*p).pe { (*p).next = IO7_HEAD; IO7_HEAD = p; }
    else {
        let mut q = IO7_HEAD;
        while !q.is_null() {
            if (*q).pe == (*p).pe { printk(KERN_ERR, b"Too many IO7s at PE %d\n\0".as_ptr() as _, (*p).pe); return core::ptr::null_mut(); }
            if (*q).next.is_null() || (*q).next.as_ref().unwrap().pe > (*p).pe { (*p).next = (*q).next; (*q).next = p; break; }
            q = (*q).next;
        }
        if q.is_null() { (*p).next = IO7_HEAD; IO7_HEAD = p; }
    } p
}

pub unsafe fn io7_clear_errors(io7: *mut io7) {
    for port in 0..4 { let c = IO7_CSRS_KERN((*io7).pe, port); (*c).POx_ERR_SUM.csr = !0; (*c).POx_TLB_ERR.csr = !0; (*c).POx_SPL_COMPLT.csr = !0; (*c).POx_TRANS_SUM.csr = !0; }
    let c = IO7_PORT7_CSRS_KERN((*io7).pe); (*c).PO7_ERROR_SUM.csr = !0; (*c).PO7_UNCRR_SYM.csr = !0; (*c).PO7_CRRCT_SYM.csr = !0;
}

unsafe fn io7_init_hose(io7: *mut io7, port: i32) {
    static mut HOSE_INDEX: i32 = 0;
    let hose = alloc_pci_controller(); let p = &mut (*io7).ports[port as usize]; let c = IO7_CSRS_KERN((*io7).pe, port);
    (*hose).index = HOSE_INDEX; HOSE_INDEX += 1; if (*hose).index == 0 { pci_isa_hose = hose; }
    p.csrs = c; p.hose = hose; (*hose).sysdata = p as *mut _ as *mut core::ffi::c_void;
    (*hose).io_space = alloc_resource(); (*hose).mem_space = alloc_resource(); (*hose).sparse_mem_base = 0; (*hose).sparse_io_base = 0;
    (*hose).dense_mem_base = IO7_MEM_PHYS((*io7).pe, port); (*hose).dense_io_base = IO7_IO_PHYS((*io7).pe, port);
    (*hose).config_space_base = IO7_CONF_KERN((*io7).pe, port) as usize;
    (*(*hose).io_space).start = IO7_IO_KERN((*io7).pe, port) as usize; (*(*hose).io_space).end = (*(*hose).io_space).start + IO7_IO_SPACE - 1; (*(*hose).io_space).name = mk_resource_name((*io7).pe as i32, port, b"IO\0".as_ptr() as _); (*(*hose).io_space).flags = IORESOURCE_IO;
    (*(*hose).mem_space).start = IO7_MEM_KERN((*io7).pe, port) as usize; (*(*hose).mem_space).end = (*(*hose).mem_space).start + IO7_MEM_SPACE - 1; (*(*hose).mem_space).name = mk_resource_name((*io7).pe as i32, port, b"MEM\0".as_ptr() as _); (*(*hose).mem_space).flags = IORESOURCE_MEM;
    request_resource(&mut ioport_resource, (*hose).io_space); request_resource(&mut iomem_resource, (*hose).mem_space);
    for i in 0..4 { p.saved_wbase[i] = (*c).POx_WBASE[i].csr; p.saved_wmask[i] = (*c).POx_WMASK[i].csr; p.saved_tbase[i] = (*c).POx_TBASE[i].csr; }
    marvel_pci_tbi(hose, 0, !0); (*hose).sg_isa = iommu_arena_new_node(0, hose, 0x00800000, 0x00800000, 0); (*(*hose).sg_isa).align_entry = 8;
    (*c).POx_WBASE[0].csr = (*hose).sg_isa.dma_base | wbase_m_ena | wbase_m_sg; (*c).POx_WMASK[0].csr = ((*hose).sg_isa.size - 1) & wbase_m_addr; (*c).POx_TBASE[0].csr = virt_to_phys((*hose).sg_isa.ptes);
    (*c).POx_WBASE[1].csr = __direct_map_base | wbase_m_ena; (*c).POx_WMASK[1].csr = (__direct_map_size - 1) & wbase_m_addr; (*c).POx_TBASE[1].csr = 0;
    (*hose).sg_pci = iommu_arena_new_node(0, hose, 0xc0000000, 0x40000000, 0); (*(*hose).sg_pci).align_entry = 8; (*c).POx_WBASE[2].csr = (*hose).sg_pci.dma_base | wbase_m_ena | wbase_m_sg; (*c).POx_WMASK[2].csr = ((*hose).sg_pci.size - 1) & wbase_m_addr; (*c).POx_TBASE[2].csr = virt_to_phys((*hose).sg_pci.ptes); (*c).POx_WBASE[3].csr = 0; (*c).POx_CTRL.csr &= !(1usize << 61); (*c).POx_MSK_HEI.csr &= !(3usize << 14); marvel_pci_tbi(hose, 0, !0);
}

unsafe fn marvel_init_io7(io7: *mut io7) { printk(0, b"Initializing IO7 at PID %d\0".as_ptr() as _, (*io7).pe); (*io7).csrs = IO7_PORT7_CSRS_KERN((*io7).pe); for i in 0..IO7_NUM_PORTS { let c = IO7_CSRS_KERN((*io7).pe, i as i32); if (*c).POx_CACHE_CTL.csr == 8 { (*io7).ports[i].enabled = 1; io7_init_hose(io7, i as i32); } } }
unsafe fn marvel_io7_present(node: *mut gct6_node) { if (*node).type_ != GCT_TYPE_HOSE || (*node).subtype != GCT_SUBTYPE_IO_PORT_MODULE { return; } let pe = ((*node).id >> 8) & 0xff; printk(0, b"Found an IO7 at PID %d\0".as_ptr() as _, pe); alloc_io7(pe); }

pub static mut gct_wanted_node_list: [gct6_search_struct; 2] = [gct6_search_struct { type_: GCT_TYPE_HOSE, subtype: GCT_SUBTYPE_IO_PORT_MODULE, fn_: Some(marvel_io7_present) }, gct6_search_struct { type_: 0, subtype: 0, fn_: None }];

pub unsafe fn marvel_init_arch() { ioport_resource.end = !0; __direct_map_base = 0x80000000; __direct_map_size = 0x40000000; gct6_find_nodes(GCT_NODE_PTR(0), gct_wanted_node_list.as_mut_ptr()); let mut p = core::ptr::null_mut(); loop { p = marvel_next_io7(p); if p.is_null() { break; } marvel_init_io7(p); } }
pub unsafe fn marvel_kill_arch(_mode: i32) {}

unsafe fn build_conf_addr(hose: *mut pci_controller, bus: u8, devfn: u32, where_: i32) -> usize { (*hose).config_space_base | ((bus as usize) << 16) | ((devfn as usize) << 8) | where_ as usize }
unsafe fn mk_conf_addr(pbus: *mut pci_bus, devfn: u32, where_: i32) -> usize { let hose = (*pbus).sysdata as *mut pci_controller; if hose.is_null() { return 0; } let port = (*hose).sysdata as *mut io7_port; if !(*port).enabled { return 0; } let mut bus = (*pbus).number; if (*pbus).parent.is_null() { if devfn >= PCI_DEVFN(21, 0) { return 0; } bus = 0; } build_conf_addr(hose, bus, devfn, where_) }

unsafe fn marvel_read_config(bus: *mut pci_bus, devfn: u32, where_: i32, size: i32, value: *mut u32) -> i32 { let a = mk_conf_addr(bus, devfn, where_); if a == 0 { return PCIBIOS_DEVICE_NOT_FOUND; } match size { 1 => *value = __kernel_ldbu(a as *const u8) as u32, 2 => *value = __kernel_ldwu(a as *const u16) as u32, 4 => *value = *(a as *const u32), _ => return PCIBIOS_FUNC_NOT_SUPPORTED }; PCIBIOS_SUCCESSFUL }
unsafe fn marvel_write_config(bus: *mut pci_bus, devfn: u32, where_: i32, size: i32, value: u32) -> i32 { let a = mk_conf_addr(bus, devfn, where_); if a == 0 { return PCIBIOS_DEVICE_NOT_FOUND; } match size { 1 => { __kernel_stb(value as u8, a as *mut u8); mb(); __kernel_ldbu(a as *const u8); }, 2 => { __kernel_stw(value as u16, a as *mut u16); mb(); __kernel_ldwu(a as *const u16); }, 4 => { *(a as *mut u32) = value; mb(); core::ptr::read_volatile(a as *const u32); }, _ => return PCIBIOS_FUNC_NOT_SUPPORTED }; PCIBIOS_SUCCESSFUL }
pub static mut marvel_pci_ops: pci_ops = pci_ops { read: Some(marvel_read_config), write: Some(marvel_write_config) };

pub unsafe fn marvel_pci_tbi(hose: *mut pci_controller, _start: usize, _end: usize) { let c = (*(hose).sysdata as *mut io7_port).as_ref().unwrap().csrs; wmb(); (*c).POx_SG_TBIA.csr = 0; mb(); core::ptr::read_volatile(&(*c).POx_SG_TBIA.csr); }

pub unsafe fn marvel_ioremap(addr: usize, size: usize) -> *mut core::ffi::c_void { let mut hose = hose_head; while !hose.is_null() { if addr >> 32 == (*(*hose).mem_space).start >> 32 { break; } hose = (*hose).next; } if hose.is_null() { return core::ptr::null_mut(); } let b = addr - (*(*hose).mem_space).start; if b >= __direct_map_base && b + size - 1 < __direct_map_base + __direct_map_size { return (IDENT_ADDR | b - __direct_map_base) as *mut _; } (addr) as *mut _ }
pub unsafe fn marvel_iounmap(xaddr: *mut core::ffi::c_void) { let a = xaddr as usize; if a >= VMALLOC_START { vfree((PAGE_MASK & a) as *mut _); } }
pub unsafe fn marvel_is_mmio(xaddr: *const core::ffi::c_void) -> i32 { let a = xaddr as usize; if a >= VMALLOC_START { 1 } else if a & 0xff000000 != 0 { 0 } else { 1 } }
pub unsafe fn marvel_ioportmap(addr: usize) -> *mut core::ffi::c_void { addr as *mut _ }
pub unsafe fn marvel_ioread8(xaddr: *const core::ffi::c_void) -> u8 { let a=xaddr as usize; if a==0x60||a==0x64 {0} else if a==0x70||a==0x71 {0} else if marvel_is_ioaddr(a) {__kernel_ldbu(a as _)} else {!0} }
pub unsafe fn marvel_iowrite8(b:u8,xaddr:*mut core::ffi::c_void){let a=xaddr as usize;if a==0x60||a==0x64{return;}if marvel_is_ioaddr(a){__kernel_stb(b,a as _);}}

#[repr(C)] pub struct marvel_rtc_access_info { pub function: usize, pub index: usize, pub data: usize }
unsafe fn __marvel_access_rtc(info: *mut marvel_rtc_access_info) { (*info).data = 0; /* Alpha call_pal cserve rtc; supplied by the architecture backend. */ }
unsafe fn __marvel_rtc_io(b: u8, addr: usize, write: i32) -> u8 { static mut INDEX:u8=0; match addr { 0x70 => { if write != 0 { INDEX=b; } INDEX }, 0x71 => { let mut x=marvel_rtc_access_info{function:0x48+(write==0) as usize,index:INDEX as usize,data:b as usize}; __marvel_access_rtc(&mut x); bin2bcd(x.data as u8) }, _=>{printk(KERN_WARNING,b"Illegal RTC port %lx\0".as_ptr() as _,addr);0} } }

#[repr(C)] pub struct marvel_agp_aperture { pub arena:*mut pci_iommu_arena, pub pg_start:isize, pub pg_count:isize }
unsafe fn marvel_agp_setup(agp:*mut alpha_agp_info)->i32 { if alpha_agpgart_size==0{return -ENOMEM;} let a=kmalloc_obj(core::mem::size_of::<marvel_agp_aperture>()) as *mut marvel_agp_aperture;if a.is_null(){return -ENOMEM;}(*a).arena=(*agp).hose.as_ref().unwrap().sg_pci;(*a).pg_count=(alpha_agpgart_size/PAGE_SIZE) as isize;(*a).pg_start=iommu_reserve((*a).arena,(*a).pg_count,(*a).pg_count-1);if (*a).pg_start<0{kfree(a as _);return -ENOMEM;}(*agp).aperture.bus_base=(*(*a).arena).dma_base+(*a).pg_start as usize*PAGE_SIZE;(*agp).aperture.size=(*a).pg_count as usize*PAGE_SIZE;(*agp).aperture.sysdata=a as _;0 }
unsafe fn marvel_agp_cleanup(agp:*mut alpha_agp_info){let a=(*agp).aperture.sysdata as *mut marvel_agp_aperture;let mut s=iommu_release((*a).arena,(*a).pg_start,(*a).pg_count);if s== -EBUSY{iommu_unbind((*a).arena,(*a).pg_start,(*a).pg_count);s=iommu_release((*a).arena,(*a).pg_start,(*a).pg_count);}if s<0{printk(KERN_ERR,b"Failed to release AGP memory\0".as_ptr() as _);}kfree(a as _);kfree(agp as _);}
unsafe fn marvel_agp_bind_memory(agp:*mut alpha_agp_info,start:isize,mem:*mut agp_memory)->i32{let a=(*agp).aperture.sysdata as *mut marvel_agp_aperture;iommu_bind((*a).arena,(*a).pg_start+start,(*mem).page_count,(*mem).pages)}
unsafe fn marvel_agp_unbind_memory(agp:*mut alpha_agp_info,start:isize,mem:*mut agp_memory)->i32{let a=(*agp).aperture.sysdata as *mut marvel_agp_aperture;iommu_unbind((*a).arena,(*a).pg_start+start,(*mem).page_count)}
unsafe fn marvel_agp_translate(agp:*mut alpha_agp_info,addr:usize)->usize{let a=(*agp).aperture.sysdata as *mut marvel_agp_aperture;let b=addr-(*(*a).arena).dma_base;let p=(*(*a).arena).ptes[b>>PAGE_SHIFT];if p&1==0{return (-EINVAL) as usize;}(p>>1)<<PAGE_SHIFT}
pub static mut marvel_agp_ops:alpha_agp_ops=alpha_agp_ops{setup:Some(marvel_agp_setup),cleanup:Some(marvel_agp_cleanup),configure:None,bind:Some(marvel_agp_bind_memory),unbind:Some(marvel_agp_unbind_memory),translate:Some(marvel_agp_translate)};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
