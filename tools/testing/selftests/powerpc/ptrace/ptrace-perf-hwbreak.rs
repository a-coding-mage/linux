// SPDX-License-Identifier: GPL-2.0+

// Dependencies from the original C includes:
// asm/unistd.h, linux/hw_breakpoint.h, linux/ptrace.h, memory.h, stdlib.h,
// sys/wait.h, and "utils.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::size_of;

type pid_t = c_int;
type ssize_t = isize;
type u64 = u64;

#[repr(C)]
struct pt_regs {
    nip: c_ulong,
}

#[repr(C)]
struct ppc_debug_info {
    num_data_bps: c_int,
}

#[repr(C)]
struct ppc_hw_breakpoint {
    version: c_int,
    trigger_type: c_int,
    condition_mode: c_int,
    addr: u64,
    addr2: u64,
    condition_value: u64,
    addr_mode: c_int,
}

#[repr(C)]
struct perf_event_attr {
    type_: u32,
    size: u32,
    bp_type: u64,
    bp_addr: u64,
    bp_len: u64,
    exclude_kernel: u64,
    exclude_hv: u64,
}

extern "C" {
    static __NR_ptrace: c_long;
    static __NR_perf_event_open: c_long;

    static PTRACE_TRACEME: c_long;
    static PTRACE_GETREGS: c_long;
    static PTRACE_SETREGS: c_long;
    static PTRACE_CONT: c_long;
    static PTRACE_SINGLESTEP: c_long;
    static PPC_PTRACE_GETHWDBGINFO: c_long;
    static PPC_PTRACE_SETHWDEBUG: c_long;
    static PPC_PTRACE_DELHWDEBUG: c_long;

    static PERF_TYPE_BREAKPOINT: u32;
    static HW_BREAKPOINT_R: u64;
    static PPC_BREAKPOINT_CONDITION_NONE: c_int;
    static PPC_BREAKPOINT_MODE_EXACT: c_int;
    static PPC_BREAKPOINT_MODE_RANGE_INCLUSIVE: c_int;
    static PPC_BREAKPOINT_TRIGGER_READ: c_int;

    static SIGSTOP: c_int;
    static SIGKILL: c_int;

    fn syscall(num: c_long, ...) -> c_long;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> ssize_t;
    fn fork() -> pid_t;
    fn raise(sig: c_int) -> c_int;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn exit(status: c_int) -> !;

    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;

    /*
     * Child subroutine that performs a load on the address, then traps
     */
    fn same_watch_addr_child(addr: *mut c_ulong);

    /* Address of the ld instruction in same_watch_addr_child() */
    static mut same_watch_addr_load: c_char;

    /* Address of the end trap instruction in same_watch_addr_child() */
    static mut same_watch_addr_trap: c_char;

    /*
     * Child subroutine that performs a load on the first address, then a load on
     * the second address (with no instructions separating this from the first
     * load), then traps.
     */
    fn perf_then_ptrace_child(first_addr: *mut c_ulong, second_addr: *mut c_ulong);

    /* Address of the first ld instruction in perf_then_ptrace_child() */
    static mut perf_then_ptrace_load1: c_char;

    /* Address of the second ld instruction in perf_then_ptrace_child() */
    static mut perf_then_ptrace_load2: c_char;

    /* Address of the end trap instruction in perf_then_ptrace_child() */
    static mut perf_then_ptrace_trap: c_char;
}

macro_rules! FAIL_IF_MSG {
    ($cond:expr, $msg:expr) => {
        if $cond {
            return 1;
        }
    };
}

macro_rules! FAIL_IF_EXIT_MSG {
    ($cond:expr, $msg:expr) => {
        if $cond {
            exit(1);
        }
    };
}

macro_rules! SKIP_IF_MSG {
    ($cond:expr, $msg:expr) => {
        if $cond {
            return 0;
        }
    };
}

unsafe fn WIFSTOPPED(status: c_int) -> bool {
    (status & 0xff) == 0x7f
}

#[inline]
unsafe fn sys_ptrace(request: c_long, pid: pid_t, addr: c_ulong, data: c_ulong) -> c_long {
    syscall(__NR_ptrace, request, pid, addr, data)
}

