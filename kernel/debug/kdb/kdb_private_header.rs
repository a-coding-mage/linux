/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Kernel Debugger Architecture Independent Private Headers
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (c) 2000-2004 Silicon Graphics, Inc.  All Rights Reserved.
 * Copyright (c) 2009 Wind River Systems, Inc.  All Rights Reserved.
 */

/* Dependencies supplied by the kernel debugger implementation. */

/* Kernel Debugger Command codes. Must not overlap with error codes. */
pub const KDB_CMD_GO: i32 = -1001;
pub const KDB_CMD_CPU: i32 = -1002;
pub const KDB_CMD_SS: i32 = -1003;
pub const KDB_CMD_KGDB: i32 = -1005;

/* Internal debug flags */
pub const KDB_DEBUG_FLAG_BP: usize = 0x0002;
pub const KDB_DEBUG_FLAG_BB_SUMM: usize = 0x0004;
pub const KDB_DEBUG_FLAG_AR: usize = 0x0008;
pub const KDB_DEBUG_FLAG_ARA: usize = 0x0010;
pub const KDB_DEBUG_FLAG_BB: usize = 0x0020;
pub const KDB_DEBUG_FLAG_STATE: usize = 0x0040;
pub const KDB_DEBUG_FLAG_MASK: usize = 0xffff;
pub const KDB_DEBUG_FLAG_SHIFT: usize = 16;
unsafe extern "C" { pub static mut kdb_flags: usize; }

#[macro_export]
macro_rules! KDB_DEBUG {
    (BP) => { unsafe { kdb_flags & (KDB_DEBUG_FLAG_BP << KDB_DEBUG_FLAG_SHIFT) } };
    (BB_SUMM) => { unsafe { kdb_flags & (KDB_DEBUG_FLAG_BB_SUMM << KDB_DEBUG_FLAG_SHIFT) } };
    (AR) => { unsafe { kdb_flags & (KDB_DEBUG_FLAG_AR << KDB_DEBUG_FLAG_SHIFT) } };
    (ARA) => { unsafe { kdb_flags & (KDB_DEBUG_FLAG_ARA << KDB_DEBUG_FLAG_SHIFT) } };
    (BB) => { unsafe { kdb_flags & (KDB_DEBUG_FLAG_BB << KDB_DEBUG_FLAG_SHIFT) } };
    (STATE) => { unsafe { kdb_flags & (KDB_DEBUG_FLAG_STATE << KDB_DEBUG_FLAG_SHIFT) } };
}

#[macro_export]
macro_rules! KDB_STATE {
    (KDB) => { unsafe { kdb_state & KDB_STATE_KDB } };
    ($flag:ident) => { unsafe { kdb_state & concat_idents!(KDB_STATE_, $flag) } };
}
#[macro_export]
macro_rules! KDB_STATE_SET { ($flag:ident) => { unsafe { kdb_state |= KDB_STATE_ $flag; } }; }
#[macro_export]
macro_rules! KDB_STATE_CLEAR { ($flag:ident) => { unsafe { kdb_state &= !KDB_STATE_ $flag; } }; }

/* BITS_PER_LONG selects the platform-specific format constants. */
#[cfg(target_pointer_width = "32")]
pub const KDB_PLATFORM_ENV: &str = "BYTESPERWORD=4";
#[cfg(target_pointer_width = "32")]
pub const kdb_machreg_fmt: &str = "0x%lx";
#[cfg(target_pointer_width = "32")]
pub const kdb_machreg_fmt0: &str = "0x%08lx";
#[cfg(target_pointer_width = "32")]
pub const kdb_bfd_vma_fmt: &str = "0x%lx";
#[cfg(target_pointer_width = "32")]
pub const kdb_bfd_vma_fmt0: &str = "0x%08lx";
#[cfg(target_pointer_width = "32")]
pub const kdb_elfw_addr_fmt: &str = "0x%x";
#[cfg(target_pointer_width = "32")]
pub const kdb_elfw_addr_fmt0: &str = "0x%08x";
#[cfg(target_pointer_width = "32")]
pub const kdb_f_count_fmt: &str = "%d";

