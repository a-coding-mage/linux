// SPDX-License-Identifier: GPL-2.0-only
/*
 * Landlock - Network management and hooks
 *
 * Copyright © 2022-2023 Huawei Tech. Co., Ltd.
 * Copyright © 2022-2025 Microsoft Corporation
 */

/* C includes translated as external dependency intent:
 * <linux/in.h>, <linux/lsm_audit.h>, <linux/net.h>, <linux/socket.h>,
 * <net/ipv6.h>, "common.h", "cred.h", "domain.h", "limits.h", "log.h",
 * "net.h", "ruleset.h", and <trace/events/landlock.h>.
 */

pub unsafe extern "C" fn landlock_append_net_rule(
    ruleset: *mut landlock_ruleset,
    port: u16,
    mut access_rights: access_mask_t,
    flags: u32,
) -> core::ffi::c_int {
    let err: core::ffi::c_int;
    let id = landlock_id {
        key: landlock_id_key {
            data: htons(port) as uintptr_t,
        },
        type_: LANDLOCK_KEY_NET_PORT,
    };

    /* BUILD_BUG_ON(sizeof(port) > sizeof(id.key.data)); */

    /* Transforms relative access rights to absolute ones. */
    access_rights |= LANDLOCK_MASK_ACCESS_NET & !(*ruleset).handled_masks.net;

    mutex_lock(&mut (*ruleset).lock);
    err = landlock_insert_rule(ruleset, id, access_rights, flags);

    /*
     * Emit after the rule insertion succeeds, so every event corresponds to
     * a rule that is actually in the ruleset.  The ruleset lock is still
     * held for BTF consistency (enforced by lockdep_assert_held in
     * TP_fast_assign).
     */
    if err == 0 {
        trace_landlock_add_rule_net(ruleset, access_rights, port);
    }
    mutex_unlock(&mut (*ruleset).lock);

    err
}

unsafe fn unmask_layers_net(
    domain: *const landlock_domain,
    id: landlock_id,
    masks: *mut layer_masks,
    access_request: access_mask_t,
) -> bool {
    let mut rule: *const landlock_rule = core::ptr::null();
    let ret: bool;

    ret = landlock_unmask_layers(domain, id, masks, &mut rule);
    if !rule.is_null() {
        trace_landlock_check_rule_net(
            domain,
            rule,
            access_request,
            ntohs(id.key.data as __be16),
        );
    }
    ret
}

