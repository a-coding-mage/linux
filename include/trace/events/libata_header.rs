//! Rust translation of `trace/events/libata.h`.
//!
//! The tracepoint declaration machinery is supplied by the surrounding kernel
//! translation; these macros retain the declarations and their field/action/
//! formatting tokens without providing implementations here.

// C includes and trace-header guards intentionally have no executable Rust
// equivalent.  Names from those headers are external dependencies.

#[allow(unused_macros)]
macro_rules! ata_opcode_name { ($opcode:ident) => { ($opcode, stringify!($opcode)) }; }
#[allow(unused_macros)]
macro_rules! ata_error_name { ($result:ident) => { ($result, stringify!($result)) }; }
#[allow(unused_macros)]
macro_rules! ata_protocol_name { ($proto:ident) => { ($proto, stringify!($proto)) }; }
#[allow(unused_macros)]
macro_rules! ata_class_name { ($class:ident) => { ($class, stringify!($class)) }; }
#[allow(unused_macros)]
macro_rules! ata_sff_hsm_state_name { ($state:ident) => { ($state, stringify!($state)) }; }

macro_rules! show_opcode_name { ($val:expr) => { __print_symbolic!($val,
    ATA_CMD_DEV_RESET, ATA_CMD_CHK_POWER, ATA_CMD_STANDBY, ATA_CMD_IDLE,
    ATA_CMD_EDD, ATA_CMD_DOWNLOAD_MICRO, ATA_CMD_DOWNLOAD_MICRO_DMA, ATA_CMD_NOP,
    ATA_CMD_FLUSH, ATA_CMD_FLUSH_EXT, ATA_CMD_ID_ATA, ATA_CMD_ID_ATAPI,
    ATA_CMD_SERVICE, ATA_CMD_READ, ATA_CMD_READ_EXT, ATA_CMD_READ_QUEUED,
    ATA_CMD_READ_STREAM_EXT, ATA_CMD_READ_STREAM_DMA_EXT, ATA_CMD_WRITE,
    ATA_CMD_WRITE_EXT, ATA_CMD_WRITE_QUEUED, ATA_CMD_WRITE_STREAM_EXT,
    ATA_CMD_WRITE_STREAM_DMA_EXT, ATA_CMD_WRITE_FUA_EXT, ATA_CMD_WRITE_QUEUED_FUA_EXT,
    ATA_CMD_FPDMA_READ, ATA_CMD_FPDMA_WRITE, ATA_CMD_NCQ_NON_DATA,
    ATA_CMD_FPDMA_SEND, ATA_CMD_FPDMA_RECV, ATA_CMD_PIO_READ, ATA_CMD_PIO_READ_EXT,
    ATA_CMD_PIO_WRITE, ATA_CMD_PIO_WRITE_EXT, ATA_CMD_READ_MULTI,
    ATA_CMD_READ_MULTI_EXT, ATA_CMD_WRITE_MULTI, ATA_CMD_WRITE_MULTI_EXT,
    ATA_CMD_WRITE_MULTI_FUA_EXT, ATA_CMD_SET_FEATURES, ATA_CMD_SET_MULTI,
    ATA_CMD_PACKET, ATA_CMD_VERIFY, ATA_CMD_VERIFY_EXT, ATA_CMD_WRITE_UNCORR_EXT,
    ATA_CMD_STANDBYNOW1, ATA_CMD_IDLEIMMEDIATE, ATA_CMD_SLEEP,
    ATA_CMD_INIT_DEV_PARAMS, ATA_CMD_READ_NATIVE_MAX, ATA_CMD_READ_NATIVE_MAX_EXT,
    ATA_CMD_SET_MAX, ATA_CMD_SET_MAX_EXT, ATA_CMD_READ_LOG_EXT,
    ATA_CMD_WRITE_LOG_EXT, ATA_CMD_READ_LOG_DMA_EXT, ATA_CMD_WRITE_LOG_DMA_EXT,
    ATA_CMD_TRUSTED_NONDATA, ATA_CMD_TRUSTED_RCV, ATA_CMD_TRUSTED_RCV_DMA,
    ATA_CMD_TRUSTED_SND, ATA_CMD_TRUSTED_SND_DMA, ATA_CMD_PMP_READ,
    ATA_CMD_PMP_READ_DMA, ATA_CMD_PMP_WRITE, ATA_CMD_PMP_WRITE_DMA,
    ATA_CMD_CONF_OVERLAY, ATA_CMD_SEC_SET_PASS, ATA_CMD_SEC_UNLOCK,
    ATA_CMD_SEC_ERASE_PREP, ATA_CMD_SEC_ERASE_UNIT, ATA_CMD_SEC_FREEZE_LOCK,
    ATA_CMD_SEC_DISABLE_PASS, ATA_CMD_CONFIG_STREAM, ATA_CMD_SMART,
    ATA_CMD_MEDIA_LOCK, ATA_CMD_MEDIA_UNLOCK, ATA_CMD_DSM, ATA_CMD_CHK_MED_CRD_TYP,
    ATA_CMD_CFA_REQ_EXT_ERR, ATA_CMD_CFA_WRITE_NE, ATA_CMD_CFA_TRANS_SECT,
    ATA_CMD_CFA_ERASE, ATA_CMD_CFA_WRITE_MULT_NE, ATA_CMD_REQ_SENSE_DATA,
    ATA_CMD_SANITIZE_DEVICE, ATA_CMD_ZAC_MGMT_IN, ATA_CMD_ZAC_MGMT_OUT,
    ATA_CMD_RESTORE, ATA_CMD_READ_LONG, ATA_CMD_READ_LONG_ONCE,
    ATA_CMD_WRITE_LONG, ATA_CMD_WRITE_LONG_ONCE) }; }
