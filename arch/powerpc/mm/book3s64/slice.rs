// SPDX-License-Identifier: GPL-2.0-or-later
/* address space "slices" (meta-segments) support */

// Kernel headers and configuration-provided symbols are external dependencies.

extern "C" {
    static mut slice_convert_lock: SpinLock;
}

#[allow(non_camel_case_types)]
type ulong = usize;
#[allow(non_camel_case_types)]
type u64_t = u64;

#[repr(C)] pub struct SpinLock { _private: [u8; 0] }
#[repr(C)] pub struct mm_struct { pub context: mm_context, pub task_size: ulong, pub mmap_base: ulong }
#[repr(C)] pub struct mm_context { _private: [u8; 0] }
#[repr(C)] pub struct vm_area_struct { pub vm_mm: *mut mm_struct, pub vm_start: ulong }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct hstate { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { pub active_mm: *mut mm_struct, pub mm: *mut mm_struct }
#[repr(C)] pub struct slice_mask { pub low_slices: ulong, pub high_slices: [ulong; 0] }
#[repr(C)] pub struct vm_unmapped_area_info { pub flags: ulong, pub length: ulong, pub low_limit: ulong, pub high_limit: ulong, pub align_mask: ulong }
#[repr(C)] pub struct mmu_psize_def { pub shift: i32 }

extern "C" {
    static mut current: *mut task_struct;
    static mut mmu_psize_defs: *mut mmu_psize_def;
    static mut mmap_min_addr: ulong;
    static mut mmu_virtual_psize: u32;
    fn mm_ctx_slb_addr_limit(c: *const mm_context) -> ulong;
    fn mm_ctx_set_slb_addr_limit(c: *mut mm_context, v: ulong);
    fn mm_ctx_set_user_psize(c: *mut mm_context, v: u32);
    fn mm_ctx_user_psize(c: *const mm_context) -> u32;
    fn mm_ctx_low_slices(c: *mut mm_context) -> *mut u8;
    fn mm_ctx_high_slices(c: *mut mm_context) -> *mut u8;
    fn slice_mask_for_size(c: *mut mm_context, psize: i32) -> *mut slice_mask;
    fn find_vma(mm: *mut mm_struct, addr: ulong) -> *mut vm_area_struct;
    fn vm_start_gap(vma: *mut vm_area_struct) -> ulong;
    fn vm_unmapped_area(info: *mut vm_unmapped_area_info) -> ulong;
    fn radix_enabled() -> bool;
    fn need_extra_context(mm: *mut mm_struct, addr: ulong) -> bool;
    fn alloc_extended_context(mm: *mut mm_struct, addr: ulong) -> i32;
    fn on_each_cpu(f: unsafe extern "C" fn(*mut core::ffi::c_void), arg: *mut core::ffi::c_void, wait: i32);
    fn slb_flush_and_restore_bolted();
    fn copy_mm_to_paca(mm: *mut mm_struct);
    fn generic_get_unmapped_area(f: *mut file, addr: ulong, len: ulong, off: ulong, flags: ulong, vm_flags: ulong) -> ulong;
    fn generic_get_unmapped_area_topdown(f: *mut file, addr: ulong, len: ulong, off: ulong, flags: ulong, vm_flags: ulong) -> ulong;
    fn legacy_to_vma_flags(v: ulong) -> ulong;
    fn is_file_hugepages(f: *mut file) -> bool;
    fn hstate_file(f: *mut file) -> *mut hstate;
    fn huge_page_shift(h: *mut hstate) -> i32;
    fn shift_to_mmu_psize(v: i32) -> i32;
    fn is_32bit_task() -> bool;
    fn vma_kernel_pagesize(vma: *mut vm_area_struct) -> ulong;
    fn mmu_psize_to_shift(v: u32) -> i32;
    fn spu_flush_all_slbs(mm: *mut mm_struct);
}

const ENOMEM: ulong = (!0usize) - 11;
const EINVAL: ulong = (!0usize) - 21;
const EBUSY: ulong = (!0usize) - 15;

#[inline] unsafe fn slice_addr_is_low(addr: ulong) -> bool { (addr as u64) < SLICE_LOW_TOP as u64 }

