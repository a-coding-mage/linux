// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Copyright (C) 1994 Linus Torvalds
 *
 *  Pentium III FXSR, SSE support
 *  General FPU state handling cleanups
 *	Gareth Hughes <gareth@valinux.com>, May 2000
 */

// Dependencies supplied by the surrounding kernel translation unit.

#[cfg(target_arch = "x86_64")]
static mut XFD_STATE: u64 = 0;

// The FPU state configuration data for kernel and user space.
#[no_mangle] pub static mut fpu_kernel_cfg: fpu_state_config = unsafe { core::mem::zeroed() };
#[no_mangle] pub static mut fpu_user_cfg: fpu_state_config = unsafe { core::mem::zeroed() };
#[no_mangle] pub static mut guest_default_cfg: vcpu_fpu_config = unsafe { core::mem::zeroed() };

// Represents the initial FPU state.
#[no_mangle] pub static mut init_fpstate: fpstate = unsafe { core::mem::zeroed() };

#[no_mangle] pub static mut kernel_fpu_allowed: bool = false;
#[no_mangle] pub static mut fpu_fpregs_owner_ctx: *mut fpu = core::ptr::null_mut();

#[cfg(CONFIG_X86_DEBUG_FPU)]
pub unsafe fn x86_task_fpu(task: *mut task_struct) -> *mut fpu {
    if WARN_ON_ONCE((*task).flags & PF_KTHREAD != 0) { return core::ptr::null_mut(); }
    (task as *mut u8).add(core::mem::size_of::<task_struct>()) as *mut fpu
}

pub unsafe fn irq_fpu_usable() -> bool {
    if WARN_ON_ONCE(in_nmi()) { return false; }
    if !this_cpu_read(kernel_fpu_allowed) { return false; }
    if !in_hardirq() { return true; }
    !softirq_count()
}

unsafe fn update_avx_timestamp(fpu: *mut fpu) {
    const AVX512_TRACKING_MASK: u64 = XFEATURE_MASK_ZMM_HI256 | XFEATURE_MASK_HI16_ZMM;
    if (*(*fpu).fpstate).regs.xsave.header.xfeatures & AVX512_TRACKING_MASK != 0 {
        (*fpu).avx512_timestamp = jiffies;
    }
}

pub unsafe fn save_fpregs_to_fpstate(fpu: *mut fpu) {
    if likely(use_xsave()) { os_xsave((*fpu).fpstate); update_avx_timestamp(fpu); return; }
    if likely(use_fxsr()) { fxsave(&mut (*fpu).fpstate.as_mut().unwrap().regs.fxsave); return; }
    // Legacy FNSAVE clears the registers, so reload them.
    core::arch::asm!("fnsave [{}]; fwait", in(reg) &mut (*fpu).fpstate.as_mut().unwrap().regs.fsave);
    frstor(&(*fpu).fpstate.as_ref().unwrap().regs.fsave);
}

pub unsafe fn restore_fpregs_from_fpstate(fpstate: *mut fpstate, mut mask: u64) {
    if unlikely(static_cpu_has_bug(X86_BUG_FXSAVE_LEAK)) {
        core::arch::asm!("fnclex", "emms", "fildl [{}]", in(reg) fpstate);
    }
    if use_xsave() {
        xfd_update_state(fpstate);
        mask = fpu_kernel_cfg.max_features & mask;
        os_xrstor(fpstate, mask);
    } else if use_fxsr() { fxrstor(&(*fpstate).regs.fxsave); } else { frstor(&(*fpstate).regs.fsave); }
}

pub unsafe fn fpu_reset_from_exception_fixup() { restore_fpregs_from_fpstate(&raw mut init_fpstate, XFEATURE_MASK_FPSTATE); }

#[cfg(CONFIG_KVM)]
unsafe fn __fpstate_reset(fpstate: *mut fpstate) { __fpstate_reset_impl(fpstate); }

