// SPDX-License-Identifier: GPL-2.0
/* Author: Dmitry Safonov <dima@arista.com> */
/* Translated from testing/selftests/net/tcp_ao/setsockopt-closed.c. */
/* Dependencies from "../../../../include/linux/kernel.h" and "aolib.h". */

use core::ffi::{c_char, c_int, c_void};
use core::mem::{self, size_of};
use core::ptr;

static mut tcp_md5_client: tcp_addr = unsafe { mem::zeroed() };

const FILTER_TEST_NKEYS: usize = 16;

static mut test_port: c_int = 7788;

unsafe fn make_listen(sk: c_int) {
    let mut addr: sockaddr_af = mem::zeroed();

    tcp_addr_to_sockaddr_in(
        &mut addr,
        &this_ip_addr,
        htons({
            let port = test_port;
            test_port += 1;
            port as u16
        }),
    );
    if bind(
        sk,
        &mut addr as *mut sockaddr_af as *mut sockaddr,
        size_of::<sockaddr_af>() as socklen_t,
    ) < 0
    {
        test_error!("bind()");
    }
    if listen(sk, 1) != 0 {
        test_error!("listen()");
    }
}

unsafe fn test_vefify_ao_info(sk: c_int, info: *mut tcp_ao_info_opt, tst: *const c_char) {
    let mut tmp: tcp_ao_info_opt = mem::zeroed();
    let mut len: socklen_t = size_of::<tcp_ao_info_opt>() as socklen_t;

    if getsockopt(
        sk,
        IPPROTO_TCP,
        TCP_AO_INFO,
        &mut tmp as *mut tcp_ao_info_opt as *mut c_void,
        &mut len,
    ) != 0
    {
        test_error!("getsockopt(TCP_AO_INFO) failed");
    }

    macro_rules! cmp_ao {
        ($member:ident) => {
            if (*info).$member != tmp.$member {
                test_fail!(
                    "%s: getsockopt(): " stringify!($member) " %" PRIu64 " != %" PRIu64,
                    tst,
                    (*info).$member as u64,
                    tmp.$member as u64
                );
                return;
            }
        };
    }

    if (*info).set_current != 0 {
        cmp_ao!(current_key);
    }
    if (*info).set_rnext != 0 {
        cmp_ao!(rnext);
    }
    if (*info).set_counters != 0 {
        cmp_ao!(pkt_good);
        cmp_ao!(pkt_bad);
        cmp_ao!(pkt_key_not_found);
        cmp_ao!(pkt_ao_required);
        cmp_ao!(pkt_dropped_icmp);
    }
    cmp_ao!(ao_required);
    cmp_ao!(accept_icmps);

    test_ok!("AO info get: %s", tst);
}

unsafe fn __setsockopt_checked(
    sk: c_int,
    optname: c_int,
    get: bool,
    optval: *mut c_void,
    len: *mut socklen_t,
    err: c_int,
    mut tst: *const c_char,
    mut tst2: *const c_char,
) {
    let ret: c_int;

    if tst.is_null() {
        tst = c"".as_ptr();
    }
    if tst2.is_null() {
        tst2 = c"".as_ptr();
    }

    errno = 0;
    if get {
        ret = getsockopt(sk, IPPROTO_TCP, optname, optval, len);
    } else {
        ret = setsockopt(sk, IPPROTO_TCP, optname, optval, *len);
    }
    if ret == -1 {
        if errno == err {
            test_ok!("%s%s", tst, tst2);
        } else {
            test_fail!(
                "%s%s: %setsockopt() failed",
                tst,
                tst2,
                if get { c"g".as_ptr() } else { c"s".as_ptr() }
            );
        }
        close(sk);
        return;
    }

    if err != 0 {
        test_fail!(
            "%s%s: %setsockopt() was expected to fail with %d",
            tst,
            tst2,
            if get { c"g".as_ptr() } else { c"s".as_ptr() },
            err
        );
    } else {
        test_ok!("%s%s", tst, tst2);
        if optname == TCP_AO_ADD_KEY {
            test_verify_socket_key(sk, optval);
        } else if optname == TCP_AO_INFO && !get {
            test_vefify_ao_info(sk, optval as *mut tcp_ao_info_opt, tst2);
        } else if optname == TCP_AO_GET_KEYS {
            if *len != size_of::<tcp_ao_getsockopt>() as socklen_t {
                test_fail!("%s%s: get keys returned wrong tcp_ao_getsockopt size", tst, tst2);
            }
        }
    }
    close(sk);
}

unsafe fn setsockopt_checked(
    sk: c_int,
    optname: c_int,
    optval: *mut c_void,
    err: c_int,
    tst: *const c_char,
) {
    let mut cmd: *const c_char = ptr::null();
    let mut len: socklen_t = 0;

    match optname {
        TCP_AO_ADD_KEY => {
            cmd = c"key add: ".as_ptr();
            len = size_of::<tcp_ao_add>() as socklen_t;
        }
        TCP_AO_DEL_KEY => {
            cmd = c"key del: ".as_ptr();
            len = size_of::<tcp_ao_del>() as socklen_t;
        }
        TCP_AO_INFO => {
            cmd = c"AO info set: ".as_ptr();
            len = size_of::<tcp_ao_info_opt>() as socklen_t;
        }
        _ => {}
    }

    __setsockopt_checked(sk, optname, false, optval, &mut len, err, cmd, tst);
}

unsafe fn prepare_defs(cmd: c_int, optval: *mut c_void) -> c_int {
    let sk = socket(test_family, SOCK_STREAM, IPPROTO_TCP);

    if sk < 0 {
        test_error!("socket()");
    }

    match cmd {
        TCP_AO_ADD_KEY => {
            let add = optval as *mut tcp_ao_add;

            if test_prepare_def_key(
                add,
                DEFAULT_TEST_PASSWORD,
                0,
                this_ip_dest,
                -1,
                0,
                100,
                100,
            ) != 0
            {
                test_error!("prepare default tcp_ao_add");
            }
        }
        TCP_AO_DEL_KEY => {
            let del = optval as *mut tcp_ao_del;

            if test_add_key(
                sk,
                DEFAULT_TEST_PASSWORD,
                this_ip_dest,
                DEFAULT_TEST_PREFIX,
                100,
                100,
            ) != 0
            {
                test_error!("add default key");
            }
            ptr::write_bytes(del, 0, 1);
            (*del).sndid = 100;
            (*del).rcvid = 100;
            (*del).prefix = DEFAULT_TEST_PREFIX;
            tcp_addr_to_sockaddr_in(&mut (*del).addr, &this_ip_dest, 0);
        }
        TCP_AO_INFO => {
            let info = optval as *mut tcp_ao_info_opt;

            if test_add_key(
                sk,
                DEFAULT_TEST_PASSWORD,
                this_ip_dest,
                DEFAULT_TEST_PREFIX,
                100,
                100,
            ) != 0
            {
                test_error!("add default key");
            }
            ptr::write_bytes(info, 0, 1);
        }
        TCP_AO_GET_KEYS => {
            let get = optval as *mut tcp_ao_getsockopt;

            if test_add_key(
                sk,
                DEFAULT_TEST_PASSWORD,
                this_ip_dest,
                DEFAULT_TEST_PREFIX,
                100,
                100,
            ) != 0
            {
                test_error!("add default key");
            }
            ptr::write_bytes(get, 0, 1);
            (*get).nkeys = 1;
            (*get).get_all = 1;
        }
        _ => test_error!("unknown cmd"),
    }

    sk
}

