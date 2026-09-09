// SPDX-License-Identifier: GPL-2.0-only
/* PowerPC backend to the KGDB stub. */

// Kernel headers and configuration symbols are supplied by the surrounding
// translation unit.

#[repr(C)]
pub struct hard_trap_info { pub tt: u32, pub signo: u8 }

static mut HARD_TRAP_INFO: [hard_trap_info; 25] = [
    hard_trap_info{tt:0x0100,signo:2}, hard_trap_info{tt:0x0200,signo:11},
    hard_trap_info{tt:0x0300,signo:11}, hard_trap_info{tt:0x0400,signo:11},
    hard_trap_info{tt:0x0500,signo:2}, hard_trap_info{tt:0x0600,signo:10},
    hard_trap_info{tt:0x0700,signo:5}, hard_trap_info{tt:0x0800,signo:8},
    hard_trap_info{tt:0x0900,signo:14}, hard_trap_info{tt:0x0c00,signo:20},
    // CONFIG_BOOKE / CONFIG_PPC_* conditional entries are retained by build configuration.
    hard_trap_info{tt:0x0d00,signo:5}, hard_trap_info{tt:0x0f00,signo:4},
    hard_trap_info{tt:0x0f20,signo:8}, hard_trap_info{tt:0x1300,signo:5},
    hard_trap_info{tt:0x1200,signo:5}, hard_trap_info{tt:0x1500,signo:4},
    hard_trap_info{tt:0x1600,signo:4}, hard_trap_info{tt:0x1700,signo:8},
    hard_trap_info{tt:0x1800,signo:4}, hard_trap_info{tt:0x1400,signo:2},
    hard_trap_info{tt:0x1700,signo:4}, hard_trap_info{tt:0x2000,signo:5},
    hard_trap_info{tt:0,signo:0}, hard_trap_info{tt:0,signo:0}, hard_trap_info{tt:0,signo:0},
];

unsafe fn computeSignal(tt: u32) -> i32 {
    let mut i = 0usize;
    while unsafe { HARD_TRAP_INFO[i].tt != 0 && HARD_TRAP_INFO[i].signo != 0 } {
        if unsafe { HARD_TRAP_INFO[i].tt == tt } { return unsafe { HARD_TRAP_INFO[i].signo as i32 }; }
        i += 1;
    }
    SIGHUP
}

pub unsafe fn kgdb_skipexception(_exception: i32, regs: *mut pt_regs) -> i32 {
    kgdb_isremovedbreak((*regs).nip)
}

extern "C" {
    static mut __debugger_ipi: *mut core::ffi::c_void;
    static mut __debugger: *mut core::ffi::c_void;
    static mut __debugger_bpt: *mut core::ffi::c_void;
    static mut __debugger_sstep: *mut core::ffi::c_void;
    static mut __debugger_iabr_match: *mut core::ffi::c_void;
    static mut __debugger_break_match: *mut core::ffi::c_void;
    static mut __debugger_fault_handler: *mut core::ffi::c_void;
}

unsafe fn kgdb_debugger_ipi(regs: *mut pt_regs) -> i32 {
    kgdb_nmicallback(raw_smp_processor_id(), regs); 0
}

#[cfg(CONFIG_SMP)]
pub unsafe fn kgdb_roundup_cpus() { smp_send_debugger_break(); }

unsafe fn kgdb_debugger(regs: *mut pt_regs) -> i32 {
    (!kgdb_handle_exception(1, computeSignal(TRAP(regs)), DIE_OOPS, regs)) as i32
}
unsafe fn kgdb_handle_breakpoint(regs: *mut pt_regs) -> i32 {
    if user_mode(regs) { return 0; }
    if kgdb_handle_exception(1, SIGTRAP, 0, regs) != 0 { return 0; }
    if *( (*regs).nip as *const u32) == BREAK_INSTR { regs_add_return_ip(regs, BREAK_INSTR_SIZE); }
    1
}
unsafe fn kgdb_singlestep(regs: *mut pt_regs) -> i32 {
    if user_mode(regs) { return 0; }
    kgdb_handle_exception(0, SIGTRAP, 0, regs); 1
}
unsafe fn kgdb_iabr_match(regs: *mut pt_regs) -> i32 {
    if user_mode(regs) { return 0; }
    if kgdb_handle_exception(0, computeSignal(TRAP(regs)), 0, regs) != 0 { return 0; } 1
}
unsafe fn kgdb_break_match(regs: *mut pt_regs) -> i32 { kgdb_iabr_match(regs) }

pub unsafe fn sleeping_thread_to_gdb_regs(gdb_regs: *mut usize, p: *mut task_struct) {
    let regs = ( (*p).thread.ksp + STACK_INT_FRAME_REGS) as *mut pt_regs;
    core::ptr::write_bytes(gdb_regs as *mut u8, 0, NUMREGBYTES as usize);
    let mut ptr = gdb_regs;
    for reg in 0..3 { *ptr = (*regs).gpr[reg]; ptr = ptr.add(1); }
    ptr = ptr.add(11);
    for reg in 14..32 { *ptr = (*regs).gpr[reg]; ptr = ptr.add(1); }
    ptr = ptr.add(32 * 8 / core::mem::size_of::<usize>());
    *ptr = (*regs).nip; ptr = ptr.add(1); *ptr = (*regs).msr; ptr = ptr.add(1);
    *(ptr as *mut u32) = (*regs).ccr; ptr = (ptr as *mut u32).add(1) as *mut usize;
    *ptr = (*regs).link; ptr = ptr.add(1); *ptr = (*regs).ctr; ptr = ptr.add(1);
    *(ptr as *mut u32) = (*regs).xer;
}

