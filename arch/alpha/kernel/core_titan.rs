// SPDX-License-Identifier: GPL-2.0
/* Translation of linux/arch/alpha/kernel/core_titan.c. */

// C headers and symbols supplied by the surrounding kernel are external dependencies.

#[repr(C)]
pub struct SavedConfig { pub wsba: [usize; 4], pub wsm: [usize; 4], pub tba: [usize; 4] }

static mut SAVED_CONFIG: [SavedConfig; 4] = [SavedConfig { wsba: [0; 4], wsm: [0; 4], tba: [0; 4] }; 4];
static mut TITAN_PCHIP1_PRESENT: i32 = 0;

#[inline]
unsafe fn mk_tig_addr(offset: i32) -> *mut usize { (TITAN_TIG_SPACE + ((offset as usize) << 6)) as *mut usize }

#[inline]
unsafe fn titan_read_tig(offset: i32, _value: u8) -> u8 { (core::ptr::read_volatile(mk_tig_addr(offset)) & 0xff) as u8 }

#[inline]
unsafe fn titan_write_tig(offset: i32, value: u8) { core::ptr::write_volatile(mk_tig_addr(offset), value as usize); }

unsafe fn mk_conf_addr(pbus: *mut pci_bus, device_fn: u32, where_: i32, pci_addr: *mut usize, type1: *mut u8) -> i32 {
    let hose = (*pbus).sysdata;
    let mut bus = (*pbus).number as u8;
    if (*pbus).parent.is_null() { bus = 0; }
    *type1 = (bus != 0) as u8;
    *pci_addr = ((bus as usize) << 16) | ((device_fn as usize) << 8) | where_ as usize;
    *pci_addr |= (*hose).config_space_base;
    0
}

unsafe fn titan_read_config(bus: *mut pci_bus, devfn: u32, where_: i32, size: i32, value: *mut u32) -> i32 {
    let mut addr = 0usize; let mut type1 = 0u8;
    if mk_conf_addr(bus, devfn, where_, &mut addr, &mut type1) != 0 { return PCIBIOS_DEVICE_NOT_FOUND; }
    match size { 1 => *value = core::ptr::read_volatile(addr as *const u8) as u32,
        2 => *value = core::ptr::read_volatile(addr as *const u16) as u32,
        4 => *value = core::ptr::read_volatile(addr as *const u32), _ => {} }
    PCIBIOS_SUCCESSFUL
}

unsafe fn titan_write_config(bus: *mut pci_bus, devfn: u32, where_: i32, size: i32, value: u32) -> i32 {
    let mut addr = 0usize; let mut type1 = 0u8;
    if mk_conf_addr(bus, devfn, where_, &mut addr, &mut type1) != 0 { return PCIBIOS_DEVICE_NOT_FOUND; }
    match size { 1 => core::ptr::write_volatile(addr as *mut u8, value as u8),
        2 => core::ptr::write_volatile(addr as *mut u16, value as u16),
        4 => core::ptr::write_volatile(addr as *mut u32, value), _ => {} }
    mb(); PCIBIOS_SUCCESSFUL
}

#[repr(C)] pub struct pci_ops { pub read: unsafe fn(*mut pci_bus,u32,i32,i32,*mut u32)->i32, pub write: unsafe fn(*mut pci_bus,u32,i32,i32,u32)->i32 }
#[no_mangle] pub static mut titan_pci_ops: pci_ops = pci_ops { read: titan_read_config, write: titan_write_config };

unsafe fn titan_pci_tbi(hose: *mut pci_controller, start: dma_addr_t, end: dma_addr_t) {
    let pachip = if (*hose).index & 1 != 0 { TITAN_pachip1 } else { TITAN_pachip0 };
    let port = if (*hose).index & 2 != 0 { &mut (*pachip).a_port } else { &mut (*pachip).g_port };
    let csr = if ((start ^ end) & 0xffff0000) == 0 { &mut port.port_specific.g.gtlbiv.csr } else { &mut port.port_specific.g.gtlbia.csr };
    wmb(); *csr = (start & 0xffff0000) >> 12; mb(); let _ = *csr;
}

unsafe fn titan_query_agp(port: *mut titan_pachip_port) -> i32 {
    let mut pctl: TPAchipPCTL = core::mem::zeroed(); pctl.pctl_q_whole = (*port).pctl.csr; pctl.pctl_r_bits.apctl_v_agp_present
}

