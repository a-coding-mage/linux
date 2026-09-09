// SPDX-License-Identifier: GPL-2.0

/* Dependencies supplied by the surrounding kernel translation. */

const LBR_SELECT_MASK: u64 = 0x1ff;
const LBR_SELECT_KERNEL: u32 = 0;
const LBR_SELECT_USER: u32 = 1;
const LBR_SELECT_JCC: u32 = 2;
const LBR_SELECT_CALL_NEAR_REL: u32 = 3;
const LBR_SELECT_CALL_NEAR_IND: u32 = 4;
const LBR_SELECT_RET_NEAR: u32 = 5;
const LBR_SELECT_JMP_NEAR_IND: u32 = 6;
const LBR_SELECT_JMP_NEAR_REL: u32 = 7;
const LBR_SELECT_FAR_BRANCH: u32 = 8;

const LBR_KERNEL: u64 = 1 << LBR_SELECT_KERNEL;
const LBR_USER: u64 = 1 << LBR_SELECT_USER;
const LBR_JCC: u64 = 1 << LBR_SELECT_JCC;
const LBR_REL_CALL: u64 = 1 << LBR_SELECT_CALL_NEAR_REL;
const LBR_IND_CALL: u64 = 1 << LBR_SELECT_CALL_NEAR_IND;
const LBR_RETURN: u64 = 1 << LBR_SELECT_RET_NEAR;
const LBR_REL_JMP: u64 = 1 << LBR_SELECT_JMP_NEAR_REL;
const LBR_IND_JMP: u64 = 1 << LBR_SELECT_JMP_NEAR_IND;
const LBR_FAR: u64 = 1 << LBR_SELECT_FAR_BRANCH;
const LBR_NOT_SUPP: i32 = -1;
const LBR_IGNORE: u64 = 0;
const LBR_ANY: u64 = LBR_JCC | LBR_REL_CALL | LBR_IND_CALL | LBR_RETURN | LBR_REL_JMP | LBR_IND_JMP | LBR_FAR;

#[repr(C)]
pub struct BranchEntry { pub from: u64, pub to: u64 }

#[inline(always)] unsafe fn amd_pmu_lbr_set_from(idx: u32, val: u64) { wrmsrq(MSR_AMD_SAMP_BR_FROM + idx * 2, val); }
#[inline(always)] unsafe fn amd_pmu_lbr_set_to(idx: u32, val: u64) { wrmsrq(MSR_AMD_SAMP_BR_FROM + idx * 2 + 1, val); }
#[inline(always)] unsafe fn amd_pmu_lbr_get_from(idx: u32) -> u64 { let mut val = 0; rdmsrq(MSR_AMD_SAMP_BR_FROM + idx * 2, val); val }
#[inline(always)] unsafe fn amd_pmu_lbr_get_to(idx: u32) -> u64 { let mut val = 0; rdmsrq(MSR_AMD_SAMP_BR_FROM + idx * 2 + 1, val); val }

#[inline(always)] unsafe fn sign_ext_branch_ip(ip: u64) -> u64 {
    let shift = 64 - boot_cpu_data.x86_virt_bits;
    (((ip as i64) << shift) >> shift) as u64
}

unsafe fn amd_pmu_lbr_filter() {
    let cpuc = this_cpu_ptr(&cpu_hw_events);
    let br_sel = (*cpuc).br_sel;
    let mut compress = false;
    let fused_only = (br_sel & X86_BR_ALL) == X86_BR_ALL && (br_sel & X86_BR_TYPE_SAVE) != X86_BR_TYPE_SAVE;
    let mut i = 0;
    while i < (*cpuc).lbr_stack.nr {
        let from = (*cpuc).lbr_entries.add(i).read().from;
        let to = (*cpuc).lbr_entries.add(i).read().to;
        let mut offset = 0;
        let typ = branch_type_fused(from, to, 0, &mut offset);
        if offset != 0 {
            (*cpuc).lbr_entries.add(i).as_mut().unwrap().from += offset;
            if fused_only { i += 1; continue; }
        }
        if typ == X86_BR_NONE || (br_sel & typ) != typ || ((br_sel & X86_BR_KERNEL) == 0 && kernel_ip((*cpuc).lbr_entries.add(i).read().from)) {
            (*cpuc).lbr_entries.add(i).as_mut().unwrap().from = 0;
            compress = true;
        }
        if (br_sel & X86_BR_TYPE_SAVE) == X86_BR_TYPE_SAVE { (*cpuc).lbr_entries.add(i).as_mut().unwrap().typ = common_branch_type(typ); }
        i += 1;
    }
    if !compress { return; }
    i = 0;
    while i < (*cpuc).lbr_stack.nr {
        if (*cpuc).lbr_entries.add(i).read().from == 0 {
            let mut j = i;
            while { j += 1; j < (*cpuc).lbr_stack.nr } { *(*cpuc).lbr_entries.add(j - 1) = (*cpuc).lbr_entries.add(j).read(); }
            (*cpuc).lbr_stack.nr -= 1;
            if (*cpuc).lbr_entries.add(i).read().from == 0 { continue; }
        }
        i += 1;
    }
}

