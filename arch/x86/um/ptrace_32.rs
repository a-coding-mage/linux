/*
 * Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 * Licensed under the GPL
 */

// Dependencies are supplied by the surrounding kernel/UML translation unit.

const FLAG_MASK: c_ulong = 0x0004_4dd5;

// C designated initializers, indexed by the register constants, expanded here
// in register-index order.
static REG_OFFSETS: [c_int; 17] = [
    HOST_BX, HOST_CX, HOST_DX, HOST_SI, HOST_DI, HOST_BP, HOST_AX,
    HOST_DS, HOST_ES, HOST_FS, HOST_GS, HOST_IP, HOST_CS, HOST_EFLAGS,
    HOST_SP, HOST_SS, HOST_ORIG_AX,
];

pub unsafe fn arch_switch_to(to: *mut task_struct) {
    let err = arch_switch_tls(to);
    if err == 0 {
        return;
    }

    if err != -EINVAL {
        printk(KERN_WARNING, "arch_switch_tls failed, errno %d, not EINVAL\n", -err);
    } else {
        printk(KERN_WARNING, "arch_switch_tls failed, errno = EINVAL\n");
    }
}

pub unsafe fn putreg(child: *mut task_struct, mut regno: c_int, mut value: c_ulong) -> c_int {
    regno >>= 2;
    match regno {
        EBX | ECX | EDX | ESI | EDI | EBP | EAX | EIP | UESP => {}
        ORIG_EAX => {
            // UPT_SYSCALL_NR(&(*child).thread.regs.regs) = value;
            *UPT_SYSCALL_NR(&mut (*child).thread.regs.regs) = value;
        }
        FS | GS => {
            if value != 0 && (value & 3) != 3 { return -EIO; }
        }
        DS | ES => {
            if value != 0 && (value & 3) != 3 { return -EIO; }
            value &= 0xffff;
        }
        SS | CS => {
            if (value & 3) != 3 { return -EIO; }
            value &= 0xffff;
        }
        EFL => {
            value &= FLAG_MASK;
            (*child).thread.regs.regs.gp[HOST_EFLAGS as usize] |= value;
            return 0;
        }
        _ => panic!("Bad register in putreg() : {}\n", regno),
    }
    (*child).thread.regs.regs.gp[REG_OFFSETS[regno as usize] as usize] = value;
    0
}

pub unsafe fn poke_user(child: *mut task_struct, mut addr: c_long, data: c_long) -> c_int {
    if (addr & 3) != 0 || addr < 0 { return -EIO; }
    if addr < MAX_REG_OFFSET { return putreg(child, addr as c_int, data as c_ulong); }
    if addr >= offsetof!(user, u_debugreg[0]) && addr <= offsetof!(user, u_debugreg[7]) {
        addr = (addr - offsetof!(user, u_debugreg[0])) >> 2;
        if addr == 4 || addr == 5 { return -EIO; }
        (*child).thread.arch.debugregs[addr as usize] = data as c_ulong;
        return 0;
    }
    -EIO
}

pub unsafe fn getreg(child: *mut task_struct, mut regno: c_int) -> c_ulong {
    let mut mask: c_ulong = !0;
    regno >>= 2;
    match regno {
        FS | GS | DS | ES | SS | CS => mask = 0xffff,
        EIP | UESP | EAX | EBX | ECX | EDX | ESI | EDI | EBP | EFL | ORIG_EAX => {}
        _ => panic!("Bad register in getreg() : {}\n", regno),
    }
    mask & (*child).thread.regs.regs.gp[REG_OFFSETS[regno as usize] as usize]
}

/* read the word at location addr in the USER area. */
pub unsafe fn peek_user(child: *mut task_struct, mut addr: c_long, data: c_long) -> c_int {
    let mut tmp: c_ulong = 0;
    if (addr & 3) != 0 || addr < 0 { return -EIO; }
    if addr < MAX_REG_OFFSET {
        tmp = getreg(child, addr as c_int);
    } else if addr >= offsetof!(user, u_debugreg[0]) && addr <= offsetof!(user, u_debugreg[7]) {
        addr = (addr - offsetof!(user, u_debugreg[0])) >> 2;
        tmp = (*child).thread.arch.debugregs[addr as usize];
    }
    put_user(tmp, data as *mut c_ulong)
}

pub unsafe fn subarch_ptrace(child: *mut task_struct, request: c_long,
                             addr: c_ulong, data: c_ulong) -> c_long {
    let mut ret = -EIO;
    let datap = data as *mut c_void;
    match request {
        PTRACE_GETFPREGS => return copy_regset_to_user(child, task_user_regset_view(child), REGSET_FP_LEGACY, 0, core::mem::size_of::<user_i387_struct>(), datap),
        PTRACE_SETFPREGS => return copy_regset_from_user(child, task_user_regset_view(child), REGSET_FP_LEGACY, 0, core::mem::size_of::<user_i387_struct>(), datap),
        PTRACE_GETFPXREGS => return copy_regset_to_user(child, task_user_regset_view(child), REGSET_FP, 0, core::mem::size_of::<user_fxsr_struct>(), datap),
        PTRACE_SETFPXREGS => return copy_regset_from_user(child, task_user_regset_view(child), REGSET_FP, 0, core::mem::size_of::<user_fxsr_struct>(), datap),
        _ => ret = -EIO,
    }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
