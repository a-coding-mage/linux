// SPDX-License-Identifier: GPL-2.0
/* kgdb.c: KGDB support for 64-bit sparc.
 *
 * Copyright (C) 2008 David S. Miller <davem@davemloft.net>
 */

// Translated from the corresponding C implementation. Kernel declarations
// supplied by the surrounding build are intentionally left external.

pub unsafe fn pt_regs_to_gdb_regs(gdb_regs: *mut ::core::ffi::c_ulong,
                                  regs: *mut pt_regs) {
    let win: *mut reg_window;
    let mut i: i32;

    *gdb_regs.add(GDB_G0 as usize) = 0;
    i = 0;
    while i < 15 {
        *gdb_regs.add((GDB_G1 + i as usize) as usize) =
            (*regs).u_regs[(UREG_G1 + i as usize) as usize];
        i += 1;
    }

    win = ((*regs).u_regs[UREG_FP as usize].wrapping_add(STACK_BIAS))
        as *mut reg_window;
    i = 0;
    while i < 8 {
        *gdb_regs.add((GDB_L0 + i as usize) as usize) = (*win).locals[i as usize];
        i += 1;
    }
    i = 0;
    while i < 8 {
        *gdb_regs.add((GDB_I0 + i as usize) as usize) = (*win).ins[i as usize];
        i += 1;
    }

    i = GDB_F0 as i32;
    while i <= GDB_F62 as i32 {
        *gdb_regs.add(i as usize) = 0;
        i += 1;
    }

    *gdb_regs.add(GDB_PC as usize) = (*regs).tpc;
    *gdb_regs.add(GDB_NPC as usize) = (*regs).tnpc;
    *gdb_regs.add(GDB_STATE as usize) = (*regs).tstate;
    *gdb_regs.add(GDB_FSR as usize) = 0;
    *gdb_regs.add(GDB_FPRS as usize) = 0;
    *gdb_regs.add(GDB_Y as usize) = (*regs).y;
}

pub unsafe fn sleeping_thread_to_gdb_regs(gdb_regs: *mut ::core::ffi::c_ulong,
                                          p: *mut task_struct) {
    let t = task_thread_info(p);
    extern "C" {
        static mut switch_to_pc: u32;
        static mut ret_from_fork: u32;
    }
    let win: *mut reg_window;
    let pc: ::core::ffi::c_ulong;
    let cwp: ::core::ffi::c_ulong;
    let mut i: i32;

    i = GDB_G0 as i32;
    while i < GDB_G6 as i32 {
        *gdb_regs.add(i as usize) = 0;
        i += 1;
    }
    *gdb_regs.add(GDB_G6 as usize) = t as ::core::ffi::c_ulong;
    *gdb_regs.add(GDB_G7 as usize) = p as ::core::ffi::c_ulong;
    i = GDB_O0 as i32;
    while i < GDB_SP as i32 {
        *gdb_regs.add(i as usize) = 0;
        i += 1;
    }
    *gdb_regs.add(GDB_SP as usize) = (*t).ksp;
    *gdb_regs.add(GDB_O7 as usize) = 0;

    win = ((*t).ksp.wrapping_add(STACK_BIAS)) as *mut reg_window;
    i = 0;
    while i < 8 {
        *gdb_regs.add((GDB_L0 + i as usize) as usize) = (*win).locals[i as usize];
        i += 1;
    }
    i = 0;
    while i < 8 {
        *gdb_regs.add((GDB_I0 + i as usize) as usize) = (*win).ins[i as usize];
        i += 1;
    }

    i = GDB_F0 as i32;
    while i <= GDB_F62 as i32 {
        *gdb_regs.add(i as usize) = 0;
        i += 1;
    }

    pc = if (*t).new_child {
        (&ret_from_fork as *const _ as ::core::ffi::c_ulong)
    } else {
        (&switch_to_pc as *const _ as ::core::ffi::c_ulong)
    };
    *gdb_regs.add(GDB_PC as usize) = pc;
    *gdb_regs.add(GDB_NPC as usize) = pc.wrapping_add(4);

    cwp = __thread_flag_byte_ptr(t)[TI_FLAG_BYTE_CWP as usize] as ::core::ffi::c_ulong;
    *gdb_regs.add(GDB_STATE as usize) = TSTATE_PRIV | TSTATE_IE | cwp;
    *gdb_regs.add(GDB_FSR as usize) = 0;
    *gdb_regs.add(GDB_FPRS as usize) = 0;
    *gdb_regs.add(GDB_Y as usize) = 0;
}

