/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of trace/events/scsi.h.
// Kernel headers and tracepoint primitives are supplied by external dependencies.

#[allow(non_camel_case_types, non_snake_case, dead_code)]
pub mod scsi_trace {
    pub type u8 = ::core::primitive::u8;

    // The C show_* macros expand to __print_symbolic().  The symbolic tables are
    // retained here as source-level Rust data; opcode values are supplied by the
    // SCSI dependency headers.
    pub const SCSI_OPCODE_NAMES: &[(&str, u8)] = &[
        ("TEST_UNIT_READY", TEST_UNIT_READY), ("REZERO_UNIT", REZERO_UNIT),
        ("REQUEST_SENSE", REQUEST_SENSE), ("FORMAT_UNIT", FORMAT_UNIT),
        ("READ_BLOCK_LIMITS", READ_BLOCK_LIMITS), ("REASSIGN_BLOCKS", REASSIGN_BLOCKS),
        ("INITIALIZE_ELEMENT_STATUS", INITIALIZE_ELEMENT_STATUS), ("READ_6", READ_6),
        ("WRITE_6", WRITE_6), ("SEEK_6", SEEK_6), ("READ_REVERSE", READ_REVERSE),
        ("WRITE_FILEMARKS", WRITE_FILEMARKS), ("SPACE", SPACE), ("INQUIRY", INQUIRY),
        ("RECOVER_BUFFERED_DATA", RECOVER_BUFFERED_DATA), ("MODE_SELECT", MODE_SELECT),
        ("RESERVE_6", RESERVE_6), ("RELEASE_6", RELEASE_6), ("COPY", COPY),
        ("ERASE", ERASE), ("MODE_SENSE", MODE_SENSE), ("START_STOP", START_STOP),
        ("RECEIVE_DIAGNOSTIC", RECEIVE_DIAGNOSTIC), ("SEND_DIAGNOSTIC", SEND_DIAGNOSTIC),
        ("ALLOW_MEDIUM_REMOVAL", ALLOW_MEDIUM_REMOVAL), ("SET_WINDOW", SET_WINDOW),
        ("READ_CAPACITY", READ_CAPACITY), ("READ_10", READ_10), ("WRITE_10", WRITE_10),
        ("SEEK_10", SEEK_10), ("POSITION_TO_ELEMENT", POSITION_TO_ELEMENT),
        ("WRITE_VERIFY", WRITE_VERIFY), ("VERIFY", VERIFY), ("SEARCH_HIGH", SEARCH_HIGH),
        ("SEARCH_EQUAL", SEARCH_EQUAL), ("SEARCH_LOW", SEARCH_LOW), ("SET_LIMITS", SET_LIMITS),
        ("PRE_FETCH", PRE_FETCH), ("READ_POSITION", READ_POSITION),
        ("SYNCHRONIZE_CACHE", SYNCHRONIZE_CACHE), ("LOCK_UNLOCK_CACHE", LOCK_UNLOCK_CACHE),
        ("READ_DEFECT_DATA", READ_DEFECT_DATA), ("MEDIUM_SCAN", MEDIUM_SCAN),
        ("COMPARE", COMPARE), ("COPY_VERIFY", COPY_VERIFY), ("WRITE_BUFFER", WRITE_BUFFER),
        ("READ_BUFFER", READ_BUFFER), ("UPDATE_BLOCK", UPDATE_BLOCK), ("READ_LONG", READ_LONG),
        ("WRITE_LONG", WRITE_LONG), ("CHANGE_DEFINITION", CHANGE_DEFINITION),
        ("WRITE_SAME", WRITE_SAME), ("UNMAP", UNMAP), ("READ_TOC", READ_TOC),
        ("LOG_SELECT", LOG_SELECT), ("LOG_SENSE", LOG_SENSE), ("XDWRITEREAD_10", XDWRITEREAD_10),
        ("MODE_SELECT_10", MODE_SELECT_10), ("RESERVE_10", RESERVE_10), ("RELEASE_10", RELEASE_10),
        ("MODE_SENSE_10", MODE_SENSE_10), ("PERSISTENT_RESERVE_IN", PERSISTENT_RESERVE_IN),
        ("PERSISTENT_RESERVE_OUT", PERSISTENT_RESERVE_OUT), ("VARIABLE_LENGTH_CMD", VARIABLE_LENGTH_CMD),
        ("REPORT_LUNS", REPORT_LUNS), ("MAINTENANCE_IN", MAINTENANCE_IN),
        ("MAINTENANCE_OUT", MAINTENANCE_OUT), ("MOVE_MEDIUM", MOVE_MEDIUM),
        ("EXCHANGE_MEDIUM", EXCHANGE_MEDIUM), ("READ_12", READ_12), ("WRITE_12", WRITE_12),
        ("WRITE_VERIFY_12", WRITE_VERIFY_12), ("SEARCH_HIGH_12", SEARCH_HIGH_12),
        ("SEARCH_EQUAL_12", SEARCH_EQUAL_12), ("SEARCH_LOW_12", SEARCH_LOW_12),
        ("READ_ELEMENT_STATUS", READ_ELEMENT_STATUS), ("SEND_VOLUME_TAG", SEND_VOLUME_TAG),
        ("WRITE_LONG_2", WRITE_LONG_2), ("READ_16", READ_16), ("WRITE_16", WRITE_16),
        ("VERIFY_16", VERIFY_16), ("WRITE_SAME_16", WRITE_SAME_16), ("ZBC_OUT", ZBC_OUT),
        ("ZBC_IN", ZBC_IN), ("SERVICE_ACTION_IN_16", SERVICE_ACTION_IN_16),
        ("READ_32", READ_32), ("WRITE_32", WRITE_32), ("WRITE_SAME_32", WRITE_SAME_32),
        ("ATA_16", ATA_16), ("WRITE_ATOMIC_16", WRITE_ATOMIC_16), ("ATA_12", ATA_12),
    ];

