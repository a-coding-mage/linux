/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/kgdb.h. */

#[cfg(CONFIG_KGDB)]
pub struct pt_regs;
pub struct tasklet_struct;
pub struct task_struct;
pub struct uart_port;
pub struct console;

#[cfg(CONFIG_KGDB)]
extern "C" {
    pub fn kgdb_skipexception(exception: core::ffi::c_int, regs: *mut pt_regs) -> core::ffi::c_int;
    pub fn kgdb_breakpoint();

    pub static mut kgdb_connected: core::ffi::c_int;
    pub static mut kgdb_io_module_registered: core::ffi::c_int;
    pub static mut kgdb_setting_breakpoint: atomic_t;
    pub static mut kgdb_cpu_doing_single_step: atomic_t;
    pub static mut kgdb_usethread: *mut task_struct;
    pub static mut kgdb_contthread: *mut task_struct;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kgdb_bptype {
    BP_BREAKPOINT = 0,
    BP_HARDWARE_BREAKPOINT,
    BP_WRITE_WATCHPOINT,
    BP_READ_WATCHPOINT,
    BP_ACCESS_WATCHPOINT,
    BP_POKE_BREAKPOINT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kgdb_bpstate {
    BP_UNDEFINED = 0,
    BP_REMOVED,
    BP_SET,
    BP_ACTIVE,
}

#[repr(C)]
pub struct kgdb_bkpt {
    pub bpt_addr: core::ffi::c_ulong,
    pub saved_instr: [u8; BREAK_INSTR_SIZE],
    pub type_: kgdb_bptype,
    pub state: kgdb_bpstate,
}

#[repr(C)]
pub struct dbg_reg_def_t {
    pub name: *mut core::ffi::c_char,
    pub size: core::ffi::c_int,
    pub offset: core::ffi::c_int,
}

#[cfg(not(DBG_MAX_REG_NUM))]
pub const DBG_MAX_REG_NUM: core::ffi::c_int = 0;
#[cfg(DBG_MAX_REG_NUM)]
extern "C" {
    pub static mut dbg_reg_def: dbg_reg_def_t;
    pub fn dbg_get_reg(regno: core::ffi::c_int, mem: *mut core::ffi::c_void, regs: *mut pt_regs) -> *mut core::ffi::c_char;
    pub fn dbg_set_reg(regno: core::ffi::c_int, mem: *mut core::ffi::c_void, regs: *mut pt_regs) -> core::ffi::c_int;
}

#[cfg(not(KGDB_MAX_BREAKPOINTS))]
pub const KGDB_MAX_BREAKPOINTS: core::ffi::c_int = 1000;
pub const KGDB_HW_BREAKPOINT: core::ffi::c_int = 1;

#[repr(C)]
pub struct kgdb_arch {
    pub gdb_bpt_instr: [u8; BREAK_INSTR_SIZE],
    pub flags: core::ffi::c_ulong,
    pub set_breakpoint: Option<unsafe extern "C" fn(core::ffi::c_ulong, *mut core::ffi::c_char) -> core::ffi::c_int>,
    pub remove_breakpoint: Option<unsafe extern "C" fn(core::ffi::c_ulong, *mut core::ffi::c_char) -> core::ffi::c_int>,
    pub set_hw_breakpoint: Option<unsafe extern "C" fn(core::ffi::c_ulong, core::ffi::c_int, kgdb_bptype) -> core::ffi::c_int>,
    pub remove_hw_breakpoint: Option<unsafe extern "C" fn(core::ffi::c_ulong, core::ffi::c_int, kgdb_bptype) -> core::ffi::c_int>,
    pub disable_hw_break: Option<unsafe extern "C" fn(*mut pt_regs)>,
    pub remove_all_hw_break: Option<unsafe extern "C" fn()>,
    pub correct_hw_break: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct kgdb_io {
    pub name: *const core::ffi::c_char,
    pub read_char: Option<unsafe extern "C" fn() -> core::ffi::c_int>,
    pub write_char: Option<unsafe extern "C" fn(u8)>,
    pub flush: Option<unsafe extern "C" fn()>,
    pub init: Option<unsafe extern "C" fn() -> core::ffi::c_int>,
    pub deinit: Option<unsafe extern "C" fn()>,
    pub pre_exception: Option<unsafe extern "C" fn()>,
    pub post_exception: Option<unsafe extern "C" fn()>,
    pub cons: *mut console,
}

#[cfg(CONFIG_KGDB)]
extern "C" {
    pub fn kgdb_arch_init() -> core::ffi::c_int;
    pub fn kgdb_arch_exit();
    pub fn pt_regs_to_gdb_regs(gdb_regs: *mut core::ffi::c_ulong, regs: *mut pt_regs);
    pub fn sleeping_thread_to_gdb_regs(gdb_regs: *mut core::ffi::c_ulong, p: *mut task_struct);
    pub fn gdb_regs_to_pt_regs(gdb_regs: *mut core::ffi::c_ulong, regs: *mut pt_regs);
    pub fn kgdb_arch_handle_exception(vector: core::ffi::c_int, signo: core::ffi::c_int, err_code: core::ffi::c_int, remcom_in_buffer: *mut core::ffi::c_char, remcom_out_buffer: *mut core::ffi::c_char, regs: *mut pt_regs) -> core::ffi::c_int;
    pub fn kgdb_arch_handle_qxfer_pkt(remcom_in_buffer: *mut core::ffi::c_char, remcom_out_buffer: *mut core::ffi::c_char);
    pub fn kgdb_call_nmi_hook(ignored: *mut core::ffi::c_void);
    pub fn kgdb_roundup_cpus();
    pub fn kgdb_arch_set_pc(regs: *mut pt_regs, pc: core::ffi::c_ulong);
    pub fn kgdb_validate_break_address(addr: core::ffi::c_ulong) -> core::ffi::c_int;
    pub fn kgdb_arch_set_breakpoint(bpt: *mut kgdb_bkpt) -> core::ffi::c_int;
    pub fn kgdb_arch_remove_breakpoint(bpt: *mut kgdb_bkpt) -> core::ffi::c_int;
    pub fn kgdb_arch_late();
    pub static arch_kgdb_ops: kgdb_arch;
    pub fn kgdb_arch_pc(exception: core::ffi::c_int, regs: *mut pt_regs) -> core::ffi::c_ulong;
    pub fn kgdb_register_io_module(local_kgdb_io_ops: *mut kgdb_io) -> core::ffi::c_int;
    pub fn kgdb_unregister_io_module(local_kgdb_io_ops: *mut kgdb_io);
    pub static mut dbg_io_ops: *mut kgdb_io;
    pub fn kgdb_hex2long(ptr: *mut *mut core::ffi::c_char, long_val: *mut core::ffi::c_ulong) -> core::ffi::c_int;
    pub fn kgdb_mem2hex(mem: *mut core::ffi::c_char, buf: *mut core::ffi::c_char, count: core::ffi::c_int) -> *mut core::ffi::c_char;
    pub fn kgdb_hex2mem(buf: *mut core::ffi::c_char, mem: *mut core::ffi::c_char, count: core::ffi::c_int) -> core::ffi::c_int;
    pub fn kgdb_isremovedbreak(addr: core::ffi::c_ulong) -> core::ffi::c_int;
    pub fn kgdb_has_hit_break(addr: core::ffi::c_ulong) -> core::ffi::c_int;
    pub fn kgdb_handle_exception(ex_vector: core::ffi::c_int, signo: core::ffi::c_int, err_code: core::ffi::c_int, regs: *mut pt_regs) -> core::ffi::c_int;
    pub fn kgdb_nmicallback(cpu: core::ffi::c_int, regs: *mut core::ffi::c_void) -> core::ffi::c_int;
    pub fn kgdb_nmicallin(cpu: core::ffi::c_int, trapnr: core::ffi::c_int, regs: *mut core::ffi::c_void, err_code: core::ffi::c_int, snd_rdy: *mut atomic_t) -> core::ffi::c_int;
    pub fn gdbstub_exit(status: core::ffi::c_int);
    pub static mut kgdb_single_step: core::ffi::c_int;
    pub static mut kgdb_active: atomic_t;
    pub static mut dbg_is_early: bool;
    pub fn dbg_late_init();
    pub fn kgdb_panic(msg: *const core::ffi::c_char);
    pub fn kgdb_free_init_mem();
}

#[cfg(CONFIG_KGDB_HONOUR_BLOCKLIST)]
pub unsafe fn kgdb_within_blocklist(addr: core::ffi::c_ulong) -> bool { within_kprobe_blacklist(addr) }
#[cfg(not(CONFIG_KGDB_HONOUR_BLOCKLIST))]
pub unsafe fn kgdb_within_blocklist(_addr: core::ffi::c_ulong) -> bool { false }

#[cfg(CONFIG_KGDB)]
pub unsafe fn in_dbg_master() -> bool {
    irqs_disabled() && (smp_processor_id() == atomic_read(&kgdb_active))
}

#[cfg(not(CONFIG_KGDB))]
pub const fn in_dbg_master() -> bool { false }

#[cfg(not(CONFIG_KGDB))]
pub unsafe fn dbg_late_init() {}

#[cfg(not(CONFIG_KGDB))]
pub unsafe fn kgdb_panic(_msg: *const core::ffi::c_char) {}

#[cfg(not(CONFIG_KGDB))]
pub unsafe fn kgdb_free_init_mem() {}

#[cfg(not(CONFIG_KGDB))]
pub unsafe fn kgdb_nmicallback(_cpu: core::ffi::c_int, _regs: *mut core::ffi::c_void) -> core::ffi::c_int { 1 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