unsafe fn ptrace_traceme() -> c_long {
    sys_ptrace(PTRACE_TRACEME, 0, 0, 0)
}

unsafe fn ptrace_getregs(pid: pid_t, result: *mut pt_regs) -> c_long {
    sys_ptrace(PTRACE_GETREGS, pid, 0, result as c_ulong)
}

unsafe fn ptrace_setregs(pid: pid_t, result: *mut pt_regs) -> c_long {
    sys_ptrace(PTRACE_SETREGS, pid, 0, result as c_ulong)
}

unsafe fn ptrace_cont(pid: pid_t, signal: c_long) -> c_long {
    sys_ptrace(PTRACE_CONT, pid, 0, signal as c_ulong)
}

unsafe fn ptrace_singlestep(pid: pid_t, signal: c_long) -> c_long {
    sys_ptrace(PTRACE_SINGLESTEP, pid, 0, signal as c_ulong)
}

unsafe fn ppc_ptrace_gethwdbginfo(pid: pid_t, dbginfo: *mut ppc_debug_info) -> c_long {
    sys_ptrace(PPC_PTRACE_GETHWDBGINFO, pid, 0, dbginfo as c_ulong)
}

unsafe fn ppc_ptrace_sethwdbg(pid: pid_t, bp_info: *mut ppc_hw_breakpoint) -> c_long {
    sys_ptrace(PPC_PTRACE_SETHWDEBUG, pid, 0, bp_info as c_ulong)
}

unsafe fn ppc_ptrace_delhwdbg(pid: pid_t, bp_id: c_int) -> c_long {
    sys_ptrace(PPC_PTRACE_DELHWDEBUG, pid, 0, bp_id as c_ulong)
}

unsafe fn ptrace_getreg_pc(pid: pid_t, pc: *mut *mut c_void) -> c_long {
    let mut regs: pt_regs = core::mem::zeroed();
    let mut err: c_long;

    err = ptrace_getregs(pid, &mut regs);
    if err != 0 {
        return err;
    }

    *pc = regs.nip as *mut c_void;

    0
}

unsafe fn ptrace_setreg_pc(pid: pid_t, pc: *mut c_void) -> c_long {
    let mut regs: pt_regs = core::mem::zeroed();
    let mut err: c_long;

    err = ptrace_getregs(pid, &mut regs);
    if err != 0 {
        return err;
    }

    regs.nip = pc as c_ulong;

    err = ptrace_setregs(pid, &mut regs);
    if err != 0 {
        return err;
    }

    0
}

unsafe fn perf_event_open(
    attr: *mut perf_event_attr,
    pid: pid_t,
    cpu: c_int,
    group_fd: c_int,
    flags: c_ulong,
) -> c_int {
    syscall(__NR_perf_event_open, attr, pid, cpu, group_fd, flags) as c_int
}

unsafe fn perf_user_event_attr_set(attr: *mut perf_event_attr, addr: *mut c_void, len: u64) {
    memset(
        attr as *mut c_void,
        0,
        size_of::<perf_event_attr>(),
    );

    (*attr).type_ = PERF_TYPE_BREAKPOINT;
    (*attr).size = size_of::<perf_event_attr>() as u32;
    (*attr).bp_type = HW_BREAKPOINT_R;
    (*attr).bp_addr = addr as u64;
    (*attr).bp_len = len;
    (*attr).exclude_kernel = 1;
    (*attr).exclude_hv = 1;
}

unsafe fn perf_watchpoint_open(child_pid: pid_t, addr: *mut c_void, len: u64) -> c_int {
    let mut attr: perf_event_attr = core::mem::zeroed();

    perf_user_event_attr_set(&mut attr, addr, len);
    perf_event_open(&mut attr, child_pid, -1, -1, 0)
}

unsafe fn perf_read_counter(perf_fd: c_int, count: *mut u64) -> c_int {
    /*
     * A perf counter is retrieved by the read() syscall. It contains
     * the current count as 8 bytes that are interpreted as a u64
     */
    let len: ssize_t = read(perf_fd, count as *mut c_void, size_of::<u64>());

    if len != size_of::<u64>() as ssize_t {
        return -1;
    }

    0
}