#[cfg(CONFIG_KVM)]
unsafe fn fpu_lock_guest_permissions() {
    if !IS_ENABLED(CONFIG_X86_64) { return; }
    spin_lock_irq((*current).sighand.as_ref().unwrap().siglock);
    let fpuperm = &mut (*x86_task_fpu((*current).group_leader)).guest_perm;
    let perm = fpuperm.__state_perm;
    WRITE_ONCE(fpuperm.__state_perm, perm | FPU_GUEST_PERM_LOCKED);
    spin_unlock_irq((*current).sighand.as_ref().unwrap().siglock);
}

#[cfg(CONFIG_KVM)]
pub unsafe fn fpu_alloc_guest_fpstate(gfpu: *mut fpu_guest) -> bool {
    let size = guest_default_cfg.size + ALIGN(core::mem::offset_of!(fpstate, regs), 64);
    let fpstate = vzalloc(size) as *mut fpstate;
    if fpstate.is_null() { return false; }
    (*fpstate).is_valloc = true; (*fpstate).is_guest = true;
    __fpstate_reset(fpstate); fpstate_init_user(fpstate);
    (*gfpu).fpstate = fpstate; (*gfpu).xfeatures = guest_default_cfg.features;
    (*gfpu).uabi_size = core::mem::size_of::<kvm_xsave>();
    if WARN_ON_ONCE(fpu_user_cfg.default_size > (*gfpu).uabi_size) { (*gfpu).uabi_size = fpu_user_cfg.default_size; }
    fpu_lock_guest_permissions(); true
}

#[cfg(CONFIG_KVM)]
pub unsafe fn fpu_free_guest_fpstate(gfpu: *mut fpu_guest) {
    let fpstate = (*gfpu).fpstate; if fpstate.is_null() { return; }
    if WARN_ON_ONCE(!(*fpstate).is_valloc || !(*fpstate).is_guest || (*fpstate).in_use) { return; }
    (*gfpu).fpstate = core::ptr::null_mut(); vfree(fpstate as *mut core::ffi::c_void);
}

#[cfg(CONFIG_KVM)]
pub unsafe fn fpu_enable_guest_xfd_features(guest_fpu: *mut fpu_guest, mut xfeatures: u64) -> i32 {
    lockdep_assert_preemption_enabled(); xfeatures &= !(*guest_fpu).xfeatures;
    if xfeatures == 0 { return 0; } __xfd_enable_feature(xfeatures, guest_fpu)
}

#[cfg(all(CONFIG_KVM, target_arch = "x86_64"))]
pub unsafe fn fpu_update_guest_xfd(guest_fpu: *mut fpu_guest, xfd: u64) {
    let fpstate = (*guest_fpu).fpstate; fpregs_lock();
    if xfd != 0 && test_thread_flag(TIF_NEED_FPU_LOAD) { (*fpstate).regs.xsave.header.xfeatures &= !xfd; }
    (*fpstate).xfd = xfd; if (*fpstate).in_use { xfd_update_state(fpstate); } fpregs_unlock();
}

#[cfg(all(CONFIG_KVM, target_arch = "x86_64"))]
pub unsafe fn fpu_sync_guest_vmexit_xfd_state() {
    let fpstate = (*x86_task_fpu(current)).fpstate; lockdep_assert_irqs_disabled();
    if fpu_state_size_dynamic() { rdmsrq(MSR_IA32_XFD, &mut (*fpstate).xfd); __this_cpu_write(XFD_STATE, (*fpstate).xfd); }
}

