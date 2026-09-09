// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2019 Arm Ltd.

// Kernel dependencies supplied by the surrounding translation unit.

const KVM_ARM_SMCCC_STD_FEATURES: usize = genmask(KVM_REG_ARM_STD_BMAP_BIT_COUNT - 1, 0);
const KVM_ARM_SMCCC_STD_HYP_FEATURES: usize = genmask(KVM_REG_ARM_STD_HYP_BMAP_BIT_COUNT - 1, 0);
const KVM_ARM_SMCCC_VENDOR_HYP_FEATURES: usize = genmask(KVM_REG_ARM_VENDOR_HYP_BMAP_BIT_COUNT - 1, 0);
const KVM_ARM_SMCCC_VENDOR_HYP_FEATURES_2: usize = genmask(KVM_REG_ARM_VENDOR_HYP_BMAP_2_BIT_COUNT - 1, 0);

unsafe fn kvm_ptp_get_time(vcpu: *mut kvm_vcpu, val: *mut u64) {
    let mut systime_snapshot: system_time_snapshot = core::mem::zeroed();
    let mut cycles: u64 = !0u64;
    ktime_get_snapshot_id(CLOCK_REALTIME, &mut systime_snapshot);
    if systime_snapshot.cs_id != CSID_ARM_ARCH_COUNTER { return; }
    match smccc_get_arg1(vcpu) {
        KVM_PTP_VIRT_COUNTER => cycles = systime_snapshot.cycles.wrapping_sub((*(*vcpu).kvm).arch.timer_data.voffset),
        KVM_PTP_PHYS_COUNTER => cycles = systime_snapshot.cycles.wrapping_sub((*(*vcpu).kvm).arch.timer_data.poffset),
        _ => return,
    }
    *val.add(0) = upper_32_bits(systime_snapshot.systime);
    *val.add(1) = lower_32_bits(systime_snapshot.systime);
    *val.add(2) = upper_32_bits(cycles);
    *val.add(3) = lower_32_bits(cycles);
}

unsafe fn kvm_smccc_default_allowed(func_id: u32) -> bool {
    match func_id {
        ARM_SMCCC_VERSION_FUNC_ID | ARM_SMCCC_ARCH_FEATURES_FUNC_ID => true,
        _ => {
            if ARM_SMCCC_OWNER_NUM(func_id) == ARM_SMCCC_OWNER_STANDARD && ARM_SMCCC_FUNC_NUM(func_id) <= 0x1f { return true; }
            if func_id >= KVM_PSCI_FN(0) && func_id <= KVM_PSCI_FN(3) { return true; }
            false
        }
    }
}

unsafe fn kvm_smccc_test_fw_bmap(vcpu: *mut kvm_vcpu, func_id: u32) -> bool {
    let f = &(*(*vcpu).kvm).arch.smccc_feat;
    match func_id {
        ARM_SMCCC_TRNG_VERSION | ARM_SMCCC_TRNG_FEATURES | ARM_SMCCC_TRNG_GET_UUID | ARM_SMCCC_TRNG_RND32 | ARM_SMCCC_TRNG_RND64 => test_bit(KVM_REG_ARM_STD_BIT_TRNG_V1_0, &f.std_bmap),
        ARM_SMCCC_HV_PV_TIME_FEATURES | ARM_SMCCC_HV_PV_TIME_ST => test_bit(KVM_REG_ARM_STD_HYP_BIT_PV_TIME, &f.std_hyp_bmap),
        ARM_SMCCC_VENDOR_HYP_KVM_FEATURES_FUNC_ID | ARM_SMCCC_VENDOR_HYP_CALL_UID_FUNC_ID => test_bit(KVM_REG_ARM_VENDOR_HYP_BIT_FUNC_FEAT, &f.vendor_hyp_bmap),
        ARM_SMCCC_VENDOR_HYP_KVM_PTP_FUNC_ID => test_bit(KVM_REG_ARM_VENDOR_HYP_BIT_PTP, &f.vendor_hyp_bmap),
        _ => false,
    }
}