unsafe fn ppc_ptrace_init_breakpoint(
    info: *mut ppc_hw_breakpoint,
    type_: c_int,
    addr: *mut c_void,
    len: c_int,
) {
    (*info).version = 1;
    (*info).trigger_type = type_;
    (*info).condition_mode = PPC_BREAKPOINT_CONDITION_NONE;
    (*info).addr = addr as u64;
    (*info).addr2 = (addr as u64).wrapping_add(len as u64);
    (*info).condition_value = 0;
    if len == 0 {
        (*info).addr_mode = PPC_BREAKPOINT_MODE_EXACT;
    } else {
        (*info).addr_mode = PPC_BREAKPOINT_MODE_RANGE_INCLUSIVE;
    }
}

/*
 * Checks if we can place at least 2 watchpoints on the child process
 */
unsafe fn check_watchpoints(pid: pid_t) -> c_int {
    let mut dbginfo: ppc_debug_info = core::mem::zeroed();

    FAIL_IF_MSG!(
        ppc_ptrace_gethwdbginfo(pid, &mut dbginfo) != 0,
        "PPC_PTRACE_GETHWDBGINFO failed"
    );
    SKIP_IF_MSG!(
        dbginfo.num_data_bps <= 1,
        "Not enough data watchpoints (need at least 2)"
    );

    0
}

/*
 * Wrapper around a plain fork() call that sets up the child for
 * ptrace-ing. Both the parent and child return from this, though
 * the child is stopped until ptrace_cont(pid) is run by the parent.
 */
unsafe fn ptrace_fork_child(pid: *mut pid_t) -> c_int {
    let mut status: c_int = 0;

    *pid = fork();

    if *pid < 0 {
        FAIL_IF_MSG!(true, "Failed to fork child");
    }

    if *pid == 0 {
        FAIL_IF_EXIT_MSG!(ptrace_traceme() != 0, "PTRACE_TRACEME failed");
        FAIL_IF_EXIT_MSG!(raise(SIGSTOP) != 0, "Child failed to raise SIGSTOP");
    } else {
        /* Synchronise on child SIGSTOP */
        FAIL_IF_MSG!(
            waitpid(*pid, &mut status, 0) == -1,
            "Failed to wait for child"
        );
        FAIL_IF_MSG!(!WIFSTOPPED(status), "Child is not stopped");
    }

    0
}

/*
 * Tests the interaction between ptrace and perf watching the same data.
 *
 * We expect ptrace to take 'priority', as it is has before-execute
 * semantics.
 *
 * The perf counter should not be incremented yet because perf has after-execute
 * semantics. E.g., if ptrace changes the child PC, we don't even execute the
 * instruction at all.
 *
 * When the child is stopped for ptrace, we test both continue and single step.
 * Both should increment the perf counter. We also test changing the PC somewhere
 * different and stepping, which should not increment the perf counter.
 */
