// SPDX-License-Identifier: GPL-2.0+

/*
 * Ptrace test for hw breakpoints
 *
 * Based on tools/testing/selftests/breakpoints/breakpoint_test.c
 *
 * This test forks and the parent then traces the child doing various
 * types of ptrace enabled breakpoints
 *
 * Copyright (C) 2018 Michael Neuling, IBM Corporation.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem;
use core::ptr;

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;
type __u64 = u64;
type pid_t = c_int;

const SPRN_PVR: c_ulong = 0x11F;
const PVR_8xx: c_ulong = 0x00500000;

static mut is_8xx: bool = false;

/*
 * Use volatile on all global var so that compiler doesn't
 * optimise their load/stores. Otherwise selftest can fail.
 */
static mut glvar: __u64 = 0;

const DAWR_MAX_LEN: usize = 512;

#[repr(C, align(512))]
struct AlignedBigVar {
    v: [__u8; DAWR_MAX_LEN],
}

static mut big_var: AlignedBigVar = AlignedBigVar {
    v: [0; DAWR_MAX_LEN],
};

const A_LEN: usize = 6;
const B_LEN: usize = 6;

#[repr(C)]
struct gstruct_type {
    a: [__u8; A_LEN], /* double word aligned */
    b: [__u8; B_LEN], /* double word unaligned */
}

#[repr(C, align(512))]
struct AlignedGstruct {
    v: gstruct_type,
}

static mut gstruct: AlignedGstruct = AlignedGstruct {
    v: gstruct_type {
        a: [0; A_LEN],
        b: [0; B_LEN],
    },
};

const PATH_MAX: usize = 4096;

#[repr(C, align(8))]
struct AlignedCwd {
    v: [c_char; PATH_MAX],
}

static mut cwd: AlignedCwd = AlignedCwd { v: [0; PATH_MAX] };

#[repr(C)]
struct ppc_debug_info {
    version: __u32,
    num_instruction_bps: __u32,
    num_data_bps: __u32,
    num_condition_regs: __u32,
    data_bp_alignment: __u32,
    sizeof_condition: __u32,
    features: __u64,
}

#[repr(C)]
struct ppc_hw_breakpoint {
    version: __u32,
    trigger_type: __u32,
    addr_mode: __u32,
    condition_mode: __u32,
    addr: __u64,
    addr2: __u64,
    condition_value: __u64,
}

#[repr(C)]
struct siginfo_t {
    si_signo: c_int,
    si_errno: c_int,
    si_code: c_int,
    si_addr: *mut c_void,
}

