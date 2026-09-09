/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by <uapi/asm/kvm_para.h> is intentionally not included here.

/* Hypercall code field */
pub const HYPERVISOR_KVM: usize = 1;
pub const HYPERVISOR_VENDOR_SHIFT: usize = 8;
pub const HYPERCALL_ENCODE: usize = |vendor: usize, code: usize| {
    (vendor << HYPERVISOR_VENDOR_SHIFT) + code
};

pub const KVM_HCALL_CODE_SERVICE: usize = 0;
pub const KVM_HCALL_CODE_SWDBG: usize = 1;
pub const KVM_HCALL_CODE_USER_SERVICE: usize = 2;

pub const KVM_HCALL_SERVICE: usize = HYPERCALL_ENCODE(HYPERVISOR_KVM, KVM_HCALL_CODE_SERVICE);
pub const KVM_HCALL_FUNC_IPI: usize = 1;
pub const KVM_HCALL_FUNC_NOTIFY: usize = 2;
pub const KVM_HCALL_SWDBG: usize = HYPERCALL_ENCODE(HYPERVISOR_KVM, KVM_HCALL_CODE_SWDBG);
pub const KVM_HCALL_USER_SERVICE: usize =
    HYPERCALL_ENCODE(HYPERVISOR_KVM, KVM_HCALL_CODE_USER_SERVICE);

/* LoongArch hypercall return code */
pub const KVM_HCALL_SUCCESS: usize = 0;
pub const KVM_HCALL_INVALID_CODE: usize = usize::MAX;
pub const KVM_HCALL_INVALID_PARAMETER: usize = usize::MAX - 1;

pub const KVM_STEAL_PHYS_VALID: u64 = 1u64 << 0;
pub const KVM_STEAL_PHYS_MASK: u64 = ((1u64 << 64) - 1) & !((1u64 << 6) - 1);

#[repr(C)]
pub struct kvm_steal_time {
    pub steal: u64,
    pub version: u32,
    pub flags: u32,
    pub preempted: u8,
    pub pad: [u8; 47],
}

pub const KVM_VCPU_PREEMPTED: u32 = 1 << 0;

/*
 * Hypercall interface for KVM hypervisor
 *
 * a0: function identifier
 * a1-a5: args
 * Return value will be placed in a0.
 * Up to 5 arguments are passed in a1, a2, a3, a4, a5.
 */
#[inline(always)]
pub unsafe fn kvm_hypercall0(fid: u64) -> isize {
    let mut ret = fid as isize;
    core::arch::asm!("hvcl {service}", service = const KVM_HCALL_SERVICE,
        inlateout("a0") ret, options(nostack));
    ret
}

#[inline(always)]
pub unsafe fn kvm_hypercall1(fid: u64, arg0: usize) -> isize {
    let mut ret = fid as isize;
    core::arch::asm!("hvcl {service}", service = const KVM_HCALL_SERVICE,
        inlateout("a0") ret, in("a1") arg0, options(nostack));
    ret
}

#[inline(always)]
pub unsafe fn kvm_hypercall2(fid: u64, arg0: usize, arg1: usize) -> isize {
    let mut ret = fid as isize;
    core::arch::asm!("hvcl {service}", service = const KVM_HCALL_SERVICE,
        inlateout("a0") ret, in("a1") arg0, in("a2") arg1, options(nostack));
    ret
}

#[inline(always)]
pub unsafe fn kvm_hypercall3(fid: u64, arg0: usize, arg1: usize, arg2: usize) -> isize {
    let mut ret = fid as isize;
    core::arch::asm!("hvcl {service}", service = const KVM_HCALL_SERVICE,
        inlateout("a0") ret, in("a1") arg0, in("a2") arg1, in("a3") arg2, options(nostack));
    ret
}

#[inline(always)]
pub unsafe fn kvm_hypercall4(fid: u64, arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> isize {
    let mut ret = fid as isize;
    core::arch::asm!("hvcl {service}", service = const KVM_HCALL_SERVICE,
        inlateout("a0") ret, in("a1") arg0, in("a2") arg1, in("a3") arg2, in("a4") arg3,
        options(nostack));
    ret
}

#[inline(always)]
pub unsafe fn kvm_hypercall5(
    fid: u64, arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize,
) -> isize {
    let mut ret = fid as isize;
    core::arch::asm!("hvcl {service}", service = const KVM_HCALL_SERVICE,
        inlateout("a0") ret, in("a1") arg0, in("a2") arg1, in("a3") arg2, in("a4") arg3,
        in("a5") arg4, options(nostack));
    ret
}

// CONFIG_PARAVIRT selects the external declarations in the C header.
#[cfg(feature = "CONFIG_PARAVIRT")]
extern "C" {
    pub fn kvm_para_available() -> bool;
    pub fn kvm_arch_para_features() -> u32;
}

#[cfg(not(feature = "CONFIG_PARAVIRT"))]
#[inline]
pub fn kvm_para_available() -> bool { false }

#[cfg(not(feature = "CONFIG_PARAVIRT"))]
#[inline]
pub fn kvm_arch_para_features() -> u32 { 0 }

#[inline]
pub fn kvm_arch_para_hints() -> u32 { 0 }

#[inline]
pub fn kvm_check_and_clear_guest_paused() -> bool { false }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
