/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// asm/cpufeature.h, asm/fpu/xstate.h, asm/fpu/xcr.h, and asm/msr.h.

#[cfg(target_arch = "x86_64")]
extern "C" {
    pub static mut xfd_state: u64;
}

#[inline]
pub unsafe fn xstate_init_xcomp_bv(xsave: *mut xregs_state, mask: u64) {
    /* XRSTORS requires these bits set in xcomp_bv, or it will trigger #GP. */
    if cpu_feature_enabled(X86_FEATURE_XCOMPACTED) {
        (*xsave).header.xcomp_bv = mask | XCOMP_BV_COMPACTED_FORMAT;
    }
}

#[inline]
pub unsafe fn xstate_get_group_perm(guest: bool) -> u64 {
    let fpu = x86_task_fpu((*current).group_leader);
    let perm = if guest { &(*fpu).guest_perm } else { &(*fpu).perm };
    core::ptr::read_volatile(&perm.__state_perm)
}

#[inline]
pub unsafe fn xstate_get_host_group_perm() -> u64 {
    xstate_get_group_perm(false)
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum xstate_copy_mode {
    XSTATE_COPY_FP,
    XSTATE_COPY_FX,
    XSTATE_COPY_XSAVE,
}

extern "C" {
    pub fn __copy_xstate_to_uabi_buf(to: membuf, fpstate: *mut fpstate,
        xfeatures: u64, pkru_val: u32, copy_mode: xstate_copy_mode);
    pub fn copy_xstate_to_uabi_buf(to: membuf, tsk: *mut task_struct,
        mode: xstate_copy_mode);
    pub fn copy_uabi_from_kernel_to_xstate(fpstate: *mut fpstate,
        kbuf: *const core::ffi::c_void, pkru: *mut u32) -> i32;
    pub fn copy_sigframe_from_user_to_xstate(tsk: *mut task_struct,
        ubuf: *const core::ffi::c_void) -> i32;
    pub fn fpu__init_cpu_xstate();
    pub fn fpu__init_system_xstate(legacy_size: u32);
    pub fn get_xsave_addr_user(xsave: *mut xregs_state, xfeature_nr: i32)
        -> *mut core::ffi::c_void;
}

#[inline]
pub unsafe fn xfeatures_mask_supervisor() -> u64 {
    fpu_kernel_cfg.max_features & XFEATURE_MASK_SUPERVISOR_SUPPORTED
}

#[inline]
pub unsafe fn xfeatures_mask_independent() -> u64 {
    if !cpu_feature_enabled(X86_FEATURE_ARCH_LBR) {
        fpu_kernel_cfg.independent_features & !XFEATURE_MASK_LBR
    } else { fpu_kernel_cfg.independent_features }
}

#[inline]
pub unsafe fn set_xfeature_in_sigframe(xbuf: *mut xregs_state, mask: u64) -> i32 {
    let mut xfeatures = 0u64;
    let mut err = __get_user(&mut xfeatures, &(*xbuf).header.xfeatures);
    xfeatures |= mask;
    err |= __put_user(xfeatures, &mut (*xbuf).header.xfeatures);
    err
}

#[inline]
pub unsafe fn update_pkru_in_sigframe(buf: *mut xregs_state, pkru: u32) -> i32 {
    if !cpu_feature_enabled(X86_FEATURE_OSPKE) { return 0; }
    let err = set_xfeature_in_sigframe(buf, XFEATURE_MASK_PKRU);
    if err != 0 { return err; }
    __put_user(pkru, get_xsave_addr_user(buf, XFEATURE_PKRU) as *mut u32)
}

// The following instruction wrappers preserve the C inline-assembly interfaces.
// Their instruction selection and exception-table annotations are supplied by the
// architecture-specific assembly integration.
#[macro_export]
macro_rules! XSTATE_OP { ($op:expr, $st:expr, $lmask:expr, $hmask:expr, $err:expr) => {{
    $err = xstate_op($op, $st, $lmask, $hmask);
}} }
#[macro_export]
macro_rules! XSTATE_XSAVE { ($st:expr, $lmask:expr, $hmask:expr, $err:expr) => {{
    $err = xstate_xsave($st, $lmask, $hmask);
}} }
#[macro_export]
macro_rules! XSTATE_XRESTORE { ($st:expr, $lmask:expr, $hmask:expr) => {{
    xstate_xrestore($st, $lmask, $hmask);
}} }

#[cfg(all(target_arch = "x86_64", feature = "CONFIG_X86_DEBUG_FPU"))]
extern "C" { pub fn xfd_validate_state(fpstate: *mut fpstate, mask: u64, rstor: bool); }
#[cfg(not(all(target_arch = "x86_64", feature = "CONFIG_X86_DEBUG_FPU")))]
#[inline] pub unsafe fn xfd_validate_state(_: *mut fpstate, _: u64, _: bool) {}

#[cfg(target_arch = "x86_64")]
#[inline] pub unsafe fn xfd_set_state(xfd: u64) {
    wrmsrq(MSR_IA32_XFD, xfd);
    core::ptr::write_volatile(&mut xfd_state, xfd);
}
#[cfg(target_arch = "x86_64")]
#[inline] pub unsafe fn xfd_update_state(fpstate: *mut fpstate) {
    if fpu_state_size_dynamic() && core::ptr::read_volatile(&xfd_state) != (*fpstate).xfd {
        xfd_set_state((*fpstate).xfd);
    }
}
#[cfg(target_arch = "x86_64")]
extern "C" { pub fn __xfd_enable_feature(which: u64, guest_fpu: *mut fpu_guest) -> i32; }
#[cfg(not(target_arch = "x86_64"))]
#[inline] pub unsafe fn xfd_set_state(_: u64) {}
#[cfg(not(target_arch = "x86_64"))]
#[inline] pub unsafe fn xfd_update_state(_: *mut fpstate) {}
#[cfg(not(target_arch = "x86_64"))]
#[inline] pub unsafe fn __xfd_enable_feature(_: u64, _: *mut fpu_guest) -> i32 { -EPERM }

