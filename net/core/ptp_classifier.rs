// SPDX-License-Identifier: GPL-2.0-only
// PTP classifier. The bpf program below is the direct translation of the
// opcode array in the ptp_filter structure.

// C dependencies supplied by the surrounding kernel translation unit:
// linux/skbuff.h, linux/filter.h, and linux/ptp_classify.h.

static mut ptp_insns: *mut bpf_prog = core::ptr::null_mut();

pub unsafe fn ptp_classify_raw(skb: *const sk_buff) -> u32 {
    bpf_prog_run(ptp_insns, skb)
}

pub unsafe fn ptp_parse_header(skb: *mut sk_buff, type_: u32) -> *mut ptp_header {
    let mut ptr: *mut u8 = skb_mac_header(skb);

    if (type_ & PTP_CLASS_VLAN) != 0 {
        ptr = ptr.add(VLAN_HLEN as usize);
    }

    match type_ & PTP_CLASS_PMASK {
        PTP_CLASS_IPV4 => {
            ptr = ptr.add((IPV4_HLEN(ptr) + UDP_HLEN) as usize);
        }
        PTP_CLASS_IPV6 => {
            ptr = ptr.add((IP6_HLEN + UDP_HLEN) as usize);
        }
        PTP_CLASS_L2 => {}
        _ => return core::ptr::null_mut(),
    }

    ptr = ptr.add(ETH_HLEN as usize);

    // Ensure that the entire header is present in this packet.
    if ptr.add(core::mem::size_of::<ptp_header>())
        > (*skb).data.add((*skb).len as usize)
    {
        return core::ptr::null_mut();
    }

    ptr as *mut ptp_header
}

pub unsafe fn ptp_msg_is_sync(skb: *mut sk_buff, type_: u32) -> bool {
    let hdr: *mut ptp_header = ptp_parse_header(skb, type_);
    if hdr.is_null() {
        return false;
    }

    ptp_get_msgtype(hdr, type_) == PTP_MSGTYPE_SYNC
}

