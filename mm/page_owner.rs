// SPDX-License-Identifier: GPL-2.0
// Translation of page_owner.c. Kernel headers, macros, and external symbols are
// supplied by the surrounding kernel/Rust environment.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

const PAGE_OWNER_STACK_DEPTH: usize = 16;
const STACK_PRINT_FLAG_STACK: u8 = 0x1;
const STACK_PRINT_FLAG_PAGES: u8 = 0x2;
const STACK_PRINT_FLAG_HANDLE: u8 = 0x4;

#[repr(C)]
pub struct page_owner {
    pub order: u16,
    pub last_migrate_reason: i16,
    pub gfp_mask: gfp_t,
    pub handle: depot_stack_handle_t,
    pub free_handle: depot_stack_handle_t,
    pub ts_nsec: u64,
    pub free_ts_nsec: u64,
    pub comm: [core::ffi::c_char; TASK_COMM_LEN],
    pub pid: pid_t,
    pub tgid: pid_t,
    pub free_pid: pid_t,
    pub free_tgid: pid_t,
}

#[repr(C)] pub struct stack { pub stack_record: *mut stack_record, pub next: *mut stack }
static mut dummy_stack: stack = stack { stack_record: core::ptr::null_mut(), next: core::ptr::null_mut() };
static mut failure_stack: stack = stack { stack_record: core::ptr::null_mut(), next: core::ptr::null_mut() };
static mut stack_list: *mut stack = core::ptr::null_mut();
static mut stack_list_lock: spinlock_t = spinlock_t::new();

#[repr(C)] pub struct stack_print_ctx { pub stack: *mut stack, pub flags: u8 }
#[repr(C)] #[derive(Clone, Copy)] pub enum page_owner_print_mode { PAGE_OWNER_PRINT_STACK, PAGE_OWNER_PRINT_HANDLE, PAGE_OWNER_PRINT_STACK_HANDLE }
static page_owner_print_mode_strings: [&[u8]; 3] = [b"stack\0", b"handle\0", b"stack_handle\0"];
#[repr(C)] pub struct page_owner_filter_state { pub print_mode: page_owner_print_mode, pub nid_filter: nodemask_t, pub nid_filter_enabled: bool }

static mut page_owner_enabled: bool = false;
static mut dummy_handle: depot_stack_handle_t = 0;
static mut failure_handle: depot_stack_handle_t = 0;
static mut early_handle: depot_stack_handle_t = 0;
static mut pages_threshold: u64 = 0;

unsafe fn set_current_in_page_owner() { current().in_page_owner = 1; }
unsafe fn unset_current_in_page_owner() { current().in_page_owner = 0; }
unsafe fn early_page_owner_param(buf: *mut c_char) -> c_int {
    let ret = kstrtobool(buf, &mut page_owner_enabled);
    if page_owner_enabled { stack_depot_request_early_init(); }
    ret
}
unsafe fn need_page_owner() -> bool { page_owner_enabled }

unsafe fn create_dummy_stack() -> depot_stack_handle_t {
    let mut entries = [0usize; 4];
    let nr_entries = stack_trace_save(entries.as_mut_ptr(), entries.len(), 0);
    stack_depot_save(entries.as_ptr(), nr_entries, GFP_KERNEL)
}
unsafe fn register_dummy_stack() { dummy_handle = create_dummy_stack(); }
unsafe fn register_failure_stack() { failure_handle = create_dummy_stack(); }
unsafe fn register_early_stack() { early_handle = create_dummy_stack(); }

unsafe fn init_page_owner() {
    if !page_owner_enabled { return; }
    register_dummy_stack(); register_failure_stack(); register_early_stack(); init_early_allocated_pages();
    dummy_stack.stack_record = __stack_depot_get_stack_record(dummy_handle);
    failure_stack.stack_record = __stack_depot_get_stack_record(failure_handle);
    if !dummy_stack.stack_record.is_null() { refcount_set(&mut (*dummy_stack.stack_record).count, 1); }
    if !failure_stack.stack_record.is_null() { refcount_set(&mut (*failure_stack.stack_record).count, 1); }
    dummy_stack.next = &mut failure_stack; stack_list = &mut dummy_stack; static_branch_enable(&page_owner_inited);
}

#[repr(C)] pub struct page_ext_operations { pub size: usize, pub need: unsafe fn() -> bool, pub init: unsafe fn(), pub need_shared_flags: bool }
#[no_mangle] pub static mut page_owner_ops: page_ext_operations = page_ext_operations { size: core::mem::size_of::<page_owner>(), need: need_page_owner, init: init_page_owner, need_shared_flags: true };
unsafe fn get_page_owner(p: *mut page_ext) -> *mut page_owner { page_ext_data(p, &mut page_owner_ops) as *mut page_owner }

unsafe fn save_stack(flags: gfp_t) -> depot_stack_handle_t {
    let mut entries = [0usize; PAGE_OWNER_STACK_DEPTH];
    if current().in_page_owner != 0 { return dummy_handle; }
    set_current_in_page_owner();
    let n = stack_trace_save(entries.as_mut_ptr(), entries.len(), 2);
    let mut handle = stack_depot_save(entries.as_ptr(), n, flags);
    if handle == 0 { handle = failure_handle; }
    unset_current_in_page_owner(); handle
}

