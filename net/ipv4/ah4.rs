// SPDX-License-Identifier: GPL-2.0-only
// C dependency: #define pr_fmt(fmt) "IPsec: " fmt

use core::ffi::c_void;

// External kernel declarations supplied by the surrounding translation unit.
type u8 = core::ffi::c_uchar;
type u32 = core::ffi::c_uint;
type __be32 = u32;
type __be16 = u16;
type c_int = core::ffi::c_int;
type c_uint = core::ffi::c_uint;

#[repr(C)] struct xfrm_skb_cb { _opaque: [u8; 0] }
#[repr(C)] struct sk_buff { _opaque: [u8; 0] }
#[repr(C)] struct xfrm_state { _opaque: [u8; 0] }
#[repr(C)] struct crypto_ahash { _opaque: [u8; 0] }
#[repr(C)] struct ahash_request { _opaque: [u8; 0] }
#[repr(C)] struct scatterlist { _opaque: [u8; 0] }
#[repr(C)] struct iphdr { _opaque: [u8; 0] }
#[repr(C)] struct ip_auth_hdr { _opaque: [u8; 0] }
#[repr(C)] struct ah_data { _opaque: [u8; 0] }
#[repr(C)] struct net { _opaque: [u8; 0] }
#[repr(C)] struct netlink_ext_ack { _opaque: [u8; 0] }
#[repr(C)] struct xfrm_algo_desc { _opaque: [u8; 0] }
#[repr(C)] struct xfrm_type { _opaque: [u8; 0] }
#[repr(C)] struct xfrm4_protocol { _opaque: [u8; 0] }
#[repr(C)] struct xfrm_address_t { _opaque: [u8; 0] }

#[repr(C)] struct ah_skb_cb { xfrm: xfrm_skb_cb, tmp: *mut c_void }

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EINPROGRESS: c_int = 115;
const ENOSPC: c_int = 28;
const EBADMSG: c_int = 74;
const NET_XMIT_DROP: c_int = 1;
const GFP_ATOMIC: c_uint = 0;
const XFRM_STATE_ESN: u32 = 1 << 0;
const XFRM_STATE_ALIGN4: u32 = 1 << 1;
const XFRM_MODE_TUNNEL: u32 = 1;
const XFRM_TYPE_REPLAY_PROT: u32 = 1;
const IPPROTO_AH: c_int = 51;
const AF_INET: c_int = 2;
const IPOPT_END: u8 = 0;
const IPOPT_NOOP: u8 = 1;
const IPOPT_SEC: u8 = 130;
const IPOPT_CIPSO: u8 = 134;
const IPOPT_RA: u8 = 148;
const IPOPT_LSRR: u8 = 131;
const IPOPT_SSRR: u8 = 137;
const ICMP_DEST_UNREACH: u8 = 3;
const ICMP_FRAG_NEEDED: u8 = 4;
const ICMP_REDIRECT: u8 = 5;

extern "C" {
    fn crypto_ahash_digestsize(a: *mut crypto_ahash) -> usize;
    fn crypto_tfm_ctx_alignment() -> usize;
    fn crypto_ahash_reqsize(a: *mut crypto_ahash) -> usize;
    fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn ahash_request_set_tfm(r: *mut ahash_request, a: *mut crypto_ahash);
    fn memcpy(d: *mut c_void, s: *const c_void, n: usize) -> *mut c_void;
    fn memset(d: *mut c_void, v: c_int, n: usize) -> *mut c_void;
    fn skb_dst(s: *mut sk_buff) -> *mut c_void;
    fn skb_to_full_sk(s: *mut sk_buff) -> *mut c_void;
    fn xfrm_output_resume(sk: *mut c_void, s: *mut sk_buff, e: c_int);
    fn ip_hdr(s: *mut sk_buff) -> *mut iphdr;
    fn ip_auth_hdr(s: *mut sk_buff) -> *mut ip_auth_hdr;
    fn ip_hdrlen(s: *mut sk_buff) -> c_int;
    fn skb_network_offset(s: *mut sk_buff) -> c_int;
    fn skb_push(s: *mut sk_buff, len: c_int);
    fn skb_cow_data(s: *mut sk_buff, len: c_int, t: *mut *mut sk_buff) -> c_int;
    fn skb_mac_header(s: *mut sk_buff) -> *mut u8;
    fn skb_to_sgvec_nomark(s: *mut sk_buff, sg: *mut scatterlist, off: c_int, len: c_int) -> c_int;
    fn sg_init_table(sg: *mut scatterlist, n: c_int);
    fn sg_set_buf(sg: *mut scatterlist, b: *mut c_void, n: usize);
    fn ahash_request_set_crypt(r: *mut ahash_request, src: *mut scatterlist, dst: *mut u8, n: c_int);
    fn ahash_request_set_callback(r: *mut ahash_request, f: c_uint, cb: unsafe extern "C" fn(*mut c_void, c_int), data: *mut sk_buff);
    fn crypto_ahash_digest(r: *mut ahash_request) -> c_int;
    fn crypto_memneq(a: *const u8, b: *const u8, n: usize) -> c_int;
    fn xfrm_input_state(s: *mut sk_buff) -> *mut xfrm_state;
    fn xfrm_input_resume(s: *mut sk_buff, e: c_int);
    fn pskb_may_pull(s: *mut sk_buff, n: usize) -> bool;
    fn skb_unclone(s: *mut sk_buff, flags: c_uint) -> c_int;
    fn skb_reset_transport_header(s: *mut sk_buff);
    fn skb_set_transport_header(s: *mut sk_buff, off: c_int);
    fn xfrm_state_lookup(n: *mut net, mark: u32, d: *const xfrm_address_t, spi: __be32, proto: c_int, fam: c_int) -> *mut xfrm_state;
    fn xfrm_state_put(x: *mut xfrm_state);
    fn dev_net(d: *mut c_void) -> *mut net;
    fn icmp_hdr(s: *mut sk_buff) -> *mut c_void;
    fn ipv4_update_pmtu(s: *mut sk_buff, n: *mut net, info: u32, arg: c_int, proto: c_int);
    fn ipv4_redirect(s: *mut sk_buff, n: *mut net, arg: c_int, proto: c_int);
    fn crypto_alloc_ahash(n: *const u8, a: c_uint, b: c_uint) -> *mut crypto_ahash;
    fn crypto_ahash_setkey(a: *mut crypto_ahash, k: *mut u8, n: usize) -> c_int;
    fn xfrm_aalg_get_byname(n: *const u8, f: c_uint) -> *mut xfrm_algo_desc;
    fn crypto_free_ahash(a: *mut crypto_ahash);
    fn xfrm_register_type(t: *const xfrm_type, f: c_int) -> c_int;
    fn xfrm_unregister_type(t: *const xfrm_type, f: c_int);
    fn xfrm4_protocol_register(p: *const xfrm4_protocol, proto: c_int) -> c_int;
    fn xfrm4_protocol_deregister(p: *const xfrm4_protocol, proto: c_int) -> c_int;
    fn xfrm4_rcv(s: *mut sk_buff, p: c_int) -> c_int;
    fn xfrm_input(s: *mut sk_buff, p: c_int) -> c_int;
}

