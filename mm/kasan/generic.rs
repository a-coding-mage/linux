// SPDX-License-Identifier: GPL-2.0
/* Core generic KASAN code, translated from generic.c. */

// Linux headers and symbols referenced below are supplied by other translation units.

unsafe extern "C" {
    fn kasan_enable();
    fn kasan_enabled() -> bool;
    fn kasan_mem_to_shadow(addr: *const core::ffi::c_void) -> *mut u8;
    fn kasan_report(addr: *const core::ffi::c_void, size: usize, write: bool, ret_ip: usize) -> bool;
    fn addr_has_metadata(addr: *const core::ffi::c_void) -> bool;
    fn kasan_unpoison(addr: *const core::ffi::c_void, size: usize, init: bool);
    fn kasan_poison(addr: *const core::ffi::c_void, size: usize, value: u8, init: bool);
    fn kasan_quarantine_remove_cache(cache: *mut kmem_cache);
    fn __kmem_cache_empty(cache: *mut kmem_cache) -> bool;
    fn kasan_requires_meta() -> bool;
    fn slub_debug_orig_size(cache: *mut kmem_cache) -> bool;
    fn __slub_debug_enabled() -> bool;
    fn __memset(dst: *mut core::ffi::c_void, value: i32, size: usize);
    fn kasan_addr_to_slab(addr: *mut core::ffi::c_void) -> *mut slab;
    fn nearest_obj(cache: *mut kmem_cache, slab: *mut slab, addr: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn is_kfence_address(addr: *mut core::ffi::c_void) -> bool;
    fn kasan_save_stack(gfp: u32, entries: u32) -> usize;
    fn kasan_save_track(track: *mut kasan_track, flags: u32);
}

const KASAN_GRANULE_SIZE: usize = 8;
const KASAN_GRANULE_MASK: usize = KASAN_GRANULE_SIZE - 1;
const KASAN_ALLOCA_REDZONE_SIZE: usize = 32;
const KASAN_GLOBAL_REDZONE: u8 = 0xfa;
const KASAN_ALLOCA_LEFT: u8 = 0xca;
const KASAN_ALLOCA_RIGHT: u8 = 0xcb;
const KASAN_SLAB_FREE_META: u8 = 0xfb;
const KASAN_SLAB_FREE: u8 = 0xfc;
const KASAN_NO_FREE_META: usize = usize::MAX;
const KMALLOC_MAX_SIZE: usize = 1 << 20;
const SLAB_KASAN: usize = 1 << 0;
const SLAB_NO_MERGE: usize = 1 << 1;
const SLAB_TYPESAFE_BY_RCU: usize = 1 << 2;

#[repr(C)] pub struct kasan_track { pub stack: usize }
#[repr(C)] pub struct kasan_alloc_meta { pub aux_stack: [usize; 2], pub alloc_track: kasan_track }
#[repr(C)] pub struct kasan_free_meta { pub free_track: kasan_track }
#[repr(C)] pub struct kasan_global { pub beg: *mut u8, pub size: usize, pub size_with_redzone: usize }
#[repr(C)] pub struct kasan_cache { pub alloc_meta_offset: usize, pub free_meta_offset: usize }
#[repr(C)] pub struct kmem_cache { pub kasan_info: kasan_cache, pub flags: usize, pub ctor: Option<unsafe extern "C" fn()> , pub object_size: usize }
#[repr(C)] pub struct slab { pub slab_cache: *mut kmem_cache }

#[inline] unsafe fn round_up(x: usize, a: usize) -> usize { (x + a - 1) / a * a }
#[inline] unsafe fn round_down(x: usize, a: usize) -> usize { x / a * a }

pub unsafe extern "C" fn kasan_init_generic() { kasan_enable(); }

#[inline(always)] unsafe fn memory_is_poisoned_1(addr: *const u8) -> bool {
    let shadow = *(kasan_mem_to_shadow(addr.cast()) as *const i8);
    shadow != 0 && ((addr as usize & KASAN_GRANULE_MASK) as i8 >= shadow)
}
#[inline(always)] unsafe fn memory_is_poisoned_2_4_8(addr: *const u8, size: usize) -> bool {
    let shadow = kasan_mem_to_shadow(addr.cast());
    if (((addr as usize + size - 1) & KASAN_GRANULE_MASK) < size - 1) { *shadow != 0 || memory_is_poisoned_1(addr.add(size - 1)) } else { memory_is_poisoned_1(addr.add(size - 1)) }
}
#[inline(always)] unsafe fn memory_is_poisoned_16(addr: *const u8) -> bool {
    let shadow = kasan_mem_to_shadow(addr.cast()) as *const u16;
    if addr as usize % KASAN_GRANULE_SIZE != 0 { *shadow != 0 || memory_is_poisoned_1(addr.add(15)) } else { *shadow != 0 }
}
#[inline(always)] unsafe fn bytes_is_nonzero(mut start: *const u8, mut size: usize) -> usize { while size != 0 { if *start != 0 { return start as usize; } start = start.add(1); size -= 1; } 0 }
#[inline(always)] unsafe fn memory_is_nonzero(mut start: *const u8, end: *const u8) -> usize {
    if end.offset_from(start) <= 16 { return bytes_is_nonzero(start, end.offset_from(start) as usize); }
    let mut prefix = start as usize % 8; if prefix != 0 { prefix = 8 - prefix; let r = bytes_is_nonzero(start, prefix); if r != 0 { return r; } start = start.add(prefix); }
    let mut words = end.offset_from(start) as usize / 8; while words != 0 { if *(start as *const u64) != 0 { return bytes_is_nonzero(start, 8); } start = start.add(8); words -= 1; }
    bytes_is_nonzero(start, end.offset_from(start) as usize % 8)
}
#[inline(always)] unsafe fn memory_is_poisoned_n(addr: *const u8, size: usize) -> bool {
    let ret = memory_is_nonzero(kasan_mem_to_shadow(addr.cast()), kasan_mem_to_shadow(addr.add(size-1).cast()).add(1));
    if ret != 0 { let last = addr.add(size-1); let shadow = kasan_mem_to_shadow(last.cast()) as *const i8; return ret != shadow as usize || (last as usize & KASAN_GRANULE_MASK) as i8 >= *shadow; } false
}
#[inline(always)] unsafe fn memory_is_poisoned(addr: *const u8, size: usize) -> bool { match size { 1 => memory_is_poisoned_1(addr), 2|4|8 => memory_is_poisoned_2_4_8(addr,size), 16 => memory_is_poisoned_16(addr), _ => memory_is_poisoned_n(addr,size) } }
#[inline(always)] unsafe fn check_region_inline(addr: *const u8, size: usize, write: bool, ret_ip: usize) -> bool { if !kasan_enabled() || size == 0 { return true; } if addr as usize + size < addr as usize || !addr_has_metadata(addr.cast()) || memory_is_poisoned(addr,size) { return !kasan_report(addr.cast(),size,write,ret_ip); } true }

