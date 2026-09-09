// SPDX-License-Identifier: GPL-2.0
// C dependencies supplied by the surrounding kernel translation.

extern "C" {
    fn __cpu_suspend(arg: ::core::ffi::c_ulong,
                     fn_: Option<unsafe extern "C" fn(::core::ffi::c_ulong) -> ::core::ffi::c_int>,
                     cpuid: u32) -> ::core::ffi::c_int;
    fn cpu_resume_mmu();
    fn cpu_logical_map(cpu: ::core::ffi::c_uint) -> u32;
    fn smp_processor_id() -> ::core::ffi::c_uint;
    fn pause_graph_tracing();
    fn unpause_graph_tracing();
    fn uaccess_save_and_enable();
    fn cpu_switch_mm(pgd: *mut core::ffi::c_void, mm: *mut mm_struct);
    fn local_flush_bp_all();
    fn local_flush_tlb_all();
    fn check_other_bugs();
    fn virt_to_phys(ptr: *const core::ffi::c_void) -> u32;
    fn cpu_do_resume();
    fn cpu_do_suspend(ptr: *mut u32);
    fn flush_cache_louis();
    fn __cpuc_flush_dcache_area(ptr: *const core::ffi::c_void, size: usize);
    fn outer_clean_range(start: u32, end: u32);
    fn mpidr_hash_size() -> usize;
    fn kcalloc(n: usize, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn sync_cache_w(ptr: *mut sleep_save_sp);
    fn warn_on(condition: bool) -> bool;
}

const EINVAL: ::core::ffi::c_int = 22;
const ENOMEM: ::core::ffi::c_int = 12;
const GFP_KERNEL: u32 = 0;

#[repr(C)]
pub struct mm_struct {
    pub pgd: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct sleep_save_sp {
    pub save_ptr_stash: *mut core::ffi::c_void,
    pub save_ptr_stash_phys: u32,
}

extern "C" {
    static mut idmap_pgd: *mut core::ffi::c_void;
    static mut sleep_save_sp: sleep_save_sp;
    static mut current: *mut task_struct;
}

#[repr(C)]
pub struct task_struct {
    pub active_mm: *mut mm_struct,
}

// CONFIG_MMU conditional implementation.
#[cfg(feature = "CONFIG_MMU")]
pub unsafe fn cpu_suspend(
    arg: ::core::ffi::c_ulong,
    fn_: Option<unsafe extern "C" fn(::core::ffi::c_ulong) -> ::core::ffi::c_int>,
) -> ::core::ffi::c_int {
    let mm = (*current).active_mm;
    let __mpidr = cpu_logical_map(smp_processor_id());
    let mut ret: ::core::ffi::c_int;

    if idmap_pgd.is_null() {
        return -EINVAL;
    }

    // Needed for the MMU disabling/enabling code to be able to run from TTBR0 addresses.
    // CONFIG_CPU_TTBR0_PAN is a build-time kernel condition.
    #[cfg(feature = "CONFIG_CPU_TTBR0_PAN")]
    uaccess_save_and_enable();

    // Disable graph tracing while executing suspend finishers, which never return.
    pause_graph_tracing();
    ret = __cpu_suspend(arg, fn_, __mpidr);
    unpause_graph_tracing();

    if ret == 0 {
        cpu_switch_mm((*mm).pgd, mm);
        local_flush_bp_all();
        local_flush_tlb_all();
        check_other_bugs();
    }

    ret
}

#[cfg(not(feature = "CONFIG_MMU"))]
pub unsafe fn cpu_suspend(
    arg: ::core::ffi::c_ulong,
    fn_: Option<unsafe extern "C" fn(::core::ffi::c_ulong) -> ::core::ffi::c_int>,
) -> ::core::ffi::c_int {
    let __mpidr = cpu_logical_map(smp_processor_id());
    pause_graph_tracing();
    let ret = __cpu_suspend(arg, fn_, __mpidr);
    unpause_graph_tracing();
    ret
}

// Called by __cpu_suspend() to save state and flush data needed while caches are not searched.
pub unsafe fn __cpu_suspend_save(ptr: *mut u32, ptrsz: u32, sp: u32, save_ptr: *mut u32) {
    let ctx = ptr;

    *save_ptr = virt_to_phys(ptr.cast());

    // This must correspond to the LDM in cpu_resume() assembly.
    *ptr = virt_to_phys(idmap_pgd.cast());
    ptr = ptr.add(1);
    *ptr = sp;
    ptr = ptr.add(1);
    *ptr = virt_to_phys(cpu_do_resume as *const () as *const core::ffi::c_void);
    ptr = ptr.add(1);

    cpu_do_suspend(ptr);
    flush_cache_louis();
    __cpuc_flush_dcache_area(ctx.cast(), ptrsz as usize);
    __cpuc_flush_dcache_area(save_ptr.cast(), core::mem::size_of::<u32>());
    outer_clean_range(*save_ptr, (*save_ptr).wrapping_add(ptrsz));
    outer_clean_range(virt_to_phys(save_ptr.cast()),
                      virt_to_phys(save_ptr.cast()).wrapping_add(core::mem::size_of::<u32>() as u32));
}

unsafe fn cpu_suspend_alloc_sp() -> ::core::ffi::c_int {
    let ctx_ptr: *mut core::ffi::c_void;
    // ctx_ptr is an array of physical addresses.
    ctx_ptr = kcalloc(mpidr_hash_size(), core::mem::size_of::<u32>(), GFP_KERNEL);

    if warn_on(ctx_ptr.is_null()) {
        return -ENOMEM;
    }
    sleep_save_sp.save_ptr_stash = ctx_ptr;
    sleep_save_sp.save_ptr_stash_phys = virt_to_phys(ctx_ptr);
    sync_cache_w(&raw mut sleep_save_sp);
    0
}

// early_initcall(cpu_suspend_alloc_sp);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