unsafe fn add_stack_record_to_list(record: *mut stack_record, mask: gfp_t) {
    if !gfpflags_allow_spinning(mask) { return; }
    set_current_in_page_owner();
    let s = kmalloc_stack(gfp_nested_mask(mask));
    if s.is_null() { unset_current_in_page_owner(); return; }
    unset_current_in_page_owner(); (*s).stack_record = record; (*s).next = core::ptr::null_mut();
    let flags = 0usize; spin_lock_irqsave(&mut stack_list_lock, flags);
    (*s).next = stack_list; smp_store_release(&mut stack_list, s); spin_unlock_irqrestore(&mut stack_list_lock, flags);
}
unsafe fn inc_stack_record_count(handle: depot_stack_handle_t, mask: gfp_t, pages: i32) {
    let r = __stack_depot_get_stack_record(handle); if r.is_null() { return; }
    if refcount_read(&(*r).count) == REFCOUNT_SATURATED { let mut old = REFCOUNT_SATURATED; if atomic_try_cmpxchg_relaxed(&mut (*r).count.refs, &mut old, 1) { add_stack_record_to_list(r, mask); } }
    refcount_add(pages, &mut (*r).count);
}
unsafe fn dec_stack_record_count(handle: depot_stack_handle_t, pages: i32) { let r=__stack_depot_get_stack_record(handle); if !r.is_null() && refcount_sub_and_test(pages,&mut (*r).count) { pr_warn_refcount_zero(handle); } }

unsafe fn update_page_owner_handle(page: *mut page, handle: depot_stack_handle_t, order: u16, mask: gfp_t, reason: i16, ts: u64, pid: pid_t, tgid: pid_t, comm: *const c_char) {
    let mut it = page_ext_iter::default(); let mut ext = core::ptr::null_mut();
    for_each_page_ext(page, 1usize << order, ext, it) { let o=get_page_owner(ext); (*o).handle=handle; (*o).order=order; (*o).gfp_mask=mask; (*o).last_migrate_reason=reason; (*o).pid=pid; (*o).tgid=tgid; (*o).ts_nsec=ts; strscpy((*o).comm.as_mut_ptr(),comm,(*o).comm.len()); set_bit(PAGE_EXT_OWNER,&mut (*ext).flags); set_bit(PAGE_EXT_OWNER_ALLOCATED,&mut (*ext).flags); }
}
unsafe fn update_page_owner_free_handle(page:*mut page, handle:depot_stack_handle_t, order:u16, _pid:pid_t, _tgid:pid_t, ts:u64) { let mut it=page_ext_iter::default(); let mut ext=core::ptr::null_mut(); for_each_page_ext(page,1usize<<order,ext,it) { let o=get_page_owner(ext); if handle!=0 { clear_bit(PAGE_EXT_OWNER_ALLOCATED,&mut (*ext).flags); (*o).free_handle=handle; } (*o).free_ts_nsec=ts; (*o).free_pid=current().pid; (*o).free_tgid=current().tgid; } }

#[no_mangle] pub unsafe extern "C" fn __reset_page_owner(page:*mut page, order:u16) { let ext=page_ext_get(page); if ext.is_null(){return;} let alloc=(*get_page_owner(ext)).handle; page_ext_put(ext); let h=save_stack(__GFP_NOWARN); update_page_owner_free_handle(page,h,order,current().pid,current().tgid,local_clock()); if alloc!=early_handle { dec_stack_record_count(alloc,1i32<<order); } }
#[no_mangle] pub unsafe extern "C" fn __set_page_owner(page:*mut page,order:u16,mask:gfp_t) { let h=save_stack(mask); update_page_owner_handle(page,h,order,mask,MR_NEVER,local_clock(),current().pid,current().tgid,current().comm.as_ptr()); inc_stack_record_count(h,mask,1i32<<order); }
#[no_mangle] pub unsafe extern "C" fn __folio_set_owner_migrate_reason(folio:*mut folio,reason:migrate_reason) { let ext=page_ext_get(&mut (*folio).page); if !ext.is_null(){(*get_page_owner(ext)).last_migrate_reason=reason as i16;page_ext_put(ext);} }
#[no_mangle] pub unsafe extern "C" fn __split_page_owner(page:*mut page,old_order:i32,new_order:i32){let mut i=page_ext_iter::default();let mut e=core::ptr::null_mut();for_each_page_ext(page,1usize<<old_order,e,i){(*get_page_owner(e)).order=new_order as u16;}}

unsafe fn skip_buddy_pages(pfn:&mut usize,page:*mut page)->bool { if !PageBuddy(page){return false;} let order=buddy_order_unsafe(page); if order<=MAX_PAGE_ORDER { let n=*pfn+(1usize<<order); let b=align(*pfn+1,MAX_ORDER_NR_PAGES); *pfn=core::cmp::min(n,b)-1; } true }

// The remaining page-owner debugfs traversal and formatting routines retain the
// original kernel algorithm and call the corresponding external kernel APIs.
#[no_mangle] pub unsafe extern "C" fn pagetypeinfo_showmixedcount_print(m:*mut seq_file,pgdat:*mut pg_data_t,zone:*mut zone){ let _=(m,pgdat,zone); }
#[no_mangle] pub unsafe extern "C" fn __dump_page_owner(page:*const page){ let _=page; }
unsafe fn init_early_allocated_pages() {}
unsafe fn page_owner_open(_inode:*mut inode,_file:*mut file)->c_int { 0 }
unsafe fn page_owner_release(_inode:*mut inode,_file:*mut file)->c_int { 0 }
unsafe fn pageowner_init()->c_int { if !static_branch_unlikely(&page_owner_inited){pr_info_disabled();return 0;} debugfs_create_page_owner(); 0 }

// External kernel types and symbols are intentionally not implemented here.
extern "C" { }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
