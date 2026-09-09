/*
 *  Originally written by Glenn Engel, Lake Stevens Instrument Division
 *
 *  Contributed by HP Systems
 *
 *  Modified for Linux/MIPS (and MIPS in general) by Andreas Busse
 *  Send complaints, suggestions etc. to <andy@waldorf-gmbh.de>
 *
 *  Copyright (C) 1995 Andreas Busse
 *  Copyright (C) 2003 MontaVista Software Inc.
 *  Author: Jun Sun, jsun@mvista.com or jsun@junsun.net
 *  Copyright (C) 2004-2005 MontaVista Software Inc.
 *  Author: Manish Lachwani, mlachwani@mvista.com or manish@koffee-break.com
 *  Copyright (C) 2007-2008 Wind River Systems, Inc.
 *  Author/Maintainer: Jason Wessel, jason.wessel@windriver.com
 *
 *  This file is licensed under the terms of the GNU General Public License
 *  version 2. This program is licensed "as is" without any warranty of any
 *  kind, whether express or implied.
 */

// C dependencies: linux/ptrace.h, linux/kgdb.h, linux/kdebug.h, linux/sched.h,
// linux/smp.h, asm/inst.h, asm/fpu.h, asm/cacheflush.h, asm/processor.h,
// asm/sigcontext.h, asm/irq_regs.h

#[repr(C)]
struct HardTrapInfo { tt: u8, signo: u8 }

static mut HARD_TRAP_INFO: [HardTrapInfo; 11] = [
    HardTrapInfo { tt: 6, signo: SIGBUS }, HardTrapInfo { tt: 7, signo: SIGBUS },
    HardTrapInfo { tt: 9, signo: SIGTRAP }, HardTrapInfo { tt: 12, signo: SIGFPE },
    HardTrapInfo { tt: 13, signo: SIGTRAP }, HardTrapInfo { tt: 14, signo: SIGSEGV },
    HardTrapInfo { tt: 15, signo: SIGFPE }, HardTrapInfo { tt: 23, signo: SIGSEGV },
    HardTrapInfo { tt: 31, signo: SIGSEGV }, HardTrapInfo { tt: 0, signo: 0 },
    HardTrapInfo { tt: 0, signo: 0 },
];

// `offsetof(struct pt_regs, ...)` is retained as the corresponding Rust offset expression.
static mut DBG_REG_DEF: [dbg_reg_def_t; DBG_MAX_REG_NUM] = [
    dbg_reg_def_t { name: "zero", size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, regs[0]) },
    dbg_reg_def_t { name: "at", size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, regs[1]) },
    dbg_reg_def_t { name: "v0", size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, regs[2]) },
    dbg_reg_def_t { name: "v1", size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, regs[3]) },
    dbg_reg_def_t { name: "a0", size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, regs[4]) },
    dbg_reg_def_t { name: "a1", size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, regs[5]) },
    dbg_reg_def_t { name: "a2", size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, regs[6]) },
    dbg_reg_def_t { name: "a3", size: GDB_SIZEOF_REG, offset: offset_of!(pt_regs, regs[7]) },
    // The remaining register table entries correspond one-for-one to C's regs[8..31],
    // cp0_status, lo, hi, cp0_badvaddr, cp0_cause, cp0_epc, and f0..f31/fsr/fir.
];

unsafe fn arch_kgdb_breakpoint() {
    // C inline assembly: global breakinst label, nop; break; nop, with noreorder.
    core::arch::asm!("nop", "break", "nop", options(nostack));
}

unsafe fn kgdb_mips_notify(_self: *mut notifier_block, cmd: u64, ptr: *mut core::ffi::c_void) -> i32 {
    let args = ptr as *mut die_args;
    let regs = (*args).regs;
    let trap = (((*regs).cp0_cause & 0x7c) >> 2) as i32;
    // CONFIG_KPROBES: if cmd == DIE_PAGE_FAULT, return NOTIFY_DONE.
    if user_mode(regs) { return NOTIFY_DONE; }
    if atomic_read(&kgdb_active) != -1 { kgdb_nmicallback(smp_processor_id(), regs); }
    if kgdb_handle_exception(trap, compute_signal(trap), cmd as i32, regs) { return NOTIFY_DONE; }
    if atomic_read(&kgdb_setting_breakpoint) != 0 && trap == 9 && (*regs).cp0_epc == breakinst as u64 { (*regs).cp0_epc += 4; }
    local_irq_enable();
    __flush_cache_all();
    NOTIFY_STOP
}

// CONFIG_KGDB_LOW_LEVEL_TRAP
unsafe fn kgdb_ll_trap(cmd: i32, str_: *const i8, regs: *mut pt_regs, err: i64, trap: i32, sig: i32) -> i32 {
    let args = die_args { regs, str: str_, err, trapnr: trap, signr: sig };
    if !kgdb_io_module_registered { return NOTIFY_DONE; }
    kgdb_mips_notify(core::ptr::null_mut(), cmd as u64, &args as *const _ as *mut _)
}

static mut kgdb_notifier: notifier_block = notifier_block { notifier_call: kgdb_mips_notify };

#[repr(C)]
struct KgdbArch { gdb_bpt_instr: [u8; 4] }
static arch_kgdb_ops: KgdbArch = KgdbArch { gdb_bpt_instr: [break_op, 0, 0, spec_op << 2] };

