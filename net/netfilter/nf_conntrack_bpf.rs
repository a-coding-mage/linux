// SPDX-License-Identifier: GPL-2.0-only
/* Unstable Conntrack Helpers for XDP and TC-BPF hook
 *
 * These are called from the XDP and SCHED_CLS BPF programs. Note that it is
 * allowed to break compatibility for these functions since the interface they
 * are exposed through to BPF programs is explicitly unstable.
 */

// Kernel dependencies supplied by the surrounding repository are intentionally
// left external; the original C file includes the corresponding kernel headers.

#[repr(C)]
pub struct bpf_ct_opts {
    pub netns_id: i32,
    pub error: i32,
    pub l4proto: u8,
    pub dir: u8,
    pub ct_zone_id: u16,
    pub ct_zone_dir: u8,
    pub reserved: [u8; 3],
}

pub const NF_BPF_CT_OPTS_SZ: u32 = 16;

unsafe fn bpf_ct_opts_result(opts: *mut bpf_ct_opts, opts__sz: u32, ret: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    if !is_err(ret) {
        return ret;
    }
    if opts__sz >= core::mem::offset_of!(bpf_ct_opts, error) as u32 + core::mem::size_of::<i32>() as u32 {
        (*opts).error = ptr_err(ret);
    }
    core::ptr::null_mut()
}

unsafe fn bpf_nf_ct_tuple_parse(
    bpf_tuple: *mut bpf_sock_tuple,
    tuple_len: u32,
    protonum: u8,
    dir: u8,
    tuple: *mut nf_conntrack_tuple,
) -> i32 {
    let src = if dir != 0 { &mut (*tuple).dst.u3 } else { &mut (*tuple).src.u3 };
    let dst = if dir != 0 { &mut (*tuple).src.u3 } else { &mut (*tuple).dst.u3 };
    let sport = if dir != 0 { &mut (*tuple).dst.u } else { &mut (*tuple).src.u };
    let dport = if dir != 0 { &mut (*tuple).src.u } else { &mut (*tuple).dst.u };

    if protonum != IPPROTO_TCP && protonum != IPPROTO_UDP {
        return -EPROTO;
    }

    core::ptr::write_bytes(tuple, 0, 1);
    match tuple_len {
        n if n == core::mem::size_of::<bpf_sock_tuple_ipv4>() as u32 => {
            (*tuple).src.l3num = AF_INET;
            (*src).ip = (*bpf_tuple).ipv4.saddr;
            (*sport).tcp.port = (*bpf_tuple).ipv4.sport;
            (*dst).ip = (*bpf_tuple).ipv4.daddr;
            (*dport).tcp.port = (*bpf_tuple).ipv4.dport;
        }
        n if n == core::mem::size_of::<bpf_sock_tuple_ipv6>() as u32 => {
            (*tuple).src.l3num = AF_INET6;
            core::ptr::copy_nonoverlapping((*bpf_tuple).ipv6.saddr.as_ptr(), (*src).ip6.as_mut_ptr(), (*bpf_tuple).ipv6.saddr.len());
            (*sport).tcp.port = (*bpf_tuple).ipv6.sport;
            core::ptr::copy_nonoverlapping((*bpf_tuple).ipv6.daddr.as_ptr(), (*dst).ip6.as_mut_ptr(), (*bpf_tuple).ipv6.daddr.len());
            (*dport).tcp.port = (*bpf_tuple).ipv6.dport;
        }
        _ => return -EAFNOSUPPORT,
    }
    (*tuple).dst.protonum = protonum;
    (*tuple).dst.dir = dir;
    0
}

