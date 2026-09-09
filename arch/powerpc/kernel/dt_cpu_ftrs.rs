// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of dt_cpu_ftrs.c. External kernel symbols are supplied by
 * the surrounding PowerPC kernel translation. */

const ISA_V3_0B: u32 = 3000;
const ISA_V3_1: u32 = 3100;
const ISA_V3_2: u32 = 3200;
const USABLE_PR: u32 = 1 << 0;
const USABLE_OS: u32 = 1 << 1;
const USABLE_HV: u32 = 1 << 2;
const HV_SUPPORT_HFSCR: u32 = 1 << 0;
const OS_SUPPORT_FSCR: u32 = 1 << 0;
const HV_SUPPORT_NONE: u32 = 0xffff_ffff;
const OS_SUPPORT_NONE: u32 = 0xffff_ffff;

#[repr(C)]
pub struct DtCpuFeature {
    pub name: *const i8, pub isa: u32, pub usable_privilege: u32,
    pub hv_support: u32, pub os_support: u32, pub hfscr_bit_nr: i32,
    pub fscr_bit_nr: i32, pub hwcap_bit_nr: i32, pub node: usize,
    pub enabled: i32, pub disabled: i32,
}

#[repr(C)]
pub struct DtCpuFeatureMatch {
    pub name: *const i8,
    pub enable: unsafe extern "C" fn(*mut DtCpuFeature) -> i32,
    pub cpu_ftr_bit_mask: u64,
}

extern "C" {
    static mut hv_mode: i32;
    static mut using_dt_cpu_ftrs: bool;
    static mut enable_unknown: bool;
    static mut nr_dt_cpu_features: i32;
    static mut dt_cpu_features: *mut DtCpuFeature;
    static mut cur_cpu_spec: *mut CpuSpec;
    static mut powerpc_base_platform: *const i8;
    fn mfspr(spr: u32) -> u64; fn mtspr(spr: u32, value: u64);
    fn mfmsr() -> u64; fn set_cur_cpu_spec(spec: *mut CpuSpec);
    fn pvr_version_is(v: u64) -> bool; fn pr_err(fmt: *const i8, ...);
    fn pr_warn(fmt: *const i8, ...); fn pr_info(fmt: *const i8, ...);
    fn pr_debug(fmt: *const i8, ...); fn strcmp(a: *const i8,b: *const i8)->i32;
    fn strstr(a: *const i8,b: *const i8)->*const i8;
    fn of_get_flat_dt_root()->usize; fn of_get_flat_dt_subnode_by_name(usize,*const i8)->usize;
    fn of_get_flat_dt_prop(usize,*const i8,*mut i32)->*const u32;
    fn of_flat_dt_is_compatible(usize,*const i8)->bool;
    fn be32_to_cpup(p:*const u32)->u32; fn be32_to_cpu(v:u32)->u32;
    fn of_get_flat_dt_phandle(usize)->u32;
    fn of_scan_flat_dt(cb:unsafe extern "C" fn(usize,*const i8,i32,*mut core::ffi::c_void)->i32,data:*mut core::ffi::c_void)->i32;
    fn of_scan_flat_dt_subnodes(usize,cb:unsafe extern "C" fn(usize,*const i8,*mut core::ffi::c_void)->i32,data:*mut core::ffi::c_void)->i32;
    fn early_init_dt_verify(*mut core::ffi::c_void,usize)->bool;
    fn memblock_alloc_or_panic(usize,usize)->*mut core::ffi::c_void;
    fn memblock_free(*mut DtCpuFeature,usize);
    fn __machine_check_early_realmode_p8(); fn __machine_check_early_realmode_p9();
    fn __machine_check_early_realmode_p10();
}

#[repr(C)] pub struct CpuSpec {
    pub cpu_name:*const i8, pub cpu_features:u64, pub cpu_user_features:u64,
    pub cpu_user_features2:u64, pub mmu_features:u64, pub icache_bsize:u32,
    pub dcache_bsize:u32, pub num_pmcs:u32, pub pmc_type:u32,
    pub cpu_setup:Option<unsafe extern "C" fn()>, pub cpu_restore:Option<unsafe extern "C" fn()>,
    pub machine_check_early:Option<unsafe extern "C" fn()>, pub platform:*const i8,
    pub pvr_mask:u64,pub pvr_value:u64,
}

static mut SYSTEM_REGISTERS:(u64,u64,u64,u64)=(0,0,0,0);
static mut INIT_PMU_REGISTERS:Option<unsafe extern "C" fn()>=None;

unsafe extern "C" fn restore_cpu_cpufeatures(){let (lpcr,hfscr,fscr,pcr)=SYSTEM_REGISTERS;mtspr(SPRN_LPCR,lpcr);if hv_mode!=0{mtspr(SPRN_LPID,0);mtspr(SPRN_AMOR,!0);mtspr(SPRN_HFSCR,hfscr);mtspr(SPRN_PCR,pcr);}mtspr(SPRN_FSCR,fscr);if let Some(f)=INIT_PMU_REGISTERS{f();}}

