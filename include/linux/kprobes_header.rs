/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of linux/kprobes.h. Included C dependencies are external. */

use core::ffi::{c_char, c_int, c_void};

#[cfg(CONFIG_KPROBES)]
pub const KPROBE_HIT_ACTIVE: u32 = 0x00000001;
#[cfg(CONFIG_KPROBES)]
pub const KPROBE_HIT_SS: u32 = 0x00000002;
#[cfg(CONFIG_KPROBES)]
pub const KPROBE_REENTER: u32 = 0x00000004;
#[cfg(CONFIG_KPROBES)]
pub const KPROBE_HIT_SSDONE: u32 = 0x00000008;

/* External kernel types supplied by the included headers. */
pub type kprobe_opcode_t = c_int;
#[repr(C)] pub struct hlist_node { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct pt_regs { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct objpool_head { _private: [u8; 0] }
#[repr(C)] pub struct rcu_head { _private: [u8; 0] }
#[repr(C)] pub struct llist_node { _private: [u8; 0] }
#[repr(C)] pub struct notifier_block { _private: [u8; 0] }
#[repr(C)] pub struct ftrace_ops { _private: [u8; 0] }
#[repr(C)] pub struct ftrace_regs { _private: [u8; 0] }
#[repr(C)] pub struct rethook { pub data: *mut c_void }
#[repr(C)] pub struct rethook_node { pub rethook: *mut rethook, pub ret_addr: usize }
#[repr(C)] pub struct arch_specific_insn { pub dummy: c_int }
#[repr(C)] pub struct arch_optimized_insn { _private: [u8; 0] }
#[repr(C)] pub struct kprobe_ctlblk { _private: [u8; 0] }

pub type kprobe_pre_handler_t = Option<unsafe extern "C" fn(*mut kprobe, *mut pt_regs) -> c_int>;
pub type kprobe_post_handler_t = Option<unsafe extern "C" fn(*mut kprobe, *mut pt_regs, usize)>;
pub type kretprobe_handler_t = Option<unsafe extern "C" fn(*mut kretprobe_instance, *mut pt_regs) -> c_int>;

#[repr(C)]
pub struct kprobe {
    pub hlist: hlist_node,
    pub list: list_head,
    pub nmissed: usize,
    pub addr: *mut kprobe_opcode_t,
    pub symbol_name: *const c_char,
    pub offset: u32,
    pub pre_handler: kprobe_pre_handler_t,
    pub post_handler: kprobe_post_handler_t,
    pub opcode: kprobe_opcode_t,
    pub ainsn: arch_specific_insn,
    pub flags: u32,
}

pub const KPROBE_FLAG_GONE: u32 = 1;
pub const KPROBE_FLAG_DISABLED: u32 = 2;
pub const KPROBE_FLAG_OPTIMIZED: u32 = 4;
pub const KPROBE_FLAG_FTRACE: u32 = 8;
pub const KPROBE_FLAG_ON_FUNC_ENTRY: u32 = 16;

#[inline] pub unsafe fn kprobe_gone(p: *const kprobe) -> bool { (*p).flags & KPROBE_FLAG_GONE != 0 }
#[inline] pub unsafe fn kprobe_disabled(p: *const kprobe) -> bool { (*p).flags & (KPROBE_FLAG_DISABLED | KPROBE_FLAG_GONE) != 0 }
#[inline] pub unsafe fn kprobe_optimized(p: *const kprobe) -> bool { (*p).flags & KPROBE_FLAG_OPTIMIZED != 0 }
#[inline] pub unsafe fn kprobe_ftrace(p: *const kprobe) -> bool { (*p).flags & KPROBE_FLAG_FTRACE != 0 }

#[repr(C)] pub struct kretprobe_holder { pub rp: *mut kretprobe, pub pool: objpool_head }
#[repr(C)] pub struct kretprobe {
    pub kp: kprobe, pub handler: kretprobe_handler_t, pub entry_handler: kretprobe_handler_t,
    pub maxactive: c_int, pub nmissed: c_int, pub data_size: usize,
    #[cfg(CONFIG_KRETPROBE_ON_RETHOOK)] pub rh: *mut rethook,
    #[cfg(not(CONFIG_KRETPROBE_ON_RETHOOK))] pub rph: *mut kretprobe_holder,
}
pub const KRETPROBE_MAX_DATA_SIZE: usize = 4096;
#[repr(C)] pub struct kretprobe_instance {
    #[cfg(CONFIG_KRETPROBE_ON_RETHOOK)] pub node: rethook_node,
    #[cfg(not(CONFIG_KRETPROBE_ON_RETHOOK))] pub rcu: rcu_head,
    #[cfg(not(CONFIG_KRETPROBE_ON_RETHOOK))] pub llist: llist_node,
    #[cfg(not(CONFIG_KRETPROBE_ON_RETHOOK))] pub rph: *mut kretprobe_holder,
    #[cfg(not(CONFIG_KRETPROBE_ON_RETHOOK))] pub ret_addr: *mut kprobe_opcode_t,
    #[cfg(not(CONFIG_KRETPROBE_ON_RETHOOK))] pub fp: *mut c_void,
    pub data: [u8; 0],
}
#[repr(C)] pub struct kretprobe_blackpoint { pub name: *const c_char, pub addr: *mut c_void }
#[repr(C)] pub struct kprobe_blacklist_entry { pub list: list_head, pub start_addr: usize, pub end_addr: usize }

