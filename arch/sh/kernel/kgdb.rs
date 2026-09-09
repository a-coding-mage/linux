// SPDX-License-Identifier: GPL-2.0
/*
 * SuperH KGDB support
 *
 * Copyright (C) 2008 - 2012  Paul Mundt
 *
 * Single stepping taken from the old stub by Henry Bell and Jeremy Siegel.
 */

// Macros for single step instruction identification
const SR_T_BIT_MASK: u32 = 0x1;
const STEP_OPCODE: u16 = 0xc33d;

#[inline]
fn opcode_bt(op: insn_size_t) -> bool { (op & 0xff00) == 0x8900 }
#[inline]
fn opcode_bf(op: insn_size_t) -> bool { (op & 0xff00) == 0x8b00 }
#[inline]
fn opcode_btf_disp(op: insn_size_t) -> i32 {
    if (op & 0x80) != 0 { (((op | 0xffffff80) << 1) as i32) } else { (((op & 0x7f) << 1) as i32) }
}
#[inline]
fn opcode_bfs(op: insn_size_t) -> bool { (op & 0xff00) == 0x8f00 }
#[inline]
fn opcode_bts(op: insn_size_t) -> bool { (op & 0xff00) == 0x8d00 }
#[inline]
fn opcode_bra(op: insn_size_t) -> bool { (op & 0xf000) == 0xa000 }
#[inline]
fn opcode_bra_disp(op: insn_size_t) -> i32 {
    if (op & 0x800) != 0 { (((op | 0xfffff800) << 1) as i32) } else { (((op & 0x7ff) << 1) as i32) }
}
#[inline]
fn opcode_braf(op: insn_size_t) -> bool { (op & 0xf0ff) == 0x0023 }
#[inline]
fn opcode_braf_reg(op: insn_size_t) -> usize { ((op & 0x0f00) >> 8) as usize }
#[inline]
fn opcode_bsr(op: insn_size_t) -> bool { (op & 0xf000) == 0xb000 }
#[inline]
fn opcode_bsr_disp(op: insn_size_t) -> i32 { opcode_bra_disp(op) }
#[inline]
fn opcode_bsrf(op: insn_size_t) -> bool { (op & 0xf0ff) == 0x0003 }
#[inline]
fn opcode_bsrf_reg(op: insn_size_t) -> usize { ((op >> 8) & 0xf) as usize }
#[inline]
fn opcode_jmp(op: insn_size_t) -> bool { (op & 0xf0ff) == 0x402b }
#[inline]
fn opcode_jmp_reg(op: insn_size_t) -> usize { ((op >> 8) & 0xf) as usize }
#[inline]
fn opcode_jsr(op: insn_size_t) -> bool { (op & 0xf0ff) == 0x400b }
#[inline]
fn opcode_jsr_reg(op: insn_size_t) -> usize { ((op >> 8) & 0xf) as usize }
#[inline]
fn opcode_rts(op: insn_size_t) -> bool { op == 0xb }
#[inline]
fn opcode_rte(op: insn_size_t) -> bool { op == 0x2b }

