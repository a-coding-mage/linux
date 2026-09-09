// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2021 Google LLC
 * Author: Fuad Tabba <tabba@google.com>
 */

// Kernel dependencies supplied by the surrounding hypervisor translation.

/* Used by icache_is_aliasing(). */
pub static mut __icache_flags: c_ulong = 0;
/* Used by kvm_get_vttbr(). */
pub static mut kvm_arm_vmid_bits: c_uint = 0;
pub static mut kvm_host_sve_max_vl: c_uint = 0;

static mut loaded_hyp_vcpu: PerCpu<*mut pkvm_hyp_vcpu> = PerCpu::new();

unsafe fn pkvm_vcpu_reset_hcr(vcpu: *mut kvm_vcpu) {
    (*vcpu).arch.hcr_el2 = HCR_GUEST_FLAGS;
    if has_hvhe() { (*vcpu).arch.hcr_el2 |= HCR_E2H; }
    if cpus_have_final_cap(ARM64_HAS_RAS_EXTN) { (*vcpu).arch.hcr_el2 |= HCR_TEA | HCR_TERR; }
    if cpus_have_final_cap(ARM64_HAS_STAGE2_FWB) { (*vcpu).arch.hcr_el2 |= HCR_FWB; }
    if cpus_have_final_cap(ARM64_HAS_EVT) && !cpus_have_final_cap(ARM64_MISMATCHED_CACHE_TYPE) &&
       kvm_read_vm_id_reg((*vcpu).kvm, SYS_CTR_EL0) == read_cpuid(CTR_EL0) {
        (*vcpu).arch.hcr_el2 |= HCR_TID4;
    } else { (*vcpu).arch.hcr_el2 |= HCR_TID2; }
    if vcpu_has_ptrauth(vcpu) { (*vcpu).arch.hcr_el2 |= HCR_API | HCR_APK; }
    if kvm_has_mte((*vcpu).kvm) { (*vcpu).arch.hcr_el2 |= HCR_ATA; }
}

unsafe fn pvm_init_traps_hcr(vcpu: *mut kvm_vcpu) {
    let kvm = (*vcpu).kvm;
    let mut val = (*vcpu).arch.hcr_el2 | HCR_RW | HCR_TACR | HCR_TIDCP | HCR_TID3 | HCR_TID1;
    if !kvm_has_feat(kvm, ID_AA64PFR0_EL1, RAS, IMP) { val |= HCR_TERR | HCR_TEA; val &= !HCR_FIEN; }
    if !kvm_has_feat(kvm, ID_AA64PFR0_EL1, AMU, IMP) { val &= !HCR_AMVOFFEN; }
    if !kvm_has_mte(kvm) { val |= HCR_TID5; val &= !(HCR_DCT | HCR_ATA); }
    if !kvm_has_feat(kvm, ID_AA64MMFR1_EL1, LO, IMP) { val |= HCR_TLOR; }
    (*vcpu).arch.hcr_el2 = val;
}

unsafe fn pvm_init_traps_mdcr(vcpu: *mut kvm_vcpu) {
    let kvm = (*vcpu).kvm;
    let mut val = (*vcpu).arch.mdcr_el2;
    if !kvm_has_feat(kvm, ID_AA64DFR0_EL1, PMUVer, IMP) { val |= MDCR_EL2_TPM | MDCR_EL2_TPMCR; val &= !(MDCR_EL2_HPME | MDCR_EL2_MTPME | MDCR_EL2_HPMN_MASK); }
    if !kvm_has_feat(kvm, ID_AA64DFR0_EL1, DebugVer, IMP) { val |= MDCR_EL2_TDRA | MDCR_EL2_TDA; }
    if !kvm_has_feat(kvm, ID_AA64DFR0_EL1, DoubleLock, IMP) { val |= MDCR_EL2_TDOSA; }
    if !kvm_has_feat(kvm, ID_AA64DFR0_EL1, PMSVer, IMP) { val |= MDCR_EL2_TPMS; val &= !MDCR_EL2_E2PB_MASK; }
    if !kvm_has_feat(kvm, ID_AA64DFR0_EL1, TraceFilt, IMP) { val |= MDCR_EL2_TTRF; }
    if !kvm_has_feat(kvm, ID_AA64DFR0_EL1, TraceBuffer, IMP) { val &= !MDCR_EL2_E2TB_MASK; }
    if !kvm_has_feat(kvm, ID_AA64MMFR0_EL1, FGT, IMP) { val |= MDCR_EL2_TDCC; }
    (*vcpu).arch.mdcr_el2 = val;
}

