/* SPDX-License-Identifier: GPL-2.0-or-later */
/* User-space Probes (UProbes) -- source-level Rust translation. */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced but not implemented here.

pub const UPROBE_HANDLER_REMOVE: i32 = 1;
pub const UPROBE_HANDLER_IGNORE: i32 = 2;
pub const MAX_URETPROBE_DEPTH: usize = 64;
pub const UPROBE_NO_TRAMPOLINE_VADDR: usize = !0usize;

#[repr(C)]
pub struct uprobe_consumer {
    pub handler: Option<unsafe extern "C" fn(*mut uprobe_consumer, *mut pt_regs, *mut u64) -> i32>,
    pub ret_handler: Option<unsafe extern "C" fn(*mut uprobe_consumer, c_ulong, *mut pt_regs, *mut u64) -> i32>,
    pub filter: Option<unsafe extern "C" fn(*mut uprobe_consumer, *mut mm_struct) -> bool>,
    pub cons_node: list_head,
    pub id: u64,
}

#[cfg(CONFIG_UPROBES)]
#[repr(C)]
pub enum uprobe_task_state { UTASK_RUNNING, UTASK_SSTEP, UTASK_SSTEP_ACK, UTASK_SSTEP_TRAPPED }

#[cfg(CONFIG_UPROBES)]
#[repr(C)]
pub enum hprobe_state { HPROBE_LEASED, HPROBE_STABLE, HPROBE_GONE, HPROBE_CONSUMED }

#[cfg(CONFIG_UPROBES)]
#[repr(C)]
pub struct hprobe {
    pub state: hprobe_state,
    pub srcu_scp: *mut srcu_ctr,
    pub uprobe: *mut uprobe,
}

#[cfg(CONFIG_UPROBES)]
#[repr(C)]
pub union uprobe_task_arch {
    pub xol: uprobe_task_xol,
    pub dup: uprobe_task_dup,
}
#[repr(C)] pub struct uprobe_task_xol { pub autask: arch_uprobe_task, pub vaddr: c_ulong }
#[repr(C)] pub struct uprobe_task_dup { pub dup_xol_work: callback_head, pub dup_xol_addr: c_ulong }

#[cfg(CONFIG_UPROBES)]
#[repr(C)]
pub struct uprobe_task {
    pub state: uprobe_task_state,
    pub depth: c_uint,
    pub return_instances: *mut return_instance,
    pub ri_pool: *mut return_instance,
    pub ri_timer: timer_list,
    pub ri_seqcount: seqcount_t,
    pub arch: uprobe_task_arch,
    pub active_uprobe: *mut uprobe,
    pub xol_vaddr: c_ulong,
    pub signal_denied: bool,
    pub auprobe: *mut arch_uprobe,
}

#[repr(C)] pub struct return_consumer { pub cookie: u64, pub id: u64 }
#[repr(C)] pub struct return_instance {
    pub hprobe: hprobe, pub func: c_ulong, pub stack: c_ulong,
    pub orig_ret_vaddr: c_ulong, pub chained: bool, pub cons_cnt: i32,
    pub next: *mut return_instance, pub rcu: rcu_head,
    pub consumer: return_consumer, pub extra_consumers: *mut return_consumer,
}

#[repr(C)]
pub enum rp_check { RP_CHECK_CALL, RP_CHECK_CHAIN_CALL, RP_CHECK_RET }
#[repr(C)] pub struct uprobes_state { pub xol_area: *mut xol_area }

pub type uprobe_write_verify_t = unsafe extern "C" fn(*mut page, c_ulong, *mut uprobe_opcode_t, i32, *mut c_void) -> i32;