unsafe extern "C" {
    fn ptrace(request: c_ulong, pid: pid_t, addr: *mut c_void, data: *mut c_void) -> c_long;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn getpid() -> pid_t;
    fn rand() -> c_int;
    fn syscall(num: c_long, ...) -> c_long;
    fn wait(status: *mut c_int) -> pid_t;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fork() -> pid_t;
    fn mfspr(reg: c_ulong) -> c_ulong;
    fn test_harness(test_function: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

const PTRACE_TRACEME: c_ulong = 0;
const PTRACE_CONT: c_ulong = 7;
const PTRACE_SINGLESTEP: c_ulong = 9;
const PTRACE_GETSIGINFO: c_ulong = 0x4202;
const PTRACE_SET_DEBUGREG: c_ulong = 26;
const PPC_PTRACE_GETHWDBGINFO: c_ulong = 0x89;
const PPC_PTRACE_SETHWDEBUG: c_ulong = 0x88;
const PPC_PTRACE_DELHWDEBUG: c_ulong = 0x87;
const PPC_DEBUG_FEATURE_DATA_BP_RANGE: __u64 = 0x2;
const PPC_DEBUG_FEATURE_DATA_BP_DAWR: __u64 = 0x10;
const PPC_BREAKPOINT_TRIGGER_READ: c_int = 1;
const PPC_BREAKPOINT_TRIGGER_WRITE: c_int = 2;
const PPC_BREAKPOINT_TRIGGER_RW: c_int = 3;
const PPC_BREAKPOINT_MODE_EXACT: __u32 = 0;
const PPC_BREAKPOINT_MODE_RANGE_INCLUSIVE: __u32 = 1;
const PPC_BREAKPOINT_CONDITION_NONE: __u32 = 0;
const SIGUSR1: c_int = 10;
const SIGTRAP: c_int = 5;
const __NR_getcwd: c_long = 182;
const TEST_PASS: c_int = 0;

fn WIFSTOPPED(status: c_int) -> bool {
    (status & 0xff) == 0x7f
}

fn WSTOPSIG(status: c_int) -> c_int {
    (status >> 8) & 0xff
}

unsafe fn SKIP_IF_MSG(cond: bool, msg: *const c_char) {
    if cond {
        printf(b"%s\n\0".as_ptr() as *const c_char, msg);
        exit(4);
    }
}

unsafe fn get_dbginfo(child_pid: pid_t, dbginfo: *mut ppc_debug_info) {
    if ptrace(
        PPC_PTRACE_GETHWDBGINFO,
        child_pid,
        ptr::null_mut(),
        dbginfo as *mut c_void,
    ) != 0
    {
        perror(b"Can't get breakpoint info\0".as_ptr() as *const c_char);
        exit(-1);
    }
}

unsafe fn dawr_present(dbginfo: *mut ppc_debug_info) -> bool {
    ((*dbginfo).features & PPC_DEBUG_FEATURE_DATA_BP_DAWR) != 0
}

unsafe fn write_var(len: c_int) {
    let mut pcvar: *mut __u8;
    let mut psvar: *mut __u16;
    let mut pivar: *mut __u32;
    let mut plvar: *mut __u64;

    match len {
        1 => {
            pcvar = ptr::addr_of_mut!(glvar) as *mut __u8;
            ptr::write_volatile(pcvar, 0xff);
        }
        2 => {
            psvar = ptr::addr_of_mut!(glvar) as *mut __u16;
            ptr::write_volatile(psvar, 0xffff);
        }
        4 => {
            pivar = ptr::addr_of_mut!(glvar) as *mut __u32;
            ptr::write_volatile(pivar, 0xffffffff);
        }
        8 => {
            plvar = ptr::addr_of_mut!(glvar);
            ptr::write_volatile(plvar, 0xffffffffffffffff);
        }
        _ => {}
    }
}

unsafe fn read_var(len: c_int) {
    let mut cvar: __u8;
    let mut svar: __u16;
    let mut ivar: __u32;
    let mut lvar: __u64;

    match len {
        1 => {
            cvar = ptr::read_volatile(ptr::addr_of!(glvar) as *const __u8);
            let _ = cvar;
        }
        2 => {
            svar = ptr::read_volatile(ptr::addr_of!(glvar) as *const __u16);
            let _ = svar;
        }
        4 => {
            ivar = ptr::read_volatile(ptr::addr_of!(glvar) as *const __u32);
            let _ = ivar;
        }
        8 => {
            lvar = ptr::read_volatile(ptr::addr_of!(glvar));
            let _ = lvar;
        }
        _ => {}
    }
}

unsafe fn test_workload() {
    let mut cvar: __u8;
    let mut ivar: __u32;
    let mut len: c_int = 0;

    if ptrace(PTRACE_TRACEME, 0, ptr::null_mut(), ptr::null_mut()) != 0 {
        perror(b"Child can't be traced?\0".as_ptr() as *const c_char);
        exit(-1);
    }

    /* Wake up father so that it sets up the first test */
    kill(getpid(), SIGUSR1);

    /* PTRACE_SET_DEBUGREG, WO test */
    len = 1;
    while len <= mem::size_of_val(&*ptr::addr_of!(glvar)) as c_int {
        write_var(len);
        len <<= 1;
    }

    /* PTRACE_SET_DEBUGREG, RO test */
    len = 1;
    while len <= mem::size_of_val(&*ptr::addr_of!(glvar)) as c_int {
        read_var(len);
        len <<= 1;
    }

    /* PTRACE_SET_DEBUGREG, RW test */
    len = 1;
    while len <= mem::size_of_val(&*ptr::addr_of!(glvar)) as c_int {
        if rand() % 2 != 0 {
            read_var(len);
        } else {
            write_var(len);
        }
        len <<= 1;
    }

    /* PTRACE_SET_DEBUGREG, Kernel Access Userspace test */
    syscall(
        __NR_getcwd,
        ptr::addr_of_mut!(cwd.v) as *mut c_void,
        PATH_MAX,
    );

    /* PPC_PTRACE_SETHWDEBUG, MODE_EXACT, WO test */
    write_var(1);

    /* PPC_PTRACE_SETHWDEBUG, MODE_EXACT, RO test */
    read_var(1);

    /* PPC_PTRACE_SETHWDEBUG, MODE_EXACT, RW test */
    if rand() % 2 != 0 {
        write_var(1);
    } else {
        read_var(1);
    }

    /* PPC_PTRACE_SETHWDEBUG, MODE_EXACT, Kernel Access Userspace test */
    syscall(
        __NR_getcwd,
        ptr::addr_of_mut!(cwd.v) as *mut c_void,
        PATH_MAX,
    );

    /* PPC_PTRACE_SETHWDEBUG, MODE_RANGE, DW ALIGNED, WO test */
    ptr::write_volatile(
        ptr::addr_of_mut!(gstruct.v.a).cast::<__u8>().add((rand() as usize) % A_LEN),
        b'a',
    );

    /* PPC_PTRACE_SETHWDEBUG, MODE_RANGE, DW ALIGNED, RO test */
    cvar = ptr::read_volatile(
        ptr::addr_of!(gstruct.v.a).cast::<__u8>().add((rand() as usize) % A_LEN),
    );
    let _ = cvar;

    /* PPC_PTRACE_SETHWDEBUG, MODE_RANGE, DW ALIGNED, RW test */
    if rand() % 2 != 0 {
        ptr::write_volatile(
            ptr::addr_of_mut!(gstruct.v.a).cast::<__u8>().add((rand() as usize) % A_LEN),
            b'a',
        );
    } else {
        cvar = ptr::read_volatile(
            ptr::addr_of!(gstruct.v.a).cast::<__u8>().add((rand() as usize) % A_LEN),
        );
        let _ = cvar;
    }

    /* PPC_PTRACE_SETHWDEBUG, MODE_RANGE, DW UNALIGNED, WO test */
    ptr::write_volatile(
        ptr::addr_of_mut!(gstruct.v.b).cast::<__u8>().add((rand() as usize) % B_LEN),
        b'b',
    );

    /* PPC_PTRACE_SETHWDEBUG, MODE_RANGE, DW UNALIGNED, RO test */
    cvar = ptr::read_volatile(
        ptr::addr_of!(gstruct.v.b).cast::<__u8>().add((rand() as usize) % B_LEN),
    );
    let _ = cvar;

    /* PPC_PTRACE_SETHWDEBUG, MODE_RANGE, DW UNALIGNED, RW test */
    if rand() % 2 != 0 {
        ptr::write_volatile(
            ptr::addr_of_mut!(gstruct.v.b).cast::<__u8>().add((rand() as usize) % B_LEN),
            b'b',
        );
    } else {
        cvar = ptr::read_volatile(
            ptr::addr_of!(gstruct.v.b).cast::<__u8>().add((rand() as usize) % B_LEN),
        );
        let _ = cvar;
    }

    /* PPC_PTRACE_SETHWDEBUG, MODE_RANGE, DW UNALIGNED, DAR OUTSIDE, RW test */
    if rand() % 2 != 0 {
        *(ptr::addr_of_mut!(gstruct.v.a).cast::<__u8>().add(4) as *mut c_int) = 10;
    } else {
        ivar = *(ptr::addr_of!(gstruct.v.a).cast::<__u8>().add(4) as *const c_int) as __u32;
        let _ = ivar;
    }

    /* PPC_PTRACE_SETHWDEBUG. DAWR_MAX_LEN. RW test */
    if rand() % 2 != 0 {
        ptr::write_volatile(
            ptr::addr_of_mut!(big_var.v).cast::<__u8>().add((rand() as usize) % DAWR_MAX_LEN),
            b'a',
        );
    } else {
        cvar = ptr::read_volatile(
            ptr::addr_of!(big_var.v).cast::<__u8>().add((rand() as usize) % DAWR_MAX_LEN),
        );
        let _ = cvar;
    }

    /* PPC_PTRACE_SETHWDEBUG 2, MODE_RANGE, DW ALIGNED, WO test */
    ptr::write_volatile(
        ptr::addr_of_mut!(gstruct.v.a).cast::<__u8>().add((rand() as usize) % A_LEN),
        b'a',
    );

    /* PPC_PTRACE_SETHWDEBUG 2, MODE_RANGE, DW UNALIGNED, RO test */
    cvar = ptr::read_volatile(
        ptr::addr_of!(gstruct.v.b).cast::<__u8>().add((rand() as usize) % B_LEN),
    );
    let _ = cvar;

    /* PPC_PTRACE_SETHWDEBUG 2, MODE_RANGE, DAWR Overlap, WO test */
    ptr::write_volatile(
        ptr::addr_of_mut!(gstruct.v.a).cast::<__u8>().add((rand() as usize) % A_LEN),
        b'a',
    );

    /* PPC_PTRACE_SETHWDEBUG 2, MODE_RANGE, DAWR Overlap, RO test */
    cvar = ptr::read_volatile(
        ptr::addr_of!(gstruct.v.a).cast::<__u8>().add((rand() as usize) % A_LEN),
    );
    let _ = cvar;
}

unsafe fn check_success(
    child_pid: pid_t,
    name: *const c_char,
    type_: *const c_char,
    mut saddr: c_ulong,
    len: c_int,
) {
    let mut status: c_int = 0;
    let mut siginfo: siginfo_t = mem::zeroed();
    let eaddr: c_ulong = (saddr + len as c_ulong - 1) | 0x7;

    saddr &= !0x7;

    /* Wait for the child to SIGTRAP */
    wait(&mut status);

    ptrace(
        PTRACE_GETSIGINFO,
        child_pid,
        ptr::null_mut(),
        &mut siginfo as *mut siginfo_t as *mut c_void,
    );

    if !WIFSTOPPED(status)
        || WSTOPSIG(status) != SIGTRAP
        || (siginfo.si_addr as c_ulong) < saddr
        || (siginfo.si_addr as c_ulong) > eaddr
    {
        printf(
            b"%s, %s, len: %d: Fail\n\0".as_ptr() as *const c_char,
            name,
            type_,
            len,
        );
        exit(-1);
    }

    printf(
        b"%s, %s, len: %d: Ok\n\0".as_ptr() as *const c_char,
        name,
        type_,
        len,
    );

    if !is_8xx {
        /*
         * For ptrace registered watchpoint, signal is generated
         * before executing load/store. Singlestep the instruction
         * and then continue the test.
         */
        ptrace(PTRACE_SINGLESTEP, child_pid, ptr::null_mut(), ptr::null_mut());
        wait(ptr::null_mut());
    }
}

unsafe fn ptrace_set_debugreg(child_pid: pid_t, wp_addr: c_ulong) {
    if ptrace(
        PTRACE_SET_DEBUGREG,
        child_pid,
        ptr::null_mut(),
        wp_addr as *mut c_void,
    ) != 0
    {
        perror(b"PTRACE_SET_DEBUGREG failed\0".as_ptr() as *const c_char);
        exit(-1);
    }
}

unsafe fn ptrace_sethwdebug(child_pid: pid_t, info: *mut ppc_hw_breakpoint) -> c_int {
    let wh = ptrace(
        PPC_PTRACE_SETHWDEBUG,
        child_pid,
        ptr::null_mut(),
        info as *mut c_void,
    ) as c_int;

    if wh <= 0 {
        perror(b"PPC_PTRACE_SETHWDEBUG failed\0".as_ptr() as *const c_char);
        exit(-1);
    }
    wh
}

unsafe fn ptrace_delhwdebug(child_pid: pid_t, wh: c_int) {
    if ptrace(
        PPC_PTRACE_DELHWDEBUG,
        child_pid,
        ptr::null_mut(),
        wh as usize as *mut c_void,
    ) < 0
    {
        perror(b"PPC_PTRACE_DELHWDEBUG failed\0".as_ptr() as *const c_char);
        exit(-1);
    }
}

const DABR_READ_SHIFT: c_int = 0;
const DABR_WRITE_SHIFT: c_int = 1;
const DABR_TRANSLATION_SHIFT: c_int = 2;

unsafe fn test_set_debugreg(child_pid: pid_t) -> c_int {
    let mut wp_addr: c_ulong = ptr::addr_of!(glvar) as c_ulong;
    let name = b"PTRACE_SET_DEBUGREG\0".as_ptr() as *const c_char;
    let mut len: c_int;

    /* PTRACE_SET_DEBUGREG, WO test*/
    wp_addr &= !0x7;
    wp_addr |= 1_u64 << DABR_WRITE_SHIFT;
    wp_addr |= 1_u64 << DABR_TRANSLATION_SHIFT;
    len = 1;
    while len <= mem::size_of_val(&*ptr::addr_of!(glvar)) as c_int {
        ptrace_set_debugreg(child_pid, wp_addr);
        ptrace(PTRACE_CONT, child_pid, ptr::null_mut(), ptr::null_mut());
        check_success(child_pid, name, b"WO\0".as_ptr() as *const c_char, wp_addr, len);
        len <<= 1;
    }

    /* PTRACE_SET_DEBUGREG, RO test */
    wp_addr &= !0x7;
    wp_addr |= 1_u64 << DABR_READ_SHIFT;
    wp_addr |= 1_u64 << DABR_TRANSLATION_SHIFT;
    len = 1;
    while len <= mem::size_of_val(&*ptr::addr_of!(glvar)) as c_int {
        ptrace_set_debugreg(child_pid, wp_addr);
        ptrace(PTRACE_CONT, child_pid, ptr::null_mut(), ptr::null_mut());
        check_success(child_pid, name, b"RO\0".as_ptr() as *const c_char, wp_addr, len);
        len <<= 1;
    }

    /* PTRACE_SET_DEBUGREG, RW test */
    wp_addr &= !0x7;
    wp_addr |= 1_u64 << DABR_READ_SHIFT;
    wp_addr |= 1_u64 << DABR_WRITE_SHIFT;
    wp_addr |= 1_u64 << DABR_TRANSLATION_SHIFT;
    len = 1;
    while len <= mem::size_of_val(&*ptr::addr_of!(glvar)) as c_int {
        ptrace_set_debugreg(child_pid, wp_addr);
        ptrace(PTRACE_CONT, child_pid, ptr::null_mut(), ptr::null_mut());
        check_success(child_pid, name, b"RW\0".as_ptr() as *const c_char, wp_addr, len);
        len <<= 1;
    }

    ptrace_set_debugreg(child_pid, 0);
    0
}

unsafe fn test_set_debugreg_kernel_userspace(child_pid: pid_t) -> c_int {
    let mut wp_addr: c_ulong = ptr::addr_of!(cwd.v) as c_ulong;
    let name = b"PTRACE_SET_DEBUGREG\0".as_ptr() as *const c_char;

    /* PTRACE_SET_DEBUGREG, Kernel Access Userspace test */
    wp_addr &= !0x7;
    wp_addr |= 1_u64 << DABR_READ_SHIFT;
    wp_addr |= 1_u64 << DABR_WRITE_SHIFT;
    wp_addr |= 1_u64 << DABR_TRANSLATION_SHIFT;
    ptrace_set_debugreg(child_pid, wp_addr);
    ptrace(PTRACE_CONT, child_pid, ptr::null_mut(), ptr::null_mut());
    check_success(
        child_pid,
        name,
        b"Kernel Access Userspace\0".as_ptr() as *const c_char,
        wp_addr,
        8,
    );

    ptrace_set_debugreg(child_pid, 0);
    0
}

unsafe fn get_ppc_hw_breakpoint(
    info: *mut ppc_hw_breakpoint,
    type_: c_int,
    addr: c_ulong,
    len: c_int,
) {
    (*info).version = 1;
    (*info).trigger_type = type_ as __u32;
    (*info).condition_mode = PPC_BREAKPOINT_CONDITION_NONE;
    (*info).addr = addr as __u64;
    (*info).addr2 = addr as __u64 + len as __u64;
    (*info).condition_value = 0;
    if len == 0 {
        (*info).addr_mode = PPC_BREAKPOINT_MODE_EXACT;
    } else {
        (*info).addr_mode = PPC_BREAKPOINT_MODE_RANGE_INCLUSIVE;
    }
}

unsafe fn test_sethwdebug_exact(child_pid: pid_t) {
    let mut info: ppc_hw_breakpoint = mem::zeroed();
    let wp_addr: c_ulong = ptr::addr_of!(glvar) as c_ulong;
    let name = b"PPC_PTRACE_SETHWDEBUG, MODE_EXACT\0".as_ptr() as *const c_char;
    let len: c_int = 1; /* hardcoded in kernel */
    let mut wh: c_int;

    /* PPC_PTRACE_SETHWDEBUG, MODE_EXACT, WO test */
    get_ppc_hw_breakpoint(&mut info, PPC_BREAKPOINT_TRIGGER_WRITE, wp_addr, 0);
    wh = ptrace_sethwdebug(child_pid, &mut info);
    ptrace(PTRACE_CONT, child_pid, ptr::null_mut(), ptr::null_mut());
    check_success(child_pid, name, b"WO\0".as_ptr() as *const c_char, wp_addr, len);
    ptrace_delhwdebug(child_pid, wh);

    /* PPC_PTRACE_SETHWDEBUG, MODE_EXACT, RO test */
    get_ppc_hw_breakpoint(&mut info, PPC_BREAKPOINT_TRIGGER_READ, wp_addr, 0);
    wh = ptrace_sethwdebug(child_pid, &mut info);
    ptrace(PTRACE_CONT, child_pid, ptr::null_mut(), ptr::null_mut());
    check_success(child_pid, name, b"RO\0".as_ptr() as *const c_char, wp_addr, len);
    ptrace_delhwdebug(child_pid, wh);

    /* PPC_PTRACE_SETHWDEBUG, MODE_EXACT, RW test */
    get_ppc_hw_breakpoint(&mut info, PPC_BREAKPOINT_TRIGGER_RW, wp_addr, 0);
    wh = ptrace_sethwdebug(child_pid, &mut info);
    ptrace(PTRACE_CONT, child_pid, ptr::null_mut(), ptr::null_mut());
    check_success(child_pid, name, b"RW\0".as_ptr() as *const c_char, wp_addr, len);
    ptrace_delhwdebug(child_pid, wh);
}

unsafe fn test_sethwdebug_exact_kernel_userspace(child_pid: pid_t) {
    let mut info: ppc_hw_breakpoint = mem::zeroed();
    let wp_addr: c_ulong = ptr::addr_of!(cwd.v) as c_ulong;
    let name = b"PPC_PTRACE_SETHWDEBUG, MODE_EXACT\0".as_ptr() as *const c_char;
    let len: c_int = 1; /* hardcoded in kernel */
    let mut wh: c_int;

    /* PPC_PTRACE_SETHWDEBUG, MODE_EXACT, Kernel Access Userspace test */
    get_ppc_hw_breakpoint(&mut info, PPC_BREAKPOINT_TRIGGER_WRITE, wp_addr, 0);
    wh = ptrace_sethwdebug(child_pid, &mut info);
    ptrace(PTRACE_CONT, child_pid, ptr::null_mut(), ptr::null_mut());
    check_success(
        child_pid,
        name,
        b"Kernel Access Userspace\0".as_ptr() as *const c_char,
        wp_addr,
        len,
    );
    ptrace_delhwdebug(child_pid, wh);
}

unsafe fn test_sethwdebug_range_aligned(child_pid: pid_t) {
    let mut info: ppc_hw_breakpoint = mem::zeroed();
    let mut wp_addr: c_ulong;
    let name = b"PPC_PTRACE_SETHWDEBUG, MODE_RANGE, DW ALIGNED\0".as_ptr() as *const c_char;
    let mut len: c_int;
    let mut wh: c_int;

    /* PPC_PTRACE_SETHWDEBUG, MODE_RANGE, DW ALIGNED, WO test */
    wp_addr = ptr::addr_of!(gstruct.v.a) as c_ulong;
    len = A_LEN as c_int;
    get_ppc_hw_breakpoint(&mut info, PPC_BREAKPOINT_TRIGGER_WRITE, wp_addr, len);
    wh = ptrace_sethwdebug(child_pid, &mut info);
    ptrace(PTRACE_CONT, child_pid, ptr::null_mut(), ptr::null_mut());
    check_success(child_pid, name, b"WO\0".as_ptr() as *const c_char, wp_addr, len);
    ptrace_delhwdebug(child_pid, wh);

    /* PPC_PTRACE_SETHWDEBUG, MODE_RANGE, DW ALIGNED, RO test */
    wp_addr = ptr::addr_of!(gstruct.v.a) as c_ulong;
    len = A_LEN as c_int;
    get_ppc_hw_breakpoint(&mut info, PPC_BREAKPOINT_TRIGGER_READ, wp_addr, len);
    wh = ptrace_sethwdebug(child_pid, &mut info);
    ptrace(PTRACE_CONT, child_pid, ptr::null_mut(), ptr::null_mut());
    check_success(child_pid, name, b"RO\0".as_ptr() as *const c_char, wp_addr, len);
    ptrace_delhwdebug(child_pid, wh);

    /* PPC_PTRACE_SETHWDEBUG, MODE_RANGE, DW ALIGNED, RW test */
    wp_addr = ptr::addr_of!(gstruct.v.a) as c_ulong;
    len = A_LEN as c_int;
    get_ppc_hw_breakpoint(&mut info, PPC_BREAKPOINT_TRIGGER_RW, wp_addr, len);
    wh = ptrace_sethwdebug(child_pid, &mut info);
    ptrace(PTRACE_CONT, child_pid, ptr::null_mut(), ptr::null_mut());
    check_success(child_pid, name, b"RW\0".as_ptr() as *const c_char, wp_addr, len);
    ptrace_delhwdebug(child_pid, wh);
}

unsafe fn test_multi_sethwdebug_range(child_pid: pid_t) {
    let mut info1: ppc_hw_breakpoint = mem::zeroed();
    let mut info2: ppc_hw_breakpoint = mem::zeroed();
    let mut wp_addr1: c_ulong;
    let mut wp_addr2: c_ulong;
    let name1 = b"PPC_PTRACE_SETHWDEBUG 2, MODE_RANGE, DW ALIGNED\0".as_ptr() as *const c_char;
    let name2 = b"PPC_PTRACE_SETHWDEBUG 2, MODE_RANGE, DW UNALIGNED\0".as_ptr() as *const c_char;
    let mut len1: c_int;
    let mut len2: c_int;
    let mut wh1: c_int;
    let mut wh2: c_int;

    wp_addr1 = ptr::addr_of!(gstruct.v.a) as c_ulong;
    wp_addr2 = ptr::addr_of!(gstruct.v.b) as c_ulong;
    len1 = A_LEN as c_int;
    len2 = B_LEN as c_int;
    get_ppc_hw_breakpoint(&mut info1, PPC_BREAKPOINT_TRIGGER_WRITE, wp_addr1, len1);
    get_ppc_hw_breakpoint(&mut info2, PPC_BREAKPOINT_TRIGGER_READ, wp_addr2, len2);

    /* PPC_PTRACE_SETHWDEBUG 2, MODE_RANGE, DW ALIGNED, WO test */
    wh1 = ptrace_sethwdebug(child_pid, &mut info1);

    /* PPC_PTRACE_SETHWDEBUG 2, MODE_RANGE, DW UNALIGNED, RO test */
    wh2 = ptrace_sethwdebug(child_pid, &mut info2);

    ptrace(PTRACE_CONT, child_pid, ptr::null_mut(), ptr::null_mut());
    check_success(child_pid, name1, b"WO\0".as_ptr() as *const c_char, wp_addr1, len1);

    ptrace(PTRACE_CONT, child_pid, ptr::null_mut(), ptr::null_mut());
    check_success(child_pid, name2, b"RO\0".as_ptr() as *const c_char, wp_addr2, len2);

    ptrace_delhwdebug(child_pid, wh1);
    ptrace_delhwdebug(child_pid, wh2);
}

unsafe fn test_multi_sethwdebug_range_dawr_overlap(child_pid: pid_t) {
    let mut info1: ppc_hw_breakpoint = mem::zeroed();
    let mut info2: ppc_hw_breakpoint = mem::zeroed();
    let mut wp_addr1: c_ulong;
    let mut wp_addr2: c_ulong;
    let name = b"PPC_PTRACE_SETHWDEBUG 2, MODE_RANGE, DAWR Overlap\0".as_ptr() as *const c_char;
    let mut len1: c_int;
    let mut len2: c_int;
    let mut wh1: c_int;
    let mut wh2: c_int;

    wp_addr1 = ptr::addr_of!(gstruct.v.a) as c_ulong;
    wp_addr2 = ptr::addr_of!(gstruct.v.a) as c_ulong;
    len1 = A_LEN as c_int;
    len2 = A_LEN as c_int;
    get_ppc_hw_breakpoint(&mut info1, PPC_BREAKPOINT_TRIGGER_WRITE, wp_addr1, len1);
    get_ppc_hw_breakpoint(&mut info2, PPC_BREAKPOINT_TRIGGER_READ, wp_addr2, len2);

    /* PPC_PTRACE_SETHWDEBUG 2, MODE_RANGE, DAWR Overlap, WO test */
    wh1 = ptrace_sethwdebug(child_pid, &mut info1);

    /* PPC_PTRACE_SETHWDEBUG 2, MODE_RANGE, DAWR Overlap, RO test */
    wh2 = ptrace_sethwdebug(child_pid, &mut info2);

    ptrace(PTRACE_CONT, child_pid, ptr::null_mut(), ptr::null_mut());
    check_success(child_pid, name, b"WO\0".as_ptr() as *const c_char, wp_addr1, len1);

    ptrace(PTRACE_CONT, child_pid, ptr::null_mut(), ptr::null_mut());
    check_success(child_pid, name, b"RO\0".as_ptr() as *const c_char, wp_addr2, len2);

    ptrace_delhwdebug(child_pid, wh1);
    ptrace_delhwdebug(child_pid, wh2);
}

unsafe fn test_sethwdebug_range_unaligned(child_pid: pid_t) {
    let mut info: ppc_hw_breakpoint = mem::zeroed();
    let mut wp_addr: c_ulong;
    let name = b"PPC_PTRACE_SETHWDEBUG, MODE_RANGE, DW UNALIGNED\0".as_ptr() as *const c_char;
    let mut len: c_int;
    let mut wh: c_int;

    /* PPC_PTRACE_SETHWDEBUG, MODE_RANGE, DW UNALIGNED, WO test */
    wp_addr = ptr::addr_of!(gstruct.v.b) as c_ulong;
    len = B_LEN as c_int;
    get_ppc_hw_breakpoint(&mut info, PPC_BREAKPOINT_TRIGGER_WRITE, wp_addr, len);
    wh = ptrace_sethwdebug(child_pid, &mut info);
    ptrace(PTRACE_CONT, child_pid, ptr::null_mut(), ptr::null_mut());
    check_success(child_pid, name, b"WO\0".as_ptr() as *const c_char, wp_addr, len);
    ptrace_delhwdebug(child_pid, wh);

    /* PPC_PTRACE_SETHWDEBUG, MODE_RANGE, DW UNALIGNED, RO test */
    wp_addr = ptr::addr_of!(gstruct.v.b) as c_ulong;
    len = B_LEN as c_int;
    get_ppc_hw_breakpoint(&mut info, PPC_BREAKPOINT_TRIGGER_READ, wp_addr, len);
    wh = ptrace_sethwdebug(child_pid, &mut info);
    ptrace(PTRACE_CONT, child_pid, ptr::null_mut(), ptr::null_mut());
    check_success(child_pid, name, b"RO\0".as_ptr() as *const c_char, wp_addr, len);
    ptrace_delhwdebug(child_pid, wh);

    /* PPC_PTRACE_SETHWDEBUG, MODE_RANGE, DW UNALIGNED, RW test */
    wp_addr = ptr::addr_of!(gstruct.v.b) as c_ulong;
    len = B_LEN as c_int;
    get_ppc_hw_breakpoint(&mut info, PPC_BREAKPOINT_TRIGGER_RW, wp_addr, len);
    wh = ptrace_sethwdebug(child_pid, &mut info);
    ptrace(PTRACE_CONT, child_pid, ptr::null_mut(), ptr::null_mut());
    check_success(child_pid, name, b"RW\0".as_ptr() as *const c_char, wp_addr, len);
    ptrace_delhwdebug(child_pid, wh);
}

unsafe fn test_sethwdebug_range_unaligned_dar(child_pid: pid_t) {
    let mut info: ppc_hw_breakpoint = mem::zeroed();
    let mut wp_addr: c_ulong;
    let name =
        b"PPC_PTRACE_SETHWDEBUG, MODE_RANGE, DW UNALIGNED, DAR OUTSIDE\0".as_ptr() as *const c_char;
    let mut len: c_int;
    let mut wh: c_int;

    /* PPC_PTRACE_SETHWDEBUG, MODE_RANGE, DW UNALIGNED, DAR OUTSIDE, RW test */
    wp_addr = ptr::addr_of!(gstruct.v.b) as c_ulong;
    len = B_LEN as c_int;
    get_ppc_hw_breakpoint(&mut info, PPC_BREAKPOINT_TRIGGER_WRITE, wp_addr, len);
    wh = ptrace_sethwdebug(child_pid, &mut info);
    ptrace(PTRACE_CONT, child_pid, ptr::null_mut(), ptr::null_mut());
    check_success(child_pid, name, b"RW\0".as_ptr() as *const c_char, wp_addr, len);
    ptrace_delhwdebug(child_pid, wh);
}

unsafe fn test_sethwdebug_dawr_max_range(child_pid: pid_t) {
    let mut info: ppc_hw_breakpoint = mem::zeroed();
    let mut wp_addr: c_ulong;
    let name = b"PPC_PTRACE_SETHWDEBUG, DAWR_MAX_LEN\0".as_ptr() as *const c_char;
    let mut len: c_int;
    let mut wh: c_int;

    /* PPC_PTRACE_SETHWDEBUG, DAWR_MAX_LEN, RW test */
    wp_addr = ptr::addr_of!(big_var.v) as c_ulong;
    len = DAWR_MAX_LEN as c_int;
    get_ppc_hw_breakpoint(&mut info, PPC_BREAKPOINT_TRIGGER_RW, wp_addr, len);
    wh = ptrace_sethwdebug(child_pid, &mut info);
    ptrace(PTRACE_CONT, child_pid, ptr::null_mut(), ptr::null_mut());
    check_success(child_pid, name, b"RW\0".as_ptr() as *const c_char, wp_addr, len);
    ptrace_delhwdebug(child_pid, wh);
}

/* Set the breakpoints and check the child successfully trigger them */
unsafe fn run_tests(child_pid: pid_t, dbginfo: *mut ppc_debug_info, dawr: bool) {
    test_set_debugreg(child_pid);
    test_set_debugreg_kernel_userspace(child_pid);
    test_sethwdebug_exact(child_pid);
    test_sethwdebug_exact_kernel_userspace(child_pid);
    if (*dbginfo).features & PPC_DEBUG_FEATURE_DATA_BP_RANGE != 0 {
        test_sethwdebug_range_aligned(child_pid);
        if dawr || is_8xx {
            test_sethwdebug_range_unaligned(child_pid);
            test_sethwdebug_range_unaligned_dar(child_pid);
            test_sethwdebug_dawr_max_range(child_pid);
            if (*dbginfo).num_data_bps > 1 {
                test_multi_sethwdebug_range(child_pid);
                test_multi_sethwdebug_range_dawr_overlap(child_pid);
            }
        }
    }
}

unsafe extern "C" fn ptrace_hwbreak() -> c_int {
    let mut child_pid: pid_t;
    let mut dbginfo: ppc_debug_info = mem::zeroed();
    let mut dawr: bool;

    child_pid = fork();
    if child_pid == 0 {
        test_workload();
        return 0;
    }

    wait(ptr::null_mut());

    get_dbginfo(child_pid, &mut dbginfo);
    SKIP_IF_MSG(
        dbginfo.num_data_bps == 0,
        b"No data breakpoints present\0".as_ptr() as *const c_char,
    );

    dawr = dawr_present(&mut dbginfo);
    run_tests(child_pid, &mut dbginfo, dawr);

    /* Let the child exit first. */
    ptrace(PTRACE_CONT, child_pid, ptr::null_mut(), ptr::null_mut());
    wait(ptr::null_mut());

    /*
     * Testcases exits immediately with -1 on any failure. If
     * it has reached here, it means all tests were successful.
     */
    TEST_PASS
}

#[no_mangle]
pub unsafe extern "C" fn main(
    _argc: c_int,
    _argv: *mut *mut c_char,
    _envp: *mut *mut c_char,
) -> c_int {
    is_8xx = mfspr(SPRN_PVR) == PVR_8xx;

    test_harness(
        ptrace_hwbreak,
        b"ptrace-hwbreak\0".as_ptr() as *const c_char,
    )
}