#[cfg(CONFIG_KVM)]
pub unsafe fn fpu_swap_kvm_fpstate(guest_fpu: *mut fpu_guest, enter_guest: bool) -> i32 {
    let guest_fps = (*guest_fpu).fpstate; let fpu = x86_task_fpu(current); let mut cur_fps = (*fpu).fpstate;
    fpregs_lock(); if !(*cur_fps).is_confidential && !test_thread_flag(TIF_NEED_FPU_LOAD) { save_fpregs_to_fpstate(fpu); }
    if enter_guest { (*fpu).__task_fpstate = cur_fps; (*fpu).fpstate = guest_fps; (*guest_fps).in_use = true; }
    else { (*guest_fps).in_use = false; (*fpu).fpstate = (*fpu).__task_fpstate; (*fpu).__task_fpstate = core::ptr::null_mut(); }
    cur_fps = (*fpu).fpstate;
    if !(*cur_fps).is_confidential { restore_fpregs_from_fpstate(cur_fps, XFEATURE_MASK_FPSTATE); } else { xfd_update_state(cur_fps); }
    fpregs_mark_activate(); fpregs_unlock(); 0
}

#[cfg(CONFIG_KVM)]
pub unsafe fn fpu_copy_guest_fpstate_to_uabi(gfpu: *mut fpu_guest, buf: *mut core::ffi::c_void, size: u32, xfeatures: u64, pkru: u32) {
    let kstate = (*gfpu).fpstate; let ustate = buf as *mut fpregs_state; let mb = membuf { p: buf, left: size };
    if cpu_feature_enabled(X86_FEATURE_XSAVE) { __copy_xstate_to_uabi_buf(mb, kstate, xfeatures, pkru, XSTATE_COPY_XSAVE); }
    else { core::ptr::copy_nonoverlapping(&(*kstate).regs.fxsave, &mut (*ustate).fxsave, 1); (*ustate).xsave.header.xfeatures = XFEATURE_MASK_FPSSE; }
}

#[cfg(CONFIG_KVM)]
pub unsafe fn fpu_copy_uabi_to_guest_fpstate(gfpu: *mut fpu_guest, buf: *const core::ffi::c_void, xcr0: u64, vpkru: *mut u32) -> i32 {
    let kstate = (*gfpu).fpstate; let ustate = buf as *const fpregs_state;
    if !cpu_feature_enabled(X86_FEATURE_XSAVE) { if (*ustate).xsave.header.xfeatures & !XFEATURE_MASK_FPSSE != 0 || (*ustate).fxsave.mxcsr & !mxcsr_feature_mask != 0 { return -EINVAL; } core::ptr::copy_nonoverlapping(&(*ustate).fxsave, &mut (*kstate).regs.fxsave, 1); return 0; }
    if (*ustate).xsave.header.xfeatures & !xcr0 != 0 || ((*ustate).xsave.header.xfeatures & (*kstate).xfd) != 0 { return -EINVAL; }
    let mut v = vpkru; if (*ustate).xsave.header.xfeatures & XFEATURE_MASK_PKRU == 0 { v = core::ptr::null_mut(); }
    copy_uabi_from_kernel_to_xstate(kstate, ustate, v)
}

// The remaining routines are direct low-level translations of the source.
pub unsafe fn kernel_fpu_begin_mask(kfpu_mask: u32) {
    if !irqs_disabled() { fpregs_lock(); }
    WARN_ON_FPU(!irq_fpu_usable()); this_cpu_write(kernel_fpu_allowed, false);
    if ((*current).flags & (PF_KTHREAD | PF_USER_WORKER)) == 0 && !test_thread_flag(TIF_NEED_FPU_LOAD) { set_thread_flag(TIF_NEED_FPU_LOAD); save_fpregs_to_fpstate(x86_task_fpu(current)); }
    __cpu_invalidate_fpregs_state();
    if likely(kfpu_mask & KFPU_MXCSR != 0) && boot_cpu_has(X86_FEATURE_XMM) { ldmxcsr(MXCSR_DEFAULT); }
    if unlikely(kfpu_mask & KFPU_387 != 0) && boot_cpu_has(X86_FEATURE_FPU) { core::arch::asm!("fninit"); }
}

