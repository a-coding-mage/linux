/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright(C) 2015 Linaro Limited. All rights reserved.
 * Author: Mathieu Poirier <mathieu.poirier@linaro.org>
 */

// C header dependencies: "debug.h", "util/event.h", <linux/bits.h>.

pub type pid_t = i32;

#[repr(C)]
pub enum perf_session {}

#[repr(C)]
pub enum perf_pmu {}

#[repr(C)]
pub enum perf_event {}

#[repr(C)]
pub enum perf_event_attr {}

/*
 * Versioning header in case things need to change in the future.  That way
 * decoding of old snapshot is still possible.
 */
pub const CS_HEADER_VERSION: u32 = 0;
pub const CS_PMU_TYPE_CPUS: u32 = 1;
pub const CS_ETM_SNAPSHOT: u32 = 2;
pub const CS_HEADER_VERSION_MAX: u32 = 3;

/*
 * Update the version for new format.
 *
 * Version 1: format adds a param count to the per cpu metadata.
 * This allows easy adding of new metadata parameters.
 * Requires that new params always added after current ones.
 * Also allows client reader to handle file versions that are different by
 * checking the number of params in the file vs the number expected.
 *
 * Version 2: Drivers will use PERF_RECORD_AUX_OUTPUT_HW_ID to output
 * CoreSight Trace ID. ...TRACEIDR metadata will be set to legacy values
 * but with addition flags.
 */
pub const CS_HEADER_CURRENT_VERSION: u32 = 2;

/* Beginning of header common to both ETMv3 and V4 */
pub const CS_ETM_MAGIC: u32 = 0;
pub const CS_ETM_CPU: u32 = 1;
/* Number of trace config params in following ETM specific block */
pub const CS_ETM_NR_TRC_PARAMS: u32 = 2;
pub const CS_ETM_COMMON_BLK_MAX_V1: u32 = 3;

/* ETMv3/PTM metadata */
/* Dynamic, configurable parameters */
pub const CS_ETM_ETMCR: u32 = CS_ETM_COMMON_BLK_MAX_V1;
pub const CS_ETM_ETMTRACEIDR: u32 = CS_ETM_ETMCR + 1;
/* RO, taken from sysFS */
pub const CS_ETM_ETMCCER: u32 = CS_ETM_ETMTRACEIDR + 1;
pub const CS_ETM_ETMIDR: u32 = CS_ETM_ETMCCER + 1;
pub const CS_ETM_PRIV_MAX: u32 = CS_ETM_ETMIDR + 1;

/* define fixed version 0 length - allow new format reader to read old files. */
pub const CS_ETM_NR_TRC_PARAMS_V0: u32 = CS_ETM_ETMIDR - CS_ETM_ETMCR + 1;

/* ETMv4 metadata */
/* Dynamic, configurable parameters */
pub const CS_ETMV4_TRCCONFIGR: u32 = CS_ETM_COMMON_BLK_MAX_V1;
pub const CS_ETMV4_TRCTRACEIDR: u32 = CS_ETMV4_TRCCONFIGR + 1;
/* RO, taken from sysFS */
pub const CS_ETMV4_TRCIDR0: u32 = CS_ETMV4_TRCTRACEIDR + 1;
pub const CS_ETMV4_TRCIDR1: u32 = CS_ETMV4_TRCIDR0 + 1;
pub const CS_ETMV4_TRCIDR2: u32 = CS_ETMV4_TRCIDR1 + 1;
pub const CS_ETMV4_TRCIDR8: u32 = CS_ETMV4_TRCIDR2 + 1;
pub const CS_ETMV4_TRCAUTHSTATUS: u32 = CS_ETMV4_TRCIDR8 + 1;
pub const CS_ETMV4_TS_SOURCE: u32 = CS_ETMV4_TRCAUTHSTATUS + 1;
pub const CS_ETMV4_PRIV_MAX: u32 = CS_ETMV4_TS_SOURCE + 1;

/* define fixed version 0 length - allow new format reader to read old files. */
pub const CS_ETMV4_NR_TRC_PARAMS_V0: u32 =
    CS_ETMV4_TRCAUTHSTATUS - CS_ETMV4_TRCCONFIGR + 1;