#[repr(C)]
union test_extend_opt_u {
    add: tcp_ao_add,
    del: tcp_ao_del,
    get: tcp_ao_getsockopt,
    info: tcp_ao_info_opt,
}

#[repr(C)]
struct test_extend_opt {
    u: test_extend_opt_u,
    extend: [*mut c_char; 100],
}

unsafe fn test_extend(cmd: c_int, get: bool, tst: *const c_char, mut under_size: socklen_t) {
    let mut tmp_opt: test_extend_opt = mem::zeroed();
    let mut extended_size: socklen_t = size_of::<test_extend_opt>() as socklen_t;
    let mut sk: c_int;

    ptr::write_bytes(&mut tmp_opt, 0, 1);
    sk = prepare_defs(cmd, &mut tmp_opt as *mut test_extend_opt as *mut c_void);
    __setsockopt_checked(
        sk,
        cmd,
        get,
        &mut tmp_opt as *mut test_extend_opt as *mut c_void,
        &mut under_size,
        EINVAL,
        tst,
        c": minimum size".as_ptr(),
    );

    ptr::write_bytes(&mut tmp_opt, 0, 1);
    sk = prepare_defs(cmd, &mut tmp_opt as *mut test_extend_opt as *mut c_void);
    __setsockopt_checked(
        sk,
        cmd,
        get,
        &mut tmp_opt as *mut test_extend_opt as *mut c_void,
        &mut extended_size,
        0,
        tst,
        c": extended size".as_ptr(),
    );

    ptr::write_bytes(&mut tmp_opt, 0, 1);
    sk = prepare_defs(cmd, &mut tmp_opt as *mut test_extend_opt as *mut c_void);
    __setsockopt_checked(
        sk,
        cmd,
        get,
        ptr::null_mut(),
        &mut extended_size,
        EFAULT,
        tst,
        c": null optval".as_ptr(),
    );

    if get {
        ptr::write_bytes(&mut tmp_opt, 0, 1);
        sk = prepare_defs(cmd, &mut tmp_opt as *mut test_extend_opt as *mut c_void);
        __setsockopt_checked(
            sk,
            cmd,
            get,
            &mut tmp_opt as *mut test_extend_opt as *mut c_void,
            ptr::null_mut(),
            EFAULT,
            tst,
            c": null optlen".as_ptr(),
        );
    }
}

unsafe fn extend_tests() {
    test_extend(
        TCP_AO_ADD_KEY,
        false,
        c"AO add".as_ptr(),
        offset_of!(tcp_ao_add, key) as socklen_t,
    );
    test_extend(
        TCP_AO_DEL_KEY,
        false,
        c"AO del".as_ptr(),
        offset_of!(tcp_ao_del, keyflags) as socklen_t,
    );
    test_extend(
        TCP_AO_INFO,
        false,
        c"AO set info".as_ptr(),
        offset_of!(tcp_ao_info_opt, pkt_dropped_icmp) as socklen_t,
    );
    test_extend(TCP_AO_INFO, true, c"AO get info".as_ptr(), -1i32 as socklen_t);
    test_extend(TCP_AO_GET_KEYS, true, c"AO get keys".as_ptr(), -1i32 as socklen_t);
}

unsafe fn test_optmem_limit() {
    let mut i: usize;
    let keys_limit: usize;
    let current_optmem: usize = test_get_optmem();
    let mut ao: tcp_ao_add = mem::zeroed();
    let mut net: tcp_addr = mem::zeroed();
    let sk: c_int;

    if inet_pton(TEST_FAMILY, TEST_NETWORK, &mut net as *mut tcp_addr as *mut c_void) != 1 {
        test_error!("Can't convert ip address %s", TEST_NETWORK);
    }

    sk = prepare_defs(TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void);
    keys_limit = current_optmem / KERNEL_TCP_AO_KEY_SZ_ROUND_UP;
    i = 0;
    loop {
        let mut key_peer: tcp_addr;
        let err: c_int;

        key_peer = gen_tcp_addr(net, i + 1);
        tcp_addr_to_sockaddr_in(&mut ao.addr, &key_peer, 0);
        err = setsockopt(
            sk,
            IPPROTO_TCP,
            TCP_AO_ADD_KEY,
            &mut ao as *mut tcp_ao_add as *mut c_void,
            size_of::<tcp_ao_add>() as socklen_t,
        );
        if err == 0 {
            /*
             * TCP_AO_ADD_KEY should be the same order as the real
             * sizeof(struct tcp_ao_key) in kernel.
             */
            if i <= keys_limit * 10 {
                i += 1;
                continue;
            }
            test_fail!("optmem limit test failed: added %zu key", i);
            break;
        }
        if i < keys_limit {
            test_fail!("optmem limit test failed: couldn't add %zu key", i);
            break;
        }
        test_ok!("optmem limit was hit on adding %zu key", i);
        break;
    }
    close(sk);
}

