// SPDX-License-Identifier: GPL-2.0-only
/* ipv6header match - matches IPv6 packets based
   on whether they contain certain headers */

/* Original idea: Brad Chapman
 * Rewritten by: Andras Kis-Szabo <kisza@sch.bme.hu> */

/* (C) 2001-2002 Andras Kis-Szabo <kisza@sch.bme.hu>
 */

// Kernel dependencies supplied by the surrounding translation unit/build.

extern "C" {
    fn ipv6_hdr(skb: *const sk_buff) -> *const ipv6hdr;
    fn nf_ip6_ext_hdr(nexthdr: u8) -> bool;
    fn skb_header_pointer(
        skb: *const sk_buff,
        offset: u32,
        len: u32,
        buffer: *mut ipv6_opt_hdr,
    ) -> *const ipv6_opt_hdr;
    fn ipv6_authlen(hp: *const ipv6_opt_hdr) -> i32;
    fn ipv6_optlen(hp: *const ipv6_opt_hdr) -> i32;
    fn xt_register_match(reg: *mut xt_match) -> i32;
    fn xt_unregister_match(reg: *mut xt_match);
}

#[repr(C)]
pub struct sk_buff {
    pub len: u32,
}

#[repr(C)]
pub struct xt_action_param {
    pub matchinfo: *const ip6t_ipv6header_info,
    pub hotdrop: bool,
}

#[repr(C)]
pub struct xt_mtchk_param {
    pub matchinfo: *const ip6t_ipv6header_info,
}

#[repr(C)]
pub struct ipv6hdr {
    pub nexthdr: u8,
}

#[repr(C)]
pub struct ipv6_opt_hdr {
    pub nexthdr: u8,
    pub hdrlen: u8,
}

#[repr(C)]
pub struct ip6t_ipv6header_info {
    pub matchflags: u32,
    pub invflags: u32,
    pub modeflag: bool,
}

#[repr(C)]
pub struct xt_match {
    pub name: *const u8,
    pub family: u16,
    pub match_: Option<unsafe extern "C" fn(*const sk_buff, *mut xt_action_param) -> bool>,
    pub matchsize: usize,
    pub checkentry: Option<unsafe extern "C" fn(*const xt_mtchk_param) -> i32>,
    pub destroy: Option<unsafe extern "C" fn()>,
    pub me: *mut core::ffi::c_void,
}

const NEXTHDR_NONE: u8 = 59;
const NEXTHDR_ESP: u8 = 50;
const NEXTHDR_FRAGMENT: u8 = 44;
const NEXTHDR_AUTH: u8 = 51;
const NEXTHDR_HOP: u8 = 0;
const NEXTHDR_ROUTING: u8 = 43;
const NEXTHDR_DEST: u8 = 60;
const MASK_NONE: u32 = 1 << 0;
const MASK_ESP: u32 = 1 << 1;
const MASK_HOPOPTS: u32 = 1 << 2;
const MASK_ROUTING: u32 = 1 << 3;
const MASK_FRAGMENT: u32 = 1 << 4;
const MASK_AH: u32 = 1 << 5;
const MASK_DSTOPTS: u32 = 1 << 6;
const MASK_PROTO: u32 = 1 << 7;
const NFPROTO_IPV6: u16 = 10;

unsafe extern "C" fn ipv6header_mt6(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let info = (*par).matchinfo;
    let mut temp: u32;
    let mut len: i32;
    let mut nexthdr: u8;
    let mut ptr: u32;

    /* Make sure this isn't an evil packet */

    /* type of the 1st exthdr */
    nexthdr = (*ipv6_hdr(skb)).nexthdr;
    /* pointer to the 1st exthdr */
    ptr = core::mem::size_of::<ipv6hdr>() as u32;
    /* available length */
    len = (*skb).len as i32 - ptr as i32;
    temp = 0;

    while nf_ip6_ext_hdr(nexthdr) {
        let mut _hdr = ipv6_opt_hdr { nexthdr: 0, hdrlen: 0 };
        let hp: *const ipv6_opt_hdr;
        let hdrlen: i32;

        /* No more exthdr -> evaluate */
        if nexthdr == NEXTHDR_NONE {
            temp |= MASK_NONE;
            break;
        }
        /* Is there enough space for the next ext header? */
        if len < core::mem::size_of::<ipv6_opt_hdr>() as i32 {
            return false;
        }
        /* ESP -> evaluate */
        if nexthdr == NEXTHDR_ESP {
            temp |= MASK_ESP;
            break;
        }

        hp = skb_header_pointer(
            skb,
            ptr,
            core::mem::size_of::<ipv6_opt_hdr>() as u32,
            &mut _hdr,
        );
        if hp.is_null() {
            (*par).hotdrop = true;
            return false;
        }

        /* Calculate the header length */
        if nexthdr == NEXTHDR_FRAGMENT {
            hdrlen = 8;
        } else if nexthdr == NEXTHDR_AUTH {
            hdrlen = ipv6_authlen(hp);
        } else {
            hdrlen = ipv6_optlen(hp);
        }

        /* set the flag */
        match nexthdr {
            NEXTHDR_HOP => temp |= MASK_HOPOPTS,
            NEXTHDR_ROUTING => temp |= MASK_ROUTING,
            NEXTHDR_FRAGMENT => temp |= MASK_FRAGMENT,
            NEXTHDR_AUTH => temp |= MASK_AH,
            NEXTHDR_DEST => temp |= MASK_DSTOPTS,
            _ => return false,
        }

        nexthdr = (*hp).nexthdr;
        len -= hdrlen;
        ptr = ptr.wrapping_add(hdrlen as u32);
        if ptr > (*skb).len {
            break;
        }
    }

    if nexthdr != NEXTHDR_NONE && nexthdr != NEXTHDR_ESP {
        temp |= MASK_PROTO;
    }

    if (*info).modeflag {
        !((temp ^ (*info).matchflags ^ (*info).invflags) & (*info).matchflags != 0)
    } else if (*info).invflags != 0 {
        temp != (*info).matchflags
    } else {
        temp == (*info).matchflags
    }
}

unsafe extern "C" fn ipv6header_mt6_check(par: *const xt_mtchk_param) -> i32 {
    let info = (*par).matchinfo;

    /* invflags is 0 or 0xff in hard mode */
    if !(*info).modeflag && (*info).invflags != 0x00 && (*info).invflags != 0xFF {
        return -22;
    }

    0
}

static mut IPV6HEADER_MT6_REG: xt_match = xt_match {
    name: b"ipv6header\0".as_ptr(),
    family: NFPROTO_IPV6,
    match_: Some(ipv6header_mt6),
    matchsize: core::mem::size_of::<ip6t_ipv6header_info>(),
    checkentry: Some(ipv6header_mt6_check),
    destroy: None,
    me: core::ptr::null_mut(),
};

unsafe extern "C" fn ipv6header_mt6_init() -> i32 {
    xt_register_match(&raw mut IPV6HEADER_MT6_REG)
}

unsafe extern "C" fn ipv6header_mt6_exit() {
    xt_unregister_match(&raw mut IPV6HEADER_MT6_REG);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
