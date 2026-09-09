// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2018 Facebook
 *
 * BPF program to automatically reflect TOS option from received syn packet
 *
 * Use "bpftool cgroup attach $cg sock_ops $prog" to load this BPF program.
 */

// C headers supplied by the surrounding BPF environment are intentionally
// omitted; their types, constants, helpers, and section mechanism remain
// external dependencies of this translation.

const DEBUG: i32 = 1;

// The following names are supplied by the BPF headers/environment:
// bpf_sock_ops, ipv6hdr, iphdr, bpf_printk, bpf_setsockopt, bpf_getsockopt,
// SEC, BPF_SOCK_OPS_TCP_LISTEN_CB, BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB,
// SOL_TCP, TCP_SAVE_SYN, TCP_SAVED_SYN, AF_INET, SOL_IP, IP_TOS, SOL_IPV6,
// IPV6_TCLASS.

#[allow(non_snake_case)]
pub unsafe fn bpf_basertt(skops: *mut bpf_sock_ops) -> i32 {
    let mut header: [u8; core::mem::size_of::<ipv6hdr>()] =
        [0; core::mem::size_of::<ipv6hdr>()];
    let mut hdr6: *mut ipv6hdr;
    let mut hdr: *mut iphdr;
    let mut hdr_size: i32 = 0;
    let mut save_syn: i32 = 1;
    let mut tos: i32 = 0;
    let mut rv: i32 = 0;
    let op: i32 = (*skops).op as i32;

    #[cfg(feature = "DEBUG")]
    {
        bpf_printk!("BPF command: %d\n", op);
    }

    match op {
        BPF_SOCK_OPS_TCP_LISTEN_CB => {
            rv = bpf_setsockopt(
                skops,
                SOL_TCP,
                TCP_SAVE_SYN,
                &mut save_syn as *mut i32 as *mut core::ffi::c_void,
                core::mem::size_of_val(&save_syn) as i32,
            );
        }
        BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB => {
            if (*skops).family == AF_INET {
                hdr_size = core::mem::size_of::<iphdr>() as i32;
            } else {
                hdr_size = core::mem::size_of::<ipv6hdr>() as i32;
            }
            rv = bpf_getsockopt(
                skops,
                SOL_TCP,
                TCP_SAVED_SYN,
                header.as_mut_ptr() as *mut core::ffi::c_void,
                hdr_size,
            );
            if rv == 0 {
                if (*skops).family == AF_INET {
                    hdr = header.as_mut_ptr() as *mut iphdr;
                    tos = (*hdr).tos as i32;
                    if tos != 0 {
                        bpf_setsockopt(
                            skops,
                            SOL_IP,
                            IP_TOS,
                            &mut tos as *mut i32 as *mut core::ffi::c_void,
                            core::mem::size_of_val(&tos) as i32,
                        );
                    }
                } else {
                    hdr6 = header.as_mut_ptr() as *mut ipv6hdr;
                    tos = (((*hdr6).priority as i32) << 4)
                        | (((*hdr6).flow_lbl[0] as i32) >> 4);
                    if tos != 0 {
                        bpf_setsockopt(
                            skops,
                            SOL_IPV6,
                            IPV6_TCLASS,
                            &mut tos as *mut i32 as *mut core::ffi::c_void,
                            core::mem::size_of_val(&tos) as i32,
                        );
                    }
                }
                rv = 0;
            }
        }
        _ => {
            rv = -1;
        }
    }

    #[cfg(feature = "DEBUG")]
    {
        bpf_printk!("Returning %d\n", rv);
    }
    (*skops).reply = rv;
    1
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
