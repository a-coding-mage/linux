// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012,2013 - ARM Ltd
 * Author: Marc Zyngier <marc.zyngier@arm.com>
 *
 * Derived from arch/arm/kvm/reset.c
 * Copyright (C) 2012 - Virtual Open Systems and Columbia University
 * Author: Christoffer Dall <c.dall@virtualopensystems.com>
 */

// Kernel and architecture dependencies are supplied by the surrounding build.

static mut KVM_IPA_LIMIT: u32 = 0;
static mut KVM_HOST_SVE_MAX_VL: u32 = 0;
static mut KVM_SVE_MAX_VL: u32 = 0;

pub unsafe fn kvm_arm_init_sve() -> i32 {
    if system_supports_sve() {
        KVM_SVE_MAX_VL = sve_max_virtualisable_vl();
        KVM_HOST_SVE_MAX_VL = sve_max_vl();
        kvm_nvhe_sym(KVM_HOST_SVE_MAX_VL);

        /* The ioctl interface currently supports only one register slice. */
        if WARN_ON(KVM_SVE_MAX_VL > VL_ARCH_MAX) {
            KVM_SVE_MAX_VL = VL_ARCH_MAX;
        }

        /* Do not use vector lengths unavailable on all CPUs. */
        if KVM_SVE_MAX_VL < sve_max_vl() {
            pr_warn("KVM: SVE vector length for guests limited to %u bytes\n", KVM_SVE_MAX_VL);
        }
    }
    0
}

unsafe fn kvm_vcpu_enable_sve(vcpu: *mut kvm_vcpu) {
    (*vcpu).arch.sve_max_vl = KVM_SVE_MAX_VL;
    set_bit(KVM_ARCH_FLAG_GUEST_HAS_SVE, &mut (*(*vcpu).kvm).arch.flags);
}

unsafe fn kvm_vcpu_finalize_sve(vcpu: *mut kvm_vcpu) -> i32 {
    let vl = (*vcpu).arch.sve_max_vl;
    if WARN_ON(!sve_vl_valid(vl) || vl > sve_max_virtualisable_vl() || vl > VL_ARCH_MAX) {
        return -EIO;
    }

    let reg_sz = vcpu_sve_state_size(vcpu);
    let buf = kzalloc(reg_sz, GFP_KERNEL_ACCOUNT);
    if buf.is_null() {
        return -ENOMEM;
    }

    let ret = kvm_share_hyp(buf, (buf as *mut u8).add(reg_sz) as *mut _);
    if ret != 0 {
        kfree(buf);
        return ret;
    }

    (*vcpu).arch.sve_state = buf;
    vcpu_set_flag(vcpu, VCPU_SVE_FINALIZED);
    0
}

pub unsafe fn kvm_arm_vcpu_finalize(vcpu: *mut kvm_vcpu, feature: i32) -> i32 {
    match feature {
        KVM_ARM_VCPU_SVE => {
            if !vcpu_has_sve(vcpu) { return -EINVAL; }
            if kvm_arm_vcpu_sve_finalized(vcpu) { return -EPERM; }
            kvm_vcpu_finalize_sve(vcpu)
        }
        _ => -EINVAL,
    }
}

pub unsafe fn kvm_arm_vcpu_is_finalized(vcpu: *mut kvm_vcpu) -> bool {
    !(vcpu_has_sve(vcpu) && !kvm_arm_vcpu_sve_finalized(vcpu))
}

pub unsafe fn kvm_arm_vcpu_destroy(vcpu: *mut kvm_vcpu) {
    let sve_state = (*vcpu).arch.sve_state;
    kvm_unshare_hyp(vcpu as *mut _, vcpu.add(1) as *mut _);
    if !sve_state.is_null() {
        kvm_unshare_hyp(sve_state, (sve_state as *mut u8).add(vcpu_sve_state_size(vcpu)) as *mut _);
    }
    kfree(sve_state);
    free_page((*vcpu).arch.ctxt.vncr_array as usize);
    kfree((*vcpu).arch.vncr_tlb);
    kfree((*vcpu).arch.ccsidr);
}

unsafe fn kvm_vcpu_reset_sve(vcpu: *mut kvm_vcpu) {
    if vcpu_has_sve(vcpu) {
        memset((*vcpu).arch.sve_state, 0, vcpu_sve_state_size(vcpu));
    }
}

pub unsafe fn kvm_reset_vcpu(vcpu: *mut kvm_vcpu) {
    let reset_state;
    spin_lock(&mut (*vcpu).arch.mp_state_lock);
    reset_state = (*vcpu).arch.reset_state;
    (*vcpu).arch.reset_state.reset = false;
    spin_unlock(&mut (*vcpu).arch.mp_state_lock);

    preempt_disable();
    let loaded = (*vcpu).cpu != -1;
    if loaded { kvm_arch_vcpu_put(vcpu); }
    if !kvm_arm_vcpu_sve_finalized(vcpu) {
        if vcpu_has_feature(vcpu, KVM_ARM_VCPU_SVE) { kvm_vcpu_enable_sve(vcpu); }
    } else {
        kvm_vcpu_reset_sve(vcpu);
    }
    kvm_reset_vcpu_core(vcpu);
    kvm_reset_sys_regs(vcpu);
    if reset_state.reset { kvm_reset_vcpu_psci(vcpu, &reset_state); }
    kvm_timer_vcpu_reset(vcpu);
    if loaded { kvm_arch_vcpu_load(vcpu, smp_processor_id()); }
    preempt_enable();
}

pub unsafe fn kvm_get_pa_bits(_kvm: *mut kvm) -> u32 { KVM_IPA_LIMIT }
pub unsafe fn get_kvm_ipa_limit() -> u32 { KVM_IPA_LIMIT }

pub unsafe fn kvm_set_ipa_limit() -> i32 {
    let mmfr0 = read_sanitised_ftr_reg(SYS_ID_AA64MMFR0_EL1);
    let mut parange = cpuid_feature_extract_unsigned_field(mmfr0, ID_AA64MMFR0_EL1_PARANGE_SHIFT);
    if !kvm_lpa2_is_enabled() && PAGE_SIZE != SZ_64K {
        parange = min(parange, ID_AA64MMFR0_EL1_PARANGE_48 as u32);
    }
    match cpuid_feature_extract_unsigned_field(mmfr0, ID_AA64MMFR0_EL1_TGRAN_2_SHIFT) {
        ID_AA64MMFR0_EL1_TGRAN_2_SUPPORTED_NONE => { kvm_err!("PAGE_SIZE not supported at Stage-2, giving up\n"); return -EINVAL; }
        ID_AA64MMFR0_EL1_TGRAN_2_SUPPORTED_DEFAULT => kvm_debug!("PAGE_SIZE supported at Stage-2 (default)\n"),
        x if x >= ID_AA64MMFR0_EL1_TGRAN_2_SUPPORTED_MIN && x <= ID_AA64MMFR0_EL1_TGRAN_2_SUPPORTED_MAX => kvm_debug!("PAGE_SIZE supported at Stage-2 (advertised)\n"),
        _ => { kvm_err!("Unsupported value for TGRAN_2, giving up\n"); return -EINVAL; }
    }
    KVM_IPA_LIMIT = id_aa64mmfr0_parange_to_phys_shift(parange);
    kvm_info!("IPA Size Limit: %d bits%s\n", KVM_IPA_LIMIT,
        if KVM_IPA_LIMIT < KVM_PHYS_SHIFT { " (Reduced IPA size, limited VM/VMM compatibility)" } else { "" });
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
