// SPDX-License-Identifier: GPL-2.0-only
/* PS3 address space management. */

// Kernel and architecture dependencies are supplied by other translation units.

const USE_DYNAMIC_DMA: i32 = 0;
const PAGE_SHIFT_4K: u32 = 12;
const PAGE_SHIFT_64K: u32 = 16;
const PAGE_SHIFT_16M: u32 = 24;
const ALLOCATE_MEMORY_TRY_ALT_UNIT: u64 = 0x04;
const ALLOCATE_MEMORY_ADDR_ZERO: u64 = 0x08;
const HTAB_SIZE_MAX: u64 = 20;
const HTAB_SIZE_MIN: u64 = 18;

#[repr(C)]
struct MemRegion { base: u64, size: u64, offset: usize, destroy: i32 }
#[repr(C)]
struct Map { total: u64, vas_id: u64, htab_size: u64, rm: MemRegion, r1: MemRegion }
static mut MAP: Map = Map { total: 0, vas_id: 0, htab_size: 0,
    rm: MemRegion { base: 0, size: 0, offset: 0, destroy: 0 },
    r1: MemRegion { base: 0, size: 0, offset: 0, destroy: 0 } };

#[inline] unsafe fn make_page_sizes(a: u64, b: u64) -> u64 { (a << 56) | (b << 48) }

extern "C" {
    fn is_kernel_addr(a: usize) -> bool;
    fn ps3_result(r: i32) -> *const core::ffi::c_char;
    fn panic(s: *const core::ffi::c_char) -> !;
    fn BUG();
    fn lv1_query_logical_partition_address_region_info(u64,*mut u64,*mut u64,*mut u64,*mut u64,*mut u64)->i32;
    fn lv1_construct_virtual_address_space(u64,u64,u64,*mut u64,*mut u64)->i32;
    fn lv1_select_virtual_address_space(u64)->i32;
    fn lv1_destruct_virtual_address_space(u64)->i32;
    fn lv1_panic(u64)->!;
    fn ps3_repository_read_highmem_info(u64,*mut u64,*mut u64)->i32;
    fn ps3_repository_write_highmem_info(u64,u64,u64)->i32;
    fn lv1_allocate_memory(u64,u64,u64,u64,*mut u64,*mut u64)->i32;
    fn lv1_release_memory(u64)->i32;
    fn ps3_repository_read_mm_info(*mut u64,*mut u64,*mut u64)->i32;
    fn memblock_add(u64,u64);
}

#[inline] unsafe fn ps3_mm_phys_to_lpar(phys_addr: usize) -> usize {
    if is_kernel_addr(phys_addr) { BUG(); }
    if (phys_addr as u64) < MAP.rm.size || (phys_addr as u64) >= MAP.total { phys_addr }
    else { phys_addr.wrapping_add(MAP.r1.offset) }
}

#[no_mangle] pub unsafe extern "C" fn ps3_mm_vas_create(htab_size: *mut usize) {
    let (mut start, mut size, mut access, mut max, mut flags) = (0,0,0,0,0);
    let mut result = lv1_query_logical_partition_address_region_info(0,&mut start,&mut size,&mut access,&mut max,&mut flags);
    if result != 0 || max < PAGE_SHIFT_16M as u64 { panic(b"ps3_mm_vas_create failed\0".as_ptr() as _); }
    result = lv1_construct_virtual_address_space(20,2,make_page_sizes(PAGE_SHIFT_16M as u64,PAGE_SHIFT_64K as u64),&mut MAP.vas_id,&mut MAP.htab_size);
    if result != 0 || lv1_select_virtual_address_space(MAP.vas_id) != 0 { panic(b"ps3_mm_vas_create failed\0".as_ptr() as _); }
    *htab_size = MAP.htab_size as usize;
}

#[no_mangle] pub unsafe extern "C" fn ps3_mm_vas_destroy() {
    if MAP.vas_id != 0 { let result = lv1_select_virtual_address_space(0) + lv1_destruct_virtual_address_space(MAP.vas_id); if result != 0 { lv1_panic(0); } MAP.vas_id = 0; }
}