unsafe fn titan_init_one_pachip_port(port: *mut titan_pachip_port, index: i32) {
    let hose = alloc_pci_controller(); if index == 0 { pci_isa_hose = hose; }
    (*hose).io_space = alloc_resource(); (*hose).mem_space = alloc_resource();
    (*hose).sparse_mem_base=0; (*hose).sparse_io_base=0;
    (*hose).dense_mem_base=(TITAN_MEM(index)&0xffffffffff)|0x80000000000; (*hose).dense_io_base=(TITAN_IO(index)&0xffffffffff)|0x80000000000;
    (*hose).config_space_base=TITAN_CONF(index); (*hose).index=index;
    (*(*hose).io_space).start=TITAN_IO(index)-TITAN_IO_BIAS; (*(*hose).io_space).end=(*(*hose).io_space).start+TITAN_IO_SPACE-1; (*(*hose).io_space).name=pci_io_names[index as usize]; (*(*hose).io_space).flags=IORESOURCE_IO;
    (*(*hose).mem_space).start=TITAN_MEM(index)-TITAN_MEM_BIAS; (*(*hose).mem_space).end=(*(*hose).mem_space).start+0xffffffff; (*(*hose).mem_space).name=pci_mem_names[index as usize]; (*(*hose).mem_space).flags=IORESOURCE_MEM;
    request_resource(&mut ioport_resource, (*hose).io_space); request_resource(&mut iomem_resource, (*hose).mem_space);
    for i in 0..4 { SAVED_CONFIG[index as usize].wsba[i]=(*port).wsba[i].csr; SAVED_CONFIG[index as usize].wsm[i]=(*port).wsm[i].csr; SAVED_CONFIG[index as usize].tba[i]=(*port).tba[i].csr; }
    (*hose).sg_isa=iommu_arena_new(hose,0x00800000,0x00800000,SMP_CACHE_BYTES); (*(*hose).sg_isa).align_entry=8;
    (*hose).sg_pci=iommu_arena_new(hose,0xc0000000,0x40000000,SMP_CACHE_BYTES); (*(*hose).sg_pci).align_entry=4;
    (*port).wsba[0].csr=(*(*hose).sg_isa).dma_base|3; (*port).wsm[0].csr=((*(*hose).sg_isa).size-1)&0xfff00000; (*port).tba[0].csr=virt_to_phys((*(*hose).sg_isa).ptes);
    (*port).wsba[1].csr=__direct_map_base|1; (*port).wsm[1].csr=(__direct_map_size-1)&0xfff00000; (*port).tba[1].csr=0;
    (*port).wsba[2].csr=(*(*hose).sg_pci).dma_base|3; (*port).wsm[2].csr=((*(*hose).sg_pci).size-1)&0xfff00000; (*port).tba[2].csr=virt_to_phys((*(*hose).sg_pci).ptes); (*port).wsba[3].csr=0;
    (*port).pctl.csr|=pctl_m_mwin; if titan_query_agp(port)!=0 { (*port).port_specific.a.agplastwr.csr=__direct_map_base; } titan_pci_tbi(hose,0,!0);
}

unsafe fn titan_init_pachips(p0:*mut titan_pachip,p1:*mut titan_pachip) { TITAN_PCHIP1_PRESENT=(TITAN_cchip.csc.csr&(1<<14)) as i32; titan_init_one_pachip_port(&mut (*p0).g_port,0); if TITAN_PCHIP1_PRESENT!=0 {titan_init_one_pachip_port(&mut (*p1).g_port,1);} titan_init_one_pachip_port(&mut (*p0).a_port,2); if TITAN_PCHIP1_PRESENT!=0 {titan_init_one_pachip_port(&mut (*p1).a_port,3);} }

pub unsafe fn titan_init_arch() { boot_cpuid=__hard_smp_processor_id(); ioport_resource.end=!0; iomem_resource.end=!0; __direct_map_base=0x80000000; __direct_map_size=0x40000000; titan_init_pachips(TITAN_pachip0,TITAN_pachip1); find_console_vga_hose(); }

unsafe fn titan_kill_one_pachip_port(port:*mut titan_pachip_port,index:i32) { for i in 0..4 {(*port).wsba[i].csr=SAVED_CONFIG[index as usize].wsba[i];(*port).wsm[i].csr=SAVED_CONFIG[index as usize].wsm[i];(*port).tba[i].csr=SAVED_CONFIG[index as usize].tba[i];} }
unsafe fn titan_kill_pachips(p0:*mut titan_pachip,p1:*mut titan_pachip) { if TITAN_PCHIP1_PRESENT!=0 {titan_kill_one_pachip_port(&mut(*p1).g_port,1);titan_kill_one_pachip_port(&mut(*p1).a_port,3);} titan_kill_one_pachip_port(&mut(*p0).g_port,0);titan_kill_one_pachip_port(&mut(*p0).a_port,2); }
pub unsafe fn titan_kill_arch(_mode:i32){titan_kill_pachips(TITAN_pachip0,TITAN_pachip1);}