unsafe fn current_check_access_socket(
    sock: *mut socket,
    address: *mut sockaddr,
    addrlen: core::ffi::c_int,
    mut access_request: access_mask_t,
    connecting: bool,
) -> core::ffi::c_int {
    let sock_family: core::ffi::c_ushort;
    let port: __be16;
    let mut layer_masks: layer_masks = core::mem::zeroed();
    let mut id = landlock_id {
        key: landlock_id_key { data: 0 },
        type_: LANDLOCK_KEY_NET_PORT,
    };
    let masks = access_masks { net: access_request };
    let subject: *const landlock_cred_security =
        landlock_get_applicable_subject(current_cred(), masks, core::ptr::null_mut());
    let mut audit_net: lsm_network_audit = core::mem::zeroed();

    if subject.is_null() {
        return 0;
    }

    /* Checks for minimal header length to safely read sa_family. */
    if addrlen < offsetofend_sockaddr_sa_family() as core::ffi::c_int {
        return -EINVAL;
    }

    /*
     * The socket is not locked, so sk_family can change concurrently due to
     * e.g. setsockopt(IPV6_ADDRFORM).
     */
    sock_family = READ_ONCE((*(*sock).sk).sk_family);

    match (*address).sa_family as core::ffi::c_int {
        AF_UNSPEC => {
            if access_request == LANDLOCK_ACCESS_NET_CONNECT_TCP
                || (access_request == LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP && connecting)
            {
                /*
                 * Connecting to an address with AF_UNSPEC dissolves the
                 * remote association while retaining the socket object
                 * (i.e., the file descriptor). For TCP, it has the same
                 * effect as closing the connection. For UDP, it removes
                 * any preset remote address. As for dropping
                 * privileges, these actions are always allowed.  Let
                 * the network stack handle potential inconsistencies
                 * and return -EINVAL if needed.
                 */
                return 0;
            } else if access_request == LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP {
                if sock_family as core::ffi::c_int == AF_INET6 {
                    /*
                     * We cannot allow sending UDP datagrams to an
                     * explicit AF_UNSPEC address on IPv6 sockets,
                     * even if AF_UNSPEC is treated as "no address"
                     * on such sockets (so it should always be
                     * allowed).  That's because the socket's family
                     * can change under our feet (if another thread
                     * calls setsockopt(IPV6_ADDRFORM)) to IPv4,
                     * which would then treat AF_UNSPEC as AF_INET.
                     */
                    audit_net.family = AF_UNSPEC;
                    audit_net.sk = (*sock).sk;
                    landlock_init_layer_masks(
                        (*subject).domain,
                        access_request,
                        &mut layer_masks,
                        LANDLOCK_KEY_NET_PORT,
                    );
                    landlock_log_denial(
                        subject,
                        &mut landlock_request {
                            type_: LANDLOCK_REQUEST_NET_ACCESS,
                            audit: landlock_request_audit {
                                type_: LSM_AUDIT_DATA_NET,
                                u: landlock_request_audit_u { net: &mut audit_net },
                            },
                            access: access_request,
                            layer_masks: &mut layer_masks,
                        },
                    );
                    return -EACCES;
                }
            } else if access_request == LANDLOCK_ACCESS_NET_BIND_TCP
                || access_request == LANDLOCK_ACCESS_NET_BIND_UDP
            {
                /*
                 * Binding to an AF_UNSPEC address is treated
                 * differently by IPv4 and IPv6 sockets. The socket's
                 * family may change under our feet due to
                 * setsockopt(IPV6_ADDRFORM), but that's ok: we either
                 * reject entirely for IPv6 or require
                 * %LANDLOCK_ACCESS_NET_BIND_TCP or
                 * %LANDLOCK_ACCESS_NET_BIND_UDP for IPv4, so it cannot
                 * be used to bypass the policy.
                 *
                 * IPv4 sockets map AF_UNSPEC to AF_INET for
                 * retrocompatibility for bind accesses, only if the
                 * address is INADDR_ANY (cf. __inet_bind). IPv6
                 * sockets always reject it.
                 *
                 * Checking the address is required to not wrongfully
                 * return -EACCES instead of -EAFNOSUPPORT or -EINVAL.
                 * We could return 0 and let the network stack handle
                 * these checks, but it is safer to return a proper
                 * error and test consistency thanks to kselftest.
                 */
                if sock_family as core::ffi::c_int == AF_INET {
                    let sockaddr = address as *const sockaddr_in;

                    if addrlen < core::mem::size_of::<sockaddr_in>() as core::ffi::c_int {
                        return -EINVAL;
                    }

                    if (*sockaddr).sin_addr.s_addr != htonl(INADDR_ANY) {
                        return -EAFNOSUPPORT;
                    }
                } else if addrlen < SIN6_LEN_RFC2133 {
                    return -EINVAL;
                } else {
                    return -EAFNOSUPPORT;
                }
            } else {
                WARN_ON_ONCE(1);
            }
            /*
             * AF_UNSPEC is treated as AF_INET only in
             * bind(AF_UNSPEC+INADDR_ANY) on IPv4 sockets and when sending
             * to AF_UNSPEC addresses on IPv4 sockets.
             */
            let addr4: *const sockaddr_in;

            if addrlen < core::mem::size_of::<sockaddr_in>() as core::ffi::c_int {
                return -EINVAL;
            }

            addr4 = address as *const sockaddr_in;
            port = (*addr4).sin_port;

            if access_request == LANDLOCK_ACCESS_NET_CONNECT_TCP
                || access_request == LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP
            {
                audit_net.dport = port;
                audit_net.v4info.daddr = (*addr4).sin_addr.s_addr;
            } else if access_request == LANDLOCK_ACCESS_NET_BIND_TCP
                || access_request == LANDLOCK_ACCESS_NET_BIND_UDP
            {
                audit_net.sport = port;
                audit_net.v4info.saddr = (*addr4).sin_addr.s_addr;
            } else {
                WARN_ON_ONCE(1);
            }
        }
        AF_INET => {
            let addr4: *const sockaddr_in;

            if addrlen < core::mem::size_of::<sockaddr_in>() as core::ffi::c_int {
                return -EINVAL;
            }

            addr4 = address as *const sockaddr_in;
            port = (*addr4).sin_port;

            if access_request == LANDLOCK_ACCESS_NET_CONNECT_TCP
                || access_request == LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP
            {
                audit_net.dport = port;
                audit_net.v4info.daddr = (*addr4).sin_addr.s_addr;
            } else if access_request == LANDLOCK_ACCESS_NET_BIND_TCP
                || access_request == LANDLOCK_ACCESS_NET_BIND_UDP
            {
                audit_net.sport = port;
                audit_net.v4info.saddr = (*addr4).sin_addr.s_addr;
            } else {
                WARN_ON_ONCE(1);
            }
        }

        /* Original C condition: #if IS_ENABLED(CONFIG_IPV6) */
        AF_INET6 => {
            let addr6: *const sockaddr_in6;

            if addrlen < SIN6_LEN_RFC2133 {
                return -EINVAL;
            }

            addr6 = address as *const sockaddr_in6;
            port = (*addr6).sin6_port;

            if access_request == LANDLOCK_ACCESS_NET_CONNECT_TCP
                || access_request == LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP
            {
                audit_net.dport = port;
                audit_net.v6info.daddr = (*addr6).sin6_addr;
            } else if access_request == LANDLOCK_ACCESS_NET_BIND_TCP
                || access_request == LANDLOCK_ACCESS_NET_BIND_UDP
            {
                audit_net.sport = port;
                audit_net.v6info.saddr = (*addr6).sin6_addr;
            } else {
                WARN_ON_ONCE(1);
            }
        }

        _ => {
            return 0;
        }
    }

    /*
     * Checks sa_family consistency to not wrongfully return
     * -EACCES instead of -EINVAL.  Valid sa_family changes are
     * only (from AF_INET or AF_INET6) to AF_UNSPEC.
     *
     * We could return 0 and let the network stack handle this
     * check, but it is safer to return a proper error and test
     * consistency thanks to kselftest.
     */
    if (*address).sa_family != sock_family && (*address).sa_family as core::ffi::c_int != AF_UNSPEC {
        return -EINVAL;
    }

    id.key.data = port as uintptr_t;
    /* BUILD_BUG_ON(sizeof(port) > sizeof(id.key.data)); */

    access_request = landlock_init_layer_masks(
        (*subject).domain,
        access_request,
        &mut layer_masks,
        LANDLOCK_KEY_NET_PORT,
    );
    if access_request == 0 {
        return 0;
    }

    if unmask_layers_net((*subject).domain, id, &mut layer_masks, access_request) {
        return 0;
    }

    audit_net.family = (*address).sa_family;
    audit_net.sk = (*sock).sk;
    landlock_log_denial(
        subject,
        &mut landlock_request {
            type_: LANDLOCK_REQUEST_NET_ACCESS,
            audit: landlock_request_audit {
                type_: LSM_AUDIT_DATA_NET,
                u: landlock_request_audit_u { net: &mut audit_net },
            },
            access: access_request,
            layer_masks: &mut layer_masks,
        },
    );
    -EACCES
}