unsafe fn slice_range_to_mask(start: ulong, len: ulong, ret: *mut slice_mask) {
    let end = start.wrapping_add(len).wrapping_sub(1);
    (*ret).low_slices = 0;
    if SLICE_NUM_HIGH != 0 { bitmap_zero((*ret).high_slices.as_mut_ptr(), SLICE_NUM_HIGH); }
    if slice_addr_is_low(start) {
        let mend = core::cmp::min(end, SLICE_LOW_TOP - 1);
        (*ret).low_slices = (1usize << (get_low_slice_index(mend) + 1)) - (1usize << get_low_slice_index(start));
    }
    if SLICE_NUM_HIGH != 0 && !slice_addr_is_low(end) {
        let si = get_high_slice_index(start);
        let ae = align(end, 1usize << SLICE_HIGH_SHIFT);
        bitmap_set((*ret).high_slices.as_mut_ptr(), si, get_high_slice_index(ae) - si);
    }
}

unsafe fn slice_area_is_free(mm: *mut mm_struct, addr: ulong, len: ulong) -> i32 {
    if mm_ctx_slb_addr_limit(&(*mm).context).wrapping_sub(len) < addr { return 0; }
    let vma = find_vma(mm, addr);
    if vma.is_null() || addr.wrapping_add(len) <= vm_start_gap(vma) { 1 } else { 0 }
}
unsafe fn slice_low_has_vma(mm: *mut mm_struct, slice: ulong) -> i32 { !slice_area_is_free(mm, slice << SLICE_LOW_SHIFT, 1usize << SLICE_LOW_SHIFT) as i32 }
unsafe fn slice_high_has_vma(mm: *mut mm_struct, slice: ulong) -> i32 {
    let mut start = slice << SLICE_HIGH_SHIFT; let end = start + (1usize << SLICE_HIGH_SHIFT);
    if start == 0 { start = SLICE_LOW_TOP; }
    !slice_area_is_free(mm, start, end - start) as i32
}

unsafe fn slice_mask_for_free(mm: *mut mm_struct, ret: *mut slice_mask, high_limit: ulong) {
    (*ret).low_slices = 0; if SLICE_NUM_HIGH != 0 { bitmap_zero((*ret).high_slices.as_mut_ptr(), SLICE_NUM_HIGH); }
    for i in 0..SLICE_NUM_LOW { if slice_low_has_vma(mm, i) == 0 { (*ret).low_slices |= 1usize << i; } }
    if slice_addr_is_low(high_limit - 1) { return; }
    for i in 0..get_high_slice_index(high_limit) { if slice_high_has_vma(mm, i) == 0 { set_bit(i, (*ret).high_slices.as_mut_ptr()); } }
}

unsafe fn slice_check_range_fits(_mm: *mut mm_struct, available: *const slice_mask, start: ulong, len: ulong) -> bool {
    let end = start + len - 1; let mut low = 0;
    if slice_addr_is_low(start) { let mend = core::cmp::min(end, SLICE_LOW_TOP - 1); low = (1usize << (get_low_slice_index(mend)+1)) - (1usize << get_low_slice_index(start)); }
    if low & (*available).low_slices != low { return false; }
    if SLICE_NUM_HIGH != 0 && !slice_addr_is_low(end) { let si=get_high_slice_index(start); let ae=align(end,1usize<<SLICE_HIGH_SHIFT); let count=get_high_slice_index(ae)-si; for i in si..si+count { if test_bit(i, (*available).high_slices.as_ptr()) == 0 { return false; } } }
    true
}

unsafe fn slice_scan_available(addr: ulong, available: *const slice_mask, end: i32, boundary: *mut ulong) -> bool {
    if slice_addr_is_low(addr) { let s=get_low_slice_index(addr); *boundary=(s+end as ulong)<<SLICE_LOW_SHIFT; ((*available).low_slices & (1usize<<s)) != 0 }
    else { let s=get_high_slice_index(addr); *boundary=if s+end as ulong != 0 {(s+end as ulong)<<SLICE_HIGH_SHIFT} else {SLICE_LOW_TOP}; test_bit(s,(*available).high_slices.as_ptr()) != 0 }
}

unsafe fn slice_copy_mask(dst:*mut slice_mask, src:*const slice_mask) { (*dst).low_slices=(*src).low_slices; if SLICE_NUM_HIGH!=0 { bitmap_copy((*dst).high_slices.as_mut_ptr(),(*src).high_slices.as_ptr(),SLICE_NUM_HIGH); } }
unsafe fn slice_or_mask(dst:*mut slice_mask,a:*const slice_mask,b:*const slice_mask) { (*dst).low_slices=(*a).low_slices|(*b).low_slices; if SLICE_NUM_HIGH!=0 { bitmap_or((*dst).high_slices.as_mut_ptr(),(*a).high_slices.as_ptr(),(*b).high_slices.as_ptr(),SLICE_NUM_HIGH); } }
unsafe fn slice_andnot_mask(dst:*mut slice_mask,a:*const slice_mask,b:*const slice_mask) { (*dst).low_slices=(*a).low_slices&!(*b).low_slices; if SLICE_NUM_HIGH!=0 { bitmap_andnot((*dst).high_slices.as_mut_ptr(),(*a).high_slices.as_ptr(),(*b).high_slices.as_ptr(),SLICE_NUM_HIGH); } }

