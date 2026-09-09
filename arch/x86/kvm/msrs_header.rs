/* SPDX-License-Identifier: GPL-2.0 */

// Declarations supplied by linux/kvm_host.h, linux/user-return-notifier.h,
// cpuid.h, and regs.h remain external dependencies.

extern "C" {
    pub static mut report_ignored_msrs: bool;
    pub static mut ignore_msrs: bool;
    pub static mut kvm_nr_uret_msrs: u32;
}

#[inline]
pub unsafe fn kvm_pr_unimpl_wrmsr(vcpu: *mut kvm_vcpu, msr: u32, data: u64) {
    if report_ignored_msrs {
        vcpu_unimpl(vcpu, "Unhandled WRMSR(0x%x) = 0x%llx\n", msr, data);
    }
}

#[inline]
pub unsafe fn kvm_pr_unimpl_rdmsr(vcpu: *mut kvm_vcpu, msr: u32) {
    if report_ignored_msrs {
        vcpu_unimpl(vcpu, "Unhandled RDMSR(0x%x)\n", msr);
    }
}

// The first...last VMX feature MSRs that are emulated by KVM. This may or may
// not cover all known VMX MSRs, as KVM doesn't emulate an MSR until there's an
// associated feature that KVM supports for nested virtualization.
pub const KVM_FIRST_EMULATED_VMX_MSR: u32 = MSR_IA32_VMX_BASIC;
pub const KVM_LAST_EMULATED_VMX_MSR: u32 = MSR_IA32_VMX_VMFUNC;

// KVM's internal, non-ABI indices for synthetic MSRs. The values themselves
// are arbitrary and have no meaning, the only requirement is that they don't
// conflict with "real" MSRs that KVM supports.
pub const MSR_KVM_INTERNAL_GUEST_SSP: u32 = 0x4b564dff;

pub const MSR_IA32_CR_PAT_DEFAULT: u64 = PAT_VALUE!(WB, WT, UC_MINUS, UC, WB, WT, UC_MINUS, UC);

