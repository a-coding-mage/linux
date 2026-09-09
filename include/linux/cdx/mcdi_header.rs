/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright 2008-2013 Solarflare Communications Inc.
 * Copyright (C) 2022-2023, Advanced Micro Devices, Inc.
 */

// Dependencies supplied by the surrounding kernel/Rust translation.

/**
 * enum cdx_mcdi_mode - MCDI transaction mode
 * @MCDI_MODE_EVENTS: wait for an mcdi response callback.
 * @MCDI_MODE_FAIL: we think MCDI is dead, so fail-fast all calls
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum cdx_mcdi_mode {
    MCDI_MODE_EVENTS,
    MCDI_MODE_FAIL,
}

pub const MCDI_RPC_TIMEOUT: usize = 10 * HZ;
pub const MCDI_RPC_LONG_TIMEOU: usize = 60 * HZ;
pub const MCDI_RPC_POST_RST_TIME: usize = 10 * HZ;

/**
 * enum cdx_mcdi_cmd_state - State for an individual MCDI command
 * @MCDI_STATE_QUEUED: Command not started and is waiting to run.
 * @MCDI_STATE_RETRY: Command was submitted and MC rejected with no resources,
 *	as MC have too many outstanding commands. Command will be retried once
 *	another command returns.
 * @MCDI_STATE_RUNNING: Command was accepted and is running.
 * @MCDI_STATE_RUNNING_CANCELLED: Command is running but the issuer cancelled
 *	the command.
 * @MCDI_STATE_FINISHED: Processing of this command has completed.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum cdx_mcdi_cmd_state {
    MCDI_STATE_QUEUED,
    MCDI_STATE_RETRY,
    MCDI_STATE_RUNNING,
    MCDI_STATE_RUNNING_CANCELLED,
    MCDI_STATE_FINISHED,
}

#[repr(C)]
pub struct cdx_mcdi {
    pub mcdi: *mut cdx_mcdi_data,
    pub mcdi_ops: *const cdx_mcdi_ops,
    pub r5_rproc: *mut rproc,
    pub rpdev: *mut rpmsg_device,
    pub ept: *mut rpmsg_endpoint,
    pub work: work_struct,
}

#[repr(C)]
pub struct cdx_mcdi_ops {
    pub mcdi_request: Option<unsafe extern "C" fn(*mut cdx_mcdi, *const cdx_dword, usize, *const cdx_dword, usize)>,
    pub mcdi_rpc_timeout: Option<unsafe extern "C" fn(*mut cdx_mcdi, c_uint) -> c_uint>,
}

pub type cdx_mcdi_async_completer = unsafe extern "C" fn(*mut cdx_mcdi, c_ulong, c_int, *mut cdx_dword, usize);

#[repr(C)]
pub struct cdx_mcdi_cmd {
    pub ref_: kref,
    pub list: list_head,
    pub cleanup_list: list_head,
    pub work: work_struct,
    pub mcdi: *mut cdx_mcdi_iface,
    pub state: cdx_mcdi_cmd_state,
    pub inlen: usize,
    pub inbuf: *const cdx_dword,
    pub quiet: bool,
    pub reboot_seen: bool,
    pub seq: u8,
    pub started: c_ulong,
    pub cookie: c_ulong,
    pub completer: Option<cdx_mcdi_async_completer>,
    pub handle: c_uint,
    pub cmd: c_uint,
    pub rc: c_int,
    pub outlen: usize,
    pub outbuf: *mut cdx_dword,
}

#[repr(C)]
pub struct cdx_mcdi_iface {
    pub cdx: *mut cdx_mcdi,
    pub iface_lock: mutex,
    pub outstanding_cleanups: c_uint,
    pub cmd_list: list_head,
    pub workqueue: *mut workqueue_struct,
    pub cmd_complete_wq: wait_queue_head_t,
    pub db_held_by: *mut cdx_mcdi_cmd,
    pub seq_held_by: [*mut cdx_mcdi_cmd; 16],
    pub prev_handle: c_uint,
    pub mode: cdx_mcdi_mode,
    pub prev_seq: u8,
    pub new_epoch: bool,
}

#[repr(C)]
pub struct cdx_mcdi_data {
    pub iface: cdx_mcdi_iface,
    pub fn_flags: u32,
}

unsafe extern "C" {
    pub fn cdx_mcdi_finish(cdx: *mut cdx_mcdi);
    pub fn cdx_mcdi_init(cdx: *mut cdx_mcdi) -> c_int;
    pub fn cdx_mcdi_process_cmd(cdx: *mut cdx_mcdi, outbuf: *mut cdx_dword, len: c_int);
    pub fn cdx_mcdi_rpc(cdx: *mut cdx_mcdi, cmd: c_uint, inbuf: *const cdx_dword, inlen: usize,
                        outbuf: *mut cdx_dword, outlen: usize, outlen_actual: *mut usize) -> c_int;
}

/* We expect that 16- and 32-bit fields in MCDI requests and responses
 * are appropriately aligned, but 64-bit fields are only
 * 32-bit-aligned.
 */
#[macro_export]
macro_rules! MCDI_DECLARE_BUF { ($name:ident, $len:expr) => {
    let mut $name: [cdx_dword; (($len + 3) / 4)] = [cdx_dword::default(); (($len + 3) / 4)];
}; }

#[macro_export]
macro_rules! _MCDI_PTR { ($buf:expr, $offset:expr) => {
    ($buf as *mut u8).wrapping_add($offset)
}; }

#[macro_export]
macro_rules! MCDI_PTR { ($buf:expr, $field:ident) => {
    _MCDI_PTR!($buf, MC_CMD_$field##_OFST)
}; }

#[macro_export]
macro_rules! _MCDI_CHECK_ALIGN { ($ofst:expr, $align:expr) => {
    $ofst
}; }

#[macro_export]
macro_rules! _MCDI_DWORD { ($buf:expr, $field:ident) => {
    ($buf).wrapping_add((_MCDI_CHECK_ALIGN!(MC_CMD_$field##_OFST, 4) >> 2))
}; }

#[macro_export]
macro_rules! MCDI_SET_DWORD { ($buf:expr, $field:ident, $value:expr) => {
    CDX_POPULATE_DWORD_1!(*_MCDI_DWORD!($buf, $field), CDX_DWORD, $value)
}; }

#[macro_export]
macro_rules! MCDI_DWORD { ($buf:expr, $field:ident) => {
    CDX_DWORD_FIELD!(*_MCDI_DWORD!($buf, $field), CDX_DWORD)
}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
