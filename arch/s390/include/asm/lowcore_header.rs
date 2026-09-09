/* SPDX-License-Identifier: GPL-2.0 */
/* Direct Rust translation of asm/lowcore.h. */

pub const LC_ORDER: usize = 1;
pub const LC_PAGES: usize = 2;
pub const LOWCORE_ALT_ADDRESS: usize = 0x70000;

#[repr(C)]
pub struct pgm_tdb {
    pub data: [u64; 32],
}

#[repr(C)]
pub union lowcore_ext_int {
    pub fields: lowcore_ext_int_fields,
    pub ext_int_code_addr: u32,
}

#[repr(C)]
pub struct lowcore_ext_int_fields {
    pub ext_cpu_addr: u16,
    pub ext_int_code: u16,
}

#[repr(C)]
pub union lowcore_pgm_int {
    pub fields: lowcore_pgm_int_fields,
    pub pgm_int_code: u32,
}

#[repr(C)]
pub struct lowcore_pgm_int_fields {
    pub pgm_ilc: u16,
    pub pgm_code: u16,
}

#[repr(C)]
pub union lowcore_per {
    pub fields: lowcore_per_fields,
    pub per_code_combined: u16,
}

#[repr(C)]
pub struct lowcore_per_fields {
    pub per_code: u8,
    pub per_atmid: u8,
}

#[repr(C)]
pub union lowcore_io {
    pub fields: lowcore_io_fields,
    pub tpi_info: tpi_info,
}

#[repr(C)]
pub struct lowcore_io_fields {
    pub subchannel_id: u16,
    pub subchannel_nr: u16,
    pub io_int_parm: u32,
    pub io_int_word: u32,
}

#[repr(C)]
pub union lowcore_preempt {
    pub preempt: lowcore_preempt_fields,
    pub preempt_count: u64,
}

#[repr(C)]
pub struct lowcore_preempt_fields {
    pub need_resched: u32,
    pub count: u32,
}

