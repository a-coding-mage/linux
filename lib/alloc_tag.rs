// SPDX-License-Identifier: GPL-2.0-only
// Translated from alloc_tag.c. Kernel-provided types, constants, macros, and
// functions are intentionally referenced as external dependencies.

const ALLOCINFO_FILE_NAME: &[u8] = b"allocinfo\0";
const MODULE_ALLOC_TAG_VMAP_SIZE: usize = 100000 * core::mem::size_of::<alloc_tag>();

#[cfg(CONFIG_MEM_ALLOC_PROFILING_ENABLED_BY_DEFAULT)]
static mut mem_profiling_support: bool = true;
#[cfg(not(CONFIG_MEM_ALLOC_PROFILING_ENABLED_BY_DEFAULT))]
static mut mem_profiling_support: bool = false;

#[repr(C)] pub struct alloc_tag { pub ct: codetag, pub counters: *mut alloc_tag_counters }
#[repr(C)] pub struct codetag { pub filename: *const i8, pub function: *const i8, pub modname: *const i8, pub lineno: u32 }
#[repr(C)] pub struct alloc_tag_counters { pub bytes: i64, pub calls: u64 }
#[repr(C)] pub struct codetag_bytes { pub ct: *mut codetag, pub bytes: i64 }
#[repr(C)] pub struct codetag_iterator { pub ct: *mut codetag }
#[repr(C)] pub struct alloc_tag_kernel_section { pub first_tag: *mut alloc_tag, pub count: usize }
#[repr(C)] pub struct alloc_tag_module_section { pub start_addr: usize, pub end_addr: usize, pub size: usize }
#[repr(C)] pub struct allocinfo_private { pub iter: codetag_iterator, pub reported_iter: codetag_iterator, pub print_header: bool }
#[repr(C)] pub struct seq_file { pub private: *mut core::ffi::c_void }
#[repr(C)] pub struct seq_buf;
#[repr(C)] pub struct folio { pub page: page }
#[repr(C)] pub struct page;
#[repr(C)] pub struct module { pub name: *const i8 }
#[repr(C)] pub struct ctl_table;
#[repr(C)] pub struct page_ext_operations { pub size: usize, pub need: Option<unsafe extern "C" fn() -> bool>, pub init: Option<unsafe extern "C" fn()> }
#[repr(C)] pub union codetag_ref { pub ct: *mut codetag, pub raw: usize }
#[repr(C)] pub union pgtag_ref_handle { pub raw: usize }
pub type loff_t = i64; pub type gfp_t = u32;

extern "C" {
    static mut mem_profiling_key: usize; static mut mem_profiling_compressed: usize;
    static mut alloc_tag_cttype: *mut core::ffi::c_void; static mut kernel_tags: alloc_tag_kernel_section;
    static mut module_tags: alloc_tag_module_section; static mut alloc_tag_ref_mask: usize; static mut alloc_tag_ref_offs: i32;
    fn mem_alloc_profiling_enabled() -> bool; fn static_key_enabled(k: *const usize) -> bool;
    fn static_branch_enable(k: *mut usize); fn static_branch_disable(k: *mut usize);
    fn codetag_lock_module_list(t: *mut core::ffi::c_void); fn codetag_unlock_module_list(t: *mut core::ffi::c_void);
    fn codetag_trylock_module_list(t: *mut core::ffi::c_void) -> bool; fn codetag_get_ct_iter(t: *mut core::ffi::c_void) -> codetag_iterator;
    fn codetag_next_ct(i: *mut codetag_iterator) -> *mut codetag; fn ct_to_alloc_tag(c: *mut codetag) -> *mut alloc_tag;
    fn alloc_tag_read(t: *mut alloc_tag) -> alloc_tag_counters; fn alloc_tag_is_inaccurate(t: *mut alloc_tag) -> bool;
    fn seq_buf_printf(b: *mut seq_buf, f: *const i8, ...); fn codetag_to_text(b: *mut seq_buf, c: *mut codetag);
    fn seq_buf_putc(b: *mut seq_buf, c: i32); fn seq_get_buf(m: *mut seq_file, p: *mut *mut i8) -> usize;
    fn seq_buf_init(b: *mut seq_buf, p: *mut i8, n: usize); fn seq_buf_used(b: *const seq_buf) -> usize; fn seq_commit(m: *mut seq_file, n: usize);
    fn get_page_tag_ref(p: *mut page, r: *mut codetag_ref, h: *mut pgtag_ref_handle) -> bool; fn put_page_tag_ref(h: pgtag_ref_handle);
    fn update_page_tag_ref(h: pgtag_ref_handle, r: *mut codetag_ref); fn set_codetag_empty(r: *mut codetag_ref); fn alloc_tag_ref_set(r: *mut codetag_ref, t: *mut alloc_tag); fn __alloc_tag_ref_set(r: *mut codetag_ref, t: *mut alloc_tag);
    fn __pgalloc_tag_get(p: *mut page) -> *mut alloc_tag; fn folio_page(f: *mut folio, i: usize) -> *mut page;
    fn free_percpu(p: *mut core::ffi::c_void); fn alloc_percpu() -> *mut alloc_tag_counters;
    fn pr_err(f: *const i8, ...); fn pr_warn(f: *const i8, ...); fn pr_info(f: *const i8, ...); fn pr_debug(f: *const i8, ...);
    fn remove_proc_entry(n: *const i8, p: *mut core::ffi::c_void); fn proc_create_seq_private(...);
    fn register_sysctl_init(n: *const i8, t: *const ctl_table); fn proc_do_static_key(...)->i32;
}

