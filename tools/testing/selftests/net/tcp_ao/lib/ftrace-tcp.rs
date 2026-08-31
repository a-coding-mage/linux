// SPDX-License-Identifier: GPL-2.0
// C dependencies: <inttypes.h>, <pthread.h>, "aolib.h"

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type size_t = usize;
type uint64_t = u64;

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const EAFNOSUPPORT: c_int = 97;
const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const INET6_ADDRSTRLEN: size_t = 46;
const DEFAULT_FTRACE_BUFFER_KB: c_int = 1024;
const DEFAULT_TRACER_LINES_ARR: c_int = 1000;
const KCONFIG_FTRACE: c_int = 0;

#[repr(C)]
pub struct pthread_mutex_t {
    __private: [u8; 40],
}

const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t { __private: [0; 40] };

#[repr(C)]
#[derive(Copy, Clone)]
pub union tcp_addr {
    pub a4: [u8; 4],
    pub a6: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum trace_events {
    TCP_HASH_BAD_HEADER = 0,
    TCP_HASH_MD5_REQUIRED = 1,
    TCP_HASH_MD5_UNEXPECTED = 2,
    TCP_HASH_MD5_MISMATCH = 3,
    TCP_HASH_AO_REQUIRED = 4,
    TCP_AO_HANDSHAKE_FAILURE = 5,
    TCP_AO_WRONG_MACLEN = 6,
    TCP_AO_MISMATCH = 7,
    TCP_AO_KEY_NOT_FOUND = 8,
    TCP_AO_RNEXT_REQUEST = 9,
    TCP_AO_SYNACK_NO_KEY = 10,
    TCP_AO_SND_SNE_UPDATE = 11,
    TCP_AO_RCV_SNE_UPDATE = 12,
}

const __MAX_TRACE_EVENTS: size_t = 13;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ftracer_op {
    FTRACER_LINE_PRESERVE = 0,
    FTRACER_LINE_DISCARD = 1,
}

#[repr(C)]
pub struct test_ftracer {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut ns_cookie1: uint64_t;
    static mut ns_cookie2: uint64_t;
    static __test_msg: *mut c_void;

    fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;
    fn reallocarray(ptr: *mut c_void, nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    fn inet_ntop(af: c_int, src: *const c_void, dst: *mut c_char, size: size_t) -> *const c_char;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;

    fn kernel_config_has(config: c_int) -> bool;
    fn test_print(fmt: *const c_char, ...);
    fn test_error(fmt: *const c_char, ...) -> !;
    fn test_sprintf(fmt: *const c_char, ...) -> *mut c_char;
    fn test_fail(fmt: *const c_char, ...);
    fn test_ok(fmt: *const c_char, ...);
    fn test_skip(fmt: *const c_char, ...);
    fn test_xfail(fmt: *const c_char, ...);
    fn __test_print(msg: *mut c_void, fmt: *const c_char, ...);
    fn tracer_get_savedlines_nr(tracer: *mut test_ftracer) -> size_t;
    fn tracer_get_savedlines(tracer: *mut test_ftracer) -> *mut *const c_char;
    fn setup_trace_event(tracer: *mut test_ftracer, event_name: *const c_char, filter: *const c_char) -> c_int;
    fn create_ftracer(
        name: *const c_char,
        process_event: Option<unsafe extern "C" fn(*const c_char) -> ftracer_op>,
        destroy: Option<unsafe extern "C" fn(*mut test_ftracer)>,
        expecting_more: Option<unsafe extern "C" fn() -> bool>,
        buffer_kb: c_int,
        lines_arr: c_int,
    ) -> *mut test_ftracer;
}

static TRACE_EVENT_NAMES: [*const c_char; __MAX_TRACE_EVENTS] = [
    /* TCP_HASH_EVENT */
    b"tcp_hash_bad_header\0".as_ptr() as *const c_char,
    b"tcp_hash_md5_required\0".as_ptr() as *const c_char,
    b"tcp_hash_md5_unexpected\0".as_ptr() as *const c_char,
    b"tcp_hash_md5_mismatch\0".as_ptr() as *const c_char,
    b"tcp_hash_ao_required\0".as_ptr() as *const c_char,
    /* TCP_AO_EVENT */
    b"tcp_ao_handshake_failure\0".as_ptr() as *const c_char,
    b"tcp_ao_wrong_maclen\0".as_ptr() as *const c_char,
    b"tcp_ao_mismatch\0".as_ptr() as *const c_char,
    b"tcp_ao_key_not_found\0".as_ptr() as *const c_char,
    b"tcp_ao_rnext_request\0".as_ptr() as *const c_char,
    /* TCP_AO_EVENT_SK */
    b"tcp_ao_synack_no_key\0".as_ptr() as *const c_char,
    /* TCP_AO_EVENT_SNE */
    b"tcp_ao_snd_sne_update\0".as_ptr() as *const c_char,
    b"tcp_ao_rcv_sne_update\0".as_ptr() as *const c_char,
];

#[repr(C)]
#[derive(Copy, Clone)]
struct expected_trace_point {
    /* required */
    type_: trace_events,
    family: c_int,
    src: tcp_addr,
    dst: tcp_addr,

