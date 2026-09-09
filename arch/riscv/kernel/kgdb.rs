// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2020 SiFive
 */

// Declarations supplied by the surrounding kernel translation.

const NOT_KGDB_BREAK: i32 = 0;
const KGDB_SW_BREAK: i32 = 1;
const KGDB_COMPILED_BREAK: i32 = 2;
const KGDB_SW_SINGLE_STEP: i32 = 3;

static mut stepped_address: usize = 0;
static mut stepped_opcode: u32 = 0;

unsafe fn decode_register_index(opcode: usize, offset: i32) -> i32 {
    ((opcode >> offset) & 0x1f) as i32
}

unsafe fn decode_register_index_short(opcode: usize, offset: i32) -> i32 {
    (((opcode >> offset) & 0x7) + 8) as i32
}

/* Calculate the new address for after a step */
unsafe fn get_step_address(regs: *mut pt_regs, next_addr: *mut usize) -> i32 {
    let pc = (*regs).epc as usize;
    let regs_ptr = regs as *mut usize;
    let mut rs1_num: i32;
    let mut rs2_num: i32;
    let op_code: i32;

    if get_kernel_nofault(&mut op_code, pc as *const core::ffi::c_void) != 0 {
        return -EINVAL;
    }
    if (op_code & __INSN_LENGTH_MASK) != __INSN_LENGTH_GE_32 {
        if riscv_insn_is_c_jalr(op_code) || riscv_insn_is_c_jr(op_code) {
            rs1_num = decode_register_index(op_code as usize, RVC_C2_RS1_OPOFF);
            *next_addr = *regs_ptr.add(rs1_num as usize);
        } else if riscv_insn_is_c_j(op_code) || riscv_insn_is_c_jal(op_code) {
            *next_addr = (RVC_EXTRACT_JTYPE_IMM(op_code) as usize).wrapping_add(pc);
        } else if riscv_insn_is_c_beqz(op_code) {
            rs1_num = decode_register_index_short(op_code as usize, RVC_C1_RS1_OPOFF);
            if rs1_num == 0 || *regs_ptr.add(rs1_num as usize) == 0 {
                *next_addr = (RVC_EXTRACT_BTYPE_IMM(op_code) as usize).wrapping_add(pc);
            } else { *next_addr = pc + 2; }
        } else if riscv_insn_is_c_bnez(op_code) {
            rs1_num = decode_register_index_short(op_code as usize, RVC_C1_RS1_OPOFF);
            if rs1_num != 0 && *regs_ptr.add(rs1_num as usize) != 0 {
                *next_addr = (RVC_EXTRACT_BTYPE_IMM(op_code) as usize).wrapping_add(pc);
            } else { *next_addr = pc + 2; }
        } else { *next_addr = pc + 2; }
    } else if (op_code & __INSN_OPCODE_MASK) == __INSN_BRANCH_OPCODE {
        let mut result = false;
        let imm = RV_EXTRACT_BTYPE_IMM(op_code);
        let mut rs1_val: usize = 0;
        let mut rs2_val: usize = 0;
        rs1_num = decode_register_index(op_code as usize, RVG_RS1_OPOFF);
        rs2_num = decode_register_index(op_code as usize, RVG_RS2_OPOFF);
        if rs1_num != 0 { rs1_val = *regs_ptr.add(rs1_num as usize); }
        if rs2_num != 0 { rs2_val = *regs_ptr.add(rs2_num as usize); }
        if riscv_insn_is_beq(op_code) { result = rs1_val == rs2_val; }
        else if riscv_insn_is_bne(op_code) { result = rs1_val != rs2_val; }
        else if riscv_insn_is_blt(op_code) { result = (rs1_val as isize) < (rs2_val as isize); }
        else if riscv_insn_is_bge(op_code) { result = (rs1_val as isize) >= (rs2_val as isize); }
        else if riscv_insn_is_bltu(op_code) { result = rs1_val < rs2_val; }
        else if riscv_insn_is_bgeu(op_code) { result = rs1_val >= rs2_val; }
        *next_addr = if result { (imm as usize).wrapping_add(pc) } else { pc + 4 };
    } else if riscv_insn_is_jal(op_code) {
        *next_addr = (RV_EXTRACT_JTYPE_IMM(op_code) as usize).wrapping_add(pc);
    } else if riscv_insn_is_jalr(op_code) {
        rs1_num = decode_register_index(op_code as usize, RVG_RS1_OPOFF);
        if rs1_num != 0 { *next_addr = *regs_ptr.add(rs1_num as usize); }
        *next_addr = (*next_addr).wrapping_add(RV_EXTRACT_ITYPE_IMM(op_code) as usize);
    } else if riscv_insn_is_sret(op_code) { *next_addr = pc; }
    else { *next_addr = pc + 4; }
    0
}