unsafe fn current_check_autobind_udp_socket(sock: *mut socket) -> core::ffi::c_int {
    let bind_udp = access_masks {
        net: LANDLOCK_ACCESS_NET_BIND_UDP,
    };
    let mut port0: sockaddr_storage = core::mem::zeroed();
    let num: core::ffi::c_ushort;
    let slow: bool;

    /* Quick return for non-Landlocked tasks. */
    if landlock_get_applicable_subject(current_cred(), bind_udp, core::ptr::null_mut()).is_null() {
        return 0;
    }

    /*
     * On UDP sockets, if a local port has not already been bound, calling
     * connect() or sending a first datagram has the side effect of
     * autobinding an ephemeral port: we also have to check that the process
     * would have had the right to bind(0) explicitly.  Hold the socket lock
     * around the inet_num read to exclude udp_lib_get_port()'s transient
     * inet_num = snum write that is reverted to 0 on a failing reuseport
     * bind.
     */
    slow = lock_sock_fast((*sock).sk);
    num = (*inet_sk((*sock).sk)).inet_num;
    unlock_sock_fast((*sock).sk, slow);
    if num != 0 {
        return 0;
    }

    /*
     * Construct a struct sockaddr* with port 0 to pretend the process tried
     * to bind() on that address.
     */
    port0.ss_family = READ_ONCE((*(*sock).sk).sk_family);

    current_check_access_socket(
        sock,
        &mut port0 as *mut sockaddr_storage as *mut sockaddr,
        core::mem::size_of_val(&port0) as core::ffi::c_int,
        bind_udp.net,
        false,
    )
}

