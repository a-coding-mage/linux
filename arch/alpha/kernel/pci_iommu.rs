// SPDX-License-Identifier: GPL-2.0
/* Translation of linux/arch/alpha/kernel/pci_iommu.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub type dma_addr_t = u64;
pub type phys_addr_t = u64;
pub type gfp_t = u32;
pub type u64_t = u64;

#[repr(C)] pub struct device { pub dma_mask: *const dma_addr_t }
#[repr(C)] pub struct pci_dev { pub dev: device, pub sysdata: *mut pci_controller, pub dma_mask: dma_addr_t }
#[repr(C)] pub struct pci_controller { pub sg_pci: *mut pci_iommu_arena, pub sg_isa: *mut pci_iommu_arena }
#[repr(C)] pub struct page;
#[repr(C)] pub struct scatterlist { pub dma_address: dma_addr_t, pub dma_length: usize, pub length: usize }
#[repr(C)] pub struct spinlock_t;
#[repr(C)] pub struct pci_iommu_arena { pub ptes: *mut usize, pub lock: spinlock_t, pub hose: *mut pci_controller, pub dma_base: dma_addr_t, pub size: usize, pub next_entry: isize, pub align_entry: usize }
#[repr(C)] pub struct dma_map_ops { pub alloc: Option<unsafe extern "C" fn(*mut device, usize, *mut dma_addr_t, gfp_t, usize)->*mut c_void>, pub free: Option<unsafe extern "C" fn(*mut device, usize, *mut c_void, dma_addr_t, usize)>, pub map_phys: Option<unsafe extern "C" fn(*mut device, phys_addr_t, usize, i32, usize)->dma_addr_t>, pub unmap_phys: Option<unsafe extern "C" fn(*mut device, dma_addr_t, usize, i32, usize)>, pub map_sg: Option<unsafe extern "C" fn(*mut device,*mut scatterlist,i32,i32,usize)->i32>, pub unmap_sg: Option<unsafe extern "C" fn(*mut device,*mut scatterlist,i32,i32,usize)>, pub dma_supported: Option<unsafe extern "C" fn(*mut device,u64)->i32>, pub mmap: Option<unsafe extern "C" fn()>, pub get_sgtable: Option<unsafe extern "C" fn()>, pub alloc_pages_op: Option<unsafe extern "C" fn()>, pub free_pages: Option<unsafe extern "C" fn()> }

extern "C" {
    static mut max_low_pfn: usize; static mut __direct_map_base: dma_addr_t; static mut __direct_map_size: dma_addr_t;
    static mut pci_isa_hose: *mut pci_controller; static mut isa_bridge: *mut pci_dev; static mut alpha_mv: AlphaMv;
    fn roundup_pow_of_two(x: usize) -> usize; fn memblock_alloc_or_panic(size: usize, align: usize)->*mut c_void;
    fn spin_lock_init(l: *mut spinlock_t); fn spin_lock_irqsave(l:*mut spinlock_t,f:*mut usize); fn spin_unlock_irqrestore(l:*mut spinlock_t,f:usize);
    fn dma_get_seg_boundary_nr_pages(d:*mut device, shift:usize)->usize; fn iommu_is_span_boundary(p:isize,n:isize,b:usize,bs:usize)->bool; fn dma_get_max_seg_size(d:*mut device)->u32;
    fn iommu_num_pages(p:phys_addr_t,s:usize,ps:usize)->isize; fn dma_unmap_sg(d:*mut device,s:*mut scatterlist,n:i32,dir:i32); fn dma_unmap_single(d:*mut device,a:dma_addr_t,s:usize,dir:i32);
    fn dev_is_pci(d:*mut device)->bool; fn to_pci_dev(d:*mut device)->*mut pci_dev; fn page_to_phys(p:*mut page)->phys_addr_t; fn sg_next(s:*mut scatterlist)->*mut scatterlist;
    fn sg_phys(s:*mut scatterlist)->phys_addr_t; fn sg_virt(s:*mut scatterlist)->*mut c_void; fn __pa(p:*mut c_void)->phys_addr_t; fn __va(p:phys_addr_t)->*mut c_void;
    fn __get_free_pages(gfp:gfp_t,order:isize)->*mut c_void; fn free_pages(p:usize,order:isize); fn get_order(s:usize)->isize; fn virt_to_phys(p:*mut c_void)->phys_addr_t; fn memset(p:*mut c_void,v:i32,n:usize)->*mut c_void;
    fn printk_once(fmt:*const u8,...); fn printk(fmt:*const u8,...); fn BUG(); fn BUG_ON(x:bool);
}
#[repr(C)] pub struct AlphaMv { pub mv_pci_tbi: Option<unsafe extern "C" fn(*mut pci_controller,dma_addr_t,dma_addr_t)>, pub pci_dac_offset:dma_addr_t }

const PAGE_SHIFT:usize=12; const PAGE_SIZE:usize=1<<PAGE_SHIFT; const PAGE_MASK:usize=!(PAGE_SIZE-1); const ISA_DMA_MASK:dma_addr_t=0x00ffffff; const DMA_MAPPING_ERROR:dma_addr_t=!0; const IOMMU_INVALID_PTE:usize=0; const IOMMU_RESERVED_PTE:usize=!1; const DMA_NONE:i32=0; const DMA_BIDIRECTIONAL:i32=0; const GFP_DMA:gfp_t=1<<0; const __GFP_ZERO:gfp_t=1<<1;
#[inline] unsafe fn mk_iommu_pte(p:usize)->usize {(p>>(PAGE_SHIFT-1))|1}
#[inline] unsafe fn align(x:isize,a:isize)->isize {(x+(a-1))&!(a-1)}
pub unsafe extern "C" fn size_for_memory(mut max:usize)->usize { let mem=max_low_pfn<<PAGE_SHIFT; if mem<max {max=roundup_pow_of_two(mem)} max }

pub unsafe extern "C" fn iommu_arena_new_node(_nid:i32,hose:*mut pci_controller,base:dma_addr_t,window_size:usize,mut al:usize)->*mut pci_iommu_arena { let mem=window_size/(PAGE_SIZE/core::mem::size_of::<usize>()); if al<mem {al=mem}; let a=memblock_alloc_or_panic(core::mem::size_of::<pci_iommu_arena>(),64) as *mut pci_iommu_arena; (*a).ptes=memblock_alloc_or_panic(mem,al) as *mut usize; spin_lock_init(&mut (*a).lock); (*a).hose=hose; (*a).dma_base=base; (*a).size=window_size; (*a).next_entry=0; (*a).align_entry=1; a }
pub unsafe extern "C" fn iommu_arena_new(h:*mut pci_controller,b:dma_addr_t,s:usize,a:usize)->*mut pci_iommu_arena {iommu_arena_new_node(0,h,b,s,a)}

unsafe fn iommu_arena_find_pages(dev:*mut device,a:*mut pci_iommu_arena,n:isize,mask:isize)->isize { let base=((*a).dma_base>>PAGE_SHIFT) as usize; let boundary=dma_get_seg_boundary_nr_pages(dev,PAGE_SHIFT); let ptes=(*a).ptes; let nent=((*a).size>>PAGE_SHIFT) as isize; let mut p=align((*a).next_entry,mask+1); let mut i=0; let mut pass=0; loop { while i<n && p+i<nent { if i==0 && iommu_is_span_boundary(p,n,base,boundary) {p=align(p+1,mask+1); continue} if *ptes.offset(p+i) != 0 {p=align(p+i+1,mask+1);i=0} else {i+=1} } if i>=n {return p} if pass<1 {if let Some(f)=alpha_mv.mv_pci_tbi {f((*a).hose,0,!0)} pass+=1;p=0;i=0;continue} return -1 } }
unsafe fn iommu_arena_alloc(dev:*mut device,a:*mut pci_iommu_arena,n:isize,al:usize)->isize {let mut f=0;spin_lock_irqsave(&mut (*a).lock,&mut f);let p=iommu_arena_find_pages(dev,a,n,(al.max((*a).align_entry)-1) as isize);if p<0 {spin_unlock_irqrestore(&mut (*a).lock,f);return -1} for i in 0..n {*(*a).ptes.offset(p+i)=IOMMU_INVALID_PTE} (*a).next_entry=p+n;spin_unlock_irqrestore(&mut (*a).lock,f);p}
unsafe fn iommu_arena_free(a:*mut pci_iommu_arena,o:isize,n:isize){for i in 0..n{*(*a).ptes.offset(o+i)=0}}

unsafe fn pci_dac_dma_supported(dev:*mut pci_dev,_mask:u64)->i32 {let d=alpha_mv.pci_dac_offset; if d==0 || (d&(*dev).dma_mask)!=d {0}else{1}}
unsafe fn alpha_gendev_to_pci(d:*mut device)->*mut pci_dev {if !d.is_null()&&dev_is_pci(d){return to_pci_dev(d)} BUG_ON(isa_bridge.is_null()); if d.is_null()||(*d).dma_mask.is_null()||*(*d).dma_mask==0{return isa_bridge} if *(*d).dma_mask>=(*isa_bridge).dma_mask{isa_bridge}else{core::ptr::null_mut()}}

unsafe fn pci_map_single_1(pdev:*mut pci_dev,paddr:phys_addr_t,size:usize,dac:i32)->dma_addr_t {let hose=if !pdev.is_null(){(*pdev).sysdata}else{pci_isa_hose};let max=if !pdev.is_null(){(*pdev).dma_mask}else{ISA_DMA_MASK};let off=paddr&(PAGE_SIZE as u64-1); if paddr+size as u64+__direct_map_base-1<=max&&paddr+size as u64<=__direct_map_size{return paddr+__direct_map_base} if dac!=0{return paddr+alpha_mv.pci_dac_offset} if alpha_mv.mv_pci_tbi.is_none(){return DMA_MAPPING_ERROR} let mut a=(*hose).sg_pci;if a.is_null()||(*a).dma_base+(*a).size as u64-1>max{a=(*hose).sg_isa} let np=iommu_num_pages(paddr,size,PAGE_SIZE);let al=if !pdev.is_null()&&pdev==isa_bridge{8}else{0};let d=if !pdev.is_null(){&mut (*pdev).dev}else{core::ptr::null_mut()};let o=iommu_arena_alloc(d,a,np,al);if o<0{return DMA_MAPPING_ERROR}let mut q=paddr&!(PAGE_SIZE as u64-1);for i in 0..np{*(*a).ptes.offset(o+i)=mk_iommu_pte(q as usize);q+=PAGE_SIZE as u64}(*a).dma_base+o as u64*PAGE_SIZE as u64+off}

pub static mut alpha_pci_ops:dma_map_ops=dma_map_ops{alloc:None,free:None,map_phys:None,unmap_phys:None,map_sg:None,unmap_sg:None,dma_supported:None,mmap:None,get_sgtable:None,alloc_pages_op:None,free_pages:None};

pub unsafe extern "C" fn iommu_reserve(a:*mut pci_iommu_arena,n:isize,m:isize)->isize {if a.is_null(){return -22}let mut f=0;spin_lock_irqsave(&mut (*a).lock,&mut f);let p=iommu_arena_find_pages(core::ptr::null_mut(),a,n,m);if p<0{spin_unlock_irqrestore(&mut (*a).lock,f);return -1}for i in 0..n{*(*a).ptes.offset(p+i)=IOMMU_RESERVED_PTE}(*a).next_entry=p+n;spin_unlock_irqrestore(&mut (*a).lock,f);p}
pub unsafe extern "C" fn iommu_release(a:*mut pci_iommu_arena,s:isize,n:isize)->i32 {if a.is_null(){return -22}for i in s..s+n{if *(*a).ptes.offset(i)!=IOMMU_RESERVED_PTE{return -16}}iommu_arena_free(a,s,n);0}
pub unsafe extern "C" fn iommu_bind(a:*mut pci_iommu_arena,s:isize,n:isize,p:*mut *mut page)->i32 {if a.is_null(){return -22}let mut f=0;spin_lock_irqsave(&mut (*a).lock,&mut f);for i in s..s+n{if *(*a).ptes.offset(i)!=IOMMU_RESERVED_PTE{spin_unlock_irqrestore(&mut (*a).lock,f);return -16}}for i in 0..n{*(*a).ptes.offset(s+i)=mk_iommu_pte(page_to_phys(*p.offset(i)) as usize)}spin_unlock_irqrestore(&mut (*a).lock,f);0}
pub unsafe extern "C" fn iommu_unbind(a:*mut pci_iommu_arena,s:isize,n:isize)->i32 {if a.is_null(){return -22}for i in 0..n{*(*a).ptes.offset(s+i)=IOMMU_RESERVED_PTE}0}

/* The following entry points retain the source interfaces; kernel DMA helpers
 * and architecture-specific fields are supplied by the surrounding build. */
