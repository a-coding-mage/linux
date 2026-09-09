// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2025 Rivos Inc.
 *
 * Authors:
 *     Clément Léger <cleger@rivosinc.com>
 */

// Linux and architecture dependencies supplied by the surrounding kernel translation.

const MIS_DELEG: u64 = (1u64 << EXC_LOAD_MISALIGNED) | (1u64 << EXC_STORE_MISALIGNED);

#[repr(C)]
struct KvmSbiFwftFeature {
    id: SbiFwftFeatureT,
    first_reg_num: usize,
    supported: Option<unsafe extern "C" fn(*mut KvmVcpu) -> bool>,
    init: Option<unsafe extern "C" fn(*mut KvmVcpu) -> bool>,
    reset: unsafe extern "C" fn(*mut KvmVcpu),
    set: unsafe extern "C" fn(*mut KvmVcpu, *mut KvmSbiFwftConfig, bool, usize) -> i64,
    get: unsafe extern "C" fn(*mut KvmVcpu, *mut KvmSbiFwftConfig, bool, *mut usize) -> i64,
}

static KVM_FWFT_DEFINED_FEATURES: &[SbiFwftFeatureT] = &[
    SBI_FWFT_MISALIGNED_EXC_DELEG, SBI_FWFT_LANDING_PAD,
    SBI_FWFT_SHADOW_STACK, SBI_FWFT_DOUBLE_TRAP,
    SBI_FWFT_PTE_AD_HW_UPDATING, SBI_FWFT_POINTER_MASKING_PMLEN,
];

unsafe fn kvm_fwft_is_defined_feature(feature: SbiFwftFeatureT) -> bool {
    KVM_FWFT_DEFINED_FEATURES.iter().any(|&x| x == feature)
}

unsafe fn kvm_sbi_fwft_envcfg_flag_reset(vcpu: *mut KvmVcpu, flag: u64) {
    (*vcpu).arch.cfg.henvcfg &= !flag;
}

unsafe fn kvm_sbi_fwft_envcfg_flag_set(vcpu: *mut KvmVcpu, _conf: *mut KvmSbiFwftConfig, one_reg_access: bool, value: usize, flag: u64) -> i64 {
    if value == 0 { (*vcpu).arch.cfg.henvcfg &= !flag; }
    else if value == 1 { (*vcpu).arch.cfg.henvcfg |= flag; }
    else { return SBI_ERR_INVALID_PARAM; }
    if !one_reg_access {
        csr_write(CSR_HENVCFG, (*vcpu).arch.cfg.henvcfg);
        // CONFIG_32BIT: the high HENVCFG CSR is written on 32-bit builds.
        if IS_ENABLED_CONFIG_32BIT { csr_write(CSR_HENVCFGH, (*vcpu).arch.cfg.henvcfg >> 32); }
    }
    SBI_SUCCESS
}

unsafe fn kvm_sbi_fwft_envcfg_flag_get(vcpu: *mut KvmVcpu, _conf: *mut KvmSbiFwftConfig, _one_reg_access: bool, value: *mut usize, flag: u64) -> i64 {
    *value = (((*vcpu).arch.cfg.henvcfg & flag) == flag) as usize;
    SBI_SUCCESS
}

unsafe extern "C" fn kvm_sbi_fwft_misaligned_delegation_supported(_vcpu: *mut KvmVcpu) -> bool { misaligned_traps_can_delegate() }
unsafe extern "C" fn kvm_sbi_fwft_reset_misaligned_delegation(vcpu: *mut KvmVcpu) { (*vcpu).arch.cfg.hedeleg &= !MIS_DELEG; }
unsafe extern "C" fn kvm_sbi_fwft_set_misaligned_delegation(vcpu: *mut KvmVcpu, _conf: *mut KvmSbiFwftConfig, one_reg_access: bool, value: usize) -> i64 {
    if value == 1 { (*vcpu).arch.cfg.hedeleg |= MIS_DELEG; if !one_reg_access { csr_set(CSR_HEDELEG, MIS_DELEG); } }
    else if value == 0 { (*vcpu).arch.cfg.hedeleg &= !MIS_DELEG; if !one_reg_access { csr_clear(CSR_HEDELEG, MIS_DELEG); } }
    else { return SBI_ERR_INVALID_PARAM; } SBI_SUCCESS
}
unsafe extern "C" fn kvm_sbi_fwft_get_misaligned_delegation(vcpu: *mut KvmVcpu, _conf: *mut KvmSbiFwftConfig, _one_reg_access: bool, value: *mut usize) -> i64 { *value = (((*vcpu).arch.cfg.hedeleg & MIS_DELEG) == MIS_DELEG) as usize; SBI_SUCCESS }