#[no_mangle]
pub unsafe extern "C" fn same_watch_addr_test() -> c_int {
    let mut bp_info: ppc_hw_breakpoint = core::mem::zeroed(); /* ptrace breakpoint info */
    let mut bp_id: c_int; /* Breakpoint handle of ptrace watchpoint */
    let mut perf_fd: c_int; /* File descriptor of perf performance counter */
    let mut perf_count: u64 = 0; /* Most recently fetched perf performance counter value */
    let mut pid: pid_t = 0; /* PID of child process */
    let mut pc: *mut c_void = core::ptr::null_mut(); /* Most recently fetched child PC value */
    let mut status: c_int = 0; /* Stop status of child after waitpid */
    let mut value: c_ulong = 0; /* Dummy value to be read/written to by child */
    let mut err: c_int;

    err = ptrace_fork_child(&mut pid);
    if err != 0 {
        return err;
    }

    if pid == 0 {
        same_watch_addr_child(&mut value);
        exit(1);
    }

    err = check_watchpoints(pid);
    if err != 0 {
        return err;
    }

    /* Place a perf watchpoint counter on value */
    perf_fd = perf_watchpoint_open(pid, &mut value as *mut _ as *mut c_void, size_of::<c_ulong>() as u64);
    FAIL_IF_MSG!(perf_fd < 0, "Failed to open perf performance counter");

    /* Place a ptrace watchpoint on value */
    ppc_ptrace_init_breakpoint(
        &mut bp_info,
        PPC_BREAKPOINT_TRIGGER_READ,
        &mut value as *mut _ as *mut c_void,
        size_of::<c_ulong>() as c_int,
    );
    bp_id = ppc_ptrace_sethwdbg(pid, &mut bp_info) as c_int;
    FAIL_IF_MSG!(bp_id < 0, "Failed to set ptrace watchpoint");

    /* Let the child run. It should stop on the ptrace watchpoint */
    FAIL_IF_MSG!(ptrace_cont(pid, 0) != 0, "Failed to continue child");

    FAIL_IF_MSG!(waitpid(pid, &mut status, 0) == -1, "Failed to wait for child");
    FAIL_IF_MSG!(!WIFSTOPPED(status), "Child is not stopped");
    FAIL_IF_MSG!(ptrace_getreg_pc(pid, &mut pc) != 0, "Failed to get child PC");
    FAIL_IF_MSG!(
        pc != (&raw mut same_watch_addr_load) as *mut c_void,
        "Child did not stop on load instruction"
    );

    /*
     * We stopped before executing the load, so perf should not have
     * recorded any events yet
     */
    FAIL_IF_MSG!(perf_read_counter(perf_fd, &mut perf_count) != 0, "Failed to read perf counter");
    FAIL_IF_MSG!(perf_count != 0, "perf recorded unexpected event");

    /* Single stepping over the load should increment the perf counter */
    FAIL_IF_MSG!(ptrace_singlestep(pid, 0) != 0, "Failed to single step child");

    FAIL_IF_MSG!(waitpid(pid, &mut status, 0) == -1, "Failed to wait for child");
    FAIL_IF_MSG!(!WIFSTOPPED(status), "Child is not stopped");
    FAIL_IF_MSG!(ptrace_getreg_pc(pid, &mut pc) != 0, "Failed to get child PC");
    FAIL_IF_MSG!(
        pc != ((&raw mut same_watch_addr_load) as *mut c_char).add(4) as *mut c_void,
        "Failed to single step load instruction"
    );
    FAIL_IF_MSG!(perf_read_counter(perf_fd, &mut perf_count) != 0, "Failed to read perf counter");
    FAIL_IF_MSG!(perf_count != 1, "perf counter did not increment");

    /*
     * Set up a ptrace watchpoint on the value again and trigger it.
     * The perf counter should not have incremented because we do not
     * execute the load yet.
     */
    FAIL_IF_MSG!(
        ppc_ptrace_delhwdbg(pid, bp_id) != 0,
        "Failed to remove old ptrace watchpoint"
    );
    bp_id = ppc_ptrace_sethwdbg(pid, &mut bp_info) as c_int;
    FAIL_IF_MSG!(bp_id < 0, "Failed to set ptrace watchpoint");
    FAIL_IF_MSG!(
        ptrace_setreg_pc(pid, (&raw mut same_watch_addr_load) as *mut c_void) != 0,
        "Failed to set child PC"
    );
    FAIL_IF_MSG!(ptrace_cont(pid, 0) != 0, "Failed to continue child");

    FAIL_IF_MSG!(waitpid(pid, &mut status, 0) == -1, "Failed to wait for child");
    FAIL_IF_MSG!(!WIFSTOPPED(status), "Child is not stopped");
    FAIL_IF_MSG!(ptrace_getreg_pc(pid, &mut pc) != 0, "Failed to get child PC");
    FAIL_IF_MSG!(
        pc != (&raw mut same_watch_addr_load) as *mut c_void,
        "Child did not stop on load trap"
    );
    FAIL_IF_MSG!(perf_read_counter(perf_fd, &mut perf_count) != 0, "Failed to read perf counter");
    FAIL_IF_MSG!(perf_count != 1, "perf counter should not have changed");

    /* Continuing over the load should increment the perf counter */
    FAIL_IF_MSG!(ptrace_cont(pid, 0) != 0, "Failed to continue child");

    FAIL_IF_MSG!(waitpid(pid, &mut status, 0) == -1, "Failed to wait for child");
    FAIL_IF_MSG!(!WIFSTOPPED(status), "Child is not stopped");
    FAIL_IF_MSG!(ptrace_getreg_pc(pid, &mut pc) != 0, "Failed to get child PC");
    FAIL_IF_MSG!(
        pc != (&raw mut same_watch_addr_trap) as *mut c_void,
        "Child did not stop on end trap"
    );
    FAIL_IF_MSG!(perf_read_counter(perf_fd, &mut perf_count) != 0, "Failed to read perf counter");
    FAIL_IF_MSG!(perf_count != 2, "perf counter did not increment");

    /*
     * If we set the child PC back to the load instruction, then continue,
     * we should reach the end trap (because ptrace is one-shot) and have
     * another perf event.
     */
    FAIL_IF_MSG!(
        ptrace_setreg_pc(pid, (&raw mut same_watch_addr_load) as *mut c_void) != 0,
        "Failed to set child PC"
    );
    FAIL_IF_MSG!(ptrace_cont(pid, 0) != 0, "Failed to continue child");

    FAIL_IF_MSG!(waitpid(pid, &mut status, 0) == -1, "Failed to wait for child");
    FAIL_IF_MSG!(!WIFSTOPPED(status), "Child is not stopped");
    FAIL_IF_MSG!(ptrace_getreg_pc(pid, &mut pc) != 0, "Failed to get child PC");
    FAIL_IF_MSG!(
        pc != (&raw mut same_watch_addr_trap) as *mut c_void,
        "Child did not stop on end trap"
    );
    FAIL_IF_MSG!(perf_read_counter(perf_fd, &mut perf_count) != 0, "Failed to read perf counter");
    FAIL_IF_MSG!(perf_count != 3, "perf counter did not increment");

    /*
     * If we set the child PC back to the load instruction, set a ptrace
     * watchpoint on the load, then continue, we should immediately get
     * the ptrace trap without incrementing the perf counter
     */
    FAIL_IF_MSG!(
        ppc_ptrace_delhwdbg(pid, bp_id) != 0,
        "Failed to remove old ptrace watchpoint"
    );
    bp_id = ppc_ptrace_sethwdbg(pid, &mut bp_info) as c_int;
    FAIL_IF_MSG!(bp_id < 0, "Failed to set ptrace watchpoint");
    FAIL_IF_MSG!(
        ptrace_setreg_pc(pid, (&raw mut same_watch_addr_load) as *mut c_void) != 0,
        "Failed to set child PC"
    );
    FAIL_IF_MSG!(ptrace_cont(pid, 0) != 0, "Failed to continue child");

    FAIL_IF_MSG!(waitpid(pid, &mut status, 0) == -1, "Failed to wait for child");
    FAIL_IF_MSG!(!WIFSTOPPED(status), "Child is not stopped");
    FAIL_IF_MSG!(ptrace_getreg_pc(pid, &mut pc) != 0, "Failed to get child PC");
    FAIL_IF_MSG!(
        pc != (&raw mut same_watch_addr_load) as *mut c_void,
        "Child did not stop on load instruction"
    );
    FAIL_IF_MSG!(perf_read_counter(perf_fd, &mut perf_count) != 0, "Failed to read perf counter");
    FAIL_IF_MSG!(perf_count != 3, "perf counter should not have changed");

    /*
     * If we change the PC while stopped on the load instruction, we should
     * not increment the perf counter (because ptrace is before-execute,
     * perf is after-execute).
     */
    FAIL_IF_MSG!(
        ptrace_setreg_pc(
            pid,
            ((&raw mut same_watch_addr_load) as *mut c_char).add(4) as *mut c_void,
        ) != 0,
        "Failed to set child PC"
    );
    FAIL_IF_MSG!(ptrace_cont(pid, 0) != 0, "Failed to continue child");

    FAIL_IF_MSG!(waitpid(pid, &mut status, 0) == -1, "Failed to wait for child");
    FAIL_IF_MSG!(!WIFSTOPPED(status), "Child is not stopped");
    FAIL_IF_MSG!(ptrace_getreg_pc(pid, &mut pc) != 0, "Failed to get child PC");
    FAIL_IF_MSG!(
        pc != (&raw mut same_watch_addr_trap) as *mut c_void,
        "Child did not stop on end trap"
    );
    FAIL_IF_MSG!(perf_read_counter(perf_fd, &mut perf_count) != 0, "Failed to read perf counter");
    FAIL_IF_MSG!(perf_count != 3, "perf counter should not have changed");

    /* Clean up child */
    FAIL_IF_MSG!(kill(pid, SIGKILL) != 0, "Failed to kill child");

    0
}

