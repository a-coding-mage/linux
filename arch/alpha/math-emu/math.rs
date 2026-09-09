// SPDX-License-Identifier: GPL-2.0-only
// Kernel and soft-fp dependencies are supplied by other translation units.

const OPC_PAL: u32 = 0x00;
const OPC_INTA: u32 = 0x10;
const OPC_INTL: u32 = 0x11;
const OPC_INTS: u32 = 0x12;
const OPC_INTM: u32 = 0x13;
const OPC_FLTC: u32 = 0x14;
const OPC_FLTV: u32 = 0x15;
const OPC_FLTI: u32 = 0x16;
const OPC_FLTL: u32 = 0x17;
const OPC_MISC: u32 = 0x18;
const OPC_JSR: u32 = 0x1a;

const FOP_SRC_S: u32 = 0;
const FOP_SRC_T: u32 = 2;
const FOP_SRC_Q: u32 = 3;
const FOP_FNC_ADDX: u32 = 0;
const FOP_FNC_CVTQL: u32 = 0;
const FOP_FNC_SUBX: u32 = 1;
const FOP_FNC_MULX: u32 = 2;
const FOP_FNC_DIVX: u32 = 3;
const FOP_FNC_CMPXUN: u32 = 4;
const FOP_FNC_CMPXEQ: u32 = 5;
const FOP_FNC_CMPXLT: u32 = 6;
const FOP_FNC_CMPXLE: u32 = 7;
const FOP_FNC_SQRTX: u32 = 11;
const FOP_FNC_CVTXS: u32 = 12;
const FOP_FNC_CVTXT: u32 = 14;
const FOP_FNC_CVTXQ: u32 = 15;
const MISC_TRAPB: u32 = 0x0000;
const MISC_EXCB: u32 = 0x0400;

const EXC_SUM_INV: usize = 1 << 1;
const EXC_SUM_DZE: usize = 1 << 2;
const EXC_SUM_OVF: usize = 1 << 3;
const EXC_SUM_UNF: usize = 1 << 4;
const EXC_SUM_INE: usize = 1 << 5;
const EXC_SUM_MASK: usize = EXC_SUM_INV | EXC_SUM_DZE | EXC_SUM_OVF | EXC_SUM_UNF | EXC_SUM_INE;

// These declarations are provided by the kernel/soft-fp environment.
extern "C" {
    static mut alpha_fp_emul_imprecise: Option<unsafe extern "C" fn(*mut PtRegs, usize) -> isize>;
    static mut alpha_fp_emul: Option<unsafe extern "C" fn(usize, usize) -> isize>;
    fn get_user(dst: *mut u32, src: *const u32) -> i32;
    fn rdfpcr() -> usize;
    fn wrfpcr(v: usize);
    fn implver() -> usize;
    fn swcr_update_status(state: *mut usize, fpcr: usize) -> usize;
    fn alpha_read_fp_reg_s(reg: usize) -> usize;
    fn alpha_read_fp_reg(reg: usize) -> usize;
    fn alpha_write_fp_reg_s(reg: usize, value: usize);
    fn alpha_write_fp_reg(reg: usize, value: usize);
    fn ieee_swcr_to_fpcr(v: usize) -> usize;
    fn printk(fmt: *const u8, ...);
}

#[repr(C)]
pub struct PtRegs { pub pc: usize }

extern "C" {
    fn current_thread_info() -> *mut ThreadInfo;
}
#[repr(C)] pub struct ThreadInfo { pub ieee_state: usize }