extern "C" {
    pub fn kvm_init_msr_lists();
    pub fn kvm_get_msr_index_list(user_msr_list: *mut kvm_msr_list) -> i32;
    pub fn kvm_get_feature_msr_index_list(user_msr_list: *mut kvm_msr_list) -> i32;
    pub fn kvm_get_feature_msrs(user_msrs: *mut kvm_msrs) -> i32;

    pub fn kvm_get_msrs(vcpu: *mut kvm_vcpu, user_msrs: *mut kvm_msrs) -> i32;
    pub fn kvm_set_msrs(vcpu: *mut kvm_vcpu, user_msrs: *mut kvm_msrs) -> i32;

    pub fn kvm_get_set_one_reg(vcpu: *mut kvm_vcpu, ioctl: u32, argp: *mut core::ffi::c_void) -> i32;
    pub fn kvm_get_reg_list(vcpu: *mut kvm_vcpu, user_list: *mut kvm_reg_list) -> i32;

    pub fn kvm_valid_efer(vcpu: *mut kvm_vcpu, efer: u64) -> bool;
    pub fn kvm_emulate_msr_read(vcpu: *mut kvm_vcpu, index: u32, data: *mut u64) -> i32;
    pub fn kvm_emulate_msr_write(vcpu: *mut kvm_vcpu, index: u32, data: u64) -> i32;
    pub fn __kvm_emulate_msr_read(vcpu: *mut kvm_vcpu, index: u32, data: *mut u64) -> i32;
    pub fn __kvm_emulate_msr_write(vcpu: *mut kvm_vcpu, index: u32, data: u64) -> i32;
    pub fn kvm_msr_read(vcpu: *mut kvm_vcpu, index: u32, data: *mut u64) -> i32;
    pub fn kvm_msr_write(vcpu: *mut kvm_vcpu, index: u32, data: u64) -> i32;
    pub fn kvm_emulate_rdmsr(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_emulate_rdmsr_imm(vcpu: *mut kvm_vcpu, msr: u32, reg: i32) -> i32;
    pub fn kvm_emulate_wrmsr(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_emulate_wrmsr_imm(vcpu: *mut kvm_vcpu, msr: u32, reg: i32) -> i32;

    pub fn handle_fastpath_wrmsr(vcpu: *mut kvm_vcpu) -> fastpath_t;
    pub fn handle_fastpath_wrmsr_imm(vcpu: *mut kvm_vcpu, msr: u32, reg: i32) -> fastpath_t;

    pub fn kvm_get_msr_common(vcpu: *mut kvm_vcpu, msr: *mut msr_data) -> i32;
    pub fn kvm_set_msr_common(vcpu: *mut kvm_vcpu, msr: *mut msr_data) -> i32;
    pub fn kvm_add_user_return_msr(msr: u32) -> i32;
    pub fn kvm_find_user_return_msr(msr: u32) -> i32;
    pub fn kvm_set_user_return_msr(index: u32, val: u64, mask: u64) -> i32;
    pub fn kvm_get_user_return_msr(slot: u32) -> u64;
}

#[inline]
pub unsafe fn kvm_is_supported_user_return_msr(msr: u32) -> bool {
    kvm_find_user_return_msr(msr) >= 0
}

extern "C" {
    pub fn kvm_user_return_msr_cpu_online();
    pub fn drop_user_return_notifiers();
    pub fn kvm_destroy_user_return_msrs();
    pub fn kvm_emulator_get_msr_with_filter(vcpu: *mut kvm_vcpu, msr_index: u32, pdata: *mut u64) -> i32;
    pub fn kvm_emulator_set_msr_with_filter(vcpu: *mut kvm_vcpu, msr_index: u32, data: u64) -> i32;
    pub fn kvm_emulator_get_msr(vcpu: *mut kvm_vcpu, msr_index: u32, pdata: *mut u64) -> i32;
    pub fn kvm_msr_allowed(vcpu: *mut kvm_vcpu, index: u32, type_: u32) -> bool;
}

pub const MSR_TYPE_R: u32 = BIT!(0);
pub const MSR_TYPE_W: u32 = BIT!(1);
pub const MSR_TYPE_RW: u32 = MSR_TYPE_R | MSR_TYPE_W;

pub const KVM_MSR_RET_UNSUPPORTED: i32 = 2;
pub const KVM_MSR_RET_FILTERED: i32 = 3;

extern "C" {
    pub fn kvm_vm_ioctl_set_msr_filter(kvm: *mut kvm, filter: *mut kvm_msr_filter) -> i32;
    pub fn kvm_free_msr_filter(msr_filter: *mut kvm_x86_msr_filter);
    pub fn kvm_mtrr_set_msr(vcpu: *mut kvm_vcpu, msr: u32, data: u64) -> i32;
    pub fn kvm_mtrr_get_msr(vcpu: *mut kvm_vcpu, msr: u32, pdata: *mut u64) -> i32;
    pub fn kvm_get_arch_capabilities() -> u64;
    pub fn kvm_spec_ctrl_test_value(value: u64) -> i32;
}

pub const CET_US_RESERVED_BITS: u64 = GENMASK!(9, 6);
pub const CET_US_SHSTK_MASK_BITS: u64 = GENMASK!(1, 0);
pub const CET_US_IBT_MASK_BITS: u64 = GENMASK_ULL!(5, 2) | GENMASK_ULL!(63, 10);

#[inline]
pub const fn CET_US_LEGACY_BITMAP_BASE(data: u64) -> u64 { data >> 12 }

#[inline]
pub unsafe fn kvm_is_valid_u_s_cet(vcpu: *mut kvm_vcpu, data: u64) -> bool {
    if data & CET_US_RESERVED_BITS != 0 { return false; }
    if !guest_cpu_cap_has(vcpu, X86_FEATURE_SHSTK) && data & CET_US_SHSTK_MASK_BITS != 0 { return false; }
    if !guest_cpu_cap_has(vcpu, X86_FEATURE_IBT) && data & CET_US_IBT_MASK_BITS != 0 { return false; }
    if !IS_ALIGNED(CET_US_LEGACY_BITMAP_BASE(data), 4) { return false; }
    if data & CET_SUPPRESS != 0 && data & CET_WAIT_ENDBR != 0 { return false; }
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
