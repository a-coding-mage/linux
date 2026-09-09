// SPDX-License-Identifier: GPL-2.0-only
/* KVM PMU support for Intel CPUs */

// Kernel headers and build-time configuration are supplied by surrounding dependencies.

const INTEL_RDPMC_GP: u32 = 0;
const INTEL_RDPMC_FIXED: u32 = INTEL_PMC_FIXED_RDPMC_BASE;
const INTEL_RDPMC_TYPE_MASK: u32 = genmask(31, 16);
const INTEL_RDPMC_INDEX_MASK: u32 = genmask(15, 0);
const MSR_PMC_FULL_WIDTH_BIT: u32 = MSR_IA32_PMC0 - MSR_IA32_PERFCTR0;

unsafe fn vcpu_to_lbr_desc(vcpu: *mut kvm_vcpu) -> *mut lbr_desc {
    if is_td_vcpu(vcpu) { return core::ptr::null_mut(); }
    &mut (*to_vmx(vcpu)).lbr_desc
}
unsafe fn vcpu_to_lbr_records(vcpu: *mut kvm_vcpu) -> *mut x86_pmu_lbr {
    if is_td_vcpu(vcpu) { return core::ptr::null_mut(); }
    &mut (*to_vmx(vcpu)).lbr_desc.records
}

unsafe fn reprogram_fixed_counters(pmu: *mut kvm_pmu, data: u64) {
    let old_fixed_ctr_ctrl = (*pmu).fixed_ctr_ctrl_hw;
    (*pmu).fixed_ctr_ctrl = data;
    (*pmu).fixed_ctr_ctrl_hw = data;
    for i in 0..(*pmu).nr_arch_fixed_counters {
        let new_ctrl = fixed_ctrl_field(data, i);
        let old_ctrl = fixed_ctrl_field(old_fixed_ctr_ctrl, i);
        if old_ctrl == new_ctrl { continue; }
        let pmc = get_fixed_pmc(pmu, MSR_CORE_PERF_FIXED_CTR0 + i);
        set_bit(KVM_FIXED_PMC_BASE_IDX + i, (*pmu).pmc_in_use);
        kvm_pmu_request_counter_reprogram(pmc);
    }
}

unsafe fn intel_rdpmc_ecx_to_pmc(vcpu: *mut kvm_vcpu, mut idx: u32, mask: *mut u64) -> *mut kvm_pmc {
    let typ = idx & INTEL_RDPMC_TYPE_MASK;
    let pmu = vcpu_to_pmu(vcpu);
    if warn_on_once((*pmu).version == 0) { return core::ptr::null_mut(); }
    let (counters, num, bitmask) = match typ {
        INTEL_RDPMC_FIXED => ((*pmu).fixed_counters, (*pmu).nr_arch_fixed_counters, (*pmu).counter_bitmask[KVM_PMC_FIXED]),
        INTEL_RDPMC_GP => ((*pmu).gp_counters, (*pmu).nr_arch_gp_counters, (*pmu).counter_bitmask[KVM_PMC_GP]),
        _ => return core::ptr::null_mut(),
    };
    idx &= INTEL_RDPMC_INDEX_MASK;
    if idx >= num { return core::ptr::null_mut(); }
    *mask &= bitmask;
    &mut counters[array_index_nospec(idx, num)]
}

unsafe fn get_fw_gp_pmc(pmu: *mut kvm_pmu, msr: u32) -> *mut kvm_pmc {
    if !fw_writes_is_enabled(pmu_to_vcpu(pmu)) { return core::ptr::null_mut(); }
    get_gp_pmc(pmu, msr, MSR_IA32_PMC0)
}
unsafe fn intel_pmu_lbr_is_compatible(vcpu: *mut kvm_vcpu) -> bool { !is_td_vcpu(vcpu) && cpuid_model_is_consistent(vcpu) }
pub unsafe fn intel_pmu_lbr_is_enabled(vcpu: *mut kvm_vcpu) -> bool { !is_td_vcpu(vcpu) && (*vcpu_to_lbr_records(vcpu)).nr != 0 }
unsafe fn intel_pmu_is_valid_lbr_msr(vcpu: *mut kvm_vcpu, index: u32) -> bool {
    let records = vcpu_to_lbr_records(vcpu);
    if !intel_pmu_lbr_is_enabled(vcpu) { return false; }
    let mut ret = index == MSR_LBR_SELECT || index == MSR_LBR_TOS ||
        (index >= (*records).from && index < (*records).from + (*records).nr) ||
        (index >= (*records).to && index < (*records).to + (*records).nr);
    if !ret && (*records).info != 0 { ret = index >= (*records).info && index < (*records).info + (*records).nr; }
    ret
}