unsafe fn do_single_step(regs: *mut pt_regs) -> i32 {
    /* Determine where the target instruction will send us to */
    let mut addr = 0usize;
    let mut error = get_step_address(regs, &mut addr);
    if error != 0 { return error; }
    error = get_kernel_nofault(&mut stepped_opcode, addr as *const core::ffi::c_void);
    if error != 0 { return error; }
    stepped_address = addr;
    error = copy_to_kernel_nofault(stepped_address as *mut core::ffi::c_void,
        arch_kgdb_ops.gdb_bpt_instr.as_ptr() as *const core::ffi::c_void, BREAK_INSTR_SIZE);
    if error == 0 {
        flush_icache_range(addr, addr + BREAK_INSTR_SIZE);
        kgdb_single_step = 1;
        atomic_set(&mut kgdb_cpu_doing_single_step, raw_smp_processor_id());
    } else { stepped_address = 0; stepped_opcode = 0; }
    error
}

/* Undo a single step */
unsafe fn undo_single_step(_regs: *mut pt_regs) {
    if stepped_opcode != 0 {
        copy_to_kernel_nofault(stepped_address as *mut core::ffi::c_void,
            (&stepped_opcode as *const u32).cast(), BREAK_INSTR_SIZE);
        flush_icache_range(stepped_address, stepped_address + BREAK_INSTR_SIZE);
    }
    stepped_address = 0; stepped_opcode = 0;
    kgdb_single_step = 0;
    atomic_set(&mut kgdb_cpu_doing_single_step, -1);
}

#[no_mangle]
pub unsafe extern "C" fn dbg_get_reg(regno: i32, mem: *mut core::ffi::c_void, regs: *mut pt_regs) -> *mut i8 {
    if regno >= DBG_MAX_REG_NUM || regno < 0 { return core::ptr::null_mut(); }
    let def = dbg_reg_def[regno as usize];
    if def.offset != -1 { memcpy(mem, (regs as *mut u8).add(def.offset as usize).cast(), def.size); }
    else { memset(mem, 0, def.size); }
    def.name
}

#[no_mangle]
pub unsafe extern "C" fn dbg_set_reg(regno: i32, mem: *mut core::ffi::c_void, regs: *mut pt_regs) -> i32 {
    if regno >= DBG_MAX_REG_NUM || regno < 0 { return -EINVAL; }
    let def = dbg_reg_def[regno as usize];
    if def.offset != -1 { memcpy((regs as *mut u8).add(def.offset as usize).cast(), mem, def.size); }
    0
}

