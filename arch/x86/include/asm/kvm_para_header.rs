/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation. */

#[cfg(feature = "CONFIG_KVM_GUEST")]
unsafe extern "C" {
    pub fn kvm_check_and_clear_guest_paused() -> bool;
}

#[cfg(not(feature = "CONFIG_KVM_GUEST"))]
#[inline]
pub fn kvm_check_and_clear_guest_paused() -> bool {
    false
}

/* KVM_HYPERCALL expands to the architecture-specific alternative vmcall/vmmcall sequence. */

unsafe extern "C" {
    fn cpu_feature_enabled(feature: u32) -> bool;
    fn tdx_kvm_hypercall(
        nr: u32,
        p1: usize,
        p2: usize,
        p3: usize,
        p4: usize,
    ) -> isize;
}

#[inline]
pub unsafe fn kvm_hypercall0(nr: u32) -> isize {
    if cpu_feature_enabled(X86_FEATURE_TDX_GUEST) {
        return tdx_kvm_hypercall(nr, 0, 0, 0, 0);
    }
    let ret: isize;
    core::arch::asm!("vmcall", inout("rax") nr as isize => ret, options(nostack));
    ret
}

#[inline]
pub unsafe fn kvm_hypercall1(nr: u32, p1: usize) -> isize {
    if cpu_feature_enabled(X86_FEATURE_TDX_GUEST) {
        return tdx_kvm_hypercall(nr, p1, 0, 0, 0);
    }
    let ret: isize;
    core::arch::asm!("vmcall", inout("rax") nr as isize => ret, in("rbx") p1, options(nostack));
    ret
}

#[inline]
pub unsafe fn kvm_hypercall2(nr: u32, p1: usize, p2: usize) -> isize {
    if cpu_feature_enabled(X86_FEATURE_TDX_GUEST) {
        return tdx_kvm_hypercall(nr, p1, p2, 0, 0);
    }
    let ret: isize;
    core::arch::asm!("vmcall", inout("rax") nr as isize => ret, in("rbx") p1, in("rcx") p2, options(nostack));
    ret
}

#[inline]
pub unsafe fn kvm_hypercall3(nr: u32, p1: usize, p2: usize, p3: usize) -> isize {
    if cpu_feature_enabled(X86_FEATURE_TDX_GUEST) {
        return tdx_kvm_hypercall(nr, p1, p2, p3, 0);
    }
    let ret: isize;
    core::arch::asm!("vmcall", inout("rax") nr as isize => ret, in("rbx") p1, in("rcx") p2, in("rdx") p3, options(nostack));
    ret
}

#[inline]
pub unsafe fn kvm_hypercall4(nr: u32, p1: usize, p2: usize, p3: usize, p4: usize) -> isize {
    if cpu_feature_enabled(X86_FEATURE_TDX_GUEST) {
        return tdx_kvm_hypercall(nr, p1, p2, p3, p4);
    }
    let ret: isize;
    core::arch::asm!("vmcall", inout("rax") nr as isize => ret, in("rbx") p1, in("rcx") p2, in("rdx") p3, in("rsi") p4, options(nostack));
    ret
}

#[inline]
pub unsafe fn kvm_sev_hypercall3(nr: u32, p1: usize, p2: usize, p3: usize) -> isize {
    let ret: isize;
    core::arch::asm!("vmmcall", inout("rax") nr as isize => ret, in("rbx") p1, in("rcx") p2, in("rdx") p3, options(nostack));
    ret
}

#[cfg(feature = "CONFIG_KVM_GUEST")]
unsafe extern "C" {
    pub fn kvmclock_init();
    pub fn kvmclock_disable();
    pub fn kvm_para_available() -> bool;
    pub fn kvm_arch_para_features() -> u32;
    pub fn kvm_arch_para_hints() -> u32;
    pub fn kvm_async_pf_task_wait_schedule(token: u32);
    pub fn kvm_read_and_reset_apf_flags() -> u32;
    pub fn __kvm_handle_async_pf(regs: *mut crate::pt_regs, token: u32) -> bool;
    pub static kvm_async_pf_enabled: bool;
    pub fn static_branch_unlikely(key: *const bool) -> bool;
}

#[cfg(feature = "CONFIG_KVM_GUEST")]
#[inline(always)]
pub unsafe fn kvm_handle_async_pf(regs: *mut crate::pt_regs, token: u32) -> bool {
    if static_branch_unlikely(&kvm_async_pf_enabled) {
        __kvm_handle_async_pf(regs, token)
    } else {
        false
    }
}

#[cfg(all(feature = "CONFIG_KVM_GUEST", feature = "CONFIG_PARAVIRT_SPINLOCKS"))]
unsafe extern "C" {
    pub fn kvm_spinlock_init();
}

#[cfg(all(feature = "CONFIG_KVM_GUEST", not(feature = "CONFIG_PARAVIRT_SPINLOCKS")))]
#[inline]
pub fn kvm_spinlock_init() {}

#[cfg(not(feature = "CONFIG_KVM_GUEST"))]
#[inline]
pub fn kvm_async_pf_task_wait_schedule(_token: u32) {}

#[cfg(not(feature = "CONFIG_KVM_GUEST"))]
#[inline]
pub fn kvm_para_available() -> bool { false }

#[cfg(not(feature = "CONFIG_KVM_GUEST"))]
#[inline]
pub fn kvm_arch_para_features() -> u32 { 0 }

#[cfg(not(feature = "CONFIG_KVM_GUEST"))]
#[inline]
pub fn kvm_arch_para_hints() -> u32 { 0 }

#[cfg(not(feature = "CONFIG_KVM_GUEST"))]
#[inline]
pub fn kvm_read_and_reset_apf_flags() -> u32 { 0 }

#[cfg(not(feature = "CONFIG_KVM_GUEST"))]
#[inline(always)]
pub fn kvm_handle_async_pf(_regs: *mut crate::pt_regs, _token: u32) -> bool { false }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