const SMC32_ARCH_RANGE_BEGIN: u32 = ARM_SMCCC_VERSION_FUNC_ID;
const SMC32_ARCH_RANGE_END: u32 = ARM_SMCCC_CALL_VAL(ARM_SMCCC_FAST_CALL, ARM_SMCCC_SMC_32, 0, ARM_SMCCC_FUNC_MASK);
const SMC64_ARCH_RANGE_BEGIN: u32 = ARM_SMCCC_CALL_VAL(ARM_SMCCC_FAST_CALL, ARM_SMCCC_SMC_64, 0, 0);
const SMC64_ARCH_RANGE_END: u32 = ARM_SMCCC_CALL_VAL(ARM_SMCCC_FAST_CALL, ARM_SMCCC_SMC_64, 0, ARM_SMCCC_FUNC_MASK);

unsafe fn kvm_smccc_filter_insert_reserved(kvm: *mut kvm) -> i32 {
    let mut r = mtree_insert_range(&mut (*kvm).arch.smccc_filter, SMC32_ARCH_RANGE_BEGIN, SMC32_ARCH_RANGE_END, xa_mk_value(KVM_SMCCC_FILTER_HANDLE), GFP_KERNEL_ACCOUNT);
    if r != 0 { mtree_destroy(&mut (*kvm).arch.smccc_filter); return r; }
    r = mtree_insert_range(&mut (*kvm).arch.smccc_filter, SMC64_ARCH_RANGE_BEGIN, SMC64_ARCH_RANGE_END, xa_mk_value(KVM_SMCCC_FILTER_HANDLE), GFP_KERNEL_ACCOUNT);
    if r != 0 { mtree_destroy(&mut (*kvm).arch.smccc_filter); }
    r
}

unsafe fn kvm_smccc_filter_configured(kvm: *mut kvm) -> bool { !mtree_empty(&(*kvm).arch.smccc_filter) }

unsafe fn kvm_smccc_set_filter(kvm: *mut kvm, uaddr: *const core::ffi::c_void) -> i32 {
    let zero_page = page_to_virt(ZERO_PAGE(0));
    let mut filter: kvm_smccc_filter = core::mem::zeroed();
    if copy_from_user(&mut filter, uaddr, core::mem::size_of::<kvm_smccc_filter>()) != 0 { return -EFAULT; }
    if memcmp(filter.pad.as_ptr(), zero_page, filter.pad.len()) != 0 { return -EINVAL; }
    let start = filter.base; let end = start.wrapping_add(filter.nr_functions).wrapping_sub(1);
    if end < start || filter.action >= NR_SMCCC_FILTER_ACTIONS { return -EINVAL; }
    mutex_lock(&mut (*kvm).arch.config_lock);
    let r;
    if kvm_vm_has_ran_once(kvm) { r = -EBUSY; }
    else {
        if !kvm_smccc_filter_configured(kvm) {
            let x = kvm_smccc_filter_insert_reserved(kvm);
            if WARN_ON_ONCE(x != 0) { mutex_unlock(&mut (*kvm).arch.config_lock); return x; }
        }
        r = mtree_insert_range(&mut (*kvm).arch.smccc_filter, start, end, xa_mk_value(filter.action), GFP_KERNEL_ACCOUNT);
    }
    mutex_unlock(&mut (*kvm).arch.config_lock); r
}

unsafe fn kvm_smccc_filter_get_action(kvm: *mut kvm, func_id: u32) -> u8 {
    if !kvm_smccc_filter_configured(kvm) { return KVM_SMCCC_FILTER_HANDLE; }
    let mut idx = func_id as usize;
    xa_to_value(mt_find(&(*kvm).arch.smccc_filter, &mut idx, idx))
}

