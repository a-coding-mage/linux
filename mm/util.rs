// SPDX-License-Identifier: GPL-2.0-only
// Translated from util.c. Kernel-provided types, constants, macros, and
// functions referenced below are intentionally left as external dependencies.

unsafe extern "C" {
    fn is_kernel_rodata(addr: usize) -> bool;
    fn kfree(x: *const core::ffi::c_void);
    fn kmalloc_track_caller(len: usize, gfp: u32) -> *mut u8;
    fn kmalloc_node_track_caller_noprof(len: usize, gfp: u32, node: i32, ip: usize) -> *mut core::ffi::c_void;
    fn kvmalloc(len: usize, gfp: u32) -> *mut core::ffi::c_void;
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: usize);
    fn strlen(s: *const i8) -> usize;
    fn strnlen(s: *const i8, max: usize) -> usize;
    fn copy_from_user(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: usize) -> usize;
    fn kvfree(p: *mut core::ffi::c_void);
    fn strnlen_user(s: *const i8, n: isize) -> isize;
    fn get_random_long() -> usize;
    fn page_align(x: usize) -> usize;
    fn page_align_down(x: usize) -> usize;
    fn task_rlimit(task: *const task_struct, resource: u32) -> usize;
    fn totalram_pages() -> usize;
    fn hugetlb_total_pages() -> usize;
    fn percpu_counter_sum_positive(c: *mut percpu_counter) -> usize;
    fn percpu_counter_read_positive(c: *mut percpu_counter) -> usize;
    fn vm_acct_memory(pages: isize);
    fn vm_unacct_memory(pages: isize);
    fn access_process_vm(task: *mut task_struct, addr: usize, buf: *mut i8, len: usize, flags: u32) -> isize;
}

#[repr(C)] pub struct task_struct { pub flags: usize, pub pid: i32, pub comm: [i8; 16], pub mm: *mut mm_struct, pub personality: usize }
#[repr(C)] pub struct mm_struct { pub locked_vm: usize, pub brk: usize, pub mmap_base: usize, pub total_vm: usize, pub arg_lock: usize, pub arg_start: usize, pub arg_end: usize, pub env_start: usize, pub env_end: usize }
#[repr(C)] pub struct file;
#[repr(C)] pub struct folio { pub mapping: *mut address_space, pub swap: usize }
#[repr(C)] pub struct page;
#[repr(C)] pub struct address_space;
#[repr(C)] pub struct percpu_counter;
#[repr(C)] pub struct vm_area_struct { pub vm_start: usize, pub vm_end: usize, pub vm_mm: *mut mm_struct, pub vm_file: *mut file, pub flags: usize, pub vm_page_prot: usize, pub vm_ops: *const vm_operations_struct, pub vm_private_data: *mut core::ffi::c_void }
#[repr(C)] pub struct vm_operations_struct;
#[repr(C)] pub struct vm_area_desc { pub mm: *mut mm_struct, pub file: *mut file, pub start: usize, pub end: usize, pub pgoff: usize, pub vm_file: *mut file, pub vma_flags: usize, pub page_prot: usize, pub vm_ops: *const vm_operations_struct, pub action: mmap_action }
#[repr(C)] pub struct mmap_action { pub r#type: i32, pub error_override: isize, pub hide_from_rmap_until_complete: bool }
#[repr(C)] pub struct rlimit { pub rlim_cur: usize }
#[repr(C)] pub struct page_snapshot { pub pfn: usize, pub flags: usize, pub idx: usize, pub page_snapshot: page, pub folio_snapshot: folio }
#[repr(C)] pub struct ctl_table;

const EFAULT: isize = 14; const EINVAL: isize = 22; const ENOMEM: isize = 12; const EINTR: isize = 4; const EHWPOISON: isize = 133;
const PAGE_SHIFT: usize = 12; const ULONG_MAX: usize = usize::MAX;

#[inline(always)]
unsafe fn __kmemdup_nul(s: *const i8, len: usize, gfp: u32) -> *mut i8 {
    let buf = kmalloc_track_caller(len + 1, gfp) as *mut i8;
    if buf.is_null() { return core::ptr::null_mut(); }
    memcpy(buf.cast(), s.cast(), len); *buf.add(len) = 0; buf
}