unsafe fn test_einval_add_key() {
    let mut ao: tcp_ao_add = mem::zeroed();
    let mut sk: c_int;

    sk = prepare_defs(TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void);
    ao.keylen = TCP_AO_MAXKEYLEN + 1;
    setsockopt_checked(sk, TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void, EINVAL, c"too big keylen".as_ptr());

    sk = prepare_defs(TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void);
    ao.reserved = 1;
    setsockopt_checked(sk, TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void, EINVAL, c"using reserved padding".as_ptr());

    sk = prepare_defs(TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void);
    ao.reserved2 = 1;
    setsockopt_checked(sk, TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void, EINVAL, c"using reserved2 padding".as_ptr());

    /* tcp_ao_verify_ipv{4,6}() checks */
    sk = prepare_defs(TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void);
    ao.addr.ss_family = AF_UNIX;
    memcpy(&mut ao.addr as *mut _ as *mut c_void, &SOCKADDR_ANY as *const _ as *const c_void, size_of_val(&SOCKADDR_ANY));
    setsockopt_checked(sk, TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void, EINVAL, c"wrong address family".as_ptr());

    sk = prepare_defs(TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void);
    tcp_addr_to_sockaddr_in(&mut ao.addr, &this_ip_dest, 1234);
    setsockopt_checked(sk, TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void, EINVAL, c"port (unsupported)".as_ptr());

    sk = prepare_defs(TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void);
    ao.prefix = 0;
    setsockopt_checked(sk, TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void, EINVAL, c"no prefix, addr".as_ptr());

    sk = prepare_defs(TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void);
    ao.prefix = 0;
    memcpy(&mut ao.addr as *mut _ as *mut c_void, &SOCKADDR_ANY as *const _ as *const c_void, size_of_val(&SOCKADDR_ANY));
    setsockopt_checked(sk, TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void, 0, c"no prefix, any addr".as_ptr());

    sk = prepare_defs(TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void);
    ao.prefix = 32;
    memcpy(&mut ao.addr as *mut _ as *mut c_void, &SOCKADDR_ANY as *const _ as *const c_void, size_of_val(&SOCKADDR_ANY));
    setsockopt_checked(sk, TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void, EINVAL, c"prefix, any addr".as_ptr());

    sk = prepare_defs(TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void);
    ao.prefix = 129;
    setsockopt_checked(sk, TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void, EINVAL, c"too big prefix".as_ptr());

    sk = prepare_defs(TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void);
    ao.prefix = 2;
    setsockopt_checked(sk, TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void, EINVAL, c"too short prefix".as_ptr());

    sk = prepare_defs(TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void);
    ao.keyflags = (-1i8) as u8;
    setsockopt_checked(sk, TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void, EINVAL, c"bad key flags".as_ptr());

    sk = prepare_defs(TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void);
    make_listen(sk);
    ao.set_current = 1;
    setsockopt_checked(sk, TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void, EINVAL, c"add current key on a listen socket".as_ptr());

    sk = prepare_defs(TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void);
    make_listen(sk);
    ao.set_rnext = 1;
    setsockopt_checked(sk, TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void, EINVAL, c"add rnext key on a listen socket".as_ptr());

    sk = prepare_defs(TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void);
    make_listen(sk);
    ao.set_current = 1;
    ao.set_rnext = 1;
    setsockopt_checked(sk, TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void, EINVAL, c"add current+rnext key on a listen socket".as_ptr());

    sk = prepare_defs(TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void);
    ao.set_current = 1;
    setsockopt_checked(sk, TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void, 0, c"add key and set as current".as_ptr());

    sk = prepare_defs(TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void);
    ao.set_rnext = 1;
    setsockopt_checked(sk, TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void, 0, c"add key and set as rnext".as_ptr());

    sk = prepare_defs(TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void);
    ao.set_current = 1;
    ao.set_rnext = 1;
    setsockopt_checked(sk, TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void, 0, c"add key and set as current+rnext".as_ptr());

    sk = prepare_defs(TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void);
    ao.ifindex = 42;
    setsockopt_checked(sk, TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void, EINVAL, c"ifindex without TCP_AO_KEYF_IFNINDEX".as_ptr());

    sk = prepare_defs(TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void);
    ao.keyflags |= TCP_AO_KEYF_IFINDEX;
    ao.ifindex = 42;
    setsockopt_checked(sk, TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void, EINVAL, c"non-existent VRF".as_ptr());
    /*
     * tcp_md5_do_lookup{,_any_l3index}() are checked in unsigned-md5
     * see client_vrf_tests().
     */

    test_optmem_limit();

    /* tcp_ao_parse_crypto() */
    sk = prepare_defs(TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void);
    ao.maclen = 100;
    setsockopt_checked(sk, TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void, EMSGSIZE, c"maclen bigger than TCP hdr".as_ptr());

    sk = prepare_defs(TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void);
    strcpy(ao.alg_name.as_mut_ptr(), c"imaginary hash algo".as_ptr());
    setsockopt_checked(sk, TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void, ENOENT, c"bad algo".as_ptr());
}