/*
 * Tests the interaction between ptrace and perf when:
 * 1. perf watches a value
 * 2. ptrace watches a different value
 * 3. The perf value is read, then the ptrace value is read immediately after
 *
 * A breakpoint implementation may accidentally misattribute/skip one of
 * the ptrace or perf handlers, as interrupt based work is done after perf
 * and before ptrace.
 *
 * We expect the perf counter to increment before the ptrace watchpoint
 * triggers.
 */
#[no_mangle]
pub unsafe extern "C" fn perf_then_ptrace_test() -> c_int {
    let mut bp_info: ppc_hw_breakpoint = core::mem::zeroed(); /* ptrace breakpoint info */
    let mut bp_id: c_int; /* Breakpoint handle of ptrace watchpoint */
    let mut perf_fd: c_int; /* File descriptor of perf performance counter */
    let mut perf_count: u64 = 0; /* Most recently fetched perf performance counter value */
    let mut pid: pid_t = 0; /* PID of child process */
    let mut pc: *mut c_void = core::ptr::null_mut(); /* Most recently fetched child PC value */
    let mut status: c_int = 0; /* Stop status of child after waitpid */
    let mut perf_value: c_ulong = 0; /* Dummy value to be watched by perf */
    let mut ptrace_value: c_ulong = 0; /* Dummy value to be watched by ptrace */
    let mut err: c_int;

    err = ptrace_fork_child(&mut pid);
    if err != 0 {
        return err;
    }

    /*
     * If we are the child, run a subroutine that reads the perf value,
     * then reads the ptrace value with consecutive load instructions
     */
    if pid == 0 {
        perf_then_ptrace_child(&mut perf_value, &mut ptrace_value);
        exit(0);
    }

    err = check_watchpoints(pid);
    if err != 0 {
        return err;
    }

    /* Place a perf watchpoint counter */
    perf_fd = perf_watchpoint_open(
        pid,
        &mut perf_value as *mut _ as *mut c_void,
        size_of::<c_ulong>() as u64,
    );
    FAIL_IF_MSG!(perf_fd < 0, "Failed to open perf performance counter");

    /* Place a ptrace watchpoint */
    ppc_ptrace_init_breakpoint(
        &mut bp_info,
        PPC_BREAKPOINT_TRIGGER_READ,
        &mut ptrace_value as *mut _ as *mut c_void,
        size_of::<c_ulong>() as c_int,
    );
    bp_id = ppc_ptrace_sethwdbg(pid, &mut bp_info) as c_int;
    FAIL_IF_MSG!(bp_id < 0, "Failed to set ptrace watchpoint");

    /* Let the child run. It should stop on the ptrace watchpoint */
    FAIL_IF_MSG!(ptrace_cont(pid, 0) != 0, "Failed to continue child");

    FAIL_IF_MSG!(waitpid(pid, &mut status, 0) == -1, "Failed to wait for child");
    FAIL_IF_MSG!(!WIFSTOPPED(status), "Child is not stopped");
    FAIL_IF_MSG!(ptrace_getreg_pc(pid, &mut pc) != 0, "Failed to get child PC");
    FAIL_IF_MSG!(
        pc != (&raw mut perf_then_ptrace_load2) as *mut c_void,
        "Child did not stop on ptrace load"
    );

    /* perf should have recorded the first load */
    FAIL_IF_MSG!(perf_read_counter(perf_fd, &mut perf_count) != 0, "Failed to read perf counter");
    FAIL_IF_MSG!(perf_count != 1, "perf counter did not increment");

    /* Clean up child */
    FAIL_IF_MSG!(kill(pid, SIGKILL) != 0, "Failed to kill child");

    0
}

#[no_mangle]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let mut err: c_int = 0;

    err |= test_harness(same_watch_addr_test, c"same_watch_addr".as_ptr());
    err |= test_harness(perf_then_ptrace_test, c"perf_then_ptrace".as_ptr());

    err
}
