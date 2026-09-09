// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Copyright (C) 2014 ARM Limited
 */

// Kernel dependencies are supplied by the surrounding Rust translation.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum InsnEmulationMode { INSN_UNDEF, INSN_EMULATE, INSN_HW }

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum LegacyInsnStatus { INSN_DEPRECATED, INSN_OBSOLETE, INSN_UNAVAILABLE }

#[repr(C)]
struct InsnEmulation {
    name: *const c_char,
    status: LegacyInsnStatus,
    try_emulate: Option<unsafe extern "C" fn(*mut pt_regs, u32) -> bool>,
    set_hw_mode: Option<unsafe extern "C" fn(bool) -> c_int>,
    current_mode: c_int,
    min: c_int,
    max: c_int,
    sysctl: ctl_table,
}

const ARM_OPCODE_CONDTEST_FAIL: u32 = 0;
const ARM_OPCODE_CONDTEST_PASS: u32 = 1;
const ARM_OPCODE_CONDTEST_UNCOND: u32 = 2;
const ARM_OPCODE_CONDITION_UNCOND: u32 = 0xf;

unsafe fn aarch32_check_condition(opcode: u32, psr: u32) -> u32 {
    let cc_bits = opcode >> 28;
    if cc_bits != ARM_OPCODE_CONDITION_UNCOND {
        if (aarch32_opcode_cond_checks[cc_bits as usize])(psr) {
            ARM_OPCODE_CONDTEST_PASS
        } else { ARM_OPCODE_CONDTEST_FAIL }
    } else { ARM_OPCODE_CONDTEST_UNCOND }
}

#[cfg(CONFIG_SWP_EMULATION)]
const TYPE_SWPB: u32 = 1 << 22;

#[cfg(CONFIG_SWP_EMULATION)]
unsafe fn emulate_swpX(address: u32, data: *mut u32, type_: u32) -> c_int {
    let mut res: u32 = 0;
    if type_ != TYPE_SWPB && address & 3 != 0 {
        pr_debug!("SWP instruction on unaligned pointer!\n");
        return -EFAULT;
    }
    loop {
        let mut temp: c_ulong = 0;
        let mut temp2: c_ulong = 0;
        // The C implementation uses an ldxr/stxr LL/SC loop with privileged
        // uaccess and exception-table fixups; those kernel primitives remain external.
        if type_ == TYPE_SWPB {
            __user_swpb_asm!(*data, address, res, temp, temp2);
        } else {
            __user_swp_asm!(*data, address, res, temp, temp2);
        }
        if res as c_int != -EAGAIN || signal_pending(current) { break; }
        cond_resched();
    }
    res as c_int
}

#[cfg(CONFIG_SWP_EMULATION)]
unsafe fn swp_handler(regs: *mut pt_regs, instr: u32) -> c_int {
    let mut destreg: u32;
    let mut data: u32;
    let type_: u32 = instr & TYPE_SWPB;
    let mut address: u32 = 0;
    let mut res: c_int = 0;
    perf_sw_event(PERF_COUNT_SW_EMULATION_FAULTS, 1, regs, (*regs).pc);
    match aarch32_check_condition(instr, (*regs).pstate) {
        ARM_OPCODE_CONDTEST_PASS => (),
        ARM_OPCODE_CONDTEST_FAIL => goto_ret!(),
        ARM_OPCODE_CONDTEST_UNCOND => return -EFAULT,
        _ => return -EINVAL,
    }
    let rn = aarch32_insn_extract_reg_num(instr, A32_RN_OFFSET);
    let rt2 = aarch32_insn_extract_reg_num(instr, A32_RT2_OFFSET);
    address = (*regs).user_regs.regs[rn as usize] as u32;
    data = (*regs).user_regs.regs[rt2 as usize] as u32;
    destreg = aarch32_insn_extract_reg_num(instr, A32_RT_OFFSET);
    let user_ptr = (address & !3) as *const c_void;
    if !access_ok(user_ptr, 4) { goto_fault!(); }
    res = emulate_swpX(address, &mut data, type_);
    if res == -EFAULT { goto_fault!(); }
    else if res == 0 { (*regs).user_regs.regs[destreg as usize] = data as u64; }
    if type_ == TYPE_SWPB { trace_instruction_emulation("swpb", (*regs).pc); }
    else { trace_instruction_emulation("swp", (*regs).pc); }
    pr_warn_ratelimited!("obsolete SWP instruction at 0x{:x}\n", (*regs).pc);
    arm64_skip_faulting_instruction(regs, 4); return 0;
}