#[cfg(target_pointer_width = "64")]
pub const KDB_PLATFORM_ENV: &str = "BYTESPERWORD=8";
#[cfg(target_pointer_width = "64")]
pub const kdb_machreg_fmt: &str = "0x%lx";
#[cfg(target_pointer_width = "64")]
pub const kdb_machreg_fmt0: &str = "0x%016lx";
#[cfg(target_pointer_width = "64")]
pub const kdb_bfd_vma_fmt: &str = "0x%lx";
#[cfg(target_pointer_width = "64")]
pub const kdb_bfd_vma_fmt0: &str = "0x%016lx";
#[cfg(target_pointer_width = "64")]
pub const kdb_elfw_addr_fmt: &str = "0x%x";
#[cfg(target_pointer_width = "64")]
pub const kdb_elfw_addr_fmt0: &str = "0x%016x";
#[cfg(target_pointer_width = "64")]
pub const kdb_f_count_fmt: &str = "%ld";

pub const KDB_MAXBPT: usize = 16;

#[repr(C)]
pub struct kdb_symtab_t {
    pub value: usize,
    pub mod_name: *const core::ffi::c_char,
    pub mod_start: usize,
    pub mod_end: usize,
    pub sec_name: *const core::ffi::c_char,
    pub sec_start: usize,
    pub sec_end: usize,
    pub sym_name: *const core::ffi::c_char,
    pub sym_start: usize,
    pub sym_end: usize,
}

unsafe extern "C" {
    pub fn kallsyms_symbol_next(prefix_name: *mut core::ffi::c_char, flag: i32, buf_size: i32) -> i32;
    pub fn kallsyms_symbol_complete(prefix_name: *mut core::ffi::c_char, max_len: i32) -> i32;
    pub fn kdb_getarea_size(dst: *mut core::ffi::c_void, addr: usize, size: usize) -> i32;
    pub fn kdb_putarea_size(addr: usize, src: *mut core::ffi::c_void, size: usize) -> i32;
    pub fn kdb_getphysword(word: *mut usize, addr: usize, size: usize) -> i32;
    pub fn kdb_getword(word: *mut usize, addr: usize, size: usize) -> i32;
    pub fn kdb_putword(addr: usize, word: usize, size: usize) -> i32;
    pub fn kdbgetularg(arg: *const core::ffi::c_char, value: *mut usize) -> i32;
    pub fn kdbgetu64arg(arg: *const core::ffi::c_char, value: *mut u64) -> i32;
    pub fn kdbgetenv(name: *const core::ffi::c_char) -> *mut core::ffi::c_char;
    pub fn kdbgetsymval(symname: *const core::ffi::c_char, value: *mut kdb_symtab_t) -> i32;
    pub fn kdbnearsym(addr: usize, value: *mut kdb_symtab_t) -> i32;
    pub fn kdb_strdup(str_: *const core::ffi::c_char, type_: usize) -> *mut core::ffi::c_char;
    pub fn kdb_strdup_dequote(str_: *const core::ffi::c_char, type_: usize) -> *mut core::ffi::c_char;
    pub fn kdb_symbol_print(addr: usize, symtab: *const kdb_symtab_t, ssize: u32);
    pub fn kdb_print_state(text: *const core::ffi::c_char, value: i32);
    pub fn kdb_register_table(kp: *mut core::ffi::c_void, len: usize);
    pub fn kdb_bt(argc: i32, argv: *const *const core::ffi::c_char) -> i32;
    pub fn kdb_initbptab();
    pub fn kdb_bp_install(regs: *mut core::ffi::c_void);
    pub fn kdb_bp_remove();
    pub fn kdb_main_loop(reason: i32, reason2: i32, error: i32, db: i32, regs: *mut core::ffi::c_void) -> i32;
    pub fn kdb_task_state_char(task: *const core::ffi::c_void) -> core::ffi::c_char;
    pub fn kdb_task_state(task: *const core::ffi::c_void, mask: *const core::ffi::c_char) -> bool;
    pub fn kdb_ps_suppressed();
    pub fn kdb_ps1(task: *const core::ffi::c_void);
    pub fn kdb_getchar() -> core::ffi::c_char;
    pub fn kdb_getstr(buf: *mut core::ffi::c_char, size: usize, prompt: *const core::ffi::c_char) -> *mut core::ffi::c_char;
    pub fn kdb_gdb_state_pass(buf: *mut core::ffi::c_char);
    #[cfg(feature = "CONFIG_KDB_KEYBOARD")]
    pub fn kdb_kbd_cleanup_state();
}