pub unsafe fn kernel_fpu_end() { WARN_ON_FPU(this_cpu_read(kernel_fpu_allowed)); this_cpu_write(kernel_fpu_allowed, true); if !irqs_disabled() { fpregs_unlock(); } }

pub unsafe fn fpstate_init_fxstate(fpstate: *mut fpstate) { (*fpstate).regs.fxsave.cwd = 0x37f; (*fpstate).regs.fxsave.mxcsr = MXCSR_DEFAULT; }
pub unsafe fn fpstate_init_fstate(fpstate: *mut fpstate) { (*fpstate).regs.fsave.cwd = 0xffff037f; (*fpstate).regs.fsave.swd = 0xffff0000; (*fpstate).regs.fsave.twd = 0xffffffff; (*fpstate).regs.fsave.fos = 0xffff0000; }
pub unsafe fn fpstate_init_user(fpstate: *mut fpstate) { xstate_init_xcomp_bv(&mut (*fpstate).regs.xsave, (*fpstate).xfeatures); if cpu_feature_enabled(X86_FEATURE_FXSR) { fpstate_init_fxstate(fpstate); } else { fpstate_init_fstate(fpstate); } }

unsafe fn __fpstate_reset_impl(fpstate: *mut fpstate) {
    if (*fpstate).is_guest { (*fpstate).size = guest_default_cfg.size; (*fpstate).xfeatures = guest_default_cfg.features; (*fpstate).xfd = 0; }
    else { (*fpstate).size = fpu_kernel_cfg.default_size; (*fpstate).xfeatures = fpu_kernel_cfg.default_features; (*fpstate).xfd = init_fpstate.xfd; }
    (*fpstate).user_size = fpu_user_cfg.default_size; (*fpstate).user_xfeatures = fpu_user_cfg.default_features;
}

pub unsafe fn fpstate_reset(fpu: *mut fpu) { (*fpu).fpstate = &mut (*fpu).__fpstate; __fpstate_reset_impl((*fpu).fpstate); (*fpu).perm.__state_perm = fpu_kernel_cfg.default_features; (*fpu).perm.__state_size = fpu_kernel_cfg.default_size; (*fpu).perm.__user_state_size = fpu_user_cfg.default_size; (*fpu).guest_perm.__state_perm = guest_default_cfg.features; (*fpu).guest_perm.__state_size = guest_default_cfg.size; (*fpu).guest_perm.__user_state_size = fpu_user_cfg.default_size; }

pub unsafe fn fpu_sync_fpstate(fpu: *mut fpu) { WARN_ON_FPU(fpu != x86_task_fpu(current)); fpregs_lock(); trace_x86_fpu_before_save(fpu); if !test_thread_flag(TIF_NEED_FPU_LOAD) { save_fpregs_to_fpstate(fpu); } trace_x86_fpu_after_save(fpu); fpregs_unlock(); }

#[inline] unsafe fn init_fpstate_copy_size() -> usize { if !use_xsave() { fpu_kernel_cfg.default_size } else { core::mem::size_of_val(&init_fpstate.regs.xsave) } }

unsafe fn fpu_reset_fpstate_regs() {
    let fpu = x86_task_fpu(current); fpregs_lock(); __fpu_invalidate_fpregs_state(fpu);
    core::ptr::copy_nonoverlapping(&init_fpstate.regs as *const _, &mut (*(*fpu).fpstate).regs as *mut _, 1);
    set_thread_flag(TIF_NEED_FPU_LOAD); fpregs_unlock();
}

pub unsafe fn fpu__drop(tsk: *mut task_struct) {
    if test_tsk_thread_flag(tsk, TIF_NEED_FPU_LOAD) { return; }
    let fpu = x86_task_fpu(tsk); preempt_disable();
    if fpu == x86_task_fpu(current) { core::arch::asm!("fwait"); fpregs_deactivate(fpu); }
    trace_x86_fpu_dropped(fpu); preempt_enable();
}

