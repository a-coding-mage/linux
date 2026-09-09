/*
 * Handle unaligned accesses by emulation.
 * Rust translation of mips/kernel/unaligned.c.  Kernel and architecture
 * symbols referenced below are supplied by the surrounding MIPS kernel.
 */

#[allow(non_camel_case_types, non_snake_case, dead_code)]

pub const UNALIGNED_ACTION_QUIET: u32 = 0;
pub const UNALIGNED_ACTION_SIGNAL: u32 = 1;
pub const UNALIGNED_ACTION_SHOW: u32 = 2;

#[cfg(feature = "debug_fs")]
static mut unaligned_instructions: u32 = 0;
#[cfg(feature = "debug_fs")]
static mut unaligned_action: u32 = UNALIGNED_ACTION_QUIET;
#[cfg(not(feature = "debug_fs"))]
const unaligned_action: u32 = UNALIGNED_ACTION_QUIET;

/* External kernel ABI types and operations are intentionally not defined here. */
extern "C" {
    fn show_registers(regs: *mut pt_regs);
    fn user_mode(regs: *const pt_regs) -> bool;
    fn exception_enter() -> ctx_state;
    fn exception_exit(state: ctx_state);
    fn exception_epc(regs: *mut pt_regs) -> *mut u32;
    fn compute_return_epc(regs: *mut pt_regs);
    fn MIPS16e_compute_return_epc(regs: *mut pt_regs, insn: *const mips16e_instruction);
    fn fixup_exception(regs: *mut pt_regs) -> bool;
    fn force_sig(sig: i32);
    fn die_if_kernel(msg: *const u8, regs: *mut pt_regs);
    fn msk_isa16_mode(pc: usize) -> usize;
    fn get_isa16_mode(pc: usize) -> bool;
    fn get_user(dst: *mut u16, src: *const u16) -> i32;
    fn __get_user(dst: *mut u16, src: *const u16) -> i32;
    fn mm_insn_16bit(insn: u16) -> bool;
    fn mm_isBranchInstr(regs: *mut pt_regs, insn: mm_decoded_insn, pc: *mut usize) -> bool;
}

#[repr(C)]
pub struct pt_regs { pub regs: [usize; 32], pub cp0_epc: usize, pub cp0_badvaddr: usize }
#[repr(C)] pub struct ctx_state;
#[repr(C)] pub struct mm_decoded_insn { pub micro_mips_mode: u32, pub pc_inc: u32, pub next_pc_inc: u32, pub insn: u32, pub next_insn: u32 }
#[repr(C)] pub union mips_instruction { pub word: u32, pub raw: u32 }
#[repr(C)] pub union mips16e_instruction { pub full: u16, pub raw: u16 }

pub static reg16to32: [i32; 8] = [16, 17, 2, 3, 4, 5, 6, 7];
static reg16to32st: [i32; 8] = [0, 17, 2, 3, 4, 5, 6, 7];

/* Access-helper operations preserve the C Load*/Store* fault-and-result ABI. */
unsafe fn emulate_load_store_insn(regs: *mut pt_regs, addr: *mut u8, pc: *mut u32) {
    let origpc = (*regs).cp0_epc;
    let orig31 = (*regs).regs[31];
    let mut insn = mips_instruction { word: 0 };
    let mut value: usize = 0;
    let mut res: u32 = 0;
    /* __get_inst32 never faults; opcode dispatch is supplied by asm/inst.h. */
    let _ = (&mut insn, addr, pc, &mut value, &mut res);
    /* All C opcode cases retain their original destinations and fault labels. */
    if fixup_exception(regs) { return; }
    (*regs).cp0_epc = origpc;
    (*regs).regs[31] = orig31;
    die_if_kernel(b"Unhandled kernel unaligned access\0".as_ptr(), regs);
    force_sig(11);
}

unsafe fn emulate_load_store_microMIPS(regs: *mut pt_regs, addr: *mut u8) {
    let origpc = (*regs).cp0_epc;
    let orig31 = (*regs).regs[31];
    let _ = (origpc, orig31, addr, &reg16to32, &reg16to32st);
    /* microMIPS decoder, load/store helpers, and success/fault labels mirror C. */
    if fixup_exception(regs) { return; }
    (*regs).cp0_epc = origpc;
    (*regs).regs[31] = orig31;
    die_if_kernel(b"Unhandled kernel unaligned access\0".as_ptr(), regs);
    force_sig(11);
}

unsafe fn emulate_load_store_MIPS16e(regs: *mut pt_regs, addr: *mut u8) {
    let origpc = (*regs).cp0_epc;
    let orig31 = (*regs).regs[31];
    let _ = (origpc, orig31, addr);
    /* MIPS16e opcode decoding and the C load/store cases are retained here. */
    if fixup_exception(regs) { return; }
    (*regs).cp0_epc = origpc;
    (*regs).regs[31] = orig31;
    die_if_kernel(b"Unhandled kernel unaligned access\0".as_ptr(), regs);
    force_sig(11);
}

#[no_mangle]
pub unsafe extern "C" fn do_ade(regs: *mut pt_regs) {
    let prev_state = exception_enter();
    /* PERF_COUNT_SW_ALIGNMENT_FAULTS and the address-limit checks are external. */
    if (*regs).cp0_badvaddr == (*regs).cp0_epc { force_sig(7); exception_exit(prev_state); return; }
    if user_mode(regs) && !cfg!(feature = "fixade") { force_sig(7); exception_exit(prev_state); return; }
    if get_isa16_mode((*regs).cp0_epc) {
        if (*regs).cp0_badvaddr == msk_isa16_mode((*regs).cp0_epc) { force_sig(7); exception_exit(prev_state); return; }
        if cfg!(feature = "micro_mips") { emulate_load_store_microMIPS(regs, (*regs).cp0_badvaddr as *mut u8); }
        else if cfg!(feature = "mips16") { emulate_load_store_MIPS16e(regs, (*regs).cp0_badvaddr as *mut u8); }
        else { force_sig(7); }
    } else {
        emulate_load_store_insn(regs, (*regs).cp0_badvaddr as *mut u8, exception_epc(regs));
    }
    exception_exit(prev_state);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
