// SPDX-License-Identifier: GPL-2.0
// C dependencies: <inttypes.h>, <linux/bpf.h>, <bpf/bpf_endian.h>,
// <bpf/bpf_helpers.h>, <linux/if_ether.h>, <linux/ip.h>

/* This function extracts the last byte of the daddr, and uses it
 * as output dev index.
 */
#[unsafe(link_section = "lwt_xmit")]
pub unsafe extern "C" fn test_lwt_reroute(skb: *mut __sk_buff) -> i32 {
    let mut iph: *mut iphdr = core::ptr::null_mut();
    let start: *mut core::ffi::c_void = (*skb).data as usize as *mut core::ffi::c_void;
    let end: *mut core::ffi::c_void = (*skb).data_end as usize as *mut core::ffi::c_void;

    /* set mark at most once */
    if (*skb).mark != 0 {
        return BPF_OK;
    }

    if (start as *mut u8).add(core::mem::size_of_val(&*iph)) > end as *mut u8 {
        return BPF_DROP;
    }

    iph = start as *mut iphdr;
    (*skb).mark = bpf_ntohl((*iph).daddr) & 0xff;

    /* do not reroute x.x.x.0 packets */
    if (*skb).mark == 0 {
        return BPF_OK;
    }

    BPF_LWT_REROUTE
}

#[unsafe(link_section = "license")]
#[used]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