static LBR_SPEC_MAP: [i32; PERF_BR_SPEC_MAX as usize] = [PERF_BR_SPEC_NA, PERF_BR_SPEC_WRONG_PATH, PERF_BR_NON_SPEC_CORRECT_PATH, PERF_BR_SPEC_CORRECT_PATH];

pub unsafe fn amd_pmu_lbr_read() {
    let cpuc = this_cpu_ptr(&cpu_hw_events);
    if (*cpuc).lbr_users == 0 { return; }
    let mut out = 0;
    for i in 0..x86_pmu.lbr_nr {
        let from = amd_pmu_lbr_get_from(i);
        let to = amd_pmu_lbr_get_to(i);
        let valid = (to >> 63) & 1;
        let spec = (to >> 62) & 1;
        let reserved = (to >> 61) & 1;
        if (valid == 0 && spec == 0) || reserved != 0 { continue; }
        let br = (*cpuc).lbr_entries.add(out);
        perf_clear_branch_entry_bitfields(br);
        (*br).from = sign_ext_branch_ip(from & ((1u64 << 58) - 1));
        (*br).to = sign_ext_branch_ip(to & ((1u64 << 58) - 1));
        (*br).mispred = (from >> 63) & 1;
        (*br).predicted = !(*br).mispred;
        (*br).spec = LBR_SPEC_MAP[((valid << 1) | spec) as usize];
        out += 1;
    }
    (*cpuc).lbr_stack.nr = out;
    (*cpuc).lbr_stack.hw_idx = 0;
    amd_pmu_lbr_filter();
}

static LBR_SELECT_MAP: [i64; PERF_SAMPLE_BRANCH_MAX_SHIFT as usize] = [
    LBR_USER as i64, LBR_KERNEL as i64, LBR_IGNORE as i64, LBR_ANY as i64,
    (LBR_REL_CALL | LBR_IND_CALL | LBR_FAR) as i64, (LBR_RETURN | LBR_FAR) as i64,
    LBR_IND_CALL as i64, LBR_NOT_SUPP as i64, LBR_NOT_SUPP as i64, LBR_NOT_SUPP as i64,
    LBR_JCC as i64, LBR_NOT_SUPP as i64, LBR_IND_JMP as i64, LBR_REL_CALL as i64,
    LBR_NOT_SUPP as i64, LBR_NOT_SUPP as i64,
];

unsafe fn amd_pmu_lbr_setup_filter(event: *mut perf_event) -> i32 {
    let reg = &mut (*event).hw.branch_reg;
    let br_type = (*event).attr.branch_sample_type;
    let mut mask = 0u64;
    if x86_pmu.lbr_nr == 0 { return -EOPNOTSUPP; }
    if br_type & PERF_SAMPLE_BRANCH_USER != 0 { mask |= X86_BR_USER; }
    if br_type & PERF_SAMPLE_BRANCH_KERNEL != 0 { mask |= X86_BR_KERNEL; }
    if br_type & PERF_SAMPLE_BRANCH_ANY != 0 { mask |= X86_BR_ANY; }
    if br_type & PERF_SAMPLE_BRANCH_ANY_CALL != 0 { mask |= X86_BR_ANY_CALL; }
    if br_type & PERF_SAMPLE_BRANCH_ANY_RETURN != 0 { mask |= X86_BR_RET | X86_BR_IRET | X86_BR_SYSRET; }
    if br_type & PERF_SAMPLE_BRANCH_IND_CALL != 0 { mask |= X86_BR_IND_CALL; }
    if br_type & PERF_SAMPLE_BRANCH_COND != 0 { mask |= X86_BR_JCC; }
    if br_type & PERF_SAMPLE_BRANCH_IND_JUMP != 0 { mask |= X86_BR_IND_JMP; }
    if br_type & PERF_SAMPLE_BRANCH_CALL != 0 { mask |= X86_BR_CALL | X86_BR_ZERO_CALL; }
    if br_type & PERF_SAMPLE_BRANCH_TYPE_SAVE != 0 { mask |= X86_BR_TYPE_SAVE; }
    reg.reg = mask;
    mask = 0;
    for i in 0..PERF_SAMPLE_BRANCH_MAX_SHIFT { if br_type & (1u64 << i) == 0 { continue; } let v = LBR_SELECT_MAP[i as usize]; if v == LBR_NOT_SUPP { return -EOPNOTSUPP; } if v != LBR_IGNORE as i64 { mask |= v as u64; } }
    reg.config = mask ^ LBR_SELECT_MASK;
    0
}

