// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * libata-trace.c - trace functions for libata
 *
 * Copyright 2015 Hannes Reinecke
 * Copyright 2015 SUSE Linux GmbH
 */

// External kernel types, trace helpers, and ATA constants are supplied by
// other translation units.
#[repr(C)]
pub struct trace_seq {
    _private: [u8; 0],
}

extern "C" {
    fn trace_seq_buffer_ptr(p: *mut trace_seq) -> *const core::ffi::c_char;
    fn trace_seq_printf(p: *mut trace_seq, fmt: *const core::ffi::c_char, ...);
    fn trace_seq_putc(p: *mut trace_seq, c: core::ffi::c_int);
}

pub unsafe fn libata_trace_parse_status(
    p: *mut trace_seq,
    status: u8,
) -> *const core::ffi::c_char {
    let ret = trace_seq_buffer_ptr(p);

    trace_seq_printf(p, c"{ ".as_ptr());
    if status & ATA_BUSY as u8 != 0 { trace_seq_printf(p, c"BUSY ".as_ptr()); }
    if status & ATA_DRDY as u8 != 0 { trace_seq_printf(p, c"DRDY ".as_ptr()); }
    if status & ATA_DF as u8 != 0 { trace_seq_printf(p, c"DF ".as_ptr()); }
    if status & ATA_DSC as u8 != 0 { trace_seq_printf(p, c"DSC ".as_ptr()); }
    if status & ATA_DRQ as u8 != 0 { trace_seq_printf(p, c"DRQ ".as_ptr()); }
    if status & ATA_CORR as u8 != 0 { trace_seq_printf(p, c"CORR ".as_ptr()); }
    if status & ATA_SENSE as u8 != 0 { trace_seq_printf(p, c"SENSE ".as_ptr()); }
    if status & ATA_ERR as u8 != 0 { trace_seq_printf(p, c"ERR ".as_ptr()); }
    trace_seq_putc(p, b'}' as i32);
    trace_seq_putc(p, 0);
    ret
}

pub unsafe fn libata_trace_parse_host_stat(p: *mut trace_seq, host_stat: u8) -> *const core::ffi::c_char {
    let ret = trace_seq_buffer_ptr(p);
    trace_seq_printf(p, c"{ ".as_ptr());
    if host_stat & ATA_DMA_INTR as u8 != 0 { trace_seq_printf(p, c"INTR ".as_ptr()); }
    if host_stat & ATA_DMA_ERR as u8 != 0 { trace_seq_printf(p, c"ERR ".as_ptr()); }
    if host_stat & ATA_DMA_ACTIVE as u8 != 0 { trace_seq_printf(p, c"ACTIVE ".as_ptr()); }
    trace_seq_putc(p, b'}' as i32);
    trace_seq_putc(p, 0);
    ret
}

pub unsafe fn libata_trace_parse_eh_action(p: *mut trace_seq, eh_action: u32) -> *const core::ffi::c_char {
    let ret = trace_seq_buffer_ptr(p);
    trace_seq_printf(p, c"%x".as_ptr(), eh_action);
    if eh_action != 0 {
        trace_seq_printf(p, c"{ ".as_ptr());
        if eh_action & ATA_EH_REVALIDATE != 0 { trace_seq_printf(p, c"REVALIDATE ".as_ptr()); }
        if eh_action & (ATA_EH_SOFTRESET | ATA_EH_HARDRESET) != 0 { trace_seq_printf(p, c"RESET ".as_ptr()); }
        else if eh_action & ATA_EH_SOFTRESET != 0 { trace_seq_printf(p, c"SOFTRESET ".as_ptr()); }
        else if eh_action & ATA_EH_HARDRESET != 0 { trace_seq_printf(p, c"HARDRESET ".as_ptr()); }
        if eh_action & ATA_EH_ENABLE_LINK != 0 { trace_seq_printf(p, c"ENABLE_LINK ".as_ptr()); }
        if eh_action & ATA_EH_PARK != 0 { trace_seq_printf(p, c"PARK ".as_ptr()); }
        trace_seq_putc(p, b'}' as i32);
    }
    trace_seq_putc(p, 0);
    ret
}

