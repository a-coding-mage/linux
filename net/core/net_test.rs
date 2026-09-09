// SPDX-License-Identifier: GPL-2.0-or-later

// Translated from C. External kernel and KUnit symbols are supplied by dependencies.

/* GSO */

static HDR: [core::ffi::c_char; 9] = *b"abcdefgh\0";
const GSO_TEST_SIZE: u32 = 1000;

unsafe fn __init_skb(skb: *mut sk_buff) {
    skb_reset_mac_header(skb);
    core::ptr::copy_nonoverlapping(HDR.as_ptr() as *const u8, skb_mac_header(skb), HDR.len());

    /* skb_segment expects skb->data at start of payload */
    skb_pull(skb, HDR.len());
    skb_reset_network_header(skb);
    skb_reset_transport_header(skb);

    /* proto is arbitrary, as long as not ETH_P_TEB or vlan */
    (*skb).protocol = htons(ETH_P_ATALK);
    (*skb_shinfo(skb)).gso_size = GSO_TEST_SIZE;
}

#[repr(C)]
#[derive(Copy, Clone)]
enum gso_test_nr {
    GSO_TEST_LINEAR,
    GSO_TEST_NO_GSO,
    GSO_TEST_FRAGS,
    GSO_TEST_FRAGS_PURE,
    GSO_TEST_GSO_PARTIAL,
    GSO_TEST_FRAG_LIST,
    GSO_TEST_FRAG_LIST_PURE,
    GSO_TEST_FRAG_LIST_NON_UNIFORM,
    GSO_TEST_GSO_BY_FRAGS,
}

#[repr(C)]
struct gso_test_case {
    id: gso_test_nr,
    name: *const core::ffi::c_char,
    linear_len: u32,
    nr_frags: u32,
    frags: *const u32,
    nr_frag_skbs: u32,
    frag_skbs: *const u32,
    nr_segs: u32,
    segs: *const u32,
}

static GSO_SIZE_1000: [u32; 1] = [GSO_TEST_SIZE];
static GSO_LINEAR_SEGS: [u32; 3] = [GSO_TEST_SIZE, GSO_TEST_SIZE, 1];
static GSO_FRAGS: [u32; 2] = [GSO_TEST_SIZE, 1];
static GSO_FRAGS_PURE: [u32; 3] = [GSO_TEST_SIZE, GSO_TEST_SIZE, 2];
static GSO_PARTIAL_FRAGS: [u32; 2] = [GSO_TEST_SIZE, 3];
static GSO_PARTIAL_SEGS: [u32; 2] = [2 * GSO_TEST_SIZE, 3];
static FRAG_LIST: [u32; 2] = [GSO_TEST_SIZE, GSO_TEST_SIZE];
static FRAG_LIST_SEGS: [u32; 3] = [GSO_TEST_SIZE, GSO_TEST_SIZE, GSO_TEST_SIZE];
static FRAG_LIST_NON_UNIFORM: [u32; 4] = [GSO_TEST_SIZE, 1, GSO_TEST_SIZE, 2];
static FRAG_LIST_NON_UNIFORM_SEGS: [u32; 4] = [GSO_TEST_SIZE, GSO_TEST_SIZE, GSO_TEST_SIZE, 3];
static GSO_BY_FRAGS: [u32; 4] = [100, 200, 300, 400];

