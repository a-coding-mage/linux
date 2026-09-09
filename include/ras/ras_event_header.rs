/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of ras_event.h.  Kernel-provided types and helpers remain
// external dependencies, as they are in the original tracepoint header.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

pub const APEIL: &str = "ARM Processor Err Info data len";
pub const APEID: &str = "ARM Processor Err Info raw data";
pub const APECIL: &str = "ARM Processor Err Context Info data len";
pub const APECID: &str = "ARM Processor Err Context Info raw data";
pub const VSEIL: &str = "Vendor Specific Err Info data len";
pub const VSEID: &str = "Vendor Specific Err Info raw data";

pub const UUID_SIZE: usize = 16;

#[repr(C)]
pub struct guid_t {
    pub b: [u8; UUID_SIZE],
}

#[repr(C)]
pub struct cper_sec_mem_err;
#[repr(C)]
pub struct cper_sec_proc_arm;
#[repr(C)]
pub struct cper_mem_err_compact;
#[repr(C)]
pub struct pcie_tlp_log;

/*
 * The original TRACE_EVENT declarations describe externally generated kernel
 * tracepoints.  The following Rust declarations retain their call interfaces;
 * their implementations and tracepoint registration are supplied elsewhere.
 */
extern "C" {
    pub fn extlog_mem_event(
        mem: *mut cper_sec_mem_err,
        err_seq: u32,
        fru_id: *const guid_t,
        fru_text: *const core::ffi::c_char,
        sev: u8,
    );

    pub fn mc_event(
        err_type: core::ffi::c_uint,
        error_msg: *const core::ffi::c_char,
        label: *const core::ffi::c_char,
        error_count: core::ffi::c_int,
        mc_index: u8,
        top_layer: i8,
        mid_layer: i8,
        low_layer: i8,
        address: core::ffi::c_ulong,
        grain_bits: u8,
        syndrome: core::ffi::c_ulong,
        driver_detail: *const core::ffi::c_char,
    );

    pub fn arm_event(
        proc: *const cper_sec_proc_arm,
        pei_err: *const u8,
        pei_len: u32,
        ctx_err: *const u8,
        ctx_len: u32,
        oem: *const u8,
        oem_len: u32,
        sev: u8,
        cpu: core::ffi::c_int,
    );

    pub fn non_standard_event(
        sec_type: *const guid_t,
        fru_id: *const guid_t,
        fru_text: *const core::ffi::c_char,
        sev: u8,
        err: *const u8,
        len: u32,
    );

    #[cfg(feature = "CONFIG_PCIEAER")]
    pub fn aer_event(
        dev_name: *const core::ffi::c_char,
        status: u32,
        severity: u8,
        tlp_header_valid: u8,
        tlp: *mut pcie_tlp_log,
        bus_type: *const core::ffi::c_char,
    );
}

/* PCIe AER error-name tables from aer_correctable_errors and
 * aer_uncorrectable_errors.  PCI error constants are supplied externally. */
#[cfg(feature = "CONFIG_PCIEAER")]
pub const aer_correctable_errors: &[(&str, &str)] = &[
    ("PCI_ERR_COR_RCVR", "Receiver Error"),
    ("PCI_ERR_COR_BAD_TLP", "Bad TLP"),
    ("PCI_ERR_COR_BAD_DLLP", "Bad DLLP"),
    ("PCI_ERR_COR_REP_ROLL", "RELAY_NUM Rollover"),
    ("PCI_ERR_COR_REP_TIMER", "Replay Timer Timeout"),
    ("PCI_ERR_COR_ADV_NFAT", "Advisory Non-Fatal Error"),
    ("PCI_ERR_COR_INTERNAL", "Corrected Internal Error"),
    ("PCI_ERR_COR_LOG_OVER", "Header Log Overflow"),
];

#[cfg(feature = "CONFIG_PCIEAER")]
pub const aer_uncorrectable_errors: &[(&str, &str)] = &[
    ("PCI_ERR_UNC_UND", "Undefined"),
    ("PCI_ERR_UNC_DLP", "Data Link Protocol Error"),
    ("PCI_ERR_UNC_SURPDN", "Surprise Down Error"),
    ("PCI_ERR_UNC_POISON_TLP", "Poisoned TLP"),
    ("PCI_ERR_UNC_FCP", "Flow Control Protocol Error"),
    ("PCI_ERR_UNC_COMP_TIME", "Completion Timeout"),
    ("PCI_ERR_UNC_COMP_ABORT", "Completer Abort"),
    ("PCI_ERR_UNC_UNX_COMP", "Unexpected Completion"),
    ("PCI_ERR_UNC_RX_OVER", "Receiver Overflow"),
    ("PCI_ERR_UNC_MALF_TLP", "Malformed TLP"),
    ("PCI_ERR_UNC_ECRC", "ECRC Error"),
    ("PCI_ERR_UNC_UNSUP", "Unsupported Request Error"),
    ("PCI_ERR_UNC_ACSV", "ACS Violation"),
    ("PCI_ERR_UNC_INTN", "Uncorrectable Internal Error"),
    ("PCI_ERR_UNC_MCBTLP", "MC Blocked TLP"),
    ("PCI_ERR_UNC_ATOMEG", "AtomicOp Egress Blocked"),
    ("PCI_ERR_UNC_TLPPRE", "TLP Prefix Blocked Error"),
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
