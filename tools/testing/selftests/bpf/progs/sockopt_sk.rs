// SPDX-License-Identifier: GPL-2.0
// Dependencies from the original C source:
// string.h, linux/tcp.h, linux/bpf.h, netinet/in.h, bpf/bpf_helpers.h

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type __u8 = u8;
type __u32 = u32;
type __u64 = u64;

extern "C" {
    static mut AF_NETLINK: i32;
    static mut AF_INET: i32;
    static mut SOCK_RAW: i32;
    static mut SOL_IP: i32;
    static mut SOL_SOCKET: i32;
    static mut IP_TOS: i32;
    static mut IP_FREEBIND: i32;
    static mut SO_SNDBUF: i32;
    static mut IPPROTO_TCP: i32;
    static mut TCP_CONGESTION: i32;
    static mut TCP_ZEROCOPY_RECEIVE: i32;
    static mut TCP_SAVED_SYN: i32;
    static mut BPF_MAP_TYPE_SK_STORAGE: u32;
    static mut BPF_F_NO_PREALLOC: u32;
    static mut BPF_SK_STORAGE_GET_F_CREATE: u64;

    fn bpf_get_netns_cookie(ctx: *mut core::ffi::c_void) -> __u64;
    fn bpf_sk_storage_get(
        map: *mut core::ffi::c_void,
        sk: *mut bpf_sock,
        value: *mut core::ffi::c_void,
        flags: u64,
    ) -> *mut sockopt_sk;
    fn bpf_tcp_sock(sk: *mut bpf_sock) -> *mut bpf_tcp_sock;
    fn bpf_getsockopt(
        sk: *mut bpf_sock,
        level: i32,
        optname: i32,
        optval: *mut core::ffi::c_void,
        optlen: i32,
    ) -> i32;
}

// #ifndef SOL_TCP
// #define SOL_TCP IPPROTO_TCP
// #endif
unsafe fn SOL_TCP() -> i32 {
    IPPROTO_TCP
}

const SOL_CUSTOM: i32 = 0xdeadbeefu32 as i32;

#[repr(C)]
pub struct bpf_sock {
    pub family: i32,
    pub type_: i32,
}

#[repr(C)]
pub struct bpf_tcp_sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_sockopt {
    pub sk: *mut bpf_sock,
    pub optval: *mut __u8,
    pub optval_end: *mut __u8,
    pub level: i32,
    pub optname: i32,
    pub optlen: i32,
    pub retval: i32,
}

#[repr(C)]
pub struct tcp_zerocopy_receive {
    pub address: __u64,
}

#[repr(C)]
pub struct sockopt_sk {
    pub val: __u8,
}

#[used]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

pub static mut page_size: i32 = 0; /* userspace should set it */

#[repr(C)]
pub struct socket_storage_map_def {
    pub type_: u32,
    pub map_flags: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[used]
#[link_section = ".maps"]
pub static mut socket_storage_map: socket_storage_map_def = socket_storage_map_def {
    type_: 0,
    map_flags: 0,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<sockopt_sk>() as u32,
};

#[link_section = "cgroup/getsockopt"]
pub unsafe extern "C" fn _getsockopt(ctx: *mut bpf_sockopt) -> i32 {
    let optval_end: *mut __u8 = (*ctx).optval_end;
    let optval: *mut __u8 = (*ctx).optval;
    let mut storage: *mut sockopt_sk;
    let mut sk: *mut bpf_sock;

    'out: loop {
        /* Bypass AF_NETLINK. */
        sk = (*ctx).sk;
        if !sk.is_null() && (*sk).family == AF_NETLINK {
            break 'out;
        }

        /* Make sure bpf_get_netns_cookie is callable.
         */
        if bpf_get_netns_cookie(core::ptr::null_mut()) == 0 {
            return 0;
        }

        if bpf_get_netns_cookie(ctx as *mut core::ffi::c_void) == 0 {
            return 0;
        }

        if (*ctx).level == SOL_IP && (*ctx).optname == IP_TOS {
            /* Not interested in SOL_IP:IP_TOS;
             * let next BPF program in the cgroup chain or kernel
             * handle it.
             */
            break 'out;
        }

        if (*ctx).level == SOL_SOCKET && (*ctx).optname == SO_SNDBUF {
            /* Not interested in SOL_SOCKET:SO_SNDBUF;
             * let next BPF program in the cgroup chain or kernel
             * handle it.
             */
            break 'out;
        }

        if (*ctx).level == SOL_TCP() && (*ctx).optname == TCP_CONGESTION {
            /* Not interested in SOL_TCP:TCP_CONGESTION;
             * let next BPF program in the cgroup chain or kernel
             * handle it.
             */
            break 'out;
        }

        if (*ctx).level == SOL_TCP() && (*ctx).optname == TCP_ZEROCOPY_RECEIVE {
            /* Verify that TCP_ZEROCOPY_RECEIVE triggers.
             * It has a custom implementation for performance
             * reasons.
             */

            /* Check that optval contains address (__u64) */
            if optval.add(core::mem::size_of::<__u64>()) > optval_end {
                return 0; /* bounds check */
            }

            if (*(optval as *mut tcp_zerocopy_receive)).address != 0 {
                return 0; /* unexpected data */
            }

            break 'out;
        }

        if (*ctx).level == SOL_IP && (*ctx).optname == IP_FREEBIND {
            if optval.add(1) > optval_end {
                return 0; /* bounds check */
            }

            (*ctx).retval = 0; /* Reset system call return value to zero */

            /* Always export 0x55 */
            *optval.add(0) = 0x55;
            (*ctx).optlen = 1;

            /* Userspace buffer is PAGE_SIZE * 2, but BPF
             * program can only see the first PAGE_SIZE
             * bytes of data.
             */
            if optval_end.offset_from(optval) != page_size as isize {
                return 0; /* unexpected data size */
            }

            return 1;
        }

        if (*ctx).level != SOL_CUSTOM {
            return 0; /* deny everything except custom level */
        }

        if optval.add(1) > optval_end {
            return 0; /* bounds check */
        }

        storage = bpf_sk_storage_get(
            &mut socket_storage_map as *mut _ as *mut core::ffi::c_void,
            (*ctx).sk,
            core::ptr::null_mut(),
            BPF_SK_STORAGE_GET_F_CREATE,
        );
        if storage.is_null() {
            return 0; /* couldn't get sk storage */
        }

        if (*ctx).retval == 0 {
            return 0; /* kernel should not have handled
                       * SOL_CUSTOM, something is wrong!
                       */
        }
        (*ctx).retval = 0; /* Reset system call return value to zero */

        *optval.add(0) = (*storage).val;
        (*ctx).optlen = 1;

        return 1;
    }