static mut cases: [gso_test_case; 9] = [
    gso_test_case { id: gso_test_nr::GSO_TEST_NO_GSO, name: b"no_gso\0".as_ptr() as _, linear_len: GSO_TEST_SIZE, nr_frags: 0, frags: core::ptr::null(), nr_frag_skbs: 0, frag_skbs: core::ptr::null(), nr_segs: 1, segs: GSO_SIZE_1000.as_ptr() },
    gso_test_case { id: gso_test_nr::GSO_TEST_LINEAR, name: b"linear\0".as_ptr() as _, linear_len: GSO_TEST_SIZE + GSO_TEST_SIZE + 1, nr_frags: 0, frags: core::ptr::null(), nr_frag_skbs: 0, frag_skbs: core::ptr::null(), nr_segs: 3, segs: GSO_LINEAR_SEGS.as_ptr() },
    gso_test_case { id: gso_test_nr::GSO_TEST_FRAGS, name: b"frags\0".as_ptr() as _, linear_len: GSO_TEST_SIZE, nr_frags: 2, frags: GSO_FRAGS.as_ptr(), nr_frag_skbs: 0, frag_skbs: core::ptr::null(), nr_segs: 3, segs: GSO_LINEAR_SEGS.as_ptr() },
    gso_test_case { id: gso_test_nr::GSO_TEST_FRAGS_PURE, name: b"frags_pure\0".as_ptr() as _, linear_len: 0, nr_frags: 3, frags: GSO_FRAGS_PURE.as_ptr(), nr_frag_skbs: 0, frag_skbs: core::ptr::null(), nr_segs: 3, segs: GSO_FRAGS_PURE.as_ptr() },
    gso_test_case { id: gso_test_nr::GSO_TEST_GSO_PARTIAL, name: b"gso_partial\0".as_ptr() as _, linear_len: GSO_TEST_SIZE, nr_frags: 2, frags: GSO_PARTIAL_FRAGS.as_ptr(), nr_frag_skbs: 0, frag_skbs: core::ptr::null(), nr_segs: 2, segs: GSO_PARTIAL_SEGS.as_ptr() },
    gso_test_case { id: gso_test_nr::GSO_TEST_FRAG_LIST, name: b"frag_list\0".as_ptr() as _, linear_len: GSO_TEST_SIZE, nr_frags: 0, frags: core::ptr::null(), nr_frag_skbs: 2, frag_skbs: FRAG_LIST.as_ptr(), nr_segs: 3, segs: FRAG_LIST_SEGS.as_ptr() },
    gso_test_case { id: gso_test_nr::GSO_TEST_FRAG_LIST_PURE, name: b"frag_list_pure\0".as_ptr() as _, linear_len: 0, nr_frags: 0, frags: core::ptr::null(), nr_frag_skbs: 2, frag_skbs: FRAG_LIST.as_ptr(), nr_segs: 2, segs: FRAG_LIST.as_ptr() },
    gso_test_case { id: gso_test_nr::GSO_TEST_FRAG_LIST_NON_UNIFORM, name: b"frag_list_non_uniform\0".as_ptr() as _, linear_len: GSO_TEST_SIZE, nr_frags: 0, frags: core::ptr::null(), nr_frag_skbs: 4, frag_skbs: FRAG_LIST_NON_UNIFORM.as_ptr(), nr_segs: 4, segs: FRAG_LIST_NON_UNIFORM_SEGS.as_ptr() },
    gso_test_case { id: gso_test_nr::GSO_TEST_GSO_BY_FRAGS, name: b"gso_by_frags\0".as_ptr() as _, linear_len: 0, nr_frags: 0, frags: core::ptr::null(), nr_frag_skbs: 4, frag_skbs: GSO_BY_FRAGS.as_ptr(), nr_segs: 4, segs: GSO_BY_FRAGS.as_ptr() },
];

unsafe fn gso_test_case_to_desc(t: *mut gso_test_case, desc: *mut core::ffi::c_char) {
    sprintf(desc, b"%s\0".as_ptr() as _, (*t).name);
}

KUNIT_ARRAY_PARAM!(gso_test, cases, gso_test_case_to_desc);

