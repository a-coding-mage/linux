// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * IP Payload Compression Protocol (IPComp) - RFC3173.
 *
 * Copyright (c) 2003 James Morris <jmorris@intercode.com.au>
 * Copyright (c) 2003-2025 Herbert Xu <herbert@gondor.apana.org.au>
 *
 * Todo:
 *   - Tunable compression parameters.
 *   - Compression stats.
 *   - Adaptive compression.
 */

// C dependencies supplied by the surrounding kernel translation.

const IPCOMP_SCRATCH_SIZE: usize = 65400;

#[repr(C)]
struct ipcomp_skb_cb {
    xfrm: xfrm_skb_cb,
    req: *mut acomp_req,
}

#[repr(C)]
struct ipcomp_data {
    threshold: u16,
    tfm: *mut crypto_acomp,
}

#[repr(C)]
struct ipcomp_req_extra {
    x: *mut xfrm_state,
    sg: [scatterlist; 0],
}

#[inline]
unsafe fn ipcomp_cb(skb: *mut sk_buff) -> *mut ipcomp_skb_cb {
    // BUILD_BUG_ON(sizeof(*cb) > sizeof(skb->cb));
    (*skb).cb.as_mut_ptr() as *mut ipcomp_skb_cb
}

unsafe fn ipcomp_post_acomp(skb: *mut sk_buff, err: i32, hlen: i32) -> i32 {
    let req = (*ipcomp_cb(skb)).req;
    let mut extra: *mut ipcomp_req_extra;
    let mut dsg: *mut scatterlist;
    let mut len: i32;
    let mut dlen: i32;

    if req.is_null() {
        return err;
    }

    extra = acomp_request_extra(req);
    dsg = (*extra).sg.as_mut_ptr();

    if err != 0 {
        return ipcomp_post_acomp_free_req(req, dsg, err);
    }

    dlen = (*req).dlen;
    pskb_trim_unique(skb, 0);
    __skb_put(skb, hlen as u32);

    // Only update truesize on input.
    if hlen == 0 {
        (*skb).truesize += dlen as u32;
    }
    (*skb).data_len = dlen as u32;
    (*skb).len += dlen as u32;

    while dlen != 0 {
        let frag = (*skb_shinfo(skb)).frags.as_mut_ptr().add((*skb_shinfo(skb)).nr_frags as usize);
        let page = sg_page(dsg);
        dsg = sg_next(dsg);
        len = if dlen < PAGE_SIZE as i32 { dlen } else { PAGE_SIZE as i32 };
        skb_frag_fill_page_desc(frag, page, 0, len as u32);
        (*skb_shinfo(skb)).nr_frags += 1;
        dlen -= len;
    }

    ipcomp_post_acomp_free_req(req, dsg, err)
}

unsafe fn ipcomp_post_acomp_free_req(req: *mut acomp_req, mut dsg: *mut scatterlist, err: i32) -> i32 {
    while !dsg.is_null() && !sg_page(dsg).is_null() {
        __free_page(sg_page(dsg));
        dsg = sg_next(dsg);
    }
    acomp_request_free(req);
    err
}

unsafe fn ipcomp_input_done2(skb: *mut sk_buff, err: i32) -> i32 {
    let ipch = ip_comp_hdr(skb);
    let plen = (*skb).len;
    (*skb).transport_header = (*skb).network_header + core::mem::size_of::<ip_comp_hdr>() as u16;
    let post = ipcomp_post_acomp(skb, err, 0);
    if post != 0 { post } else if (*skb).len < plen + core::mem::size_of::<ip_comp_hdr>() as u32 { -EINVAL } else { (*ipch).nexthdr as i32 }
}

unsafe extern "C" fn ipcomp_input_done(data: *mut core::ffi::c_void, err: i32) {
    let skb = data as *mut sk_buff;
    xfrm_input_resume(skb, ipcomp_input_done2(skb, err));
}

unsafe fn ipcomp_setup_req(x: *mut xfrm_state, skb: *mut sk_buff, minhead: i32, mut dlen: i32) -> *mut acomp_req {
    let dnfrags = core::cmp::min(MAX_SKB_FRAGS, 16);
    let ipcd = (*x).data as *mut ipcomp_data;
    let plen = (*skb).len as i32;
    let mut nfrags: i32;
    let mut trailer: *mut sk_buff;

    (*ipcomp_cb(skb)).req = core::ptr::null_mut();
    loop {
        if (*skb).len > PAGE_SIZE as u32 {
            if skb_linearize_cow(skb) != 0 { return ERR_PTR(-ENOMEM); }
            nfrags = 1; break;
        }
        if !skb_cloned(skb) && skb_headlen(skb) >= minhead {
            if !skb_is_nonlinear(skb) { nfrags = 1; break; }
            if !skb_has_frag_list(skb) { nfrags = skb_shinfo(skb).nr_frags as i32 + 1; break; }
        }
        nfrags = skb_cow_data(skb, if skb_headlen(skb) < minhead { minhead - skb_headlen(skb) } else { 0 }, &mut trailer);
        if nfrags < 0 { return ERR_PTR(nfrags); }
        break;
    }

    let req = acomp_request_alloc_extra((*ipcd).tfm, core::mem::size_of::<ipcomp_req_extra>() + core::mem::size_of::<scatterlist>() * (nfrags as usize + dnfrags as usize), GFP_ATOMIC);
    (*ipcomp_cb(skb)).req = req;
    if req.is_null() { return ERR_PTR(-ENOMEM); }
    let extra = acomp_request_extra(req) as *mut ipcomp_req_extra;
    (*extra).x = x;
    let dsg = (*extra).sg.as_mut_ptr();
    let sg = dsg.add(dnfrags as usize);
    sg_init_table(sg, nfrags as u32);
    let err = skb_to_sgvec(skb, sg, 0, plen);
    if err < 0 { return ERR_PTR(err); }
    sg_init_table(dsg, dnfrags as u32);
    let mut total = 0;
    let mut i = 0;
    while i < dnfrags && total < dlen {
        let page = alloc_page(GFP_ATOMIC);
        if page.is_null() { break; }
        sg_set_page(dsg.add(i as usize), page, PAGE_SIZE as u32, 0);
        total += PAGE_SIZE as i32;
        i += 1;
    }
    if i == 0 { return ERR_PTR(-ENOMEM); }
    sg_mark_end(dsg.add((i - 1) as usize));
    dlen = core::cmp::min(dlen, total);
    acomp_request_set_params(req, sg, dsg, plen, dlen);
    req
}

