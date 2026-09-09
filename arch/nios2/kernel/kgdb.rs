// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Nios2 KGDB support
 *
 * Copyright (C) 2015 Altera Corporation
 * Copyright (C) 2011 Tobias Klauser <tklauser@distanz.ch>
 *
 * Based on the code posted by Kazuyasu on the Altera Forum at:
 * http://www.alteraforum.com/forum/showpost.php?p=77003&postcount=20
 */

// C dependencies: linux/ptrace.h, linux/kgdb.h, linux/kdebug.h, linux/io.h

static mut wait_for_remote_debugger: ::core::ffi::c_int = 0;

#[repr(C)]
pub struct dbg_reg_def_t {
    pub name: *const ::core::ffi::c_char,
    pub size: ::core::ffi::c_int,
    pub offset: isize,
}

// External types, constants, and functions supplied by the Linux kernel bindings.
#[repr(C)]
pub struct pt_regs {
    pub r1: usize,
    pub r2: usize,
    pub r3: usize,
    pub r4: usize,
    pub r5: usize,
    pub r6: usize,
    pub r7: usize,
    pub r8: usize,
    pub r9: usize,
    pub r10: usize,
    pub r11: usize,
    pub r12: usize,
    pub r13: usize,
    pub r14: usize,
    pub r15: usize,
    pub gp: usize,
    pub sp: usize,
    pub fp: usize,
    pub ra: usize,
    pub ea: usize,
    pub estatus: usize,
}

#[repr(C)]
pub struct task_struct {
    pub thread: thread_struct,
}

#[repr(C)]
pub struct thread_struct {
    pub kregs: *mut pt_regs,
}

pub const DBG_MAX_REG_NUM: usize = 53;
pub const GDB_SIZEOF_REG: ::core::ffi::c_int = 4;
pub const NUMREGBYTES: usize = 4 * GDB_SIZEOF_REG as usize;
pub const GDB_SP: usize = 27;
pub const GDB_PC: usize = 32;
pub const EINVAL: ::core::ffi::c_int = 22;
pub const SIGTRAP: ::core::ffi::c_int = 5;

extern "C" {
    pub fn kgdb_hex2long(ptr: *mut *mut ::core::ffi::c_char, addr: *mut usize) -> bool;
    pub fn kgdb_handle_exception(
        vector: ::core::ffi::c_int,
        signo: ::core::ffi::c_int,
        err_code: ::core::ffi::c_int,
        regs: *mut pt_regs,
    );
}