unsafe fn test_einval_del_key() {
    let mut del: tcp_ao_del = mem::zeroed();
    let mut sk: c_int;

    sk = prepare_defs(TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void);
    del.reserved = 1;
    setsockopt_checked(sk, TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void, EINVAL, c"using reserved padding".as_ptr());

    sk = prepare_defs(TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void);
    del.reserved2 = 1;
    setsockopt_checked(sk, TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void, EINVAL, c"using reserved2 padding".as_ptr());

    sk = prepare_defs(TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void);
    make_listen(sk);
    if test_add_key(sk, DEFAULT_TEST_PASSWORD, this_ip_dest, DEFAULT_TEST_PREFIX, 0, 0) != 0 { test_error!("add key"); }
    del.set_current = 1;
    setsockopt_checked(sk, TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void, EINVAL, c"del and set current key on a listen socket".as_ptr());

    sk = prepare_defs(TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void);
    make_listen(sk);
    if test_add_key(sk, DEFAULT_TEST_PASSWORD, this_ip_dest, DEFAULT_TEST_PREFIX, 0, 0) != 0 { test_error!("add key"); }
    del.set_rnext = 1;
    setsockopt_checked(sk, TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void, EINVAL, c"del and set rnext key on a listen socket".as_ptr());

    sk = prepare_defs(TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void);
    make_listen(sk);
    if test_add_key(sk, DEFAULT_TEST_PASSWORD, this_ip_dest, DEFAULT_TEST_PREFIX, 0, 0) != 0 { test_error!("add key"); }
    del.set_current = 1;
    del.set_rnext = 1;
    setsockopt_checked(sk, TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void, EINVAL, c"del and set current+rnext key on a listen socket".as_ptr());

    sk = prepare_defs(TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void);
    del.keyflags = (-1i8) as u8;
    setsockopt_checked(sk, TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void, EINVAL, c"bad key flags".as_ptr());

    sk = prepare_defs(TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void);
    del.ifindex = 42;
    setsockopt_checked(sk, TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void, EINVAL, c"ifindex without TCP_AO_KEYF_IFNINDEX".as_ptr());

    sk = prepare_defs(TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void);
    del.keyflags |= TCP_AO_KEYF_IFINDEX;
    del.ifindex = 42;
    setsockopt_checked(sk, TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void, ENOENT, c"non-existent VRF".as_ptr());

    sk = prepare_defs(TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void);
    del.set_current = 1;
    setsockopt_checked(sk, TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void, ENOENT, c"set non-existing current key".as_ptr());

    sk = prepare_defs(TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void);
    del.set_rnext = 1;
    setsockopt_checked(sk, TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void, ENOENT, c"set non-existing rnext key".as_ptr());

    sk = prepare_defs(TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void);
    del.set_current = 1;
    del.set_rnext = 1;
    setsockopt_checked(sk, TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void, ENOENT, c"set non-existing current+rnext key".as_ptr());

    sk = prepare_defs(TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void);
    if test_add_key(sk, DEFAULT_TEST_PASSWORD, this_ip_dest, DEFAULT_TEST_PREFIX, 0, 0) != 0 { test_error!("add key"); }
    del.set_current = 1;
    setsockopt_checked(sk, TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void, 0, c"set current key".as_ptr());

    sk = prepare_defs(TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void);
    if test_add_key(sk, DEFAULT_TEST_PASSWORD, this_ip_dest, DEFAULT_TEST_PREFIX, 0, 0) != 0 { test_error!("add key"); }
    del.set_rnext = 1;
    setsockopt_checked(sk, TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void, 0, c"set rnext key".as_ptr());

    sk = prepare_defs(TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void);
    if test_add_key(sk, DEFAULT_TEST_PASSWORD, this_ip_dest, DEFAULT_TEST_PREFIX, 0, 0) != 0 { test_error!("add key"); }
    del.set_current = 1;
    del.set_rnext = 1;
    setsockopt_checked(sk, TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void, 0, c"set current+rnext key".as_ptr());

    sk = prepare_defs(TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void);
    del.set_current = 1;
    del.current_key = 100;
    setsockopt_checked(sk, TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void, ENOENT, c"set as current key to be removed".as_ptr());

    sk = prepare_defs(TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void);
    del.set_rnext = 1;
    del.rnext = 100;
    setsockopt_checked(sk, TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void, ENOENT, c"set as rnext key to be removed".as_ptr());

    sk = prepare_defs(TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void);
    del.set_current = 1;
    del.current_key = 100;
    del.set_rnext = 1;
    del.rnext = 100;
    setsockopt_checked(sk, TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void, ENOENT, c"set as current+rnext key to be removed".as_ptr());

    sk = prepare_defs(TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void);
    del.del_async = 1;
    setsockopt_checked(sk, TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void, EINVAL, c"async on non-listen".as_ptr());

    sk = prepare_defs(TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void);
    del.sndid = 101;
    setsockopt_checked(sk, TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void, ENOENT, c"non-existing sndid".as_ptr());

    sk = prepare_defs(TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void);
    del.rcvid = 101;
    setsockopt_checked(sk, TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void, ENOENT, c"non-existing rcvid".as_ptr());

    sk = prepare_defs(TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void);
    tcp_addr_to_sockaddr_in(&mut del.addr, &this_ip_addr, 0);
    setsockopt_checked(sk, TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void, ENOENT, c"incorrect addr".as_ptr());

    sk = prepare_defs(TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void);
    setsockopt_checked(sk, TCP_AO_DEL_KEY, &mut del as *mut tcp_ao_del as *mut c_void, 0, c"correct key delete".as_ptr());
}

unsafe fn test_einval_ao_info() {
    let mut info: tcp_ao_info_opt = mem::zeroed();
    let mut sk: c_int;

    sk = prepare_defs(TCP_AO_INFO, &mut info as *mut tcp_ao_info_opt as *mut c_void);
    make_listen(sk);
    info.set_current = 1;
    setsockopt_checked(sk, TCP_AO_INFO, &mut info as *mut tcp_ao_info_opt as *mut c_void, EINVAL, c"set current key on a listen socket".as_ptr());

    sk = prepare_defs(TCP_AO_INFO, &mut info as *mut tcp_ao_info_opt as *mut c_void);
    make_listen(sk);
    info.set_rnext = 1;
    setsockopt_checked(sk, TCP_AO_INFO, &mut info as *mut tcp_ao_info_opt as *mut c_void, EINVAL, c"set rnext key on a listen socket".as_ptr());

    sk = prepare_defs(TCP_AO_INFO, &mut info as *mut tcp_ao_info_opt as *mut c_void);
    make_listen(sk);
    info.set_current = 1;
    info.set_rnext = 1;
    setsockopt_checked(sk, TCP_AO_INFO, &mut info as *mut tcp_ao_info_opt as *mut c_void, EINVAL, c"set current+rnext key on a listen socket".as_ptr());

    sk = prepare_defs(TCP_AO_INFO, &mut info as *mut tcp_ao_info_opt as *mut c_void);
    info.reserved = 1;
    setsockopt_checked(sk, TCP_AO_INFO, &mut info as *mut tcp_ao_info_opt as *mut c_void, EINVAL, c"using reserved padding".as_ptr());

    sk = prepare_defs(TCP_AO_INFO, &mut info as *mut tcp_ao_info_opt as *mut c_void);
    info.reserved2 = 1;
    setsockopt_checked(sk, TCP_AO_INFO, &mut info as *mut tcp_ao_info_opt as *mut c_void, EINVAL, c"using reserved2 padding".as_ptr());

    sk = prepare_defs(TCP_AO_INFO, &mut info as *mut tcp_ao_info_opt as *mut c_void);
    info.accept_icmps = 1;
    setsockopt_checked(sk, TCP_AO_INFO, &mut info as *mut tcp_ao_info_opt as *mut c_void, 0, c"accept_icmps".as_ptr());

    sk = prepare_defs(TCP_AO_INFO, &mut info as *mut tcp_ao_info_opt as *mut c_void);
    info.ao_required = 1;
    setsockopt_checked(sk, TCP_AO_INFO, &mut info as *mut tcp_ao_info_opt as *mut c_void, 0, c"ao required".as_ptr());

    if !should_skip_test(c"ao required with MD5 key".as_ptr(), KCONFIG_TCP_MD5) {
        sk = prepare_defs(TCP_AO_INFO, &mut info as *mut tcp_ao_info_opt as *mut c_void);
        info.ao_required = 1;
        if test_set_md5(sk, tcp_md5_client, TEST_PREFIX, -1, c"long long secret".as_ptr()) != 0 {
            test_error!("setsockopt(TCP_MD5SIG_EXT)");
            close(sk);
        } else {
            setsockopt_checked(sk, TCP_AO_INFO, &mut info as *mut tcp_ao_info_opt as *mut c_void, EKEYREJECTED, c"ao required with MD5 key".as_ptr());
        }
    }

    sk = prepare_defs(TCP_AO_INFO, &mut info as *mut tcp_ao_info_opt as *mut c_void);
    info.set_current = 1;
    setsockopt_checked(sk, TCP_AO_INFO, &mut info as *mut tcp_ao_info_opt as *mut c_void, ENOENT, c"set non-existing current key".as_ptr());

    sk = prepare_defs(TCP_AO_INFO, &mut info as *mut tcp_ao_info_opt as *mut c_void);
    info.set_rnext = 1;
    setsockopt_checked(sk, TCP_AO_INFO, &mut info as *mut tcp_ao_info_opt as *mut c_void, ENOENT, c"set non-existing rnext key".as_ptr());

    sk = prepare_defs(TCP_AO_INFO, &mut info as *mut tcp_ao_info_opt as *mut c_void);
    info.set_current = 1;
    info.set_rnext = 1;
    setsockopt_checked(sk, TCP_AO_INFO, &mut info as *mut tcp_ao_info_opt as *mut c_void, ENOENT, c"set non-existing current+rnext key".as_ptr());

    sk = prepare_defs(TCP_AO_INFO, &mut info as *mut tcp_ao_info_opt as *mut c_void);
    info.set_current = 1;
    info.current_key = 100;
    setsockopt_checked(sk, TCP_AO_INFO, &mut info as *mut tcp_ao_info_opt as *mut c_void, 0, c"set current key".as_ptr());

    sk = prepare_defs(TCP_AO_INFO, &mut info as *mut tcp_ao_info_opt as *mut c_void);
    info.set_rnext = 1;
    info.rnext = 100;
    setsockopt_checked(sk, TCP_AO_INFO, &mut info as *mut tcp_ao_info_opt as *mut c_void, 0, c"set rnext key".as_ptr());

    sk = prepare_defs(TCP_AO_INFO, &mut info as *mut tcp_ao_info_opt as *mut c_void);
    info.set_current = 1;
    info.set_rnext = 1;
    info.current_key = 100;
    info.rnext = 100;
    setsockopt_checked(sk, TCP_AO_INFO, &mut info as *mut tcp_ao_info_opt as *mut c_void, 0, c"set current+rnext key".as_ptr());

    sk = prepare_defs(TCP_AO_INFO, &mut info as *mut tcp_ao_info_opt as *mut c_void);
    info.set_counters = 1;
    info.pkt_good = 321;
    info.pkt_bad = 888;
    info.pkt_key_not_found = 654;
    info.pkt_ao_required = 987654;
    info.pkt_dropped_icmp = 10000;
    setsockopt_checked(sk, TCP_AO_INFO, &mut info as *mut tcp_ao_info_opt as *mut c_void, 0, c"set counters".as_ptr());

    sk = prepare_defs(TCP_AO_INFO, &mut info as *mut tcp_ao_info_opt as *mut c_void);
    setsockopt_checked(sk, TCP_AO_INFO, &mut info as *mut tcp_ao_info_opt as *mut c_void, 0, c"no-op".as_ptr());
}

