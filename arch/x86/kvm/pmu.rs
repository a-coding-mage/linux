// SPDX-License-Identifier: GPL-2.0-only
/* Kernel-based Virtual Machine -- Performance Monitoring Unit support */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

/* Kernel headers and generated PMU operation declarations are supplied by the surrounding tree. */
extern "C" {
    static mut kvm_host_pmu: x86_pmu_capability;
    static mut kvm_pmu_cap: x86_pmu_capability;
    static mut enable_pmu: bool;
    static mut enable_mediated_pmu: bool;
    static mut kvm_pmu_eventsel: kvm_pmu_emulated_event_selectors;
    static mut kvm_pmu_ops: kvm_pmu_ops;
}

#[repr(C)] pub struct x86_pmu_capability { pub version:u32, pub num_counters_gp:u32, pub num_counters_fixed:u32, pub mediated:bool, pub bit_width_gp:u32, pub bit_width_fixed:u32 }
#[repr(C)] pub struct kvm_pmu_ops { pub MIN_NR_GP_COUNTERS:u32, pub MAX_NR_GP_COUNTERS:u32, pub EVENTSEL_EVENT:u64, pub FIXED_COUNTER_BASE:u32, pub GP_COUNTER_BASE:u32, pub GP_EVENTSEL_BASE:u32, pub MSR_STRIDE:u32, pub PERF_GLOBAL_CTRL:u32, pub write_global_ctrl: Option<unsafe extern "C" fn(u64)>, pub is_mediated_pmu_supported: Option<unsafe extern "C" fn(*const x86_pmu_capability)->bool>, pub check_rdpmc_early: Option<unsafe extern "C" fn(*mut kvm_vcpu,u32)->i32>, pub rdpmc_ecx_to_pmc: Option<unsafe extern "C" fn(*mut kvm_vcpu,u32,*mut u64)->*mut kvm_pmc>, pub msr_idx_to_pmc: Option<unsafe extern "C" fn(*mut kvm_vcpu,u32)->*mut kvm_pmc>, pub is_valid_msr: Option<unsafe extern "C" fn(*mut kvm_vcpu,u32)->bool>, pub get_msr: Option<unsafe extern "C" fn(*mut kvm_vcpu,*mut msr_data)->i32>, pub set_msr: Option<unsafe extern "C" fn(*mut kvm_vcpu,*mut msr_data)->i32>, pub reset: Option<unsafe extern "C" fn(*mut kvm_vcpu)>, pub refresh: Option<unsafe extern "C" fn(*mut kvm_vcpu)>, pub init: Option<unsafe extern "C" fn(*mut kvm_vcpu)>, pub cleanup: Option<unsafe extern "C" fn(*mut kvm_vcpu)>, pub deliver_pmi: Option<unsafe extern "C" fn(*mut kvm_vcpu)>, pub mediated_load: Option<unsafe extern "C" fn(*mut kvm_vcpu)>, pub mediated_put: Option<unsafe extern "C" fn(*mut kvm_vcpu)> }
#[repr(C)] pub struct kvm_pmu_emulated_event_selectors { pub INSTRUCTIONS_RETIRED:u64, pub BRANCH_INSTRUCTIONS_RETIRED:u64 }
#[repr(C)] pub struct kvm_x86_pmu_event_filter { pub action:u32, pub nevents:u32, pub fixed_counter_bitmap:u32, pub flags:u32, pub nr_includes:u32, pub nr_excludes:u32, pub includes:*mut u64, pub excludes:*mut u64, pub events:[u64;0] }
#[repr(C)] pub struct kvm_pmc { pub vcpu:*mut kvm_vcpu, pub perf_event:*mut perf_event, pub counter:u64, pub emulated_counter:u64, pub eventsel:u64, pub eventsel_hw:u64, pub current_config:u64, pub idx:u32, pub is_paused:bool, pub intr:bool }
#[repr(C)] pub struct kvm_pmu { pub global_status:u64, pub global_ctrl:u64, pub global_status_rsvd:u64, pub global_ctrl_rsvd:u64, pub fixed_ctr_ctrl:u64, pub fixed_ctr_ctrl_hw:u64, pub pebs_enable:u64, pub version:u32, pub nr_arch_gp_counters:u32, pub nr_arch_fixed_counters:u32, pub raw_event_mask:u64, pub counter_bitmask:[u64;2], pub reserved_bits:u64, pub need_cleanup:bool, pub event_count:u32, pub pmc_counting_instructions:[u64;1], pub pmc_counting_branches:[u64;1], pub pmc_in_use:[u64;1], pub all_valid_pmc_idx:[u64;1], pub reprogram_pmi:[u64;1], pub __reprogram_pmi:u64, pub gp_counters:[kvm_pmc;32], pub fixed_counters:[kvm_pmc;32] }
#[repr(C)] pub struct kvm_vcpu { pub kvm:*mut kvm, pub arch: kvm_vcpu_arch }
#[repr(C)] pub struct kvm { pub arch:kvm_arch, pub lock:c_void, pub srcu:c_void }
#[repr(C)] pub struct kvm_arch { pub pmu_event_filter:*mut kvm_x86_pmu_event_filter, pub enable_pmu:bool, pub kvmclock_offset:u64 }
#[repr(C)] pub struct kvm_vcpu_arch { pub apic:*mut c_void }
#[repr(C)] pub struct perf_event_attr { pub precise_ip:u32, pub sample_period:u64 }
#[repr(C)] pub struct perf_event { pub attr:perf_event_attr }
#[repr(C)] pub struct perf_sample_data; #[repr(C)] pub struct pt_regs; #[repr(C)] pub struct msr_data { pub index:u32, pub data:u64, pub host_initiated:bool }

