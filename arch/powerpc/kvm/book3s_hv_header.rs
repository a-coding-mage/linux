// SPDX-License-Identifier: GPL-2.0-only

/*
 * Privileged (non-hypervisor) host registers to save.
 *
 * Dependency: asm/guest-state-buffer.h
 */

#[repr(C)]
pub struct p9_host_os_sprs {
    pub iamr: u64,
    pub amr: u64,
    pub pmc1: u32,
    pub pmc2: u32,
    pub pmc3: u32,
    pub pmc4: u32,
    pub pmc5: u32,
    pub pmc6: u32,
    pub mmcr0: u64,
    pub mmcr1: u64,
    pub mmcr2: u64,
    pub mmcr3: u64,
    pub mmcra: u64,
    pub siar: u64,
    pub sier1: u64,
    pub sier2: u64,
    pub sier3: u64,
    pub sdar: u64,
}

#[inline]
pub unsafe fn nesting_enabled(kvm: *mut kvm) -> bool {
    (*kvm).arch.nested_enable && kvm_is_radix(kvm)
}

extern "C" {
    pub fn load_vcpu_state(vcpu: *mut kvm_vcpu, host_os_sprs: *mut p9_host_os_sprs) -> bool;
    pub fn store_vcpu_state(vcpu: *mut kvm_vcpu);
    pub fn save_p9_host_os_sprs(host_os_sprs: *mut p9_host_os_sprs);
    pub fn restore_p9_host_os_sprs(vcpu: *mut kvm_vcpu, host_os_sprs: *mut p9_host_os_sprs);
    pub fn switch_pmu_to_guest(vcpu: *mut kvm_vcpu, host_os_sprs: *mut p9_host_os_sprs);
    pub fn switch_pmu_to_host(vcpu: *mut kvm_vcpu, host_os_sprs: *mut p9_host_os_sprs);
}

// CONFIG_KVM_BOOK3S_HV_P9_TIMING selects the timing implementation at build time.
#[cfg(CONFIG_KVM_BOOK3S_HV_P9_TIMING)]
extern "C" {
    pub fn accumulate_time(vcpu: *mut kvm_vcpu, next: *mut kvmhv_tb_accumulator);
}

#[cfg(CONFIG_KVM_BOOK3S_HV_P9_TIMING)]
#[inline]
pub unsafe fn start_timing(vcpu: *mut kvm_vcpu, next: *mut kvmhv_tb_accumulator) {
    accumulate_time(vcpu, next);
}

#[cfg(CONFIG_KVM_BOOK3S_HV_P9_TIMING)]
#[inline]
pub unsafe fn end_timing(vcpu: *mut kvm_vcpu) {
    accumulate_time(vcpu, core::ptr::null_mut());
}

#[cfg(not(CONFIG_KVM_BOOK3S_HV_P9_TIMING))]
#[inline]
pub unsafe fn accumulate_time(_vcpu: *mut kvm_vcpu, _next: *mut kvmhv_tb_accumulator) {}

#[cfg(not(CONFIG_KVM_BOOK3S_HV_P9_TIMING))]
#[inline]
pub unsafe fn start_timing(_vcpu: *mut kvm_vcpu, _next: *mut kvmhv_tb_accumulator) {}

#[cfg(not(CONFIG_KVM_BOOK3S_HV_P9_TIMING))]
#[inline]
pub unsafe fn end_timing(_vcpu: *mut kvm_vcpu) {}

#[inline]
pub unsafe fn __kvmppc_set_msr_hv(vcpu: *mut kvm_vcpu, val: u64) {
    (*vcpu).arch.shregs.msr = val;
    kvmhv_nestedv2_mark_dirty(vcpu, KVMPPC_GSID_MSR);
}

#[inline]
pub unsafe fn __kvmppc_get_msr_hv(vcpu: *mut kvm_vcpu) -> u64 {
    WARN_ON(kvmhv_nestedv2_cached_reload(vcpu, KVMPPC_GSID_MSR) < 0);
    (*vcpu).arch.shregs.msr
}

macro_rules! KVMPPC_BOOK3S_HV_VCPU_ACCESSOR {
    ($set:ident, $get:ident, $reg:ident, $ty:ty, $iden:expr) => {
        #[inline]
        pub unsafe fn $set(vcpu: *mut kvm_vcpu, val: $ty) {
            (*vcpu).arch.$reg = val;
            kvmhv_nestedv2_mark_dirty(vcpu, $iden);
        }
        #[inline]
        pub unsafe fn $get(vcpu: *mut kvm_vcpu) -> $ty {
            kvmhv_nestedv2_cached_reload(vcpu, $iden);
            (*vcpu).arch.$reg
        }
    };
}

macro_rules! KVMPPC_BOOK3S_HV_VCPU_ARRAY_ACCESSOR {
    ($set:ident, $get:ident, $reg:ident, $ty:ty, $iden:expr) => {
        #[inline]
        pub unsafe fn $set(vcpu: *mut kvm_vcpu, i: i32, val: $ty) {
            (*vcpu).arch.$reg[i as usize] = val;
            kvmhv_nestedv2_mark_dirty(vcpu, $iden(i));
        }
        #[inline]
        pub unsafe fn $get(vcpu: *mut kvm_vcpu, i: i32) -> $ty {
            WARN_ON(kvmhv_nestedv2_cached_reload(vcpu, $iden(i)) < 0);
            (*vcpu).arch.$reg[i as usize]
        }
    };
}

