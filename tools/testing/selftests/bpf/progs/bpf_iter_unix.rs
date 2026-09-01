// SPDX-License-Identifier: GPL-2.0
/* Copyright Amazon.com Inc. or its affiliates. */
/* Dependencies in the original C source:
 * <vmlinux.h>
 * "bpf_tracing_net.h"
 * <bpf/bpf_helpers.h>
 * <bpf/bpf_endian.h>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type __u32 = u32;
type __u64 = u64;

const BPF_MAP_TYPE_SOCKMAP: u32 = 15;
const TCP_LISTEN: u32 = 10;
const TCP_ESTABLISHED: u32 = 1;
const __SO_ACCEPTCON: u32 = 1 << 16;
const SS_CONNECTED: u32 = 1;
const SS_UNCONNECTED: u32 = 2;
const SS_CONNECTING: u32 = 3;
const SS_DISCONNECTING: u32 = 4;

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct sockmap_def {
    /* Original BPF map declaration:
     * __uint(type, BPF_MAP_TYPE_SOCKMAP);
     * __uint(max_entries, 1);
     * __type(key, __u32);
     * __type(value, __u64);
     */
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut sockmap: sockmap_def = sockmap_def {
    type_: BPF_MAP_TYPE_SOCKMAP,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>() as u32,
    value_size: core::mem::size_of::<__u64>() as u32,
};

#[repr(C)]
pub struct refcount_struct {
    pub refs: atomic_t,
}

#[repr(C)]
pub struct atomic_t {
    pub counter: i32,
}

#[repr(C)]
pub struct sock {
    pub sk_socket: *mut socket,
    pub sk_refcnt: refcount_struct,
    pub sk_state: u32,
    pub sk_type: u16,
}

#[repr(C)]
pub struct socket;

#[repr(C)]
pub struct socket_alloc {
    pub socket: socket,
    pub vfs_inode: inode,
}

#[repr(C)]
pub struct inode {
    pub i_ino: core::ffi::c_ulong,
}

#[repr(C)]
pub struct unix_sock {
    pub sk: sock,
    pub addr: *mut unix_address,
}

#[repr(C)]
pub struct unix_address {
    pub len: __u64,
    pub name: *mut sockaddr_un,
}

#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: i16,
    pub sun_path: [i8; 108],
}

#[repr(C)]
pub struct seq_file;

#[repr(C)]
pub struct bpf_iter_meta {
    pub seq: *mut seq_file,
    pub seq_num: __u32,
}

#[repr(C)]
pub struct bpf_iter__unix {
    pub meta: *mut bpf_iter_meta,
    pub unix_sk: *mut unix_sock,
}

unsafe extern "C" {
    fn bpf_probe_read_kernel(
        dst: *mut core::ffi::c_void,
        size: __u32,
        unsafe_ptr: *const core::ffi::c_void,
    ) -> i64;
    fn bpf_map_update_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: __u64,
    ) -> i64;
    fn bpf_seq_printf(
        seq: *mut seq_file,
        fmt: *const core::ffi::c_char,
        fmt_size: __u32,
        data: *const __u64,
        data_len: __u32,
    ) -> i64;
}

unsafe fn BPF_SEQ_PRINTF(
    seq: *mut seq_file,
    fmt: *const core::ffi::c_char,
    data: *const __u64,
    data_len: __u32,
) -> i64 {
    unsafe { bpf_seq_printf(seq, fmt, 0, data, data_len) }
}

unsafe fn sock_i_ino(sk: *const sock) -> core::ffi::c_long {
    let sk_socket: *const socket = unsafe { (*sk).sk_socket };
    let inode: *const inode;
    let mut ino: core::ffi::c_ulong = 0;

    if sk_socket.is_null() {
        return 0;
    }

    /* container_of(sk_socket, struct socket_alloc, socket)->vfs_inode */
    inode = unsafe {
        &(*(sk_socket as *const socket_alloc)).vfs_inode as *const inode
    };
    unsafe {
        bpf_probe_read_kernel(
            &mut ino as *mut _ as *mut core::ffi::c_void,
            core::mem::size_of_val(&ino) as __u32,
            &(*inode).i_ino as *const _ as *const core::ffi::c_void,
        );
    }
    ino as core::ffi::c_long
}

