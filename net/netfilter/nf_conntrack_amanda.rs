// SPDX-License-Identifier: GPL-2.0-or-later
/* Amanda extension for IP connection tracking
 *
 * (C) 2002 by Brian J. Murrell <netfilter@interlinx.bc.ca>
 * based on HW's ip_conntrack_irc.c as well as other modules
 * (C) 2006 Patrick McHardy <kaber@trash.net>
 */
// Linux kernel dependencies are supplied by the surrounding translation unit.

static mut MASTER_TIMEOUT: c_uint = 300;
static mut TS_ALGO: *mut c_char = b"kmp\0".as_ptr() as *mut c_char;

const HELPER_NAME: &[u8] = b"amanda\0";

// MODULE_AUTHOR("Brian J. Murrell <netfilter@interlinx.bc.ca>");
// MODULE_DESCRIPTION("Amanda connection tracking module");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("ip_conntrack_amanda");
// MODULE_ALIAS_NFCT_HELPER(HELPER_NAME);
// module_param(master_timeout, uint, 0600);
// MODULE_PARM_DESC(master_timeout, "timeout for the master connection");
// module_param(ts_algo, charp, 0400);
// MODULE_PARM_DESC(ts_algo, "textsearch algorithm to use (default kmp)");

extern "C" {
    static mut nf_nat_amanda_hook: *mut nf_nat_amanda_hook_fn;
}

#[repr(C)]
#[derive(Copy, Clone)]
enum amanda_strings {
    SEARCH_CONNECT,
    SEARCH_NEWLINE,
    SEARCH_DATA,
    SEARCH_MESG,
    SEARCH_INDEX,
    SEARCH_STATE,
}

#[repr(C)]
struct amanda_search {
    string: *const c_char,
    len: usize,
    ts: *mut ts_config,
}

static mut search: [amanda_search; 6] = [
    amanda_search { string: b"CONNECT \0".as_ptr() as *const c_char, len: 8, ts: core::ptr::null_mut() },
    amanda_search { string: b"\n\0".as_ptr() as *const c_char, len: 1, ts: core::ptr::null_mut() },
    amanda_search { string: b"DATA \0".as_ptr() as *const c_char, len: 5, ts: core::ptr::null_mut() },
    amanda_search { string: b"MESG \0".as_ptr() as *const c_char, len: 5, ts: core::ptr::null_mut() },
    amanda_search { string: b"INDEX \0".as_ptr() as *const c_char, len: 6, ts: core::ptr::null_mut() },
    amanda_search { string: b"STATE \0".as_ptr() as *const c_char, len: 6, ts: core::ptr::null_mut() },
];