#[repr(C, packed(1), align(8192))]
pub struct lowcore {
    pub pad_0x0000: [u8; 0x0014 - 0x0000],
    pub ipl_parmblock_ptr: u32,
    pub pad_0x0018: [u8; 0x0080 - 0x0018],
    pub ext_params: u32,
    pub ext_int: lowcore_ext_int,
    pub svc_int_code: u32,
    pub pgm_int: lowcore_pgm_int,
    pub data_exc_code: u32,
    pub mon_class_num: u16,
    pub per: lowcore_per,
    pub per_address: u64,
    pub exc_access_id: u8,
    pub per_access_id: u8,
    pub op_access_id: u8,
    pub ar_mode_id: u8,
    pub pad_0x00a4: [u8; 0x00a8 - 0x00a4],
    pub trans_exc_code: u64,
    pub monitor_code: u64,
    pub io: lowcore_io,
    pub pad_0x00c4: [u8; 0x00c8 - 0x00c4],
    pub stfl_fac_list: u32,
    pub pad_0x00cc: [u8; 0x00e8 - 0x00cc],
    pub mcck_interruption_code: u64,
    pub pad_0x00f0: [u8; 0x00f4 - 0x00f0],
    pub external_damage_code: u32,
    pub failing_storage_address: u64,
    pub pad_0x0100: [u8; 0x0110 - 0x0100],
    pub pgm_last_break: u64,
    pub pad_0x0118: [u8; 0x0120 - 0x0118],
    pub restart_old_psw: psw_t,
    pub external_old_psw: psw_t,
    pub svc_old_psw: psw_t,
    pub program_old_psw: psw_t,
    pub mcck_old_psw: psw_t,
    pub io_old_psw: psw_t,
    pub pad_0x0180: [u8; 0x01a0 - 0x0180],
    pub restart_psw: psw_t,
    pub external_new_psw: psw_t,
    pub svc_new_psw: psw_t,
    pub program_new_psw: psw_t,
    pub mcck_new_psw: psw_t,
    pub io_new_psw: psw_t,
    pub save_area: [u64; 8],
    pub stack_canary: u64,
    pub pad_0x0248: [u8; 0x0280 - 0x0248],
    pub save_area_restart: [u64; 1],
    pub pcpu: u64,
    pub return_psw: psw_t,
    pub return_mcck_psw: psw_t,
    pub last_break: u64,
    pub sys_enter_timer: u64,
    pub mcck_enter_timer: u64,
    pub exit_timer: u64,
    pub user_timer: u64,
    pub guest_timer: u64,
    pub system_timer: u64,
    pub hardirq_timer: u64,
    pub softirq_timer: u64,
    pub steal_timer: u64,
    pub avg_steal_timer: u64,
    pub last_update_timer: u64,
    pub last_update_clock: u64,
    pub int_clock: tod_clock,
    pub clock_comparator: u64,
    pub pad_0x0330: [u8; 0x0340 - 0x0330],
    pub current_task: u64,
    pub kernel_stack: u64,
    pub async_stack: u64,
    pub nodat_stack: u64,
    pub restart_stack: u64,
    pub mcck_stack: u64,
    pub restart_fn: u64,
    pub restart_data: u64,
    pub restart_source: u32,
    pub restart_flags: u32,
    pub kernel_asce: ctlreg,
    pub user_asce: ctlreg,
    pub lpp: u32,
    pub current_pid: u32,
    pub cpu_nr: u32,
    pub softirq_pending: u32,
    pub preempt_info: lowcore_preempt,
    pub spinlock_lockval: u32,
    pub spinlock_index: u32,
    pub percpu_offset: u64,
    pub percpu_register: u8,
    pub pad_0x03c1: [u8; 0x0400 - 0x03c1],
    pub return_lpswe: u32,
    pub return_mcck_lpswe: u32,
    pub pad_0x040a: [u8; 0x0e00 - 0x0408],
    pub ipib: u64,
    pub ipib_checksum: u32,
    pub vmcore_info: u64,
    pub pad_0x0e14: [u8; 0x0e18 - 0x0e14],
    pub os_info: u64,
    pub pad_0x0e20: [u8; 0x11b0 - 0x0e20],
    pub mcesad: u64,
    pub ext_params2: u64,
    pub pad_0x11c0: [u8; 0x1200 - 0x11c0],
    pub floating_pt_save_area: [u64; 16],
    pub gpregs_save_area: [u64; 16],
    pub psw_save_area: psw_t,
    pub pad_0x1310: [u8; 0x1318 - 0x1310],
    pub prefixreg_save_area: u32,
    pub fpt_creg_save_area: u32,
    pub pad_0x1320: [u8; 0x1324 - 0x1320],
    pub tod_progreg_save_area: u32,
    pub cpu_timer_save_area: [u32; 2],
    pub clock_comp_save_area: [u32; 2],
    pub last_break_save_area: u64,
    pub access_regs_save_area: [u32; 16],
    pub cregs_save_area: [ctlreg; 16],
    pub pad_0x1400: [u8; 0x1500 - 0x1400],
    pub ccd: u64,
    pub aicd: u64,
    pub pad_0x1510: [u8; 0x1800 - 0x1510],
    pub pgm_tdb: pgm_tdb,
    pub pad_0x1900: [u8; 0x2000 - 0x1900],
}

/* Architecture-specific alternative/assembly implementation retained as intent. */
#[inline(always)]
pub unsafe fn get_lowcore() -> *mut lowcore {
    // __DECOMPRESSOR builds return NULL. The remaining path uses the s390
    // ALTERNATIVE instruction sequence to select the lowcore address.
    core::ptr::null_mut()
}

pub unsafe extern "C" {
    pub static mut lowcore_ptr: *mut *mut lowcore;
}

#[inline]
pub unsafe fn set_prefix(address: u32) {
    // C: asm volatile("spx %0" : : "Q" (address) : "memory");
    core::hint::black_box(address);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
