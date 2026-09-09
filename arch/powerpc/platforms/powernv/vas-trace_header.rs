/* SPDX-License-Identifier: GPL-2.0+ */

// TRACE_SYSTEM: vas
// This header is conditionally included by the kernel tracepoint machinery.

// Dependencies supplied by the surrounding kernel translation:
// linux/tracepoint.h, linux/sched.h, asm/vas.h, and trace/define_trace.h.

#[repr(C)]
pub struct VasRxWinOpenEntry {
    pub tsk: *mut task_struct,
    pub pid: i32,
    pub cop: i32,
    pub vasid: i32,
    pub rxattr: *mut vas_rx_win_attr,
    pub lnotify_lpid: i32,
    pub lnotify_pid: i32,
    pub lnotify_tid: i32,
}

/// TRACE_EVENT(vas_rx_win_open)
/// TP_printk("pid=%d, vasid=%d, cop=%d, lpid=%d, pid=%d, tid=%d", ...)
#[inline]
pub unsafe fn vas_rx_win_open(
    tsk: *mut task_struct,
    vasid: i32,
    cop: i32,
    rxattr: *mut vas_rx_win_attr,
) -> VasRxWinOpenEntry {
    VasRxWinOpenEntry {
        tsk,
        pid: (*tsk).pid,
        cop,
        vasid,
        rxattr,
        lnotify_lpid: (*rxattr).lnotify_lpid,
        lnotify_pid: (*rxattr).lnotify_pid,
        lnotify_tid: (*rxattr).lnotify_tid,
    }
}

#[repr(C)]
pub struct VasTxWinOpenEntry {
    pub tsk: *mut task_struct,
    pub pid: i32,
    pub cop: i32,
    pub vasid: i32,
    pub txattr: *mut vas_tx_win_attr,
    pub lpid: i32,
    pub pidr: i32,
}

/// TRACE_EVENT(vas_tx_win_open)
/// TP_printk("pid=%d, vasid=%d, cop=%d, lpid=%d, pidr=%d", ...)
#[inline]
pub unsafe fn vas_tx_win_open(
    tsk: *mut task_struct,
    vasid: i32,
    cop: i32,
    txattr: *mut vas_tx_win_attr,
) -> VasTxWinOpenEntry {
    VasTxWinOpenEntry {
        tsk,
        pid: (*tsk).pid,
        cop,
        vasid,
        txattr,
        lpid: (*txattr).lpid,
        pidr: (*txattr).pidr,
    }
}

#[repr(C)]
pub struct VasPasteCrbEntry {
    pub tsk: *mut task_struct,
    pub win: *mut vas_window,
    pub pid: i32,
    pub vasid: i32,
    pub winid: i32,
    pub paste_kaddr: usize,
}

/// TRACE_EVENT(vas_paste_crb)
/// TP_printk("pid=%d, vasid=%d, winid=%d, paste_kaddr=0x%016lx\\n", ...)
#[inline]
pub unsafe fn vas_paste_crb(
    tsk: *mut task_struct,
    win: *mut pnv_vas_window,
) -> VasPasteCrbEntry {
    VasPasteCrbEntry {
        tsk,
        win: win as *mut vas_window,
        pid: (*tsk).pid,
        vasid: (*win).vinst.vas_id,
        winid: (*win).vas_win.winid,
        paste_kaddr: (*win).paste_kaddr as usize,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