pub unsafe fn ptp_classifier_init() {
    static mut ptp_filter: [sock_filter; 60] = [
        sock_filter { code: 0x28, jt: 0, jf: 0, k: 0x0000000c }, sock_filter { code: 0x15, jt: 0, jf: 12, k: 0x00000800 },
        sock_filter { code: 0x30, jt: 0, jf: 0, k: 0x00000017 }, sock_filter { code: 0x15, jt: 0, jf: 9, k: 0x00000011 },
        sock_filter { code: 0x28, jt: 0, jf: 0, k: 0x00000014 }, sock_filter { code: 0x45, jt: 7, jf: 0, k: 0x00001fff },
        sock_filter { code: 0xb1, jt: 0, jf: 0, k: 0x0000000e }, sock_filter { code: 0x48, jt: 0, jf: 0, k: 0x00000010 },
        sock_filter { code: 0x15, jt: 0, jf: 4, k: 0x0000013f }, sock_filter { code: 0x48, jt: 0, jf: 0, k: 0x00000016 },
        sock_filter { code: 0x54, jt: 0, jf: 0, k: 0x0000000f }, sock_filter { code: 0x44, jt: 0, jf: 0, k: 0x00000010 },
        sock_filter { code: 0x16, jt: 0, jf: 0, k: 0 }, sock_filter { code: 0x06, jt: 0, jf: 0, k: 0 },
        sock_filter { code: 0x15, jt: 0, jf: 9, k: 0x000086dd }, sock_filter { code: 0x30, jt: 0, jf: 0, k: 0x00000014 },
        sock_filter { code: 0x15, jt: 0, jf: 6, k: 0x00000011 }, sock_filter { code: 0x28, jt: 0, jf: 0, k: 0x00000038 },
        sock_filter { code: 0x15, jt: 0, jf: 4, k: 0x0000013f }, sock_filter { code: 0x28, jt: 0, jf: 0, k: 0x0000003e },
        sock_filter { code: 0x54, jt: 0, jf: 0, k: 0x0000000f }, sock_filter { code: 0x44, jt: 0, jf: 0, k: 0x00000020 },
        sock_filter { code: 0x16, jt: 0, jf: 0, k: 0 }, sock_filter { code: 0x06, jt: 0, jf: 0, k: 0 },
        sock_filter { code: 0x15, jt: 0, jf: 32, k: 0x00008100 }, sock_filter { code: 0x28, jt: 0, jf: 0, k: 0x00000010 },
        sock_filter { code: 0x15, jt: 0, jf: 7, k: 0x000088f7 }, sock_filter { code: 0x30, jt: 0, jf: 0, k: 0x00000012 },
        sock_filter { code: 0x54, jt: 0, jf: 0, k: 8 }, sock_filter { code: 0x15, jt: 0, jf: 35, k: 0 },
        sock_filter { code: 0x28, jt: 0, jf: 0, k: 0x12 }, sock_filter { code: 0x54, jt: 0, jf: 0, k: 0xf },
        sock_filter { code: 0x44, jt: 0, jf: 0, k: 0xc0 }, sock_filter { code: 0x16, jt: 0, jf: 0, k: 0 },
        sock_filter { code: 0x15, jt: 0, jf: 12, k: 0x800 }, sock_filter { code: 0x30, jt: 0, jf: 0, k: 0x1b },
        sock_filter { code: 0x15, jt: 0, jf: 9, k: 0x11 }, sock_filter { code: 0x28, jt: 0, jf: 0, k: 0x18 },
        sock_filter { code: 0x45, jt: 7, jf: 0, k: 0x1fff }, sock_filter { code: 0xb1, jt: 0, jf: 0, k: 0x12 },
        sock_filter { code: 0x48, jt: 0, jf: 0, k: 0x14 }, sock_filter { code: 0x15, jt: 0, jf: 4, k: 0x13f },
        sock_filter { code: 0x48, jt: 0, jf: 0, k: 0x1a }, sock_filter { code: 0x54, jt: 0, jf: 0, k: 0xf },
        sock_filter { code: 0x44, jt: 0, jf: 0, k: 0x90 }, sock_filter { code: 0x16, jt: 0, jf: 0, k: 0 },
        sock_filter { code: 0x06, jt: 0, jf: 0, k: 0 }, sock_filter { code: 0x15, jt: 0, jf: 8, k: 0x86dd },
        sock_filter { code: 0x30, jt: 0, jf: 0, k: 0x18 }, sock_filter { code: 0x15, jt: 0, jf: 6, k: 0x11 },
        sock_filter { code: 0x28, jt: 0, jf: 0, k: 0x3c }, sock_filter { code: 0x15, jt: 0, jf: 4, k: 0x13f },
        sock_filter { code: 0x28, jt: 0, jf: 0, k: 0x42 }, sock_filter { code: 0x54, jt: 0, jf: 0, k: 0xf },
        sock_filter { code: 0x44, jt: 0, jf: 0, k: 0xa0 }, sock_filter { code: 0x16, jt: 0, jf: 0, k: 0 },
        sock_filter { code: 0x06, jt: 0, jf: 0, k: 0 }, sock_filter { code: 0x15, jt: 0, jf: 7, k: 0x88f7 },
        sock_filter { code: 0x30, jt: 0, jf: 0, k: 0xe }, sock_filter { code: 0x54, jt: 0, jf: 0, k: 8 },
        sock_filter { code: 0x15, jt: 0, jf: 4, k: 0 }, sock_filter { code: 0x28, jt: 0, jf: 0, k: 0xe },
        sock_filter { code: 0x54, jt: 0, jf: 0, k: 0xf }, sock_filter { code: 0x44, jt: 0, jf: 0, k: 0x40 },
        sock_filter { code: 0x16, jt: 0, jf: 0, k: 0 }, sock_filter { code: 0x06, jt: 0, jf: 0, k: 0 },
    ];
    let mut ptp_prog = sock_fprog_kern {
        len: ptp_filter.len() as u32,
        filter: ptp_filter.as_mut_ptr(),
    };
    BUG_ON(bpf_prog_create(&mut ptp_insns, &mut ptp_prog));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