unsafe fn intel_is_valid_msr(vcpu: *mut kvm_vcpu, msr: u32) -> bool {
    let pmu = vcpu_to_pmu(vcpu);
    match msr {
        MSR_CORE_PERF_FIXED_CTR_CTRL => kvm_pmu_has_perf_global_ctrl(pmu),
        MSR_IA32_PEBS_ENABLE => vcpu_get_perf_capabilities(vcpu) & PERF_CAP_PEBS_FORMAT != 0,
        MSR_IA32_DS_AREA => guest_cpu_cap_has(vcpu, X86_FEATURE_DS),
        MSR_PEBS_DATA_CFG => { let p = vcpu_get_perf_capabilities(vcpu); p & PERF_CAP_PEBS_BASELINE != 0 && p & PERF_CAP_PEBS_FORMAT > 3 },
        _ => !get_gp_pmc(pmu, msr, MSR_IA32_PERFCTR0).is_null() || !get_gp_pmc(pmu, msr, MSR_P6_EVNTSEL0).is_null() || !get_fixed_pmc(pmu, msr).is_null() || !get_fw_gp_pmc(pmu, msr).is_null() || intel_pmu_is_valid_lbr_msr(vcpu, msr),
    }
}
unsafe fn intel_msr_idx_to_pmc(vcpu: *mut kvm_vcpu, msr: u32) -> *mut kvm_pmc {
    let pmu = vcpu_to_pmu(vcpu);
    let mut p = get_fixed_pmc(pmu, msr); if p.is_null() { p = get_gp_pmc(pmu, msr, MSR_P6_EVNTSEL0); } if p.is_null() { p = get_gp_pmc(pmu, msr, MSR_IA32_PERFCTR0); } p
}

unsafe fn intel_pmu_release_guest_lbr_event(vcpu: *mut kvm_vcpu) {
    let d = vcpu_to_lbr_desc(vcpu); if d.is_null() { return; }
    if !(*d).event.is_null() { perf_event_release_kernel((*d).event); (*d).event = core::ptr::null_mut(); (*vcpu_to_pmu(vcpu)).event_count -= 1; }
}

pub unsafe fn intel_pmu_create_guest_lbr_event(vcpu: *mut kvm_vcpu) -> i32 {
    let d = vcpu_to_lbr_desc(vcpu); let pmu = vcpu_to_pmu(vcpu);
    if warn_on_once(d.is_null()) { return 0; }
    if !(*d).event.is_null() { set_bit(INTEL_PMC_IDX_FIXED_VLBR, (*pmu).pmc_in_use); return 0; }
    let attr = perf_event_attr { type_: PERF_TYPE_RAW, size: core::mem::size_of::<perf_event_attr>() as u32, config: INTEL_FIXED_VLBR_EVENT, sample_type: PERF_SAMPLE_BRANCH_STACK, pinned: true, exclude_host: true, branch_sample_type: PERF_SAMPLE_BRANCH_CALL_STACK | PERF_SAMPLE_BRANCH_USER, ..core::mem::zeroed() };
    let event = perf_event_create_kernel_counter(&attr, -1, current, core::ptr::null_mut(), core::ptr::null_mut());
    if is_err(event) { pr_debug_ratelimited("{}: failed {}\n", "intel_pmu_create_guest_lbr_event", ptr_err(event)); return ptr_err(event) as i32; }
    (*d).event = event; (*pmu).event_count += 1; set_bit(INTEL_PMC_IDX_FIXED_VLBR, (*pmu).pmc_in_use); 0
}

unsafe fn intel_pmu_handle_lbr_msrs_access(vcpu: *mut kvm_vcpu, info: *mut msr_data, read: bool) -> bool {
    let d = vcpu_to_lbr_desc(vcpu); let index = (*info).index;
    if !intel_pmu_is_valid_lbr_msr(vcpu, index) { return false; }
    if (*d).event.is_null() && intel_pmu_create_guest_lbr_event(vcpu) < 0 { if read { (*info).data = 0; } return true; }
    local_irq_disable();
    if (*(*d).event).state == PERF_EVENT_STATE_ACTIVE { let mut err = 0; if read { rdmsrq(index, &mut (*info).data); } else { err = wrmsrq_safe(index, (*info).data); } set_bit(INTEL_PMC_IDX_FIXED_VLBR, (*vcpu_to_pmu(vcpu)).pmc_in_use); local_irq_enable(); return err == 0; }
    clear_bit(INTEL_PMC_IDX_FIXED_VLBR, (*vcpu_to_pmu(vcpu)).pmc_in_use); local_irq_enable(); if read { (*info).data = 0; } true
}