unsafe fn kvm_sbi_fwft_set_cfi(vcpu: *mut KvmVcpu, _conf: *mut KvmSbiFwftConfig, one_reg_access: bool, value: usize, flag: u64) -> i64 {
    if value == 0 { (*vcpu).arch.cfg.henvcfg &= !flag; } else if value == 1 { (*vcpu).arch.cfg.henvcfg |= flag; } else { return SBI_ERR_INVALID_PARAM; }
    if (*vcpu).arch.cfg.henvcfg & (ENVCFG_LPE | ENVCFG_SSE) != 0 { (*vcpu).arch.cfg.hedeleg |= 1u64 << EXC_SOFTWARE_CHECK; } else { (*vcpu).arch.cfg.hedeleg &= !(1u64 << EXC_SOFTWARE_CHECK); }
    if !one_reg_access { csr_write(CSR_HEDELEG, (*vcpu).arch.cfg.hedeleg); csr_write(CSR_HENVCFG, (*vcpu).arch.cfg.henvcfg); } SBI_SUCCESS
}

unsafe extern "C" fn kvm_sbi_fwft_landing_pad_supported(vcpu: *mut KvmVcpu) -> bool { riscv_isa_extension_available((*vcpu).arch.isa, ZICFILP) }
unsafe extern "C" fn kvm_sbi_fwft_reset_landing_pad(vcpu: *mut KvmVcpu) { kvm_sbi_fwft_envcfg_flag_reset(vcpu, ENVCFG_LPE); if (*vcpu).arch.cfg.henvcfg & (ENVCFG_LPE | ENVCFG_SSE) == 0 { (*vcpu).arch.cfg.hedeleg &= !(1u64 << EXC_SOFTWARE_CHECK); } }
unsafe extern "C" fn kvm_sbi_fwft_set_landing_pad(v: *mut KvmVcpu,c:*mut KvmSbiFwftConfig,o:bool,x:usize)->i64 { kvm_sbi_fwft_set_cfi(v,c,o,x,ENVCFG_LPE) }
unsafe extern "C" fn kvm_sbi_fwft_get_landing_pad(v:*mut KvmVcpu,c:*mut KvmSbiFwftConfig,o:bool,x:*mut usize)->i64 { kvm_sbi_fwft_envcfg_flag_get(v,c,o,x,ENVCFG_LPE) }
unsafe extern "C" fn kvm_sbi_fwft_shadow_stack_supported(v:*mut KvmVcpu)->bool { riscv_isa_extension_available((*v).arch.isa,ZICFISS) }
unsafe extern "C" fn kvm_sbi_fwft_reset_shadow_stack(v:*mut KvmVcpu) { kvm_sbi_fwft_envcfg_flag_reset(v,ENVCFG_SSE); if (*v).arch.cfg.henvcfg & (ENVCFG_LPE|ENVCFG_SSE)==0 { (*v).arch.cfg.hedeleg &= !(1u64<<EXC_SOFTWARE_CHECK); } }
unsafe extern "C" fn kvm_sbi_fwft_set_shadow_stack(v:*mut KvmVcpu,c:*mut KvmSbiFwftConfig,o:bool,x:usize)->i64 { kvm_sbi_fwft_set_cfi(v,c,o,x,ENVCFG_SSE) }
unsafe extern "C" fn kvm_sbi_fwft_get_shadow_stack(v:*mut KvmVcpu,c:*mut KvmSbiFwftConfig,o:bool,x:*mut usize)->i64 { kvm_sbi_fwft_envcfg_flag_get(v,c,o,x,ENVCFG_SSE) }
unsafe extern "C" fn kvm_sbi_fwft_pte_ad_hw_updating_supported(v:*mut KvmVcpu)->bool { riscv_isa_extension_available((*v).arch.isa,SVADU) && riscv_isa_extension_available((*v).arch.isa,SVADE) }
unsafe extern "C" fn kvm_sbi_fwft_reset_pte_ad_hw_updating(v:*mut KvmVcpu) { if kvm_sbi_fwft_pte_ad_hw_updating_supported(v) { kvm_sbi_fwft_envcfg_flag_reset(v,ENVCFG_ADUE); } }
unsafe extern "C" fn kvm_sbi_fwft_set_pte_ad_hw_updating(v:*mut KvmVcpu,c:*mut KvmSbiFwftConfig,o:bool,x:usize)->i64 { kvm_sbi_fwft_envcfg_flag_set(v,c,o,x,ENVCFG_ADUE) }
unsafe extern "C" fn kvm_sbi_fwft_get_pte_ad_hw_updating(v:*mut KvmVcpu,c:*mut KvmSbiFwftConfig,o:bool,x:*mut usize)->i64 { kvm_sbi_fwft_envcfg_flag_get(v,c,o,x,ENVCFG_ADUE) }