/* Calculate the new address for after a step */
unsafe fn get_step_address(linux_regs: *mut pt_regs) -> *mut i16 {
    let op = __raw_readw((*linux_regs).pc);
    let addr: isize;

    if opcode_bt(op) {
        if ((*linux_regs).sr & SR_T_BIT_MASK as _) != 0 { addr = (*linux_regs).pc as isize + 4 + opcode_btf_disp(op) as isize; }
        else { addr = (*linux_regs).pc as isize + 2; }
    } else if opcode_bts(op) {
        if ((*linux_regs).sr & SR_T_BIT_MASK as _) != 0 { addr = (*linux_regs).pc as isize + 4 + opcode_btf_disp(op) as isize; }
        else { addr = (*linux_regs).pc as isize + 4; }
    } else if opcode_bf(op) {
        if ((*linux_regs).sr & SR_T_BIT_MASK as _) == 0 { addr = (*linux_regs).pc as isize + 4 + opcode_btf_disp(op) as isize; }
        else { addr = (*linux_regs).pc as isize + 2; }
    } else if opcode_bfs(op) {
        if ((*linux_regs).sr & SR_T_BIT_MASK as _) == 0 { addr = (*linux_regs).pc as isize + 4 + opcode_btf_disp(op) as isize; }
        else { addr = (*linux_regs).pc as isize + 4; }
    } else if opcode_bra(op) { addr = (*linux_regs).pc as isize + 4 + opcode_bra_disp(op) as isize;
    } else if opcode_braf(op) { addr = (*linux_regs).pc as isize + 4 + (*linux_regs).regs[opcode_braf_reg(op)] as isize;
    } else if opcode_bsr(op) { addr = (*linux_regs).pc as isize + 4 + opcode_bsr_disp(op) as isize;
    } else if opcode_bsrf(op) { addr = (*linux_regs).pc as isize + 4 + (*linux_regs).regs[opcode_bsrf_reg(op)] as isize;
    } else if opcode_jmp(op) { addr = (*linux_regs).regs[opcode_jmp_reg(op)] as isize;
    } else if opcode_jsr(op) { addr = (*linux_regs).regs[opcode_jsr_reg(op)] as isize;
    } else if opcode_rts(op) { addr = (*linux_regs).pr as isize;
    } else if opcode_rte(op) { addr = (*linux_regs).regs[15] as isize;
    } else { addr = (*linux_regs).pc as isize + instruction_size(op) as isize; }

    flush_icache_range(addr as _, addr + instruction_size(op) as isize);
    addr as *mut i16
}

static mut stepped_address: usize = 0;
static mut stepped_opcode: insn_size_t = 0;

unsafe fn do_single_step(linux_regs: *mut pt_regs) {
    let addr = get_step_address(linux_regs);
    stepped_address = addr as usize;
    stepped_opcode = __raw_readw(addr as _);
    *addr = STEP_OPCODE as i16;
    flush_icache_range(addr as _, addr as usize + instruction_size(stepped_opcode) as usize);
}

unsafe fn undo_single_step(_linux_regs: *mut pt_regs) {
    if stepped_opcode != 0 {
        __raw_writew(stepped_opcode, stepped_address as _);
        flush_icache_range(stepped_address as _, stepped_address + 2);
    }
    stepped_opcode = 0;
}

#[repr(C)]
pub struct dbg_reg_def_t { pub name: *const u8, pub size: i32, pub offset: isize }