unsafe fn restore_fpregs_from_init_fpstate(features_mask: u64) {
    if use_xsave() { os_xrstor(&raw mut init_fpstate, features_mask); }
    else if use_fxsr() { fxrstor(&init_fpstate.regs.fxsave); }
    else { frstor(&init_fpstate.regs.fsave); }
    pkru_write_default();
}

pub unsafe fn fpu__clear_user_states(fpu: *mut fpu) {
    WARN_ON_FPU(fpu != x86_task_fpu(current)); fpregs_lock();
    if !cpu_feature_enabled(X86_FEATURE_FPU) { fpu_reset_fpstate_regs(); fpregs_unlock(); return; }
    if xfeatures_mask_supervisor() != 0 && !fpregs_state_valid(fpu, smp_processor_id()) { os_xrstor_supervisor((*fpu).fpstate); }
    xfd_update_state((*fpu).fpstate); restore_fpregs_from_init_fpstate(XFEATURE_MASK_USER_RESTORE);
    fpregs_mark_activate(); fpregs_unlock();
}

pub unsafe fn fpregs_lock_and_load() {
    WARN_ON_ONCE(!irq_fpu_usable()); WARN_ON_ONCE((*current).flags & PF_KTHREAD != 0); fpregs_lock(); fpregs_assert_state_consistent(); if test_thread_flag(TIF_NEED_FPU_LOAD) { fpregs_restore_userregs(); }
}

#[cfg(CONFIG_X86_DEBUG_FPU)]
pub unsafe fn fpregs_assert_state_consistent() { let fpu = x86_task_fpu(current); if test_thread_flag(TIF_NEED_FPU_LOAD) { return; } WARN_ON_FPU(!fpregs_state_valid(fpu, smp_processor_id())); }

pub unsafe fn fpu__exception_code(fpu: *mut fpu, trap_nr: i32) -> i32 {
    let err: u32;
    if trap_nr == X86_TRAP_MF {
        let (cwd, swd) = if boot_cpu_has(X86_FEATURE_FXSR) { ((*fpu).fpstate.as_ref().unwrap().regs.fxsave.cwd as u16, (*fpu).fpstate.as_ref().unwrap().regs.fxsave.swd as u16) } else { ((*fpu).fpstate.as_ref().unwrap().regs.fsave.cwd as u16, (*fpu).fpstate.as_ref().unwrap().regs.fsave.swd as u16) };
        err = (swd & !cwd) as u32;
    } else { let mxcsr = if boot_cpu_has(X86_FEATURE_XMM) { (*fpu).fpstate.as_ref().unwrap().regs.fxsave.mxcsr } else { MXCSR_DEFAULT }; err = (!(mxcsr >> 7) & mxcsr) as u32; }
    if err & 0x001 != 0 { FPE_FLTINV } else if err & 0x004 != 0 { FPE_FLTDIV } else if err & 0x008 != 0 { FPE_FLTOVF } else if err & 0x012 != 0 { FPE_FLTUND } else if err & 0x020 != 0 { FPE_FLTRES } else { 0 }
}

pub unsafe fn fpu_flush_thread() { fpstate_reset(x86_task_fpu(current)); fpu_reset_fpstate_regs(); }
pub unsafe fn switch_fpu_return() { if cpu_feature_enabled(X86_FEATURE_FPU) { fpregs_restore_userregs(); } }
pub unsafe fn fpregs_mark_activate() { let fpu = x86_task_fpu(current); fpregs_activate(fpu); (*fpu).last_cpu = smp_processor_id(); clear_thread_flag(TIF_NEED_FPU_LOAD); }

pub unsafe fn fpu_idle_fpregs() { if cpu_feature_enabled(X86_FEATURE_AMX_TILE) && xfeatures_in_use() & XFEATURE_MASK_XTILE != 0 { tile_release(); __this_cpu_write(fpu_fpregs_owner_ctx, core::ptr::null_mut()); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