pub unsafe fn libata_trace_parse_eh_err_mask(p: *mut trace_seq, eh_err_mask: u32) -> *const core::ffi::c_char {
    let ret = trace_seq_buffer_ptr(p);
    trace_seq_printf(p, c"%x".as_ptr(), eh_err_mask);
    if eh_err_mask != 0 {
        trace_seq_printf(p, c"{ ".as_ptr());
        if eh_err_mask & AC_ERR_DEV != 0 { trace_seq_printf(p, c"DEV ".as_ptr()); }
        if eh_err_mask & AC_ERR_HSM != 0 { trace_seq_printf(p, c"HSM ".as_ptr()); }
        if eh_err_mask & AC_ERR_TIMEOUT != 0 { trace_seq_printf(p, c"TIMEOUT ".as_ptr()); }
        if eh_err_mask & AC_ERR_MEDIA != 0 { trace_seq_printf(p, c"MEDIA ".as_ptr()); }
        if eh_err_mask & AC_ERR_ATA_BUS != 0 { trace_seq_printf(p, c"ATA_BUS ".as_ptr()); }
        if eh_err_mask & AC_ERR_HOST_BUS != 0 { trace_seq_printf(p, c"HOST_BUS ".as_ptr()); }
        if eh_err_mask & AC_ERR_SYSTEM != 0 { trace_seq_printf(p, c"SYSTEM ".as_ptr()); }
        if eh_err_mask & AC_ERR_INVALID != 0 { trace_seq_printf(p, c"INVALID ".as_ptr()); }
        if eh_err_mask & AC_ERR_OTHER != 0 { trace_seq_printf(p, c"OTHER ".as_ptr()); }
        if eh_err_mask & AC_ERR_NODEV_HINT != 0 { trace_seq_printf(p, c"NODEV_HINT ".as_ptr()); }
        if eh_err_mask & AC_ERR_NCQ != 0 { trace_seq_printf(p, c"NCQ ".as_ptr()); }
        trace_seq_putc(p, b'}' as i32);
    }
    trace_seq_putc(p, 0);
    ret
}

pub unsafe fn libata_trace_parse_qc_flags(p: *mut trace_seq, qc_flags: u32) -> *const core::ffi::c_char {
    let ret = trace_seq_buffer_ptr(p);
    trace_seq_printf(p, c"%x".as_ptr(), qc_flags);
    if qc_flags != 0 {
        trace_seq_printf(p, c"{ ".as_ptr());
        if qc_flags & ATA_QCFLAG_ACTIVE != 0 { trace_seq_printf(p, c"ACTIVE ".as_ptr()); }
        if qc_flags & ATA_QCFLAG_DMAMAP != 0 { trace_seq_printf(p, c"DMAMAP ".as_ptr()); }
        if qc_flags & ATA_QCFLAG_IO != 0 { trace_seq_printf(p, c"IO ".as_ptr()); }
        if qc_flags & ATA_QCFLAG_RESULT_TF != 0 { trace_seq_printf(p, c"RESULT_TF ".as_ptr()); }
        if qc_flags & ATA_QCFLAG_CLEAR_EXCL != 0 { trace_seq_printf(p, c"CLEAR_EXCL ".as_ptr()); }
        if qc_flags & ATA_QCFLAG_QUIET != 0 { trace_seq_printf(p, c"QUIET ".as_ptr()); }
        if qc_flags & ATA_QCFLAG_RETRY != 0 { trace_seq_printf(p, c"RETRY ".as_ptr()); }
        if qc_flags & ATA_QCFLAG_EH != 0 { trace_seq_printf(p, c"FAILED ".as_ptr()); }
        if qc_flags & ATA_QCFLAG_SENSE_VALID != 0 { trace_seq_printf(p, c"SENSE_VALID ".as_ptr()); }
        if qc_flags & ATA_QCFLAG_EH_SCHEDULED != 0 { trace_seq_printf(p, c"EH_SCHEDULED ".as_ptr()); }
        trace_seq_putc(p, b'}' as i32);
    }
    trace_seq_putc(p, 0);
    ret
}