unsafe fn __bpf_nf_ct_alloc_entry(net: *mut net, bpf_tuple: *mut bpf_sock_tuple, tuple_len: u32, opts: *mut bpf_ct_opts, opts_len: u32, timeout: u32) -> *mut nf_conn {
    let mut otuple = core::mem::MaybeUninit::<nf_conntrack_tuple>::uninit();
    let mut rtuple = core::mem::MaybeUninit::<nf_conntrack_tuple>::uninit();
    let mut ct_zone = core::mem::MaybeUninit::<nf_conntrack_zone>::uninit();
    let mut ct_zone_dir = 0u8;
    let ct_zone_id;
    let netns_id = core::ptr::read_volatile(&(*opts).netns_id);
    let l4proto = core::ptr::read_volatile(&(*opts).l4proto);

    if opts_len != NF_BPF_CT_OPTS_SZ && opts_len != 12 { return err_ptr(-EINVAL); }
    ct_zone_id = core::ptr::read_volatile(&(*opts).ct_zone_id);
    if opts_len == NF_BPF_CT_OPTS_SZ {
        ct_zone_dir = core::ptr::read_volatile(&(*opts).ct_zone_dir);
        if (*opts).reserved[0] != 0 || (*opts).reserved[1] != 0 || (*opts).reserved[2] != 0 { return err_ptr(-EINVAL); }
    } else if ct_zone_id != 0 { return err_ptr(-EINVAL); }
    if netns_id < BPF_F_CURRENT_NETNS { return err_ptr(-EINVAL); }
    let mut err = bpf_nf_ct_tuple_parse(bpf_tuple, tuple_len, l4proto, IP_CT_DIR_ORIGINAL, otuple.as_mut_ptr());
    if err < 0 { return err_ptr(err); }
    err = bpf_nf_ct_tuple_parse(bpf_tuple, tuple_len, l4proto, IP_CT_DIR_REPLY, rtuple.as_mut_ptr());
    if err < 0 { return err_ptr(err); }
    if netns_id >= 0 {
        net = get_net_ns_by_id(net, netns_id);
        if net.is_null() { return err_ptr(-ENONET); }
    }
    if opts_len == NF_BPF_CT_OPTS_SZ {
        if ct_zone_dir == 0 { ct_zone_dir = NF_CT_DEFAULT_ZONE_DIR; (*opts).ct_zone_dir = ct_zone_dir; }
        nf_ct_zone_init(ct_zone.as_mut_ptr(), ct_zone_id, ct_zone_dir, 0);
    } else { ct_zone.write(nf_ct_zone_dflt); }
    let ct = nf_conntrack_alloc(net, ct_zone.as_ptr(), otuple.as_ptr(), rtuple.as_ptr(), GFP_ATOMIC);
    if !is_err(ct) { core::ptr::write_bytes(&mut (*ct).proto as *mut _, 0, 1); __nf_ct_set_timeout(ct, timeout.wrapping_mul(HZ)); }
    if netns_id >= 0 { put_net(net); }
    ct
}

unsafe fn __bpf_nf_ct_lookup(net: *mut net, bpf_tuple: *mut bpf_sock_tuple, tuple_len: u32, opts: *mut bpf_ct_opts, opts_len: u32) -> *mut nf_conn {
    if opts.is_null() || bpf_tuple.is_null() { return err_ptr(-EINVAL); }
    if opts_len != NF_BPF_CT_OPTS_SZ && opts_len != 12 { return err_ptr(-EINVAL); }
    let netns_id = core::ptr::read_volatile(&(*opts).netns_id);
    let l4proto = core::ptr::read_volatile(&(*opts).l4proto);
    let ct_zone_id = core::ptr::read_volatile(&(*opts).ct_zone_id);
    let mut ct_zone_dir = 0u8;
    if opts_len == NF_BPF_CT_OPTS_SZ { ct_zone_dir = (*opts).ct_zone_dir; if (*opts).reserved != [0; 3] { return err_ptr(-EINVAL); } }
    else if ct_zone_id != 0 { return err_ptr(-EINVAL); }
    if l4proto != IPPROTO_TCP && l4proto != IPPROTO_UDP { return err_ptr(-EPROTO); }
    if netns_id < BPF_F_CURRENT_NETNS { return err_ptr(-EINVAL); }
    let mut tuple = core::mem::MaybeUninit::<nf_conntrack_tuple>::uninit();
    let err = bpf_nf_ct_tuple_parse(bpf_tuple, tuple_len, l4proto, IP_CT_DIR_ORIGINAL, tuple.as_mut_ptr());
    if err < 0 { return err_ptr(err); }
    if netns_id >= 0 { net = get_net_ns_by_id(net, netns_id); if net.is_null() { return err_ptr(-ENONET); } }
    let mut zone = core::mem::MaybeUninit::<nf_conntrack_zone>::uninit();
    if opts_len == NF_BPF_CT_OPTS_SZ { if ct_zone_dir == 0 { ct_zone_dir = NF_CT_DEFAULT_ZONE_DIR; (*opts).ct_zone_dir = ct_zone_dir; } nf_ct_zone_init(zone.as_mut_ptr(), ct_zone_id, ct_zone_dir, 0); } else { zone.write(nf_ct_zone_dflt); }
    let hash = nf_conntrack_find_get(net, zone.as_ptr(), tuple.as_ptr());
    if netns_id >= 0 { put_net(net); }
    if hash.is_null() { return err_ptr(-ENOENT); }
    let ct = nf_ct_tuplehash_to_ctrack(hash); (*opts).dir = NF_CT_DIRECTION(hash); ct
}