pub unsafe extern "C" fn kasan_check_range(addr:*const u8,size:usize,write:bool,ret_ip:usize)->bool { check_region_inline(addr,size,write,ret_ip) }
pub unsafe extern "C" fn kasan_byte_accessible(addr:*const u8)->bool { if !kasan_enabled(){return true} let x=*(kasan_mem_to_shadow(addr.cast()) as *const i8); x>=0 && x<8 }
pub unsafe extern "C" fn kasan_cache_shrink(cache:*mut kmem_cache){kasan_quarantine_remove_cache(cache)}
pub unsafe extern "C" fn kasan_cache_shutdown(cache:*mut kmem_cache){if !__kmem_cache_empty(cache){kasan_quarantine_remove_cache(cache)}}
unsafe fn register_global(g:*mut kasan_global){let a=round_up((*g).size,8);kasan_unpoison((*g).beg.cast(),(*g).size,false);kasan_poison((*g).beg.add(a).cast(),(*g).size_with_redzone-a,KASAN_GLOBAL_REDZONE,false)}
pub unsafe extern "C" fn __asan_register_globals(ptr:*mut core::ffi::c_void,size:isize){for i in 0..size as usize{register_global((ptr as *mut kasan_global).add(i))}}
pub unsafe extern "C" fn __asan_unregister_globals(_ptr:*mut core::ffi::c_void,_size:isize){}

