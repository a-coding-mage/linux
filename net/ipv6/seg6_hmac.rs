// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * SR-IPv6 implementation -- HMAC functions
 *
 * Direct Rust translation of seg6_hmac.c. Kernel types, constants, globals,
 * and functions referenced below are supplied by the surrounding kernel
 * bindings.
 */

#[repr(C)]
struct HmacStorage {
    bh_lock: local_lock_t,
    hmac_ring: [c_char; SEG6_HMAC_RING_SIZE],
}

static mut HMAC_STORAGE: PerCpu<HmacStorage> = PerCpu::new(HmacStorage {
    bh_lock: INIT_LOCAL_LOCK!(bh_lock),
    hmac_ring: [0; SEG6_HMAC_RING_SIZE],
});

unsafe extern "C" fn seg6_hmac_cmpfn(
    arg: *mut rhashtable_compare_arg,
    obj: *const c_void,
) -> c_int {
    let hinfo = obj as *const seg6_hmac_info;
    ((*hinfo).hmackeyid != *( (*arg).key as *const u32)) as c_int
}

#[inline]
unsafe fn seg6_hinfo_release(hinfo: *mut seg6_hmac_info) {
    kfree_rcu!(hinfo, rcu);
}

unsafe extern "C" fn seg6_free_hi(ptr: *mut c_void, _arg: *mut c_void) {
    let hinfo = ptr as *mut seg6_hmac_info;
    if !hinfo.is_null() {
        seg6_hinfo_release(hinfo);
    }
}

static RHT_PARAMS: rhashtable_params = rhashtable_params {
    head_offset: core::mem::offset_of!(seg6_hmac_info, node),
    key_offset: core::mem::offset_of!(seg6_hmac_info, hmackeyid),
    key_len: core::mem::size_of::<u32>(),
    automatic_shrinking: true,
    obj_cmpfn: Some(seg6_hmac_cmpfn),
};

unsafe fn seg6_get_tlv_hmac(srh: *mut ipv6_sr_hdr) -> *mut sr6_tlv_hmac {
    if (*srh).hdrlen < ((*srh).first_segment as i32 + 1) * 2 + 5 {
        return core::ptr::null_mut();
    }
    if !sr_has_hmac(srh) {
        return core::ptr::null_mut();
    }

    let tlv = ((srh as *mut c_char)
        .add((((*srh).hdrlen as usize + 1) << 3) - 40)) as *mut sr6_tlv_hmac;
    if (*tlv).tlvhdr.type_ != SR6_TLV_HMAC || (*tlv).tlvhdr.len != 38 {
        return core::ptr::null_mut();
    }
    tlv
}

pub unsafe fn seg6_hmac_compute(
    hinfo: *mut seg6_hmac_info,
    hdr: *mut ipv6_sr_hdr,
    saddr: *mut in6_addr,
    output: *mut u8,
) -> c_int {
    let hmackeyid = cpu_to_be32((*hinfo).hmackeyid);
    let plen = 16 + 1 + 1 + 4 + ((*hdr).first_segment as c_int + 1) * 16;
    if plen >= SEG6_HMAC_RING_SIZE as c_int {
        return -EMSGSIZE;
    }

    local_bh_disable();
    local_lock_nested_bh!(&mut (*HMAC_STORAGE.get()).bh_lock);
    let ring = (*HMAC_STORAGE.get()).hmac_ring.as_mut_ptr() as *mut u8;
    let mut off = ring;
    core::ptr::copy_nonoverlapping(saddr as *const u8, off, 16); off = off.add(16);
    *off = (*hdr).first_segment; off = off.add(1);
    *off = (*hdr).flags; off = off.add(1);
    core::ptr::copy_nonoverlapping(&hmackeyid as *const _ as *const u8, off, 4); off = off.add(4);
    for i in 0..((*hdr).first_segment as usize + 1) {
        core::ptr::copy_nonoverlapping((*hdr).segments.add(i) as *const u8, off, 16);
        off = off.add(16);
    }

    let mut ret = 0;
    match (*hinfo).alg_id {
        SEG6_HMAC_ALGO_SHA1 => {
            hmac_sha1(&mut (*hinfo).key.sha1, ring, plen, output);
            core::ptr::write_bytes(output.add(SHA1_DIGEST_SIZE), 0,
                SEG6_HMAC_FIELD_LEN - SHA1_DIGEST_SIZE);
        }
        SEG6_HMAC_ALGO_SHA256 => {
            hmac_sha256(&mut (*hinfo).key.sha256, ring, plen, output);
        }
        _ => { WARN_ON_ONCE!(true); ret = -EINVAL; }
    }
    local_unlock_nested_bh!(&mut (*HMAC_STORAGE.get()).bh_lock);
    local_bh_enable();
    ret
}