#[cfg(CONFIG_UPROBES)]
extern "C" {
    pub fn uprobes_init();
    pub fn set_swbp(_: *mut arch_uprobe, _: *mut vm_area_struct, _: c_ulong) -> i32;
    pub fn set_orig_insn(_: *mut arch_uprobe, _: *mut vm_area_struct, _: c_ulong) -> i32;
    pub fn is_swbp_insn(_: *mut uprobe_opcode_t) -> bool;
    pub fn is_trap_insn(_: *mut uprobe_opcode_t) -> bool;
    pub fn uprobe_get_swbp_addr(_: *mut pt_regs) -> c_ulong;
    pub fn uprobe_get_trap_addr(_: *mut pt_regs) -> c_ulong;
    pub fn uprobe_write_opcode(_: *mut arch_uprobe, _: *mut vm_area_struct, _: c_ulong, _: uprobe_opcode_t, _: bool) -> i32;
    pub fn uprobe_write(_: *mut arch_uprobe, _: *mut vm_area_struct, _: c_ulong, _: *mut uprobe_opcode_t, _: i32, _: Option<uprobe_write_verify_t>, _: bool, _: bool, _: *mut c_void) -> i32;
    pub fn uprobe_register(_: *mut inode, _: loff_t, _: loff_t, _: *mut uprobe_consumer) -> *mut uprobe;
    pub fn uprobe_apply(_: *mut uprobe, _: *mut uprobe_consumer, _: bool) -> i32;
    pub fn uprobe_unregister_nosync(_: *mut uprobe, _: *mut uprobe_consumer);
    pub fn uprobe_unregister_sync();
    pub fn uprobe_mmap(_: *mut vm_area_struct) -> i32;
    pub fn uprobe_munmap(_: *mut vm_area_struct, _: c_ulong, _: c_ulong);
    pub fn uprobe_start_dup_mmap(); pub fn uprobe_end_dup_mmap();
    pub fn uprobe_dup_mmap(_: *mut mm_struct, _: *mut mm_struct);
    pub fn uprobe_free_utask(_: *mut task_struct); pub fn uprobe_copy_process(_: *mut task_struct, _: u64);
    pub fn uprobe_post_sstep_notifier(_: *mut pt_regs) -> i32; pub fn uprobe_pre_sstep_notifier(_: *mut pt_regs) -> i32;
    pub fn uprobe_notify_resume(_: *mut pt_regs); pub fn uprobe_deny_signal() -> bool;
    pub fn arch_uprobe_skip_sstep(_: *mut arch_uprobe, _: *mut pt_regs) -> bool;
    pub fn uprobe_clear_state(_: *mut mm_struct); pub fn arch_uprobe_analyze_insn(_: *mut arch_uprobe, _: *mut mm_struct, _: c_ulong) -> i32;
    pub fn arch_uprobe_pre_xol(_: *mut arch_uprobe, _: *mut pt_regs) -> i32; pub fn arch_uprobe_post_xol(_: *mut arch_uprobe, _: *mut pt_regs) -> i32;
    pub fn arch_uprobe_xol_was_trapped(_: *mut task_struct) -> bool;
    pub fn arch_uprobe_exception_notify(_: *mut notifier_block, _: c_ulong, _: *mut c_void) -> i32;
    pub fn arch_uprobe_abort_xol(_: *mut arch_uprobe, _: *mut pt_regs);
    pub fn arch_uretprobe_hijack_return_addr(_: c_ulong, _: *mut pt_regs) -> c_ulong;
    pub fn arch_uretprobe_is_alive(_: *mut return_instance, _: rp_check, _: *mut pt_regs) -> bool;
    pub fn arch_uprobe_ignore(_: *mut arch_uprobe, _: *mut pt_regs) -> bool;
    pub fn arch_uprobe_copy_ixol(_: *mut page, _: c_ulong, _: *mut c_void, _: c_ulong);
    pub fn uprobe_handle_trampoline(_: *mut pt_regs); pub fn arch_uretprobe_trampoline(_: *mut c_ulong) -> *mut c_void;
    pub fn uprobe_get_trampoline_vaddr() -> c_ulong; pub fn uprobe_copy_from_page(_: *mut page, _: c_ulong, _: *mut c_void, _: i32);
    pub fn handle_syscall_uprobe(_: *mut pt_regs, _: c_ulong); pub fn arch_uprobe_optimize(_: *mut arch_uprobe, _: c_ulong);
    pub fn arch_uprobe_get_xol_area() -> c_ulong;
}

#[cfg(not(CONFIG_UPROBES))]
pub unsafe fn uprobes_init() {}
#[cfg(not(CONFIG_UPROBES))]
pub unsafe fn uprobe_get_trap_addr(regs: *mut pt_regs) -> c_ulong { instruction_pointer(regs) }
#[cfg(not(CONFIG_UPROBES))]
pub unsafe fn uprobe_register(_: *mut inode, _: loff_t, _: loff_t, _: *mut uprobe_consumer) -> *mut uprobe { (-38isize) as *mut uprobe }
#[cfg(not(CONFIG_UPROBES))]
pub unsafe fn uprobe_apply(_: *mut uprobe, _: *mut uprobe_consumer, _: bool) -> i32 { -38 }
#[cfg(not(CONFIG_UPROBES))] pub unsafe fn uprobe_unregister_nosync(_: *mut uprobe, _: *mut uprobe_consumer) {}
#[cfg(not(CONFIG_UPROBES))] pub unsafe fn uprobe_unregister_sync() {}
#[cfg(not(CONFIG_UPROBES))] pub unsafe fn uprobe_mmap(_: *mut vm_area_struct) -> i32 { 0 }
#[cfg(not(CONFIG_UPROBES))] pub unsafe fn uprobe_munmap(_: *mut vm_area_struct, _: c_ulong, _: c_ulong) {}
#[cfg(not(CONFIG_UPROBES))] pub unsafe fn uprobe_start_dup_mmap() {}
#[cfg(not(CONFIG_UPROBES))] pub unsafe fn uprobe_end_dup_mmap() {}
#[cfg(not(CONFIG_UPROBES))] pub unsafe fn uprobe_dup_mmap(_: *mut mm_struct, _: *mut mm_struct) {}
#[cfg(not(CONFIG_UPROBES))] pub unsafe fn uprobe_notify_resume(_: *mut pt_regs) {}
#[cfg(not(CONFIG_UPROBES))] pub unsafe fn uprobe_deny_signal() -> bool { false }
#[cfg(not(CONFIG_UPROBES))] pub unsafe fn uprobe_free_utask(_: *mut task_struct) {}
#[cfg(not(CONFIG_UPROBES))] pub unsafe fn uprobe_copy_process(_: *mut task_struct, _: u64) {}
#[cfg(not(CONFIG_UPROBES))] pub unsafe fn uprobe_clear_state(_: *mut mm_struct) {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