pub unsafe fn libata_trace_parse_tf_flags(p: *mut trace_seq, tf_flags: u32) -> *const core::ffi::c_char {
    let ret = trace_seq_buffer_ptr(p);
    trace_seq_printf(p, c"%x".as_ptr(), tf_flags);
    if tf_flags != 0 {
        trace_seq_printf(p, c"{ ".as_ptr());
        if tf_flags & ATA_TFLAG_LBA48 != 0 { trace_seq_printf(p, c"LBA48 ".as_ptr()); }
        if tf_flags & ATA_TFLAG_ISADDR != 0 { trace_seq_printf(p, c"ISADDR ".as_ptr()); }
        if tf_flags & ATA_TFLAG_DEVICE != 0 { trace_seq_printf(p, c"DEV ".as_ptr()); }
        if tf_flags & ATA_TFLAG_WRITE != 0 { trace_seq_printf(p, c"WRITE ".as_ptr()); }
        if tf_flags & ATA_TFLAG_LBA != 0 { trace_seq_printf(p, c"LBA ".as_ptr()); }
        if tf_flags & ATA_TFLAG_FUA != 0 { trace_seq_printf(p, c"FUA ".as_ptr()); }
        if tf_flags & ATA_TFLAG_POLLING != 0 { trace_seq_printf(p, c"POLL ".as_ptr()); }
        trace_seq_putc(p, b'}' as i32);
    }
    trace_seq_putc(p, 0);
    ret
}

pub unsafe fn libata_trace_parse_subcmd(p: *mut trace_seq, cmd: u8, feature: u8, hob_nsect: u8) -> *const core::ffi::c_char {
    let ret = trace_seq_buffer_ptr(p);
    match cmd {
        ATA_CMD_FPDMA_RECV => match hob_nsect & 0x5f {
            ATA_SUBCMD_FPDMA_RECV_RD_LOG_DMA_EXT => trace_seq_printf(p, c" READ_LOG_DMA_EXT".as_ptr()),
            ATA_SUBCMD_FPDMA_RECV_ZAC_MGMT_IN => trace_seq_printf(p, c" ZAC_MGMT_IN".as_ptr()),
            _ => {}
        },
        ATA_CMD_FPDMA_SEND => match hob_nsect & 0x5f {
            ATA_SUBCMD_FPDMA_SEND_WR_LOG_DMA_EXT => trace_seq_printf(p, c" WRITE_LOG_DMA_EXT".as_ptr()),
            ATA_SUBCMD_FPDMA_SEND_DSM => trace_seq_printf(p, c" DATASET_MANAGEMENT".as_ptr()),
            _ => {}
        },
        ATA_CMD_NCQ_NON_DATA => match feature {
            ATA_SUBCMD_NCQ_NON_DATA_ABORT_QUEUE => trace_seq_printf(p, c" ABORT_QUEUE".as_ptr()),
            ATA_SUBCMD_NCQ_NON_DATA_SET_FEATURES => trace_seq_printf(p, c" SET_FEATURES".as_ptr()),
            ATA_SUBCMD_NCQ_NON_DATA_ZERO_EXT => trace_seq_printf(p, c" ZERO_EXT".as_ptr()),
            ATA_SUBCMD_NCQ_NON_DATA_ZAC_MGMT_OUT => trace_seq_printf(p, c" ZAC_MGMT_OUT".as_ptr()),
            _ => {}
        },
        ATA_CMD_ZAC_MGMT_IN => if feature == ATA_SUBCMD_ZAC_MGMT_IN_REPORT_ZONES { trace_seq_printf(p, c" REPORT_ZONES".as_ptr()); },
        ATA_CMD_ZAC_MGMT_OUT => match feature {
            ATA_SUBCMD_ZAC_MGMT_OUT_CLOSE_ZONE => trace_seq_printf(p, c" CLOSE_ZONE".as_ptr()),
            ATA_SUBCMD_ZAC_MGMT_OUT_FINISH_ZONE => trace_seq_printf(p, c" FINISH_ZONE".as_ptr()),
            ATA_SUBCMD_ZAC_MGMT_OUT_OPEN_ZONE => trace_seq_printf(p, c" OPEN_ZONE".as_ptr()),
            ATA_SUBCMD_ZAC_MGMT_OUT_RESET_WRITE_POINTER => trace_seq_printf(p, c" RESET_WRITE_POINTER".as_ptr()),
            _ => {}
        },
        _ => {}
    }
    trace_seq_putc(p, 0);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