pub static mut kdb_state: i32 = 0;
pub static mut kdb_nextline: i32 = 0;
pub const KDB_STATE_KDB: i32 = 0x00000001;
pub const KDB_STATE_LEAVING: i32 = 0x00000002;
pub const KDB_STATE_CMD: i32 = 0x00000004;
pub const KDB_STATE_KDB_CONTROL: i32 = 0x00000008;
pub const KDB_STATE_HOLD_CPU: i32 = 0x00000010;
pub const KDB_STATE_DOING_SS: i32 = 0x00000020;
pub const KDB_STATE_SSBPT: i32 = 0x00000080;
pub const KDB_STATE_REENTRY: i32 = 0x00000100;
pub const KDB_STATE_SUPPRESS: i32 = 0x00000200;
pub const KDB_STATE_PAGER: i32 = 0x00000400;
pub const KDB_STATE_GO_SWITCH: i32 = 0x00000800;
pub const KDB_STATE_WAIT_IPI: i32 = 0x00002000;
pub const KDB_STATE_RECURSE: i32 = 0x00004000;
pub const KDB_STATE_IP_ADJUSTED: i32 = 0x00008000;
pub const KDB_STATE_GO1: i32 = 0x00010000;
pub const KDB_STATE_KEYBOARD: i32 = 0x00020000;
pub const KDB_STATE_KEXEC: i32 = 0x00040000;
pub const KDB_STATE_DOING_KGDB: i32 = 0x00080000;
pub const KDB_STATE_KGDB_TRANS: i32 = 0x00200000;
pub const KDB_STATE_ARCH: i32 = 0xff000000u32 as i32;

#[repr(C)]
pub struct kdb_bp_t {
    pub bp_addr: usize,
    /* C bitfields are represented by their containing unsigned-int storage. */
    pub bp_flags: u32,
    pub bph_length: u32,
}

pub const KDB_GREPPING_FLAG_SEARCH: i32 = 0x8000;
pub const KDB_GREP_STRLEN: usize = 256;
pub const KDB_SP_SPACEB: u32 = 0x0001;
pub const KDB_SP_SPACEA: u32 = 0x0002;
pub const KDB_SP_PAREN: u32 = 0x0004;
pub const KDB_SP_VALUE: u32 = 0x0008;
pub const KDB_SP_SYMSIZE: u32 = 0x0010;
pub const KDB_SP_NEWLINE: u32 = 0x0020;
pub const KDB_SP_DEFAULT: u32 = KDB_SP_VALUE | KDB_SP_PAREN;

/* The remaining declarations are enabled by CONFIG_KGDB_KDB in the C header. */
#[cfg(feature = "CONFIG_KGDB_KDB")]
unsafe extern "C" {
    pub static mut kdb_breakpoints: [kdb_bp_t; KDB_MAXBPT];
    pub static mut kdb_grepping_flag: i32;
    pub static mut kdb_grep_string: [core::ffi::c_char; KDB_GREP_STRLEN];
    pub static mut kdb_grep_leading: i32;
    pub static mut kdb_grep_trailing: i32;
    pub static mut kdb_cmds: *mut *mut core::ffi::c_char;
    pub static mut kdb_current_task: *mut core::ffi::c_void;
    pub static mut kdb_current_regs: *mut core::ffi::c_void;
    pub static mut kdb_prompt_str: [core::ffi::c_char; 0];
}

pub const KDB_WORD_SIZE: usize = core::mem::size_of::<usize>();

#[repr(i32)]
pub enum kdb_dbtrap_t { KDB_DB_BPT, KDB_DB_SS, KDB_DB_SSBPT, KDB_DB_NOBPT }

#[macro_export]
macro_rules! kdb_getarea { ($x:expr, $addr:expr) => { unsafe { kdb_getarea_size((&mut $x as *mut _).cast(), $addr, core::mem::size_of_val(&$x)) } }; }
#[macro_export]
macro_rules! kdb_putarea { ($addr:expr, $x:expr) => { unsafe { kdb_putarea_size($addr, (&mut $x as *mut _).cast(), core::mem::size_of_val(&$x)) } }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
