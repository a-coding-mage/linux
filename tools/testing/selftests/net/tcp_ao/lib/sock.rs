// SPDX-License-Identifier: GPL-2.0
// Translated from testing/selftests/net/tcp_ao/lib/sock.c.
// C include dependencies are expected to be supplied by the surrounding crate.

use core::ffi::{c_char, c_int, c_long, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

pub const test_server_port: u32 = 7010;
const POLL_USEC: u64 = 150;
const TEST_BUF_SIZE: usize = 4096;

extern "C" {
    static test_family: c_int;
    static veth_name: *const c_char;
    static mut errno: c_int;

    fn socket(domain: c_int, typ: c_int, protocol: c_int) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: socklen_t,
    ) -> c_int;
    fn getsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *mut c_void,
        option_len: *mut socklen_t,
    ) -> c_int;
    fn bind(socket: c_int, address: *const sockaddr, address_len: socklen_t) -> c_int;
    fn listen(socket: c_int, backlog: c_int) -> c_int;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_long;
    fn select(
        nfds: c_int,
        readfds: *mut fd_set,
        writefds: *mut fd_set,
        exceptfds: *mut fd_set,
        timeout: *mut timeval,
    ) -> c_int;
    fn connect(socket: c_int, address: *const sockaddr, address_len: socklen_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncpy(dst: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn recv(socket: c_int, buffer: *mut c_void, length: usize, flags: c_int) -> ssize_t;
    fn send(socket: c_int, buffer: *const c_void, length: usize, flags: c_int) -> ssize_t;
    fn alloca(size: usize) -> *mut c_void;

    fn test_error(fmt: *const c_char, ...);
    fn test_fail(fmt: *const c_char, ...);
    fn synchronize_threads();
    fn randomize_buffer(buf: *mut c_char, buflen: usize);
    fn netstat_read() -> *mut netstat;
    fn netstat_get(ns: *mut netstat, name: *const c_char, unsupported: *mut bool) -> u64;
    fn netstat_free(ns: *mut netstat);
}

pub unsafe extern "C" fn __test_listen_socket(
    backlog: c_int,
    addr: *mut c_void,
    addr_sz: usize,
) -> c_int {
    let mut err: c_int;
    let sk = socket(test_family, SOCK_STREAM, IPPROTO_TCP);
    let flags: c_long;

    if sk < 0 {
        test_error(c"socket()".as_ptr());
    }

    err = setsockopt(
        sk,
        SOL_SOCKET,
        SO_BINDTODEVICE,
        veth_name as *const c_void,
        (strlen(veth_name) + 1) as socklen_t,
    );
    if err < 0 {
        test_error(c"setsockopt(SO_BINDTODEVICE)".as_ptr());
    }

    if bind(sk, addr as *const sockaddr, addr_sz as socklen_t) < 0 {
        test_error(c"bind()".as_ptr());
    }

    flags = fcntl(sk, F_GETFL);
    if flags < 0 || fcntl(sk, F_SETFL, flags | O_NONBLOCK as c_long) < 0 {
        test_error(c"fcntl()".as_ptr());
    }

    if listen(sk, backlog) != 0 {
        test_error(c"listen()".as_ptr());
    }

    sk
}

unsafe fn __test_wait_fd(sk: c_int, tv: *mut timeval, write: bool) -> c_int {
    let mut fds: fd_set = zeroed();
    let mut efds: fd_set = zeroed();
    let mut ret: c_int;
    let mut slen: socklen_t = size_of::<c_int>() as socklen_t;

    FD_ZERO(&mut fds);
    FD_SET(sk, &mut fds);
    FD_ZERO(&mut efds);
    FD_SET(sk, &mut efds);

    errno = 0;
    if write {
        ret = select(sk + 1, null_mut(), &mut fds, &mut efds, tv);
    } else {
        ret = select(sk + 1, &mut fds, null_mut(), &mut efds, tv);
    }
    if ret < 0 {
        return -errno;
    }
    if ret == 0 {
        errno = ETIMEDOUT;
        return -ETIMEDOUT;
    }

    if getsockopt(
        sk,
        SOL_SOCKET,
        SO_ERROR,
        &mut ret as *mut _ as *mut c_void,
        &mut slen,
    ) != 0
    {
        return -errno;
    }
    if ret != 0 {
        return -ret;
    }
    0
}

pub unsafe extern "C" fn test_wait_fd(sk: c_int, sec: time_t, write: bool) -> c_int {
    let mut tv = timeval {
        tv_sec: sec,
        tv_usec: 0,
    };

    __test_wait_fd(sk, if sec != 0 { &mut tv } else { null_mut() }, write)
}

unsafe fn __skpair_poll_should_stop(
    sk: c_int,
    c: *mut tcp_counters,
    condition: test_cnt,
) -> bool {
    let mut c2: tcp_counters = zeroed();
    let diff: test_cnt;

    if test_get_tcp_counters(sk, &mut c2) != 0 {
        test_error(c"test_get_tcp_counters()".as_ptr());
    }

    diff = test_cmp_counters(c, &mut c2);
    test_tcp_counters_free(&mut c2);
    (diff & condition) == condition
}

/* How often wake up and check netns counters & paired (*err) */
unsafe fn __test_skpair_poll(
    sk: c_int,
    write: bool,
    timeout: u64,
    c: *mut tcp_counters,
    cond: test_cnt,
    err: *mut c_int,
) -> c_int {
    let mut t: u64 = 0;

    while t <= timeout * 1000000 {
        let mut tv = timeval {
            tv_sec: 0,
            tv_usec: POLL_USEC as _,
        };
        let ret = __test_wait_fd(sk, &mut tv, write);
        if ret != -ETIMEDOUT {
            return ret;
        }
        if !c.is_null() && cond != 0 && __skpair_poll_should_stop(sk, c, cond) {
            break;
        }
        if !err.is_null() && core::ptr::read_volatile(err) != 0 {
            return core::ptr::read_volatile(err);
        }
        t += POLL_USEC;
    }
    if !err.is_null() {
        core::ptr::write_volatile(err, -ETIMEDOUT);
    }
    -ETIMEDOUT
}

pub unsafe extern "C" fn __test_connect_socket(
    sk: c_int,
    device: *const c_char,
    addr: *mut c_void,
    addr_sz: usize,
    async_: bool,
) -> c_int {
    let flags: c_long;
    let mut err: c_int;

    if !device.is_null() {
        err = setsockopt(
            sk,
            SOL_SOCKET,
            SO_BINDTODEVICE,
            device as *const c_void,
            (strlen(device) + 1) as socklen_t,
        );
        if err < 0 {
            test_error(c"setsockopt(SO_BINDTODEVICE, %s)".as_ptr(), device);
        }
    }

    flags = fcntl(sk, F_GETFL);
    if flags < 0 || fcntl(sk, F_SETFL, flags | O_NONBLOCK as c_long) < 0 {
        test_error(c"fcntl()".as_ptr());
    }

    if connect(sk, addr as *const sockaddr, addr_sz as socklen_t) < 0 {
        if errno != EINPROGRESS {
            err = -errno;
            close(sk);
            return err;
        }
        if async_ {
            return sk;
        }
        err = test_wait_fd(sk, TEST_TIMEOUT_SEC as time_t, true);
        if err != 0 {
            close(sk);
            return err;
        }
    }
    sk
}

pub unsafe extern "C" fn test_skpair_wait_poll(
    sk: c_int,
    write: bool,
    cond: test_cnt,
    err: *mut c_int,
) -> c_int {
    let mut c: tcp_counters = zeroed();
    let ret: c_int;

    core::ptr::write_volatile(err, 0);
    if test_get_tcp_counters(sk, &mut c) != 0 {
        test_error(c"test_get_tcp_counters()".as_ptr());
    }
    synchronize_threads(); /* 1: init skpair & read nscounters */

    ret = __test_skpair_poll(sk, write, TEST_TIMEOUT_SEC as u64, &mut c, cond, err);
    test_tcp_counters_free(&mut c);
    ret
}

pub unsafe extern "C" fn _test_skpair_connect_poll(
    sk: c_int,
    device: *const c_char,
    addr: *mut c_void,
    addr_sz: usize,
    condition: test_cnt,
    err: *mut c_int,
) -> c_int {
    let mut c: tcp_counters = zeroed();
    let mut ret: c_int;

    core::ptr::write_volatile(err, 0);
    if test_get_tcp_counters(sk, &mut c) != 0 {
        test_error(c"test_get_tcp_counters()".as_ptr());
    }
    synchronize_threads(); /* 1: init skpair & read nscounters */
    ret = __test_connect_socket(sk, device, addr, addr_sz, true);
    if ret < 0 {
        test_tcp_counters_free(&mut c);
        core::ptr::write_volatile(err, ret);
        return ret;
    }
    ret = __test_skpair_poll(sk, true, TEST_TIMEOUT_SEC as u64, &mut c, condition, err);
    if ret < 0 {
        close(sk);
    }
    test_tcp_counters_free(&mut c);
    ret
}

pub unsafe extern "C" fn __test_set_md5(
    sk: c_int,
    addr: *mut c_void,
    addr_sz: usize,
    prefix: u8,
    vrf: c_int,
    password: *const c_char,
) -> c_int {
    let pwd_len = strlen(password);
    let mut md5sig: tcp_md5sig = zeroed();

    md5sig.tcpm_keylen = pwd_len as _;
    memcpy(md5sig.tcpm_key.as_mut_ptr() as *mut c_void, password as *const c_void, pwd_len);
    md5sig.tcpm_flags = TCP_MD5SIG_FLAG_PREFIX;
    md5sig.tcpm_prefixlen = prefix;
    if vrf >= 0 {
        md5sig.tcpm_flags |= TCP_MD5SIG_FLAG_IFINDEX;
        md5sig.tcpm_ifindex = vrf as u8;
    }
    memcpy(&mut md5sig.tcpm_addr as *mut _ as *mut c_void, addr, addr_sz);

    errno = 0;
    setsockopt(
        sk,
        IPPROTO_TCP,
        TCP_MD5SIG_EXT,
        &md5sig as *const _ as *const c_void,
        size_of::<tcp_md5sig>() as socklen_t,
    )
}

pub unsafe extern "C" fn test_prepare_key_sockaddr(
    ao: *mut tcp_ao_add,
    alg: *const c_char,
    addr: *mut c_void,
    addr_sz: usize,
    set_current: bool,
    set_rnext: bool,
    prefix: u8,
    vrf: u8,
    sndid: u8,
    rcvid: u8,
    maclen: u8,
    keyflags: u8,
    keylen: u8,
    key: *const c_char,
) -> c_int {
    memset(ao as *mut c_void, 0, size_of::<tcp_ao_add>());

    (*ao).set_current = (set_current as c_int) as _;
    (*ao).set_rnext = (set_rnext as c_int) as _;
    (*ao).prefix = prefix;
    (*ao).sndid = sndid;
    (*ao).rcvid = rcvid;
    (*ao).maclen = maclen;
    (*ao).keyflags = keyflags;
    (*ao).keylen = keylen;
    (*ao).ifindex = vrf as _;

    memcpy(&mut (*ao).addr as *mut _ as *mut c_void, addr, addr_sz);

    if strlen(alg) > 64 {
        return -ENOBUFS;
    }
    strncpy((*ao).alg_name.as_mut_ptr(), alg, 64);

    memcpy(
        (*ao).key.as_mut_ptr() as *mut c_void,
        key as *const c_void,
        if keylen as usize > TCP_AO_MAXKEYLEN {
            TCP_AO_MAXKEYLEN
        } else {
            keylen as usize
        },
    );
    0
}

unsafe fn test_get_ao_keys_nr(sk: c_int) -> c_int {
    let mut tmp: tcp_ao_getsockopt = zeroed();
    let mut tmp_sz: socklen_t = size_of::<tcp_ao_getsockopt>() as socklen_t;
    let ret: c_int;

    tmp.nkeys = 1;
    tmp.get_all = 1;

    ret = getsockopt(
        sk,
        IPPROTO_TCP,
        TCP_AO_GET_KEYS,
        &mut tmp as *mut _ as *mut c_void,
        &mut tmp_sz,
    );
    if ret != 0 {
        return -errno;
    }
    tmp.nkeys as c_int
}

pub unsafe extern "C" fn test_get_one_ao(
    sk: c_int,
    out: *mut tcp_ao_getsockopt,
    addr: *mut c_void,
    addr_sz: usize,
    prefix: u8,
    sndid: u8,
    rcvid: u8,
    keyflags: u8,
    ifindex: c_int,
) -> c_int {
    let mut tmp: tcp_ao_getsockopt = zeroed();
    let mut tmp_sz: socklen_t = size_of::<tcp_ao_getsockopt>() as socklen_t;
    let ret: c_int;

    memcpy(&mut tmp.addr as *mut _ as *mut c_void, addr, addr_sz);
    tmp.prefix = prefix;
    tmp.sndid = sndid;
    tmp.rcvid = rcvid;
    tmp.keyflags = keyflags;
    tmp.ifindex = ifindex;
    tmp.nkeys = 1;

    ret = getsockopt(
        sk,
        IPPROTO_TCP,
        TCP_AO_GET_KEYS,
        &mut tmp as *mut _ as *mut c_void,
        &mut tmp_sz,
    );
    if ret != 0 {
        return ret;
    }
    if tmp.nkeys != 1 {
        return -E2BIG;
    }
    *out = tmp;
    0
}

pub unsafe extern "C" fn test_get_ao_info(sk: c_int, out: *mut tcp_ao_info_opt) -> c_int {
    let mut sz: socklen_t = size_of::<tcp_ao_info_opt>() as socklen_t;

    (*out).reserved = 0;
    (*out).reserved2 = 0;
    if getsockopt(sk, IPPROTO_TCP, TCP_AO_INFO, out as *mut c_void, &mut sz) != 0 {
        return -errno;
    }
    if sz as usize != size_of::<tcp_ao_info_opt>() {
        return -EMSGSIZE;
    }
    0
}

pub unsafe extern "C" fn test_set_ao_info(sk: c_int, in_: *mut tcp_ao_info_opt) -> c_int {
    let sz: socklen_t = size_of::<tcp_ao_info_opt>() as socklen_t;

    (*in_).reserved = 0;
    (*in_).reserved2 = 0;
    if setsockopt(sk, IPPROTO_TCP, TCP_AO_INFO, in_ as *const c_void, sz) != 0 {
        return -errno;
    }
    0
}

pub unsafe extern "C" fn test_cmp_getsockopt_setsockopt(
    a: *const tcp_ao_add,
    b: *const tcp_ao_getsockopt,
) -> c_int {
    let mut is_kdf_aes_128_cmac = false;
    let mut is_cmac_aes = false;

    if strcmp(c"cmac(aes128)".as_ptr(), (*a).alg_name.as_ptr()) == 0 {
        is_kdf_aes_128_cmac = (*a).keylen != 16;
        is_cmac_aes = true;
    }

    macro_rules! cmp_ao {
        ($member:ident, $name:literal) => {
            if (*b).$member != (*a).$member {
                test_fail(c"getsockopt(): %s %u != %u".as_ptr(), concat!($name, "\0").as_ptr(), (*b).$member, (*a).$member);
                return -1;
            }
        };
    }
    cmp_ao!(sndid, "sndid");
    cmp_ao!(rcvid, "rcvid");
    cmp_ao!(prefix, "prefix");
    cmp_ao!(keyflags, "keyflags");
    cmp_ao!(ifindex, "ifindex");
    if (*a).maclen != 0 {
        cmp_ao!(maclen, "maclen");
    } else if (*b).maclen != 12 {
        test_fail(c"getsockopt(): expected default maclen 12, but it's %u".as_ptr(), (*b).maclen);
        return -1;
    }
    if !is_kdf_aes_128_cmac {
        cmp_ao!(keylen, "keylen");
    } else if (*b).keylen != 16 {
        test_fail(c"getsockopt(): expected keylen 16 for cmac(aes128), but it's %u".as_ptr(), (*b).keylen);
        return -1;
    }
    if !is_kdf_aes_128_cmac
        && memcmp((*b).key.as_ptr() as *const c_void, (*a).key.as_ptr() as *const c_void, (*a).keylen as usize) != 0
    {
        test_fail(c"getsockopt(): returned key is different `%s' != `%s'".as_ptr(), (*b).key.as_ptr(), (*a).key.as_ptr());
        return -1;
    }
    if memcmp(
        &(*b).addr as *const _ as *const c_void,
        &(*a).addr as *const _ as *const c_void,
        size_of_val_raw(&(*b).addr),
    ) != 0
    {
        test_fail(c"getsockopt(): returned address is different".as_ptr());
        return -1;
    }
    if !is_cmac_aes && strcmp((*b).alg_name.as_ptr(), (*a).alg_name.as_ptr()) != 0 {
        test_fail(c"getsockopt(): returned algorithm %s is different than %s".as_ptr(), (*b).alg_name.as_ptr(), (*a).alg_name.as_ptr());
        return -1;
    }
    if is_cmac_aes && strcmp((*b).alg_name.as_ptr(), c"cmac(aes)".as_ptr()) != 0 {
        test_fail(c"getsockopt(): returned algorithm %s is different than cmac(aes)".as_ptr(), (*b).alg_name.as_ptr());
        return -1;
    }
    /* For a established key rotation test don't add a key with
     * set_current = 1, as it's likely to change by peer's request;
     * rather use setsockopt(TCP_AO_INFO)
     */
    if (*a).set_current != (*b).is_current {
        test_fail(c"getsockopt(): returned key is not Current_key".as_ptr());
        return -1;
    }
    if (*a).set_rnext != (*b).is_rnext {
        test_fail(c"getsockopt(): returned key is not RNext_key".as_ptr());
        return -1;
    }

    0
}

pub unsafe extern "C" fn test_cmp_getsockopt_setsockopt_ao(
    a: *const tcp_ao_info_opt,
    b: *const tcp_ao_info_opt,
) -> c_int {
    /* No check for ::current_key, as it may change by the peer */
    if (*a).ao_required != (*b).ao_required {
        test_fail(c"getsockopt(): returned ao doesn't have ao_required".as_ptr());
        return -1;
    }
    if (*a).accept_icmps != (*b).accept_icmps {
        test_fail(c"getsockopt(): returned ao doesn't accept ICMPs".as_ptr());
        return -1;
    }
    if (*a).set_rnext != 0 && (*a).rnext != (*b).rnext {
        test_fail(c"getsockopt(): RNext KeyID has changed".as_ptr());
        return -1;
    }
    macro_rules! cmp_cnt {
        ($member:ident, $name:literal) => {
            if (*b).$member != (*a).$member {
                test_fail(c"getsockopt(): %s %llu != %llu".as_ptr(), concat!($name, "\0").as_ptr(), (*b).$member, (*a).$member);
                return -1;
            }
        };
    }
    if (*a).set_counters != 0 {
        cmp_cnt!(pkt_good, "pkt_good");
        cmp_cnt!(pkt_bad, "pkt_bad");
        cmp_cnt!(pkt_key_not_found, "pkt_key_not_found");
        cmp_cnt!(pkt_ao_required, "pkt_ao_required");
        cmp_cnt!(pkt_dropped_icmp, "pkt_dropped_icmp");
    }
    0
}

pub unsafe extern "C" fn test_get_tcp_counters(sk: c_int, out: *mut tcp_counters) -> c_int {
    let mut key_dump: *mut tcp_ao_getsockopt;
    let mut key_dump_sz: socklen_t = size_of::<tcp_ao_getsockopt>() as socklen_t;
    let mut info: tcp_ao_info_opt = zeroed();
    let (mut c1, mut c2, mut c3, mut c4, mut c5, mut c6, mut c7, mut c8) =
        (false, false, false, false, false, false, false, false);
    let ns: *mut netstat;
    let mut err: c_int;
    let mut nr_keys: c_int;

    memset(out as *mut c_void, 0, size_of::<tcp_counters>());

    /* per-netns */
    ns = netstat_read();
    (*out).ao.netns_ao_good = netstat_get(ns, c"TCPAOGood".as_ptr(), &mut c1);
    (*out).ao.netns_ao_bad = netstat_get(ns, c"TCPAOBad".as_ptr(), &mut c2);
    (*out).ao.netns_ao_key_not_found = netstat_get(ns, c"TCPAOKeyNotFound".as_ptr(), &mut c3);
    (*out).ao.netns_ao_required = netstat_get(ns, c"TCPAORequired".as_ptr(), &mut c4);
    (*out).ao.netns_ao_dropped_icmp = netstat_get(ns, c"TCPAODroppedIcmps".as_ptr(), &mut c5);
    (*out).netns_md5_notfound = netstat_get(ns, c"TCPMD5NotFound".as_ptr(), &mut c6);
    (*out).netns_md5_unexpected = netstat_get(ns, c"TCPMD5Unexpected".as_ptr(), &mut c7);
    (*out).netns_md5_failure = netstat_get(ns, c"TCPMD5Failure".as_ptr(), &mut c8);
    netstat_free(ns);
    if c1 || c2 || c3 || c4 || c5 || c6 || c7 || c8 {
        return -EOPNOTSUPP;
    }

    err = test_get_ao_info(sk, &mut info);
    if err == -ENOENT {
        return 0;
    }
    if err != 0 {
        return err;
    }

    /* per-socket */
    (*out).ao.ao_info_pkt_good = info.pkt_good;
    (*out).ao.ao_info_pkt_bad = info.pkt_bad;
    (*out).ao.ao_info_pkt_key_not_found = info.pkt_key_not_found;
    (*out).ao.ao_info_pkt_ao_required = info.pkt_ao_required;
    (*out).ao.ao_info_pkt_dropped_icmp = info.pkt_dropped_icmp;

    /* per-key */
    nr_keys = test_get_ao_keys_nr(sk);
    if nr_keys < 0 {
        return nr_keys;
    }
    if nr_keys == 0 {
        test_error(c"test_get_ao_keys_nr() == 0".as_ptr());
    }
    (*out).ao.nr_keys = nr_keys as usize;
    key_dump = calloc(nr_keys as usize, key_dump_sz as usize) as *mut tcp_ao_getsockopt;
    if key_dump.is_null() {
        return -errno;
    }

    (*key_dump).nkeys = nr_keys as _;
    (*key_dump).get_all = 1;
    err = getsockopt(
        sk,
        IPPROTO_TCP,
        TCP_AO_GET_KEYS,
        key_dump as *mut c_void,
        &mut key_dump_sz,
    );
    if err != 0 {
        free(key_dump as *mut c_void);
        return -errno;
    }

    (*out).ao.key_cnts = calloc(
        nr_keys as usize,
        size_of_val_raw((*out).ao.key_cnts),
    ) as *mut tcp_ao_key_counters;
    if (*out).ao.key_cnts.is_null() {
        free(key_dump as *mut c_void);
        return -errno;
    }

    while nr_keys != 0 {
        nr_keys -= 1;
        (*(*out).ao.key_cnts.add(nr_keys as usize)).sndid = (*key_dump.add(nr_keys as usize)).sndid;
        (*(*out).ao.key_cnts.add(nr_keys as usize)).rcvid = (*key_dump.add(nr_keys as usize)).rcvid;
        (*(*out).ao.key_cnts.add(nr_keys as usize)).pkt_good = (*key_dump.add(nr_keys as usize)).pkt_good;
        (*(*out).ao.key_cnts.add(nr_keys as usize)).pkt_bad = (*key_dump.add(nr_keys as usize)).pkt_bad;
    }
    free(key_dump as *mut c_void);

    0
}

macro_rules! cmp_counter {
    ($before:expr, $after:expr, $ret:ident, $cnt:ident, $e_cnt:expr, $name:literal) => {
        if (*$before).$cnt > (*$after).$cnt {
            test_error(c"counter %s decreased".as_ptr(), concat!($name, "\0").as_ptr());
        }
        if (*$before).$cnt != (*$after).$cnt {
            $ret |= $e_cnt;
        }
    };
}

pub unsafe extern "C" fn test_cmp_counters(
    before: *mut tcp_counters,
    after: *mut tcp_counters,
) -> test_cnt {
    let mut ret: test_cnt = 0;
    let mut i: usize;

    if (*before).ao.nr_keys != (*after).ao.nr_keys {
        test_error(c"the number of keys has changed".as_ptr());
    }

    cmp_counter!(before, after, ret, netns_md5_notfound, TEST_CNT_NS_MD5_NOTFOUND, "netns_md5_notfound");
    cmp_counter!(before, after, ret, netns_md5_unexpected, TEST_CNT_NS_MD5_UNEXPECTED, "netns_md5_unexpected");
    cmp_counter!(before, after, ret, netns_md5_failure, TEST_CNT_NS_MD5_FAILURE, "netns_md5_failure");
    cmp_ao_counter!((*before).ao, (*after).ao, ret);

    i = (*before).ao.nr_keys;
    while i != 0 {
        i -= 1;
        if (*(*before).ao.key_cnts.add(i)).pkt_good > (*(*after).ao.key_cnts.add(i)).pkt_good {
            test_error(c"counter ao.key_cnts[i].pkt_good decreased".as_ptr());
        }
        if (*(*before).ao.key_cnts.add(i)).pkt_good != (*(*after).ao.key_cnts.add(i)).pkt_good {
            ret |= TEST_CNT_KEY_GOOD;
        }
        if (*(*before).ao.key_cnts.add(i)).pkt_bad > (*(*after).ao.key_cnts.add(i)).pkt_bad {
            test_error(c"counter ao.key_cnts[i].pkt_bad decreased".as_ptr());
        }
        if (*(*before).ao.key_cnts.add(i)).pkt_bad != (*(*after).ao.key_cnts.add(i)).pkt_bad {
            ret |= TEST_CNT_KEY_BAD;
        }
    }
    ret
}

pub unsafe extern "C" fn test_assert_counters_sk(
    tst_name: *const c_char,
    before: *mut tcp_counters,
    after: *mut tcp_counters,
    expected: test_cnt,
) -> c_int {
    errno = 0;
    assert_counter!(tst_name, before, after, expected, netns_md5_notfound, TEST_CNT_NS_MD5_NOTFOUND, "netns_md5_notfound");
    assert_counter!(tst_name, before, after, expected, netns_md5_unexpected, TEST_CNT_NS_MD5_UNEXPECTED, "netns_md5_unexpected");
    assert_counter!(tst_name, before, after, expected, netns_md5_failure, TEST_CNT_NS_MD5_FAILURE, "netns_md5_failure");
    assert_ao_counters!(tst_name, (*before).ao, (*after).ao, expected);
    0
}

pub unsafe extern "C" fn test_assert_counters_key(
    tst_name: *const c_char,
    before: *mut tcp_ao_counters,
    after: *mut tcp_ao_counters,
    expected: test_cnt,
    sndid: c_int,
    rcvid: c_int,
) -> c_int {
    let mut i: usize;

    if (*before).nr_keys != (*after).nr_keys {
        test_fail(c"%s: Keys changed on the socket %zu != %zu".as_ptr(), tst_name, (*before).nr_keys, (*after).nr_keys);
        return -1;
    }

    /* per-key */
    i = (*before).nr_keys;
    while i != 0 {
        i -= 1;
        if sndid >= 0 && (*(*before).key_cnts.add(i)).sndid as c_int != sndid {
            continue;
        }
        if rcvid >= 0 && (*(*before).key_cnts.add(i)).rcvid as c_int != rcvid {
            continue;
        }
        assert_key_counter!(tst_name, before, after, expected, i, pkt_good, TEST_CNT_KEY_GOOD, "pkt_good");
        assert_key_counter!(tst_name, before, after, expected, i, pkt_bad, TEST_CNT_KEY_BAD, "pkt_bad");
    }
    0
}

pub unsafe extern "C" fn test_tcp_counters_free(cnts: *mut tcp_counters) {
    free((*cnts).ao.key_cnts as *mut c_void);
}

unsafe fn _test_server_run(
    sk: c_int,
    quota: ssize_t,
    c: *mut tcp_counters,
    cond: test_cnt,
    err: *mut c_int,
    timeout_sec: time_t,
) -> ssize_t {
    let mut total: ssize_t = 0;

    loop {
        let mut buf = [0u8; TEST_BUF_SIZE];
        let bytes: ssize_t;
        let sent: ssize_t;
        let mut ret: c_int;

        ret = __test_skpair_poll(sk, false, timeout_sec as u64, c, cond, err);
        if ret != 0 {
            return ret as ssize_t;
        }

        bytes = recv(sk, buf.as_mut_ptr() as *mut c_void, buf.len(), 0);

        if bytes < 0 {
            test_error(c"recv(): %zd".as_ptr(), bytes);
        }
        if bytes == 0 {
            break;
        }

        ret = __test_skpair_poll(sk, true, timeout_sec as u64, c, cond, err);
        if ret != 0 {
            return ret as ssize_t;
        }

        sent = send(sk, buf.as_ptr() as *const c_void, bytes as usize, 0);
        if sent == 0 {
            break;
        }
        if sent != bytes {
            test_error(c"send()".as_ptr());
        }
        total += bytes;
        if quota != 0 && total >= quota {
            break;
        }
    }

    total
}

pub unsafe extern "C" fn test_server_run(
    sk: c_int,
    quota: ssize_t,
    timeout_sec: time_t,
) -> ssize_t {
    _test_server_run(
        sk,
        quota,
        null_mut(),
        0,
        null_mut(),
        if timeout_sec != 0 {
            timeout_sec
        } else {
            TEST_TIMEOUT_SEC as time_t
        },
    )
}

pub unsafe extern "C" fn test_skpair_server(
    sk: c_int,
    quota: ssize_t,
    cond: test_cnt,
    err: *mut c_int,
) -> c_int {
    let mut c: tcp_counters = zeroed();
    let ret: ssize_t;

    core::ptr::write_volatile(err, 0);
    if test_get_tcp_counters(sk, &mut c) != 0 {
        test_error(c"test_get_tcp_counters()".as_ptr());
    }
    synchronize_threads(); /* 1: init skpair & read nscounters */

    ret = _test_server_run(sk, quota, &mut c, cond, err, TEST_TIMEOUT_SEC as time_t);
    test_tcp_counters_free(&mut c);
    ret as c_int
}

unsafe fn test_client_loop(
    sk: c_int,
    buf_sz: usize,
    msg_len: usize,
    c: *mut tcp_counters,
    cond: test_cnt,
    err: *mut c_int,
) -> ssize_t {
    let msg = alloca(msg_len) as *mut c_char;
    let mut nodelay: c_int = 1;
    let buf: *mut c_char;
    let mut i: usize;

    buf = alloca(buf_sz) as *mut c_char;
    if buf.is_null() {
        return -ENOMEM as ssize_t;
    }
    randomize_buffer(buf, buf_sz);

    if setsockopt(
        sk,
        IPPROTO_TCP,
        TCP_NODELAY,
        &mut nodelay as *mut _ as *const c_void,
        size_of::<c_int>() as socklen_t,
    ) != 0
    {
        test_error(c"setsockopt(TCP_NODELAY)".as_ptr());
    }

    i = 0;
    while i < buf_sz {
        let mut bytes = core::cmp::min(msg_len, buf_sz - i);
        let sent: ssize_t;
        let mut ret: c_int;

        ret = __test_skpair_poll(sk, true, TEST_TIMEOUT_SEC as u64, c, cond, err);
        if ret != 0 {
            return ret as ssize_t;
        }

        sent = send(sk, buf.add(i) as *const c_void, bytes, 0);
        if sent == 0 {
            break;
        }
        if sent != bytes as ssize_t {
            test_error(c"send()".as_ptr());
        }

        bytes = 0;
        loop {
            let got: ssize_t;

            ret = __test_skpair_poll(sk, false, TEST_TIMEOUT_SEC as u64, c, cond, err);
            if ret != 0 {
                return ret as ssize_t;
            }

            got = recv(
                sk,
                msg.add(bytes) as *mut c_void,
                msg_len - bytes,
                0,
            );
            if got <= 0 {
                return i as ssize_t;
            }
            bytes += got as usize;
            if bytes >= sent as usize {
                break;
            }
        }
        if bytes > sent as usize {
            test_error(c"recv(): %zd > %zd".as_ptr(), bytes as ssize_t, sent);
        }
        if memcmp(buf.add(i) as *const c_void, msg as *const c_void, bytes) != 0 {
            test_fail(c"received message differs".as_ptr());
            return -1;
        }
        i += core::cmp::min(msg_len, buf_sz - i);
    }
    i as ssize_t
}

pub unsafe extern "C" fn test_client_verify(
    sk: c_int,
    msg_len: usize,
    nr: usize,
) -> c_int {
    let buf_sz = msg_len * nr;
    let ret: ssize_t;

    ret = test_client_loop(sk, buf_sz, msg_len, null_mut(), 0, null_mut());
    if ret < 0 {
        return ret as c_int;
    }
    if ret != buf_sz as ssize_t { -1 } else { 0 }
}

pub unsafe extern "C" fn test_skpair_client(
    sk: c_int,
    msg_len: usize,
    nr: usize,
    cond: test_cnt,
    err: *mut c_int,
) -> c_int {
    let mut c: tcp_counters = zeroed();
    let buf_sz = msg_len * nr;
    let ret: ssize_t;

    core::ptr::write_volatile(err, 0);
    if test_get_tcp_counters(sk, &mut c) != 0 {
        test_error(c"test_get_tcp_counters()".as_ptr());
    }
    synchronize_threads(); /* 1: init skpair & read nscounters */

    ret = test_client_loop(sk, buf_sz, msg_len, &mut c, cond, err);
    test_tcp_counters_free(&mut c);
    if ret < 0 {
        return ret as c_int;
    }
    if ret != buf_sz as ssize_t { -1 } else { 0 }
}