unsafe fn getsockopt_checked(sk: c_int, optval: *mut tcp_ao_getsockopt, err: c_int, tst: *const c_char) {
    let mut len: socklen_t = size_of::<tcp_ao_getsockopt>() as socklen_t;

    __setsockopt_checked(
        sk,
        TCP_AO_GET_KEYS,
        true,
        optval as *mut c_void,
        &mut len,
        err,
        c"get keys: ".as_ptr(),
        tst,
    );
}

unsafe fn test_einval_get_keys() {
    let mut out: tcp_ao_getsockopt = mem::zeroed();
    let mut sk: c_int;

    sk = socket(test_family, SOCK_STREAM, IPPROTO_TCP);
    if sk < 0 {
        test_error!("socket()");
    }
    getsockopt_checked(sk, &mut out, ENOENT, c"no ao_info".as_ptr());

    sk = prepare_defs(TCP_AO_GET_KEYS, &mut out as *mut tcp_ao_getsockopt as *mut c_void);
    getsockopt_checked(sk, &mut out, 0, c"proper tcp_ao_get_mkts()".as_ptr());

    sk = prepare_defs(TCP_AO_GET_KEYS, &mut out as *mut tcp_ao_getsockopt as *mut c_void);
    out.pkt_good = 643;
    getsockopt_checked(sk, &mut out, EINVAL, c"set out-only pkt_good counter".as_ptr());

    sk = prepare_defs(TCP_AO_GET_KEYS, &mut out as *mut tcp_ao_getsockopt as *mut c_void);
    out.pkt_bad = 94;
    getsockopt_checked(sk, &mut out, EINVAL, c"set out-only pkt_bad counter".as_ptr());

    sk = prepare_defs(TCP_AO_GET_KEYS, &mut out as *mut tcp_ao_getsockopt as *mut c_void);
    out.keyflags = (-1i8) as u8;
    getsockopt_checked(sk, &mut out, EINVAL, c"bad keyflags".as_ptr());

    sk = prepare_defs(TCP_AO_GET_KEYS, &mut out as *mut tcp_ao_getsockopt as *mut c_void);
    out.ifindex = 42;
    getsockopt_checked(sk, &mut out, EINVAL, c"ifindex without TCP_AO_KEYF_IFNINDEX".as_ptr());

    sk = prepare_defs(TCP_AO_GET_KEYS, &mut out as *mut tcp_ao_getsockopt as *mut c_void);
    out.reserved = 1;
    getsockopt_checked(sk, &mut out, EINVAL, c"using reserved field".as_ptr());

    sk = prepare_defs(TCP_AO_GET_KEYS, &mut out as *mut tcp_ao_getsockopt as *mut c_void);
    out.get_all = 0;
    out.prefix = 0;
    tcp_addr_to_sockaddr_in(&mut out.addr, &this_ip_dest, 0);
    getsockopt_checked(sk, &mut out, EINVAL, c"no prefix, addr".as_ptr());

    sk = prepare_defs(TCP_AO_GET_KEYS, &mut out as *mut tcp_ao_getsockopt as *mut c_void);
    out.get_all = 0;
    out.prefix = 0;
    memcpy(&mut out.addr as *mut _ as *mut c_void, &SOCKADDR_ANY as *const _ as *const c_void, size_of_val(&SOCKADDR_ANY));
    getsockopt_checked(sk, &mut out, 0, c"no prefix, any addr".as_ptr());

    sk = prepare_defs(TCP_AO_GET_KEYS, &mut out as *mut tcp_ao_getsockopt as *mut c_void);
    out.get_all = 0;
    out.prefix = 32;
    memcpy(&mut out.addr as *mut _ as *mut c_void, &SOCKADDR_ANY as *const _ as *const c_void, size_of_val(&SOCKADDR_ANY));
    getsockopt_checked(sk, &mut out, EINVAL, c"prefix, any addr".as_ptr());

    sk = prepare_defs(TCP_AO_GET_KEYS, &mut out as *mut tcp_ao_getsockopt as *mut c_void);
    out.get_all = 0;
    out.prefix = 129;
    tcp_addr_to_sockaddr_in(&mut out.addr, &this_ip_dest, 0);
    getsockopt_checked(sk, &mut out, EINVAL, c"too big prefix".as_ptr());

    sk = prepare_defs(TCP_AO_GET_KEYS, &mut out as *mut tcp_ao_getsockopt as *mut c_void);
    out.get_all = 0;
    out.prefix = 2;
    tcp_addr_to_sockaddr_in(&mut out.addr, &this_ip_dest, 0);
    getsockopt_checked(sk, &mut out, EINVAL, c"too short prefix".as_ptr());

    sk = prepare_defs(TCP_AO_GET_KEYS, &mut out as *mut tcp_ao_getsockopt as *mut c_void);
    out.get_all = 0;
    out.prefix = DEFAULT_TEST_PREFIX;
    tcp_addr_to_sockaddr_in(&mut out.addr, &this_ip_dest, 0);
    getsockopt_checked(sk, &mut out, 0, c"prefix + addr".as_ptr());

    sk = prepare_defs(TCP_AO_GET_KEYS, &mut out as *mut tcp_ao_getsockopt as *mut c_void);
    out.get_all = 1;
    out.prefix = DEFAULT_TEST_PREFIX;
    getsockopt_checked(sk, &mut out, EINVAL, c"get_all + prefix".as_ptr());

    sk = prepare_defs(TCP_AO_GET_KEYS, &mut out as *mut tcp_ao_getsockopt as *mut c_void);
    out.get_all = 1;
    tcp_addr_to_sockaddr_in(&mut out.addr, &this_ip_dest, 0);
    getsockopt_checked(sk, &mut out, EINVAL, c"get_all + addr".as_ptr());

    sk = prepare_defs(TCP_AO_GET_KEYS, &mut out as *mut tcp_ao_getsockopt as *mut c_void);
    out.get_all = 1;
    out.sndid = 1;
    getsockopt_checked(sk, &mut out, EINVAL, c"get_all + sndid".as_ptr());

    sk = prepare_defs(TCP_AO_GET_KEYS, &mut out as *mut tcp_ao_getsockopt as *mut c_void);
    out.get_all = 1;
    out.rcvid = 1;
    getsockopt_checked(sk, &mut out, EINVAL, c"get_all + rcvid".as_ptr());

    sk = prepare_defs(TCP_AO_GET_KEYS, &mut out as *mut tcp_ao_getsockopt as *mut c_void);
    out.get_all = 0;
    out.is_current = 1;
    out.prefix = DEFAULT_TEST_PREFIX;
    getsockopt_checked(sk, &mut out, EINVAL, c"current + prefix".as_ptr());

    sk = prepare_defs(TCP_AO_GET_KEYS, &mut out as *mut tcp_ao_getsockopt as *mut c_void);
    out.get_all = 0;
    out.is_current = 1;
    tcp_addr_to_sockaddr_in(&mut out.addr, &this_ip_dest, 0);
    getsockopt_checked(sk, &mut out, EINVAL, c"current + addr".as_ptr());

    sk = prepare_defs(TCP_AO_GET_KEYS, &mut out as *mut tcp_ao_getsockopt as *mut c_void);
    out.get_all = 0;
    out.is_current = 1;
    out.sndid = 1;
    getsockopt_checked(sk, &mut out, EINVAL, c"current + sndid".as_ptr());

    sk = prepare_defs(TCP_AO_GET_KEYS, &mut out as *mut tcp_ao_getsockopt as *mut c_void);
    out.get_all = 0;
    out.is_current = 1;
    out.rcvid = 1;
    getsockopt_checked(sk, &mut out, EINVAL, c"current + rcvid".as_ptr());

    sk = prepare_defs(TCP_AO_GET_KEYS, &mut out as *mut tcp_ao_getsockopt as *mut c_void);
    out.get_all = 0;
    out.is_rnext = 1;
    out.prefix = DEFAULT_TEST_PREFIX;
    getsockopt_checked(sk, &mut out, EINVAL, c"rnext + prefix".as_ptr());

    sk = prepare_defs(TCP_AO_GET_KEYS, &mut out as *mut tcp_ao_getsockopt as *mut c_void);
    out.get_all = 0;
    out.is_rnext = 1;
    tcp_addr_to_sockaddr_in(&mut out.addr, &this_ip_dest, 0);
    getsockopt_checked(sk, &mut out, EINVAL, c"rnext + addr".as_ptr());

    sk = prepare_defs(TCP_AO_GET_KEYS, &mut out as *mut tcp_ao_getsockopt as *mut c_void);
    out.get_all = 0;
    out.is_rnext = 1;
    out.sndid = 1;
    getsockopt_checked(sk, &mut out, EINVAL, c"rnext + sndid".as_ptr());

    sk = prepare_defs(TCP_AO_GET_KEYS, &mut out as *mut tcp_ao_getsockopt as *mut c_void);
    out.get_all = 0;
    out.is_rnext = 1;
    out.rcvid = 1;
    getsockopt_checked(sk, &mut out, EINVAL, c"rnext + rcvid".as_ptr());

    sk = prepare_defs(TCP_AO_GET_KEYS, &mut out as *mut tcp_ao_getsockopt as *mut c_void);
    out.get_all = 1;
    out.is_current = 1;
    getsockopt_checked(sk, &mut out, EINVAL, c"get_all + current".as_ptr());

    sk = prepare_defs(TCP_AO_GET_KEYS, &mut out as *mut tcp_ao_getsockopt as *mut c_void);
    out.get_all = 1;
    out.is_rnext = 1;
    getsockopt_checked(sk, &mut out, EINVAL, c"get_all + rnext".as_ptr());

    sk = prepare_defs(TCP_AO_GET_KEYS, &mut out as *mut tcp_ao_getsockopt as *mut c_void);
    out.get_all = 0;
    out.is_current = 1;
    out.is_rnext = 1;
    getsockopt_checked(sk, &mut out, 0, c"current + rnext".as_ptr());
}

