// SPDX-License-Identifier: GPL-2.0-only
/*
 * crash.c - kernel crash support code.
 * Copyright (C) 2002-2004 Eric Biederman  <ebiederm@xmission.com>
 */

use core::ffi::{c_char, c_int, c_void};

// Kernel headers and configuration supplied by the surrounding translation unit.

pub static mut vmcoreinfo_data: *mut u8 = core::ptr::null_mut();
pub static mut vmcoreinfo_size: usize = 0;
pub static mut vmcoreinfo_note: *mut u32 = core::ptr::null_mut();

static mut vmcoreinfo_data_safecopy: *mut u8 = core::ptr::null_mut();

#[repr(C)]
pub struct hwerr_info {
    pub count: atomic_t,
    pub timestamp: time64_t,
}

pub static mut hwerr_data: [hwerr_info; HWERR_RECOV_MAX as usize] =
    [hwerr_info { count: atomic_t { counter: 0 }, timestamp: 0 }; HWERR_RECOV_MAX as usize];

extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(dst: *mut c_void, value: c_int, n: usize) -> *mut c_void;
    fn vscnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, args: *mut c_void) -> usize;
    fn ktime_get_real_seconds() -> time64_t;
    fn __pa(addr: *const c_void) -> phys_addr_t;
    fn alloc_pages_exact(size: usize, gfp: u32) -> *mut u32;
    fn __get_free_pages(gfp: u32, order: c_int) -> usize;
    fn free_pages(addr: usize, order: c_int);
    fn get_order(size: usize) -> c_int;
    fn atomic_inc(v: *mut atomic_t);
    fn log_buf_vmcoreinfo_setup();
    fn arch_crash_save_vmcoreinfo();
}

#[repr(C)]
pub struct atomic_t { pub counter: i32 }
pub type time64_t = i64;
pub type phys_addr_t = u64;
pub type Elf_Word = u32;
pub type u32_t = u32;

#[repr(C)]
struct elf_note { n_namesz: u32, n_descsz: u32, n_type: u32 }

pub unsafe fn append_elf_note(mut buf: *mut Elf_Word, name: *mut c_char,
                               type_: u32, data: *mut c_void, data_len: usize) -> *mut Elf_Word {
    let note = buf as *mut elf_note;
    (*note).n_namesz = (strlen(name) + 1) as u32;
    (*note).n_descsz = data_len as u32;
    (*note).n_type = type_;
    buf = buf.add((core::mem::size_of::<elf_note>() + core::mem::size_of::<Elf_Word>() - 1) / core::mem::size_of::<Elf_Word>());
    memcpy(buf as *mut c_void, name as *const c_void, (*note).n_namesz as usize);
    buf = buf.add(((*note).n_namesz as usize + core::mem::size_of::<Elf_Word>() - 1) / core::mem::size_of::<Elf_Word>());
    memcpy(buf as *mut c_void, data, data_len);
    buf.add((data_len + core::mem::size_of::<Elf_Word>() - 1) / core::mem::size_of::<Elf_Word>())
}

pub unsafe fn final_note(buf: *mut Elf_Word) {
    memset(buf as *mut c_void, 0, core::mem::size_of::<elf_note>());
}

unsafe fn update_vmcoreinfo_note() {
    let mut buf = vmcoreinfo_note;
    if vmcoreinfo_size == 0 { return; }
    buf = append_elf_note(buf, VMCOREINFO_NOTE_NAME as *mut c_char, 0,
                          vmcoreinfo_data as *mut c_void, vmcoreinfo_size);
    final_note(buf);
}

pub unsafe fn crash_update_vmcoreinfo_safecopy(ptr: *mut c_void) {
    if !ptr.is_null() { memcpy(ptr, vmcoreinfo_data as *const c_void, vmcoreinfo_size); }
    vmcoreinfo_data_safecopy = ptr as *mut u8;
}

pub unsafe fn crash_save_vmcoreinfo() {
    if vmcoreinfo_note.is_null() { return; }
    if !vmcoreinfo_data_safecopy.is_null() { vmcoreinfo_data = vmcoreinfo_data_safecopy; }
    vmcoreinfo_append_str(c"CRASHTIME=%lld\n".as_ptr(), core::ptr::null_mut());
    update_vmcoreinfo_note();
}

// C variadic definition represented with an opaque va_list carrier.
pub unsafe fn vmcoreinfo_append_str(fmt: *const c_char, args: *mut c_void) {
    let mut buf = [0i8; 0x50];
    let mut r = vscnprintf(buf.as_mut_ptr(), buf.len(), fmt, args);
    r = core::cmp::min(r, VMCOREINFO_BYTES as usize - vmcoreinfo_size);
    memcpy(vmcoreinfo_data.add(vmcoreinfo_size) as *mut c_void, buf.as_ptr() as *const c_void, r);
    vmcoreinfo_size += r;
}

#[inline]
pub unsafe fn arch_crash_save_vmcoreinfo_weak() {}

pub unsafe fn paddr_vmcoreinfo_note() -> phys_addr_t { __pa(vmcoreinfo_note as *const c_void) }

pub unsafe fn hwerr_log_error_type(src: hwerr_error_type) {
    if src < 0 || src >= HWERR_RECOV_MAX { return; }
    atomic_inc(&mut hwerr_data[src as usize].count);
    core::ptr::write_volatile(&mut hwerr_data[src as usize].timestamp, ktime_get_real_seconds());
}