#[cfg(CONFIG_SWP_EMULATION)]
unsafe extern "C" fn try_emulate_swp(regs: *mut pt_regs, insn: u32) -> bool {
    if !compat_user_mode(regs) || compat_thumb_mode(regs) || (insn & 0x0fb00ff0) != 0x01000090 { return false; }
    swp_handler(regs, insn) == 0
}

#[cfg(CONFIG_SWP_EMULATION)]
static mut insn_swp: InsnEmulation = InsnEmulation { name: b"swp\0".as_ptr() as _, status: LegacyInsnStatus::INSN_OBSOLETE, try_emulate: Some(try_emulate_swp), set_hw_mode: None, current_mode: 0, min: 0, max: 0, sysctl: unsafe { core::mem::zeroed() } };

#[cfg(CONFIG_CP15_BARRIER_EMULATION)]
unsafe extern "C" fn cp15_barrier_set_hw_mode(enable: bool) -> c_int {
    if enable { sysreg_clear_set(sctlr_el1, 0, SCTLR_EL1_CP15BEN); }
    else { sysreg_clear_set(sctlr_el1, SCTLR_EL1_CP15BEN, 0); }
    0
}

#[cfg(CONFIG_CP15_BARRIER_EMULATION)]
unsafe extern "C" fn try_emulate_cp15_barrier(regs: *mut pt_regs, insn: u32) -> bool {
    if !compat_user_mode(regs) || compat_thumb_mode(regs) { return false; }
    if (insn & 0x0fff0fdf) == 0x0e070f9a || (insn & 0x0fff0fff) == 0x0e070f95 {
        return cp15barrier_handler(regs, insn) == 0;
    }
    false
}

#[cfg(CONFIG_CP15_BARRIER_EMULATION)]
unsafe fn cp15barrier_handler(regs: *mut pt_regs, instr: u32) -> c_int {
    perf_sw_event(PERF_COUNT_SW_EMULATION_FAULTS, 1, regs, (*regs).pc);
    match aarch32_check_condition(instr, (*regs).pstate) {
        ARM_OPCODE_CONDTEST_PASS => (),
        ARM_OPCODE_CONDTEST_FAIL => (),
        ARM_OPCODE_CONDTEST_UNCOND => return -EFAULT,
        _ => return -EINVAL,
    }
    match aarch32_insn_mcr_extract_crm(instr) {
        10 => { if aarch32_insn_mcr_extract_opc2(instr) == 5 { dmb(sy); trace_instruction_emulation("mcr p15, 0, Rt, c7, c10, 5 ; dmb", (*regs).pc); } else { dsb(sy); trace_instruction_emulation("mcr p15, 0, Rt, c7, c10, 4 ; dsb", (*regs).pc); } }
        5 => trace_instruction_emulation("mcr p15, 0, Rt, c7, c5, 4 ; isb", (*regs).pc),
        _ => (),
    }
    pr_warn_ratelimited!("deprecated CP15 Barrier instruction at 0x{:x}\n", (*regs).pc);
    arm64_skip_faulting_instruction(regs, 4); 0
}

#[cfg(CONFIG_CP15_BARRIER_EMULATION)]
static mut insn_cp15_barrier: InsnEmulation = InsnEmulation { name: b"cp15_barrier\0".as_ptr() as _, status: LegacyInsnStatus::INSN_DEPRECATED, try_emulate: Some(try_emulate_cp15_barrier), set_hw_mode: Some(cp15_barrier_set_hw_mode), current_mode: 0, min: 0, max: 0, sysctl: unsafe { core::mem::zeroed() } };

#[cfg(CONFIG_SETEND_EMULATION)]
unsafe extern "C" fn setend_set_hw_mode(enable: bool) -> c_int {
    if !cpu_supports_mixed_endian_el0() { return -EINVAL; }
    if enable { sysreg_clear_set(sctlr_el1, SCTLR_EL1_SED, 0); }
    else { sysreg_clear_set(sctlr_el1, 0, SCTLR_EL1_SED); }
    0
}

