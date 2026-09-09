// SPDX-License-Identifier: GPL-2.0
/* kgdb.c: KGDB support for 32-bit sparc.
 *
 * Copyright (C) 2008 David S. Miller <davem@davemloft.net>
 */

// Linux and SPARC dependencies are supplied by the surrounding translation unit.

pub unsafe fn pt_regs_to_gdb_regs(gdb_regs: *mut ::core::ffi::c_ulong, regs: *mut pt_regs) {
    let win: *mut reg_window32;
    let mut i: ::core::ffi::c_int;

    *gdb_regs.add(GDB_G0 as usize) = 0;
    i = 0;
    while i < 15 {
        *gdb_regs.add((GDB_G1 + i) as usize) = (*regs).u_regs[(UREG_G1 + i) as usize];
        i += 1;
    }

    win = (*regs).u_regs[UREG_FP as usize] as *mut reg_window32;
    i = 0;
    while i < 8 {
        *gdb_regs.add((GDB_L0 + i) as usize) = (*win).locals[i as usize];
        i += 1;
    }
    i = 0;
    while i < 8 {
        *gdb_regs.add((GDB_I0 + i) as usize) = (*win).ins[i as usize];
        i += 1;
    }

    i = GDB_F0;
    while i <= GDB_F31 {
        *gdb_regs.add(i as usize) = 0;
        i += 1;
    }

    *gdb_regs.add(GDB_Y as usize) = (*regs).y;
    *gdb_regs.add(GDB_PSR as usize) = (*regs).psr;
    *gdb_regs.add(GDB_WIM as usize) = 0;
    *gdb_regs.add(GDB_TBR as usize) = trapbase as *const _ as ::core::ffi::c_ulong;
    *gdb_regs.add(GDB_PC as usize) = (*regs).pc;
    *gdb_regs.add(GDB_NPC as usize) = (*regs).npc;
    *gdb_regs.add(GDB_FSR as usize) = 0;
    *gdb_regs.add(GDB_CSR as usize) = 0;
}

pub unsafe fn sleeping_thread_to_gdb_regs(gdb_regs: *mut ::core::ffi::c_ulong, p: *mut task_struct) {
    let t: *mut thread_info = task_thread_info(p);
    let win: *mut reg_window32;
    let mut i: ::core::ffi::c_int;

    i = GDB_G0;
    while i < GDB_G6 { *gdb_regs.add(i as usize) = 0; i += 1; }
    *gdb_regs.add(GDB_G6 as usize) = t as ::core::ffi::c_ulong;
    *gdb_regs.add(GDB_G7 as usize) = 0;
    i = GDB_O0;
    while i < GDB_SP { *gdb_regs.add(i as usize) = 0; i += 1; }
    *gdb_regs.add(GDB_SP as usize) = (*t).ksp;
    *gdb_regs.add(GDB_O7 as usize) = 0;

    win = (*t).ksp as *mut reg_window32;
    i = 0;
    while i < 8 { *gdb_regs.add((GDB_L0 + i) as usize) = (*win).locals[i as usize]; i += 1; }
    i = 0;
    while i < 8 { *gdb_regs.add((GDB_I0 + i) as usize) = (*win).ins[i as usize]; i += 1; }
    i = GDB_F0;
    while i <= GDB_F31 { *gdb_regs.add(i as usize) = 0; i += 1; }
    *gdb_regs.add(GDB_Y as usize) = 0;
    *gdb_regs.add(GDB_PSR as usize) = (*t).kpsr;
    *gdb_regs.add(GDB_WIM as usize) = (*t).kwim;
    *gdb_regs.add(GDB_TBR as usize) = trapbase as *const _ as ::core::ffi::c_ulong;
    *gdb_regs.add(GDB_PC as usize) = (*t).kpc;
    *gdb_regs.add(GDB_NPC as usize) = (*t).kpc.wrapping_add(4);
    *gdb_regs.add(GDB_FSR as usize) = 0;
    *gdb_regs.add(GDB_CSR as usize) = 0;
}

pub unsafe fn gdb_regs_to_pt_regs(gdb_regs: *mut ::core::ffi::c_ulong, regs: *mut pt_regs) {
    let mut i: ::core::ffi::c_int = 0;
    while i < 15 { (*regs).u_regs[(UREG_G1 + i) as usize] = *gdb_regs.add((GDB_G1 + i) as usize); i += 1; }
    /* If the PSR register is changing, preserve the CWP field. */
    if (*regs).psr != *gdb_regs.add(GDB_PSR as usize) {
        let cwp = (*regs).psr & PSR_CWP;
        (*regs).psr = (*gdb_regs.add(GDB_PSR as usize) & !PSR_CWP) | cwp;
    }
    (*regs).pc = *gdb_regs.add(GDB_PC as usize);
    (*regs).npc = *gdb_regs.add(GDB_NPC as usize);
    (*regs).y = *gdb_regs.add(GDB_Y as usize);
    let win = (*regs).u_regs[UREG_FP as usize] as *mut reg_window32;
    i = 0;
    while i < 8 { (*win).locals[i as usize] = *gdb_regs.add((GDB_L0 + i) as usize); i += 1; }
    i = 0;
    while i < 8 { (*win).ins[i as usize] = *gdb_regs.add((GDB_I0 + i) as usize); i += 1; }
}

pub unsafe fn kgdb_arch_handle_exception(e_vector: ::core::ffi::c_int, signo: ::core::ffi::c_int, err_code: ::core::ffi::c_int, remcom_in_buffer: *mut ::core::ffi::c_char, remcom_out_buffer: *mut ::core::ffi::c_char, linux_regs: *mut pt_regs) -> ::core::ffi::c_int {
    let mut addr: ::core::ffi::c_ulong = 0;
    let mut ptr: *mut ::core::ffi::c_char;
    match *remcom_in_buffer {
        b'c' as ::core::ffi::c_char => {
            ptr = remcom_in_buffer.add(1);
            if kgdb_hex2long(&mut ptr, &mut addr) { (*linux_regs).pc = addr; (*linux_regs).npc = addr.wrapping_add(4); }
        }
        b'D' as ::core::ffi::c_char | b'k' as ::core::ffi::c_char => {}
        _ => return -1,
    }
    if (*linux_regs).pc == arch_kgdb_breakpoint as usize as ::core::ffi::c_ulong { (*linux_regs).pc = (*linux_regs).npc; (*linux_regs).npc = (*linux_regs).npc.wrapping_add(4); }
    0
}

pub unsafe fn kgdb_trap(trap_level: ::core::ffi::c_ulong, regs: *mut pt_regs) {
    let mut flags: ::core::ffi::c_ulong = 0;
    if user_mode(regs) { do_hw_interrupt(regs, trap_level); return; }
    flushw_all();
    local_irq_save(&mut flags);
    kgdb_handle_exception(trap_level, SIGTRAP, 0, regs);
    local_irq_restore(flags);
}

pub fn kgdb_arch_init() -> ::core::ffi::c_int { 0 }
pub fn kgdb_arch_exit() {}

pub unsafe fn kgdb_arch_set_pc(regs: *mut pt_regs, ip: ::core::ffi::c_ulong) {
    (*regs).pc = ip;
    (*regs).npc = (*regs).pc.wrapping_add(4);
}

#[repr(C)]
pub struct kgdb_arch {
    /* Breakpoint instruction: ta 0x7d */
    pub gdb_bpt_instr: [u8; 4],
}

pub static arch_kgdb_ops: kgdb_arch = kgdb_arch { gdb_bpt_instr: [0x91, 0xd0, 0x20, 0x7d] };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
