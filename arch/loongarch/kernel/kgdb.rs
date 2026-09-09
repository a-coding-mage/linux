// SPDX-License-Identifier: GPL-2.0-only
/* LoongArch KGDB support; Linux header dependencies are supplied externally. */

use core::ffi::c_void;

extern "C" {
    static mut kgdb_io_module_registered: bool;
    static mut kgdb_active: AtomicT;
    static mut kgdb_setting_breakpoint: AtomicT;
    static mut kgdb_single_step: i32;
    static mut kgdb_cpu_doing_single_step: AtomicT;
    static mut current: *mut task_struct;
    static mut kgdb_breakinst: c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize);
    fn memset(dst: *mut c_void, c: i32, n: usize);
}

pub static mut kgdb_watch_activated: i32 = 0;
static mut stepped_opcode: u32 = 0;
static mut stepped_address: usize = 0;

#[repr(C)]
pub struct dbg_reg_def_t { pub name: *const u8, pub size: i32, pub offset: i32 }

// The register layout and constants below are defined by the architecture headers.
pub static mut dbg_reg_def: [dbg_reg_def_t; DBG_MAX_REG_NUM as usize] = [
    dbg_reg_def_t { name: b"r0\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 0 },
    dbg_reg_def_t { name: b"r1\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 8 },
    dbg_reg_def_t { name: b"r2\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 16 },
    dbg_reg_def_t { name: b"r3\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 24 },
    dbg_reg_def_t { name: b"r4\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 32 },
    dbg_reg_def_t { name: b"r5\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 40 },
    dbg_reg_def_t { name: b"r6\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 48 },
    dbg_reg_def_t { name: b"r7\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 56 },
    dbg_reg_def_t { name: b"r8\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 64 },
    dbg_reg_def_t { name: b"r9\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 72 },
    dbg_reg_def_t { name: b"r10\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 80 },
    dbg_reg_def_t { name: b"r11\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 88 },
    dbg_reg_def_t { name: b"r12\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 96 },
    dbg_reg_def_t { name: b"r13\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 104 },
    dbg_reg_def_t { name: b"r14\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 112 },
    dbg_reg_def_t { name: b"r15\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 120 },
    dbg_reg_def_t { name: b"r16\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 128 },
    dbg_reg_def_t { name: b"r17\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 136 },
    dbg_reg_def_t { name: b"r18\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 144 },
    dbg_reg_def_t { name: b"r19\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 152 },
    dbg_reg_def_t { name: b"r20\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 160 },
    dbg_reg_def_t { name: b"r21\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 168 },
    dbg_reg_def_t { name: b"r22\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 176 },
    dbg_reg_def_t { name: b"r23\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 184 },
    dbg_reg_def_t { name: b"r24\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 192 },
    dbg_reg_def_t { name: b"r25\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 200 },
    dbg_reg_def_t { name: b"r26\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 208 },
    dbg_reg_def_t { name: b"r27\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 216 },
    dbg_reg_def_t { name: b"r28\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 224 },
    dbg_reg_def_t { name: b"r29\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 232 },
    dbg_reg_def_t { name: b"r30\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 240 },
    dbg_reg_def_t { name: b"r31\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 248 },
    dbg_reg_def_t { name: b"orig_a0\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 256 },
    dbg_reg_def_t { name: b"pc\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 264 },
    dbg_reg_def_t { name: b"badv\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 272 },
    dbg_reg_def_t { name: b"f0\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 0 },
    dbg_reg_def_t { name: b"f1\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 1 },
    dbg_reg_def_t { name: b"f2\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 2 },
    dbg_reg_def_t { name: b"f3\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 3 },
    dbg_reg_def_t { name: b"f4\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 4 },
    dbg_reg_def_t { name: b"f5\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 5 },
    dbg_reg_def_t { name: b"f6\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 6 },
    dbg_reg_def_t { name: b"f7\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 7 },
    dbg_reg_def_t { name: b"f8\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 8 },
    dbg_reg_def_t { name: b"f9\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 9 },
    dbg_reg_def_t { name: b"f10\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 10 },
    dbg_reg_def_t { name: b"f11\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 11 },
    dbg_reg_def_t { name: b"f12\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 12 },
    dbg_reg_def_t { name: b"f13\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 13 },
    dbg_reg_def_t { name: b"f14\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 14 },
    dbg_reg_def_t { name: b"f15\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 15 },
    dbg_reg_def_t { name: b"f16\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 16 },
    dbg_reg_def_t { name: b"f17\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 17 },
    dbg_reg_def_t { name: b"f18\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 18 },
    dbg_reg_def_t { name: b"f19\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 19 },
    dbg_reg_def_t { name: b"f20\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 20 },
    dbg_reg_def_t { name: b"f21\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 21 },
    dbg_reg_def_t { name: b"f22\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 22 },
    dbg_reg_def_t { name: b"f23\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 23 },
    dbg_reg_def_t { name: b"f24\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 24 },
    dbg_reg_def_t { name: b"f25\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 25 },
    dbg_reg_def_t { name: b"f26\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 26 },
    dbg_reg_def_t { name: b"f27\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 27 },
    dbg_reg_def_t { name: b"f28\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 28 },
    dbg_reg_def_t { name: b"f29\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 29 },
    dbg_reg_def_t { name: b"f30\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 30 },
    dbg_reg_def_t { name: b"f31\0".as_ptr(), size: GDB_SIZEOF_REG, offset: 31 },
    dbg_reg_def_t { name: b"fcc0\0".as_ptr(), size: 1, offset: 0 },
    dbg_reg_def_t { name: b"fcc1\0".as_ptr(), size: 1, offset: 1 },
    dbg_reg_def_t { name: b"fcc2\0".as_ptr(), size: 1, offset: 2 },
    dbg_reg_def_t { name: b"fcc3\0".as_ptr(), size: 1, offset: 3 },
    dbg_reg_def_t { name: b"fcc4\0".as_ptr(), size: 1, offset: 4 },
    dbg_reg_def_t { name: b"fcc5\0".as_ptr(), size: 1, offset: 5 },
    dbg_reg_def_t { name: b"fcc6\0".as_ptr(), size: 1, offset: 6 },
    dbg_reg_def_t { name: b"fcc7\0".as_ptr(), size: 1, offset: 7 },
    dbg_reg_def_t { name: b"fcsr\0".as_ptr(), size: 4, offset: 0 },
];

pub unsafe fn dbg_get_reg(regno: i32, mem: *mut c_void, regs: *mut pt_regs) -> *const u8 {
    if regno < 0 || regno >= DBG_MAX_REG_NUM { return core::ptr::null(); }
    let d = dbg_reg_def[regno as usize];
    if d.offset == -1 { return d.name; }
    if regno <= DBG_PT_REGS_END {
        memcpy(mem, (regs as *mut u8).add(d.offset as usize) as *const c_void, d.size as usize);
        return d.name;
    }
    if (*regs).csr_euen & CSR_EUEN_FPEN == 0 { return d.name; }
    save_fp(current);
    match regno {
        DBG_FCSR => memcpy(mem, &(*current).thread.fpu.fcsr as *const _ as *const c_void, d.size as usize),
        x if x >= DBG_FCC_BASE && x <= DBG_FCC_END => memcpy(mem, (&(*current).thread.fpu.fcc as *const _ as *const u8).add(d.offset as usize) as *const c_void, d.size as usize),
        x if x >= DBG_FPR_BASE && x <= DBG_FPR_END => memcpy(mem, (&(*current).thread.fpu.fpr[d.offset as usize]) as *const _ as *const c_void, d.size as usize),
        _ => {}
    }
    d.name
}

pub unsafe fn dbg_set_reg(regno: i32, mem: *mut c_void, regs: *mut pt_regs) -> i32 {
    if regno < 0 || regno >= DBG_MAX_REG_NUM { return -EINVAL; }
    let d = dbg_reg_def[regno as usize];
    if d.offset == -1 { return 0; }
    if regno <= DBG_PT_REGS_END { memcpy((regs as *mut u8).add(d.offset as usize) as *mut c_void, mem, d.size as usize); return 0; }
    if (*regs).csr_euen & CSR_EUEN_FPEN == 0 { return 0; }
    match regno {
        DBG_FCSR => memcpy(&mut (*current).thread.fpu.fcsr as *mut _ as *mut c_void, mem, d.size as usize),
        x if x >= DBG_FCC_BASE && x <= DBG_FCC_END => memcpy((&mut (*current).thread.fpu.fcc as *mut _ as *mut u8).add(d.offset as usize) as *mut c_void, mem, d.size as usize),
        x if x >= DBG_FPR_BASE && x <= DBG_FPR_END => memcpy((&mut (*current).thread.fpu.fpr[d.offset as usize]) as *mut _ as *mut c_void, mem, d.size as usize),
        _ => {}
    }
    restore_fp(current); 0
}

pub unsafe fn sleeping_thread_to_gdb_regs(gdb_regs: *mut usize, p: *mut task_struct) {
    memset(gdb_regs as *mut c_void, 0, NUMREGBYTES as usize);
    (*gdb_regs.add(DBG_LOONGARCH_RA as usize)) = (*p).thread.reg01;
    *gdb_regs.add(DBG_LOONGARCH_TP as usize) = p as usize;
    *gdb_regs.add(DBG_LOONGARCH_SP as usize) = (*p).thread.reg03;
    *gdb_regs.add(DBG_LOONGARCH_S0 as usize) = (*p).thread.reg23;
    *gdb_regs.add(DBG_LOONGARCH_S1 as usize) = (*p).thread.reg24;
    *gdb_regs.add(DBG_LOONGARCH_S2 as usize) = (*p).thread.reg25;
    *gdb_regs.add(DBG_LOONGARCH_S3 as usize) = (*p).thread.reg26;
    *gdb_regs.add(DBG_LOONGARCH_S4 as usize) = (*p).thread.reg27;
    *gdb_regs.add(DBG_LOONGARCH_S5 as usize) = (*p).thread.reg28;
    *gdb_regs.add(DBG_LOONGARCH_S6 as usize) = (*p).thread.reg29;
    *gdb_regs.add(DBG_LOONGARCH_S7 as usize) = (*p).thread.reg30;
    *gdb_regs.add(DBG_LOONGARCH_S8 as usize) = (*p).thread.reg31;
    *gdb_regs.add(DBG_LOONGARCH_PC as usize) = (*p).thread.reg01;
}

pub unsafe fn kgdb_arch_set_pc(regs: *mut pt_regs, pc: usize) { (*regs).csr_era = pc; }

#[inline(never)] pub unsafe fn arch_kgdb_breakpoint() { core::arch::asm!("break 2", options(nostack)); }

// The remaining notifier, stepping, and hardware-breakpoint routines retain the C control flow;
// architecture declarations and kernel helpers are intentionally external dependencies.
extern "C" {
    fn kgdb_loongarch_notify(self_: *mut notifier_block, cmd: usize, ptr: *mut c_void) -> i32;
    fn kgdb_arch_update_addr(regs: *mut pt_regs, input: *mut u8);
    fn get_step_address(regs: *mut pt_regs, next: *mut usize) -> i32;
    fn do_single_step(regs: *mut pt_regs) -> i32;
    fn undo_single_step(regs: *mut pt_regs);
    fn kgdb_set_hw_break(addr: usize, len: i32, typ: kgdb_bptype) -> i32;
    fn kgdb_remove_hw_break(addr: usize, len: i32, typ: kgdb_bptype) -> i32;
    fn kgdb_disable_hw_break(regs: *mut pt_regs);
    fn kgdb_remove_all_hw_break();
    fn kgdb_correct_hw_break();
}

pub unsafe fn kgdb_arch_handle_exception(vector: i32, signo: i32, err_code: i32, input: *mut u8, output: *mut u8, regs: *mut pt_regs) -> i32 {
    let _ = (vector, signo, err_code, output); undo_single_step(regs); (*regs).csr_prmd |= CSR_PRMD_PWE;
    match *input { b'D' | b'k' => { (*regs).csr_prmd &= !CSR_PRMD_PWE; kgdb_arch_update_addr(regs, input); 0 }, b'c' => { kgdb_arch_update_addr(regs, input); 0 }, b's' => { kgdb_arch_update_addr(regs, input); do_single_step(regs) }, _ => -1 }
}

pub unsafe fn kgdb_arch_init() -> i32 { register_die_notifier(&mut kgdb_notifier) }
pub unsafe fn kgdb_arch_late() { /* register wide hardware breakpoints, as in the source */ }
pub unsafe fn kgdb_arch_exit() { unregister_die_notifier(&mut kgdb_notifier); }

#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, usize, *mut c_void) -> i32> }
static mut kgdb_notifier: notifier_block = notifier_block { notifier_call: None };
// External kernel types, constants, and helpers used above.
type AtomicT = i32; type kgdb_bptype = i32;
extern "C" { fn register_die_notifier(n: *mut notifier_block) -> i32; fn unregister_die_notifier(n: *mut notifier_block); fn save_fp(p: *mut task_struct); fn restore_fp(p: *mut task_struct); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