unsafe fn kvm_smccc_get_action(vcpu: *mut kvm_vcpu, func_id: u32) -> u8 {
    let action = kvm_smccc_filter_get_action((*vcpu).kvm, func_id);
    if action != KVM_SMCCC_FILTER_HANDLE { return action; }
    if kvm_smccc_test_fw_bmap(vcpu, func_id) || kvm_smccc_default_allowed(func_id) { KVM_SMCCC_FILTER_HANDLE } else { KVM_SMCCC_FILTER_DENY }
}

unsafe fn kvm_prepare_hypercall_exit(vcpu: *mut kvm_vcpu, func_id: u32) {
    let ec = ESR_ELx_EC(kvm_vcpu_get_esr(vcpu)); let mut flags = 0u64;
    if ec == ESR_ELx_EC_SMC32 || ec == ESR_ELx_EC_SMC64 { flags |= KVM_HYPERCALL_EXIT_SMC; }
    if !kvm_vcpu_trap_il_is32bit(vcpu) { flags |= KVM_HYPERCALL_EXIT_16BIT; }
    (*vcpu).run.exit_reason = KVM_EXIT_HYPERCALL;
    (*vcpu).run.hypercall = kvm_hypercall { nr: func_id, flags };
}

pub unsafe fn kvm_smccc_call_handler(vcpu: *mut kvm_vcpu) -> i32 {
    let feat = &(*(*vcpu).kvm).arch.smccc_feat; let func_id = smccc_get_function(vcpu); let mut val = [SMCCC_RET_NOT_SUPPORTED; 4];
    let action = kvm_smccc_get_action(vcpu, func_id);
    match action { KVM_SMCCC_FILTER_HANDLE => {}, KVM_SMCCC_FILTER_DENY => {}, KVM_SMCCC_FILTER_FWD_TO_USER => { kvm_prepare_hypercall_exit(vcpu, func_id); return 0; }, _ => { WARN_RATELIMIT(true, "Unhandled SMCCC filter action: %d\n", action); } }
    if action == KVM_SMCCC_FILTER_DENY { smccc_set_retval(vcpu, val[0], val[1], val[2], val[3]); return 1; }
    match func_id {
        ARM_SMCCC_VERSION_FUNC_ID => val[0] = ARM_SMCCC_VERSION_1_1,
        ARM_SMCCC_ARCH_FEATURES_FUNC_ID => { let feature = smccc_get_arg1(vcpu); match feature { ARM_SMCCC_ARCH_WORKAROUND_1 => match arm64_get_spectre_v2_state() { SPECTRE_MITIGATED => val[0]=SMCCC_RET_SUCCESS, SPECTRE_UNAFFECTED => val[0]=SMCCC_ARCH_WORKAROUND_RET_UNAFFECTED, _=>{} }, ARM_SMCCC_ARCH_WORKAROUND_2 => match arm64_get_spectre_v4_state() { SPECTRE_MITIGATED if !kvm_has_feat((*vcpu).kvm, ID_AA64PFR1_EL1, SSBS, IMP) | SPECTRE_UNAFFECTED => val[0]=SMCCC_RET_NOT_REQUIRED, _=>{} }, ARM_SMCCC_ARCH_WORKAROUND_3 => match arm64_get_spectre_bhb_state() { SPECTRE_MITIGATED=>val[0]=SMCCC_RET_SUCCESS, SPECTRE_UNAFFECTED=>val[0]=SMCCC_ARCH_WORKAROUND_RET_UNAFFECTED, _=>{} }, ARM_SMCCC_HV_PV_TIME_FEATURES if test_bit(KVM_REG_ARM_STD_HYP_BIT_PV_TIME, &feat.std_hyp_bmap) => val[0]=SMCCC_RET_SUCCESS, _=>{} } },
        ARM_SMCCC_HV_PV_TIME_FEATURES => val[0] = kvm_hypercall_pv_features(vcpu),
        ARM_SMCCC_HV_PV_TIME_ST => { let gpa=kvm_init_stolen_time(vcpu); if gpa != INVALID_GPA { val[0]=gpa; } },
        ARM_SMCCC_VENDOR_HYP_CALL_UID_FUNC_ID => { let uuid=ARM_SMCCC_VENDOR_HYP_UID_KVM; for i in 0..4 { val[i]=smccc_uuid_to_reg(&uuid,i); } },
        ARM_SMCCC_VENDOR_HYP_KVM_FEATURES_FUNC_ID => { val[0]=feat.vendor_hyp_bmap; val[2]=feat.vendor_hyp_bmap_2; },
        ARM_SMCCC_VENDOR_HYP_KVM_PTP_FUNC_ID => kvm_ptp_get_time(vcpu,val.as_mut_ptr()),
        ARM_SMCCC_TRNG_VERSION | ARM_SMCCC_TRNG_FEATURES | ARM_SMCCC_TRNG_GET_UUID | ARM_SMCCC_TRNG_RND32 | ARM_SMCCC_TRNG_RND64 => return kvm_trng_call(vcpu),
        _ => return kvm_psci_call(vcpu),
    }
    smccc_set_retval(vcpu,val[0],val[1],val[2],val[3]); 1
}

