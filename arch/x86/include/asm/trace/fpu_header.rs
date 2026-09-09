/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies and tracepoint macro expansion are supplied by the
// surrounding kernel translation unit.

/// Opaque declaration of `struct fpu`.
#[repr(C)]
pub struct Fpu {
    _private: [u8; 0],
}

/// Payload corresponding to the `x86_fpu` trace event class.
#[repr(C)]
pub struct X86FpuEntry {
    pub fpu: *mut Fpu,
    pub load_fpu: bool,
    pub xfeatures: u64,
    pub xcomp_bv: u64,
}

/// Equivalent of the `TP_fast_assign` body for the `x86_fpu` event class.
///
/// The kernel-provided `test_thread_flag`, `TIF_NEED_FPU_LOAD`, and
/// `boot_cpu_has` operations, as well as the layout of `struct fpu`, remain
/// external dependencies of this header translation.
pub unsafe fn x86_fpu_fast_assign(
    entry: *mut X86FpuEntry,
    fpu: *mut Fpu,
    load_fpu: bool,
    osxsave: bool,
    xfeatures: u64,
    xcomp_bv: u64,
) {
    (*entry).fpu = fpu;
    (*entry).load_fpu = load_fpu;
    if osxsave {
        (*entry).xfeatures = xfeatures;
        (*entry).xcomp_bv = xcomp_bv;
    }
}

// TP_printk:
// "x86/fpu: %p load: %d xfeatures: %llx xcomp_bv: %llx"

// DEFINE_EVENT(x86_fpu, ...).  These declarations represent the externally
// generated tracepoint interfaces.
unsafe extern "C" {
    pub fn x86_fpu_before_save(fpu: *mut Fpu);
    pub fn x86_fpu_after_save(fpu: *mut Fpu);
    pub fn x86_fpu_regs_activated(fpu: *mut Fpu);
    pub fn x86_fpu_regs_deactivated(fpu: *mut Fpu);
    pub fn x86_fpu_dropped(fpu: *mut Fpu);
    pub fn x86_fpu_copy_dst(fpu: *mut Fpu);
    pub fn x86_fpu_xstate_check_failed(fpu: *mut Fpu);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