    /* optional */
    src_port: c_int,
    dst_port: c_int,
    L3index: c_int,

    fin: c_int,
    syn: c_int,
    rst: c_int,
    psh: c_int,
    ack: c_int,

    keyid: c_int,
    rnext: c_int,
    maclen: c_int,
    sne: c_int,

    matched: size_t,
}

static mut EXP_TPS: *mut expected_trace_point = ptr::null_mut();
static mut EXP_TPS_NR: size_t = 0;
static mut EXP_TPS_SIZE: size_t = 0;
static mut EXP_TPS_MUTEX: pthread_mutex_t = PTHREAD_MUTEX_INITIALIZER;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __trace_event_expect(
    type_: trace_events,
    family: c_int,
    src: tcp_addr,
    dst: tcp_addr,
    src_port: c_int,
    dst_port: c_int,
    L3index: c_int,
    fin: c_int,
    syn: c_int,
    rst: c_int,
    psh: c_int,
    ack: c_int,
    keyid: c_int,
    rnext: c_int,
    maclen: c_int,
    sne: c_int,
) -> c_int {
    let new_tp = expected_trace_point {
        type_,
        family,
        src,
        dst,
        src_port,
        dst_port,
        L3index,
        fin,
        syn,
        rst,
        psh,
        ack,
        keyid,
        rnext,
        maclen,
        sne,
        matched: 0,
    };
    let mut ret: c_int = 0;

    if !kernel_config_has(KCONFIG_FTRACE) {
        return 0;
    }

    pthread_mutex_lock(&raw mut EXP_TPS_MUTEX);
    if EXP_TPS_NR == EXP_TPS_SIZE {
        let tmp: *mut expected_trace_point;

        if EXP_TPS_SIZE == 0 {
            EXP_TPS_SIZE = 10;
        } else {
            EXP_TPS_SIZE = (EXP_TPS_SIZE as f64 * 1.6) as size_t;
        }

        tmp = reallocarray(
            EXP_TPS as *mut c_void,
            EXP_TPS_SIZE,
            mem::size_of::<expected_trace_point>(),
        ) as *mut expected_trace_point;
        if tmp.is_null() {
            ret = -ENOMEM;
            goto_out();
        } else {
            EXP_TPS = tmp;
        }
    }
    if ret == 0 {
        *EXP_TPS.add(EXP_TPS_NR) = new_tp;
        EXP_TPS_NR += 1;
    }

    unsafe fn goto_out() {}

    pthread_mutex_unlock(&raw mut EXP_TPS_MUTEX);
    ret
}

unsafe extern "C" fn free_expected_events() {
    /* We're from the process destructor - not taking the mutex */
    EXP_TPS_SIZE = 0;
    EXP_TPS = ptr::null_mut();
    free(EXP_TPS as *mut c_void);
}

#[repr(C)]
#[derive(Copy, Clone)]
struct trace_point {
    family: c_int,
    src: tcp_addr,
    dst: tcp_addr,
    src_port: c_uint,
    dst_port: c_uint,
    L3index: c_int,
    fin: c_uint,
    syn: c_uint,
    rst: c_uint,
    psh: c_uint,
    ack: c_uint,