#[unsafe(link_section = "iter/unix")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dump_unix(ctx: *mut bpf_iter__unix) -> i32 {
    let unix_sk: *mut unix_sock = unsafe { (*ctx).unix_sk };
    let sk: *mut sock = unix_sk as *mut sock;
    let seq: *mut seq_file;
    let seq_num: __u32;

    if unix_sk.is_null() {
        return 0;
    }

    seq = unsafe { (*(*ctx).meta).seq };
    seq_num = unsafe { (*(*ctx).meta).seq_num };
    if seq_num == 0 {
        unsafe {
            BPF_SEQ_PRINTF(
                seq,
                b"Num               RefCount Protocol Flags    Type St    Inode Path\n\0".as_ptr()
                    as *const core::ffi::c_char,
                core::ptr::null(),
                0,
            );
        }
    }

    let flags: u32 = unsafe {
        if (*sk).sk_state == TCP_LISTEN {
            __SO_ACCEPTCON
        } else {
            0
        }
    };
    let st: u32 = unsafe {
        if !(*sk).sk_socket.is_null() {
            if (*sk).sk_state == TCP_ESTABLISHED {
                SS_CONNECTED
            } else {
                SS_UNCONNECTED
            }
        } else if (*sk).sk_state == TCP_ESTABLISHED {
            SS_CONNECTING
        } else {
            SS_DISCONNECTING
        }
    };
    let ino = unsafe { sock_i_ino(sk) };
    let args: [__u64; 7] = unsafe {
        [
            unix_sk as __u64,
            (*sk).sk_refcnt.refs.counter as __u32 as __u64,
            0,
            flags as __u64,
            (*sk).sk_type as __u64,
            st as __u64,
            ino as __u64,
        ]
    };
    unsafe {
        BPF_SEQ_PRINTF(
            seq,
            b"%pK: %08X %08X %08X %04X %02X %8lu\0".as_ptr() as *const core::ffi::c_char,
            args.as_ptr(),
            core::mem::size_of_val(&args) as __u32,
        );
    }

    if unsafe { !(*unix_sk).addr.is_null() } {
        if unsafe { (*(*(*unix_sk).addr).name).sun_path[0] != 0 } {
            let args: [__u64; 1] = unsafe { [(*(*(*unix_sk).addr).name).sun_path.as_ptr() as __u64] };
            unsafe {
                BPF_SEQ_PRINTF(
                    seq,
                    b" %s\0".as_ptr() as *const core::ffi::c_char,
                    args.as_ptr(),
                    core::mem::size_of_val(&args) as __u32,
                );
            }
        } else {
            /* The name of the abstract UNIX domain socket starts
             * with '\0' and can contain '\0'.  The null bytes
             * should be escaped as done in unix_seq_show().
             */
            let mut i: __u64;
            let len: __u64;

            len = unsafe { (*(*unix_sk).addr).len.wrapping_sub(core::mem::size_of::<i16>() as __u64) };

            unsafe {
                BPF_SEQ_PRINTF(
                    seq,
                    b" @\0".as_ptr() as *const core::ffi::c_char,
                    core::ptr::null(),
                    0,
                );
            }

            i = 1;
            while i < len {
                /* unix_validate_addr() tests this upper bound. */
                if i >= core::mem::size_of::<sockaddr_un>() as __u64 {
                    break;
                }

                let ch = unsafe {
                    let c = (*(*(*unix_sk).addr).name).sun_path[i as usize];
                    if c != 0 {
                        c
                    } else {
                        b'@' as i8
                    }
                };
                let args: [__u64; 1] = [ch as u8 as __u64];
                unsafe {
                    BPF_SEQ_PRINTF(
                        seq,
                        b"%c\0".as_ptr() as *const core::ffi::c_char,
                        args.as_ptr(),
                        core::mem::size_of_val(&args) as __u32,
                    );
                }
                i = i.wrapping_add(1);
            }
        }
    }

    unsafe {
        BPF_SEQ_PRINTF(
            seq,
            b"\n\0".as_ptr() as *const core::ffi::c_char,
            core::ptr::null(),
            0,
        );
    }

    /* Test for deadlock. */
    let key: i32 = 0;
    unsafe {
        bpf_map_update_elem(
            &raw mut sockmap as *mut _ as *mut core::ffi::c_void,
            &key as *const _ as *const core::ffi::c_void,
            sk as *const core::ffi::c_void,
            0,
        );
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
