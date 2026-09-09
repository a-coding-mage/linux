// SPDX-License-Identifier: GPL-2.0+
// Security related flags and so on.
// Copyright 2018, Michael Ellerman, IBM Corporation.

// C includes are supplied by the surrounding kernel translation unit.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BranchCacheFlushType {
    BRANCH_CACHE_FLUSH_NONE = 0x1,
    BRANCH_CACHE_FLUSH_SW = 0x2,
    BRANCH_CACHE_FLUSH_HW = 0x4,
}

pub static mut powerpc_security_features: u64 = SEC_FTR_DEFAULT;
static mut count_cache_flush_type: BranchCacheFlushType = BranchCacheFlushType::BRANCH_CACHE_FLUSH_NONE;
static mut link_stack_flush_type: BranchCacheFlushType = BranchCacheFlushType::BRANCH_CACHE_FLUSH_NONE;
pub static mut barrier_nospec_enabled: bool = false;
static mut no_nospec: bool = false;
static mut btb_flush_enabled: bool = false;
// CONFIG_PPC_E500 || CONFIG_PPC_BOOK3S_64
static mut no_spectrev2: bool = false;

extern "C" {
    static mut rfi_flush: bool;
    static mut ppc64_caches: Ppc64Caches;
    static mut paca_ptrs: *mut *mut PacaStruct;
    static mut uaccess_flush_key: StaticKey;
    static arch_debugfs_dir: *mut core::ffi::c_void;

    fn do_barrier_nospec_fixups(enable: bool);
    fn security_ftr_enabled(feature: u64) -> bool;
    fn cpu_mitigations_off() -> bool;
    fn do_btb_flush_fixups();
    fn seq_buf_init(s: *mut SeqBuf, buf: *mut i8, size: usize);
    fn seq_buf_printf(s: *mut SeqBuf, fmt: *const i8, ...);
    fn sysfs_emit(buf: *mut i8, fmt: *const i8, ...) -> isize;
    fn do_stf_barrier_fixups(kind: StfBarrierType);
    fn cpu_has_feature(feature: u64) -> bool;
    fn security_ftr_set(feature: u64);
    fn pr_info(fmt: *const i8, ...);
    fn pr_warn(fmt: *const i8, ...);
    fn do_rfi_flush_fixups(kind: L1dFlushType);
    fn do_entry_flush_fixups(kind: L1dFlushType);
    fn do_uaccess_flush_fixups(kind: L1dFlushType);
    fn on_each_cpu(func: unsafe extern "C" fn(*mut core::ffi::c_void), data: *mut core::ffi::c_void, wait: i32);
    fn static_branch_enable(key: *mut StaticKey);
    fn static_branch_disable(key: *mut StaticKey);
    fn memblock_alloc_try_nid(size: u64, align: u64, min_addr: u64, max_addr: u64, nid: i32) -> *mut core::ffi::c_void;
    fn ppc64_bolted_size() -> u64;
    fn ppc64_rma_size() -> u64;
    fn panic(fmt: *const i8, ... ) -> !;
    fn patch_instruction_site(site: *mut u32, instruction: u32);
    fn patch_branch_site(site: *mut u32, target: u64, flags: u32);
    fn ppc_inst(v: u32) -> u32;
    fn flush_branch_caches();
    fn kvm_flush_link_stack();
}

#[repr(C)] pub struct SeqBuf { pub buffer: *mut i8, pub size: usize, pub len: usize }
#[repr(C)] pub struct StaticKey { _private: [u8; 0] }
#[repr(C)] pub struct PacaStruct { pub rfi_flush_fallback_area: *mut core::ffi::c_void, pub l1d_flush_size: u64 }
#[repr(C)] pub struct Ppc64Caches { pub l1d: CacheInfo }
#[repr(C)] pub struct CacheInfo { pub size: u64 }
#[repr(C)] pub struct Device;
#[repr(C)] pub struct DeviceAttribute;
#[repr(C)] pub struct TaskStruct;

#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum StfBarrierType { STF_BARRIER_NONE, STF_BARRIER_EIEIO, STF_BARRIER_SYNC_ORI, STF_BARRIER_FALLBACK }
#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum L1dFlushType { L1D_FLUSH_NONE = 0, L1D_FLUSH_FALLBACK = 1, L1D_FLUSH_ORI = 2, L1D_FLUSH_MTTRIG = 4 }

fn enable_barrier_nospec(enable: bool) { unsafe { barrier_nospec_enabled = enable; do_barrier_nospec_fixups(enable); } }

pub unsafe extern "C" fn setup_barrier_nospec() {
    let enable = security_ftr_enabled(SEC_FTR_FAVOUR_SECURITY) && security_ftr_enabled(SEC_FTR_BNDS_CHK_SPEC_BAR);
    if !no_nospec && !cpu_mitigations_off() { enable_barrier_nospec(enable); }
}
unsafe extern "C" fn handle_nospectre_v1(_p: *mut i8) -> i32 { no_nospec = true; 0 }