pub unsafe fn amd_pmu_lbr_hw_config(event: *mut perf_event) -> i32 { let ret = amd_pmu_lbr_setup_filter(event); if ret == 0 { (*event).attach_state |= PERF_ATTACH_SCHED_CB; } ret }

pub unsafe fn amd_pmu_lbr_reset() { let cpuc = this_cpu_ptr(&cpu_hw_events); if x86_pmu.lbr_nr == 0 { return; } for i in 0..x86_pmu.lbr_nr { amd_pmu_lbr_set_from(i, 0); amd_pmu_lbr_set_to(i, 0); } (*cpuc).last_task_ctx = core::ptr::null_mut(); (*cpuc).last_log_id = 0; wrmsrq(MSR_AMD64_LBR_SELECT, 0); }

pub unsafe fn amd_pmu_lbr_add(event: *mut perf_event) { let cpuc = this_cpu_ptr(&cpu_hw_events); if x86_pmu.lbr_nr == 0 { return; } if has_branch_stack(event) { (*cpuc).lbr_select = 1; (*cpuc).lbr_sel.config = (*event).hw.branch_reg.config; (*cpuc).br_sel = (*event).hw.branch_reg.reg; } perf_sched_cb_inc((*event).pmu); if (*cpuc).lbr_users == 0 && (*event).total_time_running == 0 { amd_pmu_lbr_reset(); } (*cpuc).lbr_users += 1; }

pub unsafe fn amd_pmu_lbr_del(event: *mut perf_event) { let cpuc = this_cpu_ptr(&cpu_hw_events); if x86_pmu.lbr_nr == 0 { return; } if has_branch_stack(event) { (*cpuc).lbr_select = 0; } (*cpuc).lbr_users -= 1; WARN_ON_ONCE((*cpuc).lbr_users < 0); perf_sched_cb_dec((*event).pmu); }

pub unsafe fn amd_pmu_lbr_sched_task(_pmu_ctx: *mut perf_event_pmu_context, _task: *mut task_struct, sched_in: bool) { let cpuc = this_cpu_ptr(&cpu_hw_events); if (*cpuc).lbr_users != 0 && sched_in { amd_pmu_lbr_reset(); } }

pub unsafe fn amd_pmu_lbr_enable_all() { let cpuc = this_cpu_ptr(&cpu_hw_events); if (*cpuc).lbr_users == 0 || x86_pmu.lbr_nr == 0 { return; } if (*cpuc).lbr_select != 0 { wrmsrq(MSR_AMD64_LBR_SELECT, (*cpuc).lbr_sel.config & LBR_SELECT_MASK); } if cpu_feature_enabled(X86_FEATURE_AMD_LBR_PMC_FREEZE) { let mut dbg_ctl = 0; rdmsrq(MSR_IA32_DEBUGCTLMSR, dbg_ctl); wrmsrq(MSR_IA32_DEBUGCTLMSR, dbg_ctl | DEBUGCTLMSR_FREEZE_LBRS_ON_PMI); } let mut dbg_extn_cfg = 0; rdmsrq(MSR_AMD_DBG_EXTN_CFG, dbg_extn_cfg); wrmsrq(MSR_AMD_DBG_EXTN_CFG, dbg_extn_cfg | DBG_EXTN_CFG_LBRV2EN); }
pub unsafe fn amd_pmu_lbr_disable_all() { let cpuc = this_cpu_ptr(&cpu_hw_events); if (*cpuc).lbr_users != 0 && x86_pmu.lbr_nr != 0 { __amd_pmu_lbr_disable(); } }

pub unsafe fn amd_pmu_lbr_init() -> i32 { if x86_pmu.version < 2 || !boot_cpu_has(X86_FEATURE_AMD_LBR_V2) { return -EOPNOTSUPP; } let ebx = cpuid_ebx(EXT_PERFMON_DEBUG_FEATURES); x86_pmu.lbr_nr = (ebx >> 0) & 0xff; pr_cont("%d-deep LBR, ", x86_pmu.lbr_nr); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
