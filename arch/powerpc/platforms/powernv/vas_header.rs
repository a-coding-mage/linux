/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of vas.h. */

// Dependencies supplied by the surrounding kernel translation unit.

pub const VAS_WINDOWS_PER_CHIP: usize = 64 << 10;
pub const VAS_HVWC_SIZE: usize = 512;
pub const VAS_UWC_SIZE: usize = PAGE_SIZE;
pub const VAS_TX_WCREDS_MAX: i32 = (4 << 10) - 1;
pub const VAS_WCREDS_DEFAULT: i32 = 1 << 10;

pub const VAS_LPID_OFFSET: i32 = 0x010;
pub const VAS_LPID: u64 = PPC_BITMASK(0, 11);
pub const VAS_PID_OFFSET: i32 = 0x018;
pub const VAS_PID_ID: u64 = PPC_BITMASK(0, 19);
pub const VAS_XLATE_MSR_OFFSET: i32 = 0x020;
pub const VAS_XLATE_MSR_DR: u64 = PPC_BIT(0);
pub const VAS_XLATE_MSR_TA: u64 = PPC_BIT(1);
pub const VAS_XLATE_MSR_PR: u64 = PPC_BIT(2);
pub const VAS_XLATE_MSR_US: u64 = PPC_BIT(3);
pub const VAS_XLATE_MSR_HV: u64 = PPC_BIT(4);
pub const VAS_XLATE_MSR_SF: u64 = PPC_BIT(5);
pub const VAS_XLATE_LPCR_OFFSET: i32 = 0x028;
pub const VAS_XLATE_LPCR_PAGE_SIZE: u64 = PPC_BITMASK(0, 2);
pub const VAS_XLATE_LPCR_ISL: u64 = PPC_BIT(3);
pub const VAS_XLATE_LPCR_TC: u64 = PPC_BIT(4);
pub const VAS_XLATE_LPCR_SC: u64 = PPC_BIT(5);
pub const VAS_XLATE_CTL_OFFSET: i32 = 0x030;
pub const VAS_XLATE_MODE: u64 = PPC_BITMASK(0, 1);
pub const VAS_AMR_OFFSET: i32 = 0x040;
pub const VAS_AMR: u64 = PPC_BITMASK(0, 63);
pub const VAS_SEIDR_OFFSET: i32 = 0x048;
pub const VAS_SEIDR: u64 = PPC_BITMASK(0, 63);
pub const VAS_FAULT_TX_WIN_OFFSET: i32 = 0x050;
pub const VAS_FAULT_TX_WIN: u64 = PPC_BITMASK(48, 63);
pub const VAS_OSU_INTR_SRC_RA_OFFSET: i32 = 0x060;
pub const VAS_OSU_INTR_SRC_RA: u64 = PPC_BITMASK(8, 63);
pub const VAS_HV_INTR_SRC_RA_OFFSET: i32 = 0x070;
pub const VAS_HV_INTR_SRC_RA: u64 = PPC_BITMASK(8, 63);
pub const VAS_PSWID_OFFSET: i32 = 0x078;
pub const VAS_PSWID_EA_HANDLE: u64 = PPC_BITMASK(0, 31);
pub const VAS_SPARE1_OFFSET: i32 = 0x080;
pub const VAS_SPARE2_OFFSET: i32 = 0x088;
pub const VAS_SPARE3_OFFSET: i32 = 0x090;
pub const VAS_SPARE4_OFFSET: i32 = 0x130;
pub const VAS_SPARE5_OFFSET: i32 = 0x160;
pub const VAS_SPARE6_OFFSET: i32 = 0x188;
pub const VAS_LFIFO_BAR_OFFSET: i32 = 0x0A0;
pub const VAS_LFIFO_BAR: u64 = PPC_BITMASK(8, 53);
pub const VAS_PAGE_MIGRATION_SELECT: u64 = PPC_BITMASK(54, 56);
pub const VAS_LDATA_STAMP_CTL_OFFSET: i32 = 0x0A8;
pub const VAS_LDATA_STAMP: u64 = PPC_BITMASK(0, 1);
pub const VAS_XTRA_WRITE: u64 = PPC_BIT(2);
pub const VAS_LDMA_CACHE_CTL_OFFSET: i32 = 0x0B0;
pub const VAS_LDMA_TYPE: u64 = PPC_BITMASK(0, 1);
pub const VAS_LDMA_FIFO_DISABLE: u64 = PPC_BIT(2);
pub const VAS_LRFIFO_PUSH_OFFSET: i32 = 0x0B8;
pub const VAS_LRFIFO_PUSH: u64 = PPC_BITMASK(0, 15);
pub const VAS_CURR_MSG_COUNT_OFFSET: i32 = 0x0C0;
pub const VAS_CURR_MSG_COUNT: u64 = PPC_BITMASK(0, 7);
pub const VAS_LNOTIFY_AFTER_COUNT_OFFSET: i32 = 0x0C8;
pub const VAS_LNOTIFY_AFTER_COUNT: u64 = PPC_BITMASK(0, 7);
pub const VAS_LRX_WCRED_OFFSET: i32 = 0x0E0;
pub const VAS_LRX_WCRED: u64 = PPC_BITMASK(0, 15);
pub const VAS_LRX_WCRED_ADDER_OFFSET: i32 = 0x190;
pub const VAS_LRX_WCRED_ADDER: u64 = PPC_BITMASK(0, 15);
pub const VAS_TX_WCRED_OFFSET: i32 = 0x0F0;
pub const VAS_TX_WCRED: u64 = PPC_BITMASK(4, 15);
pub const VAS_TX_WCRED_ADDER_OFFSET: i32 = 0x1A0;
pub const VAS_TX_WCRED_ADDER: u64 = PPC_BITMASK(4, 15);
pub const VAS_LFIFO_SIZE_OFFSET: i32 = 0x100;
pub const VAS_LFIFO_SIZE: u64 = PPC_BITMASK(0, 3);
pub const VAS_WINCTL_OFFSET: i32 = 0x108;
pub const VAS_WINCTL_OPEN: u64 = PPC_BIT(0);
pub const VAS_WINCTL_REJ_NO_CREDIT: u64 = PPC_BIT(1);
pub const VAS_WINCTL_PIN: u64 = PPC_BIT(2);
pub const VAS_WINCTL_TX_WCRED_MODE: u64 = PPC_BIT(3);
pub const VAS_WINCTL_RX_WCRED_MODE: u64 = PPC_BIT(4);
pub const VAS_WINCTL_TX_WORD_MODE: u64 = PPC_BIT(5);
pub const VAS_WINCTL_RX_WORD_MODE: u64 = PPC_BIT(6);
pub const VAS_WINCTL_RSVD_TXBUF: u64 = PPC_BIT(7);
pub const VAS_WINCTL_THRESH_CTL: u64 = PPC_BITMASK(8, 9);
pub const VAS_WINCTL_FAULT_WIN: u64 = PPC_BIT(10);
pub const VAS_WINCTL_NX_WIN: u64 = PPC_BIT(11);
pub const VAS_WIN_STATUS_OFFSET: i32 = 0x110;
pub const VAS_WIN_BUSY: u64 = PPC_BIT(1);
pub const VAS_WIN_CTX_CACHING_CTL_OFFSET: i32 = 0x118;
pub const VAS_CASTOUT_REQ: u64 = PPC_BIT(0);
pub const VAS_PUSH_TO_MEM: u64 = PPC_BIT(1);
pub const VAS_WIN_CACHE_STATUS: u64 = PPC_BIT(4);
pub const VAS_TX_RSVD_BUF_COUNT_OFFSET: i32 = 0x120;
pub const VAS_RXVD_BUF_COUNT: u64 = PPC_BITMASK(58, 63);
pub const VAS_LRFIFO_WIN_PTR_OFFSET: i32 = 0x128;
pub const VAS_LRX_WIN_ID: u64 = PPC_BITMASK(0, 15);
pub const VAS_LNOTIFY_CTL_OFFSET: i32 = 0x138;
pub const VAS_NOTIFY_DISABLE: u64 = PPC_BIT(0);
pub const VAS_INTR_DISABLE: u64 = PPC_BIT(1);
pub const VAS_NOTIFY_EARLY: u64 = PPC_BIT(2);
pub const VAS_NOTIFY_OSU_INTR: u64 = PPC_BIT(3);
pub const VAS_LNOTIFY_PID_OFFSET: i32 = 0x140;
pub const VAS_LNOTIFY_PID: u64 = PPC_BITMASK(0, 19);
pub const VAS_LNOTIFY_LPID_OFFSET: i32 = 0x148;
pub const VAS_LNOTIFY_LPID: u64 = PPC_BITMASK(0, 11);
pub const VAS_LNOTIFY_TID_OFFSET: i32 = 0x150;
pub const VAS_LNOTIFY_TID: u64 = PPC_BITMASK(0, 15);
pub const VAS_LNOTIFY_SCOPE_OFFSET: i32 = 0x158;
pub const VAS_LNOTIFY_MIN_SCOPE: u64 = PPC_BITMASK(0, 1);
pub const VAS_LNOTIFY_MAX_SCOPE: u64 = PPC_BITMASK(2, 3);
pub const VAS_NX_UTIL_OFFSET: i32 = 0x1B0;
pub const VAS_NX_UTIL: u64 = PPC_BITMASK(0, 63);
pub const VAS_NX_UTIL_SE_OFFSET: i32 = 0x1B8;
pub const VAS_NX_UTIL_SE: u64 = PPC_BITMASK(0, 63);
pub const VAS_NX_UTIL_ADDER_OFFSET: i32 = 0x180;
pub const VAS_NX_UTIL_ADDER: u64 = PPC_BITMASK(32, 63);

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum vas_notify_scope { VAS_SCOPE_LOCAL, VAS_SCOPE_GROUP, VAS_SCOPE_VECTORED_GROUP, VAS_SCOPE_UNUSED }
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum vas_dma_type { VAS_DMA_TYPE_INJECT, VAS_DMA_TYPE_WRITE }
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum vas_notify_after_count { VAS_NOTIFY_AFTER_256 = 0, VAS_NOTIFY_NONE, VAS_NOTIFY_AFTER_2 }

