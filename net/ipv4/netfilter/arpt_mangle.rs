// SPDX-License-Identifier: GPL-2.0-only
/* module that allows mangling of the arp payload */
// External Linux kernel declarations and configuration supplied by other files.

#[allow(non_camel_case_types)]
type c_int = i32;

extern "C" {
    fn skb_ensure_writable(skb: *mut sk_buff, len: usize) -> c_int;
    fn arp_hdr(skb: *mut sk_buff) -> *const arphdr;
    fn skb_network_header(skb: *mut sk_buff) -> *mut u8;
    fn skb_tail_pointer(skb: *mut sk_buff) -> *mut u8;
    fn xt_register_target(target: *mut xt_target) -> c_int;
    fn xt_unregister_target(target: *mut xt_target);
}

#[repr(C)]
struct sk_buff {
    len: usize,
    dev: *mut net_device,
}

#[repr(C)]
struct net_device {
    type_: u16,
}

#[repr(C)]
struct arphdr {
    ar_hrd: u16,
    ar_pro: u16,
    ar_hln: u8,
    ar_pln: u8,
    ar_op: u16,
}

#[repr(C)]
union arpt_mangle_ip {
    src_ip: u32,
    tgt_ip: u32,
}

#[repr(C)]
struct arpt_mangle {
    flags: u32,
    src_devaddr: [u8; ARPT_DEV_ADDR_LEN_MAX as usize],
    tgt_devaddr: [u8; ARPT_DEV_ADDR_LEN_MAX as usize],
    u_s: arpt_mangle_ip,
    u_t: arpt_mangle_ip,
    target: u32,
}

#[repr(C)]
struct xt_action_param {
    targinfo: *const core::ffi::c_void,
}

#[repr(C)]
struct xt_tgchk_param {
    targinfo: *const core::ffi::c_void,
}

#[repr(C)]
struct xt_target {
    name: *const u8,
    family: u16,
    target: Option<unsafe extern "C" fn(*mut sk_buff, *const xt_action_param) -> u32>,
    targetsize: usize,
    checkentry: Option<unsafe extern "C" fn(*const xt_tgchk_param) -> c_int>,
    me: *mut core::ffi::c_void,
}

const NF_DROP: u32 = 0;
const NF_ACCEPT: u32 = 1;
const XT_CONTINUE: u32 = 0xFFFFFFFF;
const NFPROTO_ARP: u16 = 0x0003;
const ARPHRD_IEEE1394: u16 = 24;
const ARPT_MANGLE_SDEV: u32 = 1;
const ARPT_MANGLE_SIP: u32 = 2;
const ARPT_MANGLE_TDEV: u32 = 4;
const ARPT_MANGLE_TIP: u32 = 8;
const ARPT_MANGLE_MASK: u32 = ARPT_MANGLE_SDEV | ARPT_MANGLE_SIP | ARPT_MANGLE_TDEV | ARPT_MANGLE_TIP;
const ARPT_DEV_ADDR_LEN_MAX: u32 = 16;
const ARPT_MANGLE_ADDR_LEN_MAX: u32 = 16;
const ARP_PROTOCOL: u16 = 0x0003;

static mut arpt_mangle_reg: xt_target = xt_target {
    name: b"mangle\0".as_ptr(),
    family: NFPROTO_ARP,
    target: Some(target),
    targetsize: core::mem::size_of::<arpt_mangle>(),
    checkentry: Some(checkentry),
    me: core::ptr::null_mut(),
};

unsafe extern "C" fn target(skb: *mut sk_buff, par: *const xt_action_param) -> u32 {
    let mangle = (*(par)).targinfo as *const arpt_mangle;
    let arp: *const arphdr;
    let mut arpptr: *mut u8;
    let pln: usize;
    let hln: usize;

    if skb_ensure_writable(skb, (*skb).len) != 0 {
        return NF_DROP;
    }

    arp = arp_hdr(skb);
    arpptr = skb_network_header(skb).add(core::mem::size_of::<arphdr>());
    pln = (*arp).ar_pln as usize;
    hln = (*arp).ar_hln as usize;
    /* We assume that pln and hln were checked in the match */
    if (*mangle).flags & ARPT_MANGLE_SDEV != 0 {
        if ARPT_DEV_ADDR_LEN_MAX as usize < hln || arpptr.add(hln) > skb_tail_pointer(skb) {
            return NF_DROP;
        }
        core::ptr::copy_nonoverlapping((*mangle).src_devaddr.as_ptr(), arpptr, hln);
    }
    arpptr = arpptr.add(hln);
    if (*mangle).flags & ARPT_MANGLE_SIP != 0 {
        if ARPT_MANGLE_ADDR_LEN_MAX as usize < pln || arpptr.add(pln) > skb_tail_pointer(skb) {
            return NF_DROP;
        }
        core::ptr::copy_nonoverlapping(&(*mangle).u_s.src_ip as *const u32 as *const u8, arpptr, pln);
    }
    arpptr = arpptr.add(pln);
    if (*mangle).flags & ARPT_MANGLE_TDEV != 0 {
        if cfg!(CONFIG_FIREWIRE_NET) && (*(*skb).dev).type_ == ARPHRD_IEEE1394 {
            return NF_DROP;
        }
        if arpptr.add(hln) > skb_tail_pointer(skb) {
            return NF_DROP;
        }
        core::ptr::copy_nonoverlapping((*mangle).tgt_devaddr.as_ptr(), arpptr, hln);
    }
    arpptr = arpptr.add(hln);
    if (*mangle).flags & ARPT_MANGLE_TIP != 0 {
        if cfg!(CONFIG_FIREWIRE_NET) && (*(*skb).dev).type_ == ARPHRD_IEEE1394 {
            return NF_DROP;
        }
        if arpptr.add(pln) > skb_tail_pointer(skb) {
            return NF_DROP;
        }
        core::ptr::copy_nonoverlapping(&(*mangle).u_t.tgt_ip as *const u32 as *const u8, arpptr, pln);
    }
    (*mangle).target
}

unsafe extern "C" fn checkentry(par: *const xt_tgchk_param) -> c_int {
    let mangle = (*(par)).targinfo as *const arpt_mangle;
    if (*mangle).flags & !ARPT_MANGLE_MASK != 0 || (*mangle).flags & ARPT_MANGLE_MASK == 0 {
        return -22;
    }
    if (*mangle).target != NF_DROP && (*mangle).target != NF_ACCEPT && (*mangle).target != XT_CONTINUE {
        return -22;
    }
    0
}

unsafe extern "C" fn arpt_mangle_init() -> c_int {
    xt_register_target(&raw mut arpt_mangle_reg)
}

unsafe extern "C" fn arpt_mangle_fini() {
    xt_unregister_target(&raw mut arpt_mangle_reg);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