pub unsafe fn gdb_regs_to_pt_regs(gdb_regs: *mut ::core::ffi::c_ulong,
                                  regs: *mut pt_regs) {
    let win: *mut reg_window;
    let mut i: i32;

    i = 0;
    while i < 15 {
        (*regs).u_regs[(UREG_G1 + i as usize) as usize] =
            *gdb_regs.add((GDB_G1 + i as usize) as usize);
        i += 1;
    }

    /* If the TSTATE register is changing, we have to preserve the CWP field,
     * otherwise window save/restore explodes.
     */
    if (*regs).tstate != *gdb_regs.add(GDB_STATE as usize) {
        let cwp = (*regs).tstate & TSTATE_CWP;
        (*regs).tstate = (*gdb_regs.add(GDB_STATE as usize) & !TSTATE_CWP) | cwp;
    }

    (*regs).tpc = *gdb_regs.add(GDB_PC as usize);
    (*regs).tnpc = *gdb_regs.add(GDB_NPC as usize);
    (*regs).y = *gdb_regs.add(GDB_Y as usize);

    win = ((*regs).u_regs[UREG_FP as usize].wrapping_add(STACK_BIAS))
        as *mut reg_window;
    i = 0;
    while i < 8 {
        (*win).locals[i as usize] = *gdb_regs.add((GDB_L0 + i as usize) as usize);
        i += 1;
    }
    i = 0;
    while i < 8 {
        (*win).ins[i as usize] = *gdb_regs.add((GDB_I0 + i as usize) as usize);
        i += 1;
    }
}

#[cfg(CONFIG_SMP)]
pub unsafe extern "C" fn smp_kgdb_capture_client(irq: i32, regs: *mut pt_regs) {
    let mut flags: ::core::ffi::c_ulong;

    core::arch::asm!("rdpr %pstate, {0}\n\twrpr {0}, {1}, %pstate",
                     out(reg) flags, const PSTATE_IE);
    flushw_all();
    if atomic_read(&kgdb_active) != -1 {
        kgdb_nmicallback(raw_smp_processor_id(), regs);
    }
    core::arch::asm!("wrpr {0}, 0, %pstate", in(reg) flags);
}

pub unsafe fn kgdb_arch_handle_exception(
    e_vector: i32, signo: i32, err_code: i32,
    remcomInBuffer: *mut i8, remcomOutBuffer: *mut i8,
    linux_regs: *mut pt_regs,
) -> i32 {
    let mut addr: ::core::ffi::c_ulong = 0;
    let mut ptr: *mut i8;

    match *remcomInBuffer {
        b'c' as i8 => {
            ptr = remcomInBuffer.add(1);
            if kgdb_hex2long(&mut ptr, &mut addr) {
                (*linux_regs).tpc = addr;
                (*linux_regs).tnpc = addr.wrapping_add(4);
            }
        }
        b'D' as i8 | b'k' as i8 => {}
        _ => return -1,
    }
    if (*linux_regs).tpc == arch_kgdb_breakpoint as ::core::ffi::c_ulong {
        (*linux_regs).tpc = (*linux_regs).tnpc;
        (*linux_regs).tnpc = (*linux_regs).tnpc.wrapping_add(4);
    }
    0
}

pub unsafe extern "C" fn kgdb_trap(trap_level: ::core::ffi::c_ulong,
                                    regs: *mut pt_regs) {
    let prev_state = exception_enter();
    let mut flags: ::core::ffi::c_ulong = 0;

    if user_mode(regs) {
        bad_trap(regs, trap_level);
        exception_exit(prev_state);
        return;
    }
    flushw_all();
    local_irq_save(&mut flags);
    kgdb_handle_exception(0x172, SIGTRAP, 0, regs);
    local_irq_restore(flags);
    exception_exit(prev_state);
}

pub fn kgdb_arch_init() -> i32 { 0 }

pub fn kgdb_arch_exit() {}

pub unsafe fn kgdb_arch_set_pc(regs: *mut pt_regs, ip: ::core::ffi::c_ulong) {
    (*regs).tpc = ip;
    (*regs).tnpc = (*regs).tpc.wrapping_add(4);
}

#[repr(C)]
pub struct kgdb_arch {
    pub gdb_bpt_instr: [u8; 4],
}

/* Breakpoint instruction: ta 0x72 */
#[no_mangle]
pub static arch_kgdb_ops: kgdb_arch = kgdb_arch {
    gdb_bpt_instr: [0x91, 0xd0, 0x20, 0x72],
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