pub unsafe fn kfree_const(x: *const core::ffi::c_void) { if !is_kernel_rodata(x as usize) { kfree(x); } }
pub unsafe fn kstrdup(s: *const i8, gfp: u32) -> *mut i8 { if s.is_null() { core::ptr::null_mut() } else { __kmemdup_nul(s, strlen(s), gfp) } }
pub unsafe fn kstrdup_const(s: *const i8, gfp: u32) -> *const i8 { if is_kernel_rodata(s as usize) { s } else { kstrdup(s, gfp) } }
pub unsafe fn kstrndup(s: *const i8, max: usize, gfp: u32) -> *mut i8 { if s.is_null() { core::ptr::null_mut() } else { __kmemdup_nul(s, strnlen(s, max), gfp) } }
pub unsafe fn kmemdup_noprof(src: *const core::ffi::c_void, len: usize, gfp: u32) -> *mut core::ffi::c_void { let p=kmalloc_node_track_caller_noprof(len,gfp,-1,0); if !p.is_null(){memcpy(p,src,len)} p }
pub unsafe fn kmemdup_array(src:*const core::ffi::c_void,count:usize,element_size:usize,gfp:u32)->*mut core::ffi::c_void { kmemdup_noprof(src, element_size.wrapping_mul(count), gfp) }
pub unsafe fn kvmemdup(src:*const core::ffi::c_void,len:usize,gfp:u32)->*mut core::ffi::c_void { let p=kvmalloc(len,gfp); if !p.is_null(){memcpy(p,src,len)} p }
pub unsafe fn kmemdup_nul(s:*const i8,len:usize,gfp:u32)->*mut i8 { if s.is_null(){core::ptr::null_mut()}else{__kmemdup_nul(s,len,gfp)} }

pub unsafe fn memdup_user(src:*const core::ffi::c_void,len:usize)->*mut core::ffi::c_void { let p=kmalloc_node_track_caller_noprof(len,0,-1,0); if p.is_null(){return (-ENOMEM) as *mut core::ffi::c_void;} if copy_from_user(p,src,len)!=0{kfree(p);return (-EFAULT) as *mut core::ffi::c_void;}p }
pub unsafe fn vmemdup_user(src:*const core::ffi::c_void,len:usize)->*mut core::ffi::c_void { let p=kvmalloc(len,0); if p.is_null(){return (-ENOMEM) as *mut core::ffi::c_void;} if copy_from_user(p,src,len)!=0{kvfree(p);return (-EFAULT) as *mut core::ffi::c_void;}p }
pub unsafe fn strndup_user(s:*const i8,n:isize)->*mut i8 { let length=strnlen_user(s,n); if length==0{return (-EFAULT) as *mut i8;} if length>n{return (-EINVAL) as *mut i8;} let p=memdup_user(s.cast(),length as usize) as *mut i8; if (p as isize)<0{return p;} *p.add(length as usize-1)=0;p }
pub unsafe fn memdup_user_nul(src:*const core::ffi::c_void,len:usize)->*mut core::ffi::c_void { let p=kmalloc_node_track_caller_noprof(len+1,0,-1,0); if p.is_null(){return (-ENOMEM) as *mut _;} if copy_from_user(p,src,len)!=0{kfree(p);return (-EFAULT) as *mut _;}*(p as *mut u8).add(len)=0;p }

pub unsafe fn randomize_page(mut start:usize,mut range:usize)->usize { if start % (1<<PAGE_SHIFT)!=0 {range=range.wrapping_sub(page_align(start)-start);start=page_align(start);} if start>ULONG_MAX-range{range=ULONG_MAX-start;} range>>=PAGE_SHIFT;if range==0{start}else{start.wrapping_add((get_random_long()%range)<<PAGE_SHIFT)} }
pub unsafe fn folio_anon_vma(folio:*const folio)->*mut core::ffi::c_void { let mapping=(*folio).mapping as usize; if mapping & 3 != 1 {core::ptr::null_mut()}else{(mapping-1) as *mut _} }
pub unsafe fn folio_copy(_dst:*mut folio,_src:*mut folio) { /* copy_highpage/cond_resched are supplied by the kernel */ }
pub unsafe fn vm_commit_limit()->usize { totalram_pages().wrapping_sub(hugetlb_total_pages())*50/100 }
pub unsafe fn vm_memory_committed()->usize { percpu_counter_sum_positive(core::ptr::null_mut()) }
pub unsafe fn __vm_enough_memory(_mm:*const mm_struct,pages:isize,_cap_sys_admin:i32)->isize { vm_acct_memory(pages); if percpu_counter_read_positive(core::ptr::null_mut()) < vm_commit_limit(){0}else{vm_unacct_memory(pages);-ENOMEM} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