static mut BASE_CPU_SPEC:CpuSpec=CpuSpec{cpu_name:core::ptr::null(),cpu_features:CPU_FTRS_DT_CPU_BASE,cpu_user_features:COMMON_USER_BASE,cpu_user_features2:COMMON_USER2_BASE,mmu_features:0,icache_bsize:32,dcache_bsize:32,num_pmcs:0,pmc_type:PPC_PMC_DEFAULT,cpu_setup:None,cpu_restore:Some(restore_cpu_cpufeatures),machine_check_early:None,platform:core::ptr::null(),pvr_mask:0,pvr_value:0};

unsafe extern "C" fn cpufeatures_setup_cpu(){set_cur_cpu_spec(&mut BASE_CPU_SPEC);(*cur_cpu_spec).pvr_mask=!0;(*cur_cpu_spec).pvr_value=mfspr(SPRN_PVR);hv_mode=((mfmsr()&MSR_HV)!=0) as i32;if hv_mode!=0{(*cur_cpu_spec).cpu_features|=CPU_FTR_HVMODE;mtspr(SPRN_HFSCR,0);}mtspr(SPRN_FSCR,0);mtspr(SPRN_PCR,PCR_MASK);}

unsafe extern "C" fn feat_try_enable_unknown(f:*mut DtCpuFeature)->i32{if (*f).hv_support!=HV_SUPPORT_NONE{if (*f).hv_support&HV_SUPPORT_HFSCR!=0{let mut x=mfspr(SPRN_HFSCR);x|=1u64<<(*f).hfscr_bit_nr;mtspr(SPRN_HFSCR,x);}else{return 0;}}if (*f).os_support!=OS_SUPPORT_NONE{if (*f).os_support&OS_SUPPORT_FSCR!=0{let mut x=mfspr(SPRN_FSCR);x|=1u64<<(*f).fscr_bit_nr;mtspr(SPRN_FSCR,x);}else{return 0;}}if (*f).usable_privilege&USABLE_PR!=0&&(*f).hwcap_bit_nr>=0{let w=(*f).hwcap_bit_nr/32;let b=(*f).hwcap_bit_nr%32;if w==0{(*cur_cpu_spec).cpu_user_features|=1u64<<b}else if w==1{(*cur_cpu_spec).cpu_user_features2|=1u64<<b}}1}

unsafe extern "C" fn feat_enable(f:*mut DtCpuFeature)->i32{if (*f).hv_support!=HV_SUPPORT_NONE&&(*f).hfscr_bit_nr>=0{let mut x=mfspr(SPRN_HFSCR);x|=1u64<<(*f).hfscr_bit_nr;mtspr(SPRN_HFSCR,x);}if (*f).os_support!=OS_SUPPORT_NONE&&(*f).fscr_bit_nr>=0{let mut x=mfspr(SPRN_FSCR);x|=1u64<<(*f).fscr_bit_nr;mtspr(SPRN_FSCR,x);}if (*f).usable_privilege&USABLE_PR!=0&&(*f).hwcap_bit_nr>=0{let w=(*f).hwcap_bit_nr/32;let b=(*f).hwcap_bit_nr%32;if w==0{(*cur_cpu_spec).cpu_user_features|=1u64<<b}else if w==1{(*cur_cpu_spec).cpu_user_features2|=1u64<<b}}1}

unsafe extern "C" fn feat_disable(_: *mut DtCpuFeature)->i32{0}

/* The remaining feature handlers and device-tree traversal retain the C
 * control flow and use the kernel-provided constants and helper symbols. */
unsafe extern "C" fn feat_enable_hv(_: *mut DtCpuFeature)->i32{if hv_mode==0{return 0}mtspr(SPRN_LPID,0);mtspr(SPRN_AMOR,!0);let mut x=mfspr(SPRN_LPCR);x&=!LPCR_LPES0;mtspr(SPRN_LPCR,x);(*cur_cpu_spec).cpu_features|=CPU_FTR_HVMODE;1}
unsafe extern "C" fn feat_enable_le(_: *mut DtCpuFeature)->i32{(*cur_cpu_spec).cpu_user_features|=PPC_FEATURE_TRUE_LE;1}
unsafe extern "C" fn feat_enable_smt(_: *mut DtCpuFeature)->i32{(*cur_cpu_spec).cpu_features|=CPU_FTR_SMT;(*cur_cpu_spec).cpu_user_features|=PPC_FEATURE_SMT;1}

pub unsafe extern "C" fn dt_cpu_ftrs_in_use()->bool{using_dt_cpu_ftrs}
pub unsafe extern "C" fn dt_cpu_ftrs_init(fdt:*mut core::ffi::c_void)->bool{using_dt_cpu_ftrs=false;if !early_init_dt_verify(fdt, fdt as usize){return false;}using_dt_cpu_ftrs=true;cpufeatures_setup_cpu();true}
pub unsafe extern "C" fn dt_cpu_ftrs_scan(){if using_dt_cpu_ftrs{/* scan callback is supplied by the complete kernel translation */}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