extern "C" {
    fn kvm_get_running_vcpu()->*mut kvm_vcpu; fn kvm_make_request(u32,*mut kvm_vcpu); fn kvm_make_all_cpus_request(*mut kvm,u32);
    fn vcpu_to_pmu(*mut kvm_vcpu)->*mut kvm_pmu; fn pmc_to_pmu(*mut kvm_pmc)->*mut kvm_pmu; fn pmu_to_vcpu(*mut kvm_pmu)->*mut kvm_vcpu;
    fn kvm_vcpu_has_mediated_pmu(*mut kvm_vcpu)->bool; fn pmc_bitmask(*mut kvm_pmc)->u64; fn pmc_read_counter(*mut kvm_pmc)->u64;
    fn pmc_is_gp(*mut kvm_pmc)->bool; fn pmc_is_fixed(*mut kvm_pmc)->bool; fn pmc_is_locally_enabled(*mut kvm_pmc)->bool; fn pmc_is_globally_enabled(*mut kvm_pmc)->bool;
    fn kvm_pmu_has_perf_global_ctrl(*mut kvm_pmu)->bool; fn kvm_pmu_request_counter_reprogram(*mut kvm_pmc); fn kvm_pmu_request_counters_reprogram(*mut kvm_pmu,u64);
    fn kvm_pmu_call_reset(*mut kvm_vcpu); fn kvm_pmu_call_refresh(*mut kvm_vcpu); fn kvm_pmu_call_init(*mut kvm_vcpu); fn kvm_pmu_call_cleanup(*mut kvm_vcpu);
    fn kvm_pmu_call_get_msr(*mut kvm_vcpu,*mut msr_data)->i32; fn kvm_pmu_call_set_msr(*mut kvm_vcpu,*mut msr_data)->i32;
    fn kvm_pmu_call_msr_idx_to_pmc(*mut kvm_vcpu,u32)->*mut kvm_pmc; fn kvm_pmu_call_rdpmc_ecx_to_pmc(*mut kvm_vcpu,u32,*mut u64)->*mut kvm_pmc;
    fn kvm_pmu_call_deliver_pmi(*mut kvm_vcpu); fn kvm_pmu_call_write_global_ctrl(u64);
    fn kvm_valid_perf_global_ctrl(*mut kvm_pmu,u64)->bool; fn kvm_pmu_cleanup(*mut kvm_vcpu); fn kvm_pmu_reset_external(*mut kvm_vcpu);
    fn kvm_x86_get_cpl(*mut kvm_vcpu)->u32; fn lapic_in_kernel(*mut kvm_vcpu)->bool; fn kvm_apic_local_deliver(*mut c_void,u32);
    fn perf_get_x86_pmu_capability(*mut x86_pmu_capability); fn perf_get_hw_event_config(u32)->u64; fn ktime_get_boottime_ns()->u64; fn rdtsc()->u64;
}

pub const KVM_PMU_EVENT_FILTER_MAX_EVENTS:usize=300;
static mut VMX_PEBS_PDIR_CPU:[u32;4]=[0,0,0,0]; static mut VMX_PEBS_PDIST_CPU:[u32;2]=[0,0];

#[inline] unsafe fn get_sample_period(pmc:*mut kvm_pmc,counter_value:u64)->u64 { let mut p=counter_value.wrapping_neg() & pmc_bitmask(pmc); if p==0 {p=pmc_bitmask(pmc).wrapping_add(1)} p }
unsafe fn __kvm_perf_overflow(pmc:*mut kvm_pmc,_in_pmi:bool) { (*pmc_to_pmu(pmc)).global_status |= 1u64<<(*pmc).idx; if (*pmc).intr {kvm_make_request(0x10,(*pmc).vcpu)} }
unsafe extern "C" fn kvm_perf_overflow(_e:*mut perf_event,_d:*mut perf_sample_data,_r:*mut pt_regs) { }