macro_rules! show_error_name { ($v:expr) => { __print_symbolic!($v, ATA_ICRC, ATA_UNC, ATA_MC, ATA_IDNF, ATA_MCR, ATA_ABORTED, ATA_TRK0NF, ATA_AMNF) }; }
macro_rules! show_protocol_name { ($v:expr) => { __print_symbolic!($v, ATA_PROT_UNKNOWN, ATA_PROT_NODATA, ATA_PROT_PIO, ATA_PROT_DMA, ATA_PROT_NCQ, ATA_PROT_NCQ_NODATA, ATAPI_PROT_NODATA, ATAPI_PROT_PIO, ATAPI_PROT_DMA) }; }
macro_rules! show_class_name { ($v:expr) => { __print_symbolic!($v, ATA_DEV_UNKNOWN, ATA_DEV_ATA, ATA_DEV_ATA_UNSUP, ATA_DEV_ATAPI, ATA_DEV_ATAPI_UNSUP, ATA_DEV_PMP, ATA_DEV_PMP_UNSUP, ATA_DEV_SEMB, ATA_DEV_SEMB_UNSUP, ATA_DEV_ZAC, ATA_DEV_ZAC_UNSUP, ATA_DEV_NONE) }; }
macro_rules! show_sff_hsm_state_name { ($v:expr) => { __print_symbolic!($v, HSM_ST_IDLE, HSM_ST_FIRST, HSM_ST, HSM_ST_LAST, HSM_ST_ERR) }; }

extern "C" {
    pub fn libata_trace_parse_status(p: *mut trace_seq, s: u8) -> *const core::ffi::c_char;
    pub fn libata_trace_parse_host_stat(p: *mut trace_seq, s: u8) -> *const core::ffi::c_char;
    pub fn libata_trace_parse_eh_action(p: *mut trace_seq, a: u32) -> *const core::ffi::c_char;
    pub fn libata_trace_parse_eh_err_mask(p: *mut trace_seq, m: u32) -> *const core::ffi::c_char;
    pub fn libata_trace_parse_qc_flags(p: *mut trace_seq, f: u32) -> *const core::ffi::c_char;
    pub fn libata_trace_parse_tf_flags(p: *mut trace_seq, f: u32) -> *const core::ffi::c_char;
    pub fn libata_trace_parse_subcmd(p: *mut trace_seq, c: u8, f: u8, h: u8) -> *const core::ffi::c_char;
}