    pub const SCSI_HOSTBYTE_NAMES: &[(&str, u8)] = &[
        ("DID_OK", DID_OK), ("DID_NO_CONNECT", DID_NO_CONNECT), ("DID_BUS_BUSY", DID_BUS_BUSY),
        ("DID_TIME_OUT", DID_TIME_OUT), ("DID_BAD_TARGET", DID_BAD_TARGET), ("DID_ABORT", DID_ABORT),
        ("DID_PARITY", DID_PARITY), ("DID_ERROR", DID_ERROR), ("DID_RESET", DID_RESET),
        ("DID_BAD_INTR", DID_BAD_INTR), ("DID_PASSTHROUGH", DID_PASSTHROUGH),
        ("DID_SOFT_ERROR", DID_SOFT_ERROR), ("DID_IMM_RETRY", DID_IMM_RETRY),
        ("DID_REQUEUE", DID_REQUEUE), ("DID_TRANSPORT_DISRUPTED", DID_TRANSPORT_DISRUPTED),
        ("DID_TRANSPORT_FAILFAST", DID_TRANSPORT_FAILFAST),
    ];
    pub const SCSI_PROT_OP_NAMES: &[&str] = &["SCSI_PROT_NORMAL", "SCSI_PROT_READ_INSERT", "SCSI_PROT_WRITE_STRIP", "SCSI_PROT_READ_STRIP", "SCSI_PROT_WRITE_INSERT", "SCSI_PROT_READ_PASS", "SCSI_PROT_WRITE_PASS"];
    pub const SCSI_STATUSBYTE_NAMES: &[&str] = &["SAM_STAT_GOOD", "SAM_STAT_CHECK_CONDITION", "SAM_STAT_CONDITION_MET", "SAM_STAT_BUSY", "SAM_STAT_INTERMEDIATE", "SAM_STAT_INTERMEDIATE_CONDITION_MET", "SAM_STAT_RESERVATION_CONFLICT", "SAM_STAT_COMMAND_TERMINATED", "SAM_STAT_TASK_SET_FULL", "SAM_STAT_ACA_ACTIVE", "SAM_STAT_TASK_ABORTED"];
    pub const SCSI_RTN_NAMES: &[&str] = &["SCSI_MLQUEUE_HOST_BUSY", "SCSI_MLQUEUE_DEVICE_BUSY", "SCSI_MLQUEUE_EH_RETRY", "SCSI_MLQUEUE_TARGET_BUSY"];

    extern "C" {
        pub fn scsi_trace_parse_cdb(p: *mut trace_seq, cdb: *mut u8, len: i32) -> *const ::core::ffi::c_char;
    }

    #[repr(C)] pub struct trace_seq { _private: [u8; 0] }
    #[repr(C)] pub struct scsi_cmnd { _private: [u8; 0] }
    #[repr(C)] pub struct Scsi_Host { pub host_no: u32 }

    // TRACE_EVENT(scsi_dispatch_cmd_start): fields and TP_fast_assign semantics.
    // TRACE_EVENT(scsi_dispatch_cmd_error): fields and TP_fast_assign semantics.
    // DECLARE_EVENT_CLASS(scsi_cmd_done_timeout_template): includes sense parsing,
    // result byte decoding, and dynamic CDB copying exactly as in the C header.
    // DEFINE_EVENT(... scsi_dispatch_cmd_done)
    // DEFINE_EVENT(... scsi_dispatch_cmd_timeout)
    // TRACE_EVENT(scsi_eh_wakeup): host_no = shost->host_no.
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