static KVM_ARM_FW_REG_IDS: [u64; 8] = [KVM_REG_ARM_PSCI_VERSION,KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_1,KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_2,KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_3,KVM_REG_ARM_STD_BMAP,KVM_REG_ARM_STD_HYP_BMAP,KVM_REG_ARM_VENDOR_HYP_BMAP,KVM_REG_ARM_VENDOR_HYP_BMAP_2];

pub unsafe fn kvm_arm_init_hypercalls(kvm:*mut kvm) { let f=&mut (*kvm).arch.smccc_feat; f.std_bmap=KVM_ARM_SMCCC_STD_FEATURES as _; f.std_hyp_bmap=KVM_ARM_SMCCC_STD_HYP_FEATURES as _; f.vendor_hyp_bmap=KVM_ARM_SMCCC_VENDOR_HYP_FEATURES as _; mt_init(&mut (*kvm).arch.smccc_filter); }
pub unsafe fn kvm_arm_teardown_hypercalls(kvm:*mut kvm) { mtree_destroy(&mut (*kvm).arch.smccc_filter); }
pub unsafe fn kvm_arm_get_fw_num_regs(_vcpu:*mut kvm_vcpu)->i32 { KVM_ARM_FW_REG_IDS.len() as i32 }
pub unsafe fn kvm_arm_copy_fw_reg_indices(_vcpu:*mut kvm_vcpu, mut p:*mut u64)->i32 { for x in KVM_ARM_FW_REG_IDS { if put_user(x,p)!=0{return -EFAULT;} p=p.add(1); } 0 }

const KVM_REG_FEATURE_LEVEL_MASK:u64=genmask(3,0);
unsafe fn get_kernel_wa_level(vcpu:*mut kvm_vcpu, regid:u64)->i32 { match regid { KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_1=>match arm64_get_spectre_v2_state(){SPECTRE_VULNERABLE=>KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_1_NOT_AVAIL,SPECTRE_MITIGATED=>KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_1_AVAIL,SPECTRE_UNAFFECTED=>KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_1_NOT_REQUIRED}, KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_2=>match arm64_get_spectre_v4_state(){SPECTRE_MITIGATED if kvm_has_feat((*vcpu).kvm,ID_AA64PFR1_EL1,SSBS,IMP)=>KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_2_NOT_AVAIL,SPECTRE_MITIGATED|SPECTRE_UNAFFECTED=>KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_2_NOT_REQUIRED,_=>KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_2_NOT_AVAIL}, KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_3=>match arm64_get_spectre_bhb_state(){SPECTRE_VULNERABLE=>KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_3_NOT_AVAIL,SPECTRE_MITIGATED=>KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_3_AVAIL,SPECTRE_UNAFFECTED=>KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_3_NOT_REQUIRED}, _=>-EINVAL } }