    keyid: c_uint,
    rnext: c_uint,
    maclen: c_uint,

    sne: c_uint,
}

unsafe extern "C" fn lookup_expected_event(event_type: c_int, e: *mut trace_point) -> bool {
    let mut i: size_t;

    pthread_mutex_lock(&raw mut EXP_TPS_MUTEX);
    i = 0;
    while i < EXP_TPS_NR {
        let p = EXP_TPS.add(i);
        let sk_size: size_t;

        if (*p).type_ as c_int != event_type {
            i += 1;
            continue;
        }
        if (*p).family != (*e).family {
            i += 1;
            continue;
        }
        if (*p).family == AF_INET {
            sk_size = mem::size_of_val(&(*p).src.a4);
        } else {
            sk_size = mem::size_of_val(&(*p).src.a6);
        }
        if memcmp(&raw const (*p).src as *const c_void, &raw const (*e).src as *const c_void, sk_size) != 0 {
            i += 1;
            continue;
        }
        if memcmp(&raw const (*p).dst as *const c_void, &raw const (*e).dst as *const c_void, sk_size) != 0 {
            i += 1;
            continue;
        }
        if (*p).src_port >= 0 && (*p).src_port != (*e).src_port as c_int {
            i += 1;
            continue;
        }
        if (*p).dst_port >= 0 && (*p).dst_port != (*e).dst_port as c_int {
            i += 1;
            continue;
        }
        if (*p).L3index >= 0 && (*p).L3index != (*e).L3index {
            i += 1;
            continue;
        }

        if (*p).fin >= 0 && (*p).fin != (*e).fin as c_int {
            i += 1;
            continue;
        }
        if (*p).syn >= 0 && (*p).syn != (*e).syn as c_int {
            i += 1;
            continue;
        }
        if (*p).rst >= 0 && (*p).rst != (*e).rst as c_int {
            i += 1;
            continue;
        }
        if (*p).psh >= 0 && (*p).psh != (*e).psh as c_int {
            i += 1;
            continue;
        }
        if (*p).ack >= 0 && (*p).ack != (*e).ack as c_int {
            i += 1;
            continue;
        }

        if (*p).keyid >= 0 && (*p).keyid != (*e).keyid as c_int {
            i += 1;
            continue;
        }
        if (*p).rnext >= 0 && (*p).rnext != (*e).rnext as c_int {
            i += 1;
            continue;
        }
        if (*p).maclen >= 0 && (*p).maclen != (*e).maclen as c_int {
            i += 1;
            continue;
        }
        if (*p).sne >= 0 && (*p).sne != (*e).sne as c_int {
            i += 1;
            continue;
        }
        (*p).matched += 1;
        pthread_mutex_unlock(&raw mut EXP_TPS_MUTEX);
        return true;
    }
    pthread_mutex_unlock(&raw mut EXP_TPS_MUTEX);
    false
}

unsafe extern "C" fn check_event_type(line: *const c_char) -> c_int {
    let mut i: size_t;

    /*
     * This should have been a set or hashmap, but it's a selftest,
     * so... KISS.
     */
    i = 0;
    while i < __MAX_TRACE_EVENTS {
        if strncmp(TRACE_EVENT_NAMES[i], line, strlen(TRACE_EVENT_NAMES[i])) == 0 {
            return i as c_int;
        }
        i += 1;
    }
    -1
}

unsafe extern "C" fn event_has_flags(event: trace_events) -> bool {
    match event {
        trace_events::TCP_HASH_BAD_HEADER
        | trace_events::TCP_HASH_MD5_REQUIRED
        | trace_events::TCP_HASH_MD5_UNEXPECTED
        | trace_events::TCP_HASH_MD5_MISMATCH
        | trace_events::TCP_HASH_AO_REQUIRED
        | trace_events::TCP_AO_HANDSHAKE_FAILURE
        | trace_events::TCP_AO_WRONG_MACLEN
        | trace_events::TCP_AO_MISMATCH
        | trace_events::TCP_AO_KEY_NOT_FOUND
        | trace_events::TCP_AO_RNEXT_REQUEST => true,
        _ => false,
    }
}

unsafe extern "C" fn tracer_ip_split(
    family: c_int,
    src: *mut c_char,
    addr: *mut *mut c_char,
    port: *mut *mut c_char,
) -> c_int {
    let mut p: *mut c_char;

    if family == AF_INET {
        /* fomat is <addr>:port, i.e.: 10.0.254.1:7015 */
        *addr = src;
        p = strchr(src, ':' as c_int);
        if p.is_null() {
            test_print(b"Couldn't parse trace event addr:port %s\0".as_ptr() as *const c_char, src);
            return -EINVAL;
        }
        *p = '\0' as c_char;
        p = p.add(1);
        *port = p;
        return 0;
    }
    if family != AF_INET6 {
        return -EAFNOSUPPORT;
    }

    /* format is [<addr>]:port, i.e.: [2001:db8:254::1]:7013 */
    *addr = strchr(src, '[' as c_int);
    p = strchr(src, ']' as c_int);

    if p.is_null() || (*addr).is_null() {
        test_print(b"Couldn't parse trace event [addr]:port %s\0".as_ptr() as *const c_char, src);
        return -EINVAL;
    }

    *addr = (*addr).add(1); /* '[' */
    *p = '\0' as c_char; /* ']' */
    p = p.add(1);
    if *p != ':' as c_char {
        test_print(b"Couldn't parse trace event :port %s\0".as_ptr() as *const c_char, p);
        return -EINVAL;
    }
    *p = '\0' as c_char; /* ':' */
    p = p.add(1);
    *port = p;
    0
}

unsafe extern "C" fn tracer_scan_address(
    family: c_int,
    src: *mut c_char,
    dst: *mut tcp_addr,
    port: *mut c_uint,
) -> c_int {
    let mut addr: *mut c_char = ptr::null_mut();
    let mut port_str: *mut c_char = ptr::null_mut();
    let ret: c_int;

    ret = tracer_ip_split(family, src, &mut addr, &mut port_str);
    if ret != 0 {
        return ret;
    }

    if inet_pton(family, addr, dst as *mut c_void) != 1 {
        test_print(b"Couldn't parse trace event addr %s\0".as_ptr() as *const c_char, addr);
        return -EINVAL;
    }
    errno = 0;
    *port = strtoul(port_str, ptr::null_mut(), 10) as c_uint;
    if errno != 0 {
        test_print(b"Couldn't parse trace event port %s\0".as_ptr() as *const c_char, port_str);
        return -errno;
    }
    0
}

unsafe extern "C" fn tracer_scan_event(
    line: *const c_char,
    event: trace_events,
    out: *mut trace_point,
) -> c_int {
    let mut src: *mut c_char = ptr::null_mut();
    let mut dst: *mut c_char = ptr::null_mut();
    let mut family: *mut c_char = ptr::null_mut();
    let mut fin: c_char = 0;
    let mut syn: c_char = 0;
    let mut rst: c_char = 0;
    let mut psh: c_char = 0;
    let mut ack: c_char = 0;
    let nr_matched: c_int;
    let mut ret: c_int = 0;
    let mut netns_cookie: uint64_t = 0;

    match event {
        trace_events::TCP_HASH_BAD_HEADER
        | trace_events::TCP_HASH_MD5_REQUIRED
        | trace_events::TCP_HASH_MD5_UNEXPECTED
        | trace_events::TCP_HASH_MD5_MISMATCH
        | trace_events::TCP_HASH_AO_REQUIRED => {
            nr_matched = sscanf(
                line,
                b"%*s net=%lu state%*s family=%ms src=%ms dest=%ms L3index=%d [%c%c%c%c%c]\0".as_ptr()
                    as *const c_char,
                &mut netns_cookie,
                &mut family,
                &mut src,
                &mut dst,
                &mut (*out).L3index,
                &mut fin,
                &mut syn,
                &mut rst,
                &mut psh,
                &mut ack,
            );
            if nr_matched != 10 {
                test_print(
                    b"Couldn't parse trace event, matched = %d/10\0".as_ptr() as *const c_char,
                    nr_matched,
                );
            }
        }
        trace_events::TCP_AO_HANDSHAKE_FAILURE
        | trace_events::TCP_AO_WRONG_MACLEN
        | trace_events::TCP_AO_MISMATCH
        | trace_events::TCP_AO_KEY_NOT_FOUND
        | trace_events::TCP_AO_RNEXT_REQUEST => {
            nr_matched = sscanf(
                line,
                b"%*s net=%lu state%*s family=%ms src=%ms dest=%ms L3index=%d [%c%c%c%c%c] keyid=%u rnext=%u maclen=%u\0"
                    .as_ptr() as *const c_char,
                &mut netns_cookie,
                &mut family,
                &mut src,
                &mut dst,
                &mut (*out).L3index,
                &mut fin,
                &mut syn,
                &mut rst,
                &mut psh,
                &mut ack,
                &mut (*out).keyid,
                &mut (*out).rnext,
                &mut (*out).maclen,
            );
            if nr_matched != 13 {
                test_print(
                    b"Couldn't parse trace event, matched = %d/13\0".as_ptr() as *const c_char,
                    nr_matched,
                );
            }
        }
        trace_events::TCP_AO_SYNACK_NO_KEY => {
            nr_matched = sscanf(
                line,
                b"%*s net=%lu state%*s family=%ms src=%ms dest=%ms keyid=%u rnext=%u\0".as_ptr()
                    as *const c_char,
                &mut netns_cookie,
                &mut family,
                &mut src,
                &mut dst,
                &mut (*out).keyid,
                &mut (*out).rnext,
            );
            if nr_matched != 6 {
                test_print(
                    b"Couldn't parse trace event, matched = %d/6\0".as_ptr() as *const c_char,
                    nr_matched,
                );
            }
        }
        trace_events::TCP_AO_SND_SNE_UPDATE | trace_events::TCP_AO_RCV_SNE_UPDATE => {
            nr_matched = sscanf(
                line,
                b"%*s net=%lu state%*s family=%ms src=%ms dest=%ms sne=%u\0".as_ptr() as *const c_char,
                &mut netns_cookie,
                &mut family,
                &mut src,
                &mut dst,
                &mut (*out).sne,
            );
            if nr_matched != 5 {
                test_print(
                    b"Couldn't parse trace event, matched = %d/5\0".as_ptr() as *const c_char,
                    nr_matched,
                );
            }
        }
    }

    if !family.is_null() {
        if strcmp(family, b"AF_INET\0".as_ptr() as *const c_char) == 0 {
            (*out).family = AF_INET;
        } else if strcmp(family, b"AF_INET6\0".as_ptr() as *const c_char) == 0 {
            (*out).family = AF_INET6;
        } else {
            test_print(b"Couldn't parse trace event family %s\0".as_ptr() as *const c_char, family);
            ret = -EINVAL;
            goto_out_free(src, dst, family);
            return ret;
        }
    }

    if event_has_flags(event) {
        (*out).fin = (fin == 'F' as c_char) as c_uint;
        (*out).syn = (syn == 'S' as c_char) as c_uint;
        (*out).rst = (rst == 'R' as c_char) as c_uint;
        (*out).psh = (psh == 'P' as c_char) as c_uint;
        (*out).ack = (ack == '.' as c_char) as c_uint;

        if (fin != 'F' as c_char && fin != ' ' as c_char)
            || (syn != 'S' as c_char && syn != ' ' as c_char)
            || (rst != 'R' as c_char && rst != ' ' as c_char)
            || (psh != 'P' as c_char && psh != ' ' as c_char)
            || (ack != '.' as c_char && ack != ' ' as c_char)
        {
            test_print(
                b"Couldn't parse trace event flags %c%c%c%c%c\0".as_ptr() as *const c_char,
                fin as c_int,
                syn as c_int,
                rst as c_int,
                psh as c_int,
                ack as c_int,
            );
            ret = -EINVAL;
            goto_out_free(src, dst, family);
            return ret;
        }
    }

    if !src.is_null()
        && tracer_scan_address((*out).family, src, &mut (*out).src, &mut (*out).src_port) != 0
    {
        ret = -EINVAL;
        goto_out_free(src, dst, family);
        return ret;
    }

    if !dst.is_null()
        && tracer_scan_address((*out).family, dst, &mut (*out).dst, &mut (*out).dst_port) != 0
    {
        ret = -EINVAL;
        goto_out_free(src, dst, family);
        return ret;
    }

    if netns_cookie != ns_cookie1 && netns_cookie != ns_cookie2 {
        test_print(
            b"Net namespace filter for trace event didn't work: %lu != %lu OR %lu\0".as_ptr() as *const c_char,
            netns_cookie,
            ns_cookie1,
            ns_cookie2,
        );
        ret = -EINVAL;
    }

    goto_out_free(src, dst, family);
    ret
}

unsafe fn goto_out_free(src: *mut c_char, dst: *mut c_char, family: *mut c_char) {
    free(src as *mut c_void);
    free(dst as *mut c_void);
    free(family as *mut c_void);
}

unsafe extern "C" fn aolib_tracer_process_event(line: *const c_char) -> ftracer_op {
    let event_type = check_event_type(line);
    let mut tmp: trace_point = mem::zeroed();

    if event_type < 0 {
        return ftracer_op::FTRACER_LINE_PRESERVE;
    }

    if tracer_scan_event(line, mem::transmute::<c_int, trace_events>(event_type), &mut tmp) != 0 {
        return ftracer_op::FTRACER_LINE_PRESERVE;
    }

    if lookup_expected_event(event_type, &mut tmp) {
        ftracer_op::FTRACER_LINE_DISCARD
    } else {
        ftracer_op::FTRACER_LINE_PRESERVE
    }
}

unsafe extern "C" fn dump_trace_event(e: *mut expected_trace_point) {
    let mut src: [c_char; INET6_ADDRSTRLEN] = [0; INET6_ADDRSTRLEN];
    let mut dst: [c_char; INET6_ADDRSTRLEN] = [0; INET6_ADDRSTRLEN];

    if inet_ntop((*e).family, &raw const (*e).src as *const c_void, src.as_mut_ptr(), INET6_ADDRSTRLEN).is_null() {
        test_error(b"inet_ntop()\0".as_ptr() as *const c_char);
    }
    if inet_ntop((*e).family, &raw const (*e).dst as *const c_void, dst.as_mut_ptr(), INET6_ADDRSTRLEN).is_null() {
        test_error(b"inet_ntop()\0".as_ptr() as *const c_char);
    }
    test_print(
        b"trace event filter %s [%s:%d => %s:%d, L3index %d, flags: %s%s%s%s%s, keyid: %d, rnext: %d, maclen: %d, sne: %d] = %zu\0"
            .as_ptr() as *const c_char,
        TRACE_EVENT_NAMES[(*e).type_ as usize],
        src.as_ptr(),
        (*e).src_port,
        dst.as_ptr(),
        (*e).dst_port,
        (*e).L3index,
        if (*e).fin != 0 { b"F\0".as_ptr() as *const c_char } else { b"\0".as_ptr() as *const c_char },
        if (*e).syn != 0 { b"S\0".as_ptr() as *const c_char } else { b"\0".as_ptr() as *const c_char },
        if (*e).rst != 0 { b"R\0".as_ptr() as *const c_char } else { b"\0".as_ptr() as *const c_char },
        if (*e).psh != 0 { b"P\0".as_ptr() as *const c_char } else { b"\0".as_ptr() as *const c_char },
        if (*e).ack != 0 { b".\0".as_ptr() as *const c_char } else { b"\0".as_ptr() as *const c_char },
        (*e).keyid,
        (*e).rnext,
        (*e).maclen,
        (*e).sne,
        (*e).matched,
    );
}

unsafe extern "C" fn print_match_stats(unexpected_events: bool) {
    let mut matches_per_type: [size_t; __MAX_TRACE_EVENTS] = [0; __MAX_TRACE_EVENTS];
    let mut expected_but_none = false;
    let mut i: size_t;
    let mut total_matched: size_t = 0;
    let mut stat_line: *mut c_char = ptr::null_mut();

    i = 0;
    while i < EXP_TPS_NR {
        let e = EXP_TPS.add(i);

        total_matched += (*e).matched;
        matches_per_type[(*e).type_ as usize] += (*e).matched;
        if (*e).matched == 0 {
            expected_but_none = true;
        }
        i += 1;
    }
    i = 0;
    while i < __MAX_TRACE_EVENTS {
        if matches_per_type[i] == 0 {
            i += 1;
            continue;
        }
        stat_line = test_sprintf(
            b"%s%s[%zu] \0".as_ptr() as *const c_char,
            if stat_line.is_null() { b"\0".as_ptr() as *const c_char } else { stat_line as *const c_char },
            TRACE_EVENT_NAMES[i],
            matches_per_type[i],
        );
        if stat_line.is_null() {
            test_error(b"test_sprintf()\0".as_ptr() as *const c_char);
        }
        i += 1;
    }

    if unexpected_events || expected_but_none {
        i = 0;
        while i < EXP_TPS_NR {
            dump_trace_event(EXP_TPS.add(i));
            i += 1;
        }
    }

    if unexpected_events {
        return;
    }

    if expected_but_none {
        test_fail(b"Some trace events were expected, but didn't occur\0".as_ptr() as *const c_char);
    } else if total_matched != 0 {
        test_ok(
            b"Trace events matched expectations: %zu %s\0".as_ptr() as *const c_char,
            total_matched,
            stat_line,
        );
    } else {
        test_ok(b"No unexpected trace events during the test run\0".as_ptr() as *const c_char);
    }
}

macro_rules! dump_events {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {
        __test_print(__test_msg, $fmt $(, $arg)*)
    };
}

unsafe extern "C" fn check_free_events(tracer: *mut test_ftracer) {
    let mut lines: *mut *const c_char;
    let mut nr: size_t;

    if !kernel_config_has(KCONFIG_FTRACE) {
        test_skip(b"kernel config doesn't have ftrace - no checks\0".as_ptr() as *const c_char);
        return;
    }

    nr = tracer_get_savedlines_nr(tracer);
    lines = tracer_get_savedlines(tracer);
    print_match_stats(nr != 0);
    if nr == 0 {
        return;
    }

    errno = 0;
    test_xfail(b"Trace events [%zu] were not expected:\0".as_ptr() as *const c_char, nr);
    while nr != 0 {
        nr -= 1;
        dump_events!(b"\t%s\0".as_ptr() as *const c_char, *lines.add(nr));
    }
}

unsafe extern "C" fn setup_tcp_trace_events(tracer: *mut test_ftracer) -> c_int {
    let filter: *mut c_char;
    let mut i: size_t;
    let mut ret: c_int = 0;

    filter = test_sprintf(
        b"net_cookie == %zu || net_cookie == %zu\0".as_ptr() as *const c_char,
        ns_cookie1,
        ns_cookie2,
    );
    if filter.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while i < __MAX_TRACE_EVENTS {
        let event_name = test_sprintf(
            b"tcp/%s\0".as_ptr() as *const c_char,
            TRACE_EVENT_NAMES[i],
        );

        if event_name.is_null() {
            ret = -ENOMEM;
            break;
        }
        ret = setup_trace_event(tracer, event_name, filter);
        free(event_name as *mut c_void);
        if ret != 0 {
            break;
        }
        i += 1;
    }

    free(filter as *mut c_void);
    ret
}

unsafe extern "C" fn aolib_tracer_destroy(tracer: *mut test_ftracer) {
    check_free_events(tracer);
    free_expected_events();
}

unsafe extern "C" fn aolib_tracer_expecting_more() -> bool {
    let mut i: size_t;

    i = 0;
    while i < EXP_TPS_NR {
        if (*EXP_TPS.add(i)).matched == 0 {
            return true;
        }
        i += 1;
    }
    false
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn setup_aolib_ftracer() -> c_int {
    let f: *mut test_ftracer;

    f = create_ftracer(
        b"aolib\0".as_ptr() as *const c_char,
        Some(aolib_tracer_process_event),
        Some(aolib_tracer_destroy),
        Some(aolib_tracer_expecting_more),
        DEFAULT_FTRACE_BUFFER_KB,
        DEFAULT_TRACER_LINES_ARR,
    );
    if f.is_null() {
        return -1;
    }

    setup_tcp_trace_events(f)
}
