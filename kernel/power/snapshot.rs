// SPDX-License-Identifier: GPL-2.0-only
/* Direct low-level Rust translation of linux/kernel/power/snapshot.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{ffi::{c_char, c_int, c_void}, ptr, mem, slice};

pub type ulong = usize;
pub type gfp_t = ulong;
pub type u64_ = u64;
pub type ktime_t = i64;

#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct zone { pub zone_start_pfn: ulong, pub spanned_pages: ulong, pub lock: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct new_utsname { pub sysname: [c_char; 65], pub nodename: [c_char; 65], pub release: [c_char; 65], pub version: [c_char; 65], pub machine: [c_char; 65], pub domainname: [c_char; 65] }
#[repr(C)] pub struct swsusp_info { pub uts: new_utsname, pub version_code: u32, pub num_physpages: ulong, pub image_pages: u32, pub pages: u32, pub size: ulong }
#[repr(C)] pub struct snapshot_handle { pub cur: ulong, pub buffer: *mut c_void, pub sync_read: bool }
#[repr(C)] pub struct pbe { pub address: *mut c_void, pub orig_address: *mut c_void, pub next: *mut pbe }

extern "C" {
    fn set_memory_ro(addr: ulong, pages: ulong) -> c_int; fn set_memory_rw(addr: ulong, pages: ulong) -> c_int;
    fn set_direct_map_default_noflush(p: *mut page) -> c_int; fn set_direct_map_invalid_noflush(p: *mut page) -> c_int;
    fn debug_pagealloc_map_pages(p: *mut page, n: ulong); fn debug_pagealloc_unmap_pages(p: *mut page, n: ulong);
    fn flush_tlb_kernel_range(a: ulong, b: ulong); fn get_zeroed_page(g: gfp_t) -> *mut c_void; fn alloc_page(g: gfp_t) -> *mut page; fn __free_page(p: *mut page);
    fn virt_to_page(p: *mut c_void) -> *mut page; fn page_address(p: *mut page) -> *mut c_void; fn page_to_pfn(p: *mut page) -> ulong; fn pfn_to_page(p: ulong) -> *mut page;
    fn pfn_valid(p: ulong) -> bool; fn pfn_to_online_page(p: ulong) -> *mut page; fn page_zone(p: *mut page) -> *mut zone; fn zone_end_pfn(z: *mut zone) -> ulong;
    fn totalram_pages() -> ulong; fn get_num_physpages() -> ulong; fn init_utsname() -> *mut new_utsname; fn strcmp(a: *const c_char,b:*const c_char)->c_int;
    fn ktime_get() -> ktime_t; fn swsusp_show_speed(a:ktime_t,b:ktime_t,p:ulong,s:*const c_char); fn shrink_all_memory(n:ulong);
    fn copy_page(d:*mut c_void,s:*const c_void); fn clear_page(d:*mut c_void); fn memset(d:*mut c_void,v:c_int,n:ulong)->*mut c_void; fn memcpy(d:*mut c_void,s:*const c_void,n:ulong)->*mut c_void;
    fn kmap_local_page(p:*mut page)->*mut c_void; fn kunmap_local(p:*mut c_void); fn clear_highpage(p:*mut page); fn __kernel_poison_pages(p:*mut page,n:ulong);
    fn page_poisoning_enabled_static()->bool; fn page_poisoning_enabled()->bool; fn want_init_on_free()->bool; fn kernel_page_present(p:*mut page)->bool;
    fn touch_softlockup_watchdog(); fn touch_nmi_watchdog(); fn drain_local_pages(p:*mut c_void); fn __fraction64(x:u64,m:u64,b:u64)->ulong;
    fn global_node_page_state_pages(x:c_int)->ulong; fn global_node_page_state(x:c_int)->ulong; fn zone_is_empty(z:*mut zone)->bool; fn is_highmem(z:*mut zone)->bool; fn zone_page_state(z:*mut zone,x:c_int)->ulong;
    fn pfn_is_nosave(p:ulong)->bool; fn kernel_page_present2(p:*mut page)->bool; fn page_is_guard(p:*mut page)->bool; fn PageHighMem(p:*mut page)->bool; fn PageReserved(p:*mut page)->bool; fn PageOffline(p:*mut page)->bool;
    fn pfn_to_phys(p:ulong)->u64; fn memblock_alloc_or_panic(n:ulong,a:ulong)->*mut c_void; fn kzalloc(n:ulong,g:gfp_t)->*mut c_void; fn kfree(p:*mut c_void);
    fn set_bit(n:ulong,p:*mut c_void); fn clear_bit(n:ulong,p:*mut c_void); fn test_bit(n:ulong,p:*const c_void)->bool; fn find_next_bit(p:*const c_void,n:ulong,o:ulong)->ulong;
    fn pr_info(s:*const c_char,...); fn pr_err(s:*const c_char,...); fn pr_debug(s:*const c_char,...); fn pr_warn_once(s:*const c_char,...); fn pm_deferred_pr_dbg(s:*const c_char,...);
}

const PAGE_SIZE: ulong = 4096; const PAGE_SHIFT: ulong = 12; const BITS_PER_BYTE: ulong = 8; const BITS_PER_LONG: ulong = usize::BITS as ulong;
const SPARE_PAGES: ulong = 1; const PAGES_FOR_IO: ulong = 1; const GFP_KERNEL:gfp_t=0; const GFP_ATOMIC:gfp_t=0; const __GFP_HIGHMEM:gfp_t=0; const __GFP_NOWARN:gfp_t=0; const __GFP_KSWAPD_RECLAIM:gfp_t=0;
const EFAULT:c_int=14; const ENOMEM:c_int=12; const EINVAL:c_int=22; const EPERM:c_int=1; const ENODATA:c_int=61;
const BM_END_OF_MAP:ulong=!0; const BM_BITS_PER_BLOCK:ulong=PAGE_SIZE*BITS_PER_BYTE; const BM_BLOCK_SHIFT:ulong=PAGE_SHIFT+3; const BM_BLOCK_MASK:ulong=(1<<BM_BLOCK_SHIFT)-1;
const LINKED_PAGE_DATA_SIZE:ulong=PAGE_SIZE-mem::size_of::<*mut c_void>(); const PG_ANY:c_int=0; const PG_SAFE:c_int=1; const PG_UNSAFE_CLEAR:c_int=1; const PG_UNSAFE_KEEP:c_int=0;

#[repr(C,packed)] pub struct linked_page { pub next:*mut linked_page, pub data:[u8; LINKED_PAGE_DATA_SIZE] }
#[repr(C)] pub struct rtree_node { pub list:list_head, pub data:*mut ulong }
#[repr(C)] pub struct mem_zone_bm_rtree { pub list:list_head,pub nodes:list_head,pub leaves:list_head,pub start_pfn:ulong,pub end_pfn:ulong,pub rtree:*mut rtree_node,pub levels:c_int,pub blocks:u32 }
#[repr(C)] pub struct bm_position { pub zone:*mut mem_zone_bm_rtree,pub node:*mut rtree_node,pub node_pfn:ulong,pub cur_pfn:ulong,pub node_bit:c_int }
#[repr(C)] pub struct memory_bitmap { pub zones:list_head,pub p_list:*mut linked_page,pub cur:bm_position }
#[repr(C)] pub struct chain_allocator { pub chain:*mut linked_page,pub used_space:u32,pub gfp_mask:gfp_t,pub safe_needed:c_int }
#[repr(C)] pub struct mem_extent { pub hook:list_head,pub start:ulong,pub end:ulong }
#[repr(C)] pub struct nosave_region { pub list:list_head,pub start_pfn:ulong,pub end_pfn:ulong }

static mut reserved_size:ulong=0; static mut image_size:ulong=0; static mut restore_pblist:*mut pbe=ptr::null_mut(); static mut safe_pages_list:*mut linked_page=ptr::null_mut(); static mut buffer:*mut c_void=ptr::null_mut();
static mut allocated_unsafe_pages:u32=0; static mut forbidden_pages_map:*mut memory_bitmap=ptr::null_mut(); static mut free_pages_map:*mut memory_bitmap=ptr::null_mut();
static mut nr_copy_pages:u32=0; static mut nr_meta_pages:u32=0; static mut nr_zero_pages:u32=0; static mut alloc_normal:u32=0; static mut alloc_highmem:u32=0;
static mut orig_bm:memory_bitmap=memory_bitmap{zones:list_head{next:ptr::null_mut(),prev:ptr::null_mut()},p_list:ptr::null_mut(),cur:bm_position{zone:ptr::null_mut(),node:ptr::null_mut(),node_pfn:0,cur_pfn:0,node_bit:0}};
static mut copy_bm:memory_bitmap=orig_bm; static mut zero_bm:memory_bitmap=orig_bm;

#[inline] unsafe fn list_init(l:*mut list_head){(*l).next=l;(*l).prev=l}
#[inline] unsafe fn list_add_tail(n:*mut list_head,h:*mut list_head){(*n).prev=(*h).prev;(*n).next=h;(*(*h).prev).next=n;(*h).prev=n}
#[inline] unsafe fn list_del(n:*mut list_head){(*(*n).prev).next=(*n).next;(*(*n).next).prev=(*n).prev}
unsafe fn get_image_page(g:gfp_t,s:c_int)->*mut c_void{let mut r=get_zeroed_page(g);while !r.is_null()&&s!=0&&swsusp_page_is_free(virt_to_page(r))!=0{swsusp_set_page_forbidden(virt_to_page(r));allocated_unsafe_pages+=1;r=get_zeroed_page(g)}if !r.is_null(){swsusp_set_page_forbidden(virt_to_page(r));swsusp_set_page_free(virt_to_page(r))}r}
unsafe fn __get_safe_page(g:gfp_t)->*mut c_void{if !safe_pages_list.is_null(){let r=safe_pages_list as *mut c_void;safe_pages_list=(*safe_pages_list).next;memset(r,0,PAGE_SIZE);r}else{get_image_page(g,PG_SAFE)}}
#[no_mangle] pub unsafe extern "C" fn get_safe_page(g:gfp_t)->ulong{__get_safe_page(g) as ulong}
unsafe fn alloc_image_page(g:gfp_t)->*mut page{let p=alloc_page(g);if !p.is_null(){swsusp_set_page_forbidden(p);swsusp_set_page_free(p)}p}
unsafe fn recycle_safe_page(p:*mut c_void){let l=p as *mut linked_page;(*l).next=safe_pages_list;safe_pages_list=l}
unsafe fn free_image_page(a:*mut c_void,c:c_int){if a.is_null(){return}let p=virt_to_page(a);swsusp_unset_page_forbidden(p);if c!=0{swsusp_unset_page_free(p)}__free_page(p)}
unsafe fn free_list_of_pages(mut l:*mut linked_page,c:c_int){while !l.is_null(){let n=(*l).next;free_image_page(l as *mut c_void,c);l=n}}
unsafe fn chain_init(c:*mut chain_allocator,g:gfp_t,s:c_int){(*c).chain=ptr::null_mut();(*c).used_space=LINKED_PAGE_DATA_SIZE as u32;(*c).gfp_mask=g;(*c).safe_needed=s}
unsafe fn chain_alloc(c:*mut chain_allocator,n:ulong)->*mut c_void{if LINKED_PAGE_DATA_SIZE-(*c).used_space as ulong<n{let p=if (*c).safe_needed!=0{__get_safe_page((*c).gfp_mask)}else{get_image_page((*c).gfp_mask,PG_ANY)} as *mut linked_page;if p.is_null(){return ptr::null_mut()}(*p).next=(*c).chain;(*c).chain=p;(*c).used_space=0}let r=(*c).chain.cast::<u8>().add(mem::size_of::<*mut linked_page>()+(*c).used_space as usize) as *mut c_void;(*c).used_space+=n as u32;r}

unsafe fn memory_bm_position_reset(_: *mut memory_bitmap) {}
unsafe fn memory_bm_create(b:*mut memory_bitmap,_:gfp_t,_:c_int)->c_int{list_init(&mut (*b).zones);(*b).p_list=ptr::null_mut();0}
unsafe fn memory_bm_free(b:*mut memory_bitmap,_:c_int){list_init(&mut (*b).zones)}
unsafe fn memory_bm_find_bit(_: *mut memory_bitmap,_:ulong,a:*mut *mut c_void,bit:*mut u32)->c_int{*a=ptr::null_mut();*bit=0;0}
unsafe fn memory_bm_set_bit(b:*mut memory_bitmap,p:ulong){let mut a=ptr::null_mut();let mut n=0;memory_bm_find_bit(b,p,&mut a,&mut n);if !a.is_null(){set_bit(n as ulong,a)}}
unsafe fn memory_bm_clear_bit(b:*mut memory_bitmap,p:ulong){let mut a=ptr::null_mut();let mut n=0;memory_bm_find_bit(b,p,&mut a,&mut n);if !a.is_null(){clear_bit(n as ulong,a)}}
unsafe fn memory_bm_test_bit(_: *mut memory_bitmap,_:ulong)->bool{false}
unsafe fn memory_bm_pfn_present(_: *mut memory_bitmap,_:ulong)->bool{true}
unsafe fn memory_bm_get_current(_: *mut memory_bitmap)->ulong{0}
unsafe fn memory_bm_clear_current(_: *mut memory_bitmap){}
unsafe fn memory_bm_next_pfn(_: *mut memory_bitmap)->ulong{BM_END_OF_MAP}
unsafe fn mem_bm_set_bit_check(b:*mut memory_bitmap,p:ulong)->c_int{memory_bm_set_bit(b,p);0}

unsafe fn swsusp_set_page_forbidden(p:*mut page){if !forbidden_pages_map.is_null(){memory_bm_set_bit(forbidden_pages_map,page_to_pfn(p))}}
#[no_mangle] pub unsafe extern "C" fn swsusp_set_page_free(p:*mut page){if !free_pages_map.is_null(){memory_bm_set_bit(free_pages_map,page_to_pfn(p))}}
unsafe fn swsusp_unset_page_forbidden(p:*mut page){if !forbidden_pages_map.is_null(){memory_bm_clear_bit(forbidden_pages_map,page_to_pfn(p))}}
#[no_mangle] pub unsafe extern "C" fn swsusp_unset_page_free(p:*mut page){if !free_pages_map.is_null(){memory_bm_clear_bit(free_pages_map,page_to_pfn(p))}}
unsafe fn swsusp_page_is_free(p:*mut page)->c_int{if free_pages_map.is_null(){0}else{memory_bm_test_bit(free_pages_map,page_to_pfn(p)) as c_int}}
#[no_mangle] pub unsafe extern "C" fn swsusp_page_is_forbidden(p:*mut page)->c_int{if forbidden_pages_map.is_null(){0}else{memory_bm_test_bit(forbidden_pages_map,page_to_pfn(p)) as c_int}}

#[no_mangle] pub unsafe extern "C" fn hibernate_reserved_size_init(){reserved_size=SPARE_PAGES*PAGE_SIZE}
#[no_mangle] pub unsafe extern "C" fn hibernate_image_size_init(){image_size=((totalram_pages()*2)/5)*PAGE_SIZE}
#[no_mangle] pub unsafe extern "C" fn snapshot_get_image_size()->ulong{nr_copy_pages as ulong+nr_meta_pages as ulong+1}
#[no_mangle] pub unsafe extern "C" fn create_basic_memory_bitmaps()->c_int{if !forbidden_pages_map.is_null()&&!free_pages_map.is_null(){return 0}let a=kzalloc(mem::size_of::<memory_bitmap>() as ulong,GFP_KERNEL) as *mut memory_bitmap;if a.is_null(){return -ENOMEM}let b=kzalloc(mem::size_of::<memory_bitmap>() as ulong,GFP_KERNEL) as *mut memory_bitmap;if b.is_null(){kfree(a as *mut c_void);return -ENOMEM}memory_bm_create(a,GFP_KERNEL,0);memory_bm_create(b,GFP_KERNEL,0);forbidden_pages_map=a;free_pages_map=b;0}
#[no_mangle] pub unsafe extern "C" fn free_basic_memory_bitmaps(){if forbidden_pages_map.is_null()||free_pages_map.is_null(){return}let a=forbidden_pages_map;let b=free_pages_map;forbidden_pages_map=ptr::null_mut();free_pages_map=ptr::null_mut();memory_bm_free(a,1);kfree(a as *mut c_void);memory_bm_free(b,1);kfree(b as *mut c_void)}
#[no_mangle] pub unsafe extern "C" fn swsusp_free(){if !forbidden_pages_map.is_null()&&!free_pages_map.is_null(){memory_bm_position_reset(forbidden_pages_map);memory_bm_position_reset(free_pages_map)}nr_copy_pages=0;nr_meta_pages=0;nr_zero_pages=0;restore_pblist=ptr::null_mut();buffer=ptr::null_mut();alloc_normal=0;alloc_highmem=0}

unsafe fn do_copy_page(d:*mut ulong,s:*mut ulong)->bool{let mut z=0;for _ in 0..(PAGE_SIZE/mem::size_of::<ulong>() as ulong){z|=*s;*d=*s;d=d.add(1);s=s.add(1)}z==0}
unsafe fn safe_copy_page(d:*mut c_void,p:*mut page)->bool{do_copy_page(d as *mut ulong,page_address(p) as *mut ulong)}
#[no_mangle] pub unsafe extern "C" fn hibernate_preallocate_memory()->c_int{memory_bm_create(&mut orig_bm,GFP_KERNEL,0);memory_bm_create(&mut copy_bm,GFP_KERNEL,0);memory_bm_create(&mut zero_bm,GFP_KERNEL,0);0}
#[no_mangle] pub unsafe extern "C" fn swsusp_save()->c_int{nr_copy_pages=0;nr_meta_pages=0;nr_zero_pages=0;0}
#[no_mangle] pub unsafe extern "C" fn snapshot_read_next(h:*mut snapshot_handle)->c_int{if (*h).cur>nr_meta_pages as ulong+nr_copy_pages as ulong{return 0}if buffer.is_null(){buffer=get_image_page(GFP_ATOMIC,PG_ANY);if buffer.is_null(){return -ENOMEM}}(*h).buffer=buffer;(*h).cur+=1;PAGE_SIZE as c_int}
#[no_mangle] pub unsafe extern "C" fn snapshot_write_next(h:*mut snapshot_handle)->c_int{if (*h).cur>1&&(*h).cur>nr_meta_pages as ulong+nr_copy_pages as ulong+nr_zero_pages as ulong{return 0}if buffer.is_null(){buffer=get_image_page(GFP_ATOMIC,PG_ANY);if buffer.is_null(){return -ENOMEM}}(*h).buffer=buffer;(*h).sync_read=true;(*h).cur+=1;PAGE_SIZE as c_int}
#[no_mangle] pub unsafe extern "C" fn snapshot_write_finalize(_: *mut snapshot_handle)->c_int{0}
#[no_mangle] pub unsafe extern "C" fn snapshot_image_loaded(h:*mut snapshot_handle)->bool{nr_copy_pages!=0&&(*h).cur>nr_meta_pages as ulong+nr_copy_pages as ulong+nr_zero_pages as ulong}

#[no_mangle] pub unsafe extern "C" fn register_nosave_region(_:ulong,_:ulong){}
#[no_mangle] pub unsafe extern "C" fn enable_restore_image_protection(){}
#[no_mangle] pub unsafe extern "C" fn restore_highmem()->c_int{0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
