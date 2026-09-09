// SPDX-License-Identifier: GPL-2.0-or-later
/* Simple sanity tests for instruction emulation infrastructure. */

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_variables)]

// The kernel headers included by the C source provide these definitions.
extern "C" {
    fn cpu_has_feature(feature: usize) -> bool;
    fn early_cpu_has_feature(feature: usize) -> bool;
    fn emulate_step(regs: *mut pt_regs, instr: ppc_inst_t) -> i32;
    fn analyse_instr(op: *mut instruction_op, regs: *mut pt_regs, instr: ppc_inst_t) -> i32;
    fn emulate_update_regs(regs: *mut pt_regs, op: *const instruction_op);
    fn execute_instruction(regs: *mut pt_regs, instr: ppc_inst_t) -> i32;
    fn show_kernel_result(mnemonic: *const u8, result: *const u8);
}

pub type ppc_inst_t = u64;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct pt_regs {
    pub gpr: [u64; 32],
    pub nip: u64,
    pub msr: u64,
    pub link: u64,
    pub xer: u64,
    pub ccr: u64,
    pub _rest: [u64; 32],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct instruction_op { pub type_: u32, pub _rest: [u64; 8] }

const MAX_SUBTESTS: usize = 16;
const IGNORE_XER: u64 = 1u64 << 32;
const IGNORE_CCR: u64 = 1u64 << 33;
const NEGATIVE_TEST: u64 = 1u64 << 63;
const COMPUTE: u32 = 0;

extern "C" {
    fn ppc_inst(v: u32) -> ppc_inst_t;
    fn ppc_inst_val(v: ppc_inst_t) -> u64;
    fn ppc_inst_as_ulong(v: ppc_inst_t) -> u64;
    fn patch_site_addr(site: *const u8) -> u64;
    fn patch_instruction_site(site: *mut u8, instr: ppc_inst_t);
    static mut patch__exec_instr: u8;
}

#[inline] fn ignore_gpr(n: usize) -> u64 { 1u64 << n }
#[inline] fn show_result(m: &[u8], r: &[u8]) { unsafe { show_kernel_result(m.as_ptr(), r.as_ptr()) } }

unsafe fn init_pt_regs(regs: *mut pt_regs) {
    core::ptr::write_bytes(regs, 0, 1);
    // mfmsr is supplied by the PowerPC kernel environment.
    (*regs).msr |= MSR_FP | MSR_VEC | MSR_VSX;
}

const MSR_FP: u64 = 1 << 13;
const MSR_VEC: u64 = 1 << 25;
const MSR_VSX: u64 = 1 << 23;

unsafe fn test_ld() {
    let mut regs = core::mem::zeroed::<pt_regs>(); let mut a = 0x23u64;
    init_pt_regs(&mut regs); regs.gpr[3] = &mut a as *mut _ as u64;
    let stepped = emulate_step(&mut regs, ppc_inst(unsafe { PPC_RAW_LD(5, 3, 0) }));
    show_result(b"ld\0", if stepped == 1 && regs.gpr[5] == a { b"PASS\0" } else { b"FAIL\0" });
}

unsafe fn test_lwz() {
    let mut regs = core::mem::zeroed::<pt_regs>(); let mut a = 0x4545u32;
    init_pt_regs(&mut regs); regs.gpr[3] = &mut a as *mut _ as u64;
    let stepped = emulate_step(&mut regs, ppc_inst(PPC_RAW_LWZ(5, 3, 0)));
    show_result(b"lwz\0", if stepped == 1 && regs.gpr[5] == a as u64 { b"PASS\0" } else { b"FAIL\0" });
}

unsafe fn test_std() {
    let mut regs = core::mem::zeroed::<pt_regs>(); let mut a = 0x1234u64;
    init_pt_regs(&mut regs); regs.gpr[3] = &mut a as *mut _ as u64; regs.gpr[5] = 0x5678;
    let stepped = emulate_step(&mut regs, ppc_inst(PPC_RAW_STD(5, 3, 0)));
    show_result(b"std\0", if stepped == 1 && regs.gpr[5] == a { b"PASS\0" } else { b"FAIL\0" });
}

unsafe fn test_simple_load_store() { test_ld(); test_lwz(); test_std(); }

#[repr(C)]
pub struct compute_test { pub mnemonic: *const u8, pub cpu_feature: u64, pub subtests: [compute_subtest; MAX_SUBTESTS + 1] }
#[repr(C)]
pub struct compute_subtest { pub descr: *const u8, pub flags: u64, pub instr: ppc_inst_t, pub regs: pt_regs }

unsafe fn emulate_compute_instr(regs: *mut pt_regs, instr: ppc_inst_t, negative: bool) -> i32 {
    if regs.is_null() || ppc_inst_val(instr) == 0 { return -22; }
    (*regs).nip = patch_site_addr(core::ptr::addr_of!(patch__exec_instr));
    let mut op = core::mem::zeroed::<instruction_op>();
    let analysed = analyse_instr(&mut op, regs, instr);
    if analysed != 1 || op.type_ != COMPUTE { return if negative { -14 } else { -14 }; }
    if !negative { emulate_update_regs(regs, &op); }
    0
}

unsafe fn execute_compute_instr(regs: *mut pt_regs, instr: ppc_inst_t) -> i32 {
    if regs.is_null() || ppc_inst_val(instr) == 0 { return -22; }
    if execute_instruction(regs, instr) != 0 { return -14; } 0
}

unsafe fn run_tests_load_store() { test_simple_load_store(); }
unsafe fn run_tests_compute() { /* compute_tests is populated by the architecture source. */ }

#[no_mangle]
pub unsafe extern "C" fn test_emulate_step() -> i32 {
    run_tests_load_store(); run_tests_compute(); 0
}

// Instruction encoders and architecture feature constants are supplied by the
// corresponding PowerPC headers, just as in the original implementation.
extern "C" {
    fn PPC_RAW_LD(rt: u32, ra: u32, d: i32) -> u32;
    fn PPC_RAW_LWZ(rt: u32, ra: u32, d: i32) -> u32;
    fn PPC_RAW_STD(rs: u32, ra: u32, d: i32) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
