// SPDX-License-Identifier: GPL-2.0
/*
 * Implement support for AMD Fam19h Branch Sampling feature
 * Based on specifications published in AMD PPR Fam19 Model 01
 *
 * Copyright 2021 Google LLC
 * Contributed by Stephane Eranian <eranian@google.com>
 */

const BRS_POISON: u64 = 0xFFFFFFFFFFFFFFFE; // mark limit of valid entries

#[repr(C)]
pub union AmdDebugExtnCfg {
    pub val: u64,
}

impl AmdDebugExtnCfg {
    #[inline]
    fn brsmen(&self) -> u64 { unsafe { self.val } >> 2 & 1 }
    #[inline]
    fn set_brsmen(&mut self, v: u64) { unsafe { self.val = (self.val & !(1 << 2)) | ((v & 1) << 2); } }
    #[inline]
    fn vb(&self) -> u64 { unsafe { self.val } >> 5 & 1 }
    #[inline]
    fn msroff(&self) -> u64 { unsafe { self.val } >> 16 & 0xf }
}

#[inline]
fn brs_from(idx: i32) -> u32 { MSR_AMD_SAMP_BR_FROM + 2 * idx as u32 }

#[inline]
fn brs_to(idx: i32) -> u32 { MSR_AMD_SAMP_BR_FROM + 2 * idx as u32 + 1 }

#[inline(always)]
unsafe fn set_debug_extn_cfg(val: u64) { native_wrmsrq(MSR_AMD_DBG_EXTN_CFG, val | (3u64 << 3)); }

#[inline(always)]
unsafe fn get_debug_extn_cfg() -> u64 { native_rdmsrq(MSR_AMD_DBG_EXTN_CFG) }

unsafe fn amd_brs_detect() -> bool {
    if !cpu_feature_enabled(X86_FEATURE_BRS) { return false; }
    match boot_cpu_data.x86 {
        0x19 => {
            x86_pmu.lbr_nr = 16;
            x86_pmu.lbr_sel_map = core::ptr::null_mut();
            x86_pmu.lbr_sel_mask = 0;
        }
        _ => return false,
    }
    true
}

unsafe fn amd_brs_setup_filter(event: *mut perf_event) -> i32 {
    let typ = (*event).attr.branch_sample_type;
    if x86_pmu.lbr_nr == 0 { return -EOPNOTSUPP; }
    if (typ & !PERF_SAMPLE_BRANCH_PLM_ALL) != PERF_SAMPLE_BRANCH_ANY { return -EINVAL; }
    0
}

#[inline]
unsafe fn amd_is_brs_event(e: *mut perf_event) -> bool {
    ((*e).hw.config & AMD64_RAW_EVENT_MASK) == AMD_FAM19H_BRS_EVENT
}

pub unsafe fn amd_brs_hw_config(event: *mut perf_event) -> i32 {
    if !is_sampling_event(event) || !amd_is_brs_event(event) || (*event).attr.freq ||
       (*event).attr.sample_period <= x86_pmu.lbr_nr { return -EINVAL; }
    let ret = amd_brs_setup_filter(event);
    if ret == 0 { (*event).hw.flags |= PERF_X86_EVENT_AMD_BRS; }
    ret
}

#[inline]
unsafe fn amd_brs_get_tos(cfg: *const AmdDebugExtnCfg) -> i32 {
    let off = (*cfg).msroff();
    (if off != 0 { off } else { x86_pmu.lbr_nr as u64 }) as i32 - 1
}

pub unsafe fn amd_brs_reset() {
    if !cpu_feature_enabled(X86_FEATURE_BRS) { return; }
    set_debug_extn_cfg(0);
    wrmsrq(brs_to(0), BRS_POISON);
}

pub unsafe fn amd_brs_init() -> i32 {
    if !amd_brs_detect() { return -EOPNOTSUPP; }
    pr_cont!("{}-deep BRS, ", x86_pmu.lbr_nr);
    0
}

pub unsafe fn amd_brs_enable() {
    let cpuc = this_cpu_ptr(&cpu_hw_events);
    if { (*cpuc).brs_active += 1; (*cpuc).brs_active } > 1 { return; }
    let mut cfg = AmdDebugExtnCfg { val: 0 };
    cfg.set_brsmen(1);
    set_debug_extn_cfg(cfg.val);
}