unsafe fn intel_pmu_get_msr(vcpu: *mut kvm_vcpu, i: *mut msr_data) -> i32 {
    let p=vcpu_to_pmu(vcpu); let m=(*i).index;
    match m { MSR_CORE_PERF_FIXED_CTR_CTRL=>(*i).data=(*p).fixed_ctr_ctrl, MSR_IA32_PEBS_ENABLE=>(*i).data=(*p).pebs_enable, MSR_IA32_DS_AREA=>(*i).data=(*p).ds_area, MSR_PEBS_DATA_CFG=>(*i).data=(*p).pebs_data_cfg,
    _=>{let mut c=get_gp_pmc(p,m,MSR_IA32_PERFCTR0); if c.is_null(){c=get_gp_pmc(p,m,MSR_IA32_PMC0);} if !c.is_null(){(*i).data=pmc_read_counter(c)&(*p).counter_bitmask[KVM_PMC_GP];}else if !get_fixed_pmc(p,m).is_null(){let c=get_fixed_pmc(p,m);(*i).data=pmc_read_counter(c)&(*p).counter_bitmask[KVM_PMC_FIXED];}else if !get_gp_pmc(p,m,MSR_P6_EVNTSEL0).is_null(){(*i).data=(*get_gp_pmc(p,m,MSR_P6_EVNTSEL0)).eventsel;}else if !intel_pmu_handle_lbr_msrs_access(vcpu,i,true){return 1;}}} 0
}
unsafe fn intel_pmu_set_msr(vcpu:*mut kvm_vcpu,i:*mut msr_data)->i32{let p=vcpu_to_pmu(vcpu);let m=(*i).index;let mut d=(*i).data;match m{MSR_CORE_PERF_FIXED_CTR_CTRL=>{if d&(*p).fixed_ctr_ctrl_rsvd!=0{return 1}if (*p).fixed_ctr_ctrl!=d{reprogram_fixed_counters(p,d)}},MSR_IA32_PEBS_ENABLE=>{if d&(*p).pebs_enable_rsvd!=0{return 1}let x=(*p).pebs_enable^d;(*p).pebs_enable=d;if x!=0{kvm_pmu_request_counters_reprogram(p,d)}},MSR_IA32_DS_AREA=>{if is_noncanonical_msr_address(d,vcpu){return 1}(*p).ds_area=d},MSR_PEBS_DATA_CFG=>{if d&(*p).pebs_data_cfg_rsvd!=0{return 1}(*p).pebs_data_cfg=d},_=>{let mut c=get_gp_pmc(p,m,MSR_IA32_PERFCTR0);if c.is_null(){c=get_gp_pmc(p,m,MSR_IA32_PMC0);}if !c.is_null(){if m&MSR_PMC_FULL_WIDTH_BIT!=0&&d&!(*p).counter_bitmask[KVM_PMC_GP]!=0{return 1}if !(*i).host_initiated&&m&MSR_PMC_FULL_WIDTH_BIT==0{d=(d as i32) as i64 as u64}pmc_write_counter(c,d)}else if !get_fixed_pmc(p,m).is_null(){pmc_write_counter(get_fixed_pmc(p,m),d)}else if !get_gp_pmc(p,m,MSR_P6_EVNTSEL0).is_null(){let c=get_gp_pmc(p,m,MSR_P6_EVNTSEL0);if d&(*p).reserved_bits!=0{return 1}(*c).eventsel=d;(*c).eventsel_hw=d;kvm_pmu_request_counter_reprogram(c)}else if !intel_pmu_handle_lbr_msrs_access(vcpu,i,false){return 1}}}0}
unsafe fn intel_get_fixed_pmc_eventsel(index:usize)->u64{let a=[PERF_COUNT_HW_INSTRUCTIONS,PERF_COUNT_HW_CPU_CYCLES,PERF_COUNT_HW_REF_CPU_CYCLES];perf_get_hw_event_config(a[index])}
unsafe fn intel_pmu_enable_fixed_counter_bits(p:*mut kvm_pmu,b:u64){for i in 0..(*p).nr_arch_fixed_counters{(*p).fixed_ctr_ctrl_rsvd&=!intel_fixed_bits_by_idx(i,b)}}
unsafe fn intel_pmu_refresh(v:*mut kvm_vcpu){let p=vcpu_to_pmu(v);let d=vcpu_to_lbr_desc(v);if d.is_null(){return}core::ptr::write_bytes(&mut (*d).records,0,1);let e=kvm_find_cpuid_entry(v,0xa);if e.is_null(){return}let eax=(*e).eax;let edx=(*e).edx;(*p).version=cpuid10_version(eax);if (*p).version==0{return}(*p).nr_arch_gp_counters=min(cpuid10_num(eax),kvm_pmu_cap.num_counters_gp);(*p).counter_bitmask[KVM_PMC_GP]=(1u64<<min(cpuid10_width(eax),kvm_pmu_cap.bit_width_gp))-1;(*p).available_event_types=!(*e).ebx&((1u64<<min(cpuid10_mask(eax),kvm_pmu_cap.events_mask_len))-1);if intel_pmu_lbr_is_compatible(v)&&vcpu_get_perf_capabilities(v)&PERF_CAP_LBR_FMT!=0{(*d).records=vmx_lbr_caps}if (*d).records.nr!=0{bitmap_set((*p).all_valid_pmc_idx,INTEL_PMC_IDX_FIXED_VLBR,1)}if (*p).version==1{return}(*p).nr_arch_fixed_counters=min(cpuid10_fixed_num(edx),kvm_pmu_cap.num_counters_fixed);(*p).counter_bitmask[KVM_PMC_FIXED]=(1u64<<min(cpuid10_fixed_width(edx),kvm_pmu_cap.bit_width_fixed))-1;intel_pmu_enable_fixed_counter_bits(p,INTEL_FIXED_0_KERNEL|INTEL_FIXED_0_USER|INTEL_FIXED_0_ENABLE_PMI)}
unsafe fn intel_pmu_init(v:*mut kvm_vcpu){let p=vcpu_to_pmu(v);for i in 0..KVM_MAX_NR_INTEL_GP_COUNTERS{(*p).gp_counters[i].type_=KVM_PMC_GP;(*p).gp_counters[i].vcpu=v;(*p).gp_counters[i].idx=i}for i in 0..KVM_MAX_NR_INTEL_FIXED_COUNTERS{(*p).fixed_counters[i].type_=KVM_PMC_FIXED;(*p).fixed_counters[i].vcpu=v;(*p).fixed_counters[i].idx=i+KVM_FIXED_PMC_BASE_IDX;(*p).fixed_counters[i].eventsel=intel_get_fixed_pmc_eventsel(i)}let d=vcpu_to_lbr_desc(v);if !d.is_null(){(*d).records.nr=0;(*d).event=core::ptr::null_mut();(*d).msr_passthrough=false}}
unsafe fn intel_pmu_reset(v:*mut kvm_vcpu){intel_pmu_release_guest_lbr_event(v)}
unsafe fn intel_pmu_deliver_pmi(v:*mut kvm_vcpu){if intel_pmu_lbr_is_enabled(v)&&vcpu_to_pmu(v).as_ref().unwrap().version>1{intel_pmu_legacy_freezing_lbrs_on_pmi(v)}}
unsafe fn intel_pmu_legacy_freezing_lbrs_on_pmi(v:*mut kvm_vcpu){let mut d=vmx_guest_debugctl_read();if d&DEBUGCTLMSR_FREEZE_LBRS_ON_PMI!=0{d&=!DEBUGCTLMSR_LBR;vmx_guest_debugctl_write(v,d)}}
unsafe fn intel_pmu_cleanup(v:*mut kvm_vcpu){if vmx_guest_debugctl_read()&DEBUGCTLMSR_LBR==0{intel_pmu_release_guest_lbr_event(v)}}
unsafe fn vmx_update_intercept_for_lbr_msrs(v:*mut kvm_vcpu,set:bool){let l=vcpu_to_lbr_records(v);for i in 0..(*l).nr{vmx_set_intercept_for_msr(v,(*l).from+i,MSR_TYPE_RW,set);vmx_set_intercept_for_msr(v,(*l).to+i,MSR_TYPE_RW,set);if (*l).info!=0{vmx_set_intercept_for_msr(v,(*l).info+i,MSR_TYPE_RW,set)}}vmx_set_intercept_for_msr(v,MSR_LBR_SELECT,MSR_TYPE_RW,set);vmx_set_intercept_for_msr(v,MSR_LBR_TOS,MSR_TYPE_RW,set)}
pub unsafe fn vmx_passthrough_lbr_msrs(v:*mut kvm_vcpu){let d=vcpu_to_lbr_desc(v);if d.is_null(){return}if (*d).event.is_null(){vmx_update_intercept_for_lbr_msrs(v,true);return}if (*(*d).event).state<PERF_EVENT_STATE_ACTIVE{vmx_update_intercept_for_lbr_msrs(v,true);clear_bit(INTEL_PMC_IDX_FIXED_VLBR,(*vcpu_to_pmu(v)).pmc_in_use)}else{vmx_update_intercept_for_lbr_msrs(v,false)}}
pub unsafe fn intel_pmu_cross_mapped_check(p:*mut kvm_pmu){let mut c=core::ptr::null_mut();let mut bit=0;while kvm_for_each_pmc(p,&mut c,&mut bit){if pmc_is_locally_enabled(c)&&pmc_is_globally_enabled(c)&&!(*c).perf_event.is_null(){let h=(*(*c).perf_event).hw.idx;if h!=(*c).idx&&h>-1{(*p).host_cross_mapped_mask|=1u64<<h}}}}
unsafe fn intel_pmu_is_mediated_pmu_supported(h:*mut x86_pmu_capability)->bool{let mut cap=0;if boot_cpu_has(X86_FEATURE_PDCM){rdmsrq(MSR_IA32_PERF_CAPABILITIES,&mut cap)}(*h).version>=4&&cap&PERF_CAP_FW_WRITES!=0&&cpu_has_load_perf_global_ctrl()}
unsafe fn intel_pmu_write_global_ctrl(x:u64){vmcs_write64(GUEST_IA32_PERF_GLOBAL_CTRL,x)}
unsafe fn intel_mediated_pmu_load(v:*mut kvm_vcpu){let p=vcpu_to_pmu(v);let mut s=0;rdmsrq(MSR_CORE_PERF_GLOBAL_STATUS,&mut s);let t=(*p).global_status^s;if s&t!=0{wrmsrq(MSR_CORE_PERF_GLOBAL_OVF_CTRL,s&t)}if (*p).global_status&t!=0{wrmsrq(MSR_CORE_PERF_GLOBAL_STATUS_SET,(*p).global_status&t)}wrmsrq(MSR_CORE_PERF_FIXED_CTR_CTRL,(*p).fixed_ctr_ctrl_hw)}
unsafe fn intel_mediated_pmu_put(v:*mut kvm_vcpu){let p=vcpu_to_pmu(v);rdmsrq(MSR_CORE_PERF_GLOBAL_STATUS,&mut (*p).global_status);if (*p).global_status!=0{wrmsrq(MSR_CORE_PERF_GLOBAL_OVF_CTRL,(*p).global_status)}if (*p).fixed_ctr_ctrl_hw!=0{wrmsrq(MSR_CORE_PERF_FIXED_CTR_CTRL,0)}}
pub static mut intel_pmu_ops:kvm_pmu_ops=kvm_pmu_ops{rdpmc_ecx_to_pmc:Some(intel_rdpmc_ecx_to_pmc),msr_idx_to_pmc:Some(intel_msr_idx_to_pmc),is_valid_msr:Some(intel_is_valid_msr),get_msr:Some(intel_pmu_get_msr),set_msr:Some(intel_pmu_set_msr),refresh:Some(intel_pmu_refresh),init:Some(intel_pmu_init),reset:Some(intel_pmu_reset),deliver_pmi:Some(intel_pmu_deliver_pmi),cleanup:Some(intel_pmu_cleanup),is_mediated_pmu_supported:Some(intel_pmu_is_mediated_pmu_supported),mediated_load:Some(intel_mediated_pmu_load),mediated_put:Some(intel_mediated_pmu_put),write_global_ctrl:Some(intel_pmu_write_global_ctrl),EVENTSEL_EVENT:ARCH_PERFMON_EVENTSEL_EVENT,MAX_NR_GP_COUNTERS:KVM_MAX_NR_INTEL_GP_COUNTERS,MIN_NR_GP_COUNTERS:1,PERF_GLOBAL_CTRL:MSR_CORE_PERF_GLOBAL_CTRL,GP_EVENTSEL_BASE:MSR_P6_EVNTSEL0,GP_COUNTER_BASE:MSR_IA32_PMC0,FIXED_COUNTER_BASE:MSR_CORE_PERF_FIXED_CTR0,MSR_STRIDE:1};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
