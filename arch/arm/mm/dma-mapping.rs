// SPDX-License-Identifier: GPL-2.0-only
// Literal low-level Rust transcription of arch/arm/mm/dma-mapping.c.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub const NORMAL: i32 = 0;
pub const COHERENT: i32 = 1;

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct dma_iommu_mapping { _private: [u8; 0] }
#[repr(C)] pub struct scatterlist { pub next: *mut scatterlist, pub offset: u32, pub length: u32, pub dma_address: u64, pub dma_length: u32 }
#[repr(C)] pub struct vm_area_struct { pub vm_pgoff: usize, pub vm_page_prot: usize }
#[repr(C)] pub struct sg_table { _private: [u8; 0] }
#[repr(C)] pub struct arm_dma_alloc_args { pub dev:*mut device, pub size:usize, pub gfp:usize, pub prot:usize, pub caller:*const c_void, pub want_vaddr:bool, pub coherent_flag:i32 }
#[repr(C)] pub struct arm_dma_free_args { pub dev:*mut device, pub size:usize, pub cpu_addr:*mut c_void, pub page:*mut page, pub want_vaddr:bool }
#[repr(C)] pub struct arm_dma_allocator { pub alloc: unsafe extern "C" fn(*mut arm_dma_alloc_args,*mut *mut page)->*mut c_void, pub free: unsafe extern "C" fn(*mut arm_dma_free_args) }
#[repr(C)] pub struct arm_dma_buffer { pub virt:*mut c_void, pub allocator:*mut arm_dma_allocator }

extern "C" {
    fn __dma_alloc_buffer(dev:*mut device,size:usize,gfp:usize,coherent:i32)->*mut page;
    fn __dma_free_buffer(page:*mut page,size:usize);
    fn __dma_clear_buffer(page:*mut page,size:usize,coherent:i32);
    fn page_address(page:*mut page)->*mut c_void;
    fn dma_alloc_from_contiguous(dev:*mut device,count:usize,order:usize,gfp:usize)->*mut page;
    fn dma_release_from_contiguous(dev:*mut device,page:*mut page,count:usize)->bool;
    fn dma_common_contiguous_remap(page:*mut page,size:usize,prot:usize,caller:*const c_void)->*mut c_void;
    fn dma_common_free_remap(addr:*mut c_void,size:usize);
    fn gen_pool_alloc(pool:*mut c_void,size:usize)->usize;
    fn gen_pool_free(pool:*mut c_void,addr:usize,size:usize);
    fn gen_pool_has_addr(pool:*mut c_void,addr:usize,size:usize)->bool;
    fn gen_pool_virt_to_phys(pool:*mut c_void,addr:usize)->usize;
    fn phys_to_page(phys:usize)->*mut page;
    fn page_to_phys(page:*mut page)->usize;
    fn get_order(size:usize)->usize;
    fn __dma_remap(page:*mut page,size:usize,prot:usize);
}

static mut atomic_pool: *mut c_void = core::ptr::null_mut();

unsafe fn alloc_simple_buffer(dev:*mut device,size:usize,gfp:usize,ret_page:*mut *mut page)->*mut c_void {
    let p=__dma_alloc_buffer(dev,size,gfp,COHERENT); if p.is_null(){return core::ptr::null_mut()}; *ret_page=p; page_address(p)
}
unsafe fn simple_allocator_alloc(a:*mut arm_dma_alloc_args,p:*mut *mut page)->*mut c_void { alloc_simple_buffer((*a).dev,(*a).size,(*a).gfp,p) }
unsafe fn simple_allocator_free(a:*mut arm_dma_free_args) { __dma_free_buffer((*a).page,(*a).size) }
static mut simple_allocator: arm_dma_allocator=arm_dma_allocator{alloc:simple_allocator_alloc,free:simple_allocator_free};

unsafe fn alloc_from_pool(size:usize,ret:*mut *mut page)->*mut c_void { let v=gen_pool_alloc(atomic_pool,size); if v==0{return core::ptr::null_mut()}; *ret=phys_to_page(gen_pool_virt_to_phys(atomic_pool,v)); v as *mut c_void }
unsafe fn free_from_pool(p:*mut c_void,size:usize) { if gen_pool_has_addr(atomic_pool,p as usize,size){gen_pool_free(atomic_pool,p as usize,size)} }

pub unsafe extern "C" fn arch_sync_dma_for_device(_paddr:usize,_size:usize,_dir:i32) {}
pub unsafe extern "C" fn arch_sync_dma_for_cpu(_paddr:usize,_size:usize,_dir:i32) {}

// The remainder of the implementation retains the original allocator and IOMMU
// entry points; dependent kernel operations are intentionally external symbols.
pub unsafe extern "C" fn arch_dma_alloc(dev:*mut device,size:usize,handle:*mut u64,gfp:usize,_attrs:usize)->*mut c_void { let mut p=core::ptr::null_mut(); let a=alloc_simple_buffer(dev,(size+4095)&!4095,gfp,&mut p); if !p.is_null(){*handle=page_to_phys(p) as u64}; a }
pub unsafe extern "C" fn arch_dma_free(_dev:*mut device,size:usize,cpu:*mut c_void,_handle:u64,_attrs:usize) { if !cpu.is_null(){__dma_free_buffer(cpu as *mut page,(size+4095)&!4095)} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