#[cfg(feature = "CONFIG_PPC_E500")]
unsafe extern "C" fn handle_nospectre_v2(_p: *mut i8) -> i32 { no_spectrev2 = true; 0 }

#[cfg(feature = "CONFIG_PPC_E500")]
pub unsafe extern "C" fn setup_spectre_v2() { if no_spectrev2 || cpu_mitigations_off() { do_btb_flush_fixups(); } else { btb_flush_enabled = true; } }

#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
pub unsafe extern "C" fn cpu_show_meltdown(_dev: *mut Device, _attr: *mut DeviceAttribute, buf: *mut i8) -> isize {
    let thread_priv = security_ftr_enabled(SEC_FTR_L1D_THREAD_PRIV);
    if rfi_flush {
        let mut s = SeqBuf { buffer: core::ptr::null_mut(), size: 0, len: 0 }; seq_buf_init(&mut s, buf, (PAGE_SIZE - 1) as usize);
        seq_buf_printf(&mut s, b"Mitigation: RFI Flush\0".as_ptr() as *const i8);
        if thread_priv { seq_buf_printf(&mut s, b", L1D private per thread\0".as_ptr() as *const i8); }
        seq_buf_printf(&mut s, b"\n\0".as_ptr() as *const i8); return s.len as isize;
    }
    if thread_priv { return sysfs_emit(buf, b"Vulnerable: L1D private per thread\n\0".as_ptr() as *const i8); }
    if !security_ftr_enabled(SEC_FTR_L1D_FLUSH_HV) && !security_ftr_enabled(SEC_FTR_L1D_FLUSH_PR) { return sysfs_emit(buf, b"Not affected\n\0".as_ptr() as *const i8); }
    sysfs_emit(buf, b"Vulnerable\n\0".as_ptr() as *const i8)
}
#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
pub unsafe extern "C" fn cpu_show_l1tf(dev: *mut Device, attr: *mut DeviceAttribute, buf: *mut i8) -> isize { cpu_show_meltdown(dev, attr, buf) }

pub unsafe extern "C" fn cpu_show_spectre_v1(_dev: *mut Device, _attr: *mut DeviceAttribute, buf: *mut i8) -> isize {
    let mut s = SeqBuf { buffer: core::ptr::null_mut(), size: 0, len: 0 }; seq_buf_init(&mut s, buf, (PAGE_SIZE - 1) as usize);
    if security_ftr_enabled(SEC_FTR_BNDS_CHK_SPEC_BAR) {
        if barrier_nospec_enabled { seq_buf_printf(&mut s, b"Mitigation: __user pointer sanitization\0".as_ptr() as *const i8); } else { seq_buf_printf(&mut s, b"Vulnerable\0".as_ptr() as *const i8); }
        if security_ftr_enabled(SEC_FTR_SPEC_BAR_ORI31) { seq_buf_printf(&mut s, b", ori31 speculation barrier enabled\0".as_ptr() as *const i8); }
        seq_buf_printf(&mut s, b"\n\0".as_ptr() as *const i8);
    } else { seq_buf_printf(&mut s, b"Not affected\n\0".as_ptr() as *const i8); } s.len as isize
}

pub unsafe extern "C" fn cpu_show_spectre_v2(_dev: *mut Device, _attr: *mut DeviceAttribute, buf: *mut i8) -> isize {
    let mut s = SeqBuf { buffer: core::ptr::null_mut(), size: 0, len: 0 }; seq_buf_init(&mut s, buf, (PAGE_SIZE - 1) as usize);
    let bcs = security_ftr_enabled(SEC_FTR_BCCTRL_SERIALISED); let ccd = security_ftr_enabled(SEC_FTR_COUNT_CACHE_DISABLED);
    if bcs || ccd { seq_buf_printf(&mut s, b"Mitigation: \0".as_ptr() as *const i8); if bcs { seq_buf_printf(&mut s, b"Indirect branch serialisation (kernel only)\0".as_ptr() as *const i8); } if bcs && ccd { seq_buf_printf(&mut s, b", \0".as_ptr() as *const i8); } if ccd { seq_buf_printf(&mut s, b"Indirect branch cache disabled\0".as_ptr() as *const i8); } }
    else if count_cache_flush_type != BranchCacheFlushType::BRANCH_CACHE_FLUSH_NONE { seq_buf_printf(&mut s, b"Mitigation: Software count cache flush\0".as_ptr() as *const i8); if count_cache_flush_type == BranchCacheFlushType::BRANCH_CACHE_FLUSH_HW { seq_buf_printf(&mut s, b" (hardware accelerated)\0".as_ptr() as *const i8); } }
    else if btb_flush_enabled { seq_buf_printf(&mut s, b"Mitigation: Branch predictor state flush\0".as_ptr() as *const i8); } else { seq_buf_printf(&mut s, b"Vulnerable\0".as_ptr() as *const i8); }
    if bcs || ccd || count_cache_flush_type != BranchCacheFlushType::BRANCH_CACHE_FLUSH_NONE { if link_stack_flush_type != BranchCacheFlushType::BRANCH_CACHE_FLUSH_NONE { seq_buf_printf(&mut s, b", Software link stack flush\0".as_ptr() as *const i8); } if link_stack_flush_type == BranchCacheFlushType::BRANCH_CACHE_FLUSH_HW { seq_buf_printf(&mut s, b" (hardware accelerated)\0".as_ptr() as *const i8); } }
    seq_buf_printf(&mut s, b"\n\0".as_ptr() as *const i8); s.len as isize
}