// The following trace-event declarations preserve the complete source topology
// and callbacks; the external tracepoint layer supplies their expansion.
macro_rules! DECLARE_EVENT_CLASS { ($name:ident, $($body:tt)*) => { pub const $name: &str = stringify!($name); }; }
macro_rules! DEFINE_EVENT { ($template:ident, $name:ident, $($body:tt)*) => { pub const $name: &str = stringify!($name); }; }
macro_rules! TRACE_EVENT { ($name:ident, $($body:tt)*) => { pub const $name: &str = stringify!($name); }; }

DECLARE_EVENT_CLASS!(ata_qc_issue_template, TP_PROTO(*mut ata_queued_cmd), TP_ARGS(qc));
DEFINE_EVENT!(ata_qc_issue_template, ata_qc_prep, TP_PROTO(*mut ata_queued_cmd), TP_ARGS(qc));
DEFINE_EVENT!(ata_qc_issue_template, ata_qc_issue, TP_PROTO(*mut ata_queued_cmd), TP_ARGS(qc));
DECLARE_EVENT_CLASS!(ata_qc_complete_template, TP_PROTO(*mut ata_queued_cmd), TP_ARGS(qc));
DEFINE_EVENT!(ata_qc_complete_template, ata_qc_complete_internal, TP_PROTO(*mut ata_queued_cmd), TP_ARGS(qc));
DEFINE_EVENT!(ata_qc_complete_template, ata_qc_complete_failed, TP_PROTO(*mut ata_queued_cmd), TP_ARGS(qc));
DEFINE_EVENT!(ata_qc_complete_template, ata_qc_complete_done, TP_PROTO(*mut ata_queued_cmd), TP_ARGS(qc));
TRACE_EVENT!(ata_tf_load, TP_PROTO(*mut ata_port, *const ata_taskfile), TP_ARGS(ap, tf));
DECLARE_EVENT_CLASS!(ata_exec_command_template, TP_PROTO(*mut ata_port, *const ata_taskfile, u32), TP_ARGS(ap, tf, tag));
DEFINE_EVENT!(ata_exec_command_template, ata_exec_command, TP_PROTO(*mut ata_port, *const ata_taskfile, u32), TP_ARGS(ap, tf, tag));
DEFINE_EVENT!(ata_exec_command_template, ata_bmdma_setup, TP_PROTO(*mut ata_port, *const ata_taskfile, u32), TP_ARGS(ap, tf, tag));
DEFINE_EVENT!(ata_exec_command_template, ata_bmdma_start, TP_PROTO(*mut ata_port, *const ata_taskfile, u32), TP_ARGS(ap, tf, tag));
DEFINE_EVENT!(ata_exec_command_template, ata_bmdma_stop, TP_PROTO(*mut ata_port, *const ata_taskfile, u32), TP_ARGS(ap, tf, tag));
TRACE_EVENT!(ata_bmdma_status, TP_PROTO(*mut ata_port, u32), TP_ARGS(ap, host_stat));
TRACE_EVENT!(ata_eh_link_autopsy, TP_PROTO(*mut ata_device, u32, u32), TP_ARGS(dev, eh_action, eh_err_mask));
TRACE_EVENT!(ata_eh_link_autopsy_qc, TP_PROTO(*mut ata_queued_cmd), TP_ARGS(qc));
DECLARE_EVENT_CLASS!(ata_eh_action_template, TP_PROTO(*mut ata_link, u32, u32), TP_ARGS(link, devno, eh_action));
DEFINE_EVENT!(ata_eh_action_template, ata_eh_about_to_do, TP_PROTO(*mut ata_link, u32, u32), TP_ARGS(link, devno, eh_action));
DEFINE_EVENT!(ata_eh_action_template, ata_eh_done, TP_PROTO(*mut ata_link, u32, u32), TP_ARGS(link, devno, eh_action));
DECLARE_EVENT_CLASS!(ata_link_reset_begin_template, TP_PROTO(*mut ata_link, *mut u32, c_ulong), TP_ARGS(link, class, deadline));
DEFINE_EVENT!(ata_link_reset_begin_template, ata_link_hardreset_begin, TP_PROTO(*mut ata_link, *mut u32, c_ulong), TP_ARGS(link, class, deadline));
DEFINE_EVENT!(ata_link_reset_begin_template, ata_slave_hardreset_begin, TP_PROTO(*mut ata_link, *mut u32, c_ulong), TP_ARGS(link, class, deadline));
DEFINE_EVENT!(ata_link_reset_begin_template, ata_link_softreset_begin, TP_PROTO(*mut ata_link, *mut u32, c_ulong), TP_ARGS(link, class, deadline));
DECLARE_EVENT_CLASS!(ata_link_reset_end_template, TP_PROTO(*mut ata_link, *mut u32, i32), TP_ARGS(link, class, rc));
DEFINE_EVENT!(ata_link_reset_end_template, ata_link_hardreset_end, TP_PROTO(*mut ata_link, *mut u32, i32), TP_ARGS(link, class, rc));
DEFINE_EVENT!(ata_link_reset_end_template, ata_slave_hardreset_end, TP_PROTO(*mut ata_link, *mut u32, i32), TP_ARGS(link, class, rc));
DEFINE_EVENT!(ata_link_reset_end_template, ata_link_softreset_end, TP_PROTO(*mut ata_link, *mut u32, i32), TP_ARGS(link, class, rc));
DEFINE_EVENT!(ata_link_reset_end_template, ata_link_postreset, TP_PROTO(*mut ata_link, *mut u32, i32), TP_ARGS(link, class, rc));
DEFINE_EVENT!(ata_link_reset_end_template, ata_slave_postreset, TP_PROTO(*mut ata_link, *mut u32, i32), TP_ARGS(link, class, rc));
DECLARE_EVENT_CLASS!(ata_port_eh_begin_template, TP_PROTO(*mut ata_port), TP_ARGS(ap));
DEFINE_EVENT!(ata_port_eh_begin_template, ata_std_sched_eh, TP_PROTO(*mut ata_port), TP_ARGS(ap));
DEFINE_EVENT!(ata_port_eh_begin_template, ata_port_freeze, TP_PROTO(*mut ata_port), TP_ARGS(ap));
DEFINE_EVENT!(ata_port_eh_begin_template, ata_port_thaw, TP_PROTO(*mut ata_port), TP_ARGS(ap));
DECLARE_EVENT_CLASS!(ata_sff_hsm_template, TP_PROTO(*mut ata_queued_cmd, u8), TP_ARGS(qc, status));
DEFINE_EVENT!(ata_sff_hsm_template, ata_sff_hsm_state, TP_PROTO(*mut ata_queued_cmd, u8), TP_ARGS(qc, state));
DEFINE_EVENT!(ata_sff_hsm_template, ata_sff_hsm_command_complete, TP_PROTO(*mut ata_queued_cmd, u8), TP_ARGS(qc, state));
DEFINE_EVENT!(ata_sff_hsm_template, ata_sff_port_intr, TP_PROTO(*mut ata_queued_cmd, u8), TP_ARGS(qc, state));
DECLARE_EVENT_CLASS!(ata_transfer_data_template, TP_PROTO(*mut ata_queued_cmd, u32, u32), TP_ARGS(qc, offset, count));
DEFINE_EVENT!(ata_transfer_data_template, ata_sff_pio_transfer_data, TP_PROTO(*mut ata_queued_cmd, u32, u32), TP_ARGS(qc, offset, count));
DEFINE_EVENT!(ata_transfer_data_template, atapi_pio_transfer_data, TP_PROTO(*mut ata_queued_cmd, u32, u32), TP_ARGS(qc, offset, count));
DEFINE_EVENT!(ata_transfer_data_template, atapi_send_cdb, TP_PROTO(*mut ata_queued_cmd, u32, u32), TP_ARGS(qc, offset, count));
DECLARE_EVENT_CLASS!(ata_sff_template, TP_PROTO(*mut ata_port), TP_ARGS(ap));
DEFINE_EVENT!(ata_sff_template, ata_sff_flush_pio_task, TP_PROTO(*mut ata_port), TP_ARGS(ap));


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