#[cfg(CONFIG_SETEND_EMULATION)]
unsafe fn compat_setend_handler(regs: *mut pt_regs, big_endian: u32) -> c_int {
    perf_sw_event(PERF_COUNT_SW_EMULATION_FAULTS, 1, regs, (*regs).pc);
    let insn = if big_endian != 0 { (*regs).pstate |= PSR_AA32_E_BIT; "setend be" } else { (*regs).pstate &= !PSR_AA32_E_BIT; "setend le" };
    trace_instruction_emulation(insn, (*regs).pc); pr_warn_ratelimited!("deprecated setend instruction at 0x{:x}\n", (*regs).pc); 0
}
#[cfg(CONFIG_SETEND_EMULATION)] unsafe fn a32_setend_handler(r: *mut pt_regs, i: u32) -> c_int { let rc=compat_setend_handler(r,(i>>9)&1); arm64_skip_faulting_instruction(r,4); rc }
#[cfg(CONFIG_SETEND_EMULATION)] unsafe fn t16_setend_handler(r: *mut pt_regs, i: u32) -> c_int { let rc=compat_setend_handler(r,(i>>3)&1); arm64_skip_faulting_instruction(r,2); rc }
#[cfg(CONFIG_SETEND_EMULATION)] unsafe extern "C" fn try_emulate_setend(r: *mut pt_regs, i: u32) -> bool { if compat_thumb_mode(r)&&(i&0xfffffff7)==0x0000b650{return t16_setend_handler(r,i)==0} if compat_user_mode(r)&&(i&0xfffffdff)==0xf1010000{return a32_setend_handler(r,i)==0} false }
#[cfg(CONFIG_SETEND_EMULATION)] static mut insn_setend: InsnEmulation = InsnEmulation { name:b"setend\0".as_ptr() as _, status:LegacyInsnStatus::INSN_DEPRECATED, try_emulate:Some(try_emulate_setend), set_hw_mode:Some(setend_set_hw_mode), current_mode:0,min:0,max:0,sysctl:unsafe{core::mem::zeroed()} };

// The following initialization/registration logic is kept in source order;
// kernel table and CPU-hotplug helpers are provided by the surrounding tree.
unsafe fn register_insn_emulation(insn: *mut InsnEmulation) { (*insn).min=0; match (*insn).status { LegacyInsnStatus::INSN_DEPRECATED=>{(*insn).current_mode=1;(*insn).max=2}, LegacyInsnStatus::INSN_OBSOLETE=>{(*insn).current_mode=0;(*insn).max=1}, LegacyInsnStatus::INSN_UNAVAILABLE=>{(*insn).current_mode=0;(*insn).max=0} } }

pub unsafe extern "C" fn try_emulate_armv8_deprecated(regs: *mut pt_regs, insn: u32) -> bool {
    for ie in insn_emulations.iter() { if (**ie).status != LegacyInsnStatus::INSN_UNAVAILABLE && (**ie).current_mode != 0 { if let Some(f)=(**ie).try_emulate { if f(regs,insn){return true;} } } } false
}

#[cfg(any(CONFIG_SWP_EMULATION, CONFIG_CP15_BARRIER_EMULATION, CONFIG_SETEND_EMULATION))]
static mut insn_emulations: &[*mut InsnEmulation] = &[
    #[cfg(CONFIG_SWP_EMULATION)] &raw mut insn_swp,
    #[cfg(CONFIG_CP15_BARRIER_EMULATION)] &raw mut insn_cp15_barrier,
    #[cfg(CONFIG_SETEND_EMULATION)] &raw mut insn_setend,
];

#[cfg(CONFIG_SETEND_EMULATION)]
unsafe fn armv8_deprecated_init() -> c_int {
    if !system_supports_mixed_endian_el0() { insn_setend.status=LegacyInsnStatus::INSN_UNAVAILABLE; }
    #[cfg(CONFIG_SWP_EMULATION)] if cpus_have_final_cap(ARM64_HAS_LSUI) { insn_swp.status=LegacyInsnStatus::INSN_UNAVAILABLE; }
    for ie in insn_emulations.iter() { if (**ie).status != LegacyInsnStatus::INSN_UNAVAILABLE { register_insn_emulation(*ie); } }
    cpuhp_setup_state_nocalls(CPUHP_AP_ARM64_ISNDEP_STARTING, b"arm64/isndep:starting\0".as_ptr() as _, run_all_insn_set_hw_mode, None); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