macro_rules! asan_load_store { ($n:ident,$s:expr) => { pub unsafe extern "C" fn $n(addr:*mut core::ffi::c_void){check_region_inline(addr.cast(),$s,false,0);} }; }
asan_load_store!(__asan_load1,1); asan_load_store!(__asan_load2,2); asan_load_store!(__asan_load4,4); asan_load_store!(__asan_load8,8); asan_load_store!(__asan_load16,16);
asan_load_store!(__asan_store1,1); asan_load_store!(__asan_store2,2); asan_load_store!(__asan_store4,4); asan_load_store!(__asan_store8,8); asan_load_store!(__asan_store16,16);
pub unsafe extern "C" fn __asan_loadN(addr:*mut core::ffi::c_void,size:isize){kasan_check_range(addr.cast(),size as usize,false,0)}
pub unsafe extern "C" fn __asan_storeN(addr:*mut core::ffi::c_void,size:isize){kasan_check_range(addr.cast(),size as usize,true,0)}
pub unsafe extern "C" fn __asan_handle_no_return(){}
pub unsafe extern "C" fn __asan_alloca_poison(addr:*mut u8,size:isize){let s=size as usize;let ru=round_up(s,8);let pad=round_up(s,32)-ru;let rd=round_down(s,8);kasan_unpoison(addr.add(rd).cast(),s-rd,false);kasan_poison(addr.sub(32).cast(),32,KASAN_ALLOCA_LEFT,false);kasan_poison(addr.add(ru).cast(),pad+32,KASAN_ALLOCA_RIGHT,false)}
pub unsafe extern "C" fn __asan_allocas_unpoison(top:*mut u8,bottom:isize){if top.is_null()||top as usize>bottom as usize{return}kasan_unpoison(top.cast(),bottom as usize-top as usize,false)}
macro_rules! asan_set_shadow { ($n:ident,$b:expr) => { pub unsafe extern "C" fn $n(addr:*const core::ffi::c_void,size:isize){__memset(addr as *mut _, $b, size as usize);} }; }
asan_set_shadow!(__asan_set_shadow_00,0x00); asan_set_shadow!(__asan_set_shadow_f1,0xf1); asan_set_shadow!(__asan_set_shadow_f2,0xf2); asan_set_shadow!(__asan_set_shadow_f3,0xf3); asan_set_shadow!(__asan_set_shadow_f5,0xf5); asan_set_shadow!(__asan_set_shadow_f8,0xf8);