pub static mut dbg_reg_def: [dbg_reg_def_t; DBG_MAX_REG_NUM as usize] = [
    dbg_reg_def_t { name: b"r0\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 0 },
    dbg_reg_def_t { name: b"r1\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 0 },
    dbg_reg_def_t { name: b"r2\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 0 },
    dbg_reg_def_t { name: b"r3\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 0 },
    dbg_reg_def_t { name: b"r4\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 0 },
    dbg_reg_def_t { name: b"r5\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 0 },
    dbg_reg_def_t { name: b"r6\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 0 },
    dbg_reg_def_t { name: b"r7\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 0 },
    dbg_reg_def_t { name: b"r8\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 0 },
    dbg_reg_def_t { name: b"r9\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 0 },
    dbg_reg_def_t { name: b"r10\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 0 },
    dbg_reg_def_t { name: b"r11\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 0 },
    dbg_reg_def_t { name: b"r12\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 0 },
    dbg_reg_def_t { name: b"r13\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 0 },
    dbg_reg_def_t { name: b"r14\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 0 },
    dbg_reg_def_t { name: b"r15\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 0 },
    dbg_reg_def_t { name: b"pc\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 0 },
    dbg_reg_def_t { name: b"pr\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 0 },
    dbg_reg_def_t { name: b"sr\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 0 },
    dbg_reg_def_t { name: b"gbr\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 0 },
    dbg_reg_def_t { name: b"mach\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 0 },
    dbg_reg_def_t { name: b"macl\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 0 },
    dbg_reg_def_t { name: b"vbr\0".as_ptr(), size: GDB_SIZEOF_REG, offset: -1 },
];

// The remaining declarations and implementations use kernel-provided types,
// constants, functions, and assembly interfaces from the included headers.
extern "C" {
    fn kgdb_hex2long(ptr: *mut *mut u8, value: *mut usize) -> bool;
    fn kgdb_handle_exception(vector: i32, signo: i32, err: i32, regs: *mut pt_regs) -> i32;
    fn raw_smp_processor_id() -> i32;
    fn register_die_notifier(block: *mut notifier_block) -> i32;
    fn unregister_die_notifier(block: *mut notifier_block);
}

// C declarations below retain their source-level interfaces; dependent kernel
// definitions (pt_regs, task_struct, notifier_block, and constants) are supplied
// by the surrounding tree.
pub unsafe fn dbg_set_reg(regno: i32, mem: *mut u8, regs: *mut pt_regs) -> i32 {
    if regno < 0 || regno >= DBG_MAX_REG_NUM { return -EINVAL; }
    if dbg_reg_def[regno as usize].offset != -1 {
        core::ptr::copy_nonoverlapping(mem, (regs as *mut u8).offset(dbg_reg_def[regno as usize].offset), dbg_reg_def[regno as usize].size as usize);
    }
    0
}

pub unsafe fn dbg_get_reg(regno: i32, mem: *mut u8, regs: *mut pt_regs) -> *const u8 {
    if regno >= DBG_MAX_REG_NUM || regno < 0 { return core::ptr::null(); }
    if dbg_reg_def[regno as usize].size != -1 {
        core::ptr::copy_nonoverlapping((regs as *mut u8).offset(dbg_reg_def[regno as usize].offset), mem, dbg_reg_def[regno as usize].size as usize);
    }
    if regno == GDB_VBR { /* __asm__ volatile ("stc vbr, %0") */ }
    dbg_reg_def[regno as usize].name
}

pub unsafe fn sleeping_thread_to_gdb_regs(gdb_regs: *mut usize, p: *mut task_struct) {
    let thread_regs = task_pt_regs(p);
    for reg in 0..DBG_MAX_REG_NUM as usize { *gdb_regs.add(reg) = 0; }
    for reg in GDB_R8 as usize..GDB_R15 as usize { *gdb_regs.add(reg) = (*thread_regs).regs[reg] as usize; }
    *gdb_regs.add(GDB_R15 as usize) = (*p).thread.sp as usize;
    *gdb_regs.add(GDB_PC as usize) = (*p).thread.pc as usize;
    *gdb_regs.add(GDB_PR as usize) = (*thread_regs).pr as usize;
    *gdb_regs.add(GDB_GBR as usize) = (*thread_regs).gbr as usize;
}

pub unsafe fn kgdb_arch_handle_exception(_e_vector: i32, _signo: i32, _err_code: i32, remcomInBuffer: *mut u8, _remcomOutBuffer: *mut u8, linux_regs: *mut pt_regs) -> i32 {
    let mut addr = 0usize;
    undo_single_step(linux_regs);
    match *remcomInBuffer {
        b'c' | b's' => {
            let mut ptr = remcomInBuffer.add(1);
            if kgdb_hex2long(&mut ptr, &mut addr) { (*linux_regs).pc = addr as _; }
            atomic_set(&mut kgdb_cpu_doing_single_step, -1);
            if *remcomInBuffer == b's' { do_single_step(linux_regs); kgdb_single_step = 1; atomic_set(&mut kgdb_cpu_doing_single_step, raw_smp_processor_id()); }
            0
        }
        b'D' | b'k' => { atomic_set(&mut kgdb_cpu_doing_single_step, -1); 0 }
        _ => -1,
    }
}

pub unsafe fn kgdb_arch_pc(exception: i32, regs: *mut pt_regs) -> usize { if exception == 60 { instruction_pointer(regs) - 2 } else { instruction_pointer(regs) } }
pub unsafe fn kgdb_arch_set_pc(regs: *mut pt_regs, ip: usize) { (*regs).pc = ip as _; }

pub unsafe fn kgdb_arch_init() -> i32 { register_die_notifier(&mut kgdb_notifier) }
pub unsafe fn kgdb_arch_exit() { unregister_die_notifier(&mut kgdb_notifier); }

// CONFIG_CPU_LITTLE_ENDIAN selects { 0x3c, 0xc3 }; otherwise { 0xc3, 0x3c }.
#[repr(C)]
pub struct kgdb_arch { pub gdb_bpt_instr: [u8; 2] }
pub static arch_kgdb_ops: kgdb_arch = kgdb_arch { gdb_bpt_instr: [0xc3, 0x3c] };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
