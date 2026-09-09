/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding kernel translation.

/* Max number of levels to backtrace */
pub const MAX_UNWIND_ENTRIES: usize = 30;

/* From ABI specifications */
#[repr(C)]
pub struct unwind_table_entry {
    pub region_start: core::ffi::c_uint,
    pub region_end: core::ffi::c_uint,
    /* C bitfields occupying one 32-bit word. */
    pub flags: core::ffi::c_uint,
    /* C bitfields occupying one 32-bit word. */
    pub frame_flags: core::ffi::c_uint,
}

/* Bit positions in unwind_table_entry::flags. */
pub const UNWIND_CANNOT_UNWIND: u32 = 0;
pub const UNWIND_MILLICODE: u32 = 1;
pub const UNWIND_MILLICODE_SAVE_SR0: u32 = 2;
pub const UNWIND_REGION_DESCRIPTION_SHIFT: u32 = 3;
pub const UNWIND_REGION_DESCRIPTION_MASK: u32 = 0x3;
pub const UNWIND_RESERVED1: u32 = 5;
pub const UNWIND_ENTRY_SR: u32 = 6;
pub const UNWIND_ENTRY_FR_SHIFT: u32 = 7;
pub const UNWIND_ENTRY_FR_MASK: u32 = 0xf;
pub const UNWIND_ENTRY_GR_SHIFT: u32 = 11;
pub const UNWIND_ENTRY_GR_MASK: u32 = 0x1f;
pub const UNWIND_ARGS_STORED: u32 = 16;
pub const UNWIND_VARIABLE_FRAME: u32 = 17;
pub const UNWIND_SEPARATE_PACKAGE_BODY: u32 = 18;
pub const UNWIND_FRAME_EXTENSION_MILLICODE: u32 = 19;
pub const UNWIND_STACK_OVERFLOW_CHECK: u32 = 20;
pub const UNWIND_TWO_INSTRUCTION_SP_INCREMENT: u32 = 21;
pub const UNWIND_ADA_REGION: u32 = 22;
pub const UNWIND_CXX_INFO: u32 = 23;
pub const UNWIND_CXX_TRY_CATCH: u32 = 24;
pub const UNWIND_SCHED_ENTRY_SEQ: u32 = 25;
pub const UNWIND_RESERVED2: u32 = 26;
pub const UNWIND_SAVE_SP: u32 = 27;
pub const UNWIND_SAVE_RP: u32 = 28;
pub const UNWIND_SAVE_MRP_IN_FRAME: u32 = 29;
pub const UNWIND_EXTN_PTR_DEFINED: u32 = 30;
pub const UNWIND_CLEANUP_DEFINED: u32 = 31;

/* Bit positions in unwind_table_entry::frame_flags. */
pub const UNWIND_MPE_XL_INTERRUPT_MARKER: u32 = 0;
pub const UNWIND_HP_UX_INTERRUPT_MARKER: u32 = 1;
pub const UNWIND_LARGE_FRAME: u32 = 2;
pub const UNWIND_PSEUDO_SP_SET: u32 = 3;
pub const UNWIND_RESERVED4: u32 = 4;
pub const UNWIND_TOTAL_FRAME_SIZE_SHIFT: u32 = 5;
pub const UNWIND_TOTAL_FRAME_SIZE_MASK: u32 = 0x07ff_ffff;

#[repr(C)]
pub struct unwind_table {
    pub list: list_head,
    pub name: *const core::ffi::c_char,
    pub gp: core::ffi::c_ulong,
    pub base_addr: core::ffi::c_ulong,
    pub start: core::ffi::c_ulong,
    pub end: core::ffi::c_ulong,
    pub table: *const unwind_table_entry,
    pub length: core::ffi::c_ulong,
}

#[repr(C)]
pub struct unwind_frame_info {
    pub t: *mut task_struct,
    /* Eventually we would like to be able to get at any of the registers
       available; but for now we only try to get the sp and ip for each
       frame */
    /* struct pt_regs regs; */
    pub sp: core::ffi::c_ulong,
    pub ip: core::ffi::c_ulong,
    pub rp: core::ffi::c_ulong,
    pub r31: core::ffi::c_ulong,
    pub prev_sp: core::ffi::c_ulong,
    pub prev_ip: core::ffi::c_ulong,
}

extern "C" {
    pub fn unwind_table_add(
        name: *const core::ffi::c_char,
        base_addr: core::ffi::c_ulong,
        gp: core::ffi::c_ulong,
        start: *mut core::ffi::c_void,
        end: *mut core::ffi::c_void,
    ) -> *mut unwind_table;
    pub fn unwind_table_remove(table: *mut unwind_table);
    pub fn unwind_frame_init(
        info: *mut unwind_frame_info,
        t: *mut task_struct,
        regs: *mut pt_regs,
    );
    pub fn unwind_frame_init_from_blocked_task(info: *mut unwind_frame_info, t: *mut task_struct);
    pub fn unwind_frame_init_task(
        info: *mut unwind_frame_info,
        task: *mut task_struct,
        regs: *mut pt_regs,
    );
    pub fn unwind_once(info: *mut unwind_frame_info) -> core::ffi::c_int;
    pub fn unwind_to_user(info: *mut unwind_frame_info) -> core::ffi::c_int;
    pub fn unwind_init() -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