unsafe fn einval_tests() {
    test_einval_add_key();
    test_einval_del_key();
    test_einval_ao_info();
    test_einval_get_keys();
}

unsafe fn duplicate_tests() {
    let mut network_dup: tcp_addr = mem::zeroed();
    let mut ao: tcp_ao_add = mem::zeroed();
    let mut ao2: tcp_ao_add = mem::zeroed();
    let mut sk: c_int;

    sk = prepare_defs(TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void);
    if setsockopt(sk, IPPROTO_TCP, TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void, size_of::<tcp_ao_add>() as socklen_t) != 0 {
        test_error!("setsockopt()");
    }
    setsockopt_checked(sk, TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void, EEXIST, c"duplicate: full copy".as_ptr());

    sk = prepare_defs(TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void);
    ao2 = ao;
    memcpy(&mut ao2.addr as *mut _ as *mut c_void, &SOCKADDR_ANY as *const _ as *const c_void, size_of_val(&SOCKADDR_ANY));
    ao2.prefix = 0;
    if setsockopt(sk, IPPROTO_TCP, TCP_AO_ADD_KEY, &mut ao2 as *mut tcp_ao_add as *mut c_void, size_of::<tcp_ao_add>() as socklen_t) != 0 {
        test_error!("setsockopt()");
    }
    setsockopt_checked(sk, TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void, EEXIST, c"duplicate: any addr key on the socket".as_ptr());

    sk = prepare_defs(TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void);
    if setsockopt(sk, IPPROTO_TCP, TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void, size_of::<tcp_ao_add>() as socklen_t) != 0 {
        test_error!("setsockopt()");
    }
    memcpy(&mut ao.addr as *mut _ as *mut c_void, &SOCKADDR_ANY as *const _ as *const c_void, size_of_val(&SOCKADDR_ANY));
    ao.prefix = 0;
    setsockopt_checked(sk, TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void, EEXIST, c"duplicate: add any addr key".as_ptr());

    if inet_pton(TEST_FAMILY, TEST_NETWORK, &mut network_dup as *mut tcp_addr as *mut c_void) != 1 {
        test_error!("Can't convert ip address %s", TEST_NETWORK);
    }
    sk = prepare_defs(TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void);
    if setsockopt(sk, IPPROTO_TCP, TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void, size_of::<tcp_ao_add>() as socklen_t) != 0 {
        test_error!("setsockopt()");
    }
    if test_prepare_def_key(&mut ao, c"password".as_ptr(), 0, network_dup, 16, 0, 100, 100) != 0 {
        test_error!("prepare default tcp_ao_add");
    }
    setsockopt_checked(sk, TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void, EEXIST, c"duplicate: add any addr for the same subnet".as_ptr());

    sk = prepare_defs(TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void);
    if setsockopt(sk, IPPROTO_TCP, TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void, size_of::<tcp_ao_add>() as socklen_t) != 0 {
        test_error!("setsockopt()");
    }
    setsockopt_checked(sk, TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void, EEXIST, c"duplicate: full copy of a key".as_ptr());

    sk = prepare_defs(TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void);
    if setsockopt(sk, IPPROTO_TCP, TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void, size_of::<tcp_ao_add>() as socklen_t) != 0 {
        test_error!("setsockopt()");
    }
    ao.rcvid = 101;
    setsockopt_checked(sk, TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void, EEXIST, c"duplicate: RecvID differs".as_ptr());

    sk = prepare_defs(TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void);
    if setsockopt(sk, IPPROTO_TCP, TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void, size_of::<tcp_ao_add>() as socklen_t) != 0 {
        test_error!("setsockopt()");
    }
    ao.sndid = 101;
    setsockopt_checked(sk, TCP_AO_ADD_KEY, &mut ao as *mut tcp_ao_add as *mut c_void, EEXIST, c"duplicate: SendID differs".as_ptr());
}