// The pointer-masking feature is omitted from CONFIG_32BIT builds, as in the source.
#[cfg(not(CONFIG_32BIT))]
unsafe fn try_to_set_pmm(value: usize) -> bool { let prev=csr_read_clear(CSR_HENVCFG,ENVCFG_PMM); csr_set(CSR_HENVCFG,value as u64); let ret=(csr_read_clear(CSR_HENVCFG,ENVCFG_PMM)&ENVCFG_PMM)==value as u64; csr_write(CSR_HENVCFG,prev); ret }
#[cfg(not(CONFIG_32BIT))]
unsafe extern "C" fn kvm_sbi_fwft_pointer_masking_pmlen_supported(v:*mut KvmVcpu)->bool { riscv_isa_extension_available((*v).arch.isa,SMNPM) }
#[cfg(not(CONFIG_32BIT))]
unsafe extern "C" fn kvm_sbi_fwft_reset_pointer_masking_pmlen(v:*mut KvmVcpu) { (*v).arch.cfg.henvcfg &= !ENVCFG_PMM; }
#[cfg(not(CONFIG_32BIT))]
unsafe extern "C" fn kvm_sbi_fwft_set_pointer_masking_pmlen(v:*mut KvmVcpu,_c:*mut KvmSbiFwftConfig,o:bool,value:usize)->i64 { let pmm=match value {0=>ENVCFG_PMM_PMLEN_0,7=>ENVCFG_PMM_PMLEN_7,16=>ENVCFG_PMM_PMLEN_16,_=>return SBI_ERR_INVALID_PARAM}; (*v).arch.cfg.henvcfg=((*v).arch.cfg.henvcfg & !ENVCFG_PMM)|pmm; if !o {csr_write(CSR_HENVCFG,(*v).arch.cfg.henvcfg);} SBI_SUCCESS }
#[cfg(not(CONFIG_32BIT))]
unsafe extern "C" fn kvm_sbi_fwft_get_pointer_masking_pmlen(v:*mut KvmVcpu,_c:*mut KvmSbiFwftConfig,_o:bool,value:*mut usize)->i64 { match (*v).arch.cfg.henvcfg&ENVCFG_PMM {ENVCFG_PMM_PMLEN_0=>*value=0,ENVCFG_PMM_PMLEN_7=>*value=7,ENVCFG_PMM_PMLEN_16=>*value=16,_=>return SBI_ERR_FAILURE} SBI_SUCCESS }

#[repr(C)]
static FEATURES: &[KvmSbiFwftFeature] = &[
    KvmSbiFwftFeature { id:SBI_FWFT_MISALIGNED_EXC_DELEG, first_reg_num:0, supported:Some(kvm_sbi_fwft_misaligned_delegation_supported), init:None, reset:kvm_sbi_fwft_reset_misaligned_delegation, set:kvm_sbi_fwft_set_misaligned_delegation, get:kvm_sbi_fwft_get_misaligned_delegation },
    KvmSbiFwftFeature { id:SBI_FWFT_LANDING_PAD, first_reg_num:0, supported:Some(kvm_sbi_fwft_landing_pad_supported), init:None, reset:kvm_sbi_fwft_reset_landing_pad, set:kvm_sbi_fwft_set_landing_pad, get:kvm_sbi_fwft_get_landing_pad },
    KvmSbiFwftFeature { id:SBI_FWFT_SHADOW_STACK, first_reg_num:0, supported:Some(kvm_sbi_fwft_shadow_stack_supported), init:None, reset:kvm_sbi_fwft_reset_shadow_stack, set:kvm_sbi_fwft_set_shadow_stack, get:kvm_sbi_fwft_get_shadow_stack },
    KvmSbiFwftFeature { id:SBI_FWFT_PTE_AD_HW_UPDATING, first_reg_num:0, supported:Some(kvm_sbi_fwft_pte_ad_hw_updating_supported), init:None, reset:kvm_sbi_fwft_reset_pte_ad_hw_updating, set:kvm_sbi_fwft_set_pte_ad_hw_updating, get:kvm_sbi_fwft_get_pte_ad_hw_updating },
];