unsafe fn crash_save_vmcoreinfo_init() -> c_int {
    let order = get_order(VMCOREINFO_BYTES as usize);
    vmcoreinfo_data = __get_free_pages(GFP_KERNEL | __GFP_ZERO, order) as *mut u8;
    if vmcoreinfo_data.is_null() { return -ENOMEM; }
    vmcoreinfo_note = alloc_pages_exact(VMCOREINFO_NOTE_SIZE, GFP_KERNEL | __GFP_ZERO);
    if vmcoreinfo_note.is_null() {
        free_pages(vmcoreinfo_data as usize, order);
        vmcoreinfo_data = core::ptr::null_mut();
        return -ENOMEM;
    }

    VMCOREINFO_OSRELEASE!(init_uts_ns.name.release);
    VMCOREINFO_BUILD_ID!();
    VMCOREINFO_PAGESIZE!(PAGE_SIZE);
    VMCOREINFO_SYMBOL!(init_uts_ns);
    VMCOREINFO_OFFSET!(uts_namespace, name);
    VMCOREINFO_SYMBOL!(node_online_map);
    #[cfg(CONFIG_MMU)] VMCOREINFO_SYMBOL_ARRAY!(swapper_pg_dir);
    VMCOREINFO_SYMBOL!(_stext);
    vmcoreinfo_append_str(c"NUMBER(VMALLOC_START)=0x%lx\n".as_ptr(), core::ptr::null_mut());
    VMCOREINFO_SYMBOL!(mem_map);
    VMCOREINFO_SYMBOL!(contig_page_data);
    VMCOREINFO_SYMBOL_ARRAY!(vmemmap);
    VMCOREINFO_SYMBOL_ARRAY!(mem_section);
    VMCOREINFO_LENGTH!(mem_section, NR_SECTION_ROOTS);
    VMCOREINFO_STRUCT_SIZE!(mem_section);
    VMCOREINFO_OFFSET!(mem_section, section_mem_map);
    VMCOREINFO_NUMBER!(SECTION_SIZE_BITS);
    VMCOREINFO_NUMBER!(MAX_PHYSMEM_BITS);
    VMCOREINFO_STRUCT_SIZE!(page);
    VMCOREINFO_STRUCT_SIZE!(pglist_data);
    VMCOREINFO_STRUCT_SIZE!(zone);
    VMCOREINFO_STRUCT_SIZE!(free_area);
    VMCOREINFO_STRUCT_SIZE!(list_head);
    VMCOREINFO_SIZE!(nodemask_t);
    VMCOREINFO_OFFSET!(page, flags);
    VMCOREINFO_OFFSET!(page, _refcount);
    VMCOREINFO_OFFSET!(page, mapping);
    VMCOREINFO_OFFSET!(page, lru);
    VMCOREINFO_OFFSET!(page, _mapcount);
    VMCOREINFO_OFFSET!(page, private);
    VMCOREINFO_OFFSET!(page, compound_info);
    VMCOREINFO_OFFSET!(pglist_data, node_zones);
    VMCOREINFO_OFFSET!(pglist_data, nr_zones);
    VMCOREINFO_OFFSET!(pglist_data, node_start_pfn);
    VMCOREINFO_OFFSET!(pglist_data, node_spanned_pages);
    VMCOREINFO_OFFSET!(pglist_data, node_id);
    VMCOREINFO_OFFSET!(zone, free_area);
    VMCOREINFO_OFFSET!(zone, vm_stat);
    VMCOREINFO_OFFSET!(zone, spanned_pages);
    VMCOREINFO_OFFSET!(free_area, free_list);
    VMCOREINFO_OFFSET!(list_head, next);
    VMCOREINFO_OFFSET!(list_head, prev);
    VMCOREINFO_LENGTH!(zone.free_area, NR_PAGE_ORDERS);
    log_buf_vmcoreinfo_setup();
    VMCOREINFO_LENGTH!(free_area.free_list, MIGRATE_TYPES);
    VMCOREINFO_NUMBER!(NR_FREE_PAGES);
    VMCOREINFO_NUMBER!(PG_lru);
    VMCOREINFO_NUMBER!(PG_private);
    VMCOREINFO_NUMBER!(PG_swapcache);
    VMCOREINFO_NUMBER!(PG_swapbacked);
    VMCOREINFO_NUMBER!(PAGE_SLAB_MAPCOUNT_VALUE);
    VMCOREINFO_NUMBER!(PG_hwpoison);
    VMCOREINFO_NUMBER!(PG_head_mask);
    VMCOREINFO_NUMBER!(PAGE_BUDDY_MAPCOUNT_VALUE);
    VMCOREINFO_NUMBER!(PAGE_HUGETLB_MAPCOUNT_VALUE);
    VMCOREINFO_NUMBER!(PAGE_OFFLINE_MAPCOUNT_VALUE);
    VMCOREINFO_NUMBER!(PAGE_UNACCEPTED_MAPCOUNT_VALUE);
    VMCOREINFO_SYMBOL!(kallsyms_names);
    VMCOREINFO_SYMBOL!(kallsyms_num_syms);
    VMCOREINFO_SYMBOL!(kallsyms_token_table);
    VMCOREINFO_SYMBOL!(kallsyms_token_index);
    VMCOREINFO_SYMBOL!(kallsyms_offsets);
    arch_crash_save_vmcoreinfo();
    update_vmcoreinfo_note();
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