pub unsafe fn amd_brs_enable_all() {
    let cpuc = this_cpu_ptr(&cpu_hw_events);
    if (*cpuc).lbr_users != 0 { amd_brs_enable(); }
}

pub unsafe fn amd_brs_disable() {
    let cpuc = this_cpu_ptr(&cpu_hw_events);
    if (*cpuc).brs_active == 0 { return; }
    (*cpuc).brs_active -= 1;
    if (*cpuc).brs_active != 0 { return; }
    let mut cfg = AmdDebugExtnCfg { val: get_debug_extn_cfg() };
    if cfg.brsmen() != 0 { cfg.set_brsmen(0); set_debug_extn_cfg(cfg.val); }
}

pub unsafe fn amd_brs_disable_all() {
    let cpuc = this_cpu_ptr(&cpu_hw_events);
    if (*cpuc).lbr_users != 0 { amd_brs_disable(); }
}

unsafe fn amd_brs_match_plm(event: *mut perf_event, from: u64, to: u64) -> bool {
    let typ = (*event).attr.branch_sample_type;
    let plm_k = PERF_SAMPLE_BRANCH_KERNEL | PERF_SAMPLE_BRANCH_HV;
    let plm_u = PERF_SAMPLE_BRANCH_USER;
    if (typ & plm_k) == 0 && (kernel_ip(to) || kernel_ip(from)) { return false; }
    if (typ & plm_u) == 0 && !kernel_ip(to) { return false; }
    true
}

pub unsafe fn amd_brs_drain() {
    let cpuc = this_cpu_ptr(&cpu_hw_events);
    let event = (*cpuc).events[0];
    let br = (*cpuc).lbr_entries;
    let mut nr: u32 = 0;
    if event.is_null() { (*cpuc).lbr_stack.nr = 0; return; }
    let cfg = AmdDebugExtnCfg { val: get_debug_extn_cfg() };
    if cfg.msroff() >= x86_pmu.lbr_nr as u64 || cfg.vb() == 0 { (*cpuc).lbr_stack.nr = 0; return; }
    let tos = amd_brs_get_tos(&cfg) as u32;
    let shift = 64 - boot_cpu_data.x86_virt_bits;
    for i in 0..=tos {
        let idx = tos - i;
        let mut to: u64 = 0;
        rdmsrq(brs_to(idx as i32), &mut to);
        if to == BRS_POISON { break; }
        to = (((to as i64) << shift) >> shift) as u64;
        let mut from: u64 = 0;
        rdmsrq(brs_from(idx as i32), &mut from);
        if !amd_brs_match_plm(event, from, to) { continue; }
        perf_clear_branch_entry_bitfields(br.add(nr as usize));
        (*br.add(nr as usize)).from = from;
        (*br.add(nr as usize)).to = to;
        nr += 1;
    }
    (*cpuc).lbr_stack.nr = nr;
}

unsafe fn amd_brs_poison_buffer() {
    let cfg = AmdDebugExtnCfg { val: get_debug_extn_cfg() };
    wrmsrq(brs_to(amd_brs_get_tos(&cfg)), BRS_POISON);
}

pub unsafe fn amd_pmu_brs_sched_task(_pmu_ctx: *mut perf_event_pmu_context, _task: *mut task_struct, sched_in: bool) {
    let cpuc = this_cpu_ptr(&cpu_hw_events);
    if (*cpuc).lbr_users == 0 { return; }
    if sched_in { amd_brs_poison_buffer(); }
}

pub unsafe fn perf_amd_brs_lopwr_cb(lopwr_in: bool) {
    let cpuc = this_cpu_ptr(&cpu_hw_events);
    if (*cpuc).brs_active != 0 {
        let mut cfg = AmdDebugExtnCfg { val: get_debug_extn_cfg() };
        cfg.set_brsmen((!lopwr_in) as u64);
        set_debug_extn_cfg(cfg.val);
    }
}

// DEFINE_STATIC_CALL_NULL(perf_lopwr_cb, perf_amd_brs_lopwr_cb);
// EXPORT_STATIC_CALL_TRAMP_GPL(perf_lopwr_cb);
pub unsafe fn amd_brs_lopwr_init() {
    static_call_update!(perf_lopwr_cb, perf_amd_brs_lopwr_cb);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