    /* optval larger than PAGE_SIZE use kernel's buffer. */
    if (*ctx).optlen > page_size {
        (*ctx).optlen = 0;
    }
    return 1;
}

#[link_section = "cgroup/setsockopt"]
pub unsafe extern "C" fn _setsockopt(ctx: *mut bpf_sockopt) -> i32 {
    let optval_end: *mut __u8 = (*ctx).optval_end;
    let optval: *mut __u8 = (*ctx).optval;
    let mut storage: *mut sockopt_sk;
    let mut sk: *mut bpf_sock;

    'out: loop {
        'consumed: loop {
            /* Bypass AF_NETLINK. */
            sk = (*ctx).sk;
            if !sk.is_null() && (*sk).family == AF_NETLINK {
                break 'out;
            }

            if !sk.is_null() && (*sk).family == AF_INET && (*sk).type_ == SOCK_RAW {
                let tp: *mut bpf_tcp_sock = bpf_tcp_sock(sk);

                if !tp.is_null() {
                    let mut saved_syn: [i8; 60] = [0; 60];

                    bpf_getsockopt(
                        sk,
                        SOL_TCP(),
                        TCP_SAVED_SYN,
                        saved_syn.as_mut_ptr() as *mut core::ffi::c_void,
                        core::mem::size_of_val(&saved_syn) as i32,
                    );
                    break 'consumed;
                }

                break 'out;
            }

            /* Make sure bpf_get_netns_cookie is callable.
             */
            if bpf_get_netns_cookie(core::ptr::null_mut()) == 0 {
                return 0;
            }

            if bpf_get_netns_cookie(ctx as *mut core::ffi::c_void) == 0 {
                return 0;
            }

            if (*ctx).level == SOL_IP && (*ctx).optname == IP_TOS {
                /* Not interested in SOL_IP:IP_TOS;
                 * let next BPF program in the cgroup chain or kernel
                 * handle it.
                 */
                (*ctx).optlen = 0; /* bypass optval>PAGE_SIZE */
                return 1;
            }

            if (*ctx).level == SOL_SOCKET && (*ctx).optname == SO_SNDBUF {
                /* Overwrite SO_SNDBUF value */

                if optval.add(core::mem::size_of::<__u32>()) > optval_end {
                    return 0; /* bounds check */
                }

                *(optval as *mut __u32) = 0x55AA;
                (*ctx).optlen = 4;

                return 1;
            }

            if (*ctx).level == SOL_TCP() && (*ctx).optname == TCP_CONGESTION {
                /* Always use cubic */

                if optval.add(5) > optval_end {
                    return 0; /* bounds check */
                }

                core::ptr::copy_nonoverlapping(b"cubic".as_ptr(), optval, 5);
                (*ctx).optlen = 5;

                return 1;
            }

            if (*ctx).level == SOL_IP && (*ctx).optname == IP_FREEBIND {
                /* Original optlen is larger than PAGE_SIZE. */
                if (*ctx).optlen != page_size * 2 {
                    return 0; /* unexpected data size */
                }

                if optval.add(1) > optval_end {
                    return 0; /* bounds check */
                }

                /* Make sure we can trim the buffer. */
                *optval.add(0) = 0;
                (*ctx).optlen = 1;

                /* Usepace buffer is PAGE_SIZE * 2, but BPF
                 * program can only see the first PAGE_SIZE
                 * bytes of data.
                 */
                if optval_end.offset_from(optval) != page_size as isize {
                    return 0; /* unexpected data size */
                }

                return 1;
            }

            if (*ctx).level != SOL_CUSTOM {
                return 0; /* deny everything except custom level */
            }

            if optval.add(1) > optval_end {
                return 0; /* bounds check */
            }

            storage = bpf_sk_storage_get(
                &mut socket_storage_map as *mut _ as *mut core::ffi::c_void,
                (*ctx).sk,
                core::ptr::null_mut(),
                BPF_SK_STORAGE_GET_F_CREATE,
            );
            if storage.is_null() {
                return 0; /* couldn't get sk storage */
            }

            (*storage).val = *optval.add(0);

            break 'consumed;
        }

        (*ctx).optlen = -1; /* BPF has consumed this option, don't call kernel
                             * setsockopt handler.
                             */

        return 1;
    }

    /* optval larger than PAGE_SIZE use kernel's buffer. */
    if (*ctx).optlen > page_size {
        (*ctx).optlen = 0;
    }
    return 1;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
