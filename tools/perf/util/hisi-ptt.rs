// SPDX-License-Identifier: GPL-2.0
/*
 * HiSilicon PCIe Trace and Tuning (PTT) support
 * Copyright (c) 2022 HiSilicon Technologies Co., Ltd.
 */

use core::ffi::{c_char, c_int, c_uchar, c_void};

#[repr(C)]
pub struct hisi_ptt {
    pub auxtrace: auxtrace,
    pub auxtrace_type: u32,
    pub session: *mut perf_session,
    pub machine: *mut machine,
    pub pmu_type: u32,
}

extern "C" {
    static mut stdout: *mut FILE;
    static mut errno: c_int;
    static mut dump_trace: bool;

    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;
    fn readn(fd: c_int, buf: *mut c_void, n: size_t) -> ssize_t;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn color_fprintf(stream: *mut FILE, color: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn perf_data__fd(data: *mut perf_data) -> c_int;
    fn perf_data__is_pipe(data: *mut perf_data) -> bool;
    fn hisi_ptt_pkt_desc(buf: *mut c_uchar, pos: size_t, type_: hisi_ptt_pkt_type) -> c_int;
    fn zalloc(size: size_t) -> *mut c_void;
}

const SEEK_CUR: c_int = 1;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const SSIZE_MAX: u64 = isize::MAX as u64;
const PERF_COLOR_BLUE: *const c_char = b"blue\0".as_ptr() as *const c_char;

unsafe fn get_unaligned_le32(buf: *const c_uchar) -> u32 {
    u32::from_le(core::ptr::read_unaligned(buf as *const u32))
}

fn round_down(x: size_t, y: size_t) -> size_t {
    x / y * y
}

unsafe fn hisi_ptt_from_auxtrace(auxtrace_ptr: *mut auxtrace) -> *mut hisi_ptt {
    auxtrace_ptr as *mut hisi_ptt
}

unsafe fn hisi_ptt_check_packet_type(buf: *mut c_uchar, len: size_t) -> hisi_ptt_pkt_type {
    let head: u32;

    if len < HISI_PTT_FIELD_LENGTH as size_t {
        return hisi_ptt_pkt_type::HISI_PTT_4DW_PKT;
    }

    head = get_unaligned_le32(buf);

    if (HISI_PTT_8DW_CHECK_MASK & head) == HISI_PTT_IS_8DW_PKT {
        return hisi_ptt_pkt_type::HISI_PTT_8DW_PKT;
    }

    hisi_ptt_pkt_type::HISI_PTT_4DW_PKT
}

unsafe fn hisi_ptt_dump(_ptt: *mut hisi_ptt, buf: *mut c_uchar, mut len: size_t) {
    let color: *const c_char = PERF_COLOR_BLUE;
    let type_: hisi_ptt_pkt_type;
    let mut pos: size_t = 0;
    let mut pkt_len: c_int;

    type_ = hisi_ptt_check_packet_type(buf, len);
    len = round_down(len, hisi_ptt_pkt_size[type_ as usize] as size_t);
    color_fprintf(
        stdout,
        color,
        b". ... HISI PTT data: size %zu bytes\n\0".as_ptr() as *const c_char,
        len,
    );

    while len > 0 {
        pkt_len = hisi_ptt_pkt_desc(buf, pos, type_);
        if pkt_len == 0 {
            color_fprintf(
                stdout,
                color,
                b" Bad packet!\n\0".as_ptr() as *const c_char,
            );
        }

        pos = pos.wrapping_add(pkt_len as size_t);
        len = len.wrapping_sub(pkt_len as size_t);
    }
}

unsafe fn hisi_ptt_dump_event(ptt: *mut hisi_ptt, buf: *mut c_uchar, len: size_t) {
    printf(b".\n\0".as_ptr() as *const c_char);

    hisi_ptt_dump(ptt, buf, len);
}

unsafe extern "C" fn hisi_ptt_process_event(
    _session: *mut perf_session,
    _event: *mut perf_event,
    _sample: *mut perf_sample,
    _tool: *const perf_tool,
) -> c_int {
    0
}

unsafe extern "C" fn hisi_ptt_process_auxtrace_event(
    session: *mut perf_session,
    event: *mut perf_event,
    _tool: *const perf_tool,
) -> c_int {
    let ptt: *mut hisi_ptt = hisi_ptt_from_auxtrace((*session).auxtrace);
    let fd: c_int = perf_data__fd((*session).data);
    let size: u64 = (*event).auxtrace.size;
    let data_offset: off_t;
    let err: ssize_t;
    let data: *mut c_void;

    if size > SSIZE_MAX {
        return -EINVAL;
    }

    data = malloc(size as size_t);
    if data.is_null() {
        return -errno;
    }

    if perf_data__is_pipe((*session).data) {
        data_offset = 0;
    } else {
        data_offset = lseek(fd, 0, SEEK_CUR);
        if data_offset == -1 {
            free(data);
            return -errno;
        }
    }
    let _ = data_offset;

    err = readn(fd, data, size as size_t);
    if err != size as ssize_t {
        free(data);
        return -errno;
    }

    if dump_trace {
        hisi_ptt_dump_event(ptt, data as *mut c_uchar, size as size_t);
    }

    free(data);
    0
}

unsafe extern "C" fn hisi_ptt_flush(
    _session: *mut perf_session,
    _tool: *const perf_tool,
) -> c_int {
    0
}

unsafe extern "C" fn hisi_ptt_free_events(_session: *mut perf_session) {}

unsafe extern "C" fn hisi_ptt_free(session: *mut perf_session) {
    let ptt: *mut hisi_ptt = hisi_ptt_from_auxtrace((*session).auxtrace);

    (*session).auxtrace = core::ptr::null_mut();
    free(ptt as *mut c_void);
}

unsafe extern "C" fn hisi_ptt_evsel_is_auxtrace(
    session: *mut perf_session,
    evsel: *mut evsel,
) -> bool {
    let ptt: *mut hisi_ptt = hisi_ptt_from_auxtrace((*session).auxtrace);

    (*evsel).core.attr.type_ == (*ptt).pmu_type
}

unsafe fn hisi_ptt_print_info(type_: u64) {
    if !dump_trace {
        return;
    }

    fprintf(
        stdout,
        b"  PMU Type           %ld\n\0".as_ptr() as *const c_char,
        type_ as i64,
    );
}

#[no_mangle]
pub unsafe extern "C" fn hisi_ptt_process_auxtrace_info(
    event: *mut perf_event,
    session: *mut perf_session,
) -> c_int {
    let auxtrace_info: *mut perf_record_auxtrace_info = &mut (*event).auxtrace_info;
    let ptt: *mut hisi_ptt;

    if (*auxtrace_info).header.size
        < (HISI_PTT_AUXTRACE_PRIV_SIZE as size_t
            + core::mem::size_of::<perf_record_auxtrace_info>()) as u32
    {
        return -EINVAL;
    }

    ptt = zalloc(core::mem::size_of::<hisi_ptt>()) as *mut hisi_ptt;
    if ptt.is_null() {
        return -ENOMEM;
    }

    (*ptt).session = session;
    (*ptt).machine = &mut (*session).machines.host;
    (*ptt).auxtrace_type = (*auxtrace_info).type_;
    (*ptt).pmu_type = (*auxtrace_info).priv_[0] as u32;

    (*ptt).auxtrace.process_event = Some(hisi_ptt_process_event);
    (*ptt).auxtrace.process_auxtrace_event = Some(hisi_ptt_process_auxtrace_event);
    (*ptt).auxtrace.flush_events = Some(hisi_ptt_flush);
    (*ptt).auxtrace.free_events = Some(hisi_ptt_free_events);
    (*ptt).auxtrace.free = Some(hisi_ptt_free);
    (*ptt).auxtrace.evsel_is_auxtrace = Some(hisi_ptt_evsel_is_auxtrace);
    (*session).auxtrace = &mut (*ptt).auxtrace;

    hisi_ptt_print_info((*auxtrace_info).priv_[0]);

    0
}