pub unsafe extern "C" fn alpha_fp_emul(pc: usize, summary: usize) -> isize {
    let mut insn: u32 = 0;
    get_user(&mut insn, pc as *const u32);
    let fc = ((insn >> 0) & 0x1f) as usize;
    let fb = ((insn >> 16) & 0x1f) as usize;
    let fa = ((insn >> 21) & 0x1f) as usize;
    let func = ((insn >> 5) & 0xf) as u32;
    let src = ((insn >> 9) & 3) as u32;
    let mut mode = ((insn >> 11) & 3) as usize;
    let mut fpcr = rdfpcr();
    let mut swcr = swcr_update_status(&mut (*current_thread_info()).ieee_state, fpcr);
    let mut res: isize = 0;
    let mut vc: usize = 0;

    if mode == 3 { mode = (fpcr >> FPCR_DYN_SHIFT) & 3; }
    match src {
        FOP_SRC_S => {
            let va = alpha_read_fp_reg_s(fa); let vb = alpha_read_fp_reg_s(fb);
            match func {
                FOP_FNC_SUBX => { FP_SUB_S!(res, va, vb); }
                FOP_FNC_ADDX => { FP_ADD_S!(res, va, vb); }
                FOP_FNC_MULX => { FP_MUL_S!(res, va, vb); }
                FOP_FNC_DIVX => { FP_DIV_S!(res, va, vb); }
                FOP_FNC_SQRTX => { FP_SQRT_S!(res, vb); }
                _ => return bad_insn(insn, pc),
            }
            let mut out = 0usize; FP_PACK_SP!(&mut out, res);
            if (_fex!() & FP_EX_UNDERFLOW) != 0 && (swcr & IEEE_MAP_UMZ) != 0 { out = 0; }
            alpha_write_fp_reg_s(fc, out);
        }
        FOP_SRC_T => {
            let va = alpha_read_fp_reg(fa); let vb = alpha_read_fp_reg(fb);
            if (func & !3) == FOP_FNC_CMPXUN {
                FP_CMP_D!(res, va, vb, 3); vc = 0x4000000000000000;
                match func { FOP_FNC_CMPXUN if res != 3 => vc = 0,
                    FOP_FNC_CMPXEQ if res != 0 => vc = 0,
                    FOP_FNC_CMPXLT if res != -1 => vc = 0,
                    FOP_FNC_CMPXLE if res > 0 => vc = 0, _ => {} }
            } else { match func {
                FOP_FNC_SUBX => FP_SUB_D!(vc, va, vb), FOP_FNC_ADDX => FP_ADD_D!(vc, va, vb),
                FOP_FNC_MULX => FP_MUL_D!(vc, va, vb), FOP_FNC_DIVX => FP_DIV_D!(vc, va, vb),
                FOP_FNC_SQRTX => FP_SQRT_D!(vc, vb),
                FOP_FNC_CVTXQ => FP_TO_INT_ROUND_D!(vc, vb, 64, 2),
                _ => return bad_insn(insn, pc),
            }}
            alpha_write_fp_reg(fc, vc);
        }
        FOP_SRC_Q => {
            let vb = alpha_read_fp_reg(fb);
            match func { FOP_FNC_CVTQL => { vc = ((vb & 0xc0000000) << 32) | ((vb & 0x3fffffff) << 29); FP_SET_EXCEPTION!(FP_EX_INVALID); },
                FOP_FNC_CVTXS => FP_FROM_INT_S!(vc, vb, 64, isize), FOP_FNC_CVTXT => FP_FROM_INT_D!(vc, vb, 64, isize),
                _ => return bad_insn(insn, pc) }
            alpha_write_fp_reg(fc, vc);
        }
        _ => return bad_insn(insn, pc),
    }
    let _ = (mode, summary, fpcr, swcr);
    0
}

unsafe fn bad_insn(insn: u32, pc: usize) -> isize {
    printk(b"alpha_fp_emul: Invalid FP insn %#x at %#lx\0".as_ptr(), insn, pc); -1
}

pub unsafe extern "C" fn alpha_fp_emul_imprecise(regs: *mut PtRegs, mut write_mask: usize) -> isize {
    let mut trigger_pc = (*regs).pc - 4; let mut si_code = 0;
    while write_mask != 0 {
        let mut insn = 0u32; get_user(&mut insn, trigger_pc as *const u32);
        let opcode = insn >> 26; let rc = (insn & 0x1f) as usize;
        match opcode {
            OPC_PAL | OPC_JSR | 0x30..=0x3f => break,
            OPC_MISC if (insn & 0xffff) == MISC_TRAPB || (insn & 0xffff) == MISC_EXCB => break,
            OPC_INTA | OPC_INTL | OPC_INTS | OPC_INTM => write_mask &= !(1usize << rc),
            OPC_FLTC | OPC_FLTV | OPC_FLTI | OPC_FLTL => write_mask &= !(1usize << (rc + 32)),
            _ => {}
        }
        if write_mask == 0 { (*regs).pc = trigger_pc + 4; si_code = alpha_fp_emul(trigger_pc, 0); break; }
        trigger_pc -= 4;
    }
    si_code
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