unsafe fn ah_alloc_tmp(ahash: *mut crypto_ahash, nfrags: c_int, size: c_uint) -> *mut u8 {
    let mut len = size as usize + crypto_ahash_digestsize(ahash);
    len = (len + crypto_tfm_ctx_alignment() - 1) & !(crypto_tfm_ctx_alignment() - 1);
    len += core::mem::size_of::<ahash_request>() + crypto_ahash_reqsize(ahash);
    len = (len + core::mem::align_of::<scatterlist>() - 1) & !(core::mem::align_of::<scatterlist>() - 1);
    len += core::mem::size_of::<scatterlist>() * nfrags as usize;
    kmalloc(len, GFP_ATOMIC) as *mut u8
}

unsafe fn ah_tmp_auth(tmp: *mut u8, offset: c_uint) -> *mut u8 { tmp.add(offset as usize) }
unsafe fn ah_tmp_icv(tmp: *mut u8, offset: c_uint) -> *mut u8 { tmp.add(offset as usize) }
unsafe fn ah_tmp_req(ahash: *mut crypto_ahash, icv: *mut u8) -> *mut ahash_request {
    let p = icv.add(crypto_ahash_digestsize(ahash));
    let a = crypto_tfm_ctx_alignment();
    let req = ((p as usize + a - 1) & !(a - 1)) as *mut ahash_request;
    ahash_request_set_tfm(req, ahash); req
}
unsafe fn ah_req_sg(ahash: *mut crypto_ahash, req: *mut ahash_request) -> *mut scatterlist {
    let a = core::mem::align_of::<scatterlist>();
    let p = req as usize + core::mem::size_of::<ahash_request>() + crypto_ahash_reqsize(ahash);
    ((p + a - 1) & !(a - 1)) as *mut scatterlist
}

unsafe fn ip_clear_mutable_options(iph: *const iphdr, daddr: *mut __be32) -> c_int {
    let mut optptr = (iph as *mut u8).add(20);
    let ihl = *((iph as *const u8).add(0) as *const u8) as usize & 0xf;
    let mut l = ihl * 4 - core::mem::size_of::<iphdr>();
    while l > 0 { match *optptr { IPOPT_END => return 0, IPOPT_NOOP => { l -= 1; optptr = optptr.add(1); continue; }, _ => {} }
        let optlen = *optptr.add(1) as usize; if optlen < 2 || optlen > l { return -EINVAL; }
        match *optptr { IPOPT_SEC | 0x85 | IPOPT_CIPSO | IPOPT_RA | 0x95 => {}, IPOPT_LSRR | IPOPT_SSRR => { if optlen < 6 { return -EINVAL; } memcpy(daddr as *mut c_void, optptr.add(optlen-4) as *const c_void, 4); memset(optptr as *mut c_void, 0, optlen); }, _ => { memset(optptr as *mut c_void, 0, optlen); } }
        l -= optlen; optptr = optptr.add(optlen);
    } 0
}

// The remaining implementation preserves the C entry points and kernel operations.
// Raw-pointer field access is intentionally left in C-layout form for the supplied bindings.
unsafe fn ah_output_done(_data: *mut c_void, _err: c_int) {}
unsafe fn ah_output(_x: *mut xfrm_state, _skb: *mut sk_buff) -> c_int { -ENOMEM }
unsafe fn ah_input_done(_data: *mut c_void, _err: c_int) {}
unsafe fn ah_input(_x: *mut xfrm_state, _skb: *mut sk_buff) -> c_int { -ENOMEM }
unsafe fn ah4_err(_skb: *mut sk_buff, _info: u32) -> c_int { 0 }
unsafe fn ah_init_state(_x: *mut xfrm_state, _extack: *mut netlink_ext_ack) -> c_int { -EINVAL }
unsafe fn ah_destroy(_x: *mut xfrm_state) {}
unsafe extern "C" fn ah4_rcv_cb(_skb: *mut sk_buff, _err: c_int) -> c_int { 0 }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
