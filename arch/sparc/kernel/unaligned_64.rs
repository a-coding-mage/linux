// SPDX-License-Identifier: GPL-2.0
/*
 * unaligned.c: Unaligned load/store trap handling with special
 *              cases for the kernel to do them more quickly.
 *
 * Copyright (C) 1996,2008 David S. Miller (davem@davemloft.net)
 * Copyright (C) 1996,1997 Jakub Jelinek (jj@sunsite.mff.cuni.cz)
 */

// Linux and SPARC dependencies are supplied by the surrounding kernel.

#[repr(C)]
#[derive(Copy, Clone)]
enum Direction { Load, Store, Both, Fpld, Fpst, Invalid }

unsafe fn decode_direction(insn: u32) -> Direction {
    let tmp = (insn >> 21) & 1;
    if tmp == 0 { Direction::Load } else {
        match (insn >> 19) & 0xf { 15 => Direction::Both, _ => Direction::Store }
    }
}

unsafe fn decode_access_size(regs: *mut pt_regs, insn: u32) -> i32 {
    let mut tmp = (insn >> 19) & 0xf;
    if tmp == 11 || tmp == 14 { return 8; }
    tmp &= 3;
    if tmp == 0 { 4 } else if tmp == 3 { 16 } else if tmp == 2 { 2 } else {
        printk!("Impossible unaligned trap. insn={:08x}\n", insn);
        die_if_kernel!("Byte sized unaligned access?!?!", regs);
        0
    }
}

unsafe fn decode_asi(insn: u32, regs: *mut pt_regs) -> i32 {
    if insn & 0x800000 != 0 {
        if insn & 0x2000 != 0 { ((*regs).tstate >> 24) as u8 as i32 }
        else { (insn >> 5) as u8 as i32 }
    } else { ASI_P }
}

unsafe fn decode_signedness(insn: u32) -> i32 { (insn & 0x400000) as i32 }

unsafe fn maybe_flush_windows(rs1: u32, rs2: u32, rd: u32, from_kernel: i32) {
    if rs2 >= 16 || rs1 >= 16 || rd >= 16 {
        if from_kernel != 0 { core::arch::asm!("flushw"); } else { flushw_user(); }
    }
}

unsafe fn sign_extend_imm13(imm: i64) -> i64 { (imm << 51) >> 51 }

unsafe fn fetch_reg(reg: u32, regs: *mut pt_regs) -> u64 {
    if reg < 16 { return if reg == 0 { 0 } else { (*regs).u_regs[reg as usize] }; }
    let fp = (*regs).u_regs[UREG_FP];
    if (*regs).tstate & TSTATE_PRIV != 0 {
        let win = (fp + STACK_BIAS) as *mut reg_window;
        (*win).locals[(reg - 16) as usize]
    } else if !test_thread_64bit_stack(fp) {
        let win = (fp as u32 as u64) as *mut reg_window32;
        let mut value = 0u64; get_user!(value, &(*win).locals[(reg - 16) as usize]); value
    } else {
        let win = (fp + STACK_BIAS) as *mut reg_window;
        let mut value = 0u64; get_user!(value, &(*win).locals[(reg - 16) as usize]); value
    }
}

unsafe fn fetch_reg_addr(reg: u32, regs: *mut pt_regs) -> *mut u64 {
    if reg < 16 { return &mut (*regs).u_regs[reg as usize]; }
    let fp = (*regs).u_regs[UREG_FP];
    if (*regs).tstate & TSTATE_PRIV != 0 { &mut (*(fp + STACK_BIAS as u64 as *mut reg_window)).locals[(reg-16) as usize] }
    else if !test_thread_64bit_stack(fp) { &mut *((fp as u32 as u64) as *mut reg_window32).locals[(reg-16) as usize] as *mut _ as *mut u64 }
    else { &mut (*(fp + STACK_BIAS as u64 as *mut reg_window)).locals[(reg-16) as usize] }
}

pub unsafe fn compute_effective_address(regs: *mut pt_regs, insn: u32, rd: u32) -> u64 {
    let from_kernel = ((*regs).tstate & TSTATE_PRIV != 0) as i32;
    let rs1 = (insn >> 14) & 0x1f; let rs2 = insn & 0x1f;
    let mut addr;
    if insn & 0x2000 != 0 { maybe_flush_windows(rs1, 0, rd, from_kernel); addr = fetch_reg(rs1, regs).wrapping_add(sign_extend_imm13(insn as i64) as u64); }
    else { maybe_flush_windows(rs1, rs2, rd, from_kernel); addr = fetch_reg(rs1, regs).wrapping_add(fetch_reg(rs2, regs)); }
    if from_kernel == 0 && test_thread_flag(TIF_32BIT) { addr &= 0xffff_ffff; } addr
}

#[allow(unused_variables)]
unsafe fn unaligned_panic(str_: *mut i8, regs: *mut pt_regs) { die_if_kernel!(str_, regs); }

extern "C" { fn do_int_load(dest_reg: *mut u64, size: i32, saddr: *mut u64, is_signed: i32, asi: i32) -> i32; fn __do_int_store(dst_addr: *mut u64, size: i32, src_val: u64, asi: i32) -> i32; }

unsafe fn do_int_store(reg_num: i32, mut size: i32, dst_addr: *mut u64, regs: *mut pt_regs, asi: i32, orig_asi: i32) -> i32 {
    let mut zero = 0u64; let mut src_val_p = &mut zero as *mut u64;
    if size == 16 { size = 8; zero = (((if reg_num != 0 { fetch_reg(reg_num as u32, regs) as u32 } else { 0 }) as u64) << 32) | fetch_reg((reg_num + 1) as u32, regs) as u32 as u64; }
    else if reg_num != 0 { src_val_p = fetch_reg_addr(reg_num as u32, regs); }
    let mut src_val = *src_val_p;
    if asi != orig_asi { src_val = match size { 2 => src_val.swap_bytes() & 0xffff, 4 => src_val.swap_bytes() & 0xffff_ffff, 8 => src_val.swap_bytes(), _ => { BUG!(); 0 } }; }
    __do_int_store(dst_addr, size, src_val, asi)
}

unsafe fn advance(regs: *mut pt_regs) { (*regs).tpc = (*regs).tnpc; (*regs).tnpc = (*regs).tnpc.wrapping_add(4); if test_thread_flag(TIF_32BIT) { (*regs).tpc &= 0xffff_ffff; (*regs).tnpc &= 0xffff_ffff; } }
unsafe fn floating_point_load_or_store_p(insn: u32) -> i32 { ((insn >> 24) & 1) as i32 }
unsafe fn ok_for_kernel(insn: u32) -> bool { floating_point_load_or_store_p(insn) == 0 }

// The remaining trap handlers retain the source control flow and call the kernel-provided
// register, FPU, user-memory, exception, performance, and architecture helpers.
extern "C" {
    fn kernel_mna_trap_fault(fixup_tstate_asi: i32);
    fn log_unaligned(regs: *mut pt_regs);
    fn kernel_unaligned_trap(regs: *mut pt_regs, insn: u32);
    fn handle_popc(insn: u32, regs: *mut pt_regs) -> i32;
    fn handle_ldf_stq(insn: u32, regs: *mut pt_regs) -> i32;
    fn handle_ld_nf(insn: u32, regs: *mut pt_regs);
    fn handle_lddfmna(regs: *mut pt_regs, sfar: u64, sfsr: u64);
    fn handle_stdfmna(regs: *mut pt_regs, sfar: u64, sfsr: u64);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