/*
 * ETE metadata is ETMv4 plus TRCDEVARCH register and doesn't support header V0 since it was
 * added in header V1
 */
/* Dynamic, configurable parameters */
pub const CS_ETE_TRCCONFIGR: u32 = CS_ETM_COMMON_BLK_MAX_V1;
pub const CS_ETE_TRCTRACEIDR: u32 = CS_ETE_TRCCONFIGR + 1;
/* RO, taken from sysFS */
pub const CS_ETE_TRCIDR0: u32 = CS_ETE_TRCTRACEIDR + 1;
pub const CS_ETE_TRCIDR1: u32 = CS_ETE_TRCIDR0 + 1;
pub const CS_ETE_TRCIDR2: u32 = CS_ETE_TRCIDR1 + 1;
pub const CS_ETE_TRCIDR8: u32 = CS_ETE_TRCIDR2 + 1;
pub const CS_ETE_TRCAUTHSTATUS: u32 = CS_ETE_TRCIDR8 + 1;
pub const CS_ETE_TRCDEVARCH: u32 = CS_ETE_TRCAUTHSTATUS + 1;
pub const CS_ETE_TS_SOURCE: u32 = CS_ETE_TRCDEVARCH + 1;
pub const CS_ETE_PRIV_MAX: u32 = CS_ETE_TS_SOURCE + 1;

/*
 * Check for valid CoreSight trace ID. If an invalid value is present in the metadata,
 * then IDs are present in the hardware ID packet in the data file.
 */
pub const fn CS_IS_VALID_TRACE_ID(id: u64) -> bool {
    (id > 0) && (id < 0x70)
}

/*
 * ETMv3 exception encoding number:
 * See Embedded Trace Macrocell specification (ARM IHI 0014Q)
 * table 7-12 Encoding of Exception[3:0] for non-ARMv7-M processors.
 */
pub const CS_ETMV3_EXC_NONE: u32 = 0;
pub const CS_ETMV3_EXC_DEBUG_HALT: u32 = 1;
pub const CS_ETMV3_EXC_SMC: u32 = 2;
pub const CS_ETMV3_EXC_HYP: u32 = 3;
pub const CS_ETMV3_EXC_ASYNC_DATA_ABORT: u32 = 4;
pub const CS_ETMV3_EXC_JAZELLE_THUMBEE: u32 = 5;
pub const CS_ETMV3_EXC_PE_RESET: u32 = 8;
pub const CS_ETMV3_EXC_UNDEFINED_INSTR: u32 = 9;
pub const CS_ETMV3_EXC_SVC: u32 = 10;
pub const CS_ETMV3_EXC_PREFETCH_ABORT: u32 = 11;
pub const CS_ETMV3_EXC_DATA_FAULT: u32 = 12;
pub const CS_ETMV3_EXC_GENERIC: u32 = 13;
pub const CS_ETMV3_EXC_IRQ: u32 = 14;
pub const CS_ETMV3_EXC_FIQ: u32 = 15;

/*
 * ETMv4 exception encoding number:
 * See ARM Embedded Trace Macrocell Architecture Specification (ARM IHI 0064D)
 * table 6-12 Possible values for the TYPE field in an Exception instruction
 * trace packet, for ARMv7-A/R and ARMv8-A/R PEs.
 */
