// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/kernel/swp_emulate.c
 *
 *  Copyright (C) 2009 ARM Limited
 *  __user_* functions adapted from include/asm/uaccess.h
 *
 *  Implements emulation of the SWP/SWPB instructions using load-exclusive and
 *  store-exclusive for processors that have them disabled (or future ones that
 *  might not implement them).
 */

// Dependencies are supplied by the surrounding kernel translation unit.

const RN_OFFSET: u32 = 16;
const RT_OFFSET: u32 = 12;
const RT2_OFFSET: u32 = 0;
const TYPE_SWPB: u32 = 1 << 22;

static mut swpcounter: libc::c_ulong = 0;
static mut swpbcounter: libc::c_ulong = 0;
static mut abtcounter: libc::c_ulong = 0;
static mut previous_pid: pid_t = 0;

#[inline]
const fn extract_reg_num(instruction: u32, offset: u32) -> usize {
    (((instruction & (0xf << offset)) >> offset) as usize)
}

/* Error-checking SWP macros implemented using ldrex{b}/strex{b}. */
unsafe fn user_swp_asm(data: &mut u32, addr: u32, res: &mut u32, temp: &mut libc::c_ulong) {
    core::arch::asm!(
        ".arch armv7-a\n\
         0: ldrex {tmp}, [{address}]\n\
         1: strex {result}, {value}, [{address}]\n\
            cmp {result}, #0\n\
            moveq {value}, {tmp}\n\
            movne {result}, #-11\n\
         2:\n\
            .section .text.fixup,\"ax\"\n\
            .align 2\n\
         3: mov {result}, #-14\n\
            b 2b\n\
            .previous\n\
            .section __ex_table,\"a\"\n\
            .align 3\n\
            .long 0b, 3b\n\
            .long 1b, 3b\n\
            .previous",
        value = inout(reg) *data,
        address = in(reg) addr,
        result = lateout(reg) *res,
        tmp = lateout(reg) *temp,
        options(nostack)
    );
}

unsafe fn user_swpb_asm(data: &mut u32, addr: u32, res: &mut u32, temp: &mut libc::c_ulong) {
    core::arch::asm!(
        ".arch armv7-a\n\
         0: ldrexb {tmp}, [{address}]\n\
         1: strexb {result}, {value}, [{address}]\n\
            cmp {result}, #0\n\
            moveq {value}, {tmp}\n\
            movne {result}, #-11\n\
         2:\n\
            .section .text.fixup,\"ax\"\n\
            .align 2\n\
         3: mov {result}, #-14\n\
            b 2b\n\
            .previous\n\
            .section __ex_table,\"a\"\n\
            .align 3\n\
            .long 0b, 3b\n\
            .long 1b, 3b\n\
            .previous",
        value = inout(reg) *data,
        address = in(reg) addr,
        result = lateout(reg) *res,
        tmp = lateout(reg) *temp,
        options(nostack)
    );
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn proc_status_show(m: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    seq_printf(m, "Emulated SWP:\t\t%lu\n", swpcounter);
    seq_printf(m, "Emulated SWPB:\t\t%lu\n", swpbcounter);
    seq_printf(m, "Aborted SWP{B}:\t\t%lu\n", abtcounter);
    if previous_pid != 0 {
        seq_printf(m, "Last process:\t\t%d\n", previous_pid);
    }
    0
}

unsafe fn set_segfault(regs: *mut pt_regs, addr: libc::c_ulong) {
    let si_code: i32;
    mmap_read_lock((*current).mm);
    if find_vma((*current).mm, addr).is_null() {
        si_code = SEGV_MAPERR;
    } else {
        si_code = SEGV_ACCERR;
    }
    mmap_read_unlock((*current).mm);

    pr_debug!("SWP{{B}} emulation: access caused memory abort!\n");
    arm_notify_die(
        "Illegal memory access",
        regs,
        SIGSEGV,
        si_code,
        instruction_pointer(regs) as *mut core::ffi::c_void,
        0,
        0,
    );
    abtcounter += 1;
}

unsafe fn emulate_swp_x(address: u32, data: *mut u32, kind: u32) -> i32 {
    let mut res: u32 = 0;
    if kind != TYPE_SWPB && address & 0x3 != 0 {
        pr_debug!("SWP instruction on unaligned pointer!\n");
        return -14;
    }

    loop {
        let mut temp: libc::c_ulong = 0;
        let ua_flags = uaccess_save_and_enable();
        if kind == TYPE_SWPB {
            user_swpb_asm(&mut *data, address, &mut res, &mut temp);
        } else {
            user_swp_asm(&mut *data, address, &mut res, &mut temp);
        }
        uaccess_restore(ua_flags);
        if res as i32 != -11 || signal_pending(*current) != 0 {
            break;
        }
        cond_resched();
    }

    if res == 0 {
        if kind == TYPE_SWPB { swpbcounter += 1; } else { swpcounter += 1; }
    }
    res as i32
}

unsafe fn swp_handler(regs: *mut pt_regs, instr: u32) -> i32 {
    perf_sw_event(PERF_COUNT_SW_EMULATION_FAULTS, 1, regs, (*regs).ARM_pc);
    let condition = arm_check_condition(instr, (*regs).ARM_cpsr);
    match condition {
        ARM_OPCODE_CONDTEST_PASS => {},
        ARM_OPCODE_CONDTEST_FAIL => { (*regs).ARM_pc += 4; return 0; },
        ARM_OPCODE_CONDTEST_UNCOND => return -14,
        _ => return -22,
    }

    if (*current).pid != previous_pid {
        pr_debug!("\"%s\" (%ld) uses deprecated SWP{{B}} instruction\n", (*current).comm, (*current).pid as libc::c_ulong);
        previous_pid = (*current).pid;
    }
    let address = (*regs).uregs[extract_reg_num(instr, RN_OFFSET)];
    let mut data = (*regs).uregs[extract_reg_num(instr, RT2_OFFSET)];
    let destreg = extract_reg_num(instr, RT_OFFSET);
    let kind = instr & TYPE_SWPB;

    pr_debug!("addr in r%d->0x%08x, dest is r%d, source in r%d->0x%08x)\n", extract_reg_num(instr, RN_OFFSET), address, destreg, extract_reg_num(instr, RT2_OFFSET), data);
    let mut res = if !access_ok((address & !3) as *mut core::ffi::c_void, 4) { -14 } else { emulate_swp_x(address, &mut data, kind) };
    if res == 0 { (*regs).ARM_pc += 4; (*regs).uregs[destreg] = data; }
    else if res == -14 { set_segfault(regs, address as libc::c_ulong); }
    0
}

static mut swp_hook: undef_hook = undef_hook {
    instr_mask: 0x0fb00ff0,
    instr_val: 0x01000090,
    cpsr_mask: MODE_MASK | PSR_T_BIT | PSR_J_BIT,
    cpsr_val: USR_MODE,
    fn_: swp_handler,
};

unsafe fn swp_emulation_init() -> i32 {
    if cpu_architecture() < CPU_ARCH_ARMV7 { return 0; }
    #[cfg(CONFIG_PROC_FS)]
    if proc_create_single("cpu/swp_emulation", S_IRUGO, core::ptr::null_mut(), proc_status_show).is_null() { return -12; }
    pr_notice!("Registering SWP/SWPB emulation handler\n");
    register_undef_hook(&mut swp_hook);
    0
}

// Equivalent to late_initcall(swp_emulation_init).
late_initcall!(swp_emulation_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