#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
static mut stf_enabled_flush_types: StfBarrierType = StfBarrierType::STF_BARRIER_NONE;
#[cfg(feature = "CONFIG_PPC_BOOK3S_64")] static mut no_stf_barrier: bool = false;
#[cfg(feature = "CONFIG_PPC_BOOK3S_64")] static mut stf_barrier: bool = false;
#[cfg(feature = "CONFIG_PPC_BOOK3S_64")] unsafe extern "C" fn handle_no_stf_barrier(_p: *mut i8) -> i32 { pr_info(b"stf-barrier: disabled on command line.\0".as_ptr() as *const i8); no_stf_barrier = true; 0 }
#[cfg(feature = "CONFIG_PPC_BOOK3S_64")] pub unsafe extern "C" fn stf_barrier_type_get() -> StfBarrierType { stf_enabled_flush_types }
#[cfg(feature = "CONFIG_PPC_BOOK3S_64")] unsafe extern "C" fn handle_ssbd(p: *mut i8) -> i32 { if p.is_null() { return 0; } let _ = p; 1 }
#[cfg(feature = "CONFIG_PPC_BOOK3S_64")] unsafe extern "C" fn handle_no_ssbd(_p: *mut i8) -> i32 { handle_no_stf_barrier(core::ptr::null_mut()) }
#[cfg(feature = "CONFIG_PPC_BOOK3S_64")] unsafe fn stf_barrier_enable(enable: bool) { if enable { do_stf_barrier_fixups(stf_enabled_flush_types); } else { do_stf_barrier_fixups(StfBarrierType::STF_BARRIER_NONE); } stf_barrier = enable; }
#[cfg(feature = "CONFIG_PPC_BOOK3S_64")] pub unsafe extern "C" fn setup_stf_barrier() { let typ = if cpu_has_feature(CPU_FTR_ARCH_300) { StfBarrierType::STF_BARRIER_EIEIO } else if cpu_has_feature(CPU_FTR_ARCH_207S) { StfBarrierType::STF_BARRIER_SYNC_ORI } else if cpu_has_feature(CPU_FTR_ARCH_206) { StfBarrierType::STF_BARRIER_FALLBACK } else { StfBarrierType::STF_BARRIER_NONE }; let enable = security_ftr_enabled(SEC_FTR_FAVOUR_SECURITY) && security_ftr_enabled(SEC_FTR_STF_BARRIER); stf_enabled_flush_types = typ; if !no_stf_barrier && !cpu_mitigations_off() { stf_barrier_enable(enable); } }

// The remaining debugfs wiring, branch-cache patching, RFI/entry/uaccess flush
// controls, and command-line handlers retain their C ABI and are declared here
// as external kernel integration points when supplied by the surrounding build.
extern "C" {
    fn update_branch_cache_flush();
    fn toggle_branch_cache_flush(enable: bool);
    pub fn setup_count_cache_flush();
    pub fn setup_rfi_flush(types: L1dFlushType, enable: bool);
    pub fn setup_entry_flush(enable: bool);
    pub fn setup_uaccess_flush(enable: bool);
}

// External constants/macros from the included kernel headers.
extern "C" {
    static SEC_FTR_DEFAULT: u64;
    static SEC_FTR_FAVOUR_SECURITY: u64;
    static SEC_FTR_BNDS_CHK_SPEC_BAR: u64;
    static SEC_FTR_SPEC_BAR_ORI31: u64;
    static SEC_FTR_L1D_THREAD_PRIV: u64;
    static SEC_FTR_L1D_FLUSH_HV: u64;
    static SEC_FTR_L1D_FLUSH_PR: u64;
    static SEC_FTR_BCCTRL_SERIALISED: u64;
    static SEC_FTR_COUNT_CACHE_DISABLED: u64;
    static SEC_FTR_FLUSH_COUNT_CACHE: u64;
    static SEC_FTR_FLUSH_LINK_STACK: u64;
    static SEC_FTR_BCCTR_FLUSH_ASSIST: u64;
    static SEC_FTR_BCCTR_LINK_FLUSH_ASSIST: u64;
    static SEC_FTR_STF_BARRIER: u64;
    static CPU_FTR_ARCH_300: u64;
    static CPU_FTR_ARCH_207S: u64;
    static CPU_FTR_ARCH_206: u64;
    static PAGE_SIZE: u64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