pub const FIFO_INVALID_ENTRY: u32 = 0xffffffff;
pub const CCW0_INVALID: u32 = 1;

#[repr(C)]
pub struct vas_instance {
    pub vas_id: i32, pub ida: ida, pub node: list_head, pub pdev: *mut platform_device,
    pub hvwc_bar_start: u64, pub uwc_bar_start: u64, pub paste_base_addr: u64, pub paste_win_id_shift: u64,
    pub irq_port: u64, pub virq: i32, pub fault_crbs: i32, pub fault_fifo_size: i32, pub fifo_in_progress: i32,
    pub fault_lock: spinlock_t, pub fault_fifo: *mut core::ffi::c_void, pub fault_win: *mut pnv_vas_window,
    pub mutex: mutex, pub rxwin: [*mut pnv_vas_window; VAS_COP_TYPE_MAX], pub windows: [*mut pnv_vas_window; VAS_WINDOWS_PER_CHIP],
    pub name: *mut i8, pub dbgname: *mut i8, pub dbgdir: *mut dentry,
}

#[repr(C)]
pub struct pnv_vas_window {
    pub vas_win: vas_window, pub vinst: *mut vas_instance, pub tx_win: bool, pub nx_win: bool, pub user_win: bool,
    pub hvwc_map: *mut core::ffi::c_void, pub uwc_map: *mut core::ffi::c_void, pub paste_kaddr: *mut core::ffi::c_void,
    pub paste_addr_name: *mut i8, pub rxwin: *mut pnv_vas_window, pub num_txwins: atomic_t,
}

