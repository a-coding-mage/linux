/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies supplied by the surrounding kernel translation unit:
// linux/uaccess.h, asm/fpu.h, asm/lbt.h, asm/mmu_context.h, asm/page.h,
// asm/ftrace.h, and asm-generic/asm-prototypes.h.

// Preserved from CONFIG_ARCH_SUPPORTS_INT128.
#[cfg(CONFIG_ARCH_SUPPORTS_INT128)]
extern "C" {
    pub fn __ashlti3(a: i128, b: core::ffi::c_int) -> i128;
    pub fn __ashrti3(a: i128, b: core::ffi::c_int) -> i128;
    pub fn __lshrti3(a: i128, b: core::ffi::c_int) -> i128;
}

// asmlinkage, noinstr, and __no_stack_protector are retained as source-level
// attributes in the C declaration and have no direct file-local Rust mapping.
extern "C" {
    pub fn ret_from_fork(prev: *mut task_struct, regs: *mut pt_regs);

    pub fn ret_from_kernel_thread(
        prev: *mut task_struct,
        regs: *mut pt_regs,
        func: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> core::ffi::c_int>,
        fn_arg: *mut core::ffi::c_void,
    );

    pub fn kvm_exc_entry();
    pub fn kvm_enter_guest(run: *mut kvm_run, vcpu: *mut kvm_vcpu) -> core::ffi::c_int;

    pub fn kvm_save_fpu(fpu: *mut loongarch_fpu);
    pub fn kvm_restore_fpu(fpu: *mut loongarch_fpu);
}

#[repr(C)]
pub struct kvm_run {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kvm_vcpu {
    _private: [u8; 0],
}

#[repr(C)]
pub struct loongarch_fpu {
    _private: [u8; 0],
}

// External types supplied by the surrounding kernel translation unit.
// Preserved from CONFIG_CPU_HAS_LSX.
#[cfg(CONFIG_CPU_HAS_LSX)]
extern "C" {
    pub fn kvm_save_lsx(fpu: *mut loongarch_fpu);
    pub fn kvm_restore_lsx(fpu: *mut loongarch_fpu);
}

// Preserved from CONFIG_CPU_HAS_LASX.
#[cfg(CONFIG_CPU_HAS_LASX)]
extern "C" {
    pub fn kvm_save_lasx(fpu: *mut loongarch_fpu);
    pub fn kvm_restore_lasx(fpu: *mut loongarch_fpu);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