KVMPPC_BOOK3S_HV_VCPU_ACCESSOR!(kvmppc_set_mmcra_hv, kvmppc_get_mmcra_hv, mmcra, u64, KVMPPC_GSID_MMCRA);
KVMPPC_BOOK3S_HV_VCPU_ACCESSOR!(kvmppc_set_hfscr_hv, kvmppc_get_hfscr_hv, hfscr, u64, KVMPPC_GSID_HFSCR);
KVMPPC_BOOK3S_HV_VCPU_ACCESSOR!(kvmppc_set_fscr_hv, kvmppc_get_fscr_hv, fscr, u64, KVMPPC_GSID_FSCR);
KVMPPC_BOOK3S_HV_VCPU_ACCESSOR!(kvmppc_set_dscr_hv, kvmppc_get_dscr_hv, dscr, u64, KVMPPC_GSID_DSCR);
KVMPPC_BOOK3S_HV_VCPU_ACCESSOR!(kvmppc_set_purr_hv, kvmppc_get_purr_hv, purr, u64, KVMPPC_GSID_PURR);
KVMPPC_BOOK3S_HV_VCPU_ACCESSOR!(kvmppc_set_spurr_hv, kvmppc_get_spurr_hv, spurr, u64, KVMPPC_GSID_SPURR);
KVMPPC_BOOK3S_HV_VCPU_ACCESSOR!(kvmppc_set_amr_hv, kvmppc_get_amr_hv, amr, u64, KVMPPC_GSID_AMR);
KVMPPC_BOOK3S_HV_VCPU_ACCESSOR!(kvmppc_set_uamor_hv, kvmppc_get_uamor_hv, uamor, u64, KVMPPC_GSID_UAMOR);
KVMPPC_BOOK3S_HV_VCPU_ACCESSOR!(kvmppc_set_siar_hv, kvmppc_get_siar_hv, siar, u64, KVMPPC_GSID_SIAR);
KVMPPC_BOOK3S_HV_VCPU_ACCESSOR!(kvmppc_set_sdar_hv, kvmppc_get_sdar_hv, sdar, u64, KVMPPC_GSID_SDAR);
KVMPPC_BOOK3S_HV_VCPU_ACCESSOR!(kvmppc_set_iamr_hv, kvmppc_get_iamr_hv, iamr, u64, KVMPPC_GSID_IAMR);
KVMPPC_BOOK3S_HV_VCPU_ACCESSOR!(kvmppc_set_dawr0_hv, kvmppc_get_dawr0_hv, dawr0, u64, KVMPPC_GSID_DAWR0);
KVMPPC_BOOK3S_HV_VCPU_ACCESSOR!(kvmppc_set_dawr1_hv, kvmppc_get_dawr1_hv, dawr1, u64, KVMPPC_GSID_DAWR1);
KVMPPC_BOOK3S_HV_VCPU_ACCESSOR!(kvmppc_set_dawrx0_hv, kvmppc_get_dawrx0_hv, dawrx0, u64, KVMPPC_GSID_DAWRX0);
KVMPPC_BOOK3S_HV_VCPU_ACCESSOR!(kvmppc_set_dawrx1_hv, kvmppc_get_dawrx1_hv, dawrx1, u64, KVMPPC_GSID_DAWRX1);
KVMPPC_BOOK3S_HV_VCPU_ACCESSOR!(kvmppc_set_dexcr_hv, kvmppc_get_dexcr_hv, dexcr, u64, KVMPPC_GSID_DEXCR);
KVMPPC_BOOK3S_HV_VCPU_ACCESSOR!(kvmppc_set_hashkeyr_hv, kvmppc_get_hashkeyr_hv, hashkeyr, u64, KVMPPC_GSID_HASHKEYR);
KVMPPC_BOOK3S_HV_VCPU_ACCESSOR!(kvmppc_set_hashpkeyr_hv, kvmppc_get_hashpkeyr_hv, hashpkeyr, u64, KVMPPC_GSID_HASHPKEYR);
KVMPPC_BOOK3S_HV_VCPU_ACCESSOR!(kvmppc_set_ciabr_hv, kvmppc_get_ciabr_hv, ciabr, u64, KVMPPC_GSID_CIABR);
KVMPPC_BOOK3S_HV_VCPU_ACCESSOR!(kvmppc_set_wort_hv, kvmppc_get_wort_hv, wort, u64, KVMPPC_GSID_WORT);
KVMPPC_BOOK3S_HV_VCPU_ACCESSOR!(kvmppc_set_ppr_hv, kvmppc_get_ppr_hv, ppr, u64, KVMPPC_GSID_PPR);
KVMPPC_BOOK3S_HV_VCPU_ACCESSOR!(kvmppc_set_ctrl_hv, kvmppc_get_ctrl_hv, ctrl, u64, KVMPPC_GSID_CTRL);

KVMPPC_BOOK3S_HV_VCPU_ARRAY_ACCESSOR!(kvmppc_set_mmcr_hv, kvmppc_get_mmcr_hv, mmcr, u64, KVMPPC_GSID_MMCR);
KVMPPC_BOOK3S_HV_VCPU_ARRAY_ACCESSOR!(kvmppc_set_sier_hv, kvmppc_get_sier_hv, sier, u64, KVMPPC_GSID_SIER);
KVMPPC_BOOK3S_HV_VCPU_ARRAY_ACCESSOR!(kvmppc_set_pmc_hv, kvmppc_get_pmc_hv, pmc, u32, KVMPPC_GSID_PMC);
KVMPPC_BOOK3S_HV_VCPU_ACCESSOR!(kvmppc_set_pspb_hv, kvmppc_get_pspb_hv, pspb, u32, KVMPPC_GSID_PSPB);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
