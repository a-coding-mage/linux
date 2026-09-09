// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_void};

pub type instr = u32;

const MAJOR_OP: instr = 0xfc000000;
const LDA_OP: instr = 0x20000000;
const STQ_OP: instr = 0xb4000000;
const BR_OP: instr = 0xc0000000;

const STK_ALLOC_1: instr = 0x23de8000;
const STK_ALLOC_1M: instr = 0xffff8000;
const STK_ALLOC_2: instr = 0x43c0153e;
const STK_ALLOC_2M: instr = 0xffe01fff;

const MEM_REG: instr = 0x03e00000;
const MEM_BASE: instr = 0x001f0000;
const MEM_OFF: instr = 0x0000ffff;
const MEM_OFF_SIGN: instr = 0x00008000;
const BASE_SP: instr = 0x001e0000;

extern "C" {
    fn printk(fmt: *const c_char, ...) -> i32;
    static START_ADDR: usize;
    static current_stack_pointer: *mut c_void;
}

static REG_NAME: [[u8; 4]; 32] = [
    *b"v0 ", *b"t0 ", *b"t1 ", *b"t2 ", *b"t3 ", *b"t4 ", *b"t5 ", *b"t6 ",
    *b"t7 ", *b"s0 ", *b"s1 ", *b"s2 ", *b"s3 ", *b"s4 ", *b"s5 ", *b"s6 ",
    *b"a0 ", *b"a1 ", *b"a2 ", *b"a3 ", *b"a4 ", *b"a5 ", *b"t8 ", *b"t9 ",
    *b"t10", *b"t11", *b"ra ", *b"pv ", *b"at ", *b"gp ", *b"sp ", *b"0",
];

#[inline]
fn stk_alloc_match(value: instr) -> bool {
    (value & STK_ALLOC_1M) == STK_ALLOC_1 || (value & STK_ALLOC_2M) == STK_ALLOC_2
}

#[inline]
fn stk_push_match(value: instr) -> bool {
    (value & (MAJOR_OP | MEM_BASE | MEM_OFF_SIGN)) == (STQ_OP | BASE_SP)
}

#[inline]
fn bb_end(value: instr) -> bool {
    (value >= BR_OP)
        || (value < LDA_OP)
        || (((value ^ 0x60000000) < 0x20000000) && ((value & 0x0c000000) != 0))
}

#[inline]
fn is_kernel_text(pc: *const instr) -> bool {
    pc as usize > unsafe { START_ADDR }
}

unsafe fn display_stored_regs(mut pro_pc: *mut instr, sp: *mut u8) -> *mut instr {
    let mut ret_pc: *mut instr = core::ptr::null_mut();
    let fmt = b"Prologue [<%p>], Frame %p:\n\0";
    printk(fmt.as_ptr() as *const c_char, pro_pc, sp);
    while !bb_end(*pro_pc) {
        if stk_push_match(*pro_pc) {
            let reg = ((*pro_pc & MEM_REG) >> 21) as usize;
            let value = *(sp.add((*pro_pc & MEM_OFF) as usize) as *const usize);
            if reg == 26 {
                ret_pc = value as *mut instr;
            }
            let reg_fmt = b"\t\t%s / 0x%016lx\n\0";
            printk(reg_fmt.as_ptr() as *const c_char, REG_NAME[reg].as_ptr(), value);
        }
        pro_pc = pro_pc.add(1);
    }
    ret_pc
}

unsafe fn seek_prologue(mut pc: *mut instr) -> *mut instr {
    while !stk_alloc_match(*pc) {
        pc = pc.sub(1);
    }
    while !bb_end(*pc.sub(1)) {
        pc = pc.sub(1);
    }
    pc
}

unsafe fn stack_increment(mut prologue_pc: *mut instr) -> isize {
    while !stk_alloc_match(*prologue_pc) {
        prologue_pc = prologue_pc.add(1);
    }
    if (*prologue_pc & STK_ALLOC_1M) == STK_ALLOC_1M {
        -(((*prologue_pc as i64) << 48) >> 48) as isize
    } else {
        ((*prologue_pc >> 13) & 0xff) as isize
    }
}

pub unsafe fn stacktrace() {
    let mut prologue = stacktrace as *const () as *mut instr;
    let mut sp = current_stack_pointer as *mut u8;
    let trace_fmt = b"\tstack trace:\n\0";
    printk(trace_fmt.as_ptr() as *const c_char);
    loop {
        let ret_pc = display_stored_regs(prologue, sp);
        sp = sp.offset(stack_increment(prologue));
        prologue = seek_prologue(ret_pc);
        if !is_kernel_text(ret_pc) {
            break;
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