#[no_mangle] pub unsafe extern "C" fn mem_alloc_profiling_permanently_disabled() -> bool { !mem_profiling_support }

#[no_mangle] pub unsafe extern "C" fn pgalloc_tag_split(folio: *mut folio, old_order: i32, new_order: i32) {
    if !mem_alloc_profiling_enabled() { return; }
    let tag = __pgalloc_tag_get(&mut (*folio).page); if tag.is_null() { return; }
    let nr_pages = 1usize.wrapping_shl(new_order as u32); let mut i = nr_pages;
    while i < (1usize.wrapping_shl(old_order as u32)) { let mut r = codetag_ref{raw:0}; let mut h=pgtag_ref_handle{raw:0}; if get_page_tag_ref(folio_page(folio,i),&mut r,&mut h) { alloc_tag_ref_set(&mut r,tag); update_page_tag_ref(h,&mut r); put_page_tag_ref(h); } i += nr_pages; }
}

#[no_mangle] pub unsafe extern "C" fn pgalloc_tag_swap(new: *mut folio, old: *mut folio) {
    if !mem_alloc_profiling_enabled() { return; }
    let to=__pgalloc_tag_get(&mut (*old).page); if to.is_null(){return} let tn=__pgalloc_tag_get(&mut (*new).page); if tn.is_null(){return}
    let (mut ro,mut ho)=(codetag_ref{raw:0},pgtag_ref_handle{raw:0}); let (mut rn,mut hn)=(codetag_ref{raw:0},pgtag_ref_handle{raw:0});
    if !get_page_tag_ref(&mut (*old).page,&mut ro,&mut ho){return} if !get_page_tag_ref(&mut (*new).page,&mut rn,&mut hn){put_page_tag_ref(ho);return}
    set_codetag_empty(&mut ro); set_codetag_empty(&mut rn); __alloc_tag_ref_set(&mut ro,tn); update_page_tag_ref(ho,&mut ro); __alloc_tag_ref_set(&mut rn,to); update_page_tag_ref(hn,&mut rn); put_page_tag_ref(ho); put_page_tag_ref(hn);
}

#[no_mangle] pub unsafe extern "C" fn alloc_tag_top_users(tags:*mut codetag_bytes,count:usize,can_sleep:bool)->usize {
    if alloc_tag_cttype.is_null(){return 0} if can_sleep{codetag_lock_module_list(alloc_tag_cttype)} else if !codetag_trylock_module_list(alloc_tag_cttype){return 0}
    let mut it=codetag_get_ct_iter(alloc_tag_cttype); let mut nr=0; loop { let ct=codetag_next_ct(&mut it); if ct.is_null(){break} let c=alloc_tag_read(ct_to_alloc_tag(ct)); let mut i=0; while i<nr && (*tags.add(i)).bytes>=c.bytes{i+=1} if i<count { if nr==count{nr-=1} core::ptr::copy(tags.add(i),tags.add(i+1),nr-i); (*tags.add(i))=codetag_bytes{ct,bytes:c.bytes}; nr+=1; } } codetag_unlock_module_list(alloc_tag_cttype); nr
}

#[cfg(CONFIG_MEM_ALLOC_PROFILING_DEBUG)]
#[repr(C)] pub struct pfn_pool { pub next:*mut pfn_pool, pub count:i32, pub pfns:[usize;0] }
#[cfg(CONFIG_MEM_ALLOC_PROFILING_DEBUG)]
static mut current_pfn_pool:*mut pfn_pool=core::ptr::null_mut();
#[cfg(not(CONFIG_MEM_ALLOC_PROFILING_DEBUG))]
#[inline] unsafe fn clear_early_alloc_pfn_tag_refs(){}

#[no_mangle] pub unsafe extern "C" fn page_alloc_tagging_need()->bool { !static_key_enabled(&mem_profiling_compressed) && mem_profiling_support }
#[no_mangle] pub static mut page_alloc_tagging_ops: page_ext_operations = page_ext_operations{size:core::mem::size_of::<codetag_ref>(),need:Some(page_alloc_tagging_need),init:None};

#[no_mangle] pub unsafe extern "C" fn shutdown_mem_profiling(remove_file: bool) {
    if mem_alloc_profiling_enabled() { static_branch_disable(&mut mem_profiling_key); }
    if !mem_profiling_support { return; }
    if remove_file { remove_proc_entry(ALLOCINFO_FILE_NAME.as_ptr() as *const i8, core::ptr::null_mut()); }
    mem_profiling_support = false;
}

#[no_mangle] pub unsafe extern "C" fn alloc_tag_sec_init() {
    if !mem_profiling_support || !static_key_enabled(&mem_profiling_compressed) { return; }
}

#[no_mangle] pub unsafe extern "C" fn setup_early_mem_profiling(_str: *mut i8) -> i32 { -22 }
#[no_mangle] pub unsafe extern "C" fn init_page_alloc_tagging() { clear_early_alloc_pfn_tag_refs(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
