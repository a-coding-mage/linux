/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from the Linux KDB architecture-independent global header. */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than implemented in this header.

pub const KDB_ENABLE_NO_ARGS_SHIFT: u32 = 10;

#[repr(u32)]
pub enum kdb_cmdflags_t {
    KDB_ENABLE_ALL = 1 << 0,
    KDB_ENABLE_MEM_READ = 1 << 1,
    KDB_ENABLE_MEM_WRITE = 1 << 2,
    KDB_ENABLE_REG_READ = 1 << 3,
    KDB_ENABLE_REG_WRITE = 1 << 4,
    KDB_ENABLE_INSPECT = 1 << 5,
    KDB_ENABLE_FLOW_CTRL = 1 << 6,
    KDB_ENABLE_SIGNAL = 1 << 7,
    KDB_ENABLE_REBOOT = 1 << 8,
    KDB_ENABLE_ALWAYS_SAFE = 1 << 9,
    KDB_ENABLE_MASK = (1 << KDB_ENABLE_NO_ARGS_SHIFT) - 1,
    KDB_ENABLE_ALL_NO_ARGS = 1 << (0 + KDB_ENABLE_NO_ARGS_SHIFT),
    KDB_ENABLE_MEM_READ_NO_ARGS = 1 << (1 + KDB_ENABLE_NO_ARGS_SHIFT),
    KDB_ENABLE_MEM_WRITE_NO_ARGS = 1 << (2 + KDB_ENABLE_NO_ARGS_SHIFT),
    KDB_ENABLE_REG_READ_NO_ARGS = 1 << (3 + KDB_ENABLE_NO_ARGS_SHIFT),
    KDB_ENABLE_REG_WRITE_NO_ARGS = 1 << (4 + KDB_ENABLE_NO_ARGS_SHIFT),
    KDB_ENABLE_INSPECT_NO_ARGS = 1 << (5 + KDB_ENABLE_NO_ARGS_SHIFT),
    KDB_ENABLE_FLOW_CTRL_NO_ARGS = 1 << (6 + KDB_ENABLE_NO_ARGS_SHIFT),
    KDB_ENABLE_SIGNAL_NO_ARGS = 1 << (7 + KDB_ENABLE_NO_ARGS_SHIFT),
    KDB_ENABLE_REBOOT_NO_ARGS = 1 << (8 + KDB_ENABLE_NO_ARGS_SHIFT),
    KDB_ENABLE_ALWAYS_SAFE_NO_ARGS = 1 << (9 + KDB_ENABLE_NO_ARGS_SHIFT),
    KDB_ENABLE_MASK_NO_ARGS = ((1 << KDB_ENABLE_NO_ARGS_SHIFT) - 1) << KDB_ENABLE_NO_ARGS_SHIFT,
    KDB_REPEAT_NO_ARGS = 0x40000000,
    KDB_REPEAT_WITH_ARGS = 0x80000000,
}

pub type kdb_func_t = unsafe extern "C" fn(i32, *const *const i8) -> i32;

#[repr(C)]
pub struct kdbtab_t {
    pub name: *mut i8,
    pub func: kdb_func_t,
    pub usage: *mut i8,
    pub help: *mut i8,
    pub minlen: i16,
    pub flags: kdb_cmdflags_t,
    pub list_node: list_head,
}

pub const KDB_POLL_FUNC_MAX: usize = 5;
pub static mut kdb_poll_idx: i32 = 0;
pub static mut kdb_initial_cpu: i32 = -1;
pub const KDB_MAXARGS: usize = 16;
pub const KDB_NOTFOUND: i32 = -1;
pub const KDB_ARGCOUNT: i32 = -2;
pub const KDB_BADWIDTH: i32 = -3;
pub const KDB_BADRADIX: i32 = -4;
pub const KDB_NOTENV: i32 = -5;
pub const KDB_NOENVVALUE: i32 = -6;
pub const KDB_NOTIMP: i32 = -7;
pub const KDB_ENVFULL: i32 = -8;
pub const KDB_KMALLOCFAILED: i32 = -9;
pub const KDB_TOOMANYBPT: i32 = -10;
pub const KDB_TOOMANYDBREGS: i32 = -11;
pub const KDB_DUPBPT: i32 = -12;
pub const KDB_BPTNOTFOUND: i32 = -13;
pub const KDB_BADMODE: i32 = -14;
pub const KDB_BADINT: i32 = -15;
pub const KDB_INVADDRFMT: i32 = -16;
pub const KDB_BADREG: i32 = -17;
pub const KDB_BADCPUNUM: i32 = -18;
pub const KDB_BADLENGTH: i32 = -19;
pub const KDB_NOBP: i32 = -20;
pub const KDB_BADADDR: i32 = -21;
pub const KDB_NOPERM: i32 = -22;