unsafe fn amanda_help(skb: *mut sk_buff, protoff: c_uint, ct: *mut nf_conn,
                      ctinfo: ip_conntrack_info) -> c_int {
    let mut exp: *mut nf_conntrack_expect;
    let mut tuple: *mut nf_conntrack_tuple;
    let mut dataoff: c_uint;
    let mut start: c_uint;
    let mut stop: c_uint;
    let mut off: c_uint;
    let mut i: c_uint;
    let mut pbuf = [0i8; core::mem::size_of::<[u8; 6]>()];
    let mut tmp: *mut c_char = core::ptr::null_mut();
    let mut len: u16;
    let mut port: be16;
    let mut ret: c_int = NF_ACCEPT;
    let mut nf_nat_amanda: *mut nf_nat_amanda_hook_fn;

    if CTINFO2DIR(ctinfo) == IP_CT_DIR_ORIGINAL { return NF_ACCEPT; }
    nf_ct_refresh(ct, MASTER_TIMEOUT.wrapping_mul(HZ));
    dataoff = protoff + core::mem::size_of::<udphdr>() as c_uint;
    if dataoff >= (*skb).len {
        net_err_ratelimited!("amanda_help: skblen = %u\n", (*skb).len);
        return NF_ACCEPT;
    }
    start = skb_find_text(skb, dataoff, (*skb).len, search[SEARCH_CONNECT as usize].ts);
    if start == UINT_MAX { return ret; }
    start += dataoff + search[SEARCH_CONNECT as usize].len as c_uint;
    stop = skb_find_text(skb, start, (*skb).len, search[SEARCH_NEWLINE as usize].ts);
    if stop == UINT_MAX { return ret; }
    stop += start;
    i = SEARCH_DATA as c_uint;
    while i <= SEARCH_STATE as c_uint {
        off = skb_find_text(skb, start, stop, search[i as usize].ts);
        if off != UINT_MAX {
            off += start + search[i as usize].len as c_uint;
            len = core::cmp::min((pbuf.len() - 1) as c_uint, stop - off) as u16;
            if skb_copy_bits(skb, off, pbuf.as_mut_ptr() as *mut c_void, len as c_uint) != 0 { break; }
            pbuf[len as usize] = 0;
            port = htons(simple_strtoul(pbuf.as_mut_ptr(), &mut tmp, 10) as c_ushort);
            len = tmp.offset_from(pbuf.as_mut_ptr()) as u16;
            if port == 0 || len > 5 { break; }
            exp = nf_ct_expect_alloc(ct);
            if exp.is_null() {
                nf_ct_helper_log(skb, ct, b"cannot alloc expectation\0".as_ptr() as *const c_char);
                ret = NF_DROP; return ret;
            }
            tuple = &mut (*ct).tuplehash[IP_CT_DIR_ORIGINAL as usize].tuple;
            nf_ct_expect_init(exp, NF_CT_EXPECT_CLASS_DEFAULT, nf_ct_l3num(ct),
                              &(*tuple).src.u3, &(*tuple).dst.u3, IPPROTO_TCP, core::ptr::null(), &port);
            nf_nat_amanda = rcu_dereference(nf_nat_amanda_hook);
            if !nf_nat_amanda.is_null() && ((*ct).status & IPS_NAT_MASK) != 0 {
                ret = (*nf_nat_amanda)(skb, ct, ctinfo, protoff, off - dataoff, len as c_uint, exp);
            } else if nf_ct_expect_related(exp, 0) != 0 {
                nf_ct_helper_log(skb, ct, b"cannot add expectation\0".as_ptr() as *const c_char);
                ret = NF_DROP;
            }
            nf_ct_expect_put(exp);
        }
        i += 1;
    }
    ret
}

#[repr(C)]
struct nf_conntrack_expect_policy { max_expected: c_uint, timeout: c_uint }
static amanda_exp_policy: nf_conntrack_expect_policy = nf_conntrack_expect_policy { max_expected: 4, timeout: 180 };
static mut amanda_helper: [nf_conntrack_helper; 2] = [core::mem::zeroed(), core::mem::zeroed()];
static mut amanda_helper_ptr: [*mut nf_conntrack_helper; 2] = [core::ptr::null_mut(); 2];

unsafe extern "C" fn nf_conntrack_amanda_fini() {
    nf_conntrack_helpers_unregister(amanda_helper_ptr.as_mut_ptr(), amanda_helper_ptr.len());
    for i in 0..search.len() { textsearch_destroy(search[i].ts); }
}

unsafe extern "C" fn nf_conntrack_amanda_init() -> c_int {
    let mut ret: c_int;
    let mut i: isize = 0;
    NF_CT_HELPER_BUILD_BUG_ON(0);
    while (i as usize) < search.len() {
        search[i as usize].ts = textsearch_prepare(TS_ALGO, search[i as usize].string,
                                                    search[i as usize].len, GFP_KERNEL, TS_AUTOLOAD);
        if IS_ERR(search[i as usize].ts) { ret = PTR_ERR(search[i as usize].ts); goto_err1: {
            while i > 0 { i -= 1; textsearch_destroy(search[i as usize].ts); }
            return ret;
        }}
        i += 1;
    }
    nf_ct_helper_init(&mut amanda_helper[0], AF_INET, IPPROTO_UDP, HELPER_NAME.as_ptr(), &amanda_exp_policy, 0, amanda_help, core::ptr::null_mut(), THIS_MODULE);
    nf_ct_helper_init(&mut amanda_helper[1], AF_INET6, IPPROTO_UDP, HELPER_NAME.as_ptr(), &amanda_exp_policy, 0, amanda_help, core::ptr::null_mut(), THIS_MODULE);
    ret = nf_conntrack_helpers_register(amanda_helper.as_mut_ptr(), amanda_helper.len(), amanda_helper_ptr.as_mut_ptr());
    if ret < 0 { goto_err1: { while i > 0 { i -= 1; textsearch_destroy(search[i as usize].ts); } return ret; } }
    0
}

// module_init(nf_conntrack_amanda_init);
// module_exit(nf_conntrack_amanda_fini);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