#[repr(C)]
pub struct vas_winctx {
    pub rx_fifo: u64, pub rx_fifo_size: i32, pub wcreds_max: i32, pub rsvd_txbuf_count: i32,
    pub user_win: bool, pub nx_win: bool, pub fault_win: bool, pub rsvd_txbuf_enable: bool, pub pin_win: bool,
    pub rej_no_credit: bool, pub tx_wcred_mode: bool, pub rx_wcred_mode: bool, pub tx_word_mode: bool, pub rx_word_mode: bool,
    pub data_stamp: bool, pub xtra_write: bool, pub notify_disable: bool, pub intr_disable: bool, pub fifo_disable: bool,
    pub notify_early: bool, pub notify_os_intr_reg: bool, pub lpid: i32, pub pidr: i32, pub lnotify_lpid: i32,
    pub lnotify_pid: i32, pub lnotify_tid: i32, pub pswid: u32, pub rx_win_id: i32, pub fault_win_id: i32, pub tc_mode: i32,
    pub irq_port: u64, pub dma_type: vas_dma_type, pub min_scope: vas_notify_scope, pub max_scope: vas_notify_scope,
    pub notify_after_count: vas_notify_after_count,
}

extern "C" {
    pub static mut vas_mutex: mutex;
    pub fn find_vas_instance(vasid: i32) -> *mut vas_instance;
    pub fn vas_init_dbgdir();
    pub fn vas_instance_init_dbgdir(vinst: *mut vas_instance);
    pub fn vas_window_init_dbgdir(win: *mut pnv_vas_window);
    pub fn vas_window_free_dbgdir(win: *mut pnv_vas_window);
    pub fn vas_setup_fault_window(vinst: *mut vas_instance) -> i32;
    pub fn vas_fault_thread_fn(irq: i32, data: *mut core::ffi::c_void) -> irqreturn_t;
    pub fn vas_fault_handler(irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t;
    pub fn vas_return_credit(window: *mut pnv_vas_window, tx: bool);
    pub fn vas_pswid_to_window(vinst: *mut vas_instance, pswid: u32) -> *mut pnv_vas_window;
    pub fn vas_win_paste_addr(window: *mut pnv_vas_window, addr: *mut u64, len: *mut i32);
}

#[inline]
pub unsafe fn vas_window_pid(window: *mut vas_window) -> i32 { pid_vnr((*window).task_ref.pid) }

#[inline]
pub unsafe fn vas_log_write(win: *mut pnv_vas_window, name: *mut i8, regptr: *mut core::ffi::c_void, val: u64) {
    if val != 0 { pr_debug("%swin #%d: %s reg %p, val 0x%016llx\n", if (*win).tx_win { "Tx" } else { "Rx" }, (*win).vas_win.winid, name, regptr, val); }
}

#[inline]
pub unsafe fn write_uwc_reg(win: *mut pnv_vas_window, name: *mut i8, reg: i32, val: u64) {
    let regptr = ((*win).uwc_map as *mut u8).offset(reg as isize) as *mut core::ffi::c_void;
    vas_log_write(win, name, regptr, val); out_be64(regptr, val);
}
#[inline]
pub unsafe fn write_hvwc_reg(win: *mut pnv_vas_window, name: *mut i8, reg: i32, val: u64) {
    let regptr = ((*win).hvwc_map as *mut u8).offset(reg as isize) as *mut core::ffi::c_void;
    vas_log_write(win, name, regptr, val); out_be64(regptr, val);
}
#[inline]
pub unsafe fn read_hvwc_reg(win: *mut pnv_vas_window, _name: *mut i8, reg: i32) -> u64 {
    in_be64(((*win).hvwc_map as *mut u8).offset(reg as isize) as *mut core::ffi::c_void)
}

#[inline]
pub const fn encode_pswid(vasid: i32, winid: i32) -> u32 { (winid as u32) | ((vasid as u32) << (31 - 7)) }
#[inline]
pub unsafe fn decode_pswid(pswid: u32, vasid: *mut i32, winid: *mut i32) {
    if !vasid.is_null() { *vasid = ((pswid >> (31 - 7)) & 0xFF) as i32; }
    if !winid.is_null() { *winid = (pswid & 0xFFFF) as i32; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