unsafe fn ipcomp_decompress(x: *mut xfrm_state, skb: *mut sk_buff) -> i32 {
    let req = ipcomp_setup_req(x, skb, 0, IPCOMP_SCRATCH_SIZE as i32);
    let err = PTR_ERR(req);
    if IS_ERR(req) { return ipcomp_input_done2(skb, err); }
    acomp_request_set_callback(req, 0, Some(ipcomp_input_done), skb as *mut _);
    let err = crypto_acomp_decompress(req);
    if err == -EINPROGRESS { return err; }
    ipcomp_input_done2(skb, err)
}

pub unsafe fn ipcomp_input(x: *mut xfrm_state, skb: *mut sk_buff) -> i32 {
    if !pskb_may_pull(skb, core::mem::size_of::<ip_comp_hdr>() as u32) { return -EINVAL; }
    (*skb).ip_summed = CHECKSUM_NONE;
    __skb_pull(skb, core::mem::size_of::<ip_comp_hdr>() as u32);
    ipcomp_decompress(x, skb)
}

unsafe fn ipcomp_output_push(skb: *mut sk_buff) -> i32 {
    skb_push(skb, -skb_network_offset(skb));
    0
}

unsafe fn ipcomp_output_done2(x: *mut xfrm_state, skb: *mut sk_buff, mut err: i32) -> i32 {
    err = ipcomp_post_acomp(skb, err, core::mem::size_of::<ip_comp_hdr>() as i32);
    if err == 0 {
        let ipch = ip_comp_hdr(skb);
        (*ipch).nexthdr = *skb_mac_header(skb);
        (*ipch).flags = 0;
        (*ipch).cpi = htons(ntohl((*x).id.spi) as u16);
        *skb_mac_header(skb) = IPPROTO_COMP;
    }
    ipcomp_output_push(skb)
}

unsafe extern "C" fn ipcomp_output_done(data: *mut core::ffi::c_void, err: i32) {
    let skb = data as *mut sk_buff;
    let req = (*ipcomp_cb(skb)).req;
    let extra = acomp_request_extra(req) as *mut ipcomp_req_extra;
    xfrm_output_resume(skb_to_full_sk(skb), skb, ipcomp_output_done2((*extra).x, skb, err));
}

unsafe fn ipcomp_compress(x: *mut xfrm_state, skb: *mut sk_buff) -> i32 {
    let req = ipcomp_setup_req(x, skb, core::mem::size_of::<ip_comp_hdr>() as i32, (*skb).len as i32 - core::mem::size_of::<ip_comp_hdr>() as i32);
    let err = PTR_ERR(req);
    if IS_ERR(req) { return ipcomp_output_done2(x, skb, err); }
    acomp_request_set_callback(req, 0, Some(ipcomp_output_done), skb as *mut _);
    let err = crypto_acomp_compress(req);
    if err == -EINPROGRESS { return err; }
    ipcomp_output_done2(x, skb, err)
}

pub unsafe fn ipcomp_output(x: *mut xfrm_state, skb: *mut sk_buff) -> i32 {
    let ipcd = (*x).data as *mut ipcomp_data;
    if (*skb).len < (*ipcd).threshold as u32 { return ipcomp_output_push(skb); }
    ipcomp_compress(x, skb)
}

unsafe fn ipcomp_free_data(ipcd: *mut ipcomp_data) { crypto_free_acomp((*ipcd).tfm); }

pub unsafe fn ipcomp_destroy(x: *mut xfrm_state) {
    let ipcd = (*x).data as *mut ipcomp_data;
    if ipcd.is_null() { return; }
    ipcomp_free_data(ipcd);
    kfree(ipcd as *mut _);
}

pub unsafe fn ipcomp_init_state(x: *mut xfrm_state, extack: *mut netlink_ext_ack) -> i32 {
    let mut ipcd: *mut ipcomp_data;
    if (*x).calg.is_null() { NL_SET_ERR_MSG(extack, "Missing required compression algorithm"); return -EINVAL; }
    if !(*x).encap.is_null() { NL_SET_ERR_MSG(extack, "IPComp is not compatible with encapsulation"); return -EINVAL; }
    ipcd = kzalloc_obj::<ipcomp_data>();
    if ipcd.is_null() { return -ENOMEM; }
    (*ipcd).tfm = crypto_alloc_acomp((*(*x).calg).alg_name, 0, 0);
    if IS_ERR((*ipcd).tfm) { ipcomp_free_data(ipcd); kfree(ipcd as *mut _); return PTR_ERR((*ipcd).tfm); }
    let calg_desc = xfrm_calg_get_byname((*(*x).calg).alg_name, 0);
    BUG_ON(calg_desc.is_null());
    (*ipcd).threshold = (*calg_desc).uinfo.comp.threshold;
    (*x).data = ipcd as *mut _;
    0
}

// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("IP Payload Compression Protocol (IPComp) - RFC3173");
// MODULE_AUTHOR("James Morris <jmorris@intercode.com.au>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