unsafe fn optimal_redzone(s:u32)->u32{if s<=48{16}else if s<=96{32}else if s<=448{64}else if s<=3968{128}else if s<=16000{256}else if s<=32256{512}else if s<=64512{1024}else{2048}}
pub unsafe extern "C" fn kasan_get_alloc_meta(c:*mut kmem_cache,o:*const u8)->*mut kasan_alloc_meta{if (*c).kasan_info.alloc_meta_offset==0{core::ptr::null_mut()}else{o.add((*c).kasan_info.alloc_meta_offset) as *mut _}}
pub unsafe extern "C" fn kasan_get_free_meta(c:*mut kmem_cache,o:*const u8)->*mut kasan_free_meta{if (*c).kasan_info.free_meta_offset==KASAN_NO_FREE_META{core::ptr::null_mut()}else{o.add((*c).kasan_info.free_meta_offset) as *mut _}}
pub unsafe extern "C" fn kasan_cache_create(c:*mut kmem_cache,size:*mut usize,flags:*mut usize){if !kasan_requires_meta(){return}*flags|=SLAB_KASAN|SLAB_NO_MERGE;let ok=*size;(*c).kasan_info.alloc_meta_offset=*size;*size+=core::mem::size_of::<kasan_alloc_meta>();if *size>KMALLOC_MAX_SIZE{(*c).kasan_info.alloc_meta_offset=0;*size=ok}let ok=*size;let orig=(*c).kasan_info.alloc_meta_offset;if ((*c).flags&SLAB_TYPESAFE_BY_RCU)!=0||(*c).ctor.is_some(){(*c).kasan_info.free_meta_offset=*size;*size+=core::mem::size_of::<kasan_free_meta>();}else if core::mem::size_of::<kasan_free_meta>()>(*c).object_size{if !__slub_debug_enabled(){let rem=core::mem::size_of::<kasan_free_meta>()-(*c).object_size;*size+=rem;if (*c).kasan_info.alloc_meta_offset!=0{(*c).kasan_info.alloc_meta_offset+=rem}}else{(*c).kasan_info.free_meta_offset=*size;*size+=core::mem::size_of::<kasan_free_meta>();}}if *size>KMALLOC_MAX_SIZE{(*c).kasan_info.free_meta_offset=KASAN_NO_FREE_META;(*c).kasan_info.alloc_meta_offset=orig;*size=ok}let mut optimal=(*c).object_size+optimal_redzone((*c).object_size as u32) as usize;if optimal>KMALLOC_MAX_SIZE{optimal=KMALLOC_MAX_SIZE}if *size<optimal{*size=optimal}}
pub unsafe extern "C" fn kasan_init_object_meta(c:*mut kmem_cache,o:*const u8){let m=kasan_get_alloc_meta(c,o);if !m.is_null(){__memset(m.cast(),0,core::mem::size_of::<kasan_alloc_meta>())}}
unsafe fn release_alloc_meta(m:*mut kasan_alloc_meta){__memset(m.cast(),0,core::mem::size_of::<kasan_alloc_meta>())}
unsafe fn release_free_meta(o:*const u8,_m:*mut kasan_free_meta){let s=kasan_mem_to_shadow(o.cast());if *s==KASAN_SLAB_FREE_META{*s=KASAN_SLAB_FREE}}
pub unsafe extern "C" fn kasan_metadata_size(c:*mut kmem_cache,in_object:bool)->usize{if !kasan_requires_meta(){0}else if in_object{if (*c).kasan_info.free_meta_offset==0{core::mem::size_of::<kasan_free_meta>()}else{0}}else{(if (*c).kasan_info.alloc_meta_offset!=0{core::mem::size_of::<kasan_alloc_meta>()}else{0})+(if (*c).kasan_info.free_meta_offset!=0&&(*c).kasan_info.free_meta_offset!=KASAN_NO_FREE_META{core::mem::size_of::<kasan_free_meta>()}else{0})}}
pub unsafe extern "C" fn kasan_record_aux_stack(addr:*mut core::ffi::c_void){let s=kasan_addr_to_slab(addr);if is_kfence_address(addr)||s.is_null(){return}let c=(*s).slab_cache;let o=nearest_obj(c,s,addr);let m=kasan_get_alloc_meta(c,o.cast());if !m.is_null(){(*m).aux_stack[1]=(*m).aux_stack[0];(*m).aux_stack[0]=kasan_save_stack(0,0)}}
pub unsafe extern "C" fn kasan_save_alloc_info(c:*mut kmem_cache,o:*mut u8,flags:u32){let m=kasan_get_alloc_meta(c,o);if !m.is_null(){release_alloc_meta(m);kasan_save_track(&mut (*m).alloc_track,flags)}}
pub unsafe extern "C" fn kasan_save_free_info(c:*mut kmem_cache,o:*mut u8){let m=kasan_get_free_meta(c,o);if !m.is_null(){release_free_meta(o,m);kasan_save_track(&mut (*m).free_track,0);*kasan_mem_to_shadow(o.cast())=KASAN_SLAB_FREE_META}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
