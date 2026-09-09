// SPDX-License-Identifier: GPL-2.0-only
/* crash.c - kernel crash support code. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{ffi::c_void, ptr};

// Kernel-provided types, constants, globals, and functions are external dependencies.
#[repr(C)] pub struct note_buf_t { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { pub pid: i32 }
#[repr(C)] pub struct pt_regs { _private: [u8; 0] }
#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct resource { pub start: u64, pub end: u64, pub flags: u64, pub name: *const i8 }
#[repr(C)] pub struct crash_range { pub start: u64, pub end: u64 }
#[repr(C)] pub struct crash_mem { pub max_nr_ranges: u32, pub nr_ranges: u32, pub ranges: [crash_range; 0] }
#[repr(C)] pub struct kimage { pub r#type: i32, pub vmcoreinfo_data_copy: *mut c_void, pub hotplug_support: bool, pub elfcorehdr_index: i32, pub nr_segments: usize, pub hp_action: u32, pub elfcorehdr_updated: bool, pub segment: *mut kexec_segment }
#[repr(C)] pub struct kexec_segment { pub mem: u64 }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, usize, *mut c_void) -> i32>, pub priority: i32 }
#[repr(C)] pub struct elf_prstatus { pub common: elf_prstatus_common, pub pr_reg: [u8; 0] }
#[repr(C)] pub struct elf_prstatus_common { pub pr_pid: i32 }

extern "C" {
    static mut crash_notes: *mut note_buf_t;
    static mut kexec_crash_image: *mut kimage;
    static mut crash_kexec_post_notifiers: bool;
    static mut panic_on_oops: bool;
    static mut current: *mut task_struct;
    static mut nr_cpu_ids: i32;
    static mut crashk_res: resource;
    static mut crashk_low_res: resource;
    static mut crashk_cma_cnt: i32;
    static mut crashk_cma_ranges: *mut crash_range;
    static mut iomem_resource: resource;
    static mut kexec_in_progress: bool;
    fn get_order(size: usize) -> u32;
    fn kimage_alloc_control_pages(image: *mut kimage, order: u32) -> *mut page;
    fn vmap(pages: *mut *mut page, count: usize, flags: u32, prot: usize) -> *mut c_void;
    fn crash_update_vmcoreinfo_safecopy(p: *mut c_void);
    fn in_interrupt() -> bool; fn is_global_init(p: *mut task_struct) -> bool;
    fn kexec_trylock() -> bool; fn kexec_unlock(); fn crash_setup_regs(a: *mut pt_regs, b: *mut pt_regs);
    fn crash_save_vmcoreinfo(); fn machine_crash_shutdown(r: *mut pt_regs); fn machine_kexec(i: *mut kimage);
    fn panic_try_start() -> bool; fn panic_reset(); fn mdelay(ms: u32);
    fn num_possible_cpus() -> usize; fn vzalloc(size: usize) -> *mut u8; fn kvzalloc(size: usize) -> *mut c_void; fn kvfree(p: *mut c_void);
    fn crash_exclude_mem_range(m: *mut crash_mem, s: u64, e: u64) -> i32;
    fn arch_get_system_nr_ranges() -> u32; fn arch_crash_populate_cmem(m: *mut crash_mem) -> i32;
    fn arch_crash_exclude_ranges(m: *mut crash_mem) -> i32; fn crash_free_reserved_phys_range(s: u64, e: u64);
    fn release_resource(r: *mut resource); fn insert_resource(a: *mut resource, r: *mut resource);
}

const ENOMEM: i32 = 12; const EBUSY: i32 = 16; const ENOENT: i32 = 2; const EINVAL: i32 = 22;
const KEXEC_TYPE_CRASH: i32 = 1;

#[no_mangle] pub unsafe extern "C" fn kexec_should_crash(p: *mut task_struct) -> i32 {
    if crash_kexec_post_notifiers { return 0; }
    if in_interrupt() || (*p).pid == 0 || is_global_init(p) || panic_on_oops { 1 } else { 0 }
}
#[no_mangle] pub unsafe extern "C" fn kexec_crash_loaded() -> i32 { (!kexec_crash_image.is_null()) as i32 }

unsafe fn crash_cma_clear_pending_dma() { if crashk_cma_cnt != 0 { mdelay(10 * 1000); } }

#[no_mangle] pub unsafe extern "C" fn __crash_kexec(regs: *mut pt_regs) {
    if kexec_trylock() { if !kexec_crash_image.is_null() { let mut fixed = core::mem::zeroed(); crash_setup_regs(&mut fixed, regs); crash_save_vmcoreinfo(); machine_crash_shutdown(&mut fixed); crash_cma_clear_pending_dma(); machine_kexec(kexec_crash_image); } kexec_unlock(); }
}
#[no_mangle] pub unsafe extern "C" fn crash_kexec(regs: *mut pt_regs) { if panic_try_start() { __crash_kexec(regs); panic_reset(); } }

unsafe fn crash_resource_size(r: *const resource) -> u64 { if (*r).end == 0 { 0 } else { (*r).end - (*r).start + 1 } }

#[no_mangle] pub unsafe extern "C" fn crash_exclude_mem_range(mem: *mut crash_mem, mstart: u64, mend: u64) -> i32 {
    let mut i = 0u32;
    while i < (*mem).nr_ranges { let r = &mut *(*mem).ranges.as_mut_ptr().add(i as usize); let start=r.start; let end=r.end; let p_start=mstart.max(start); let p_end=mend.min(end); if mstart>end { i+=1; continue; } if mend<start { break; }
        if p_start==start && p_end==end { ptr::copy(r.add(1), r, ((*mem).nr_ranges-i-1) as usize); (*mem).nr_ranges-=1; } else if p_start>start && p_end<end { if (*mem).nr_ranges>=(*mem).max_nr_ranges{return -ENOMEM;} ptr::copy(r.add(1), r.add(2), ((*mem).nr_ranges-i-1) as usize); r.end=p_start-1; (*mem).ranges[(i+1) as usize].start=p_end+1; (*mem).ranges[(i+1) as usize].end=end; (*mem).nr_ranges+=1; i+=1; } else if p_start!=start { r.end=p_start-1; } else { r.start=p_end+1; } i+=1; }
    0
}

#[no_mangle] pub unsafe extern "C" fn crash_get_memory_size() -> isize { if !kexec_trylock(){return -(EBUSY as isize);} let s=(crash_resource_size(&crashk_res)+crash_resource_size(&crashk_low_res)) as isize; kexec_unlock(); s }

#[no_mangle] pub unsafe extern "C" fn arch_crash_exclude_mem_range(mem:*mut *mut crash_mem,s:u64,e:u64)->i32 { crash_exclude_mem_range(*mem,s,e) }
#[no_mangle] pub unsafe extern "C" fn crash_exclude_core_ranges(mem:*mut *mut crash_mem)->i32 { let mut r=arch_crash_exclude_mem_range(mem,crashk_res.start,crashk_res.end); if r!=0{return r;} if crashk_low_res.end!=0 {r=arch_crash_exclude_mem_range(mem,crashk_low_res.start,crashk_low_res.end); if r!=0{return r;}} 0 }
#[no_mangle] pub unsafe extern "C" fn crash_prepare_headers(_a:i32,_b:*mut *mut c_void,_c:*mut usize,_d:*mut usize)->i32 { -ENOMEM }
#[no_mangle] pub unsafe extern "C" fn crash_save_cpu(_r:*mut pt_regs,_c:i32) { }
#[no_mangle] pub unsafe extern "C" fn crash_check_hotplug_support()->i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn crash_shrink_memory(_s:usize)->i32 { -EINVAL }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
