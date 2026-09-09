// SPDX-License-Identifier: GPL-2.0-only
/*
 *  ebt_stp
 *
 *	Authors:
 *	Bart De Schuymer <bdschuym@pandora.be>
 *	Stephen Hemminger <shemminger@osdl.org>
 *
 *  July, 2003
 */

// Kernel dependencies supplied by the surrounding translation unit/build.

const BPDU_TYPE_CONFIG: u8 = 0;

#[repr(C)]
struct stp_header {
    dsap: u8,
    ssap: u8,
    ctrl: u8,
    pid: u8,
    vers: u8,
    type_: u8,
}

#[repr(C)]
struct stp_config_pdu {
    flags: u8,
    root: [u8; 8],
    root_cost: [u8; 4],
    sender: [u8; 8],
    port: [u8; 2],
    msg_age: [u8; 2],
    max_age: [u8; 2],
    hello_time: [u8; 2],
    forward_delay: [u8; 2],
}

#[inline]
unsafe fn nr16(p: *const u8) -> u16 {
    ((*p as u16) << 8) | *p.add(1) as u16
}

#[inline]
unsafe fn nr32(p: *const u8) -> u32 {
    ((*p as u32) << 24)
        | ((*p.add(1) as u32) << 16)
        | ((*p.add(2) as u32) << 8)
        | *p.add(3) as u32
}

unsafe fn ebt_filter_config(
    info: *const ebt_stp_info,
    stpc: *const stp_config_pdu,
) -> bool {
    let c = &(*info).config;
    let mut v16: u16;
    let mut v32: u32;

    if (*info).bitmask & EBT_STP_FLAGS != 0
        && NF_INVF!(info, EBT_STP_FLAGS, c.flags != (*stpc).flags)
    { return false; }
    if (*info).bitmask & EBT_STP_ROOTPRIO != 0 {
        v16 = nr16((*stpc).root.as_ptr());
        if NF_INVF!(info, EBT_STP_ROOTPRIO, v16 < c.root_priol || v16 > c.root_priou) { return false; }
    }
    if (*info).bitmask & EBT_STP_ROOTADDR != 0 {
        if NF_INVF!(info, EBT_STP_ROOTADDR, !ether_addr_equal_masked((*stpc).root.as_ptr().add(2), c.root_addr.as_ptr(), c.root_addrmsk.as_ptr())) { return false; }
    }
    if (*info).bitmask & EBT_STP_ROOTCOST != 0 {
        v32 = nr32((*stpc).root_cost.as_ptr());
        if NF_INVF!(info, EBT_STP_ROOTCOST, v32 < c.root_costl || v32 > c.root_costu) { return false; }
    }
    if (*info).bitmask & EBT_STP_SENDERPRIO != 0 {
        v16 = nr16((*stpc).sender.as_ptr());
        if NF_INVF!(info, EBT_STP_SENDERPRIO, v16 < c.sender_priol || v16 > c.sender_priou) { return false; }
    }
    if (*info).bitmask & EBT_STP_SENDERADDR != 0 {
        if NF_INVF!(info, EBT_STP_SENDERADDR, !ether_addr_equal_masked((*stpc).sender.as_ptr().add(2), c.sender_addr.as_ptr(), c.sender_addrmsk.as_ptr())) { return false; }
    }
    if (*info).bitmask & EBT_STP_PORT != 0 {
        v16 = nr16((*stpc).port.as_ptr());
        if NF_INVF!(info, EBT_STP_PORT, v16 < c.portl || v16 > c.portu) { return false; }
    }
    if (*info).bitmask & EBT_STP_MSGAGE != 0 {
        v16 = nr16((*stpc).msg_age.as_ptr());
        if NF_INVF!(info, EBT_STP_MSGAGE, v16 < c.msg_agel || v16 > c.msg_ageu) { return false; }
    }
    if (*info).bitmask & EBT_STP_MAXAGE != 0 {
        v16 = nr16((*stpc).max_age.as_ptr());
        if NF_INVF!(info, EBT_STP_MAXAGE, v16 < c.max_agel || v16 > c.max_ageu) { return false; }
    }
    if (*info).bitmask & EBT_STP_HELLOTIME != 0 {
        v16 = nr16((*stpc).hello_time.as_ptr());
        if NF_INVF!(info, EBT_STP_HELLOTIME, v16 < c.hello_timel || v16 > c.hello_timeu) { return false; }
    }
    if (*info).bitmask & EBT_STP_FWDD != 0 {
        v16 = nr16((*stpc).forward_delay.as_ptr());
        if NF_INVF!(info, EBT_STP_FWDD, v16 < c.forward_delayl || v16 > c.forward_delayu) { return false; }
    }
    true
}

unsafe fn ebt_stp_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let info = (*par).matchinfo as *const ebt_stp_info;
    let mut stph = core::mem::MaybeUninit::<stp_header>::uninit();
    let header = [0x42u8, 0x42, 0x03, 0x00, 0x00, 0x00];
    let sp = skb_header_pointer(skb, 0, core::mem::size_of::<stp_header>(), stph.as_mut_ptr());
    if sp.is_null() { return false; }
    if memcmp(sp as *const _, header.as_ptr() as *const _, header.len()) != 0 { return false; }
    if (*info).bitmask & EBT_STP_TYPE != 0
        && NF_INVF!(info, EBT_STP_TYPE, (*info).type_ != (*sp).type_)
    { return false; }
    if (*sp).type_ == BPDU_TYPE_CONFIG && (*info).bitmask & EBT_STP_CONFIG_MASK != 0 {
        let mut stpc = core::mem::MaybeUninit::<stp_config_pdu>::uninit();
        let st = skb_header_pointer(skb, core::mem::size_of::<stp_header>(), core::mem::size_of::<stp_config_pdu>(), stpc.as_mut_ptr());
        if st.is_null() { return false; }
        return ebt_filter_config(info, st as *const stp_config_pdu);
    }
    true
}

unsafe fn ebt_stp_mt_check(par: *const xt_mtchk_param) -> i32 {
    let info = (*par).matchinfo as *const ebt_stp_info;
    let e = (*par).entryinfo as *const ebt_entry;
    if (*info).bitmask & !EBT_STP_MASK != 0 || (*info).invflags & !EBT_STP_MASK != 0 || (*info).bitmask & EBT_STP_MASK == 0 { return -EINVAL; }
    if !(*par).nft_compat && (!ether_addr_equal((*e).destmac.as_ptr(), eth_stp_addr.as_ptr()) || (*e).bitmask & EBT_DESTMAC == 0 || !is_broadcast_ether_addr((*e).destmsk.as_ptr())) { return -EINVAL; }
    0
}

static mut ebt_stp_mt_reg: xt_match = xt_match {
    name: "stp",
    revision: 0,
    family: NFPROTO_BRIDGE,
    match_: Some(ebt_stp_mt),
    checkentry: Some(ebt_stp_mt_check),
    matchsize: core::mem::size_of::<ebt_stp_info>(),
    me: THIS_MODULE,
};

unsafe fn ebt_stp_init() -> i32 { xt_register_match(&mut ebt_stp_mt_reg) }
unsafe fn ebt_stp_fini() { xt_unregister_match(&mut ebt_stp_mt_reg); }

// module_init(ebt_stp_init); module_exit(ebt_stp_fini);
// MODULE_DESCRIPTION("Ebtables: Spanning Tree Protocol packet match");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
