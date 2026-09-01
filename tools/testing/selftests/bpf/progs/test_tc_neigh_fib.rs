// SPDX-License-Identifier: GPL-2.0
//
// C dependencies translated as Rust dependency intent:
// <stdint.h>, <stdbool.h>, <stddef.h>
// <linux/bpf.h>, <linux/stddef.h>, <linux/pkt_cls.h>, <linux/if_ether.h>,
// <linux/in.h>, <linux/ip.h>, <linux/ipv6.h>
// <bpf/bpf_helpers.h>, <bpf/bpf_endian.h>

const AF_INET: u32 = 2;
const AF_INET6: u32 = 10;

#[inline(always)]
unsafe fn ctx_ptr(field: u32) -> *mut core::ffi::c_void {
    field as i64 as *mut core::ffi::c_void
}

#[inline(always)]
unsafe fn fill_fib_params_v4(
    skb: *mut __sk_buff,
    fib_params: *mut bpf_fib_lookup,
) -> i32 {
    let data_end = ctx_ptr((*skb).data_end);
    let data = ctx_ptr((*skb).data);
    let ip4h: *mut iphdr;

    if (data as usize).wrapping_add(core::mem::size_of::<ethhdr>()) > data_end as usize {
        return -1;
    }

    ip4h = (data as *mut u8).add(core::mem::size_of::<ethhdr>()) as *mut iphdr;
    if ip4h.add(1) as *mut core::ffi::c_void > data_end {
        return -1;
    }

    (*fib_params).family = AF_INET;
    (*fib_params).tos = (*ip4h).tos;
    (*fib_params).l4_protocol = (*ip4h).protocol;
    (*fib_params).sport = 0;
    (*fib_params).dport = 0;
    (*fib_params).tot_len = bpf_ntohs((*ip4h).tot_len);
    (*fib_params).ipv4_src = (*ip4h).saddr;
    (*fib_params).ipv4_dst = (*ip4h).daddr;

    0
}

#[inline(always)]
unsafe fn fill_fib_params_v6(
    skb: *mut __sk_buff,
    fib_params: *mut bpf_fib_lookup,
) -> i32 {
    let src = (*fib_params).ipv6_src.as_mut_ptr() as *mut in6_addr;
    let dst = (*fib_params).ipv6_dst.as_mut_ptr() as *mut in6_addr;
    let data_end = ctx_ptr((*skb).data_end);
    let data = ctx_ptr((*skb).data);
    let ip6h: *mut ipv6hdr;

    if (data as usize).wrapping_add(core::mem::size_of::<ethhdr>()) > data_end as usize {
        return -1;
    }

    ip6h = (data as *mut u8).add(core::mem::size_of::<ethhdr>()) as *mut ipv6hdr;
    if ip6h.add(1) as *mut core::ffi::c_void > data_end {
        return -1;
    }

    (*fib_params).family = AF_INET6;
    (*fib_params).flowinfo = 0;
    (*fib_params).l4_protocol = (*ip6h).nexthdr;
    (*fib_params).sport = 0;
    (*fib_params).dport = 0;
    (*fib_params).tot_len = bpf_ntohs((*ip6h).payload_len);
    *src = (*ip6h).saddr;
    *dst = (*ip6h).daddr;

    0
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn tc_chk(skb: *mut __sk_buff) -> i32 {
    let data_end = ctx_ptr((*skb).data_end);
    let data = ctx_ptr((*skb).data);
    let raw = data as *mut u32;

    if (data as usize).wrapping_add(core::mem::size_of::<ethhdr>()) > data_end as usize {
        return TC_ACT_SHOT;
    }

    if *raw.add(0) == 0 && *raw.add(1) == 0 && *raw.add(2) == 0 {
        TC_ACT_SHOT
    } else {
        TC_ACT_OK
    }
}

#[inline(always)]
unsafe fn tc_redir(skb: *mut __sk_buff) -> i32 {
    let mut fib_params: bpf_fib_lookup = core::mem::zeroed();
    fib_params.ifindex = (*skb).ingress_ifindex;
    let mut zero: [u8; ETH_ALEN * 2] = [0; ETH_ALEN * 2];
    let mut ret: i32 = -1;

    match (*skb).protocol {
        x if x == __bpf_constant_htons(ETH_P_IP) => {
            ret = fill_fib_params_v4(skb, &mut fib_params);
        }
        x if x == __bpf_constant_htons(ETH_P_IPV6) => {
            ret = fill_fib_params_v6(skb, &mut fib_params);
        }
        _ => {}
    }

    if ret != 0 {
        return TC_ACT_OK;
    }

    ret = bpf_fib_lookup(
        skb,
        &mut fib_params,
        core::mem::size_of::<bpf_fib_lookup>() as u32,
        0,
    );
    if ret == BPF_FIB_LKUP_RET_NOT_FWDED || ret < 0 {
        return TC_ACT_OK;
    }

    zero.fill(0);
    if bpf_skb_store_bytes(
        skb,
        0,
        zero.as_mut_ptr() as *mut core::ffi::c_void,
        core::mem::size_of_val(&zero) as u32,
        0,
    ) < 0
    {
        return TC_ACT_SHOT;
    }

    if ret == BPF_FIB_LKUP_RET_NO_NEIGH {
        let mut nh_params: bpf_redir_neigh = core::mem::zeroed();

        nh_params.nh_family = fib_params.family;
        core::ptr::copy_nonoverlapping(
            fib_params.ipv6_dst.as_ptr() as *const u8,
            nh_params.ipv6_nh.as_mut_ptr() as *mut u8,
            core::mem::size_of_val(&nh_params.ipv6_nh),
        );

        return bpf_redirect_neigh(
            fib_params.ifindex,
            &mut nh_params,
            core::mem::size_of::<bpf_redir_neigh>() as u32,
            0,
        );
    } else if ret == BPF_FIB_LKUP_RET_SUCCESS {
        let data_end = ctx_ptr((*skb).data_end);
        let eth = ctx_ptr((*skb).data) as *mut ethhdr;

        if eth.add(1) as *mut core::ffi::c_void > data_end {
            return TC_ACT_SHOT;
        }

        core::ptr::copy_nonoverlapping(
            fib_params.dmac.as_ptr(),
            (*eth).h_dest.as_mut_ptr(),
            ETH_ALEN,
        );
        core::ptr::copy_nonoverlapping(
            fib_params.smac.as_ptr(),
            (*eth).h_source.as_mut_ptr(),
            ETH_ALEN,
        );

        return bpf_redirect(fib_params.ifindex, 0);
    }

    TC_ACT_SHOT
}

/* these are identical, but keep them separate for compatibility with the
 * section names expected by test_tc_redirect.sh
 */
#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn tc_dst(skb: *mut __sk_buff) -> i32 {
    tc_redir(skb)
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn tc_src(skb: *mut __sk_buff) -> i32 {
    tc_redir(skb)
}

#[no_mangle]
#[link_section = "license"]
pub static __license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