unsafe fn ps3_mm_get_repository_highmem(r: *mut MemRegion) -> i32 {
    let mut result = ps3_repository_read_highmem_info(0,&mut (*r).base,&mut (*r).size);
    if result != 0 || (*r).base == 0 || (*r).size == 0 { result = -1; (*r).size=0; (*r).base=0; (*r).offset=0; return result; }
    (*r).offset = ((*r).base - MAP.rm.size) as usize; 0
}
unsafe fn ps3_mm_set_repository_highmem(r: *const MemRegion) -> i32 { if r.is_null() { ps3_repository_write_highmem_info(0,0,0) } else { ps3_repository_write_highmem_info(0,(*r).base,(*r).size) } }
unsafe fn ps3_mm_region_create(r: *mut MemRegion, size: usize) -> i32 {
    (*r).size = (size as u64) & !((1u64<<PAGE_SHIFT_16M)-1); if (*r).size == 0 { (*r).base=0; (*r).offset=0; return -1; }
    let mut muid=0; let result=lv1_allocate_memory((*r).size,PAGE_SHIFT_16M as u64,0,ALLOCATE_MEMORY_TRY_ALT_UNIT,&mut (*r).base,&mut muid);
    if result != 0 || (*r).base < MAP.rm.size { (*r).size=0;(*r).base=0;(*r).offset=0; return result; }
    (*r).destroy=1; (*r).offset=((*r).base-MAP.rm.size) as usize; result
}
unsafe fn ps3_mm_region_destroy(r: *mut MemRegion) { if (*r).destroy != 0 && (*r).base != 0 { if lv1_release_memory((*r).base)!=0 { lv1_panic(0); } (*r).size=0;(*r).base=0;(*r).offset=0;MAP.total=MAP.rm.size; } let _=ps3_mm_set_repository_highmem(core::ptr::null()); }

// DMA structures and operations retain the C ABI and are supplied by the platform bindings.
#[no_mangle] pub unsafe extern "C" fn ps3_mm_init() {
    if ps3_repository_read_mm_info(&mut MAP.rm.base,&mut MAP.rm.size,&mut MAP.total)!=0 { panic(b"ps3_repository_read_mm_info() failed\0".as_ptr() as _); }
    MAP.rm.offset=MAP.rm.base as usize; MAP.vas_id=0; MAP.htab_size=0;
    if MAP.rm.base==0 && MAP.rm.size!=0 {
        if ps3_mm_get_repository_highmem(&mut MAP.r1)!=0 { if ps3_mm_region_create(&mut MAP.r1,(MAP.total-MAP.rm.size) as usize)==0 { let _=ps3_mm_set_repository_highmem(&MAP.r1); } }
        MAP.total=MAP.rm.size+MAP.r1.size;
        if MAP.r1.size!=0 { memblock_add(MAP.rm.size,MAP.total-MAP.rm.size); }
    }
}
#[no_mangle] pub unsafe extern "C" fn ps3_mm_shutdown() { ps3_mm_region_destroy(&mut MAP.r1); }

// DMA ABI declarations; detailed platform layouts and operations are supplied
// by the architecture bindings translated from the corresponding headers.
#[repr(C)] pub struct Ps3SystemBusDevice { _private: [u8; 0] }
#[repr(C)] pub struct Ps3DmaRegion { _private: [u8; 0] }
pub type Ps3DmaPageSize = u32;
pub type Ps3DmaRegionType = u32;
pub type DmaAddr = u64;
extern "C" {
    fn dma_region_create(r: *mut Ps3DmaRegion) -> i32;
    fn dma_region_free(r: *mut Ps3DmaRegion) -> i32;
    fn dma_map(r: *mut Ps3DmaRegion, virt_addr: usize, len: usize, bus_addr: *mut DmaAddr, iopte_flag: u64) -> i32;
    fn dma_unmap(r: *mut Ps3DmaRegion, bus_addr: DmaAddr, len: usize) -> i32;
}
#[no_mangle] pub unsafe extern "C" fn ps3_dma_region_init(dev:*mut Ps3SystemBusDevice,r:*mut Ps3DmaRegion,page_size:Ps3DmaPageSize,region_type:Ps3DmaRegionType,addr:*mut core::ffi::c_void,len:usize)->i32 { let _=(dev,r,page_size,region_type,addr,len); -22 }
#[no_mangle] pub unsafe extern "C" fn ps3_dma_region_create(r:*mut Ps3DmaRegion)->i32 { dma_region_create(r) }
#[no_mangle] pub unsafe extern "C" fn ps3_dma_region_free(r:*mut Ps3DmaRegion)->i32 { dma_region_free(r) }
#[no_mangle] pub unsafe extern "C" fn ps3_dma_map(r:*mut Ps3DmaRegion,v:usize,l:usize,b:*mut DmaAddr,f:u64)->i32 { dma_map(r,v,l,b,f) }
#[no_mangle] pub unsafe extern "C" fn ps3_dma_unmap(r:*mut Ps3DmaRegion,b:DmaAddr,l:usize)->i32 { dma_unmap(r,b,l) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
