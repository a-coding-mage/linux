// SPDX-License-Identifier: GPL-2.0

// Translated from C source. Original include dependencies:
// <sys/types.h>, <sys/socket.h>, <sys/xattr.h>, <test_progs.h>, <bpf/btf.h>,
// "lsm_cgroup.skel.h", "lsm_cgroup_nonvoid.skel.h", "cgroup_helpers.h",
// "network_helpers.h".

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type __u32 = u32;
type u32 = u32;
type socklen_t = u32;

const BPF_LSM_CGROUP: c_int = 0;
const BTF_KIND_FUNC: c_int = 0;
const BPF_F_REPLACE: c_uint = 1 << 2;
const BPF_F_ALLOW_MULTI: c_uint = 1 << 0;
const ENOTSUPP: c_int = 524;
const EPERM: c_int = 1;
const AF_UNIX: c_int = 1;
const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const AF_PACKET: c_int = 17;
const SOCK_STREAM: c_int = 1;
const SOCK_RAW: c_int = 3;
const SOL_SOCKET: c_int = 1;
const SO_PRIORITY: c_int = 12;
const ETH_P_ALL: c_int = 0x0003;

#[repr(C)]
struct btf {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_prog_query_opts {
    prog_cnt: __u32,
    prog_ids: *mut __u32,
    prog_attach_flags: *mut __u32,
}

#[repr(C)]
struct bpf_prog_attach_opts {
    replace_prog_fd: c_int,
}

#[repr(C)]
struct bpf_link_update_opts {
    old_prog_fd: c_int,
    flags: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct bpf_prog_info {
    attach_btf_id: __u32,
}

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct sockaddr_ll {
    sll_family: c_uint,
    sll_protocol: u16,
}

#[repr(C)]
struct lsm_cgroup_progs {
    socket_post_create: *mut bpf_program,
    socket_post_create2: *mut bpf_program,
    socket_bind: *mut bpf_program,
    socket_bind2: *mut bpf_program,
    socket_alloc: *mut bpf_program,
    socket_clone: *mut bpf_program,
    skipcap_first: *mut bpf_program,
    skipcap_second: *mut bpf_program,
    socket_first: *mut bpf_program,
    socket_second: *mut bpf_program,
}

#[repr(C)]
struct lsm_cgroup_kconfig {
    CONFIG_SECURITY_APPARMOR: bool,
    CONFIG_SECURITY_SELINUX: bool,
    CONFIG_SECURITY_SMACK: bool,
}

#[repr(C)]
struct lsm_cgroup_bss {
    called_socket_post_create: c_int,
    called_socket_bind: c_int,
    called_socket_post_create2: c_int,
    called_socket_bind2: c_int,
    called_socket_clone: c_int,
    called_socket_alloc: c_int,
}

#[repr(C)]
struct lsm_cgroup_data {
    skipcap_retval: c_int,
    socket_retval: c_int,
}

#[repr(C)]
struct lsm_cgroup {
    progs: lsm_cgroup_progs,
    kconfig: *mut lsm_cgroup_kconfig,
    bss: *mut lsm_cgroup_bss,
    data: *mut lsm_cgroup_data,
}

#[repr(C)]
struct lsm_cgroup_nonvoid {
    _private: [u8; 0],
}

static mut btf: *mut btf = ptr::null_mut();

extern "C" {
    fn bpf_prog_query_opts(cgroup_fd: c_int, attach_type: c_int, opts: *mut bpf_prog_query_opts) -> c_int;
    fn bpf_prog_attach(prog_fd: c_int, target_fd: c_int, attach_type: c_int, flags: c_uint) -> c_int;
    fn bpf_prog_attach_opts(
        prog_fd: c_int,
        target_fd: c_int,
        attach_type: c_int,
        opts: *mut bpf_prog_attach_opts,
    ) -> c_int;
    fn bpf_link_create(prog_fd: c_int, target_fd: c_int, attach_type: c_int, opts: *const c_void) -> c_int;
    fn bpf_link_update(link_fd: c_int, new_prog_fd: c_int, opts: *mut bpf_link_update_opts) -> c_int;
    fn bpf_prog_detach2(prog_fd: c_int, target_fd: c_int, attach_type: c_int) -> c_int;
    fn bpf_prog_get_fd_by_id(id: __u32) -> c_int;
    fn bpf_prog_get_info_by_fd(fd: c_int, info: *mut bpf_prog_info, info_len: *mut __u32) -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn btf__load_vmlinux_btf() -> *mut btf;
    fn btf__find_by_name_kind(btf: *mut btf, name: *const c_char, kind: c_int) -> c_int;
    fn btf__free(btf: *mut btf);
    fn libbpf_get_error(ptr: *const c_void) -> c_int;

    fn lsm_cgroup__open_and_load() -> *mut lsm_cgroup;
    fn lsm_cgroup__destroy(skel: *mut lsm_cgroup);
    fn lsm_cgroup_nonvoid__open_and_load() -> *mut lsm_cgroup_nonvoid;
    fn lsm_cgroup_nonvoid__destroy(skel: *mut lsm_cgroup_nonvoid);

    fn test__join_cgroup(path: *const c_char) -> c_int;
    fn join_cgroup(path: *const c_char) -> c_int;
    fn test__skip();
    fn test__start_subtest(name: *const c_char) -> bool;
    fn start_server(family: c_int, socktype: c_int, addr: *const c_char, port: c_int, timeout_ms: c_int) -> c_int;
    fn connect_to_fd(fd: c_int, timeout_ms: c_int) -> c_int;

    fn socket(domain: c_int, typ: c_int, protocol: c_int) -> c_int;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn getsockopt(
        sockfd: c_int,
        level: c_int,
        optname: c_int,
        optval: *mut c_void,
        optlen: *mut socklen_t,
    ) -> c_int;
    fn accept(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn htons(hostshort: u16) -> u16;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn mkstemp(template: *mut c_char) -> c_int;
    fn setxattr(path: *const c_char, name: *const c_char, value: *const c_void, size: usize, flags: c_int) -> c_int;
    fn unlink(path: *const c_char) -> c_int;

    fn ASSERT_OK(ret: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_FD(fd: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_NULL(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_LT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
}

unsafe fn query_prog_cnt(cgroup_fd: c_int, attach_func: *const c_char) -> __u32 {
    let mut p = bpf_prog_query_opts {
        prog_cnt: 0,
        prog_ids: ptr::null_mut(),
        prog_attach_flags: ptr::null_mut(),
    };
    let mut cnt: c_int = 0;

    ASSERT_OK(
        bpf_prog_query_opts(cgroup_fd, BPF_LSM_CGROUP, &mut p),
        b"prog_query\0".as_ptr() as *const c_char,
    );

    if attach_func.is_null() {
        return p.prog_cnt;
    }

    /* When attach_func is provided, count the number of progs that
     * attach to the given symbol.
     */

    if btf.is_null() {
        btf = btf__load_vmlinux_btf();
    }
    if !ASSERT_OK(libbpf_get_error(btf as *const c_void), b"btf_vmlinux\0".as_ptr() as *const c_char) {
        return -1i32 as __u32;
    }

    p.prog_ids = malloc(size_of::<u32>() * p.prog_cnt as usize) as *mut u32;
    p.prog_attach_flags = malloc(size_of::<u32>() * p.prog_cnt as usize) as *mut u32;
    ASSERT_OK(
        bpf_prog_query_opts(cgroup_fd, BPF_LSM_CGROUP, &mut p),
        b"prog_query\0".as_ptr() as *const c_char,
    );

    for i in 0..p.prog_cnt {
        let mut info = bpf_prog_info { attach_btf_id: 0 };
        let mut info_len: __u32 = size_of::<bpf_prog_info>() as __u32;
        let fd: c_int;

        fd = bpf_prog_get_fd_by_id(*p.prog_ids.add(i as usize));
        ASSERT_GE(fd, 0, b"prog_get_fd_by_id\0".as_ptr() as *const c_char);
        ASSERT_OK(
            bpf_prog_get_info_by_fd(fd, &mut info, &mut info_len),
            b"prog_info_by_fd\0".as_ptr() as *const c_char,
        );
        close(fd);

        if info.attach_btf_id == btf__find_by_name_kind(btf, attach_func, BTF_KIND_FUNC) as __u32 {
            cnt += 1;
        }
    }

    free(p.prog_ids as *mut c_void);
    free(p.prog_attach_flags as *mut c_void);

    cnt as __u32
}

unsafe fn test_lsm_cgroup_functional() {
    let mut attach_opts = bpf_prog_attach_opts { replace_prog_fd: 0 };
    let mut update_opts = bpf_link_update_opts {
        old_prog_fd: 0,
        flags: 0,
    };
    let mut cgroup_fd: c_int = -1;
    let mut cgroup_fd2: c_int = -1;
    let mut cgroup_fd3: c_int = -1;
    let listen_fd: c_int;
    let client_fd: c_int;
    let accepted_fd: c_int;
    let mut skel: *mut lsm_cgroup = ptr::null_mut();
    let mut post_create_prog_fd2: c_int = -1;
    let mut post_create_prog_fd: c_int = -1;
    let mut bind_link_fd2: c_int = -1;
    let mut bind_prog_fd2: c_int = -1;
    let mut alloc_prog_fd: c_int = -1;
    let mut bind_prog_fd: c_int = -1;
    let mut bind_link_fd: c_int = -1;
    let mut clone_prog_fd: c_int = -1;
    let mut err: c_int;
    let mut fd: c_int;
    let mut prio: c_int;
    let mut socklen: socklen_t;

    'close_cgroup: {
        'detach_cgroup: {
            cgroup_fd3 = test__join_cgroup(b"/sock_policy_empty\0".as_ptr() as *const c_char);
            if !ASSERT_GE(cgroup_fd3, 0, b"create empty cgroup\0".as_ptr() as *const c_char) {
                break 'close_cgroup;
            }

            cgroup_fd2 = test__join_cgroup(b"/sock_policy_reuse\0".as_ptr() as *const c_char);
            if !ASSERT_GE(cgroup_fd2, 0, b"create cgroup for reuse\0".as_ptr() as *const c_char) {
                break 'close_cgroup;
            }

            cgroup_fd = test__join_cgroup(b"/sock_policy\0".as_ptr() as *const c_char);
            if !ASSERT_GE(cgroup_fd, 0, b"join_cgroup\0".as_ptr() as *const c_char) {
                break 'close_cgroup;
            }

            skel = lsm_cgroup__open_and_load();
            if !ASSERT_OK_PTR(skel as *const c_void, b"open_and_load\0".as_ptr() as *const c_char) {
                break 'close_cgroup;
            }

            post_create_prog_fd = bpf_program__fd((*skel).progs.socket_post_create);
            post_create_prog_fd2 = bpf_program__fd((*skel).progs.socket_post_create2);
            bind_prog_fd = bpf_program__fd((*skel).progs.socket_bind);
            bind_prog_fd2 = bpf_program__fd((*skel).progs.socket_bind2);
            alloc_prog_fd = bpf_program__fd((*skel).progs.socket_alloc);
            clone_prog_fd = bpf_program__fd((*skel).progs.socket_clone);

            ASSERT_EQ(query_prog_cnt(cgroup_fd, b"bpf_lsm_sk_alloc_security\0".as_ptr() as *const c_char) as c_int, 0, b"prog count\0".as_ptr() as *const c_char);
            ASSERT_EQ(query_prog_cnt(cgroup_fd, ptr::null()) as c_int, 0, b"total prog count\0".as_ptr() as *const c_char);
            err = bpf_prog_attach(alloc_prog_fd, cgroup_fd, BPF_LSM_CGROUP, 0);
            if err == -ENOTSUPP {
                test__skip();
                break 'close_cgroup;
            }
            if !ASSERT_OK(err, b"attach alloc_prog_fd\0".as_ptr() as *const c_char) {
                break 'detach_cgroup;
            }
            ASSERT_EQ(query_prog_cnt(cgroup_fd, b"bpf_lsm_sk_alloc_security\0".as_ptr() as *const c_char) as c_int, 1, b"prog count\0".as_ptr() as *const c_char);
            ASSERT_EQ(query_prog_cnt(cgroup_fd, ptr::null()) as c_int, 1, b"total prog count\0".as_ptr() as *const c_char);

            ASSERT_EQ(query_prog_cnt(cgroup_fd, b"bpf_lsm_inet_csk_clone\0".as_ptr() as *const c_char) as c_int, 0, b"prog count\0".as_ptr() as *const c_char);
            err = bpf_prog_attach(clone_prog_fd, cgroup_fd, BPF_LSM_CGROUP, 0);
            if !ASSERT_OK(err, b"attach clone_prog_fd\0".as_ptr() as *const c_char) {
                break 'detach_cgroup;
            }
            ASSERT_EQ(query_prog_cnt(cgroup_fd, b"bpf_lsm_inet_csk_clone\0".as_ptr() as *const c_char) as c_int, 1, b"prog count\0".as_ptr() as *const c_char);
            ASSERT_EQ(query_prog_cnt(cgroup_fd, ptr::null()) as c_int, 2, b"total prog count\0".as_ptr() as *const c_char);

            /* Make sure replacing works. */

            ASSERT_EQ(query_prog_cnt(cgroup_fd, b"bpf_lsm_socket_post_create\0".as_ptr() as *const c_char) as c_int, 0, b"prog count\0".as_ptr() as *const c_char);
            err = bpf_prog_attach(post_create_prog_fd, cgroup_fd, BPF_LSM_CGROUP, 0);
            if !ASSERT_OK(err, b"attach post_create_prog_fd\0".as_ptr() as *const c_char) {
                break 'detach_cgroup;
            }
            ASSERT_EQ(query_prog_cnt(cgroup_fd, b"bpf_lsm_socket_post_create\0".as_ptr() as *const c_char) as c_int, 1, b"prog count\0".as_ptr() as *const c_char);
            ASSERT_EQ(query_prog_cnt(cgroup_fd, ptr::null()) as c_int, 3, b"total prog count\0".as_ptr() as *const c_char);

            attach_opts.replace_prog_fd = post_create_prog_fd;
            err = bpf_prog_attach_opts(post_create_prog_fd2, cgroup_fd, BPF_LSM_CGROUP, &mut attach_opts);
            if !ASSERT_OK(err, b"prog replace post_create_prog_fd\0".as_ptr() as *const c_char) {
                break 'detach_cgroup;
            }
            ASSERT_EQ(query_prog_cnt(cgroup_fd, b"bpf_lsm_socket_post_create\0".as_ptr() as *const c_char) as c_int, 1, b"prog count\0".as_ptr() as *const c_char);
            ASSERT_EQ(query_prog_cnt(cgroup_fd, ptr::null()) as c_int, 3, b"total prog count\0".as_ptr() as *const c_char);

            /* Try the same attach/replace via link API. */

            ASSERT_EQ(query_prog_cnt(cgroup_fd, b"bpf_lsm_socket_bind\0".as_ptr() as *const c_char) as c_int, 0, b"prog count\0".as_ptr() as *const c_char);
            bind_link_fd = bpf_link_create(bind_prog_fd, cgroup_fd, BPF_LSM_CGROUP, ptr::null());
            if !ASSERT_GE(bind_link_fd, 0, b"link create bind_prog_fd\0".as_ptr() as *const c_char) {
                break 'detach_cgroup;
            }
            ASSERT_EQ(query_prog_cnt(cgroup_fd, b"bpf_lsm_socket_bind\0".as_ptr() as *const c_char) as c_int, 1, b"prog count\0".as_ptr() as *const c_char);
            ASSERT_EQ(query_prog_cnt(cgroup_fd, ptr::null()) as c_int, 4, b"total prog count\0".as_ptr() as *const c_char);

            update_opts.old_prog_fd = bind_prog_fd;
            update_opts.flags = BPF_F_REPLACE;

            err = bpf_link_update(bind_link_fd, bind_prog_fd2, &mut update_opts);
            if !ASSERT_OK(err, b"link update bind_prog_fd\0".as_ptr() as *const c_char) {
                break 'detach_cgroup;
            }
            ASSERT_EQ(query_prog_cnt(cgroup_fd, b"bpf_lsm_socket_bind\0".as_ptr() as *const c_char) as c_int, 1, b"prog count\0".as_ptr() as *const c_char);
            ASSERT_EQ(query_prog_cnt(cgroup_fd, ptr::null()) as c_int, 4, b"total prog count\0".as_ptr() as *const c_char);

            /* Attach another instance of bind program to another cgroup.
             * This should trigger the reuse of the trampoline shim (two
             * programs attaching to the same btf_id).
             */

            ASSERT_EQ(query_prog_cnt(cgroup_fd, b"bpf_lsm_socket_bind\0".as_ptr() as *const c_char) as c_int, 1, b"prog count\0".as_ptr() as *const c_char);
            ASSERT_EQ(query_prog_cnt(cgroup_fd2, b"bpf_lsm_socket_bind\0".as_ptr() as *const c_char) as c_int, 0, b"prog count\0".as_ptr() as *const c_char);
            bind_link_fd2 = bpf_link_create(bind_prog_fd2, cgroup_fd2, BPF_LSM_CGROUP, ptr::null());
            if !ASSERT_GE(bind_link_fd2, 0, b"link create bind_prog_fd2\0".as_ptr() as *const c_char) {
                break 'detach_cgroup;
            }
            ASSERT_EQ(query_prog_cnt(cgroup_fd2, b"bpf_lsm_socket_bind\0".as_ptr() as *const c_char) as c_int, 1, b"prog count\0".as_ptr() as *const c_char);
            ASSERT_EQ(query_prog_cnt(cgroup_fd, ptr::null()) as c_int, 4, b"total prog count\0".as_ptr() as *const c_char);
            ASSERT_EQ(query_prog_cnt(cgroup_fd2, ptr::null()) as c_int, 1, b"total prog count\0".as_ptr() as *const c_char);

            fd = socket(AF_UNIX, SOCK_STREAM, 0);
            if !((*(*skel).kconfig).CONFIG_SECURITY_APPARMOR
                || (*(*skel).kconfig).CONFIG_SECURITY_SELINUX
                || (*(*skel).kconfig).CONFIG_SECURITY_SMACK)
            {
                /* AF_UNIX is prohibited. */
                ASSERT_LT(fd, 0, b"socket(AF_UNIX)\0".as_ptr() as *const c_char);
            }
            close(fd);

            /* AF_INET6 gets default policy (sk_priority). */

            fd = socket(AF_INET6, SOCK_STREAM, 0);
            if !ASSERT_GE(fd, 0, b"socket(SOCK_STREAM)\0".as_ptr() as *const c_char) {
                break 'detach_cgroup;
            }

            prio = 0;
            socklen = size_of::<c_int>() as socklen_t;
            ASSERT_GE(
                getsockopt(fd, SOL_SOCKET, SO_PRIORITY, &mut prio as *mut _ as *mut c_void, &mut socklen),
                0,
                b"getsockopt\0".as_ptr() as *const c_char,
            );
            ASSERT_EQ(prio, 123, b"sk_priority\0".as_ptr() as *const c_char);

            close(fd);

            /* TX-only AF_PACKET is allowed. */

            ASSERT_LT(
                socket(AF_PACKET, SOCK_RAW, htons(ETH_P_ALL as u16) as c_int),
                0,
                b"socket(AF_PACKET, ..., ETH_P_ALL)\0".as_ptr() as *const c_char,
            );

            fd = socket(AF_PACKET, SOCK_RAW, 0);
            ASSERT_GE(fd, 0, b"socket(AF_PACKET, ..., 0)\0".as_ptr() as *const c_char);

            /* TX-only AF_PACKET can not be rebound. */

            let sa = sockaddr_ll {
                sll_family: AF_PACKET as c_uint,
                sll_protocol: htons(ETH_P_ALL as u16),
            };
            ASSERT_LT(
                bind(fd, &sa as *const _ as *const sockaddr, size_of::<sockaddr_ll>() as socklen_t),
                0,
                b"bind(ETH_P_ALL)\0".as_ptr() as *const c_char,
            );

            close(fd);

            /* Trigger passive open. */

            listen_fd = start_server(AF_INET6, SOCK_STREAM, b"::1\0".as_ptr() as *const c_char, 0, 0);
            ASSERT_GE(listen_fd, 0, b"start_server\0".as_ptr() as *const c_char);
            client_fd = connect_to_fd(listen_fd, 0);
            ASSERT_GE(client_fd, 0, b"connect_to_fd\0".as_ptr() as *const c_char);
            accepted_fd = accept(listen_fd, ptr::null_mut(), ptr::null_mut());
            ASSERT_GE(accepted_fd, 0, b"accept\0".as_ptr() as *const c_char);

            prio = 0;
            socklen = size_of::<c_int>() as socklen_t;
            ASSERT_GE(
                getsockopt(accepted_fd, SOL_SOCKET, SO_PRIORITY, &mut prio as *mut _ as *mut c_void, &mut socklen),
                0,
                b"getsockopt\0".as_ptr() as *const c_char,
            );
            ASSERT_EQ(prio, 234, b"sk_priority\0".as_ptr() as *const c_char);

            /* These are replaced and never called. */
            ASSERT_EQ((*(*skel).bss).called_socket_post_create, 0, b"called_create\0".as_ptr() as *const c_char);
            ASSERT_EQ((*(*skel).bss).called_socket_bind, 0, b"called_bind\0".as_ptr() as *const c_char);

            /* AF_INET6+SOCK_STREAM
             * AF_PACKET+SOCK_RAW
             * AF_UNIX+SOCK_RAW if already have non-bpf lsms installed
             * listen_fd
             * client_fd
             * accepted_fd
             */
            if (*(*skel).kconfig).CONFIG_SECURITY_APPARMOR
                || (*(*skel).kconfig).CONFIG_SECURITY_SELINUX
                || (*(*skel).kconfig).CONFIG_SECURITY_SMACK
            {
                /* AF_UNIX+SOCK_RAW if already have non-bpf lsms installed */
                ASSERT_EQ((*(*skel).bss).called_socket_post_create2, 6, b"called_create2\0".as_ptr() as *const c_char);
            } else {
                ASSERT_EQ((*(*skel).bss).called_socket_post_create2, 5, b"called_create2\0".as_ptr() as *const c_char);
            }

            /* start_server
             * bind(ETH_P_ALL)
             */
            ASSERT_EQ((*(*skel).bss).called_socket_bind2, 2, b"called_bind2\0".as_ptr() as *const c_char);
            /* Single accept(). */
            ASSERT_EQ((*(*skel).bss).called_socket_clone, 1, b"called_clone\0".as_ptr() as *const c_char);

            /* AF_UNIX+SOCK_STREAM (failed)
             * AF_INET6+SOCK_STREAM
             * AF_PACKET+SOCK_RAW (failed)
             * AF_PACKET+SOCK_RAW
             * listen_fd
             * client_fd
             * accepted_fd
             */
            ASSERT_EQ((*(*skel).bss).called_socket_alloc, 7, b"called_alloc\0".as_ptr() as *const c_char);

            close(listen_fd);
            close(client_fd);
            close(accepted_fd);

            /* Make sure other cgroup doesn't trigger the programs. */

            if !ASSERT_OK(join_cgroup(b"/sock_policy_empty\0".as_ptr() as *const c_char), b"join root cgroup\0".as_ptr() as *const c_char) {
                break 'detach_cgroup;
            }

            fd = socket(AF_INET6, SOCK_STREAM, 0);
            if !ASSERT_GE(fd, 0, b"socket(SOCK_STREAM)\0".as_ptr() as *const c_char) {
                break 'detach_cgroup;
            }

            prio = 0;
            socklen = size_of::<c_int>() as socklen_t;
            ASSERT_GE(
                getsockopt(fd, SOL_SOCKET, SO_PRIORITY, &mut prio as *mut _ as *mut c_void, &mut socklen),
                0,
                b"getsockopt\0".as_ptr() as *const c_char,
            );
            ASSERT_EQ(prio, 0, b"sk_priority\0".as_ptr() as *const c_char);

            close(fd);
        }

        ASSERT_GE(
            bpf_prog_detach2(post_create_prog_fd2, cgroup_fd, BPF_LSM_CGROUP),
            0,
            b"detach_create\0".as_ptr() as *const c_char,
        );
        close(bind_link_fd);
        /* Don't close bind_link_fd2, exercise cgroup release cleanup. */
        let _ = bind_link_fd2;
        ASSERT_GE(
            bpf_prog_detach2(alloc_prog_fd, cgroup_fd, BPF_LSM_CGROUP),
            0,
            b"detach_alloc\0".as_ptr() as *const c_char,
        );
        ASSERT_GE(
            bpf_prog_detach2(clone_prog_fd, cgroup_fd, BPF_LSM_CGROUP),
            0,
            b"detach_clone\0".as_ptr() as *const c_char,
        );
    }

    close(cgroup_fd);
    close(cgroup_fd2);
    close(cgroup_fd3);
    lsm_cgroup__destroy(skel);
}

unsafe fn test_lsm_cgroup_nonvoid() {
    let mut skel: *mut lsm_cgroup_nonvoid = ptr::null_mut();

    skel = lsm_cgroup_nonvoid__open_and_load();
    ASSERT_NULL(skel as *const c_void, b"open succeeds\0".as_ptr() as *const c_char);
    lsm_cgroup_nonvoid__destroy(skel);
}

unsafe fn test_lsm_cgroup_retval() {
    let mut skel: *mut lsm_cgroup = ptr::null_mut();
    let skipcap_prog_fd1: c_int;
    let skipcap_prog_fd2: c_int;
    let socket_prog_fd1: c_int;
    let socket_prog_fd2: c_int;
    let mut cgroup_fd: c_int = -1;
    let mut err: c_int;
    let mut fd: c_int;
    let mut tmpfile = *b"/tmp/test_lsm_cgroup_retval.XXXXXX\0";

    fd = mkstemp(tmpfile.as_mut_ptr() as *mut c_char);
    if !ASSERT_OK_FD(fd, b"mkstemp\0".as_ptr() as *const c_char) {
        return;
    }
    close(fd);

    'cleanup_tmpfile: {
        'cleanup_cgroup: {
            'cleanup_skeleton: {
                'cleanup_skipcap1: {
                    'cleanup_skipcap2: {
                        'cleanup_sock_create1: {
                            'cleanup_sock_create2: {
                                cgroup_fd = test__join_cgroup(b"/default_retval\0".as_ptr() as *const c_char);
                                if !ASSERT_OK_FD(cgroup_fd, b"join_cgroup\0".as_ptr() as *const c_char) {
                                    break 'cleanup_tmpfile;
                                }

                                skel = lsm_cgroup__open_and_load();
                                if !ASSERT_OK_PTR(skel as *const c_void, b"open_and_load\0".as_ptr() as *const c_char) {
                                    break 'cleanup_cgroup;
                                }

                                skipcap_prog_fd1 = bpf_program__fd((*skel).progs.skipcap_first);
                                skipcap_prog_fd2 = bpf_program__fd((*skel).progs.skipcap_second);
                                socket_prog_fd1 = bpf_program__fd((*skel).progs.socket_first);
                                socket_prog_fd2 = bpf_program__fd((*skel).progs.socket_second);

                                err = bpf_prog_attach(skipcap_prog_fd1, cgroup_fd, BPF_LSM_CGROUP, BPF_F_ALLOW_MULTI);
                                if err == -ENOTSUPP {
                                    test__skip();
                                    break 'cleanup_skeleton;
                                }
                                if !ASSERT_OK(err, b"attach first skipcap prog\0".as_ptr() as *const c_char) {
                                    break 'cleanup_skeleton;
                                }

                                err = bpf_prog_attach(skipcap_prog_fd2, cgroup_fd, BPF_LSM_CGROUP, BPF_F_ALLOW_MULTI);
                                if !ASSERT_OK(err, b"attach second skipcap prog\0".as_ptr() as *const c_char) {
                                    break 'cleanup_skipcap1;
                                }

                                err = bpf_prog_attach(socket_prog_fd1, cgroup_fd, BPF_LSM_CGROUP, BPF_F_ALLOW_MULTI);
                                if !ASSERT_OK(err, b"attach first sock_create prog\0".as_ptr() as *const c_char) {
                                    break 'cleanup_skipcap2;
                                }

                                err = bpf_prog_attach(socket_prog_fd2, cgroup_fd, BPF_LSM_CGROUP, BPF_F_ALLOW_MULTI);
                                if !ASSERT_OK(err, b"attach second sock_create prog\0".as_ptr() as *const c_char) {
                                    break 'cleanup_sock_create1;
                                }

                                /* trigger the bool hook by setxattr */
                                err = setxattr(
                                    tmpfile.as_ptr() as *const c_char,
                                    b"user.test\0".as_ptr() as *const c_char,
                                    b"value\0".as_ptr() as *const c_void,
                                    5,
                                    0,
                                );
                                if !ASSERT_OK(err, b"setxattr\0".as_ptr() as *const c_char) {
                                    break 'cleanup_sock_create2;
                                }

                                /* trigger the errno hook by creating a socket */
                                fd = socket(AF_INET, SOCK_STREAM, 0);
                                if !ASSERT_OK_FD(fd, b"socket\0".as_ptr() as *const c_char) {
                                    break 'cleanup_sock_create2;
                                }
                                close(fd);

                                ASSERT_EQ((*(*skel).data).skipcap_retval, 0, b"bool_hook_retval_should_be_0\0".as_ptr() as *const c_char);
                                ASSERT_EQ((*(*skel).data).socket_retval, -EPERM, b"errno_hook_retval_should_be_EPERM\0".as_ptr() as *const c_char);
                            }
                            bpf_prog_detach2(socket_prog_fd2, cgroup_fd, BPF_LSM_CGROUP);
                        }
                        bpf_prog_detach2(socket_prog_fd1, cgroup_fd, BPF_LSM_CGROUP);
                    }
                    bpf_prog_detach2(skipcap_prog_fd2, cgroup_fd, BPF_LSM_CGROUP);
                }
                bpf_prog_detach2(skipcap_prog_fd1, cgroup_fd, BPF_LSM_CGROUP);
            }
            lsm_cgroup__destroy(skel);
        }
        close(cgroup_fd);
    }
    unlink(tmpfile.as_ptr() as *const c_char);
}

#[no_mangle]
pub unsafe extern "C" fn test_lsm_cgroup() {
    if test__start_subtest(b"functional\0".as_ptr() as *const c_char) {
        test_lsm_cgroup_functional();
    }
    if test__start_subtest(b"nonvoid\0".as_ptr() as *const c_char) {
        test_lsm_cgroup_nonvoid();
    }
    if test__start_subtest(b"retval\0".as_ptr() as *const c_char) {
        test_lsm_cgroup_retval();
    }
    btf__free(btf);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