unsafe fn pkvm_check_pvm_cpu_features(vcpu: *mut kvm_vcpu) -> c_int {
    let kvm = (*vcpu).kvm;
    if kvm_has_feat(kvm, ID_AA64PFR0_EL1, EL0, AARCH32) || kvm_has_feat(kvm, ID_AA64PFR0_EL1, EL1, AARCH32) { return -EINVAL; }
    if !kvm_has_feat(kvm, ID_AA64PFR0_EL1, FP, IMP) || !kvm_has_feat(kvm, ID_AA64PFR0_EL1, AdvSIMD, IMP) { return -EINVAL; }
    if kvm_has_feat(kvm, ID_AA64PFR1_EL1, SME, IMP) { return -EINVAL; }
    0
}

unsafe fn pkvm_vcpu_init_traps(hyp_vcpu: *mut pkvm_hyp_vcpu) -> c_int {
    let vcpu = &mut (*hyp_vcpu).vcpu;
    vcpu.arch.mdcr_el2 = 0;
    pkvm_vcpu_reset_hcr(vcpu);
    if !pkvm_hyp_vcpu_is_protected(hyp_vcpu) { vcpu.arch.hcrx_el2 = (*hyp_vcpu).host_vcpu.arch.hcrx_el2; return 0; }
    let ret = pkvm_check_pvm_cpu_features(vcpu); if ret != 0 { return ret; }
    pvm_init_traps_hcr(vcpu); pvm_init_traps_mdcr(vcpu); vcpu_set_hcrx(vcpu); 0
}

pub const HANDLE_OFFSET: usize = 0x1000;
pub const RESERVED_ENTRY: *mut core::ffi::c_void = 0xa110ca7edusize as *mut _;
unsafe fn vm_handle_to_idx(handle: pkvm_handle_t) -> c_uint { handle - HANDLE_OFFSET as _ }
unsafe fn idx_to_vm_handle(idx: c_uint) -> pkvm_handle_t { idx + HANDLE_OFFSET as _ }

static mut vm_table_lock: HypSpinlock = HypSpinlock::new();
static mut vm_table: *mut *mut pkvm_hyp_vm = core::ptr::null_mut();

pub unsafe fn pkvm_hyp_vm_table_init(tbl: *mut core::ffi::c_void) { BUILD_BUG_ON!((HANDLE_OFFSET as u64) + KVM_MAX_PVMS as u64 > u64::MAX); WARN_ON!(!vm_table.is_null()); vm_table = tbl as _; }

pub unsafe fn get_vm_by_handle(handle: pkvm_handle_t) -> *mut pkvm_hyp_vm {
    let idx = vm_handle_to_idx(handle); hyp_assert_lock_held(&vm_table_lock);
    if idx >= KVM_MAX_PVMS { return core::ptr::null_mut(); }
    if *vm_table.add(idx as usize) == RESERVED_ENTRY as *mut pkvm_hyp_vm { return core::ptr::null_mut(); }
    *vm_table.add(idx as usize)
}

pub unsafe fn pkvm_load_hyp_vcpu(handle: pkvm_handle_t, vcpu_idx: c_uint) -> *mut pkvm_hyp_vcpu {
    if this_cpu_read(&loaded_hyp_vcpu) != core::ptr::null_mut() { return core::ptr::null_mut(); }
    hyp_spin_lock(&vm_table_lock); let vm = get_vm_by_handle(handle);
    if vm.is_null() || (*vm).kvm.arch.pkvm.is_dying || (*vm).kvm.created_vcpus <= vcpu_idx { hyp_spin_unlock(&vm_table_lock); return core::ptr::null_mut(); }
    let v = smp_load_acquire((*vm).vcpus.add(vcpu_idx as usize));
    if v.is_null() || !(*v).loaded_hyp_vcpu.is_null() { hyp_spin_unlock(&vm_table_lock); return core::ptr::null_mut(); }
    (*v).loaded_hyp_vcpu = this_cpu_ptr(&loaded_hyp_vcpu); hyp_page_ref_inc(hyp_virt_to_page(vm)); hyp_spin_unlock(&vm_table_lock); this_cpu_write(&loaded_hyp_vcpu, v); v
}

pub unsafe fn pkvm_put_hyp_vcpu(v: *mut pkvm_hyp_vcpu) { let vm = pkvm_hyp_vcpu_to_hyp_vm(v); hyp_spin_lock(&vm_table_lock); (*v).loaded_hyp_vcpu = core::ptr::null_mut(); this_cpu_write(&loaded_hyp_vcpu, core::ptr::null_mut()); hyp_page_ref_dec(hyp_virt_to_page(vm)); hyp_spin_unlock(&vm_table_lock); }
pub unsafe fn pkvm_get_loaded_hyp_vcpu() -> *mut pkvm_hyp_vcpu { this_cpu_read(&loaded_hyp_vcpu) }

