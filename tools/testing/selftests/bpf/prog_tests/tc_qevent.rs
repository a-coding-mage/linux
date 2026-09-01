// SPDX-License-Identifier: GPL-2.0

// Translated from C source using external test/libbpf and libc dependencies.

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

const NS_TX: &[u8] = b"tc_qevent_tx\0";
const NS_RX: &[u8] = b"tc_qevent_rx\0";
const IP_TX: &[u8] = b"10.255.0.1\0";
const IP_RX: &[u8] = b"10.255.0.2\0";
const PIN_PATH: &[u8] = b"/sys/fs/bpf/tc_qevent_redirect\0";

const AF_INET: c_int = 2;
const SOCK_DGRAM: c_int = 2;
const MSG_DONTWAIT: c_int = 0x40;

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nstoken {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr {
    pub sa_family: u16,
    pub sa_data: [c_char; 14],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_in {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}

#[repr(C)]
pub struct test_tc_qevent_progs {
    pub qevent_redirect_verdict: *mut bpf_program,
    pub qevent_redirect_helper: *mut bpf_program,
}

#[repr(C)]
pub struct test_tc_qevent_bss {
    pub verdict_calls: u64,
    pub helper_calls: u64,
}

#[repr(C)]
pub struct test_tc_qevent {
    pub progs: test_tc_qevent_progs,
    pub bss: *mut test_tc_qevent_bss,
}

unsafe extern "C" {
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn htons(hostshort: u16) -> u16;
    fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    fn sendto(
        sockfd: c_int,
        buf: *const c_void,
        len: usize,
        flags: c_int,
        dest_addr: *const sockaddr,
        addrlen: u32,
    ) -> isize;
    fn close(fd: c_int) -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;
    fn system(command: *const c_char) -> c_int;

    fn bpf_program__pin(prog: *mut bpf_program, path: *const c_char) -> c_int;
    fn bpf_program__unpin(prog: *mut bpf_program, path: *const c_char) -> c_int;
    fn open_netns(name: *const c_char) -> *mut nstoken;
    fn close_netns(tok: *mut nstoken);
    fn test__skip();
    fn test__start_subtest(name: *const c_char) -> bool;

    fn test_tc_qevent__open_and_load() -> *mut test_tc_qevent;
    fn test_tc_qevent__destroy(skel: *mut test_tc_qevent);

    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
    fn ASSERT_GT(actual: u64, expected: u64, name: *const c_char) -> bool;
}

// Local Rust equivalent of the test_progs SYS_NOFAIL helper intent.
macro_rules! SYS_NOFAIL {
    ($fmt:literal, $arg:expr) => {{
        let cmd = format!(concat!($fmt, "\0"), cstr_bytes_to_str($arg));
        unsafe {
            system(cmd.as_ptr() as *const c_char);
        }
    }};
}

// Local Rust equivalent of the test_progs SYS(label, ...) helper intent. On
// command failure it evaluates to false so callers can jump to the matching
// cleanup label equivalent.
macro_rules! SYS {
    ($fmt:literal $(, $arg:expr)*) => {{
        let cmd = format!(concat!($fmt, "\0") $(, cstr_bytes_to_str($arg))*);
        unsafe { system(cmd.as_ptr() as *const c_char) == 0 }
    }};
}

fn cstr_bytes_to_str(bytes: &[u8]) -> &str {
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    core::str::from_utf8(&bytes[..end]).unwrap()
}

unsafe fn blast_udp() {
    let mut dst: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let buf: [c_char; 1400] = [0; 1400];
    let mut i: c_int;

    let fd = socket(AF_INET, SOCK_DGRAM, 0);
    if !ASSERT_GE(fd, 0, c"udp socket".as_ptr()) {
        return;
    }

    dst.sin_family = AF_INET as u16;
    dst.sin_port = htons(12345);
    inet_pton(
        AF_INET,
        IP_RX.as_ptr() as *const c_char,
        &mut dst.sin_addr as *mut in_addr as *mut c_void,
    );

    /*
     * Push far more than the RED queue can hold. Once qavg crosses qth_min
     * every further packet hits the congestion_drop / early_drop qevent.
     */
    i = 0;
    while i < 50000 {
        sendto(
            fd,
            buf.as_ptr() as *const c_void,
            size_of::<[c_char; 1400]>(),
            MSG_DONTWAIT,
            &dst as *const sockaddr_in as *const sockaddr,
            size_of::<sockaddr_in>() as u32,
        );
        i += 1;
    }

    close(fd);
}

unsafe fn run_qevent_redirect(prog: *mut bpf_program, counter: *mut u64) {
    let mut tok: *mut nstoken = ptr::null_mut();
    let mut pinned = false;
    let mut tx_created = false;
    let mut rx_created = false;
    let mut ns_open = false;

    SYS_NOFAIL!("ip netns del {}", NS_TX);
    SYS_NOFAIL!("ip netns del {}", NS_RX);
    unlink(PIN_PATH.as_ptr() as *const c_char);

    let err = bpf_program__pin(prog, PIN_PATH.as_ptr() as *const c_char);
    if !ASSERT_OK(err, c"pin prog".as_ptr()) {
        return;
    }
    pinned = true;

    if !SYS!("ip netns add {}", NS_TX) {
        goto_unpin(prog, pinned);
        return;
    }
    tx_created = true;
    if !SYS!("ip netns add {}", NS_RX) {
        goto_del_tx(prog, pinned, tx_created);
        return;
    }
    rx_created = true;
    if !SYS!(
        "ip -n {} link add veth0 type veth peer name veth1 netns {}",
        NS_TX,
        NS_RX
    ) {
        goto_del_rx(prog, pinned, tx_created, rx_created);
        return;
    }
    if !SYS!("ip -n {} addr add {}/24 dev veth0", NS_TX, IP_TX) {
        goto_del_rx(prog, pinned, tx_created, rx_created);
        return;
    }
    if !SYS!("ip -n {} link set veth0 up", NS_TX) {
        goto_del_rx(prog, pinned, tx_created, rx_created);
        return;
    }
    if !SYS!("ip -n {} addr add {}/24 dev veth1", NS_RX, IP_RX) {
        goto_del_rx(prog, pinned, tx_created, rx_created);
        return;
    }
    if !SYS!("ip -n {} link set veth1 up", NS_RX) {
        goto_del_rx(prog, pinned, tx_created, rx_created);
        return;
    }

    tok = open_netns(NS_TX.as_ptr() as *const c_char);
    if !ASSERT_OK_PTR(tok as *mut c_void, c"open_netns".as_ptr()) {
        goto_del_rx(prog, pinned, tx_created, rx_created);
        return;
    }
    ns_open = true;

    if !SYS!("tc qdisc add dev veth0 root handle 1: htb default 1") {
        goto_close_ns(tok, ns_open, prog, pinned, tx_created, rx_created);
        return;
    }
    if !SYS!("tc class add dev veth0 parent 1: classid 1:1 htb rate 1mbit ceil 1mbit") {
        goto_close_ns(tok, ns_open, prog, pinned, tx_created, rx_created);
        return;
    }

    if system(c"tc qdisc add dev veth0 parent 1:1 handle 11: red limit 500000 avpkt 1000 probability 1 min 5000 max 6000 burst 6 qevent early_drop block 10 2>/dev/null".as_ptr()) != 0 {
        test__skip();
        goto_close_ns(tok, ns_open, prog, pinned, tx_created, rx_created);
        return;
    }

    if system(c"tc filter add block 10 bpf da object-pinned /sys/fs/bpf/tc_qevent_redirect 2>/dev/null".as_ptr()) != 0 {
        test__skip();
        goto_close_ns(tok, ns_open, prog, pinned, tx_created, rx_created);
        return;
    }

    blast_udp();
    ASSERT_GT(*counter, 0, c"qevent classifier ran".as_ptr());

    goto_close_ns(tok, ns_open, prog, pinned, tx_created, rx_created);
}

unsafe fn goto_close_ns(
    tok: *mut nstoken,
    ns_open: bool,
    prog: *mut bpf_program,
    pinned: bool,
    tx_created: bool,
    rx_created: bool,
) {
    if ns_open {
        close_netns(tok);
    }
    goto_del_rx(prog, pinned, tx_created, rx_created);
}

unsafe fn goto_del_rx(prog: *mut bpf_program, pinned: bool, tx_created: bool, rx_created: bool) {
    if rx_created {
        SYS_NOFAIL!("ip netns del {}", NS_RX);
    }
    goto_del_tx(prog, pinned, tx_created);
}

unsafe fn goto_del_tx(prog: *mut bpf_program, pinned: bool, tx_created: bool) {
    if tx_created {
        SYS_NOFAIL!("ip netns del {}", NS_TX);
    }
    goto_unpin(prog, pinned);
}

unsafe fn goto_unpin(prog: *mut bpf_program, pinned: bool) {
    if pinned {
        bpf_program__unpin(prog, PIN_PATH.as_ptr() as *const c_char);
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_tc_qevent() {
    let skel = test_tc_qevent__open_and_load();
    if !ASSERT_OK_PTR(skel as *mut c_void, c"open_and_load".as_ptr()) {
        return;
    }

    if test__start_subtest(c"redirect_verdict".as_ptr()) {
        run_qevent_redirect(
            (*skel).progs.qevent_redirect_verdict,
            &mut (*(*skel).bss).verdict_calls as *mut u64,
        );
    }
    if test__start_subtest(c"redirect_helper".as_ptr()) {
        run_qevent_redirect(
            (*skel).progs.qevent_redirect_helper,
            &mut (*(*skel).bss).helper_calls as *mut u64,
        );
    }

    test_tc_qevent__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