extern "C" {
    pub fn kprobe_busy_begin(); pub fn kprobe_busy_end();
    pub fn arch_prepare_kprobe(p: *mut kprobe) -> c_int;
    pub fn arch_arm_kprobe(p: *mut kprobe); pub fn arch_disarm_kprobe(p: *mut kprobe);
    pub fn arch_init_kprobes() -> c_int; pub fn kprobes_inc_nmissed_count(p: *mut kprobe);
    pub fn arch_within_kprobe_blacklist(addr: usize) -> bool; pub fn arch_populate_kprobe_blacklist() -> c_int;
    pub fn kprobe_on_func_entry(addr: *mut kprobe_opcode_t, sym: *const c_char, offset: usize) -> c_int;
    pub fn within_kprobe_blacklist(addr: usize) -> bool;
    pub fn kprobe_add_ksym_blacklist(entry: usize) -> c_int; pub fn kprobe_add_area_blacklist(start: usize, end: usize) -> c_int;
    pub fn get_kprobe(addr: *mut c_void) -> *mut kprobe;
    pub fn kprobe_lookup_name(name: *const c_char, offset: u32) -> *mut kprobe_opcode_t;
    pub fn arch_adjust_kprobe_addr(addr: usize, offset: usize, on_func_entry: *mut bool) -> *mut kprobe_opcode_t;
    pub fn register_kprobe(p: *mut kprobe) -> c_int; pub fn unregister_kprobe(p: *mut kprobe);
    pub fn register_kprobes(kps: *mut *mut kprobe, num: c_int) -> c_int; pub fn unregister_kprobes(kps: *mut *mut kprobe, num: c_int);
    pub fn register_kretprobe(rp: *mut kretprobe) -> c_int; pub fn unregister_kretprobe(rp: *mut kretprobe);
    pub fn register_kretprobes(rps: *mut *mut kretprobe, num: c_int) -> c_int; pub fn unregister_kretprobes(rps: *mut *mut kretprobe, num: c_int);
    pub fn kprobe_free_init_mem(); pub fn disable_kprobe(kp: *mut kprobe) -> c_int; pub fn enable_kprobe(kp: *mut kprobe) -> c_int;
    pub fn dump_kprobe(kp: *mut kprobe); pub fn alloc_insn_page() -> *mut c_void;
    pub fn alloc_optinsn_page() -> *mut c_void; pub fn free_optinsn_page(page: *mut c_void);
    pub fn kprobe_get_kallsym(symnum: u32, value: *mut usize, ty: *mut c_char, sym: *mut c_char) -> c_int;
    pub fn arch_kprobe_get_kallsym(symnum: *mut u32, value: *mut usize, ty: *mut c_char, sym: *mut c_char) -> c_int;
    pub fn kprobe_exceptions_notify(self_: *mut notifier_block, val: usize, data: *mut c_void) -> c_int;
}

#[inline] pub unsafe fn disable_kretprobe(rp: *mut kretprobe) -> c_int { disable_kprobe(&mut (*rp).kp) }
#[inline] pub unsafe fn enable_kretprobe(rp: *mut kretprobe) -> c_int { enable_kprobe(&mut (*rp).kp) }
#[inline] pub unsafe fn is_kprobe_insn_slot(_addr: usize) -> bool { false }
#[inline] pub unsafe fn is_kprobe_optinsn_slot(_addr: usize) -> bool { false }
#[inline] pub unsafe fn is_kretprobe_trampoline(_addr: usize) -> bool { false }
#[inline] pub unsafe fn kretprobe_find_ret_addr(_tsk: *mut task_struct, _fp: *mut c_void, _cur: *mut *mut llist_node) -> usize { 0 }
#[inline] pub unsafe fn kprobe_page_fault(_regs: *mut pt_regs, _trap: u32) -> bool { false }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