// The remainder is a direct low-level translation of gso_test_func and the
// IP tunnel flags tests; kernel-provided types/macros are intentionally external.
unsafe fn gso_test_func(test: *mut kunit) {
    let shinfo_size = SKB_DATA_ALIGN(core::mem::size_of::<skb_shared_info>());
    let mut skb: *mut sk_buff;
    let mut segs: *mut sk_buff;
    let mut cur: *mut sk_buff;
    let mut next: *mut sk_buff;
    let mut last: *mut sk_buff;
    let tcase = (*test).param_value as *const gso_test_case;
    let page = alloc_page(GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL!(test, page);
    skb = build_skb(page_address(page), HDR.len() + (*tcase).linear_len as usize + shinfo_size);
    KUNIT_ASSERT_NOT_NULL!(test, skb);
    __skb_put(skb, HDR.len() + (*tcase).linear_len as usize);
    __init_skb(skb);
    if (*tcase).nr_frags != 0 {
        let mut pg_off: u32 = 0;
        let page = alloc_page(GFP_KERNEL);
        KUNIT_ASSERT_NOT_NULL!(test, page);
        page_ref_add(page, (*tcase).nr_frags - 1);
        for i in 0..(*tcase).nr_frags as usize {
            let n = *(*tcase).frags.add(i);
            skb_fill_page_desc(skb, i, page, pg_off, n);
            pg_off += n;
        }
        KUNIT_ASSERT_LE!(test, pg_off, PAGE_SIZE);
        (*skb).data_len = pg_off;
        (*skb).len += (*skb).data_len;
        (*skb).truesize += (*skb).data_len;
    }
    if !(*tcase).frag_skbs.is_null() {
        let mut total_size = 0;
        let mut total_true_size = 0;
        let mut prev: *mut sk_buff = core::ptr::null_mut();
        for i in 0..(*tcase).nr_frag_skbs as usize {
            let page = alloc_page(GFP_KERNEL);
            KUNIT_ASSERT_NOT_NULL!(test, page);
            let frag_size = *(*tcase).frag_skbs.add(i);
            let frag_skb = build_skb(page_address(page), frag_size as usize + shinfo_size);
            KUNIT_ASSERT_NOT_NULL!(test, frag_skb);
            __skb_put(frag_skb, frag_size as usize);
            if !prev.is_null() { (*prev).next = frag_skb; }
            else { (*skb_shinfo(skb)).frag_list = frag_skb; }
            prev = frag_skb;
            total_size += frag_size;
            total_true_size += (*frag_skb).truesize;
        }
        (*skb).len += total_size;
        (*skb).data_len += total_size;
        (*skb).truesize += total_true_size;
        if matches!((*tcase).id, gso_test_nr::GSO_TEST_GSO_BY_FRAGS) {
            (*skb_shinfo(skb)).gso_size = GSO_BY_FRAGS;
        }
    }
    let mut features = NETIF_F_SG | NETIF_F_HW_CSUM;
    if matches!((*tcase).id, gso_test_nr::GSO_TEST_GSO_PARTIAL) { features |= NETIF_F_GSO_PARTIAL; }
    if matches!((*tcase).id, gso_test_nr::GSO_TEST_FRAG_LIST_NON_UNIFORM) { features &= !NETIF_F_SG; }
    segs = skb_segment(skb, features);
    KUNIT_ASSERT_NOT_NULL!(test, segs);
    last = (*segs).prev;
    cur = segs;
    let mut i = 0;
    while !cur.is_null() {
        next = (*cur).next;
        KUNIT_ASSERT_EQ!(test, (*cur).len, HDR.len() + *(*tcase).segs.add(i));
        KUNIT_ASSERT_PTR_EQ!(test, skb_mac_header(cur), (*cur).data);
        KUNIT_ASSERT_PTR_EQ!(test, skb_network_header(cur), (*cur).data.add(HDR.len()));
        KUNIT_ASSERT_EQ!(test, memcmp(skb_mac_header(cur), HDR.as_ptr(), HDR.len()), 0);
        if next.is_null() { KUNIT_ASSERT_PTR_EQ!(test, cur, last); }
        consume_skb(cur);
        cur = next;
        i += 1;
    }
    KUNIT_ASSERT_EQ!(test, i, (*tcase).nr_segs as usize);
    consume_skb(skb);
}

#[repr(C)]
struct ip_tunnel_flags_test {
    name: *const core::ffi::c_char,
    src_bits: *const u16,
    exp_bits: *const u16,
    src_num: u8,
    exp_num: u8,
    exp_val: __be16,
    exp_comp: bool,
}

static ip_tunnel_flags_1: [u16; 3] = [IP_TUNNEL_KEY_BIT, IP_TUNNEL_STRICT_BIT, IP_TUNNEL_ERSPAN_OPT_BIT];
static ip_tunnel_flags_2_src: [u16; 1] = [IP_TUNNEL_CONFLICT_BIT];
static ip_tunnel_flags_2_exp: [u16; 2] = [IP_TUNNEL_CONFLICT_BIT, IP_TUNNEL_SIT_ISATAP_BIT];
static ip_tunnel_flags_3_src: [u16; 4] = [IP_TUNNEL_VXLAN_OPT_BIT, 17, 18, 20];
static ip_tunnel_flags_3_exp: [u16; 1] = [IP_TUNNEL_VXLAN_OPT_BIT];
static ip_tunnel_flags_test: [ip_tunnel_flags_test; 3] = [
    ip_tunnel_flags_test { name: b"compat\0".as_ptr() as _, src_bits: ip_tunnel_flags_1.as_ptr(), exp_bits: ip_tunnel_flags_1.as_ptr(), src_num: 3, exp_num: 3, exp_val: cpu_to_be16(BIT(IP_TUNNEL_KEY_BIT) | BIT(IP_TUNNEL_STRICT_BIT) | BIT(IP_TUNNEL_ERSPAN_OPT_BIT)), exp_comp: true },
    ip_tunnel_flags_test { name: b"conflict\0".as_ptr() as _, src_bits: ip_tunnel_flags_2_src.as_ptr(), exp_bits: ip_tunnel_flags_2_exp.as_ptr(), src_num: 1, exp_num: 2, exp_val: VTI_ISVTI, exp_comp: true },
    ip_tunnel_flags_test { name: b"new\0".as_ptr() as _, src_bits: ip_tunnel_flags_3_src.as_ptr(), exp_bits: ip_tunnel_flags_3_exp.as_ptr(), src_num: 4, exp_num: 1, exp_val: cpu_to_be16(BIT(IP_TUNNEL_VXLAN_OPT_BIT)), exp_comp: false },
];

unsafe fn ip_tunnel_flags_test_case_to_desc(t: *const ip_tunnel_flags_test, desc: *mut core::ffi::c_char) {
    strscpy(desc, (*t).name, KUNIT_PARAM_DESC_SIZE);
}

unsafe fn ip_tunnel_flags_test_run(test: *mut kunit) {
    let t = (*test).param_value as *const ip_tunnel_flags_test;
    let mut src = IP_TUNNEL_DECLARE_FLAGS!();
    let mut exp = IP_TUNNEL_DECLARE_FLAGS!();
    let mut out = IP_TUNNEL_DECLARE_FLAGS!();
    for j in 0..(*t).src_num as usize { __set_bit(*(*t).src_bits.add(j), &mut src); }
    for j in 0..(*t).exp_num as usize { __set_bit(*(*t).exp_bits.add(j), &mut exp); }
    KUNIT_ASSERT_EQ!(test, (*t).exp_comp, ip_tunnel_flags_is_be16_compat(src));
    KUNIT_ASSERT_EQ!(test, (*t).exp_val as u16, ip_tunnel_flags_to_be16(src) as u16);
    ip_tunnel_flags_from_be16(&mut out, (*t).exp_val);
    KUNIT_ASSERT_TRUE!(test, __ipt_flag_op!(bitmap_equal, exp, out));
}

static mut net_test_cases: [kunit_case; 3] = [
    KUNIT_CASE_PARAM!(gso_test_func, gso_test_gen_params),
    KUNIT_CASE_PARAM!(ip_tunnel_flags_test_run, ip_tunnel_flags_test_gen_params),
    kunit_case { ..Default::default() },
];
KUNIT_ARRAY_PARAM!(ip_tunnel_flags_test, ip_tunnel_flags_test, ip_tunnel_flags_test_case_to_desc);
static mut net_test_suite: kunit_suite = kunit_suite { name: b"net_core\0".as_ptr() as _, test_cases: net_test_cases };
KUNIT_TEST_SUITE!(net_test_suite);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