unsafe fn kvm_sbi_fwft_regnum_to_feature(reg_num: usize) -> *const KvmSbiFwftFeature {
    for feature in FEATURES { if feature.first_reg_num <= reg_num && reg_num < feature.first_reg_num + 3 { return feature; } } std::ptr::null()
}

unsafe fn kvm_fwft_get_feature(vcpu:*mut KvmVcpu, feature:SbiFwftFeatureT, conf:*mut *mut KvmSbiFwftConfig)->i32 {
    let tconf=kvm_sbi_fwft_get_config(vcpu,feature); if tconf.is_null() { return if kvm_fwft_is_defined_feature(feature){SBI_ERR_NOT_SUPPORTED as i32}else{SBI_ERR_DENIED as i32}; }
    if !(*tconf).supported || !(*tconf).enabled { return SBI_ERR_NOT_SUPPORTED as i32; }
    let f=(*tconf).feature; if !f.is_null() { if let Some(s)=(*f).supported { if !s(vcpu){return SBI_ERR_NOT_SUPPORTED as i32;} } } *conf=tconf; SBI_SUCCESS as i32
}

unsafe fn kvm_sbi_fwft_set(v:*mut KvmVcpu, feature:u32, value:usize, flags:usize)->i32 { let mut c=std::ptr::null_mut(); let mut ret=kvm_fwft_get_feature(v,feature as SbiFwftFeatureT,&mut c); if ret!=0{return ret;} if flags & !SBI_FWFT_SET_FLAG_LOCK !=0{return SBI_ERR_INVALID_PARAM as i32;} if (*c).flags&SBI_FWFT_SET_FLAG_LOCK!=0{return SBI_ERR_DENIED_LOCKED as i32;} ret=((*(*c).feature).set)(v,c,false,value) as i32; if ret==SBI_SUCCESS as i32 {(*c).flags=flags;} ret }
unsafe fn kvm_sbi_fwft_get(v:*mut KvmVcpu, feature:usize, value:*mut usize)->i32 { let mut c=std::ptr::null_mut(); let r=kvm_fwft_get_feature(v,feature as SbiFwftFeatureT,&mut c); if r!=0{return r;} ((*(*c).feature).get)(v,c,false,value) as i32 }

unsafe fn kvm_sbi_ext_fwft_handler(v:*mut KvmVcpu,_run:*mut KvmRun,retdata:*mut KvmVcpuSbiReturn)->i32 { let cp=&(*v).arch.guest_context; (*retdata).err_val=match cp.a6 {SBI_EXT_FWFT_SET=>kvm_sbi_fwft_set(v,cp.a0,cp.a1,cp.a2),SBI_EXT_FWFT_GET=>kvm_sbi_fwft_get(v,cp.a0,&mut (*retdata).out_val),_=>SBI_ERR_NOT_SUPPORTED as i32}; 0 }

unsafe fn kvm_sbi_ext_fwft_init(v:*mut KvmVcpu)->i32 { let fwft=vcpu_to_fwft(v); (*fwft).configs=kzalloc_objs::<KvmSbiFwftConfig>(FEATURES.len(),GFP_KERNEL_ACCOUNT); if (*fwft).configs.is_null(){return -ENOMEM;} for (i,f) in FEATURES.iter().enumerate(){let c=&mut *(*fwft).configs.add(i); c.supported=f.supported.map_or(true,|x|x(v)); c.enabled=c.supported; c.feature=f;} 0 }
unsafe fn kvm_sbi_ext_fwft_deinit(v:*mut KvmVcpu){ kfree((*vcpu_to_fwft(v)).configs); }
unsafe fn kvm_sbi_ext_fwft_reset(v:*mut KvmVcpu){let f=vcpu_to_fwft(v); for(i,x)in FEATURES.iter().enumerate(){(*f).configs.add(i).as_mut().unwrap().flags=0;(x.reset)(v);} (*v).arch.csr_dirty=true;}
unsafe fn kvm_sbi_fwft_get_config(_v:*mut KvmVcpu,_f:SbiFwftFeatureT)->*mut KvmSbiFwftConfig { std::ptr::null_mut() }

extern "C" { static vcpu_sbi_ext_fwft: KvmVcpuSbiExtension; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