// Firmware-register accessors and attribute handlers retain the kernel ABI and are declared below.
pub unsafe fn kvm_arm_get_fw_reg(vcpu:*mut kvm_vcpu, reg:*const kvm_one_reg)->i32 { let mut val=match (*reg).id { KVM_REG_ARM_PSCI_VERSION=>kvm_psci_version(vcpu), KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_1|KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_2|KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_3=>(get_kernel_wa_level(vcpu,(*reg).id) as u64)&KVM_REG_FEATURE_LEVEL_MASK, KVM_REG_ARM_STD_BMAP=>READ_ONCE((*vcpu).kvm.arch.smccc_feat.std_bmap), KVM_REG_ARM_STD_HYP_BMAP=>READ_ONCE((*vcpu).kvm.arch.smccc_feat.std_hyp_bmap), KVM_REG_ARM_VENDOR_HYP_BMAP=>READ_ONCE((*vcpu).kvm.arch.smccc_feat.vendor_hyp_bmap), KVM_REG_ARM_VENDOR_HYP_BMAP_2=>READ_ONCE((*vcpu).kvm.arch.smccc_feat.vendor_hyp_bmap_2), _=>return -ENOENT}; if copy_to_user((*reg).addr as *mut _,&mut val,KVM_REG_SIZE((*reg).id))!=0{-EFAULT}else{0} }
pub unsafe fn kvm_arm_set_fw_reg(vcpu:*mut kvm_vcpu,reg:*const kvm_one_reg)->i32 {
    let mut val=0u64; if KVM_REG_SIZE((*reg).id)!=core::mem::size_of::<u64>() {return -ENOENT;} if copy_from_user(&mut val,(*reg).addr as *const _,KVM_REG_SIZE((*reg).id))!=0{return -EFAULT;}
    match (*reg).id {
        KVM_REG_ARM_PSCI_VERSION=>{let wants=vcpu_has_feature(vcpu,KVM_ARM_VCPU_PSCI_0_2); match val {KVM_ARM_PSCI_0_1 if wants=>-EINVAL,KVM_ARM_PSCI_0_1=>{(*vcpu).kvm.arch.psci_version=val;0},KVM_ARM_PSCI_0_2|KVM_ARM_PSCI_1_0|KVM_ARM_PSCI_1_1|KVM_ARM_PSCI_1_2|KVM_ARM_PSCI_1_3 if !wants=>-EINVAL,KVM_ARM_PSCI_0_2|KVM_ARM_PSCI_1_0|KVM_ARM_PSCI_1_1|KVM_ARM_PSCI_1_2|KVM_ARM_PSCI_1_3=>{(*vcpu).kvm.arch.psci_version=val;0},_=>-EINVAL}},
        KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_1|KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_3=>if val&!KVM_REG_FEATURE_LEVEL_MASK!=0||get_kernel_wa_level(vcpu,(*reg).id)<val as i32{-EINVAL}else{0},
        KVM_REG_ARM_STD_BMAP|KVM_REG_ARM_STD_HYP_BMAP|KVM_REG_ARM_VENDOR_HYP_BMAP|KVM_REG_ARM_VENDOR_HYP_BMAP_2=>-ENOENT,
        _=>-ENOENT,
    }
}
pub unsafe fn kvm_vm_smccc_has_attr(_kvm:*mut kvm,attr:*const kvm_device_attr)->i32 { if (*attr).attr==KVM_ARM_VM_SMCCC_FILTER {0}else{-ENXIO} }
pub unsafe fn kvm_vm_smccc_set_attr(kvm:*mut kvm,attr:*const kvm_device_attr)->i32 { if (*attr).attr==KVM_ARM_VM_SMCCC_FILTER {kvm_smccc_set_filter(kvm,(*attr).addr as *const _)}else{-ENXIO} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
