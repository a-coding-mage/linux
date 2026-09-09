/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Copyright 2016-17 IBM Corp. */

// Dependencies supplied by the surrounding kernel translation unit.

pub const VAS_RX_FIFO_SIZE_MIN: u32 = 1 << 10;
pub const VAS_RX_FIFO_SIZE_MAX: u32 = 8 << 20;

pub const VAS_THRESH_DISABLED: i32 = 0;
pub const VAS_THRESH_FIFO_GT_HALF_FULL: i32 = 1;
pub const VAS_THRESH_FIFO_GT_QTR_FULL: i32 = 2;
pub const VAS_THRESH_FIFO_GT_EIGHTH_FULL: i32 = 3;

pub const VAS_WIN_ACTIVE: u32 = 0x0;
pub const VAS_WIN_NO_CRED_CLOSE: u32 = 0x00000001;
pub const VAS_WIN_MIGRATE_CLOSE: u32 = 0x00000002;

// C macros retained as Rust macros because their operand types are intentional.
#[macro_export]
macro_rules! MASK_LSH {
    ($m:expr) => {{ ($m as u64).trailing_zeros() }};
}
#[macro_export]
macro_rules! GET_FIELD {
    ($m:expr, $v:expr) => {{ (($v & $m) >> MASK_LSH!($m)) }};
}
#[macro_export]
macro_rules! SET_FIELD {
    ($m:expr, $v:expr, $val:expr) => {{
        (($v & !($m)) | ((($val as _) << MASK_LSH!($m)) & $m))
    }};
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum vas_cop_type {
    VAS_COP_TYPE_FAULT = 0,
    VAS_COP_TYPE_842,
    VAS_COP_TYPE_842_HIPRI,
    VAS_COP_TYPE_GZIP,
    VAS_COP_TYPE_GZIP_HIPRI,
    VAS_COP_TYPE_FTW,
    VAS_COP_TYPE_MAX,
}

#[repr(C)]
pub struct vas_user_win_ref {
    pub pid: *mut pid,
    pub tgid: *mut pid,
    pub mm: *mut mm_struct,
    pub mmap_mutex: mutex,
    pub vma: *mut vm_area_struct,
}

#[repr(C)]
pub struct vas_window {
    pub winid: u32,
    pub wcreds_max: u32,
    pub status: u32,
    pub cop: vas_cop_type,
    pub task_ref: vas_user_win_ref,
    pub dbgname: *mut core::ffi::c_char,
    pub dbgdir: *mut dentry,
}

#[repr(C)]
pub struct vas_user_win_ops {
    pub open_win: Option<unsafe extern "C" fn(i32, u64, vas_cop_type) -> *mut vas_window>,
    pub paste_addr: Option<unsafe extern "C" fn(*mut vas_window) -> u64>,
    pub close_win: Option<unsafe extern "C" fn(*mut vas_window) -> i32>,
}

pub unsafe fn put_vas_user_win_ref(ref_: *mut vas_user_win_ref) {
    put_pid((*ref_).pid);
    put_pid((*ref_).tgid);
    if !(*ref_).mm.is_null() {
        mmdrop((*ref_).mm);
    }
}

pub unsafe fn vas_user_win_add_mm_context(ref_: *mut vas_user_win_ref) {
    mm_context_add_vas_window((*ref_).mm);
    // The C implementation issues PPC_CP_ABORT here to clear pending COPY state.
    core::arch::asm!("", options(nostack, preserves_flags));
}

#[repr(C)]
pub struct vas_rx_win_attr {
    pub rx_fifo: u64,
    pub rx_fifo_size: i32,
    pub wcreds_max: i32,
    pub pin_win: bool,
    pub rej_no_credit: bool,
    pub tx_wcred_mode: bool,
    pub rx_wcred_mode: bool,
    pub tx_win_ord_mode: bool,
    pub rx_win_ord_mode: bool,
    pub data_stamp: bool,
    pub nx_win: bool,
    pub fault_win: bool,
    pub user_win: bool,
    pub notify_disable: bool,
    pub intr_disable: bool,
    pub notify_early: bool,
    pub lnotify_lpid: i32,
    pub lnotify_pid: i32,
    pub lnotify_tid: i32,
    pub pswid: u32,
    pub tc_mode: i32,
}

#[repr(C)]
pub struct vas_tx_win_attr {
    pub cop: vas_cop_type,
    pub wcreds_max: i32,
    pub lpid: i32,
    pub pidr: i32,
    pub pswid: i32,
    pub rsvd_txbuf_count: i32,
    pub tc_mode: i32,
    pub user_win: bool,
    pub pin_win: bool,
    pub rej_no_credit: bool,
    pub rsvd_txbuf_enable: bool,
    pub tx_wcred_mode: bool,
    pub rx_wcred_mode: bool,
    pub tx_win_ord_mode: bool,
    pub rx_win_ord_mode: bool,
}

// CONFIG_PPC_POWERNV declarations.
extern "C" {
    pub fn chip_to_vas_id(chipid: i32) -> i32;
    pub fn vas_init_rx_win_attr(rxattr: *mut vas_rx_win_attr, cop: vas_cop_type);
    pub fn vas_rx_win_open(vasid: i32, cop: vas_cop_type, attr: *mut vas_rx_win_attr) -> *mut vas_window;
    pub fn vas_init_tx_win_attr(txattr: *mut vas_tx_win_attr, cop: vas_cop_type);
    pub fn vas_tx_win_open(vasid: i32, cop: vas_cop_type, attr: *mut vas_tx_win_attr) -> *mut vas_window;
    pub fn vas_win_close(win: *mut vas_window) -> i32;
    pub fn vas_copy_crb(crb: *mut core::ffi::c_void, offset: i32) -> i32;
    pub fn vas_paste_crb(win: *mut vas_window, offset: i32, re: bool) -> i32;
    pub fn vas_register_api_powernv(mod_: *mut module, cop_type: vas_cop_type, name: *const core::ffi::c_char) -> i32;
    pub fn vas_unregister_api_powernv();
}

pub const VAS_GZIP_QOS_FEAT: u32 = 0x1;
pub const VAS_GZIP_DEF_FEAT: u32 = 0x2;
pub const VAS_NX_GZIP_FEAT: u32 = 0x1;

#[repr(C, align(4096))]
pub struct hv_vas_all_caps {
    pub descriptor: u64,
    pub feat_type: u64,
}

#[repr(C)]
pub struct vas_all_caps { pub descriptor: u64, pub feat_type: u64 }

extern "C" {
    pub fn h_query_vas_capabilities(hcall: u64, query_type: u8, result: u64) -> i32;
    pub fn vas_register_api_pseries(mod_: *mut module, cop_type: vas_cop_type, name: *const core::ffi::c_char) -> i32;
    pub fn vas_unregister_api_pseries();
    pub fn vas_register_coproc_api(mod_: *mut module, cop_type: vas_cop_type, name: *const core::ffi::c_char, vops: *const vas_user_win_ops) -> i32;
    pub fn vas_unregister_coproc_api();
    pub fn get_vas_user_win_ref(task_ref: *mut vas_user_win_ref) -> i32;
    pub fn vas_update_csb(crb: *mut coprocessor_request_block, task_ref: *mut vas_user_win_ref);
    pub fn vas_dump_crb(crb: *mut coprocessor_request_block);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