pub unsafe fn titan_ioportmap(mut addr:usize)->*mut u8 { FIXUP_IOADDR_VGA(addr); (addr+TITAN_IO_BIAS) as *mut u8 }
pub unsafe fn titan_iounmap(xaddr:*const u8){let addr=xaddr as usize;if addr>=VMALLOC_START{vfree((addr&PAGE_MASK) as *mut core::ffi::c_void);}}
pub unsafe fn titan_is_mmio(xaddr:*const u8)->i32{let addr=xaddr as usize;if addr>=VMALLOC_START{1}else{((addr&0x100000000)==0) as i32}}

pub unsafe fn titan_ioremap(addr:usize,size:usize)->*mut u8 {
    let h=(addr&TITAN_HOSE_MASK)>>TITAN_HOSE_SHIFT; let baddr=addr&!TITAN_HOSE_MASK; let last=baddr+size-1;
    let mut hose=hose_head; while !hose.is_null() && (*hose).index!=h {hose=(*hose).next;} if hose.is_null(){return core::ptr::null_mut();}
    if baddr>=__direct_map_base && baddr+size-1<__direct_map_base+__direct_map_size{return (addr-__direct_map_base+TITAN_MEM_BIAS) as *mut u8;}
    if !(*hose).sg_pci.is_null() && baddr>=(*(*hose).sg_pci).dma_base && last<(*(*hose).sg_pci).dma_base+(*(*hose).sg_pci).size {
        let off=addr&!PAGE_MASK; let base=(baddr-(*(*hose).sg_pci).dma_base)&PAGE_MASK; let end=(last-(*(*hose).sg_pci).dma_base+PAGE_SIZE-1)&PAGE_MASK; let map=end-base;
        let area=get_vm_area(map,VM_IOREMAP); if area.is_null(){return core::ptr::null_mut();} let mut v=(*area).addr as usize; let mut p=base;
        while p<=end {let pfn=*(*hose).sg_pci.ptes.add(p>>PAGE_SHIFT);if pfn&1==0{vfree((*area).addr);return core::ptr::null_mut();}if __alpha_remap_area_pages(v,pfn>>1<<PAGE_SHIFT,PAGE_SIZE,0)!=0{vfree((*area).addr);return core::ptr::null_mut();}p+=PAGE_SIZE;v+=PAGE_SIZE;}
        flush_tlb_all(); return ((*area).addr as usize+off) as *mut u8;
    } (addr+TITAN_MEM_BIAS) as *mut u8
}

#[repr(C)] pub struct titan_agp_aperture { pub arena:*mut pci_iommu_arena, pub pg_start:isize, pub pg_count:isize }
unsafe fn titan_agp_setup(agp:*mut alpha_agp_info)->i32 { if alpha_agpgart_size==0{return -ENOMEM;} let aper=kmalloc_obj::<titan_agp_aperture>();if aper.is_null(){return -ENOMEM;}(*aper).arena=(*agp).hose.sg_pci;(*aper).pg_count=(alpha_agpgart_size/PAGE_SIZE) as isize;(*aper).pg_start=iommu_reserve((*aper).arena,(*aper).pg_count,(*aper).pg_count-1);if (*aper).pg_start<0{kfree(aper);return -ENOMEM;}(*agp).aperture.bus_base=(*(*aper).arena).dma_base+(*aper).pg_start as usize*PAGE_SIZE;(*agp).aperture.size=(*aper).pg_count as usize*PAGE_SIZE;(*agp).aperture.sysdata=aper as *mut _;0 }
unsafe fn titan_agp_cleanup(agp:*mut alpha_agp_info){let a=(*agp).aperture.sysdata as *mut titan_agp_aperture;let mut s=iommu_release((*a).arena,(*a).pg_start,(*a).pg_count);if s==-EBUSY{iommu_unbind((*a).arena,(*a).pg_start,(*a).pg_count);s=iommu_release((*a).arena,(*a).pg_start,(*a).pg_count);}kfree(a);kfree(agp);}
unsafe fn titan_agp_bind_memory(agp:*mut alpha_agp_info,pg:isize,mem:*mut agp_memory)->i32{let a=(*agp).aperture.sysdata as *mut titan_agp_aperture;iommu_bind((*a).arena,(*a).pg_start+pg,(*mem).page_count,(*mem).pages)}
unsafe fn titan_agp_unbind_memory(agp:*mut alpha_agp_info,pg:isize,mem:*mut agp_memory)->i32{let a=(*agp).aperture.sysdata as *mut titan_agp_aperture;iommu_unbind((*a).arena,(*a).pg_start+pg,(*mem).page_count)}
unsafe fn titan_agp_translate(agp:*mut alpha_agp_info,addr:dma_addr_t)->usize{let a=(*agp).aperture.sysdata as *mut titan_agp_aperture;let b=addr-(*(*a).arena).dma_base;let p=*(*a).arena.ptes.add(b>>PAGE_SHIFT);if p&1==0{return (-EINVAL) as usize;}p>>1<<PAGE_SHIFT}

pub unsafe fn titan_agp_info()->*mut alpha_agp_info { core::ptr::null_mut() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