unsafe fn fetch_all_keys(sk: c_int, keys: *mut tcp_ao_getsockopt) {
    let mut optlen: socklen_t = size_of::<tcp_ao_getsockopt>() as socklen_t;

    ptr::write_bytes(keys, 0, FILTER_TEST_NKEYS);
    (*keys.add(0)).get_all = 1;
    (*keys.add(0)).nkeys = FILTER_TEST_NKEYS as _;
    if getsockopt(
        sk,
        IPPROTO_TCP,
        TCP_AO_GET_KEYS,
        keys.add(0) as *mut c_void,
        &mut optlen,
    ) != 0
    {
        test_error!("getsockopt");
    }
}

unsafe fn prepare_test_keys(keys: *mut tcp_ao_getsockopt) -> c_int {
    let test_password = c"Test password number ".as_ptr();
    let mut test_ao: [tcp_ao_add; FILTER_TEST_NKEYS] = mem::zeroed();
    let mut test_password_scratch: [c_char; 64] = [0; 64];
    let mut rcvid: u8 = 100;
    let mut sndid: u8 = 100;
    let sk: c_int;

    sk = socket(test_family, SOCK_STREAM, IPPROTO_TCP);
    if sk < 0 {
        test_error!("socket()");
    }

    for i in 0..FILTER_TEST_NKEYS {
        snprintf(
            test_password_scratch.as_mut_ptr(),
            64,
            c"%s %d".as_ptr(),
            test_password,
            i as c_int,
        );
        test_prepare_key(
            &mut test_ao[i],
            DEFAULT_TEST_ALGO,
            this_ip_dest,
            false,
            false,
            DEFAULT_TEST_PREFIX,
            0,
            {
                let v = sndid;
                sndid = sndid.wrapping_add(1);
                v
            },
            {
                let v = rcvid;
                rcvid = rcvid.wrapping_add(1);
                v
            },
            0,
            0,
            strlen(test_password_scratch.as_ptr()),
            test_password_scratch.as_ptr(),
        );
    }
    test_ao[0].set_current = 1;
    test_ao[1].set_rnext = 1;
    /* One key with a different addr and overlapping sndid, rcvid */
    tcp_addr_to_sockaddr_in(&mut test_ao[2].addr, &this_ip_addr, 0);
    test_ao[2].sndid = 100;
    test_ao[2].rcvid = 100;

    /* Add keys in a random order */
    for i in 0..FILTER_TEST_NKEYS {
        let randidx = (rand() as usize) % (FILTER_TEST_NKEYS - i);

        if setsockopt(
            sk,
            IPPROTO_TCP,
            TCP_AO_ADD_KEY,
            &mut test_ao[randidx] as *mut tcp_ao_add as *mut c_void,
            size_of::<tcp_ao_add>() as socklen_t,
        ) != 0
        {
            test_error!("setsockopt()");
        }
        ptr::copy_nonoverlapping(
            &test_ao[FILTER_TEST_NKEYS - 1 - i],
            &mut test_ao[randidx],
            1,
        );
    }

    fetch_all_keys(sk, keys);

    sk
}