unsafe fn hook_socket_bind(
    sock: *mut socket,
    address: *mut sockaddr,
    addrlen: core::ffi::c_int,
) -> core::ffi::c_int {
    let access_request: access_mask_t;

    if sk_is_tcp((*sock).sk) {
        access_request = LANDLOCK_ACCESS_NET_BIND_TCP;
    } else if sk_is_udp((*sock).sk) {
        access_request = LANDLOCK_ACCESS_NET_BIND_UDP;
    } else {
        return 0;
    }

    current_check_access_socket(sock, address, addrlen, access_request, false)
}

unsafe fn hook_socket_connect(
    sock: *mut socket,
    address: *mut sockaddr,
    addrlen: core::ffi::c_int,
) -> core::ffi::c_int {
    let access_request: access_mask_t;
    let mut ret: core::ffi::c_int = 0;

    if sk_is_tcp((*sock).sk) {
        access_request = LANDLOCK_ACCESS_NET_CONNECT_TCP;
    } else if sk_is_udp((*sock).sk) {
        access_request = LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP;
    } else {
        return 0;
    }

    ret = current_check_access_socket(sock, address, addrlen, access_request, true);

    /*
     * connect()ing to an AF_UNSPEC address does not trigger an autobind and
     * should never be restricted.
     */
    if ret == 0
        && sk_is_udp((*sock).sk)
        && addrlen >= offsetofend_sockaddr_sa_family() as core::ffi::c_int
        && (*address).sa_family as core::ffi::c_int != AF_UNSPEC
    {
        ret = current_check_autobind_udp_socket(sock);
    }

    ret
}

unsafe fn hook_socket_sendmsg(
    sock: *mut socket,
    msg: *mut msghdr,
    size: core::ffi::c_int,
) -> core::ffi::c_int {
    let address: *mut sockaddr = (*msg).msg_name as *mut sockaddr;
    let addrlen: core::ffi::c_int = (*msg).msg_namelen;
    let access_request: access_mask_t;
    let mut ret: core::ffi::c_int = 0;
    let _ = size;

    if ((*msg).msg_flags & MSG_FASTOPEN) != 0 && !address.is_null() && sk_is_tcp((*sock).sk) {
        ret = current_check_access_socket(
            sock,
            address,
            addrlen,
            LANDLOCK_ACCESS_NET_CONNECT_TCP,
            true,
        );
        if ret != 0 {
            return ret;
        }
    }

    if sk_is_udp((*sock).sk) {
        access_request = LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP;
    } else {
        return 0;
    }

    if !address.is_null() {
        ret = current_check_access_socket(sock, address, addrlen, access_request, false);
    }

    if ret == 0 {
        ret = current_check_autobind_udp_socket(sock);
    }

    ret
}

#[link_section = ".data..ro_after_init"]
static mut landlock_hooks: [security_hook_list; 3] = [
    LSM_HOOK_INIT(socket_bind, hook_socket_bind),
    LSM_HOOK_INIT(socket_connect, hook_socket_connect),
    LSM_HOOK_INIT(socket_sendmsg, hook_socket_sendmsg),
];

#[init]
pub unsafe extern "C" fn landlock_add_net_hooks() {
    security_add_hooks(
        landlock_hooks.as_mut_ptr(),
        ARRAY_SIZE(&landlock_hooks),
        &mut landlock_lsmid,
    );
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