pub unsafe extern "C" fn sleeping_thread_to_gdb_regs(gdb_regs: *mut usize, task: *mut task_struct) {
    /* Initialize to zero */
    memset(gdb_regs.cast(), 0, NUMREGBYTES);
    (*gdb_regs.add(DBG_REG_SP_OFF)) = (*task).thread.sp;
    (*gdb_regs.add(DBG_REG_FP_OFF)) = (*task).thread.s[0];
    (*gdb_regs.add(DBG_REG_S1_OFF)) = (*task).thread.s[1];
    (*gdb_regs.add(DBG_REG_S2_OFF)) = (*task).thread.s[2];
    (*gdb_regs.add(DBG_REG_S3_OFF)) = (*task).thread.s[3];
    (*gdb_regs.add(DBG_REG_S4_OFF)) = (*task).thread.s[4];
    (*gdb_regs.add(DBG_REG_S5_OFF)) = (*task).thread.s[5];
    (*gdb_regs.add(DBG_REG_S6_OFF)) = (*task).thread.s[6];
    (*gdb_regs.add(DBG_REG_S7_OFF)) = (*task).thread.s[7];
    (*gdb_regs.add(DBG_REG_S8_OFF)) = (*task).thread.s[8];
    (*gdb_regs.add(DBG_REG_S9_OFF)) = (*task).thread.s[9];
    (*gdb_regs.add(DBG_REG_S10_OFF)) = (*task).thread.s[10];
    (*gdb_regs.add(DBG_REG_S11_OFF)) = (*task).thread.s[11];
    (*gdb_regs.add(DBG_REG_EPC_OFF)) = (*task).thread.ra;
}

pub unsafe extern "C" fn kgdb_arch_set_pc(regs: *mut pt_regs, pc: usize) { (*regs).epc = pc; }

#[no_mangle]
pub unsafe extern "C" fn arch_kgdb_breakpoint() { core::arch::asm!("ebreak"); }

pub unsafe extern "C" fn kgdb_arch_handle_qxfer_pkt(inp: *mut i8, out: *mut i8) {
    if !strncmp(inp, gdb_xfer_read_target, core::mem::size_of_val(&gdb_xfer_read_target)) {
        strscpy(out, riscv_gdb_stub_target_desc, BUFMAX);
    } else if !strncmp(inp, gdb_xfer_read_cpuxml, core::mem::size_of_val(&gdb_xfer_read_cpuxml)) {
        strscpy(out, riscv_gdb_stub_cpuxml, BUFMAX);
    }
}

unsafe fn kgdb_arch_update_addr(regs: *mut pt_regs, inp: *mut i8) {
    let mut addr = 0usize;
    let mut ptr = inp.add(1);
    if kgdb_hex2long(&mut ptr, &mut addr) != 0 { (*regs).epc = addr; }
}

pub unsafe extern "C" fn kgdb_arch_handle_exception(_vector: i32, _signo: i32, _err_code: i32, inp: *mut i8, _out: *mut i8, regs: *mut pt_regs) -> i32 {
    undo_single_step(regs);
    let mut err = 0;
    match *inp as u8 {
        b'c' | b'D' | b'k' => { if *inp as u8 == b'c' { kgdb_arch_update_addr(regs, inp); } }
        b's' => { kgdb_arch_update_addr(regs, inp); err = do_single_step(regs); }
        _ => err = -1,
    }
    err
}

unsafe fn kgdb_riscv_kgdbbreak(addr: usize) -> i32 {
    if stepped_address == addr { return KGDB_SW_SINGLE_STEP; }
    if atomic_read(&kgdb_setting_breakpoint) != 0 && addr == (&kgdb_compiled_break as *const _ as usize) { return KGDB_COMPILED_BREAK; }
    kgdb_has_hit_break(addr)
}