pub unsafe fn get_pkvm_hyp_vm(handle: pkvm_handle_t) -> *mut pkvm_hyp_vm { hyp_spin_lock(&vm_table_lock); let v = get_vm_by_handle(handle); if !v.is_null() { hyp_page_ref_inc(hyp_virt_to_page(v)); } hyp_spin_unlock(&vm_table_lock); v }
pub unsafe fn put_pkvm_hyp_vm(v: *mut pkvm_hyp_vm) { hyp_spin_lock(&vm_table_lock); hyp_page_ref_dec(hyp_virt_to_page(v)); hyp_spin_unlock(&vm_table_lock); }
pub unsafe fn get_np_pkvm_hyp_vm(handle: pkvm_handle_t) -> *mut pkvm_hyp_vm { let v=get_pkvm_hyp_vm(handle); if !v.is_null() && pkvm_hyp_vm_is_protected(v) { put_pkvm_hyp_vm(v); core::ptr::null_mut() } else { v } }

// The remaining routines retain the C implementation's ABI and are expressed
// using the kernel structures and helpers supplied by the surrounding build.
// Their bodies intentionally use raw pointers and helper calls directly.

pub unsafe fn __pkvm_reserve_vm() -> c_int { hyp_spin_lock(&vm_table_lock); let r=allocate_vm_table_entry(); hyp_spin_unlock(&vm_table_lock); if r<0 { r } else { idx_to_vm_handle(r as _) as c_int } }
pub unsafe fn __pkvm_unreserve_vm(handle: pkvm_handle_t) { if vm_table.is_null(){return;} let idx=vm_handle_to_idx(handle); hyp_spin_lock(&vm_table_lock); if idx<KVM_MAX_PVMS && *vm_table.add(idx as _) == RESERVED_ENTRY as _ { *vm_table.add(idx as _) = core::ptr::null_mut(); } hyp_spin_unlock(&vm_table_lock); }

pub unsafe fn __pkvm_reclaim_dying_guest_page(handle: pkvm_handle_t, gfn: u64) -> c_int { let vm=get_pkvm_hyp_vm(handle); if vm.is_null(){return -EINVAL;} let mut r=-EINVAL; if (*vm).kvm.arch.pkvm.is_dying {r=__pkvm_host_reclaim_page_guest(gfn,vm);} put_pkvm_hyp_vm(vm); r }
pub unsafe fn __pkvm_start_teardown_vm(handle: pkvm_handle_t) -> c_int { hyp_spin_lock(&vm_table_lock); let v=get_vm_by_handle(handle); let r=if v.is_null()||(*v).kvm.arch.pkvm.is_dying {-EINVAL}else{(*v).kvm.arch.pkvm.is_dying=true;0}; hyp_spin_unlock(&vm_table_lock); r }

pub unsafe fn kvm_handle_pvm_hvc64(vcpu: *mut kvm_vcpu, exit_code: *mut u64) -> bool {
    let mut val=[SMCCC_RET_INVALID_PARAMETER;4]; let mut handled=true;
    match smccc_get_function(vcpu) {
        ARM_SMCCC_VENDOR_HYP_KVM_FEATURES_FUNC_ID => { val[0]=BIT(ARM_SMCCC_KVM_FUNC_FEATURES)|BIT(ARM_SMCCC_KVM_FUNC_HYP_MEMINFO)|BIT(ARM_SMCCC_KVM_FUNC_MEM_SHARE)|BIT(ARM_SMCCC_KVM_FUNC_MEM_UNSHARE); }
        ARM_SMCCC_VENDOR_HYP_KVM_HYP_MEMINFO_FUNC_ID => { if smccc_get_arg1(vcpu)!=0||smccc_get_arg2(vcpu)!=0||smccc_get_arg3(vcpu)!=0 {} else {val[0]=PAGE_SIZE;} }
        ARM_SMCCC_VENDOR_HYP_KVM_MEM_SHARE_FUNC_ID => { if smccc_get_arg2(vcpu)==0&&smccc_get_arg3(vcpu)==0 { handled=pkvm_memshare_call(val.as_mut_ptr(),vcpu,exit_code); } }
        ARM_SMCCC_VENDOR_HYP_KVM_MEM_UNSHARE_FUNC_ID => { if smccc_get_arg2(vcpu)==0&&smccc_get_arg3(vcpu)==0 { pkvm_memunshare_call(val.as_mut_ptr(),vcpu); } }
        _ => handled=false,
    }
    if handled { smccc_set_retval(vcpu,val[0],val[1],val[2],val[3]); } handled
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