pub static mut kdb_diemsg: *const i8 = core::ptr::null();
pub const KDB_FLAG_EARLYKDB: u32 = 1 << 0;
pub const KDB_FLAG_CATASTROPHIC: u32 = 1 << 1;
pub const KDB_FLAG_CMD_INTERRUPT: u32 = 1 << 2;
pub const KDB_FLAG_NOIPI: u32 = 1 << 3;
pub const KDB_FLAG_NO_CONSOLE: u32 = 1 << 5;
pub const KDB_FLAG_NO_VT_CONSOLE: u32 = 1 << 6;
pub const KDB_FLAG_NO_I8042: u32 = 1 << 7;
pub static mut kdb_flags: u32 = 0;

#[inline]
pub unsafe fn KDB_FLAG(flag: u32) -> u32 { kdb_flags & flag }
#[inline]
pub unsafe fn KDB_FLAG_SET(flag: u32) { kdb_flags |= flag; }
#[inline]
pub unsafe fn KDB_FLAG_CLEAR(flag: u32) { kdb_flags &= !flag; }

#[repr(u32)]
pub enum kdb_reason_t {
    KDB_REASON_ENTER = 1, KDB_REASON_ENTER_SLAVE, KDB_REASON_BREAK,
    KDB_REASON_DEBUG, KDB_REASON_OOPS, KDB_REASON_SWITCH, KDB_REASON_KEYBOARD,
    KDB_REASON_NMI, KDB_REASON_RECURSE, KDB_REASON_SSTEP, KDB_REASON_SYSTEM_NMI,
}
#[repr(C)]
pub enum kdb_msgsrc { KDB_MSGSRC_INTERNAL, KDB_MSGSRC_PRINTK }

pub static mut kdb_trap_printk: i32 = 0;
pub static mut kdb_printf_cpu: i32 = 0;
pub unsafe extern "C" fn vkdb_printf(_src: kdb_msgsrc, _fmt: *const i8, _args: va_list) -> i32;
pub unsafe extern "C" fn kdb_printf(_fmt: *const i8, ...) -> i32;
pub type kdb_printf_t = unsafe extern "C" fn(*const i8, ...) -> i32;
pub unsafe extern "C" fn kdb_init(level: i32);
pub type get_char_func = unsafe extern "C" fn() -> i32;
pub static mut kdb_poll_funcs: *mut get_char_func = core::ptr::null_mut();
pub unsafe extern "C" fn kdb_get_kbd_char() -> i32;
pub unsafe extern "C" fn kdb_send_sig(p: *mut task_struct, sig: i32);
// CONFIG_KALLSYMS selects the external implementation; otherwise this inline
// function returns NULL.
pub unsafe extern "C" fn kdb_walk_kallsyms(pos: *mut loff_t) -> *const i8 {
    let _ = pos;
    core::ptr::null()
}
pub unsafe extern "C" fn kdb_register(cmd: *mut kdbtab_t) -> i32;
pub unsafe extern "C" fn kdb_unregister(cmd: *mut kdbtab_t);
pub unsafe extern "C" fn kdb_printf_on_this_cpu() -> bool {
    // raw_smp_processor_id() and READ_ONCE preserve the kernel operation.
    READ_ONCE(kdb_printf_cpu) == raw_smp_processor_id()
}

// When CONFIG_KGDB_KDB is disabled, these inline stubs are used.
#[inline]
pub unsafe extern "C" fn kdb_printf_disabled(_fmt: *const i8, ...) -> i32 { 0 }
#[inline]
pub unsafe extern "C" fn kdb_init_disabled(_level: i32) {}
#[inline]
pub unsafe extern "C" fn kdb_register_disabled(_cmd: *mut kdbtab_t) -> i32 { 0 }
#[inline]
pub unsafe extern "C" fn kdb_unregister_disabled(_cmd: *mut kdbtab_t) {}
#[inline]
pub unsafe extern "C" fn kdb_printf_on_this_cpu_disabled() -> bool { false }

pub const KDB_NOT_INITIALIZED: i32 = 0;
pub const KDB_INIT_EARLY: i32 = 1;
pub const KDB_INIT_FULL: i32 = 2;
pub unsafe extern "C" fn kdbgetintenv(name: *const i8, value: *mut i32) -> i32;
pub unsafe extern "C" fn kdb_set(argc: i32, argv: *const *const i8) -> i32;
pub unsafe extern "C" fn kdb_lsmod(argc: i32, argv: *const *const i8) -> i32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