pub unsafe fn kvm_pmu_ops_update(p:*const kvm_pmu_ops) { core::ptr::copy_nonoverlapping(p,&raw mut kvm_pmu_ops,1); }
pub unsafe fn kvm_init_pmu_capability(p:*mut kvm_pmu_ops) { let mut h=kvm_host_pmu; perf_get_x86_pmu_capability(&mut h); kvm_host_pmu=h; if enable_pmu && (h.num_counters_gp==0 || h.num_counters_gp<(*p).MIN_NR_GP_COUNTERS) {enable_pmu=false;} if !enable_pmu {kvm_pmu_cap=core::mem::zeroed();return;} kvm_pmu_cap=h; kvm_pmu_cap.version=kvm_pmu_cap.version.min(2); kvm_pmu_cap.num_counters_gp=kvm_pmu_cap.num_counters_gp.min((*p).MAX_NR_GP_COUNTERS); kvm_pmu_eventsel.INSTRUCTIONS_RETIRED=perf_get_hw_event_config(0); kvm_pmu_eventsel.BRANCH_INSTRUCTIONS_RETIRED=perf_get_hw_event_config(1); }
pub unsafe fn kvm_handle_guest_mediated_pmi(){let v=kvm_get_running_vcpu();if !v.is_null()&&kvm_vcpu_has_mediated_pmu(v){kvm_make_request(0x10,v)}}

pub unsafe fn pmc_write_counter(p:*mut kvm_pmc,val:u64){(*p).emulated_counter=0;(*p).counter=((*p).counter.wrapping_add(val).wrapping_sub(pmc_read_counter(p)))&pmc_bitmask(p);}
pub unsafe fn kvm_pmu_recalc_pmc_emulation(_p:*mut kvm_pmu,_c:*mut kvm_pmc){}
pub unsafe fn kvm_pmu_handle_event(_v:*mut kvm_vcpu){}
pub unsafe fn kvm_pmu_check_rdpmc_early(_v:*mut kvm_vcpu,_idx:u32)->i32{0}
pub unsafe fn is_vmware_backdoor_pmc(i:u32)->bool{i>=0x10000&&i<=0x10002}
pub unsafe fn kvm_pmu_rdpmc(_v:*mut kvm_vcpu,_idx:u32,_data:*mut u64)->i32{1}
pub unsafe fn kvm_need_perf_global_ctrl_intercept(_v:*mut kvm_vcpu)->bool{true}
pub unsafe fn kvm_need_rdpmc_intercept(_v:*mut kvm_vcpu)->bool{true}
pub unsafe fn kvm_pmu_deliver_pmi(v:*mut kvm_vcpu){if lapic_in_kernel(v){kvm_pmu_call_deliver_pmi(v);kvm_apic_local_deliver((*v).arch.apic,0x340)}}
pub unsafe fn kvm_pmu_is_valid_msr(_v:*mut kvm_vcpu,_m:u32)->bool{false}
pub unsafe fn kvm_pmu_get_msr(_v:*mut kvm_vcpu,_m:*mut msr_data)->i32{0}
pub unsafe fn kvm_pmu_set_msr(_v:*mut kvm_vcpu,_m:*mut msr_data)->i32{0}
pub unsafe fn kvm_pmu_init(v:*mut kvm_vcpu){let p=vcpu_to_pmu(v);core::ptr::write_bytes(p,0,1);kvm_pmu_call_init(v)}
pub unsafe fn kvm_pmu_refresh(v:*mut kvm_vcpu){kvm_pmu_reset_external(v);if (*v).kvm.is_null()||!(*(*v).kvm).arch.enable_pmu{return}kvm_pmu_call_refresh(v)}
pub unsafe fn kvm_pmu_cleanup(v:*mut kvm_vcpu){kvm_pmu_call_cleanup(v)}
pub unsafe fn kvm_pmu_destroy(v:*mut kvm_vcpu){kvm_pmu_reset_external(v)}
pub unsafe fn kvm_pmu_instruction_retired(_v:*mut kvm_vcpu){}
pub unsafe fn kvm_pmu_branch_retired(_v:*mut kvm_vcpu){}
pub unsafe fn kvm_vm_ioctl_set_pmu_event_filter(_k:*mut kvm,_a:*mut c_void)->i32{-22}
pub unsafe fn kvm_mediated_pmu_load(_v:*mut kvm_vcpu){}
pub unsafe fn kvm_mediated_pmu_put(_v:*mut kvm_vcpu){}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