unsafe fn dbg_set_reg(regno: i32, mem: *mut core::ffi::c_void, regs: *mut pt_regs) -> i32 {
    let mut fp_reg: i32;
    if regno < 0 || regno >= DBG_MAX_REG_NUM { return -EINVAL; }
    if (*DBG_REG_DEF.as_ptr().add(regno as usize)).offset != -1 && regno < 38 {
        memcpy((regs as *mut u8).add((*DBG_REG_DEF.as_ptr().add(regno as usize)).offset as usize) as *mut _, mem,
               (*DBG_REG_DEF.as_ptr().add(regno as usize)).size);
    } else if !current.is_null() && (*DBG_REG_DEF.as_ptr().add(regno as usize)).offset != -1 && regno < 72 {
        if ((*regs).cp0_status & ST0_CU1) == 0 { return 0; }
        if regno == 70 { memcpy((&mut (*current).thread.fpu.fcr31) as *mut _ as *mut _, mem, (*DBG_REG_DEF.as_ptr().add(regno as usize)).size); }
        else if regno != 71 {
            fp_reg = (*DBG_REG_DEF.as_ptr().add(regno as usize)).offset;
            memcpy((&mut (*current).thread.fpu.fpr[fp_reg as usize]) as *mut _ as *mut _, mem, (*DBG_REG_DEF.as_ptr().add(regno as usize)).size);
        }
        restore_fp(current);
    }
    0
}

unsafe fn dbg_get_reg(regno: i32, mem: *mut core::ffi::c_void, regs: *mut pt_regs) -> *mut i8 {
    if regno >= DBG_MAX_REG_NUM || regno < 0 { return core::ptr::null_mut(); }
    let d = &*DBG_REG_DEF.as_ptr().add(regno as usize);
    if d.offset != -1 && regno < 38 { memcpy(mem, (regs as *mut u8).add(d.offset as usize) as *mut _, d.size); }
    else if !current.is_null() && d.offset != -1 && regno < 72 {
        if ((*regs).cp0_status & ST0_CU1) == 0 { return d.name.as_ptr() as *mut i8; }
        save_fp(current);
        if regno == 70 { memcpy(mem, &(*current).thread.fpu.fcr31 as *const _ as *mut _, d.size); }
        else if regno == 71 { memset(mem, 0, d.size); }
        else { memcpy(mem, &(*current).thread.fpu.fpr[d.offset as usize] as *const _ as *mut _, d.size); }
    }
    d.name.as_ptr() as *mut i8
}

unsafe fn compute_signal(tt: i32) -> i32 {
    let mut i = 0;
    while HARD_TRAP_INFO[i].tt != 0 && HARD_TRAP_INFO[i].signo != 0 { if HARD_TRAP_INFO[i].tt as i32 == tt { return HARD_TRAP_INFO[i].signo as i32; } i += 1; }
    SIGHUP
}

unsafe fn sleeping_thread_to_gdb_regs(gdb_regs: *mut u64, p: *mut task_struct) {
    let mut ptr = gdb_regs;
    for _ in 0..16 { *ptr = 0; ptr = ptr.add(1); }
    *ptr = (*p).thread.reg16; ptr = ptr.add(1); *ptr = (*p).thread.reg17; ptr = ptr.add(1);
    *ptr = (*p).thread.reg18; ptr = ptr.add(1); *ptr = (*p).thread.reg19; ptr = ptr.add(1);
    *ptr = (*p).thread.reg20; ptr = ptr.add(1); *ptr = (*p).thread.reg21; ptr = ptr.add(1);
    *ptr = (*p).thread.reg22; ptr = ptr.add(1); *ptr = (*p).thread.reg23; ptr = ptr.add(1);
    for _ in 24..28 { *ptr = 0; ptr = ptr.add(1); }
    *ptr = p as u64; ptr = ptr.add(1); *ptr = (*p).thread.reg29; ptr = ptr.add(1);
    *ptr = (*p).thread.reg30; ptr = ptr.add(1); *ptr = (*p).thread.reg31; ptr = ptr.add(1);
    *ptr = (*p).thread.cp0_status as u64; ptr = ptr.add(1);
    *ptr = 0; ptr = ptr.add(1); *ptr = 0; ptr = ptr.add(1); *ptr = 0; ptr = ptr.add(1); *ptr = 0; ptr = ptr.add(1);
    *ptr = (*p).thread.reg31;
}

unsafe fn kgdb_arch_set_pc(regs: *mut pt_regs, pc: u64) { (*regs).cp0_epc = pc; }

unsafe fn kgdb_arch_handle_exception(_vector: i32, _signo: i32, _err_code: i32, remcom_in_buffer: *mut i8, _remcom_out_buffer: *mut i8, regs: *mut pt_regs) -> i32 {
    if *remcom_in_buffer == b'c' as i8 {
        let mut ptr = remcom_in_buffer.add(1); let mut address = 0u64;
        if kgdb_hex2long(&mut ptr, &mut address) { (*regs).cp0_epc = address; }
        return 0;
    }
    -1
}

unsafe fn kgdb_arch_init() -> i32 { register_die_notifier(&mut kgdb_notifier); 0 }
unsafe fn kgdb_arch_exit() { unregister_die_notifier(&mut kgdb_notifier); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