pub static mut dbg_reg_def: [dbg_reg_def_t; DBG_MAX_REG_NUM] = [
    dbg_reg_def_t { name: b"zero\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: -1 },
    dbg_reg_def_t { name: b"at\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: 8 },
    dbg_reg_def_t { name: b"r2\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: 16 },
    dbg_reg_def_t { name: b"r3\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: 24 },
    dbg_reg_def_t { name: b"r4\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: 32 },
    dbg_reg_def_t { name: b"r5\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: 40 },
    dbg_reg_def_t { name: b"r6\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: 48 },
    dbg_reg_def_t { name: b"r7\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: 56 },
    dbg_reg_def_t { name: b"r8\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: 64 },
    dbg_reg_def_t { name: b"r9\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: 72 },
    dbg_reg_def_t { name: b"r10\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: 80 },
    dbg_reg_def_t { name: b"r11\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: 88 },
    dbg_reg_def_t { name: b"r12\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: 96 },
    dbg_reg_def_t { name: b"r13\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: 104 },
    dbg_reg_def_t { name: b"r14\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: 112 },
    dbg_reg_def_t { name: b"r15\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: 120 },
    dbg_reg_def_t { name: b"r16\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: -1 },
    dbg_reg_def_t { name: b"r17\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: -1 },
    dbg_reg_def_t { name: b"r18\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: -1 },
    dbg_reg_def_t { name: b"r19\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: -1 },
    dbg_reg_def_t { name: b"r20\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: -1 },
    dbg_reg_def_t { name: b"r21\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: -1 },
    dbg_reg_def_t { name: b"r22\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: -1 },
    dbg_reg_def_t { name: b"r23\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: -1 },
    dbg_reg_def_t { name: b"et\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: -1 },
    dbg_reg_def_t { name: b"bt\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: -1 },
    dbg_reg_def_t { name: b"gp\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: 128 },
    dbg_reg_def_t { name: b"sp\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: 136 },
    dbg_reg_def_t { name: b"fp\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: 144 },
    dbg_reg_def_t { name: b"ea\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: -1 },
    dbg_reg_def_t { name: b"ba\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: -1 },
    dbg_reg_def_t { name: b"ra\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: 152 },
    dbg_reg_def_t { name: b"pc\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: 160 },
    dbg_reg_def_t { name: b"status\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: -1 },
    dbg_reg_def_t { name: b"estatus\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: 168 },
    dbg_reg_def_t { name: b"bstatus\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: -1 },
    dbg_reg_def_t { name: b"ienable\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: -1 },
    dbg_reg_def_t { name: b"ipending\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: -1 },
    dbg_reg_def_t { name: b"cpuid\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: -1 },
    dbg_reg_def_t { name: b"ctl6\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: -1 },
    dbg_reg_def_t { name: b"exception\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: -1 },
    dbg_reg_def_t { name: b"pteaddr\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: -1 },
    dbg_reg_def_t { name: b"tlbacc\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: -1 },
    dbg_reg_def_t { name: b"tlbmisc\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: -1 },
    dbg_reg_def_t { name: b"eccinj\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: -1 },
    dbg_reg_def_t { name: b"badaddr\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: -1 },
    dbg_reg_def_t { name: b"config\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: -1 },
    dbg_reg_def_t { name: b"mpubase\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: -1 },
    dbg_reg_def_t { name: b"mpuacc\0".as_ptr() as *const _, size: GDB_SIZEOF_REG, offset: -1 },
];

pub unsafe fn dbg_get_reg(regno: ::core::ffi::c_int, mem: *mut u8, regs: *mut pt_regs) -> *const ::core::ffi::c_char {
    if regno >= DBG_MAX_REG_NUM as i32 || regno < 0 { return core::ptr::null(); }
    let d = &dbg_reg_def[regno as usize];
    if d.offset != -1 { core::ptr::copy_nonoverlapping((regs as *mut u8).offset(d.offset), mem, d.size as usize); }
    else { core::ptr::write_bytes(mem, 0, d.size as usize); }
    d.name
}

pub unsafe fn dbg_set_reg(regno: ::core::ffi::c_int, mem: *const u8, regs: *mut pt_regs) -> ::core::ffi::c_int {
    if regno >= DBG_MAX_REG_NUM as i32 || regno < 0 { return -EINVAL; }
    let d = &dbg_reg_def[regno as usize];
    if d.offset != -1 { core::ptr::copy_nonoverlapping(mem, (regs as *mut u8).offset(d.offset), d.size as usize); }
    0
}

pub unsafe fn sleeping_thread_to_gdb_regs(gdb_regs: *mut usize, p: *mut task_struct) {
    core::ptr::write_bytes(gdb_regs as *mut u8, 0, NUMREGBYTES);
    (*gdb_regs.add(GDB_SP)) = (*(*p).thread.kregs).sp;
    (*gdb_regs.add(GDB_PC)) = (*(*p).thread.kregs).ea;
}

pub unsafe fn kgdb_arch_set_pc(regs: *mut pt_regs, pc: usize) { (*regs).ea = pc; }

pub unsafe fn kgdb_arch_handle_exception(vector: i32, signo: i32, err_code: i32, remcom_in_buffer: *mut i8, remcom_out_buffer: *mut i8, regs: *mut pt_regs) -> i32 {
    let mut addr = 0usize;
    match *remcom_in_buffer {
        b's' | b'c' => {
            let mut ptr = remcom_in_buffer.add(1);
            if kgdb_hex2long(&mut ptr, &mut addr) { (*regs).ea = addr; }
            0
        }
        _ => -1, /* this means that we do not want to exit from the handler */
    }
}

pub unsafe extern "C" fn kgdb_breakpoint_c(regs: *mut pt_regs) {
    /* The breakpoint entry code has moved the PC on by 4 bytes, so we must move it back. */
    if wait_for_remote_debugger == 0 { (*regs).ea = (*regs).ea.wrapping_sub(4); }
    else { wait_for_remote_debugger = 0; } /* pass the first trap 30 code */
    kgdb_handle_exception(30, SIGTRAP, 0, regs);
}

pub unsafe fn kgdb_arch_init() -> i32 { wait_for_remote_debugger = 1; 0 }

pub unsafe fn kgdb_arch_exit() { /* Nothing to do */ }

#[repr(C)]
pub struct kgdb_arch {
    pub gdb_bpt_instr: [u8; 4],
}

pub static arch_kgdb_ops: kgdb_arch = kgdb_arch {
    /* Breakpoint instruction: trap 30 */
    gdb_bpt_instr: [0xba, 0x6f, 0x3b, 0x00],
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
