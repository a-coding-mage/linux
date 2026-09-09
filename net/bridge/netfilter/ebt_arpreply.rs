// SPDX-License-Identifier: GPL-2.0-only
/*
 *  ebt_arpreply
 *
 *	Authors:
 *		Grzegorz Borowiak <grzes@gnu.univ.gda.pl>
 *		Bart De Schuymer <bdschuym@pandora.be>
 *
 *  August, 2003
 *
 */

// C dependencies supplied by the surrounding kernel/netfilter bindings.

unsafe extern "C" {
    fn skb_header_pointer(
        skb: *mut sk_buff,
        offset: usize,
        len: usize,
        buffer: *mut core::ffi::c_void,
    ) -> *const core::ffi::c_void;
    fn arp_send(
        op: u16,
        ptype: u16,
        dest: __be32,
        dev: *mut net_device,
        src: __be32,
        tha: *const u8,
        sha: *const u8,
        target: *const u8,
    );
    fn xt_in(par: *const xt_action_param) -> *mut net_device;
    fn ebt_invalid_target(target: i32) -> bool;
    fn htons(value: u16) -> u16;
    fn xt_register_target(target: *mut xt_target) -> i32;
    fn xt_unregister_target(target: *mut xt_target);
}

extern "C" {
    static THIS_MODULE: *mut module;
}

#[repr(C)]
struct sk_buff;
#[repr(C)]
struct net_device;
#[repr(C)]
struct module;
#[repr(C)]
struct xt_action_param {
    targinfo: *const core::ffi::c_void,
}
#[repr(C)]
struct xt_tgchk_param {
    targinfo: *const core::ffi::c_void,
    entryinfo: *const ebt_entry,
}
#[repr(C)]
struct ebt_entry {
    ethproto: u16,
    invflags: u32,
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
struct ebt_arpreply_info {
    mac: [u8; ETH_ALEN],
    target: u32,
}
#[repr(C)]
struct xt_target {
    name: *const u8,
    revision: u8,
    family: u16,
    table: *const u8,
    hooks: u32,
    target: Option<unsafe extern "C" fn(*mut sk_buff, *const xt_action_param) -> u32>,
    checkentry: Option<unsafe extern "C" fn(*const xt_tgchk_param) -> i32>,
    targetsize: usize,
    me: *mut module,
}

type __be32 = u32;

const ETH_ALEN: usize = 6;
const EBT_DROP: u32 = 0;
const EBT_CONTINUE: u32 = 0xFFFFFFFF;
const EBT_RETURN: u32 = 0xFFFFFFFE;
const EBT_IPROTO: u32 = 0x01;
const ARPOP_REQUEST: u16 = 1;
const ARPOP_REPLY: u16 = 2;
const ETH_P_IP: u16 = 0x0800;
const ETH_P_ARP: u16 = 0x0806;
const NFPROTO_BRIDGE: u16 = 7;
const NF_BR_NUMHOOKS: u32 = 3;
const NF_BR_PRE_ROUTING: u32 = 0;

unsafe extern "C" fn ebt_arpreply_tg(
    skb: *mut sk_buff,
    par: *const xt_action_param,
) -> u32 {
    let info = (*par).targinfo as *const ebt_arpreply_info;
    let mut _sip: __be32 = 0;
    let mut _dip: __be32 = 0;
    let mut _ah: arphdr = core::mem::zeroed();
    let mut _sha = [0u8; ETH_ALEN];

    let ap = skb_header_pointer(
        skb, 0, core::mem::size_of::<arphdr>(),
        &mut _ah as *mut _ as *mut core::ffi::c_void,
    ) as *const arphdr;
    if ap.is_null() {
        return EBT_DROP;
    }

    if (*ap).ar_op != htons(ARPOP_REQUEST)
        || (*ap).ar_hln != ETH_ALEN as u8
        || (*ap).ar_pro != htons(ETH_P_IP)
        || (*ap).ar_pln != 4
    {
        return EBT_CONTINUE;
    }

    let shp = skb_header_pointer(
        skb, core::mem::size_of::<arphdr>(), ETH_ALEN,
        _sha.as_mut_ptr() as *mut core::ffi::c_void,
    ) as *const u8;
    if shp.is_null() {
        return EBT_DROP;
    }
    let siptr = skb_header_pointer(
        skb, core::mem::size_of::<arphdr>() + ETH_ALEN,
        core::mem::size_of::<__be32>(), &mut _sip as *mut _ as *mut core::ffi::c_void,
    ) as *const __be32;
    if siptr.is_null() {
        return EBT_DROP;
    }
    let diptr = skb_header_pointer(
        skb, core::mem::size_of::<arphdr>() + 2 * ETH_ALEN + core::mem::size_of::<__be32>(),
        core::mem::size_of::<__be32>(), &mut _dip as *mut _ as *mut core::ffi::c_void,
    ) as *const __be32;
    if diptr.is_null() {
        return EBT_DROP;
    }

    arp_send(ARPOP_REPLY, ETH_P_ARP, *siptr, xt_in(par), *diptr, shp, (*info).mac.as_ptr(), shp);
    (*info).target
}

unsafe extern "C" fn ebt_arpreply_tg_check(par: *const xt_tgchk_param) -> i32 {
    let info = (*par).targinfo as *const ebt_arpreply_info;
    let e = (*par).entryinfo;
    if BASE_CHAIN && (*info).target == EBT_RETURN { return -22; }
    if (*e).ethproto != htons(ETH_P_ARP) || (*e).invflags & EBT_IPROTO != 0 { return -22; }
    if ebt_invalid_target((*info).target as i32) { return -22; }
    0
}

// BASE_CHAIN is a build-time kernel configuration condition.
const BASE_CHAIN: bool = false;

static mut ebt_arpreply_tg_reg: xt_target = xt_target {
    name: b"arpreply\0".as_ptr(), revision: 0, family: NFPROTO_BRIDGE,
    table: b"nat\0".as_ptr(), hooks: (1u32 << NF_BR_NUMHOOKS) | (1u32 << NF_BR_PRE_ROUTING),
    target: Some(ebt_arpreply_tg), checkentry: Some(ebt_arpreply_tg_check),
    targetsize: core::mem::size_of::<ebt_arpreply_info>(), me: core::ptr::null_mut(),
};

unsafe extern "C" fn ebt_arpreply_init() -> i32 {
    ebt_arpreply_tg_reg.me = THIS_MODULE;
    xt_register_target(&mut ebt_arpreply_tg_reg)
}

unsafe extern "C" fn ebt_arpreply_fini() {
    xt_unregister_target(&mut ebt_arpreply_tg_reg);
}

// module_init(ebt_arpreply_init);
// module_exit(ebt_arpreply_fini);
// MODULE_DESCRIPTION("Ebtables: ARP reply target");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
