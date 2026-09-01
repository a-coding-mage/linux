// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use std::ffi::{c_char, c_double, c_int, c_uint, c_ulonglong, c_void};
use std::ptr;

type __u64 = u64;

const IF_NAMESIZE: usize = 16;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netdev_qstats_scope {
    NETDEV_QSTATS_SCOPE_UNSPEC = 0,
    NETDEV_QSTATS_SCOPE_QUEUE = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netdev_queue_type {
    NETDEV_QUEUE_TYPE_UNSPEC = 0,
    NETDEV_QUEUE_TYPE_RX = 1,
    NETDEV_QUEUE_TYPE_TX = 2,
}

#[repr(C)]
pub struct queue_balance {
    ifindex: c_uint,
    type_: netdev_queue_type,
    queue_count: c_uint,
    rx_packets: *mut __u64,
    rx_bytes: *mut __u64,
    tx_packets: *mut __u64,
    tx_bytes: *mut __u64,
}

#[repr(C)]
pub struct netdev_qstats_get_rsp_present {
    queue_type: bool,
    queue_id: bool,
    rx_packets: bool,
    rx_bytes: bool,
    rx_alloc_fail: bool,
    rx_hw_drops: bool,
    rx_hw_drop_overruns: bool,
    rx_hw_drop_ratelimits: bool,
    rx_csum_complete: bool,
    rx_csum_unnecessary: bool,
    rx_csum_none: bool,
    rx_csum_bad: bool,
    rx_hw_gro_packets: bool,
    rx_hw_gro_bytes: bool,
    rx_hw_gro_wire_packets: bool,
    rx_hw_gro_wire_bytes: bool,
    tx_packets: bool,
    tx_bytes: bool,
    tx_hw_drops: bool,
    tx_hw_drop_errors: bool,
    tx_hw_drop_ratelimits: bool,
    tx_csum_none: bool,
    tx_needs_csum: bool,
    tx_hw_gso_packets: bool,
    tx_hw_gso_bytes: bool,
    tx_hw_gso_wire_packets: bool,
    tx_hw_gso_wire_bytes: bool,
    tx_stop: bool,
    tx_wake: bool,
}

#[repr(C)]
pub struct netdev_qstats_get_rsp {
    _present: netdev_qstats_get_rsp_present,
    ifindex: c_uint,
    queue_type: netdev_queue_type,
    queue_id: c_uint,
    rx_packets: __u64,
    rx_bytes: __u64,
    rx_alloc_fail: __u64,
    rx_hw_drops: __u64,
    rx_hw_drop_overruns: __u64,
    rx_hw_drop_ratelimits: __u64,
    rx_csum_complete: __u64,
    rx_csum_unnecessary: __u64,
    rx_csum_none: __u64,
    rx_csum_bad: __u64,
    rx_hw_gro_packets: __u64,
    rx_hw_gro_bytes: __u64,
    rx_hw_gro_wire_packets: __u64,
    rx_hw_gro_wire_bytes: __u64,
    tx_packets: __u64,
    tx_bytes: __u64,
    tx_hw_drops: __u64,
    tx_hw_drop_errors: __u64,
    tx_hw_drop_ratelimits: __u64,
    tx_csum_none: __u64,
    tx_needs_csum: __u64,
    tx_hw_gso_packets: __u64,
    tx_hw_gso_bytes: __u64,
    tx_hw_gso_wire_packets: __u64,
    tx_hw_gso_wire_bytes: __u64,
    tx_stop: __u64,
    tx_wake: __u64,
}

#[repr(C)]
pub struct netdev_qstats_get_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct netdev_qstats_get_req {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ynl_family {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ynl_error {
    msg: *const c_char,
}

#[repr(C)]
pub struct ynl_sock {
    err: ynl_error,
}

#[repr(C)]
pub struct json_writer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cmd {
    name: *const c_char,
    func: Option<unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int>,
}

unsafe extern "C" {
    static mut json_wtr: *mut json_writer;
    static mut json_output: bool;
    static mut ynl_netdev_family: ynl_family;
    static mut bin_name: *const c_char;
    static mut stderr: *mut c_void;

    fn printf(fmt: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn qsort(
        base: *mut c_void,
        nmemb: usize,
        size: usize,
        compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
    );
    fn sqrt(x: c_double) -> c_double;
    fn if_indextoname(ifindex: c_uint, ifname: *mut c_char) -> *const c_char;

    fn jsonw_start_array(w: *mut json_writer);
    fn jsonw_end_array(w: *mut json_writer);
    fn jsonw_start_object(w: *mut json_writer);
    fn jsonw_end_object(w: *mut json_writer);
    fn jsonw_name(w: *mut json_writer, name: *const c_char);
    fn jsonw_string_field(w: *mut json_writer, name: *const c_char, val: *const c_char);
    fn jsonw_uint_field(w: *mut json_writer, name: *const c_char, val: c_ulonglong);
    fn jsonw_float_field(w: *mut json_writer, name: *const c_char, val: c_double);
    fn jsonw_null(w: *mut json_writer);

    fn ynl_sock_create(family: *mut ynl_family, yerr: *mut ynl_error) -> *mut ynl_sock;
    fn ynl_sock_destroy(ys: *mut ynl_sock);
    fn ynl_dump_next(
        list: *mut netdev_qstats_get_list,
        prev: *mut netdev_qstats_get_rsp,
    ) -> *mut netdev_qstats_get_rsp;

    fn netdev_queue_type_str(type_: netdev_queue_type) -> *const c_char;
    fn netdev_qstats_get_req_alloc() -> *mut netdev_qstats_get_req;
    fn netdev_qstats_get_req_free(req: *mut netdev_qstats_get_req);
    fn netdev_qstats_get_req_set_scope(
        req: *mut netdev_qstats_get_req,
        scope: netdev_qstats_scope,
    );
    fn netdev_qstats_get_dump(
        ys: *mut ynl_sock,
        req: *mut netdev_qstats_get_req,
    ) -> *mut netdev_qstats_get_list;
    fn netdev_qstats_get_list_free(qstats: *mut netdev_qstats_get_list);

    fn p_err(fmt: *const c_char, ...);
    fn is_prefix(prefix: *const c_char, string: *const c_char) -> bool;
    fn cmd_select(
        cmds: *const cmd,
        argc: c_int,
        argv: *mut *mut c_char,
        help: Option<unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int>,
    ) -> c_int;
}

static mut scope: netdev_qstats_scope = netdev_qstats_scope::NETDEV_QSTATS_SCOPE_UNSPEC;

unsafe fn req_args(argc: c_int, required: c_int) -> bool {
    argc >= required
}

unsafe fn next_arg(argc: &mut c_int, argv: &mut *mut *mut c_char) {
    *argc -= 1;
    *argv = (*argv).add(1);
}

unsafe fn dump_foreach<F>(qstats: *mut netdev_qstats_get_list, mut f: F)
where
    F: FnMut(*mut netdev_qstats_get_rsp),
{
    let mut qs: *mut netdev_qstats_get_rsp = ptr::null_mut();
    loop {
        qs = ynl_dump_next(qstats, qs);
        if qs.is_null() {
            break;
        }
        f(qs);
    }
}

unsafe fn print_json_qstats(qstats: *mut netdev_qstats_get_list) {
    jsonw_start_array(json_wtr);

    dump_foreach(qstats, |qs| unsafe {
        let mut ifname = [0 as c_char; IF_NAMESIZE];
        let mut name: *const c_char;

        jsonw_start_object(json_wtr);

        name = if_indextoname((*qs).ifindex, ifname.as_mut_ptr());
        if !name.is_null() {
            jsonw_string_field(json_wtr, c"ifname".as_ptr(), name);
        }
        jsonw_uint_field(json_wtr, c"ifindex".as_ptr(), (*qs).ifindex as c_ulonglong);

        if (*qs)._present.queue_type {
            jsonw_string_field(
                json_wtr,
                c"queue-type".as_ptr(),
                netdev_queue_type_str((*qs).queue_type),
            );
        }
        if (*qs)._present.queue_id {
            jsonw_uint_field(json_wtr, c"queue-id".as_ptr(), (*qs).queue_id as c_ulonglong);
        }

        if (*qs)._present.rx_packets
            || (*qs)._present.rx_bytes
            || (*qs)._present.rx_alloc_fail
            || (*qs)._present.rx_hw_drops
            || (*qs)._present.rx_csum_complete
            || (*qs)._present.rx_hw_gro_packets
        {
            jsonw_name(json_wtr, c"rx".as_ptr());
            jsonw_start_object(json_wtr);
            if (*qs)._present.rx_packets {
                jsonw_uint_field(json_wtr, c"packets".as_ptr(), (*qs).rx_packets);
            }
            if (*qs)._present.rx_bytes {
                jsonw_uint_field(json_wtr, c"bytes".as_ptr(), (*qs).rx_bytes);
            }
            if (*qs)._present.rx_alloc_fail {
                jsonw_uint_field(json_wtr, c"alloc-fail".as_ptr(), (*qs).rx_alloc_fail);
            }
            if (*qs)._present.rx_hw_drops {
                jsonw_uint_field(json_wtr, c"hw-drops".as_ptr(), (*qs).rx_hw_drops);
            }
            if (*qs)._present.rx_hw_drop_overruns {
                jsonw_uint_field(json_wtr, c"hw-drop-overruns".as_ptr(), (*qs).rx_hw_drop_overruns);
            }
            if (*qs)._present.rx_hw_drop_ratelimits {
                jsonw_uint_field(json_wtr, c"hw-drop-ratelimits".as_ptr(), (*qs).rx_hw_drop_ratelimits);
            }
            if (*qs)._present.rx_csum_complete {
                jsonw_uint_field(json_wtr, c"csum-complete".as_ptr(), (*qs).rx_csum_complete);
            }
            if (*qs)._present.rx_csum_unnecessary {
                jsonw_uint_field(json_wtr, c"csum-unnecessary".as_ptr(), (*qs).rx_csum_unnecessary);
            }
            if (*qs)._present.rx_csum_none {
                jsonw_uint_field(json_wtr, c"csum-none".as_ptr(), (*qs).rx_csum_none);
            }
            if (*qs)._present.rx_csum_bad {
                jsonw_uint_field(json_wtr, c"csum-bad".as_ptr(), (*qs).rx_csum_bad);
            }
            if (*qs)._present.rx_hw_gro_packets {
                jsonw_uint_field(json_wtr, c"hw-gro-packets".as_ptr(), (*qs).rx_hw_gro_packets);
            }
            if (*qs)._present.rx_hw_gro_bytes {
                jsonw_uint_field(json_wtr, c"hw-gro-bytes".as_ptr(), (*qs).rx_hw_gro_bytes);
            }
            if (*qs)._present.rx_hw_gro_wire_packets {
                jsonw_uint_field(json_wtr, c"hw-gro-wire-packets".as_ptr(), (*qs).rx_hw_gro_wire_packets);
            }
            if (*qs)._present.rx_hw_gro_wire_bytes {
                jsonw_uint_field(json_wtr, c"hw-gro-wire-bytes".as_ptr(), (*qs).rx_hw_gro_wire_bytes);
            }
            jsonw_end_object(json_wtr);
        }

        if (*qs)._present.tx_packets
            || (*qs)._present.tx_bytes
            || (*qs)._present.tx_hw_drops
            || (*qs)._present.tx_csum_none
            || (*qs)._present.tx_hw_gso_packets
        {
            jsonw_name(json_wtr, c"tx".as_ptr());
            jsonw_start_object(json_wtr);
            if (*qs)._present.tx_packets {
                jsonw_uint_field(json_wtr, c"packets".as_ptr(), (*qs).tx_packets);
            }
            if (*qs)._present.tx_bytes {
                jsonw_uint_field(json_wtr, c"bytes".as_ptr(), (*qs).tx_bytes);
            }
            if (*qs)._present.tx_hw_drops {
                jsonw_uint_field(json_wtr, c"hw-drops".as_ptr(), (*qs).tx_hw_drops);
            }
            if (*qs)._present.tx_hw_drop_errors {
                jsonw_uint_field(json_wtr, c"hw-drop-errors".as_ptr(), (*qs).tx_hw_drop_errors);
            }
            if (*qs)._present.tx_hw_drop_ratelimits {
                jsonw_uint_field(json_wtr, c"hw-drop-ratelimits".as_ptr(), (*qs).tx_hw_drop_ratelimits);
            }
            if (*qs)._present.tx_csum_none {
                jsonw_uint_field(json_wtr, c"csum-none".as_ptr(), (*qs).tx_csum_none);
            }
            if (*qs)._present.tx_needs_csum {
                jsonw_uint_field(json_wtr, c"needs-csum".as_ptr(), (*qs).tx_needs_csum);
            }
            if (*qs)._present.tx_hw_gso_packets {
                jsonw_uint_field(json_wtr, c"hw-gso-packets".as_ptr(), (*qs).tx_hw_gso_packets);
            }
            if (*qs)._present.tx_hw_gso_bytes {
                jsonw_uint_field(json_wtr, c"hw-gso-bytes".as_ptr(), (*qs).tx_hw_gso_bytes);
            }
            if (*qs)._present.tx_hw_gso_wire_packets {
                jsonw_uint_field(json_wtr, c"hw-gso-wire-packets".as_ptr(), (*qs).tx_hw_gso_wire_packets);
            }
            if (*qs)._present.tx_hw_gso_wire_bytes {
                jsonw_uint_field(json_wtr, c"hw-gso-wire-bytes".as_ptr(), (*qs).tx_hw_gso_wire_bytes);
            }
            if (*qs)._present.tx_stop {
                jsonw_uint_field(json_wtr, c"stop".as_ptr(), (*qs).tx_stop);
            }
            if (*qs)._present.tx_wake {
                jsonw_uint_field(json_wtr, c"wake".as_ptr(), (*qs).tx_wake);
            }
            jsonw_end_object(json_wtr);
        }

        jsonw_end_object(json_wtr);
    });

    jsonw_end_array(json_wtr);
}

unsafe fn print_one(present: bool, mut name: *const c_char, val: c_ulonglong, line: *mut c_int) {
    if !present {
        return;
    }

    if *line == 0 {
        printf(c"              ".as_ptr());
        *line += 1;
    }

    /* Don't waste space on tx- and rx- prefix, its implied by queue type */
    if scope == netdev_qstats_scope::NETDEV_QSTATS_SCOPE_QUEUE
        && (*name == b'r' as c_char || *name == b't' as c_char)
        && *name.add(1) == b'x' as c_char
        && *name.add(2) == b'-' as c_char
    {
        name = name.add(3);
    }

    printf(c" %15s: %15llu".as_ptr(), name, val);

    *line += 1;
    if *line == 3 {
        printf(c"\n".as_ptr());
        *line = 0;
    }
}

unsafe fn print_plain_qstats(qstats: *mut netdev_qstats_get_list) {
    dump_foreach(qstats, |qs| unsafe {
        let mut ifname = [0 as c_char; IF_NAMESIZE];
        let name: *const c_char;
        let mut n: c_int;

        name = if_indextoname((*qs).ifindex, ifname.as_mut_ptr());
        if !name.is_null() {
            printf(c"%s".as_ptr(), name);
        } else {
            printf(c"ifindex:%u".as_ptr(), (*qs).ifindex);
        }

        if (*qs)._present.queue_type && (*qs)._present.queue_id {
            printf(
                c"\t%s-%-3u".as_ptr(),
                netdev_queue_type_str((*qs).queue_type),
                (*qs).queue_id,
            );
        } else {
            printf(c"\t      ".as_ptr());
        }

        n = 1;

        /* Basic counters */
        print_one((*qs)._present.rx_packets, c"rx-packets".as_ptr(), (*qs).rx_packets, &mut n);
        print_one((*qs)._present.rx_bytes, c"rx-bytes".as_ptr(), (*qs).rx_bytes, &mut n);
        print_one((*qs)._present.tx_packets, c"tx-packets".as_ptr(), (*qs).tx_packets, &mut n);
        print_one((*qs)._present.tx_bytes, c"tx-bytes".as_ptr(), (*qs).tx_bytes, &mut n);

        /* RX error/drop counters */
        print_one((*qs)._present.rx_alloc_fail, c"rx-alloc-fail".as_ptr(), (*qs).rx_alloc_fail, &mut n);
        print_one((*qs)._present.rx_hw_drops, c"rx-hw-drops".as_ptr(), (*qs).rx_hw_drops, &mut n);
        print_one((*qs)._present.rx_hw_drop_overruns, c"rx-hw-drop-overruns".as_ptr(), (*qs).rx_hw_drop_overruns, &mut n);
        print_one((*qs)._present.rx_hw_drop_ratelimits, c"rx-hw-drop-ratelimits".as_ptr(), (*qs).rx_hw_drop_ratelimits, &mut n);

        /* RX checksum counters */
        print_one((*qs)._present.rx_csum_complete, c"rx-csum-complete".as_ptr(), (*qs).rx_csum_complete, &mut n);
        print_one((*qs)._present.rx_csum_unnecessary, c"rx-csum-unnecessary".as_ptr(), (*qs).rx_csum_unnecessary, &mut n);
        print_one((*qs)._present.rx_csum_none, c"rx-csum-none".as_ptr(), (*qs).rx_csum_none, &mut n);
        print_one((*qs)._present.rx_csum_bad, c"rx-csum-bad".as_ptr(), (*qs).rx_csum_bad, &mut n);

        /* RX GRO counters */
        print_one((*qs)._present.rx_hw_gro_packets, c"rx-hw-gro-packets".as_ptr(), (*qs).rx_hw_gro_packets, &mut n);
        print_one((*qs)._present.rx_hw_gro_bytes, c"rx-hw-gro-bytes".as_ptr(), (*qs).rx_hw_gro_bytes, &mut n);
        print_one((*qs)._present.rx_hw_gro_wire_packets, c"rx-hw-gro-wire-packets".as_ptr(), (*qs).rx_hw_gro_wire_packets, &mut n);
        print_one((*qs)._present.rx_hw_gro_wire_bytes, c"rx-hw-gro-wire-bytes".as_ptr(), (*qs).rx_hw_gro_wire_bytes, &mut n);

        /* TX error/drop counters */
        print_one((*qs)._present.tx_hw_drops, c"tx-hw-drops".as_ptr(), (*qs).tx_hw_drops, &mut n);
        print_one((*qs)._present.tx_hw_drop_errors, c"tx-hw-drop-errors".as_ptr(), (*qs).tx_hw_drop_errors, &mut n);
        print_one((*qs)._present.tx_hw_drop_ratelimits, c"tx-hw-drop-ratelimits".as_ptr(), (*qs).tx_hw_drop_ratelimits, &mut n);

        /* TX checksum counters */
        print_one((*qs)._present.tx_csum_none, c"tx-csum-none".as_ptr(), (*qs).tx_csum_none, &mut n);
        print_one((*qs)._present.tx_needs_csum, c"tx-needs-csum".as_ptr(), (*qs).tx_needs_csum, &mut n);

        /* TX GSO counters */
        print_one((*qs)._present.tx_hw_gso_packets, c"tx-hw-gso-packets".as_ptr(), (*qs).tx_hw_gso_packets, &mut n);
        print_one((*qs)._present.tx_hw_gso_bytes, c"tx-hw-gso-bytes".as_ptr(), (*qs).tx_hw_gso_bytes, &mut n);
        print_one((*qs)._present.tx_hw_gso_wire_packets, c"tx-hw-gso-wire-packets".as_ptr(), (*qs).tx_hw_gso_wire_packets, &mut n);
        print_one((*qs)._present.tx_hw_gso_wire_bytes, c"tx-hw-gso-wire-bytes".as_ptr(), (*qs).tx_hw_gso_wire_bytes, &mut n);

        /* TX queue control */
        print_one((*qs)._present.tx_stop, c"tx-stop".as_ptr(), (*qs).tx_stop, &mut n);
        print_one((*qs)._present.tx_wake, c"tx-wake".as_ptr(), (*qs).tx_wake, &mut n);

        if n != 0 {
            printf(c"\n".as_ptr());
        }
    });
}

unsafe fn qstats_dump(scope_arg: netdev_qstats_scope) -> *mut netdev_qstats_get_list {
    let qstats: *mut netdev_qstats_get_list;
    let req: *mut netdev_qstats_get_req;
    let mut yerr = ynl_error { msg: ptr::null() };
    let ys: *mut ynl_sock;

    ys = ynl_sock_create(&mut ynl_netdev_family, &mut yerr);
    if ys.is_null() {
        p_err(c"YNL: %s".as_ptr(), yerr.msg);
        return ptr::null_mut();
    }

    req = netdev_qstats_get_req_alloc();
    if req.is_null() {
        p_err(c"failed to allocate qstats request".as_ptr());
        ynl_sock_destroy(ys);
        return ptr::null_mut();
    }

    if scope_arg as c_int != 0 {
        netdev_qstats_get_req_set_scope(req, scope_arg);
    }

    qstats = netdev_qstats_get_dump(ys, req);
    netdev_qstats_get_req_free(req);
    if qstats.is_null() {
        p_err(c"failed to get queue stats: %s".as_ptr(), (*ys).err.msg);
        ynl_sock_destroy(ys);
        return ptr::null_mut();
    }

    ynl_sock_destroy(ys);
    qstats
}

unsafe extern "C" fn do_show(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let qstats: *mut netdev_qstats_get_list;

    /* Parse options */
    while argc > 0 {
        if is_prefix(*argv, c"scope".as_ptr()) || is_prefix(*argv, c"group-by".as_ptr()) {
            next_arg(&mut argc, &mut argv);

            if !req_args(argc, 1) {
                return -1;
            }

            if is_prefix(*argv, c"queue".as_ptr()) {
                scope = netdev_qstats_scope::NETDEV_QSTATS_SCOPE_QUEUE;
            } else if is_prefix(*argv, c"device".as_ptr()) {
                scope = netdev_qstats_scope::NETDEV_QSTATS_SCOPE_UNSPEC;
            } else {
                p_err(c"invalid scope value '%s'".as_ptr(), *argv);
                return -1;
            }
            next_arg(&mut argc, &mut argv);
        } else {
            p_err(c"unknown option '%s'".as_ptr(), *argv);
            return -1;
        }
    }

    qstats = qstats_dump(scope);
    if qstats.is_null() {
        return -1;
    }

    /* Print the stats as returned by the kernel */
    if json_output {
        print_json_qstats(qstats);
    } else {
        print_plain_qstats(qstats);
    }

    netdev_qstats_get_list_free(qstats);
    0
}

unsafe fn compute_stats(
    values: *mut __u64,
    count: c_uint,
    mean: *mut c_double,
    stddev: *mut c_double,
    min: *mut __u64,
    max: *mut __u64,
) {
    let mut sum: c_double = 0.0;
    let mut variance: c_double = 0.0;
    let mut i: c_uint;

    *min = !0_u64;
    *max = 0;

    if count == 0 {
        *mean = 0.0;
        *stddev = 0.0;
        *min = 0;
        return;
    }

    i = 0;
    while i < count {
        sum += *values.add(i as usize) as c_double;
        if *values.add(i as usize) < *min {
            *min = *values.add(i as usize);
        }
        if *values.add(i as usize) > *max {
            *max = *values.add(i as usize);
        }
        i += 1;
    }

    *mean = sum / count as c_double;

    if count > 1 {
        i = 0;
        while i < count {
            let diff: c_double = *values.add(i as usize) as c_double - *mean;

            variance += diff * diff;
            i += 1;
        }
        *stddev = sqrt(variance / (count - 1) as c_double);
    } else {
        *stddev = 0.0;
    }
}

unsafe fn print_balance_stats(
    name: *const c_char,
    type_: netdev_queue_type,
    values: *mut __u64,
    count: c_uint,
) {
    let mut mean: c_double = 0.0;
    let mut stddev: c_double = 0.0;
    let cv: c_double;
    let ns: c_double;
    let mut min: __u64 = 0;
    let mut max: __u64 = 0;

    if (*name == b'r' as c_char && type_ != netdev_queue_type::NETDEV_QUEUE_TYPE_RX)
        || (*name == b't' as c_char && type_ != netdev_queue_type::NETDEV_QUEUE_TYPE_TX)
    {
        return;
    }

    compute_stats(values, count, &mut mean, &mut stddev, &mut min, &mut max);

    cv = if mean > 0.0 { (stddev / mean) * 100.0 } else { 0.0 };
    ns = if min + max > 0 {
        2.0 * (max - min) as c_double / (max + min) as c_double * 100.0
    } else {
        0.0
    };

    printf(c"  %-12s: cv=%.1f%% ns=%.1f%% stddev=%.0f\n".as_ptr(), name, cv, ns, stddev);
    printf(c"  %-12s  min=%llu max=%llu mean=%.0f\n".as_ptr(), c"".as_ptr(), min, max, mean);
}

unsafe fn print_balance_stats_json(
    name: *const c_char,
    type_: netdev_queue_type,
    values: *mut __u64,
    count: c_uint,
) {
    let mut mean: c_double = 0.0;
    let mut stddev: c_double = 0.0;
    let cv: c_double;
    let ns: c_double;
    let mut min: __u64 = 0;
    let mut max: __u64 = 0;

    if (*name == b'r' as c_char && type_ != netdev_queue_type::NETDEV_QUEUE_TYPE_RX)
        || (*name == b't' as c_char && type_ != netdev_queue_type::NETDEV_QUEUE_TYPE_TX)
    {
        return;
    }

    compute_stats(values, count, &mut mean, &mut stddev, &mut min, &mut max);

    cv = if mean > 0.0 { (stddev / mean) * 100.0 } else { 0.0 };
    ns = if min + max > 0 {
        2.0 * (max - min) as c_double / (max + min) as c_double * 100.0
    } else {
        0.0
    };

    jsonw_name(json_wtr, name);
    jsonw_start_object(json_wtr);
    jsonw_uint_field(json_wtr, c"queue-count".as_ptr(), count as c_ulonglong);
    jsonw_uint_field(json_wtr, c"min".as_ptr(), min);
    jsonw_uint_field(json_wtr, c"max".as_ptr(), max);
    jsonw_float_field(json_wtr, c"mean".as_ptr(), mean);
    jsonw_float_field(json_wtr, c"stddev".as_ptr(), stddev);
    jsonw_float_field(json_wtr, c"coefficient-of-variation".as_ptr(), cv);
    jsonw_float_field(json_wtr, c"normalized-spread".as_ptr(), ns);
    jsonw_end_object(json_wtr);
}

unsafe extern "C" fn cmp_ifindex_type(a: *const c_void, b: *const c_void) -> c_int {
    let qa = *(a as *const *mut netdev_qstats_get_rsp);
    let qb = *(b as *const *mut netdev_qstats_get_rsp);

    if (*qa).ifindex != (*qb).ifindex {
        return (*qa).ifindex as c_int - (*qb).ifindex as c_int;
    }
    if (*qa).queue_type as c_int != (*qb).queue_type as c_int {
        return (*qa).queue_type as c_int - (*qb).queue_type as c_int;
    }
    (*qa).queue_id as c_int - (*qb).queue_id as c_int
}

unsafe extern "C" fn do_balance(argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let qstats: *mut netdev_qstats_get_list;
    let sorted: *mut *mut netdev_qstats_get_rsp;
    let mut count: c_uint = 0;
    let mut i: c_uint;
    let mut j: c_uint;
    let mut ret: c_int = 0;

    if argc > 0 {
        p_err(c"balance command takes no arguments".as_ptr());
        return -1;
    }

    qstats = qstats_dump(netdev_qstats_scope::NETDEV_QSTATS_SCOPE_QUEUE);
    if qstats.is_null() {
        return -1;
    }

    /* Count and sort queues */
    dump_foreach(qstats, |_qs| {
        count += 1;
    });

    if count == 0 {
        if json_output {
            jsonw_start_array(json_wtr);
        } else {
            printf(c"No queue statistics available\n".as_ptr());
        }
        netdev_qstats_get_list_free(qstats);
        return ret;
    }

    sorted = calloc(count as usize, std::mem::size_of::<*mut netdev_qstats_get_rsp>())
        as *mut *mut netdev_qstats_get_rsp;
    if sorted.is_null() {
        p_err(c"failed to allocate sorted array".as_ptr());
        ret = -1;
        netdev_qstats_get_list_free(qstats);
        return ret;
    }

    i = 0;
    dump_foreach(qstats, |qs| unsafe {
        *sorted.add(i as usize) = qs;
        i += 1;
    });

    qsort(
        sorted as *mut c_void,
        count as usize,
        std::mem::size_of::<*mut netdev_qstats_get_rsp>(),
        Some(cmp_ifindex_type),
    );

    if json_output {
        jsonw_start_array(json_wtr);
    }

    /* Process each device/queue-type combination */
    i = 0;
    while i < count {
        let rx_packets: *mut __u64;
        let rx_bytes: *mut __u64;
        let tx_packets: *mut __u64;
        let tx_bytes: *mut __u64;
        let type_ = (**sorted.add(i as usize)).queue_type;
        let ifindex = (**sorted.add(i as usize)).ifindex;
        let mut queue_count: c_uint = 0;
        let mut ifname = [0 as c_char; IF_NAMESIZE];
        let name: *const c_char;

        /* Count queues for this device/type */
        j = i;
        while j < count
            && (**sorted.add(j as usize)).ifindex == ifindex
            && (**sorted.add(j as usize)).queue_type == type_
        {
            queue_count += 1;
            j += 1;
        }

        /* Skip if no packets/bytes (inactive queues) */
        if !(**sorted.add(i as usize))._present.rx_packets
            && !(**sorted.add(i as usize))._present.rx_bytes
            && !(**sorted.add(i as usize))._present.tx_packets
            && !(**sorted.add(i as usize))._present.tx_bytes
        {
            i += queue_count;
            continue;
        }

        /* Allocate arrays for statistics */
        rx_packets = calloc(queue_count as usize, std::mem::size_of::<__u64>()) as *mut __u64;
        rx_bytes = calloc(queue_count as usize, std::mem::size_of::<__u64>()) as *mut __u64;
        tx_packets = calloc(queue_count as usize, std::mem::size_of::<__u64>()) as *mut __u64;
        tx_bytes = calloc(queue_count as usize, std::mem::size_of::<__u64>()) as *mut __u64;

        if rx_packets.is_null() || rx_bytes.is_null() || tx_packets.is_null() || tx_bytes.is_null()
        {
            p_err(c"failed to allocate statistics arrays".as_ptr());
            free(rx_packets as *mut c_void);
            free(rx_bytes as *mut c_void);
            free(tx_packets as *mut c_void);
            free(tx_bytes as *mut c_void);
            ret = -1;
            break;
        }

        /* Collect statistics */
        j = 0;
        while j < queue_count {
            *rx_packets.add(j as usize) = if (**sorted.add((i + j) as usize))._present.rx_packets {
                (**sorted.add((i + j) as usize)).rx_packets
            } else {
                0
            };
            *rx_bytes.add(j as usize) = if (**sorted.add((i + j) as usize))._present.rx_bytes {
                (**sorted.add((i + j) as usize)).rx_bytes
            } else {
                0
            };
            *tx_packets.add(j as usize) = if (**sorted.add((i + j) as usize))._present.tx_packets {
                (**sorted.add((i + j) as usize)).tx_packets
            } else {
                0
            };
            *tx_bytes.add(j as usize) = if (**sorted.add((i + j) as usize))._present.tx_bytes {
                (**sorted.add((i + j) as usize)).tx_bytes
            } else {
                0
            };
            j += 1;
        }

        name = if_indextoname(ifindex, ifname.as_mut_ptr());

        if json_output {
            jsonw_start_object(json_wtr);
            if !name.is_null() {
                jsonw_string_field(json_wtr, c"ifname".as_ptr(), name);
            }
            jsonw_uint_field(json_wtr, c"ifindex".as_ptr(), ifindex as c_ulonglong);
            jsonw_string_field(json_wtr, c"queue-type".as_ptr(), netdev_queue_type_str(type_));

            print_balance_stats_json(c"rx-packets".as_ptr(), type_, rx_packets, queue_count);
            print_balance_stats_json(c"rx-bytes".as_ptr(), type_, rx_bytes, queue_count);
            print_balance_stats_json(c"tx-packets".as_ptr(), type_, tx_packets, queue_count);
            print_balance_stats_json(c"tx-bytes".as_ptr(), type_, tx_bytes, queue_count);

            jsonw_end_object(json_wtr);
        } else {
            if !name.is_null() {
                printf(c"%s".as_ptr(), name);
            } else {
                printf(c"ifindex:%u".as_ptr(), ifindex);
            }
            printf(c" %s %d queues:\n".as_ptr(), netdev_queue_type_str(type_), queue_count);

            print_balance_stats(c"rx-packets".as_ptr(), type_, rx_packets, queue_count);
            print_balance_stats(c"rx-bytes".as_ptr(), type_, rx_bytes, queue_count);
            print_balance_stats(c"tx-packets".as_ptr(), type_, tx_packets, queue_count);
            print_balance_stats(c"tx-bytes".as_ptr(), type_, tx_bytes, queue_count);
            printf(c"\n".as_ptr());
        }

        free(rx_packets as *mut c_void);
        free(rx_bytes as *mut c_void);
        free(tx_packets as *mut c_void);
        free(tx_bytes as *mut c_void);

        i += queue_count;
    }

    if json_output {
        jsonw_end_array(json_wtr);
    }

    free(sorted as *mut c_void);
    netdev_qstats_get_list_free(qstats);
    ret
}

unsafe extern "C" fn do_hw_gro(argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let qstats: *mut netdev_qstats_get_list;

    if argc > 0 {
        p_err(c"hw-gro command takes no arguments".as_ptr());
        return -1;
    }

    qstats = qstats_dump(netdev_qstats_scope::NETDEV_QSTATS_SCOPE_UNSPEC);
    if qstats.is_null() {
        return -1;
    }

    if json_output {
        jsonw_start_array(json_wtr);
    }

    dump_foreach(qstats, |qs| unsafe {
        let mut ifname = [0 as c_char; IF_NAMESIZE];
        let name: *const c_char;
        let savings: c_double;

        if !(*qs)._present.rx_packets
            || !(*qs)._present.rx_hw_gro_packets
            || !(*qs)._present.rx_hw_gro_wire_packets
        {
            return;
        }

        if (*qs).rx_packets == 0 {
            return;
        }

        /* How many skbs did we avoid allocating thanks to HW GRO */
        savings = ((*qs).rx_hw_gro_wire_packets - (*qs).rx_hw_gro_packets) as c_double
            / (*qs).rx_packets as c_double
            * 100.0;

        name = if_indextoname((*qs).ifindex, ifname.as_mut_ptr());

        if json_output {
            jsonw_start_object(json_wtr);
            jsonw_uint_field(json_wtr, c"ifindex".as_ptr(), (*qs).ifindex as c_ulonglong);
            if !name.is_null() {
                jsonw_string_field(json_wtr, c"ifname".as_ptr(), name);
            }
            jsonw_float_field(json_wtr, c"savings".as_ptr(), savings);
            jsonw_end_object(json_wtr);
        } else {
            if !name.is_null() {
                printf(c"%s".as_ptr(), name);
            } else {
                printf(c"ifindex:%u".as_ptr(), (*qs).ifindex);
            }
            printf(c": %.1f%% savings\n".as_ptr(), savings);
        }
    });

    if json_output {
        jsonw_end_array(json_wtr);
    }

    netdev_qstats_get_list_free(qstats);
    0
}

unsafe extern "C" fn do_help(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    if json_output {
        jsonw_null(json_wtr);
        return 0;
    }

    fprintf(
        stderr,
        c"Usage: %1$s qstats { COMMAND | help }\n       %1$s qstats [ show ] [ OPTIONS ]\n       %1$s qstats balance\n       %1$s qstats hw-gro\n\n       OPTIONS := { scope queue | group-by { device | queue } }\n\n       show                  - Display queue statistics (default)\n                               Statistics are aggregated for the entire device.\n       show scope queue      - Display per-queue statistics\n       show group-by device  - Display device-aggregated statistics (default)\n       show group-by queue   - Display per-queue statistics\n\n  Analysis:\n       balance               - Traffic distribution between queues.\n       hw-gro                - HW GRO effectiveness analysis\n                               - savings - delta between packets received\n                                 on the wire and packets seen by the kernel.\n".as_ptr(),
        bin_name,
    );

    0
}

static qstats_cmds: [cmd; 5] = [
    cmd {
        name: c"show".as_ptr(),
        func: Some(do_show),
    },
    cmd {
        name: c"balance".as_ptr(),
        func: Some(do_balance),
    },
    cmd {
        name: c"hw-gro".as_ptr(),
        func: Some(do_hw_gro),
    },
    cmd {
        name: c"help".as_ptr(),
        func: Some(do_help),
    },
    cmd {
        name: ptr::null(),
        func: None,
    },
];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_qstats(argc: c_int, argv: *mut *mut c_char) -> c_int {
    cmd_select(qstats_cmds.as_ptr(), argc, argv, Some(do_help))
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
