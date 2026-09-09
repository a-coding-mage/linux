// SPDX-License-Identifier: GPL-2.0-only
/* Kernel module to match AH parameters. */

/* (C) 2001-2002 Andras Kis-Szabo <kisza@sch.bme.hu>
 */

// Dependency and build-time configuration supplied by the kernel headers.

extern "C" {
    fn ipv6_find_hdr(
        skb: *const sk_buff,
        ptr: *mut u32,
        target: i32,
        fragoff: *mut core::ffi::c_void,
        flags: *mut core::ffi::c_void,
    ) -> i32;
    fn skb_header_pointer(
        skb: *const sk_buff,
        offset: u32,
        len: usize,
        buffer: *mut core::ffi::c_void,
    ) -> *const ip_auth_hdr;
    fn ipv6_authlen(ah: *const ip_auth_hdr) -> u32;
    fn xt_register_match(m: *mut xt_match) -> i32;
    fn xt_unregister_match(m: *mut xt_match);
    fn ntohl(value: u32) -> u32;
}

extern "C" {
    static THIS_MODULE: *mut module;
}

// Types and constants are provided by the corresponding kernel headers.
extern "C" {
    type sk_buff;
    type xt_action_param;
    type xt_mtchk_param;
    type ip_auth_hdr;
    type ip6t_ah;
    type xt_match;
    type module;
}

const NEXTHDR_AUTH: i32 = 51;
const NFPROTO_IPV6: u8 = 10;
const IP6T_AH_INV_SPI: u8 = 0x01;
const IP6T_AH_INV_LEN: u8 = 0x02;
const IP6T_AH_INV_MASK: u8 = IP6T_AH_INV_SPI | IP6T_AH_INV_LEN;
const ENOENT: i32 = 2;
const EINVAL: i32 = 22;

// Returns 1 if the spi is matched by the range, 0 otherwise
#[inline]
unsafe fn spi_match(min: u32, max: u32, spi: u32, invert: bool) -> bool {
    ((spi >= min && spi <= max) as u8 != 0) ^ invert
}

unsafe fn ah_mt6(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let mut _ah = core::mem::MaybeUninit::<ip_auth_hdr>::uninit();
    let ah: *const ip_auth_hdr;
    let ahinfo: *const ip6t_ah;
    let mut ptr: u32 = 0;
    let mut hdrlen: u32 = 0;
    let err: i32;

    ahinfo = (*(par)).matchinfo as *const ip6t_ah;
    err = ipv6_find_hdr(
        skb,
        &mut ptr,
        NEXTHDR_AUTH,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
    );
    if err < 0 {
        if err != -ENOENT {
            (*(par)).hotdrop = true;
        }
        return false;
    }

    ah = skb_header_pointer(
        skb,
        ptr,
        core::mem::size_of::<ip_auth_hdr>(),
        _ah.as_mut_ptr() as *mut core::ffi::c_void,
    );
    if ah.is_null() {
        (*(par)).hotdrop = true;
        return false;
    }

    hdrlen = ipv6_authlen(ah);
    if (*(skb)).len - ptr < hdrlen {
        /* Packet smaller than its length field */
        (*(par)).hotdrop = true;
        return false;
    }

    spi_match(
        (*(ahinfo)).spis[0],
        (*(ahinfo)).spis[1],
        ntohl((*(ah)).spi),
        ((*(ahinfo)).invflags & IP6T_AH_INV_SPI) != 0,
    ) &&
        ((*(ahinfo)).hdrlen == 0 ||
            ((*(ahinfo)).hdrlen == hdrlen) ^
                ((*(ahinfo)).invflags & IP6T_AH_INV_LEN) != 0) &&
        !((*(ahinfo)).hdrres != 0 && (*(ah)).reserved != 0)
}

unsafe fn ah_mt6_check(par: *const xt_mtchk_param) -> i32 {
    let ahinfo = (*(par)).matchinfo as *const ip6t_ah;

    if (*(ahinfo)).invflags & !IP6T_AH_INV_MASK != 0 {
        // pr_info_ratelimited("unknown flags %X\n", ahinfo->invflags);
        return -EINVAL;
    }
    0
}

#[no_mangle]
static mut ah_mt6_reg: xt_match = xt_match {
    name: b"ah\0".as_ptr() as *const i8,
    family: NFPROTO_IPV6,
    match_: Some(ah_mt6),
    matchsize: core::mem::size_of::<ip6t_ah>(),
    checkentry: Some(ah_mt6_check),
    me: unsafe { THIS_MODULE },
};

unsafe fn ah_mt6_init() -> i32 {
    xt_register_match(&mut ah_mt6_reg)
}

unsafe fn ah_mt6_exit() {
    xt_unregister_match(&mut ah_mt6_reg);
}

// module_init(ah_mt6_init);
// module_exit(ah_mt6_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