pub const CS_ETMV4_EXC_RESET: u32 = 0;
pub const CS_ETMV4_EXC_DEBUG_HALT: u32 = 1;
pub const CS_ETMV4_EXC_CALL: u32 = 2;
pub const CS_ETMV4_EXC_TRAP: u32 = 3;
pub const CS_ETMV4_EXC_SYSTEM_ERROR: u32 = 4;
pub const CS_ETMV4_EXC_INST_DEBUG: u32 = 6;
pub const CS_ETMV4_EXC_DATA_DEBUG: u32 = 7;
pub const CS_ETMV4_EXC_ALIGNMENT: u32 = 10;
pub const CS_ETMV4_EXC_INST_FAULT: u32 = 11;
pub const CS_ETMV4_EXC_DATA_FAULT: u32 = 12;
pub const CS_ETMV4_EXC_IRQ: u32 = 14;
pub const CS_ETMV4_EXC_FIQ: u32 = 15;
pub const CS_ETMV4_EXC_END: u32 = 31;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum cs_etm_sample_type {
    CS_ETM_EMPTY,
    CS_ETM_RANGE,
    CS_ETM_DISCONTINUITY,
    CS_ETM_EXCEPTION,
    CS_ETM_EXCEPTION_RET,
    CS_ETM_CONTEXT,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum cs_etm_isa {
    CS_ETM_ISA_UNKNOWN,
    CS_ETM_ISA_A64,
    CS_ETM_ISA_A32,
    CS_ETM_ISA_T32,
}

#[repr(C)]
pub enum cs_etm_queue {}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct cs_etm_packet {
    pub sample_type: cs_etm_sample_type,
    pub isa: cs_etm_isa,
    pub start_addr: u64,
    pub end_addr: u64,
    pub instr_count: u32,
    pub last_instr_type: u32,
    pub last_instr_subtype: u32,
    pub flags: u32,
    pub exception_number: u32,
    pub last_instr_cond: bool,
    pub last_instr_taken_branch: bool,
    pub last_instr_size: u8,
    pub trace_chan_id: u8,
    pub cpu: i32,
    pub el: i32,
    pub tid: pid_t,
}

pub const CS_ETM_PACKET_MAX_BUFFER: usize = 1024;

/*
 * When working with per-thread scenarios the process under trace can
 * be scheduled on any CPU and as such, more than one traceID may be
 * associated with the same process.  Since a traceID of '0' is illegal
 * as per the CoreSight architecture, use that specific value to
 * identify the queue where all packets (with any traceID) are
 * aggregated.
 */
pub const CS_ETM_PER_THREAD_TRACEID: u32 = 0;

#[repr(C)]
pub struct cs_etm_packet_queue {
    pub packet_count: u32,
    pub head: u32,
    pub tail: u32,
    pub instr_count: u32,
    pub cs_timestamp: u64, /* Timestamp from trace data, converted to ns if possible */
    pub next_cs_timestamp: u64,
    pub packet_buffer: [cs_etm_packet; CS_ETM_PACKET_MAX_BUFFER],
}

pub const fn KiB(x: usize) -> usize {
    x * 1024
}

pub const fn MiB(x: usize) -> usize {
    x * 1024 * 1024
}

pub const CS_ETM_INVAL_ADDR: u64 = 0xdeadbeefdeadbeef_u64;

pub const fn BIT(nr: u32) -> u64 {
    1_u64 << nr
}

pub const fn GENMASK(msb: u32, lsb: u32) -> u64 {
    if msb == 63 {
        u64::MAX << lsb
    } else {
        ((1_u64 << (msb + 1)) - 1) & !(if lsb == 0 { 0 } else { (1_u64 << lsb) - 1 })
    }
}

pub const fn BMVAL(val: u64, lsb: u32, msb: u32) -> u64 {
    (val & GENMASK(msb, lsb)) >> lsb
}

pub const CS_ETM_HEADER_SIZE: usize = CS_HEADER_VERSION_MAX as usize * core::mem::size_of::<u64>();

pub const __perf_cs_etmv3_magic: u64 = 0x3030303030303030_u64;
pub const __perf_cs_etmv4_magic: u64 = 0x4040404040404040_u64;
pub const __perf_cs_ete_magic: u64 = 0x5050505050505050_u64;
pub const CS_ETMV3_PRIV_SIZE: usize = CS_ETM_PRIV_MAX as usize * core::mem::size_of::<u64>();
pub const CS_ETMV4_PRIV_SIZE: usize = CS_ETMV4_PRIV_MAX as usize * core::mem::size_of::<u64>();
pub const CS_ETE_PRIV_SIZE: usize = CS_ETE_PRIV_MAX as usize * core::mem::size_of::<u64>();

/*
 * INFO_HEADER_SIZE in C is:
 * sizeof(((struct perf_record_auxtrace_info *)0)->type) +
 * sizeof(((struct perf_record_auxtrace_info *)0)->reserved__)
 *
 * The field types are supplied by util/event.h and are intentionally not
 * recreated in this isolated header translation.
 */

/* CoreSight trace ID is currently the bottom 7 bits of the value */
pub const CORESIGHT_TRACE_ID_VAL_MASK: u64 = GENMASK(6, 0);

/* ETMv4 CONFIGR register bits */
pub const TRCCONFIGR_BB: u64 = BIT(3);
pub const TRCCONFIGR_CCI: u64 = BIT(4);
pub const TRCCONFIGR_CID: u64 = BIT(6);
pub const TRCCONFIGR_VMID: u64 = BIT(7);
pub const TRCCONFIGR_TS: u64 = BIT(11);
pub const TRCCONFIGR_RS: u64 = BIT(12);
pub const TRCCONFIGR_VMIDOPT: u64 = BIT(15);

/* ETMv3 ETMCR register bits */
pub const ETMCR_CYC_ACC: u64 = BIT(12);
pub const ETMCR_CTXTID: u64 = BIT(14);
pub const ETMCR_TIMESTAMP_EN: u64 = BIT(28);
pub const ETMCR_RETURN_STACK: u64 = BIT(29);

extern "C" {
    pub fn cs_etm__process_auxtrace_info(
        event: *mut perf_event,
        session: *mut perf_session,
    ) -> i32;
    pub fn cs_etm_get_default_config(pmu: *const perf_pmu, attr: *mut perf_event_attr);
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum cs_etm_pid_fmt {
    CS_ETM_PIDFMT_NONE,
    CS_ETM_PIDFMT_CTXTID,
    CS_ETM_PIDFMT_CTXTID2,
}

/*
 * HAVE_CSTRACE_SUPPORT conditional:
 * The OpenCSD-backed declarations below are available when C builds define
 * HAVE_CSTRACE_SUPPORT and include <opencsd/ocsd_if_types.h>.
 */
pub type ocsd_ex_level = i32;

extern "C" {
    pub fn cs_etm__get_cpu(
        etmq: *mut cs_etm_queue,
        trace_chan_id: u8,
        cpu: *mut i32,
    ) -> i32;
    pub fn cs_etm__get_pid_fmt(etmq: *mut cs_etm_queue) -> cs_etm_pid_fmt;
    pub fn cs_etm__etmq_update_decode_context(
        etmq: *mut cs_etm_queue,
        trace_chan_id: u8,
        el: ocsd_ex_level,
        tid: pid_t,
    ) -> i32;
    pub fn cs_etm__etmq_is_timeless(etmq: *mut cs_etm_queue) -> bool;
    pub fn cs_etm__etmq_set_traceid_queue_timestamp(
        etmq: *mut cs_etm_queue,
        trace_chan_id: u8,
    );
    pub fn cs_etm__etmq_get_packet_queue(
        etmq: *mut cs_etm_queue,
        trace_chan_id: u8,
    ) -> *mut cs_etm_packet_queue;
    pub fn cs_etm__convert_sample_time(etmq: *mut cs_etm_queue, cs_timestamp: u64) -> u64;
}

/*
 * Without HAVE_CSTRACE_SUPPORT, C provides a static inline fallback for
 * cs_etm__process_auxtrace_info_full() that prints:
 * "\nCS ETM Trace: OpenCSD is not linked in, please recompile with CORESIGHT=1\n"
 * and returns -1.
 */
pub unsafe fn cs_etm__process_auxtrace_info_full(
    _event: *mut perf_event,
    _session: *mut perf_session,
) -> i32 {
    // pr_err is supplied by debug.h in the original C header.
    extern "C" {
        fn pr_err(fmt: *const core::ffi::c_char, ...);
    }

    unsafe {
        pr_err(
            b"\nCS ETM Trace: OpenCSD is not linked in, please recompile with CORESIGHT=1\n\0"
                .as_ptr() as *const core::ffi::c_char,
        );
    }
    -1
}