pub unsafe fn seg6_hmac_validate_skb(skb: *mut sk_buff) -> bool {
    let mut hmac_output = [0u8; SEG6_HMAC_FIELD_LEN];
    let net = dev_net((*skb).dev);
    let idev = __in6_dev_get((*skb).dev);
    if idev.is_null() { return false; }
    let srh = skb_transport_header(skb) as *mut ipv6_sr_hdr;
    let tlv = seg6_get_tlv_hmac(srh);
    let require_hmac = READ_ONCE!((*idev).cnf.seg6_require_hmac);
    if require_hmac > 0 && tlv.is_null() { return false; }
    if require_hmac < 0 { return true; }
    if require_hmac == 0 && tlv.is_null() { return true; }
    let hinfo = seg6_hmac_info_lookup(net, be32_to_cpu((*tlv).hmackeyid));
    if hinfo.is_null() || seg6_hmac_compute(hinfo, srh, &mut (*ipv6_hdr(skb)).saddr, hmac_output.as_mut_ptr()) != 0 { return false; }
    crypto_memneq(hmac_output.as_ptr() as *const c_void, (*tlv).hmac.as_ptr() as *const c_void, SEG6_HMAC_FIELD_LEN) == 0
}

pub unsafe fn seg6_hmac_info_lookup(net: *mut net, key: u32) -> *mut seg6_hmac_info {
    let sdata = seg6_pernet(net);
    rhashtable_lookup_fast(&mut (*sdata).hmac_infos, &key, &RHT_PARAMS)
}

pub unsafe fn seg6_hmac_info_add(net: *mut net, _key: u32, hinfo: *mut seg6_hmac_info) -> c_int {
    let sdata = seg6_pernet(net);
    match (*hinfo).alg_id {
        SEG6_HMAC_ALGO_SHA1 => hmac_sha1_preparekey(&mut (*hinfo).key.sha1, (*hinfo).secret, (*hinfo).slen),
        SEG6_HMAC_ALGO_SHA256 => hmac_sha256_preparekey(&mut (*hinfo).key.sha256, (*hinfo).secret, (*hinfo).slen),
        _ => return -EINVAL,
    }
    rhashtable_lookup_insert_fast(&mut (*sdata).hmac_infos, &mut (*hinfo).node, &RHT_PARAMS)
}

pub unsafe fn seg6_hmac_info_del(net: *mut net, key: u32) -> c_int {
    let sdata = seg6_pernet(net);
    let hinfo = seg6_hmac_info_lookup(net, key);
    if hinfo.is_null() { return -ENOENT; }
    let err = rhashtable_remove_fast(&mut (*sdata).hmac_infos, &mut (*hinfo).node, &RHT_PARAMS);
    if err != 0 { return err; }
    seg6_hinfo_release(hinfo); 0
}

pub unsafe fn seg6_push_hmac(net: *mut net, saddr: *mut in6_addr, srh: *mut ipv6_sr_hdr) -> c_int {
    let tlv = seg6_get_tlv_hmac(srh);
    if tlv.is_null() { return -EINVAL; }
    rcu_read_lock();
    let hinfo = seg6_hmac_info_lookup(net, be32_to_cpu((*tlv).hmackeyid));
    let err = if hinfo.is_null() { -ENOENT } else {
        core::ptr::write_bytes((*tlv).hmac.as_mut_ptr(), 0, SEG6_HMAC_FIELD_LEN);
        seg6_hmac_compute(hinfo, srh, saddr, (*tlv).hmac.as_mut_ptr())
    };
    rcu_read_unlock(); err
}

pub unsafe fn seg6_hmac_net_init(net: *mut net) -> c_int {
    let sdata = seg6_pernet(net);
    rhashtable_init(&mut (*sdata).hmac_infos, &RHT_PARAMS)
}

pub unsafe fn seg6_hmac_net_exit(net: *mut net) {
    let sdata = seg6_pernet(net);
    rhashtable_free_and_destroy(&mut (*sdata).hmac_infos, Some(seg6_free_hi), core::ptr::null_mut());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
