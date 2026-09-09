/*
 * Copyright 2003 PathScale, Inc.
 * Copyright (C) 2003 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 *
 * Licensed under the GPL
 */

/* C headers and build-time architecture definitions are supplied by the surrounding kernel. */

const FLAG_MASK: c_ulong = 0x44dd5;

/* C designated initializers, retained as an index-to-host-register mapping. */
static mut REG_OFFSETS: [c_int; 27] = [
    HOST_R8, HOST_R9, HOST_R10, HOST_R11, HOST_R12, HOST_R13, HOST_R14, HOST_R15,
    HOST_IP, HOST_SP, HOST_AX, HOST_BX, HOST_CX, HOST_DX, HOST_SI, HOST_DI,
    HOST_BP, HOST_CS, HOST_SS, HOST_FS_BASE, HOST_GS_BASE, HOST_DS, HOST_ES,
    HOST_FS, HOST_GS, HOST_EFLAGS, HOST_ORIG_AX,
];

pub unsafe fn putreg(child: *mut task_struct, regno: c_int, mut value: c_ulong) -> c_int {
    match regno {
        R8 | R9 | R10 | R11 | R12 | R13 | R14 | R15 |
        RIP | RSP | RAX | RBX | RCX | RDX | RSI | RDI | RBP => {}
        ORIG_RAX => {
            /* Update the syscall number. */
            (*child).thread.regs.regs.gp[HOST_ORIG_AX as usize] = value;
        }
        FS | GS | DS | ES | SS | CS => {
            if value != 0 && (value & 3) != 3 { return -EIO; }
            value &= 0xffff;
        }
        FS_BASE | GS_BASE => {
            if !((value >> 48) == 0 || (value >> 48) == 0xffff) { return -EIO; }
        }
        EFLAGS => {
            value &= FLAG_MASK;
            (*child).thread.regs.regs.gp[HOST_EFLAGS] |= value;
            return 0;
        }
        _ => panic!("Bad register in putreg(): {}", regno),
    }
    (*child).thread.regs.regs.gp[REG_OFFSETS[(regno >> 3) as usize] as usize] = value;
    0
}

pub unsafe fn poke_user(child: *mut task_struct, mut addr: c_long, data: c_long) -> c_int {
    if (addr & 3) != 0 || addr < 0 { return -EIO; }
    if addr < MAX_REG_OFFSET { return putreg(child, addr as c_int, data as c_ulong); }
    if addr >= offsetof!(user, u_debugreg[0]) && addr <= offsetof!(user, u_debugreg[7]) {
        addr -= offsetof!(user, u_debugreg[0]);
        addr >>= 3;
        if addr == 4 || addr == 5 { return -EIO; }
        (*child).thread.arch.debugregs[addr as usize] = data as c_ulong;
        return 0;
    }
    -EIO
}

pub unsafe fn getreg(child: *mut task_struct, regno: c_int) -> c_ulong {
    let mut mask: c_ulong = !0;
    match regno {
        R8 | R9 | R10 | R11 | R12 | R13 | R14 | R15 | RIP | RSP |
        RAX | RBX | RCX | RDX | RSI | RDI | RBP | ORIG_RAX | EFLAGS |
        FS_BASE | GS_BASE => {}
        FS | GS | DS | ES | SS | CS => mask = 0xffff,
        _ => panic!("Bad register in getreg: {}", regno),
    }
    mask & (*child).thread.regs.regs.gp[REG_OFFSETS[(regno >> 3) as usize] as usize]
}

pub unsafe fn peek_user(child: *mut task_struct, mut addr: c_long, data: c_long) -> c_int {
    /* read the word at location addr in the USER area. */
    let mut tmp: c_ulong;
    if (addr & 3) != 0 || addr < 0 { return -EIO; }
    tmp = 0; /* Default return condition */
    if addr < MAX_REG_OFFSET {
        tmp = getreg(child, addr as c_int);
    } else if addr >= offsetof!(user, u_debugreg[0]) && addr <= offsetof!(user, u_debugreg[7]) {
        addr -= offsetof!(user, u_debugreg[0]);
        addr >>= 2;
        tmp = (*child).thread.arch.debugregs[addr as usize];
    }
    put_user(tmp, data as *mut c_ulong)
}

pub unsafe fn subarch_ptrace(child: *mut task_struct, request: c_long,
                             addr: c_ulong, data: c_ulong) -> c_long {
    let mut ret: c_int = -EIO;
    let datap = data as *mut c_void;
    match request {
        PTRACE_GETFPREGS => return copy_regset_to_user(child, task_user_regset_view(child), REGSET_FP, 0, core::mem::size_of::<user_i387_struct>(), datap),
        PTRACE_SETFPREGS => return copy_regset_from_user(child, task_user_regset_view(child), REGSET_FP, 0, core::mem::size_of::<user_i387_struct>(), datap),
        PTRACE_ARCH_PRCTL => {
            /* XXX Calls ptrace on the host - needs some SMP thinking */
            ret = arch_prctl(child, data, addr as *mut c_void);
        }
        _ => {}
    }
    ret as c_long
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