#[repr(C)]
pub struct dbg_reg_def_t { pub name: *const u8, pub size: usize, pub offset: usize }
pub static mut dbg_reg_def: [dbg_reg_def_t; DBG_MAX_REG_NUM as usize] = [dbg_reg_def_t { name: core::ptr::null(), size: 0, offset: 0 }; DBG_MAX_REG_NUM as usize];

pub unsafe fn dbg_get_reg(regno: i32, mem: *mut core::ffi::c_void, regs: *mut pt_regs) -> *mut u8 {
    if regno >= DBG_MAX_REG_NUM || regno < 0 { return core::ptr::null_mut(); }
    let d = &dbg_reg_def[regno as usize];
    if regno < 32 || regno >= 64 { core::ptr::copy_nonoverlapping((regs as *mut u8).add(d.offset), mem as *mut u8, d.size); }
    if regno >= 32 && regno < 64 { core::ptr::write_bytes(mem as *mut u8, 0, d.size); }
    d.name as *mut u8
}
pub unsafe fn dbg_set_reg(regno: i32, mem: *const core::ffi::c_void, regs: *mut pt_regs) -> i32 {
    if regno >= DBG_MAX_REG_NUM || regno < 0 { return -EINVAL; }
    let d = &dbg_reg_def[regno as usize];
    if regno < 32 || regno >= 64 { core::ptr::copy_nonoverlapping(mem as *const u8, (regs as *mut u8).add(d.offset), d.size); }
    0
}
pub unsafe fn kgdb_arch_set_pc(regs: *mut pt_regs, pc: usize) { regs_set_return_ip(regs, pc); }

pub unsafe fn kgdb_arch_handle_exception(_vector: i32, _signo: i32, _err_code: i32, input: *mut u8, _output: *mut u8, regs: *mut pt_regs) -> i32 {
    let mut ptr = input.add(1); let mut addr = 0usize;
    match *input as char {
        's' | 'c' => {
            if kgdb_hex2long(&mut ptr, &mut addr) != 0 { regs_set_return_ip(regs, addr); }
            atomic_set(&mut kgdb_cpu_doing_single_step, -1);
            if *input as char == 's' { regs_set_return_msr(regs, (*regs).msr | MSR_SE); atomic_set(&mut kgdb_cpu_doing_single_step, raw_smp_processor_id()); }
            0
        },
        _ => -1,
    }
}

pub unsafe fn kgdb_arch_set_breakpoint(bpt: *mut kgdb_bkpt) -> i32 {
    let mut instr = 0u32; let addr = (*bpt).bpt_addr as *mut u32;
    let mut err = get_kernel_nofault(&mut instr, addr); if err != 0 { return err; }
    err = patch_instruction(addr, ppc_inst(BREAK_INSTR)); if err != 0 { return -EFAULT; }
    *((*bpt).saved_instr as *mut u32) = instr; 0
}
pub unsafe fn kgdb_arch_remove_breakpoint(bpt: *mut kgdb_bkpt) -> i32 {
    if patch_instruction((*bpt).bpt_addr as *mut u32, ppc_inst(*((*bpt).saved_instr as *mut u32))) != 0 { return -EFAULT; } 0
}

pub static arch_kgdb_ops: kgdb_arch;
unsafe fn kgdb_not_implemented(_regs: *mut pt_regs) -> i32 { 0 }
static mut old__debugger_ipi: *mut core::ffi::c_void = core::ptr::null_mut();
static mut old__debugger: *mut core::ffi::c_void = core::ptr::null_mut();
static mut old__debugger_bpt: *mut core::ffi::c_void = core::ptr::null_mut();
static mut old__debugger_sstep: *mut core::ffi::c_void = core::ptr::null_mut();
static mut old__debugger_iabr_match: *mut core::ffi::c_void = core::ptr::null_mut();
static mut old__debugger_break_match: *mut core::ffi::c_void = core::ptr::null_mut();
static mut old__debugger_fault_handler: *mut core::ffi::c_void = core::ptr::null_mut();

pub unsafe fn kgdb_arch_init() -> i32 {
    old__debugger_ipi=__debugger_ipi; old__debugger=__debugger; old__debugger_bpt=__debugger_bpt; old__debugger_sstep=__debugger_sstep; old__debugger_iabr_match=__debugger_iabr_match; old__debugger_break_match=__debugger_break_match; old__debugger_fault_handler=__debugger_fault_handler;
    __debugger_ipi=kgdb_debugger_ipi as *mut _; __debugger=kgdb_debugger as *mut _; __debugger_bpt=kgdb_handle_breakpoint as *mut _; __debugger_sstep=kgdb_singlestep as *mut _; __debugger_iabr_match=kgdb_iabr_match as *mut _; __debugger_break_match=kgdb_break_match as *mut _; __debugger_fault_handler=kgdb_not_implemented as *mut _; 0
}
pub unsafe fn kgdb_arch_exit() { __debugger_ipi=old__debugger_ipi; __debugger=old__debugger; __debugger_bpt=old__debugger_bpt; __debugger_sstep=old__debugger_sstep; __debugger_iabr_match=old__debugger_iabr_match; __debugger_break_match=old__debugger_break_match; __debugger_fault_handler=old__debugger_fault_handler; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