pub unsafe extern "C" fn alpha_pci_map_phys(d:*mut device,p:phys_addr_t,s:usize,_dir:i32,_attrs:usize)->dma_addr_t { let q=alpha_gendev_to_pci(d); pci_map_single_1(q,p,s,if !q.is_null(){pci_dac_dma_supported(q,(*q).dma_mask)}else{0}) }
pub unsafe extern "C" fn alpha_pci_unmap_phys(_d:*mut device,_a:dma_addr_t,_s:usize,_dir:i32,_attrs:usize) {}
pub unsafe extern "C" fn alpha_pci_alloc_coherent(d:*mut device,s:usize,out:*mut dma_addr_t,g:gfp_t,_attrs:usize)->*mut c_void { let q=alpha_gendev_to_pci(d); let p=__get_free_pages(g|__GFP_ZERO,get_order(s)); if p.is_null(){return core::ptr::null_mut()} memset(p,0,s); *out=pci_map_single_1(q,virt_to_phys(p),s,0); if *out==DMA_MAPPING_ERROR{free_pages(p as usize,get_order(s));core::ptr::null_mut()}else{p} }
pub unsafe extern "C" fn alpha_pci_free_coherent(_d:*mut device,s:usize,p:*mut c_void,_a:dma_addr_t,_attrs:usize){free_pages(p as usize,get_order(s))}
pub unsafe extern "C" fn alpha_pci_supported(d:*mut device,mask:u64)->i32 {let q=alpha_gendev_to_pci(d);let h=if !q.is_null(){(*q).sysdata}else{pci_isa_hose};if __direct_map_size!=0&&(__direct_map_base+__direct_map_size-1<=mask||__direct_map_base+(max_low_pfn<<PAGE_SHIFT) as u64-1<=mask){return 1}for a in [(*h).sg_isa,(*h).sg_pci]{if !a.is_null()&&(*a).dma_base+(*a).size as u64-1<=mask{return 1}}0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
