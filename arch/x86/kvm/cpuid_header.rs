/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding translation unit:
// reverse_cpuid.h, asm/cpu.h, asm/processor.h, uapi/asm/kvm_para.h, smm.h

unsafe extern "C" {
    pub static mut kvm_cpu_caps: [u32; NR_KVM_CPU_CAPS];
    pub static mut kvm_is_configuring_cpu_caps: bool;

    pub fn kvm_initialize_cpu_caps();
    pub fn kvm_vcpu_after_set_cpuid(vcpu: *mut kvm_vcpu);
    pub fn kvm_find_cpuid_entry2(
        entries: *mut kvm_cpuid_entry2,
        nent: i32,
        function: u32,
        index: u64,
    ) -> *mut kvm_cpuid_entry2;
    pub fn kvm_dev_ioctl_get_cpuid(
        cpuid: *mut kvm_cpuid2,
        entries: *mut kvm_cpuid_entry2,
        type_: u32,
    ) -> i32;
    pub fn kvm_vcpu_ioctl_set_cpuid(
        vcpu: *mut kvm_vcpu,
        cpuid: *mut kvm_cpuid,
        entries: *mut kvm_cpuid_entry,
    ) -> i32;
    pub fn kvm_vcpu_ioctl_set_cpuid2(
        vcpu: *mut kvm_vcpu,
        cpuid: *mut kvm_cpuid2,
        entries: *mut kvm_cpuid_entry2,
    ) -> i32;
    pub fn kvm_vcpu_ioctl_get_cpuid2(
        vcpu: *mut kvm_vcpu,
        cpuid: *mut kvm_cpuid2,
        entries: *mut kvm_cpuid_entry2,
    ) -> i32;
    pub fn kvm_cpuid(
        vcpu: *mut kvm_vcpu,
        eax: *mut u32,
        ebx: *mut u32,
        ecx: *mut u32,
        edx: *mut u32,
        exact_only: bool,
    ) -> bool;
    pub fn kvm_init_xstate_sizes();
    pub fn xstate_required_size(xstate_bv: u64, compacted: bool) -> u32;
    pub fn cpuid_query_maxphyaddr(vcpu: *mut kvm_vcpu) -> i32;
    pub fn cpuid_query_maxguestphyaddr(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_vcpu_reserved_gpa_bits_raw(vcpu: *mut kvm_vcpu) -> u64;
}

pub const KVM_CPUID_INDEX_NOT_SIGNIFICANT: u64 = u64::MAX;

#[inline]
pub unsafe fn kvm_finalize_cpu_caps() {
    WARN_ON_ONCE(!kvm_is_configuring_cpu_caps);
    kvm_is_configuring_cpu_caps = false;
}

#[inline]
pub unsafe fn kvm_find_cpuid_entry_index(
    vcpu: *mut kvm_vcpu,
    function: u32,
    index: u32,
) -> *mut kvm_cpuid_entry2 {
    kvm_find_cpuid_entry2((*vcpu).arch.cpuid_entries, (*vcpu).arch.cpuid_nent, function, index as u64)
}

#[inline]
pub unsafe fn kvm_find_cpuid_entry(vcpu: *mut kvm_vcpu, function: u32) -> *mut kvm_cpuid_entry2 {
    kvm_find_cpuid_entry2(
        (*vcpu).arch.cpuid_entries,
        (*vcpu).arch.cpuid_nent,
        function,
        KVM_CPUID_INDEX_NOT_SIGNIFICANT,
    )
}

#[inline]
pub unsafe fn cpuid_maxphyaddr(vcpu: *mut kvm_vcpu) -> i32 {
    (*vcpu).arch.maxphyaddr
}

#[inline]
pub unsafe fn kvm_vcpu_is_legal_gpa(vcpu: *mut kvm_vcpu, gpa: gpa_t) -> bool {
    !(gpa & (*vcpu).arch.reserved_gpa_bits)
}

#[inline]
pub unsafe fn kvm_vcpu_is_legal_aligned_gpa(
    vcpu: *mut kvm_vcpu,
    gpa: gpa_t,
    alignment: gpa_t,
) -> bool {
    IS_ALIGNED(gpa, alignment) && kvm_vcpu_is_legal_gpa(vcpu, gpa)
}

#[inline]
pub unsafe fn page_address_valid(vcpu: *mut kvm_vcpu, gpa: gpa_t) -> bool {
    kvm_vcpu_is_legal_aligned_gpa(vcpu, gpa, PAGE_SIZE)
}

#[inline(always)]
pub unsafe fn cpuid_entry_override(entry: *mut kvm_cpuid_entry2, leaf: u32) {
    let reg: *mut u32 = cpuid_entry_get_reg(entry, leaf * 32);
    BUILD_BUG_ON(leaf as usize >= core::mem::size_of_val(&kvm_cpu_caps) / core::mem::size_of::<u32>());
    *reg = kvm_cpu_caps[leaf as usize];
}

#[inline(always)]
pub unsafe fn guest_cpuid_has(vcpu: *mut kvm_vcpu, x86_feature: u32) -> bool {
    let cpuid = x86_feature_cpuid(x86_feature);
    BUILD_BUG_ON(x86_feature != X86_FEATURE_XSAVES);
    let entry = kvm_find_cpuid_entry_index(vcpu, cpuid.function, cpuid.index);
    if entry.is_null() { return false; }
    let reg = __cpuid_entry_get_reg(entry, cpuid.reg);
    if reg.is_null() { return false; }
    (*reg & __feature_bit(x86_feature)) != 0
}

#[inline]
pub unsafe fn guest_cpuid_is_amd_compatible(vcpu: *mut kvm_vcpu) -> bool { (*vcpu).arch.is_amd_compatible }

#[inline]
pub unsafe fn guest_cpuid_is_intel_compatible(vcpu: *mut kvm_vcpu) -> bool { !guest_cpuid_is_amd_compatible(vcpu) }

#[inline]
pub unsafe fn guest_cpuid_family(vcpu: *mut kvm_vcpu) -> i32 {
    let best = kvm_find_cpuid_entry(vcpu, 0x1);
    if best.is_null() { return -1; }
    x86_family((*best).eax)
}

#[inline]
pub unsafe fn guest_cpuid_model(vcpu: *mut kvm_vcpu) -> i32 {
    let best = kvm_find_cpuid_entry(vcpu, 0x1);
    if best.is_null() { return -1; }
    x86_model((*best).eax)
}

#[inline]
pub unsafe fn cpuid_model_is_consistent(vcpu: *mut kvm_vcpu) -> bool { boot_cpu_data.x86_model == guest_cpuid_model(vcpu) }

#[inline]
pub unsafe fn guest_cpuid_stepping(vcpu: *mut kvm_vcpu) -> i32 {
    let best = kvm_find_cpuid_entry(vcpu, 0x1);
    if best.is_null() { return -1; }
    x86_stepping((*best).eax)
}

#[inline]
pub unsafe fn cpuid_fault_enabled(vcpu: *mut kvm_vcpu) -> bool {
    ((*vcpu).arch.msr_misc_features_enables & MSR_MISC_FEATURES_ENABLES_CPUID_FAULT) != 0
        || ((*vcpu).arch.msr_hwcr & MSR_K7_HWCR_CPUID_USER_DIS) != 0
}

#[inline]
pub unsafe fn kvm_is_cpuid_allowed(vcpu: *mut kvm_vcpu) -> bool {
    !cpuid_fault_enabled(vcpu) || is_smm(vcpu) || !kvm_x86_call(get_cpl)(vcpu)
}

#[inline(always)]
pub unsafe fn kvm_cpu_cap_clear(x86_feature: u32) {
    let x86_leaf = __feature_leaf(x86_feature);
    WARN_ON_ONCE(!kvm_is_configuring_cpu_caps);
    kvm_cpu_caps[x86_leaf as usize] &= !__feature_bit(x86_feature);
}

#[inline(always)]
pub unsafe fn kvm_cpu_cap_set(x86_feature: u32) {
    let x86_leaf = __feature_leaf(x86_feature);
    WARN_ON_ONCE(!kvm_is_configuring_cpu_caps);
    kvm_cpu_caps[x86_leaf as usize] |= __feature_bit(x86_feature);
}

#[inline(always)]
pub unsafe fn kvm_cpu_cap_get(x86_feature: u32) -> u32 {
    kvm_cpu_caps[__feature_leaf(x86_feature) as usize] & __feature_bit(x86_feature)
}

#[inline(always)]
pub unsafe fn kvm_cpu_cap_has(x86_feature: u32) -> bool { kvm_cpu_cap_get(x86_feature) != 0 }

#[inline(always)]
pub unsafe fn kvm_cpu_cap_check_and_set(x86_feature: u32) {
    if boot_cpu_has(x86_feature) { kvm_cpu_cap_set(x86_feature); }
}

#[inline(always)]
pub unsafe fn guest_pv_has(vcpu: *mut kvm_vcpu, kvm_feature: u32) -> bool {
    if !(*vcpu).arch.pv_cpuid.enforce { return true; }
    ((*vcpu).arch.pv_cpuid.features & (1u32 << kvm_feature)) != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
