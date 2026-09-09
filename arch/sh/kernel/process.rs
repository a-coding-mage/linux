// SPDX-License-Identifier: GPL-2.0
// Dependencies corresponding to the original Linux and architecture headers
// are supplied externally.

#[repr(C)]
pub struct kmem_cache {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    pub thread: thread_struct,
}

#[repr(C)]
pub struct thread_struct {
    pub xstate: *mut core::ffi::c_void,
}

#[repr(C)]
pub union thread_xstate {
    _align: u64,
}

extern "C" {
    static mut boot_cpu_data: cpuinfo_arch;

    fn unlazy_fpu(src: *mut task_struct, regs: *mut pt_regs);
    fn task_pt_regs(task: *mut task_struct) -> *mut pt_regs;
    fn kmem_cache_alloc(cachep: *mut kmem_cache, flags: u32) -> *mut core::ffi::c_void;
    fn kmem_cache_free(cachep: *mut kmem_cache, objp: *mut core::ffi::c_void);
    fn memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize)
        -> *mut core::ffi::c_void;
    fn kmem_cache_create(
        name: *const core::ffi::c_char,
        size: usize,
        align: usize,
        flags: u32,
        ctor: *mut core::ffi::c_void,
    ) -> *mut kmem_cache;
}

#[repr(C)]
pub struct cpuinfo_arch {
    pub flags: u32,
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

pub const GFP_KERNEL: u32 = 0;
pub const SLAB_PANIC: u32 = 0;
pub const CPU_HAS_FPU: u32 = 1;

pub static mut task_xstate_cachep: *mut kmem_cache = core::ptr::null_mut();
pub static mut xstate_size: u32 = 0;

#[cfg(CONFIG_STACKPROTECTOR)]
#[no_mangle]
pub static mut __stack_chk_guard: usize = 0;

#[cfg(CONFIG_STACKPROTECTOR)]
// EXPORT_SYMBOL(__stack_chk_guard);

/*
 * this gets called so that we can store lazy state into memory and copy the
 * current task into the new thread.
 */
#[no_mangle]
pub unsafe extern "C" fn arch_dup_task_struct(
    dst: *mut task_struct,
    src: *mut task_struct,
) -> i32 {
    unlazy_fpu(src, task_pt_regs(src));
    core::ptr::copy_nonoverlapping(src, dst, 1);

    if !(*src).thread.xstate.is_null() {
        (*dst).thread.xstate = kmem_cache_alloc(task_xstate_cachep, GFP_KERNEL);
        if (*dst).thread.xstate.is_null() {
            return -12;
        }
        memcpy(
            (*dst).thread.xstate,
            (*src).thread.xstate,
            xstate_size as usize,
        );
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn free_thread_xstate(tsk: *mut task_struct) {
    if !(*tsk).thread.xstate.is_null() {
        kmem_cache_free(task_xstate_cachep, (*tsk).thread.xstate);
        (*tsk).thread.xstate = core::ptr::null_mut();
    }
}

#[no_mangle]
pub unsafe extern "C" fn arch_release_task_struct(tsk: *mut task_struct) {
    free_thread_xstate(tsk);
}

#[no_mangle]
pub unsafe extern "C" fn arch_task_cache_init() {
    if xstate_size == 0 {
        return;
    }

    task_xstate_cachep = kmem_cache_create(
        b"task_xstate\0".as_ptr() as *const core::ffi::c_char,
        xstate_size as usize,
        core::mem::align_of::<thread_xstate>(),
        SLAB_PANIC,
        core::ptr::null_mut(),
    );
}

#[cfg(CONFIG_SH_FPU_EMU)]
pub const HAVE_SOFTFP: bool = true;
#[cfg(not(CONFIG_SH_FPU_EMU))]
pub const HAVE_SOFTFP: bool = false;

extern "C" {
    type sh_fpu_hard_struct;
    type sh_fpu_soft_struct;
}

#[no_mangle]
pub unsafe extern "C" fn init_thread_xstate() {
    if boot_cpu_data.flags & CPU_HAS_FPU != 0 {
        xstate_size = core::mem::size_of::<sh_fpu_hard_struct>() as u32;
    } else if HAVE_SOFTFP {
        xstate_size = core::mem::size_of::<sh_fpu_soft_struct>() as u32;
    } else {
        xstate_size = 0;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