#[no_mangle]
pub static mut dbg_reg_def: [dbg_reg_def_t; DBG_MAX_REG_NUM as usize] = [
    dbg_reg_def_t { name: DBG_REG_ZERO, size: GDB_SIZEOF_REG, offset: -1 },
    dbg_reg_def_t { name: DBG_REG_RA, size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, ra) },
    dbg_reg_def_t { name: DBG_REG_SP, size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, sp) },
    dbg_reg_def_t { name: DBG_REG_GP, size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, gp) },
    dbg_reg_def_t { name: DBG_REG_TP, size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, tp) },
    dbg_reg_def_t { name: DBG_REG_T0, size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, t0) },
    dbg_reg_def_t { name: DBG_REG_T1, size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, t1) },
    dbg_reg_def_t { name: DBG_REG_T2, size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, t2) },
    dbg_reg_def_t { name: DBG_REG_FP, size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, s0) },
    dbg_reg_def_t { name: DBG_REG_S1, size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, s1) },
    dbg_reg_def_t { name: DBG_REG_A0, size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, a0) },
    dbg_reg_def_t { name: DBG_REG_A1, size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, a1) },
    dbg_reg_def_t { name: DBG_REG_A2, size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, a2) },
    dbg_reg_def_t { name: DBG_REG_A3, size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, a3) },
    dbg_reg_def_t { name: DBG_REG_A4, size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, a4) },
    dbg_reg_def_t { name: DBG_REG_A5, size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, a5) },
    dbg_reg_def_t { name: DBG_REG_A6, size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, a6) },
    dbg_reg_def_t { name: DBG_REG_A7, size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, a7) },
    dbg_reg_def_t { name: DBG_REG_S2, size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, s2) },
    dbg_reg_def_t { name: DBG_REG_S3, size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, s3) },
    dbg_reg_def_t { name: DBG_REG_S4, size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, s4) },
    dbg_reg_def_t { name: DBG_REG_S5, size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, s5) },
    dbg_reg_def_t { name: DBG_REG_S6, size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, s6) },
    dbg_reg_def_t { name: DBG_REG_S7, size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, s7) },
    dbg_reg_def_t { name: DBG_REG_S8, size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, s8) },
    dbg_reg_def_t { name: DBG_REG_S9, size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, s9) },
    dbg_reg_def_t { name: DBG_REG_S10, size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, s10) },
    dbg_reg_def_t { name: DBG_REG_S11, size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, s11) },
    dbg_reg_def_t { name: DBG_REG_T3, size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, t3) },
    dbg_reg_def_t { name: DBG_REG_T4, size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, t4) },
    dbg_reg_def_t { name: DBG_REG_T5, size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, t5) },
    dbg_reg_def_t { name: DBG_REG_T6, size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, t6) },
    dbg_reg_def_t { name: DBG_REG_EPC, size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, epc) },
    dbg_reg_def_t { name: DBG_REG_STATUS, size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, status) },
    dbg_reg_def_t { name: DBG_REG_BADADDR, size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, badaddr) },
    dbg_reg_def_t { name: DBG_REG_CAUSE, size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, cause) },
];

unsafe fn kgdb_riscv_notify(_self: *mut notifier_block, cmd: usize, ptr: *mut core::ffi::c_void) -> i32 {
    let args = ptr as *mut die_args;
    let regs = (*args).regs;
    if user_mode(regs) != 0 { return NOTIFY_DONE; }
    let typ = kgdb_riscv_kgdbbreak((*regs).epc);
    if typ == NOT_KGDB_BREAK && cmd == DIE_TRAP { return NOTIFY_DONE; }
    let mut flags = 0usize;
    local_irq_save(&mut flags);
    if kgdb_handle_exception(if typ == KGDB_SW_SINGLE_STEP { 0 } else { 1 }, (*args).signr, cmd, regs) != 0 { return NOTIFY_DONE; }
    if typ == KGDB_COMPILED_BREAK { (*regs).epc += 4; }
    local_irq_restore(flags);
    NOTIFY_STOP
}

static mut kgdb_notifier: notifier_block = notifier_block { notifier_call: Some(kgdb_riscv_notify) };

pub unsafe extern "C" fn kgdb_arch_init() -> i32 {
    register_die_notifier(&mut kgdb_notifier); 0
}

pub unsafe extern "C" fn kgdb_arch_exit() { unregister_die_notifier(&mut kgdb_notifier); }

// CONFIG_RISCV_ISA_C selects the two-byte c.ebreak encoding; otherwise ebreak is four bytes.
#[cfg(CONFIG_RISCV_ISA_C)]
pub static arch_kgdb_ops: kgdb_arch = kgdb_arch { gdb_bpt_instr: [0x02, 0x90, 0, 0] };
#[cfg(not(CONFIG_RISCV_ISA_C))]
pub static arch_kgdb_ops: kgdb_arch = kgdb_arch { gdb_bpt_instr: [0x73, 0x00, 0x10, 0x00] };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