#[inline]
pub unsafe fn os_xsave(fpstate: *mut fpstate) {
    let mask = (*fpstate).xfeatures; let lmask = mask as u32; let hmask = (mask >> 32) as u32; let mut err = 0;
    WARN_ON_FPU(alternatives_patched); xfd_validate_state(fpstate, mask, false);
    XSTATE_XSAVE!(&mut (*fpstate).regs.xsave, lmask, hmask, err); WARN_ON_FPU(err);
}

#[inline]
pub unsafe fn os_xrstor(fpstate: *mut fpstate, mask: u64) {
    let lmask = mask as u32; let hmask = (mask >> 32) as u32;
    xfd_validate_state(fpstate, mask, true); XSTATE_XRESTORE!(&mut (*fpstate).regs.xsave, lmask, hmask);
}

#[inline]
pub unsafe fn os_xrstor_supervisor(fpstate: *mut fpstate) {
    let mask = xfeatures_mask_supervisor(); XSTATE_XRESTORE!(&mut (*fpstate).regs.xsave, mask as u32, (mask >> 32) as u32);
}

#[inline]
pub unsafe fn xfeatures_need_sigframe_write() -> u64 {
    xfeatures_in_use() | (XFEATURE_MASK_USER_SUPPORTED & !XFEATURE_MASK_SIGFRAME_INITOPT)
}

#[inline]
pub unsafe fn xsave_to_user_sigframe(buf: *mut xregs_state, pkru: u32) -> i32 {
    let fpstate = (*x86_task_fpu((*current))).fpstate;
    let mut mask = (*fpstate).user_xfeatures;
    if fpu_state_size_dynamic() { mask &= xfeatures_need_sigframe_write(); }
    let mut err = 0; let lmask = mask as u32; let hmask = (mask >> 32) as u32;
    xfd_validate_state(fpstate, mask, false); stac();
    XSTATE_OP!("xsave", buf, lmask, hmask, err); clac();
    if err == 0 { err = update_pkru_in_sigframe(buf, pkru); } err
}

#[inline]
pub unsafe fn xrstor_from_user_sigframe(buf: *mut xregs_state, mask: u64) -> i32 {
    let xstate = buf as *mut xregs_state; let mut err = 0;
    xfd_validate_state((*x86_task_fpu((*current))).fpstate, mask, true); stac();
    XSTATE_OP!("xrstor", xstate, mask as u32, (mask >> 32) as u32, err); clac(); err
}

#[inline]
pub unsafe fn os_xrstor_safe(fpstate: *mut fpstate, mask: u64) -> i32 {
    let mut err = 0; xfd_update_state(fpstate);
    if cpu_feature_enabled(X86_FEATURE_XSAVES) {
        XSTATE_OP!("xrstors", &mut (*fpstate).regs.xsave, mask as u32, (mask >> 32) as u32, err);
    } else {
        XSTATE_OP!("xrstor", &mut (*fpstate).regs.xsave, mask as u32, (mask >> 32) as u32, err);
    } err
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