pub unsafe fn bpf_xdp_ct_alloc(xdp_ctx: *mut xdp_md, bpf_tuple: *mut bpf_sock_tuple, tuple__sz: u32, opts: *mut bpf_ct_opts, opts__sz: u32) -> *mut nf_conn___init {
    let ctx = xdp_ctx as *mut xdp_buff;
    let nfct = __bpf_nf_ct_alloc_entry(dev_net((*(*ctx).rxq).dev), bpf_tuple, tuple__sz, opts, opts__sz, 10);
    bpf_ct_opts_result(opts, opts__sz, nfct as *mut _ ) as *mut nf_conn___init
}

pub unsafe fn bpf_xdp_ct_lookup(xdp_ctx: *mut xdp_md, bpf_tuple: *mut bpf_sock_tuple, tuple__sz: u32, opts: *mut bpf_ct_opts, opts__sz: u32) -> *mut nf_conn {
    let ctx = xdp_ctx as *mut xdp_buff; bpf_ct_opts_result(opts, opts__sz, __bpf_nf_ct_lookup(dev_net((*(*ctx).rxq).dev), bpf_tuple, tuple__sz, opts, opts__sz)) as *mut nf_conn
}

pub unsafe fn bpf_skb_ct_alloc(skb_ctx: *mut __sk_buff, bpf_tuple: *mut bpf_sock_tuple, tuple__sz: u32, opts: *mut bpf_ct_opts, opts__sz: u32) -> *mut nf_conn___init {
    let skb = skb_ctx as *mut sk_buff; let net = if !(*skb).dev.is_null() { dev_net((*skb).dev) } else { sock_net((*skb).sk) };
    bpf_ct_opts_result(opts, opts__sz, __bpf_nf_ct_alloc_entry(net, bpf_tuple, tuple__sz, opts, opts__sz, 10) as *mut _) as *mut nf_conn___init
}

pub unsafe fn bpf_skb_ct_lookup(skb_ctx: *mut __sk_buff, bpf_tuple: *mut bpf_sock_tuple, tuple__sz: u32, opts: *mut bpf_ct_opts, opts__sz: u32) -> *mut nf_conn {
    let skb = skb_ctx as *mut sk_buff; let net = if !(*skb).dev.is_null() { dev_net((*skb).dev) } else { sock_net((*skb).sk) };
    bpf_ct_opts_result(opts, opts__sz, __bpf_nf_ct_lookup(net, bpf_tuple, tuple__sz, opts, opts__sz)) as *mut nf_conn
}

pub unsafe fn bpf_ct_insert_entry(nfct_i: *mut nf_conn___init) -> *mut nf_conn {
    let nfct = nfct_i as *mut nf_conn;
    if !nf_ct_is_confirmed(nfct) { (*nfct).timeout = (*nfct).timeout.wrapping_add(nfct_time_stamp); }
    (*nfct).status |= IPS_CONFIRMED;
    let err = nf_conntrack_hash_check_insert(nfct);
    if err < 0 { nf_conntrack_free(nfct); return core::ptr::null_mut(); }
    nfct
}

pub unsafe fn bpf_ct_release(nfct: *mut nf_conn) { nf_ct_put(nfct); }
pub unsafe fn bpf_ct_set_timeout(nfct: *mut nf_conn___init, timeout: u32) { __nf_ct_set_timeout(nfct as *mut nf_conn, msecs_to_jiffies(timeout)); }
pub unsafe fn bpf_ct_change_timeout(nfct: *mut nf_conn, timeout: u32) -> i32 { __nf_ct_change_timeout(nfct, msecs_to_jiffies(timeout)) }
pub unsafe fn bpf_ct_set_status(nfct: *const nf_conn___init, status: u32) -> i32 { nf_ct_change_status_common(nfct as *mut nf_conn, status) }
pub unsafe fn bpf_ct_change_status(nfct: *mut nf_conn, status: u32) -> i32 { nf_ct_change_status_common(nfct, status) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