// The remaining exported entry points retain the kernel implementation's ABI and
// call into the externally supplied slice allocation and bitmap primitives.
#[no_mangle] pub unsafe extern "C" fn slice_get_unmapped_area(addr:ulong,len:ulong,flags:ulong,psize:u32,topdown:i32)->ulong { let mm=(*current).mm; let high=if addr>=DEFAULT_MAP_WINDOW || (flags&MAP_FIXED!=0 && addr+len>DEFAULT_MAP_WINDOW){TASK_SIZE}else{DEFAULT_MAP_WINDOW}; if len>high{return ENOMEM}; if len & ((1usize<<core::cmp::max((*mmu_psize_defs.add(psize as usize)).shift,PAGE_SHIFT))-1)!=0{return EINVAL}; if flags&MAP_FIXED!=0 && (addr&((1usize<<core::cmp::max((*mmu_psize_defs.add(psize as usize)).shift,PAGE_SHIFT))-1)!=0){return EINVAL}; if flags&MAP_FIXED!=0 && addr>high-len{return ENOMEM}; let mut good=slice_mask_zero(); let mask=slice_mask_for_size(&mut (*mm).context,psize as i32); slice_copy_mask(&mut good,mask); if addr!=0 || flags&MAP_FIXED!=0 { if slice_check_range_fits(mm,&good,addr,len){return addr;} } let mut free=slice_mask_zero(); slice_mask_for_free(mm,&mut free,high); slice_or_mask(&mut free,&free,&good); if addr!=0 || flags&MAP_FIXED!=0 {if slice_check_range_fits(mm,&free,addr,len){return addr;} } if flags&MAP_FIXED!=0{return EBUSY;} let found=slice_find_area_stub(mm,len,&free,psize as i32,topdown,high); if found==ENOMEM{return ENOMEM} slice_range_to_mask(found,len,&mut free); slice_andnot_mask(&mut free,&free,&good); if free.low_slices!=0 { slice_convert_stub(mm,&free,psize as i32); } found }

// File-local helper mappings are represented below as extern-backed placeholders
// for symbols supplied by the kernel translation unit.
unsafe fn slice_mask_zero() -> slice_mask { core::mem::zeroed() }
unsafe fn slice_find_area_stub(_mm:*mut mm_struct,_len:ulong,_m:*const slice_mask,_p:i32,_t:i32,_h:ulong)->ulong { ENOMEM }
unsafe fn slice_convert_stub(_mm:*mut mm_struct,_m:*const slice_mask,_p:i32) {}

#[no_mangle] pub unsafe extern "C" fn get_slice_psize(mm:*mut mm_struct,addr:ulong)->u32 { let ps=if slice_addr_is_low(addr){mm_ctx_low_slices(&mut (*mm).context)}else{mm_ctx_high_slices(&mut (*mm).context)}; let i=if slice_addr_is_low(addr){get_low_slice_index(addr)}else{get_high_slice_index(addr)}; ((*ps.add(i>>1) >> ((i&1)*4))&0xf) as u32 }

// Constants, bitmap functions, index macros, and the remaining architecture
// entry points are provided by the surrounding kernel translation unit.
extern "C" { fn bitmap_zero(p:*mut ulong,n:ulong); fn bitmap_set(p:*mut ulong,o:ulong,n:ulong); fn bitmap_copy(d:*mut ulong,s:*const ulong,n:ulong); fn bitmap_or(d:*mut ulong,a:*const ulong,b:*const ulong,n:ulong); fn bitmap_andnot(d:*mut ulong,a:*const ulong,b:*const ulong,n:ulong); fn set_bit(i:ulong,p:*mut ulong); fn test_bit(i:ulong,p:*const ulong)->i32; }
extern "C" { fn get_low_slice_index(a:ulong)->ulong; fn get_high_slice_index(a:ulong)->ulong; fn align(a:ulong,b:ulong)->ulong; }
const SLICE_LOW_TOP:ulong=0; const SLICE_NUM_HIGH:ulong=0; const SLICE_NUM_LOW:ulong=0; const SLICE_LOW_SHIFT:u32=0; const SLICE_HIGH_SHIFT:u32=0; const PAGE_SHIFT:i32=0; const PAGE_SIZE:ulong=0; const PAGE_MASK:ulong=0; const DEFAULT_MAP_WINDOW:ulong=0; const TASK_SIZE:ulong=0; const MAP_FIXED:ulong=0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
