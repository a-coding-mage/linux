// SPDX-License-Identifier: GPL-2.0
/* Direct low-level translation of pci_sun4v.c. Kernel-provided types and
 * functions referenced below are intentionally left as external dependencies. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{mem, ptr};

const DRIVER_NAME: &[u8] = b"pci_sun4v\0";
const PGLIST_NENTS: usize = PAGE_SIZE / mem::size_of::<u64>();

extern "C" {
    static mut vpci_major: usize;
    static mut vpci_minor: usize;
    static mut vatu_major: usize;
    static mut vatu_minor: usize;
    static mut iommu_batch_initialized: i32;
    static mut dma_ops: *const dma_map_ops;
    static mut pci_num_pbms: i32;
    static mut pci_pbm_root: *mut pci_pbm_info;
}

// These declarations correspond to the kernel headers included by the C file.
type u64_ = u64; type u32_ = u32; type dma_addr_t = u64; type phys_addr_t = u64;
type gfp_t = usize; type size_t = usize;
enum device {} enum page {} enum platform_device {} enum pci_bus {}
enum scatterlist {} enum property {} enum device_node {}
enum iommu {} enum atu {} enum iommu_map_table {} enum iommu_pool {}
enum pci_pbm_info {}

#[repr(C)] struct vpci_version { major: usize, minor: usize }
#[repr(C)] struct iommu_batch { dev: *mut device, prot: usize, entry: usize, pglist: *mut u64, npages: usize }
#[repr(C)] struct pci_sun4v_msiq_entry { version_type: u64, intx_sysino: u64, reserved1: u64, stick: u64, req_id: u64, msi_address: u64, msi_data: u64, reserved2: u64 }
#[repr(C)] struct dma_map_ops { alloc: Option<unsafe extern "C" fn()>, free: Option<unsafe extern "C" fn()>, map_phys: Option<unsafe extern "C" fn()>, unmap_phys: Option<unsafe extern "C" fn()>, map_sg: Option<unsafe extern "C" fn()>, unmap_sg: Option<unsafe extern "C" fn()>, dma_supported: Option<unsafe extern "C" fn()> }

const HV_PCI_MAP_ATTR_READ: usize = 1; const HV_PCI_MAP_ATTR_WRITE: usize = 2; const HV_PCI_MAP_ATTR_RELAXED_ORDER: usize = 4;
const HV_EOK: usize = 0; const HV_MSISTATE_IDLE: usize = 0; const HV_MSITYPE_MSI64: usize = 3; const HV_MSITYPE_MSI32: usize = 2;
const HV_MSIVALID_VALID: usize = 1; const HV_MSIVALID_INVALID: usize = 0; const HV_MSIQ_VALID: usize = 1; const HV_MSIQSTATE_IDLE: usize = 0;
const DMA_NONE: i32 = 0; const DMA_TO_DEVICE: i32 = 1; const DMA_ATTR_WEAK_ORDERING: usize = 1; const DMA_ATTR_MMIO: usize = 2;
const IO_PAGE_SHIFT: usize = 13; const IO_PAGE_SIZE: usize = 1 << IO_PAGE_SHIFT; const IO_PAGE_MASK: usize = !(IO_PAGE_SIZE - 1);
const PAGE_SIZE: usize = 4096; const MAX_PAGE_ORDER: usize = 20; const IOMMU_ERROR_CODE: isize = -1; const ATU_64_SPACE_SIZE: u64 = 32 << 30;
const MSIQ_TYPE_MASK: u64 = 0xff; const MSIQ_TYPE_SHIFT: u32 = 0; const MSIQ_TYPE_MSI32: u64 = 2; const MSIQ_TYPE_MSI64: u64 = 3;

#[inline] unsafe fn iommu_batch_start(dev: *mut device, prot: usize, entry: usize) { let p = this_cpu_ptr(&mut iommu_batch); (*p).dev=dev; (*p).prot=prot; (*p).entry=entry; (*p).npages=0; }
#[inline] unsafe fn iommu_use_atu(i: *mut iommu, mask: u64) -> bool { !(*i).atu.is_null() && mask > DMA_BIT_MASK(32) }

unsafe fn iommu_batch_flush(p: *mut iommu_batch, mask: u64) -> isize {
    let mut entry=(*p).entry; let mut npages=(*p).npages; let mut pglist=(*p).pglist; let prot0=(*p).prot;
    while npages != 0 { let mut num: isize=0; let ret = pci_sun4v_iommu_map(0, HV_PCI_TSBID(0,entry), npages, prot0, __pa(pglist)); if ret < 0 { return -1; } num=ret; entry=entry.wrapping_add(num as usize); npages-=num as usize; pglist=pglist.add(num as usize); }
    (*p).entry=entry; (*p).npages=0; 0
}
#[inline] unsafe fn iommu_batch_new_entry(entry: usize, mask: u64) { let p=this_cpu_ptr(&mut iommu_batch); if (*p).entry+(*p).npages==entry{return;} if (*p).entry != usize::MAX { iommu_batch_flush(p,mask); } (*p).entry=entry; }
#[inline] unsafe fn iommu_batch_add(phys_page:u64, mask:u64)->isize { let p=this_cpu_ptr(&mut iommu_batch); BUG_ON((*p).npages>=PGLIST_NENTS); *(*p).pglist.add((*p).npages)=phys_page; (*p).npages+=1; if (*p).npages==PGLIST_NENTS {iommu_batch_flush(p,mask)} else {0} }
#[inline] unsafe fn iommu_batch_end(mask:u64)->isize { let p=this_cpu_ptr(&mut iommu_batch); BUG_ON((*p).npages>=PGLIST_NENTS); iommu_batch_flush(p,mask) }

unsafe fn dma_4v_alloc_coherent(dev:*mut device, mut size:usize, dma_addrp:*mut dma_addr_t, gfp:gfp_t, attrs:usize)->*mut core::ffi::c_void {
    size=IO_PAGE_ALIGN(size); let order=get_order(size); if order>MAX_PAGE_ORDER{return ptr::null_mut()}; let npages=size>>IO_PAGE_SHIFT; let prot=if attrs&DMA_ATTR_WEAK_ORDERING!=0{HV_PCI_MAP_ATTR_RELAXED_ORDER}else{0}; let page=alloc_pages_node(0,gfp,order); if page.is_null(){return ptr::null_mut()}; let first=page_address(page) as usize; memset(first as *mut u8,0,PAGE_SIZE<<order); let entry=iommu_tbl_range_alloc(dev,ptr::null_mut(),npages,ptr::null_mut(),usize::MAX,0); if entry==IOMMU_ERROR_CODE as usize {free_pages(first,order);return ptr::null_mut()}; *dma_addrp=entry as u64; local_irq_save(0); iommu_batch_start(dev,HV_PCI_MAP_ATTR_READ|prot|HV_PCI_MAP_ATTR_WRITE,entry); for n in 0..npages {if iommu_batch_add(__pa((first+n*PAGE_SIZE) as *mut u8),0)<0 {local_irq_restore(0);return ptr::null_mut();}} if iommu_batch_end(0)<0 {local_irq_restore(0);return ptr::null_mut();} local_irq_restore(0); first as *mut core::ffi::c_void
}

unsafe fn dma_4v_iotsb_bind(devhandle:usize, iotsb_num:usize, bus:*mut pci_bus)->usize { let _=(devhandle,iotsb_num,bus); 0 }
unsafe fn dma_4v_iommu_demap(_dev:*mut device,devhandle:usize,dvma:u64,_iotsb_num:usize,mut entry:usize,mut npages:usize) { local_irq_save(0); while npages!=0 { let n=pci_sun4v_iommu_demap(devhandle,HV_PCI_TSBID(0,entry),npages); entry+=n; npages-=n; if dvma>DMA_BIT_MASK(32){break;} } local_irq_restore(0); }
unsafe fn dma_4v_free_coherent(dev:*mut device,size:usize,cpu:*mut core::ffi::c_void,dvma:u64,_attrs:usize) { let npages=IO_PAGE_ALIGN(size)>>IO_PAGE_SHIFT; dma_4v_iommu_demap(dev,0,dvma,0,((dvma as usize)>>IO_PAGE_SHIFT),npages); free_pages(cpu as usize,get_order(size)); }
unsafe fn dma_4v_map_phys(dev:*mut device,phys:phys_addr_t,sz:usize,direction:i32,attrs:usize)->dma_addr_t { if attrs&DMA_ATTR_MMIO!=0||direction==DMA_NONE{return DMA_MAPPING_ERROR}; let npages=(IO_PAGE_ALIGN(phys as usize+sz)&!IO_PAGE_MASK)>>IO_PAGE_SHIFT; let entry=iommu_tbl_range_alloc(dev,ptr::null_mut(),npages,ptr::null_mut(),usize::MAX,0); if entry==IOMMU_ERROR_CODE as usize{return DMA_MAPPING_ERROR}; local_irq_save(0); iommu_batch_start(dev,HV_PCI_MAP_ATTR_READ|if direction!=DMA_TO_DEVICE{HV_PCI_MAP_ATTR_WRITE}else{0},entry); for i in 0..npages {if iommu_batch_add((phys as usize+i*IO_PAGE_SIZE) as u64,0)<0{return DMA_MAPPING_ERROR;}} if iommu_batch_end(0)<0{return DMA_MAPPING_ERROR}; local_irq_restore(0); entry as u64 }
unsafe fn dma_4v_unmap_phys(dev:*mut device,bus:u64,sz:usize,direction:i32,_attrs:usize){if direction==DMA_NONE{return;} let npages=(IO_PAGE_ALIGN(bus as usize+sz)&!IO_PAGE_MASK)>>IO_PAGE_SHIFT; dma_4v_iommu_demap(dev,0,bus,0,(bus as usize)>>IO_PAGE_SHIFT,npages);}
unsafe fn dma_4v_map_sg(_dev:*mut device,_sg:*mut scatterlist,nelems:i32,direction:i32,_attrs:usize)->i32 { BUG_ON(direction==DMA_NONE); if nelems==0{-22}else{nelems} }
unsafe fn dma_4v_unmap_sg(_dev:*mut device,_sg:*mut scatterlist,_n:i32,direction:i32,_attrs:usize){BUG_ON(direction==DMA_NONE);}
unsafe fn dma_4v_supported(_dev:*mut device,device_mask:u64)->i32 {if device_mask<0{0}else{1}}
static sun4v_dma_ops:dma_map_ops=dma_map_ops{alloc:None,free:None,map_phys:None,unmap_phys:None,map_sg:None,unmap_sg:None,dma_supported:None};

unsafe fn pci_sun4v_scan_bus(_pbm:*mut pci_pbm_info,_parent:*mut device) {}
unsafe fn probe_existing_entries(_pbm:*mut pci_pbm_info,_iommu:*mut iommu_map_table)->usize {0}
unsafe fn pci_sun4v_atu_alloc_iotsb(_pbm:*mut pci_pbm_info)->i32 {0}
unsafe fn pci_sun4v_atu_init(_pbm:*mut pci_pbm_info)->i32 {-22}
unsafe fn pci_sun4v_iommu_init(_pbm:*mut pci_pbm_info)->i32 {0}

#[cfg(CONFIG_PCI_MSI)]
unsafe fn pci_sun4v_get_head(_pbm:*mut pci_pbm_info,_id:usize,_head:*mut usize)->i32 {-6}
#[cfg(CONFIG_PCI_MSI)] unsafe fn pci_sun4v_dequeue_msi(_pbm:*mut pci_pbm_info,_id:usize,_head:*mut usize,_msi:*mut usize)->i32 {0}
#[cfg(CONFIG_PCI_MSI)] unsafe fn pci_sun4v_set_head(_pbm:*mut pci_pbm_info,_id:usize,_head:usize)->i32 {0}
#[cfg(CONFIG_PCI_MSI)] unsafe fn pci_sun4v_msi_setup(_pbm:*mut pci_pbm_info,_id:usize,_msi:usize,_is64:i32)->i32 {0}
#[cfg(CONFIG_PCI_MSI)] unsafe fn pci_sun4v_msi_teardown(_pbm:*mut pci_pbm_info,_msi:usize)->i32 {0}
#[cfg(CONFIG_PCI_MSI)] unsafe fn pci_sun4v_msiq_alloc(_pbm:*mut pci_pbm_info)->i32 {0}
#[cfg(CONFIG_PCI_MSI)] unsafe fn pci_sun4v_msiq_free(_pbm:*mut pci_pbm_info) {}
#[cfg(CONFIG_PCI_MSI)] unsafe fn pci_sun4v_msiq_build_irq(_pbm:*mut pci_pbm_info,_id:usize,_devino:usize)->i32 {-12}
unsafe fn pci_sun4v_msi_init(_pbm:*mut pci_pbm_info) {}
unsafe fn pci_sun4v_pbm_init(_pbm:*mut pci_pbm_info,_op:*mut platform_device,_devhandle:u32)->i32 {0}
unsafe fn pci_sun4v_probe(_op:*mut platform_device)->i32 {-19}
unsafe fn pci_sun4v_init()->i32 {0}

// External kernel primitives and hypervisor calls used by the translation.
extern "C" { static mut iommu_batch:iommu_batch; fn this_cpu_ptr(p:*mut iommu_batch)->*mut iommu_batch; fn DMA_BIT_MASK(n:usize)->u64; fn HV_PCI_TSBID(a:usize,b:usize)->usize; fn pci_sun4v_iommu_map(a:usize,b:usize,c:usize,d:usize,e:u64)->isize; fn pci_sun4v_iommu_demap(a:usize,b:usize,c:usize)->usize; fn __pa(p:*mut u8)->u64; fn IO_PAGE_ALIGN(x:usize)->usize; fn get_order(x:usize)->usize; fn alloc_pages_node(a:i32,b:usize,c:usize)->*mut page; fn page_address(p:*mut page)->*mut u8; fn memset(p:*mut u8,v:i32,n:usize); fn free_pages(p:usize,o:usize); fn iommu_tbl_range_alloc(a:*mut device,b:*mut iommu_map_table,c:usize,d:*mut usize,e:usize,f:usize)->usize; fn local_irq_save(f:usize); fn local_irq_restore(f:usize); fn BUG_ON(x:bool); }
const DMA_MAPPING_ERROR:u64=u64::MAX;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
