// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright IBM Corp. 2024
 *
 * Author(s):
 *   Niklas Schnelle <schnelle@linux.ibm.com>
 */

// Dependencies supplied by the surrounding kernel translation.

pub const ZPCI_ERR_LOG_ID_KERNEL_REPORT: u64 = 0x4714;

#[repr(C, packed)]
pub struct ZpciReportErrorData {
    pub timestamp: u64,
    pub err_log_id: u64,
    pub log_data: [u8; 0],
}

pub const ZPCI_REPORT_SIZE: usize = PAGE_SIZE - core::mem::size_of::<ErrNotifySccb>();
pub const ZPCI_REPORT_DATA_SIZE: usize =
    ZPCI_REPORT_SIZE - core::mem::size_of::<ZpciReportErrorData>();

#[repr(C, packed)]
pub struct ZpciReportError {
    pub header: ZpciReportErrorHeader,
    pub data: ZpciReportErrorData,
}

unsafe fn zpci_state_str(state: PciChannelStateT) -> &'static [u8] {
    match state {
        PciChannelIoNormal => b"normal\0",
        PciChannelIoFrozen => b"frozen\0",
        PciChannelIoPermFailure => b"permanent-failure\0",
        _ => b"invalid\0",
    }
}

unsafe extern "C" fn debug_log_header_fn(
    _id: *mut DebugInfoT,
    _view: *mut DebugView,
    _area: i32,
    entry: *mut DebugEntryT,
    out_buf: *mut i8,
    out_buf_size: usize,
) -> i32 {
    let mut sec: c_ulong = (*entry).clock;
    let usec: c_ulong = sec % USEC_PER_SEC as c_ulong;
    sec /= USEC_PER_SEC as c_ulong;
    let except_str: *const i8 = if (*entry).exception {
        b"*\0".as_ptr() as *const i8
    } else {
        b"-\0".as_ptr() as *const i8
    };
    scnprintf(
        out_buf,
        out_buf_size,
        b"%011ld:%06lu %1u %1s %04u  \0".as_ptr() as *const i8,
        sec,
        usec,
        (*entry).level,
        except_str,
        (*entry).cpu,
    )
}

unsafe extern "C" fn debug_prolog_header(
    _id: *mut DebugInfoT,
    _view: *mut DebugView,
    out_buf: *mut i8,
    out_buf_size: usize,
) -> i32 {
    scnprintf(
        out_buf,
        out_buf_size,
        b"sec:usec level except cpu  msg\n\0".as_ptr() as *const i8,
    )
}

static mut debug_log_view: DebugView = DebugView {
    name: b"pci_msg_log\0".as_ptr() as *const i8,
    prolog: Some(debug_prolog_header),
    header: Some(debug_log_header_fn),
    format: Some(debug_sprintf_format_fn),
    detail: core::ptr::null_mut(),
    next: core::ptr::null_mut(),
};

/// Report the status of operations on a PCI device.
pub unsafe extern "C" fn zpci_report_status(
    zdev: *mut ZpciDev,
    operation: *const i8,
    status: *const i8,
) -> i32 {
    let mut report: *mut ZpciReportError;
    let mut driver: *mut PciDriver = core::ptr::null_mut();
    let mut pdev: *mut PciDev = core::ptr::null_mut();
    let mut buf: *mut i8;
    let mut end: *mut i8;
    let mut ret: i32;

    if zdev.is_null() || (*zdev).zbus.is_null() {
        return -ENODEV;
    }
    if prot_virt_guest {
        return -ENODATA;
    }

    report = get_zeroed_page(GFP_KERNEL) as *mut ZpciReportError;
    if report.is_null() {
        return -ENOMEM;
    }
    if !(*(*zdev).zbus).bus.is_null() {
        pdev = pci_get_slot((*(*zdev).zbus).bus, (*zdev).devfn);
    }
    if !pdev.is_null() {
        driver = to_pci_driver((*pdev).dev.driver);
    }

    buf = (*report).data.log_data.as_mut_ptr() as *mut i8;
    end = buf.add(ZPCI_REPORT_DATA_SIZE);
    buf = buf.add(snprintf(buf, end.offset_from(buf) as usize, b"report: %s\n\0".as_ptr() as *const i8, operation) as usize);
    buf = buf.add(snprintf(buf, end.offset_from(buf) as usize, b"status: %s\n\0".as_ptr() as *const i8, status) as usize);
    let state = if !pdev.is_null() { zpci_state_str((*pdev).error_state) } else { b"n/a\0" };
    buf = buf.add(snprintf(buf, end.offset_from(buf) as usize, b"state: %s\n\0".as_ptr() as *const i8, state.as_ptr()) as usize);
    let driver_name = if !driver.is_null() { (*driver).name } else { b"n/a\0".as_ptr() as *const i8 };
    buf = buf.add(snprintf(buf, end.offset_from(buf) as usize, b"driver: %s\n\0".as_ptr() as *const i8, driver_name) as usize);
    ret = debug_dump(pci_debug_msg_id, &mut debug_log_view, buf, end.offset_from(buf) as usize, true);
    if ret < 0 {
        pr_err(b"Reading PCI debug messages failed with code %d\n\0".as_ptr() as *const i8, ret);
    } else {
        buf = buf.add(ret as usize);
    }

    (*report).header.version = 1;
    (*report).header.action = SCLP_ERRNOTIFY_AQ_INFO_LOG;
    (*report).header.length = buf.offset_from(&mut (*report).data as *mut _ as *mut i8) as u16;
    (*report).data.timestamp = ktime_get_clocktai_seconds();
    (*report).data.err_log_id = ZPCI_ERR_LOG_ID_KERNEL_REPORT;
    ret = sclp_pci_report(&mut (*report).header, (*zdev).fh, (*zdev).fid);
    if ret != 0 { pr_err(b"Reporting PCI status failed with code %d\n\0".as_ptr() as *const i8, ret); }
    else { pr_info(b"Reported PCI device status\n\0".as_ptr() as *const i8); }
    free_page(report as usize);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