/* Assumes passwords are unique */
unsafe fn compare_mkts(
    expected: *mut tcp_ao_getsockopt,
    nexpected: c_int,
    actual: *mut tcp_ao_getsockopt,
    nactual: c_int,
) -> c_int {
    let mut matches: c_int = 0;

    for i in 0..nexpected {
        for j in 0..nactual {
            if memcmp(
                (*expected.add(i as usize)).key.as_ptr() as *const c_void,
                (*actual.add(j as usize)).key.as_ptr() as *const c_void,
                TCP_AO_MAXKEYLEN as usize,
            ) == 0
            {
                matches += 1;
            }
        }
    }
    nexpected - matches
}

unsafe fn filter_keys_checked(
    sk: c_int,
    filter: *mut tcp_ao_getsockopt,
    expected: *mut tcp_ao_getsockopt,
    nexpected: c_uint,
    tst: *const c_char,
) {
    let mut filtered_keys: [tcp_ao_getsockopt; FILTER_TEST_NKEYS] = mem::zeroed();
    let mut all_keys: [tcp_ao_getsockopt; FILTER_TEST_NKEYS] = mem::zeroed();
    let mut len: socklen_t = size_of::<tcp_ao_getsockopt>() as socklen_t;

    fetch_all_keys(sk, all_keys.as_mut_ptr());
    ptr::copy_nonoverlapping(filter, filtered_keys.as_mut_ptr(), 1);
    filtered_keys[0].nkeys = FILTER_TEST_NKEYS as _;
    if getsockopt(
        sk,
        IPPROTO_TCP,
        TCP_AO_GET_KEYS,
        filtered_keys.as_mut_ptr() as *mut c_void,
        &mut len,
    ) != 0
    {
        test_error!("getsockopt");
    }
    if filtered_keys[0].nkeys != nexpected {
        test_fail!(
            "wrong nr of keys, expected %u got %u",
            nexpected,
            filtered_keys[0].nkeys
        );
        close(sk);
        ptr::write_bytes(filter, 0, 1);
        return;
    }
    if compare_mkts(
        expected,
        nexpected as c_int,
        filtered_keys.as_mut_ptr(),
        filtered_keys[0].nkeys as c_int,
    ) != 0
    {
        test_fail!("got wrong keys back");
        close(sk);
        ptr::write_bytes(filter, 0, 1);
        return;
    }
    test_ok!("filter keys: %s", tst);

    close(sk);
    ptr::write_bytes(filter, 0, 1);
}

unsafe fn filter_tests() {
    let mut original_keys: [tcp_ao_getsockopt; FILTER_TEST_NKEYS] = mem::zeroed();
    let mut expected_keys: [tcp_ao_getsockopt; FILTER_TEST_NKEYS] = mem::zeroed();
    let mut filter: tcp_ao_getsockopt = mem::zeroed();
    let mut sk: c_int;
    let mut f: c_int;
    let mut nmatches: c_int;
    let mut len: socklen_t;

    f = 2;
    sk = prepare_test_keys(original_keys.as_mut_ptr());
    filter.rcvid = original_keys[f as usize].rcvid;
    filter.sndid = original_keys[f as usize].sndid;
    ptr::copy_nonoverlapping(&original_keys[f as usize].addr, &mut filter.addr, 1);
    filter.prefix = original_keys[f as usize].prefix;
    filter_keys_checked(sk, &mut filter, &mut original_keys[f as usize], 1, c"by sndid, rcvid, address".as_ptr());

    f = -1;
    sk = prepare_test_keys(original_keys.as_mut_ptr());
    for i in 0..original_keys[0].nkeys {
        if original_keys[i as usize].is_current != 0 {
            f = i as c_int;
            break;
        }
    }
    if f < 0 {
        test_error!("No current key after adding one");
    }
    filter.is_current = 1;
    filter_keys_checked(sk, &mut filter, &mut original_keys[f as usize], 1, c"by is_current".as_ptr());

    f = -1;
    sk = prepare_test_keys(original_keys.as_mut_ptr());
    for i in 0..original_keys[0].nkeys {
        if original_keys[i as usize].is_rnext != 0 {
            f = i as c_int;
            break;
        }
    }
    if f < 0 {
        test_error!("No rnext key after adding one");
    }
    filter.is_rnext = 1;
    filter_keys_checked(sk, &mut filter, &mut original_keys[f as usize], 1, c"by is_rnext".as_ptr());

    f = -1;
    nmatches = 0;
    sk = prepare_test_keys(original_keys.as_mut_ptr());
    for i in 0..original_keys[0].nkeys {
        if original_keys[i as usize].sndid == 100 {
            f = i as c_int;
            ptr::copy_nonoverlapping(
                &original_keys[i as usize],
                &mut expected_keys[nmatches as usize],
                1,
            );
            nmatches += 1;
        }
    }
    if f < 0 {
        test_error!("No key for sndid 100");
    }
    if nmatches != 2 {
        test_error!("Should have 2 keys with sndid 100");
    }
    filter.rcvid = original_keys[f as usize].rcvid;
    filter.sndid = original_keys[f as usize].sndid;
    filter.addr.ss_family = test_family;
    filter_keys_checked(sk, &mut filter, expected_keys.as_mut_ptr(), nmatches as c_uint, c"by sndid, rcvid".as_ptr());

    sk = prepare_test_keys(original_keys.as_mut_ptr());
    filter.get_all = 1;
    filter.nkeys = (FILTER_TEST_NKEYS / 2) as _;
    len = size_of::<tcp_ao_getsockopt>() as socklen_t;
    if getsockopt(
        sk,
        IPPROTO_TCP,
        TCP_AO_GET_KEYS,
        &mut filter as *mut tcp_ao_getsockopt as *mut c_void,
        &mut len,
    ) != 0
    {
        test_error!("getsockopt");
    }
    if filter.nkeys == FILTER_TEST_NKEYS as _ {
        test_ok!("filter keys: correct nkeys when in.nkeys < matches");
    } else {
        test_fail!(
            "filter keys: wrong nkeys, expected %u got %u",
            FILTER_TEST_NKEYS as c_uint,
            filter.nkeys
        );
    }
}

unsafe extern "C" fn client_fn(_arg: *mut c_void) -> *mut c_void {
    if inet_pton(
        TEST_FAMILY,
        __TEST_CLIENT_IP(2),
        &mut tcp_md5_client as *mut tcp_addr as *mut c_void,
    ) != 1
    {
        test_error!("Can't convert ip address");
    }
    extend_tests();
    einval_tests();
    filter_tests();
    duplicate_tests();

    ptr::null_mut()
}

fn main() {
    unsafe {
        test_init(126, Some(client_fn), ptr::null_mut());
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
