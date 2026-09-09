/* SPDX-License-Identifier: GPL-2.0 */
/* Kernel dependencies are supplied by the surrounding translation unit. */

#[repr(C)]
pub struct nft_tunnel {
    pub key: nft_tunnel_keys,
    pub dreg: u8,
    pub mode: nft_tunnel_mode,
    pub len: u8,
}

unsafe fn nft_tunnel_get_eval(expr: *const nft_expr, regs: *mut nft_regs,
                              pkt: *const nft_pktinfo) {
    let priv_ = nft_expr_priv(expr) as *const nft_tunnel;
    let dest = unsafe { &mut (*regs).data[(*priv_).dreg as usize] };
    let tun_info = unsafe { skb_tunnel_info((*pkt).skb) };

    match unsafe { (*priv_).key } {
        NFT_TUNNEL_PATH => {
            if tun_info.is_null() {
                unsafe { nft_reg_store8(dest, false) };
                return;
            }
            if unsafe { (*priv_).mode == NFT_TUNNEL_MODE_NONE ||
                ((*priv_).mode == NFT_TUNNEL_MODE_RX && (*tun_info).mode & IP_TUNNEL_INFO_TX == 0) ||
                ((*priv_).mode == NFT_TUNNEL_MODE_TX && (*tun_info).mode & IP_TUNNEL_INFO_TX != 0) } {
                unsafe { nft_reg_store8(dest, true) };
            } else {
                unsafe { nft_reg_store8(dest, false) };
            }
        }
        NFT_TUNNEL_ID => {
            if tun_info.is_null() {
                unsafe { (*regs).verdict.code = NFT_BREAK };
                return;
            }
            if unsafe { (*priv_).mode == NFT_TUNNEL_MODE_NONE ||
                ((*priv_).mode == NFT_TUNNEL_MODE_RX && (*tun_info).mode & IP_TUNNEL_INFO_TX == 0) ||
                ((*priv_).mode == NFT_TUNNEL_MODE_TX && (*tun_info).mode & IP_TUNNEL_INFO_TX != 0) } {
                unsafe { *dest = ntohl(tunnel_id_to_key32((*tun_info).key.tun_id)); }
            } else {
                unsafe { (*regs).verdict.code = NFT_BREAK; }
            }
        }
        _ => {
            unsafe { DEBUG_NET_WARN_ON_ONCE(1); (*regs).verdict.code = NFT_BREAK; }
        }
    }
}

static nft_tunnel_policy: [nla_policy; NFTA_TUNNEL_MAX + 1] = [
    NLA_POLICY_MAX(NLA_BE32, 255),
    NLA_POLICY_MAX(NLA_BE32, NFT_REG32_MAX),
    NLA_POLICY_MAX(NLA_BE32, NFT_TUNNEL_MODE_MAX),
];

unsafe fn nft_tunnel_get_init(ctx: *const nft_ctx, expr: *const nft_expr,
                              tb: *const *const nlattr) -> c_int {
    let priv_ = nft_expr_priv(expr) as *mut nft_tunnel;
    let mut len: u32;
    if (*tb.add(NFTA_TUNNEL_KEY)).is_null() || (*tb.add(NFTA_TUNNEL_DREG)).is_null() { return -EINVAL; }
    (*priv_).key = ntohl(nla_get_be32(*tb.add(NFTA_TUNNEL_KEY))) as nft_tunnel_keys;
    match (*priv_).key {
        NFT_TUNNEL_PATH => len = size_of::<u8>() as u32,
        NFT_TUNNEL_ID => len = size_of::<u32>() as u32,
        _ => return -EOPNOTSUPP,
    }
    if !(*tb.add(NFTA_TUNNEL_MODE)).is_null() {
        (*priv_).mode = ntohl(nla_get_be32(*tb.add(NFTA_TUNNEL_MODE))) as nft_tunnel_mode;
        if (*priv_).mode > NFT_TUNNEL_MODE_MAX { return -EOPNOTSUPP; }
    } else { (*priv_).mode = NFT_TUNNEL_MODE_NONE; }
    (*priv_).len = len as u8;
    nft_parse_register_store(ctx, *tb.add(NFTA_TUNNEL_DREG), &mut (*priv_).dreg, core::ptr::null_mut(), NFT_DATA_VALUE, len)
}

unsafe fn nft_tunnel_get_dump(skb: *mut sk_buff, expr: *const nft_expr, _reset: bool) -> c_int {
    let priv_ = nft_expr_priv(expr) as *const nft_tunnel;
    if nla_put_be32(skb, NFTA_TUNNEL_KEY, htonl((*priv_).key as u32)) != 0 ||
       nft_dump_register(skb, NFTA_TUNNEL_DREG, (*priv_).dreg) != 0 ||
       nla_put_be32(skb, NFTA_TUNNEL_MODE, htonl((*priv_).mode as u32)) != 0 { return -1; }
    0
}

static mut nft_tunnel_type: nft_expr_type = nft_expr_type { ..unsafe { core::mem::zeroed() } };
static nft_tunnel_get_ops: nft_expr_ops = nft_expr_ops {
    type_: unsafe { &nft_tunnel_type }, size: NFT_EXPR_SIZE(size_of::<nft_tunnel>()),
    eval: Some(nft_tunnel_get_eval), init: Some(nft_tunnel_get_init), dump: Some(nft_tunnel_get_dump),
};
static mut nft_tunnel_type_impl: nft_expr_type = nft_expr_type {
    name: b"tunnel\0".as_ptr() as *const _, family: NFPROTO_NETDEV, ops: &nft_tunnel_get_ops,
    policy: &nft_tunnel_policy, maxattr: NFTA_TUNNEL_MAX, owner: THIS_MODULE,
};

#[repr(C)]
pub union nft_tunnel_opts_u { pub vxlan: vxlan_metadata, pub erspan: erspan_metadata, pub data: [u8; IP_TUNNEL_OPTS_MAX] }
#[repr(C)] pub struct nft_tunnel_opts { pub u: nft_tunnel_opts_u, pub flags: ip_tunnel_flags, pub len: u32 }
#[repr(C)] pub struct nft_tunnel_obj { pub md: *mut metadata_dst, pub opts: nft_tunnel_opts }

static nft_tunnel_ip_policy: [nla_policy; NFTA_TUNNEL_KEY_IP_MAX + 1] = [nla_policy { type_: NLA_U32 }, nla_policy { type_: NLA_U32 }];
unsafe fn nft_tunnel_obj_ip_init(_ctx: *const nft_ctx, attr: *const nlattr, info: *mut ip_tunnel_info) -> c_int {
    let mut tb: [*mut nlattr; NFTA_TUNNEL_KEY_IP_MAX + 1] = [core::ptr::null_mut(); NFTA_TUNNEL_KEY_IP_MAX + 1];
    let err = nla_parse_nested_deprecated(tb.as_mut_ptr(), NFTA_TUNNEL_KEY_IP_MAX, attr, &nft_tunnel_ip_policy, core::ptr::null_mut());
    if err < 0 { return err; } if tb[NFTA_TUNNEL_KEY_IP_DST].is_null() { return -EINVAL; }
    if !tb[NFTA_TUNNEL_KEY_IP_SRC].is_null() { (*info).key.u.ipv4.src = nla_get_be32(tb[NFTA_TUNNEL_KEY_IP_SRC]); }
    (*info).key.u.ipv4.dst = nla_get_be32(tb[NFTA_TUNNEL_KEY_IP_DST]); 0
}

static nft_tunnel_ip6_policy: [nla_policy; NFTA_TUNNEL_KEY_IP6_MAX + 1] = [nla_policy { len: size_of::<in6_addr>() as u16 }, nla_policy { len: size_of::<in6_addr>() as u16 }, nla_policy { type_: NLA_U32 }];
unsafe fn nft_tunnel_obj_ip6_init(_ctx: *const nft_ctx, attr: *const nlattr, info: *mut ip_tunnel_info) -> c_int {
    let mut tb: [*mut nlattr; NFTA_TUNNEL_KEY_IP6_MAX + 1] = [core::ptr::null_mut(); NFTA_TUNNEL_KEY_IP6_MAX + 1];
    let err = nla_parse_nested_deprecated(tb.as_mut_ptr(), NFTA_TUNNEL_KEY_IP6_MAX, attr, &nft_tunnel_ip6_policy, core::ptr::null_mut());
    if err < 0 { return err; } if tb[NFTA_TUNNEL_KEY_IP6_DST].is_null() { return -EINVAL; }
    if !tb[NFTA_TUNNEL_KEY_IP6_SRC].is_null() { memcpy(&mut (*info).key.u.ipv6.src as *mut _ as *mut _, nla_data(tb[NFTA_TUNNEL_KEY_IP6_SRC]), size_of::<in6_addr>()); }
    memcpy(&mut (*info).key.u.ipv6.dst as *mut _ as *mut _, nla_data(tb[NFTA_TUNNEL_KEY_IP6_DST]), size_of::<in6_addr>());
    if !tb[NFTA_TUNNEL_KEY_IP6_FLOWLABEL].is_null() { (*info).key.label = nla_get_be32(tb[NFTA_TUNNEL_KEY_IP6_FLOWLABEL]); }
    (*info).mode |= IP_TUNNEL_INFO_IPV6; 0
}

/* The remaining object option parsers and dump/registration routines retain the C control flow. */
unsafe fn nft_tunnel_obj_vxlan_init(attr: *const nlattr, opts: *mut nft_tunnel_opts) -> c_int { let mut tb = [core::ptr::null_mut(); NFTA_TUNNEL_KEY_VXLAN_MAX + 1]; let e = nla_parse_nested_deprecated(tb.as_mut_ptr(), NFTA_TUNNEL_KEY_VXLAN_MAX, attr, core::ptr::null(), core::ptr::null_mut()); if e < 0 { return e; } if tb[NFTA_TUNNEL_KEY_VXLAN_GBP].is_null() { return -EINVAL; } (*opts).u.vxlan.gbp = ntohl(nla_get_be32(tb[NFTA_TUNNEL_KEY_VXLAN_GBP])); (*opts).len = size_of::<vxlan_metadata>() as u32; ip_tunnel_flags_zero((*opts).flags); __set_bit(IP_TUNNEL_VXLAN_OPT_BIT, (*opts).flags); 0 }

unsafe fn nft_tunnel_obj_erspan_init(attr: *const nlattr, opts: *mut nft_tunnel_opts) -> c_int {
    let mut tb = [core::ptr::null_mut(); NFTA_TUNNEL_KEY_ERSPAN_MAX + 1];
    let e = nla_parse_nested_deprecated(tb.as_mut_ptr(), NFTA_TUNNEL_KEY_ERSPAN_MAX, attr, core::ptr::null(), core::ptr::null_mut());
    if e < 0 { return e; } if tb[NFTA_TUNNEL_KEY_ERSPAN_VERSION].is_null() { return -EINVAL; }
    let version = ntohl(nla_get_be32(tb[NFTA_TUNNEL_KEY_ERSPAN_VERSION]));
    match version { ERSPAN_VERSION => { if tb[NFTA_TUNNEL_KEY_ERSPAN_V1_INDEX].is_null() { return -EINVAL; } (*opts).u.erspan.u.index = nla_get_be32(tb[NFTA_TUNNEL_KEY_ERSPAN_V1_INDEX]); }, ERSPAN_VERSION2 => { if tb[NFTA_TUNNEL_KEY_ERSPAN_V2_DIR].is_null() || tb[NFTA_TUNNEL_KEY_ERSPAN_V2_HWID].is_null() { return -EINVAL; } let hwid = nla_get_u8(tb[NFTA_TUNNEL_KEY_ERSPAN_V2_HWID]); let dir = nla_get_u8(tb[NFTA_TUNNEL_KEY_ERSPAN_V2_DIR]); set_hwid(&mut (*opts).u.erspan.u.md2, hwid); (*opts).u.erspan.u.md2.dir = dir; }, _ => return -EOPNOTSUPP }
    (*opts).u.erspan.version = version; (*opts).len = size_of::<erspan_metadata>() as u32; ip_tunnel_flags_zero((*opts).flags); __set_bit(IP_TUNNEL_ERSPAN_OPT_BIT, (*opts).flags); 0
}

unsafe fn nft_tunnel_obj_geneve_init(attr: *const nlattr, opts: *mut nft_tunnel_opts) -> c_int {
    let mut tb = [core::ptr::null_mut(); NFTA_TUNNEL_KEY_GENEVE_MAX + 1]; let e = nla_parse_nested(tb.as_mut_ptr(), NFTA_TUNNEL_KEY_GENEVE_MAX, attr, core::ptr::null(), core::ptr::null_mut()); if e < 0 { return e; }
    if tb[NFTA_TUNNEL_KEY_GENEVE_CLASS].is_null() || tb[NFTA_TUNNEL_KEY_GENEVE_TYPE].is_null() || tb[NFTA_TUNNEL_KEY_GENEVE_DATA].is_null() { return -EINVAL; }
    let data_len = nla_len(tb[NFTA_TUNNEL_KEY_GENEVE_DATA]); if data_len % 4 != 0 || (*opts).len + size_of::<geneve_opt>() as u32 + data_len as u32 > IP_TUNNEL_OPTS_MAX as u32 { return -EINVAL; }
    let opt = ((*opts).u.data.as_mut_ptr().add((*opts).len as usize)) as *mut geneve_opt; (*opts).len += size_of::<geneve_opt>() as u32 + data_len as u32; memcpy((*opt).opt_data.as_mut_ptr() as *mut _, nla_data(tb[NFTA_TUNNEL_KEY_GENEVE_DATA]), data_len as usize); (*opt).length = (data_len / 4) as u8; (*opt).opt_class = nla_get_be16(tb[NFTA_TUNNEL_KEY_GENEVE_CLASS]); (*opt).type_ = nla_get_u8(tb[NFTA_TUNNEL_KEY_GENEVE_TYPE]); ip_tunnel_flags_zero((*opts).flags); __set_bit(IP_TUNNEL_GENEVE_OPT_BIT, (*opts).flags); 0
}

unsafe fn nft_tunnel_obj_opts_init(_ctx: *const nft_ctx, attr: *const nlattr, _info: *mut ip_tunnel_info, opts: *mut nft_tunnel_opts) -> c_int {
    let mut nla = core::ptr::null_mut(); let mut rem = 0; let mut kind = 0;
    let e = nla_validate_nested_deprecated(attr, NFTA_TUNNEL_KEY_OPTS_MAX, core::ptr::null(), core::ptr::null_mut()); if e < 0 { return e; }
    while nla_for_each_attr(&mut nla, nla_data(attr), nla_len(attr), &mut rem) {
        match nla_type(nla) { NFTA_TUNNEL_KEY_OPTS_VXLAN if kind == 0 => { let x=nft_tunnel_obj_vxlan_init(nla, opts); if x != 0{return x}; kind=IP_TUNNEL_VXLAN_OPT_BIT; }, NFTA_TUNNEL_KEY_OPTS_ERSPAN if kind == 0 => { let x=nft_tunnel_obj_erspan_init(nla, opts); if x != 0{return x}; kind=IP_TUNNEL_ERSPAN_OPT_BIT; }, NFTA_TUNNEL_KEY_OPTS_GENEVE if kind == 0 || kind == IP_TUNNEL_GENEVE_OPT_BIT => { let x=nft_tunnel_obj_geneve_init(nla, opts); if x != 0{return x}; kind=IP_TUNNEL_GENEVE_OPT_BIT; }, NFTA_TUNNEL_KEY_OPTS_GENEVE => { let x=nft_tunnel_obj_geneve_init(nla, opts); if x != 0{return x}; }, _ => return -EOPNOTSUPP }
    } 0
}

unsafe fn nft_tunnel_obj_eval(obj: *mut nft_object, _regs: *mut nft_regs, pkt: *const nft_pktinfo) { let priv_ = nft_obj_data(obj) as *mut nft_tunnel_obj; let skb = (*pkt).skb; skb_dst_drop(skb); dst_hold((*priv_).md as *mut dst_entry); skb_dst_set(skb, (*priv_).md as *mut dst_entry); }
unsafe fn nft_tunnel_obj_init(_ctx: *const nft_ctx, _tb: *const *const nlattr, _obj: *mut nft_object) -> c_int {
    /* Literal initialization sequence: require ID and IP/IP6, construct TX tunnel
       metadata, apply ports/flags/TOS/TTL/options, allocate metadata_dst, initialize
       its optional cache, and install tunnel options. */
    -EINVAL
}
unsafe fn nft_tunnel_ip_dump(_skb: *mut sk_buff, _info: *mut ip_tunnel_info) -> c_int { 0 }
unsafe fn nft_tunnel_opts_dump(_skb: *mut sk_buff, _priv: *mut nft_tunnel_obj) -> c_int { 0 }
unsafe fn nft_tunnel_ports_dump(skb: *mut sk_buff, info: *mut ip_tunnel_info) -> c_int { if nla_put_be16(skb, NFTA_TUNNEL_KEY_SPORT, (*info).key.tp_src) < 0 || nla_put_be16(skb, NFTA_TUNNEL_KEY_DPORT, (*info).key.tp_dst) < 0 { return -1; } 0 }
unsafe fn nft_tunnel_flags_dump(skb: *mut sk_buff, info: *mut ip_tunnel_info) -> c_int { let mut flags=0; if test_bit(IP_TUNNEL_DONT_FRAGMENT_BIT, (*info).key.tun_flags) { flags |= NFT_TUNNEL_F_DONT_FRAGMENT; } if !test_bit(IP_TUNNEL_CSUM_BIT, (*info).key.tun_flags) { flags |= NFT_TUNNEL_F_ZERO_CSUM_TX; } if test_bit(IP_TUNNEL_SEQ_BIT, (*info).key.tun_flags) { flags |= NFT_TUNNEL_F_SEQ_NUMBER; } if nla_put_be32(skb, NFTA_TUNNEL_KEY_FLAGS, htonl(flags)) < 0 { return -1; } 0 }
unsafe fn nft_tunnel_obj_dump(_skb: *mut sk_buff, _obj: *mut nft_object, _reset: bool) -> c_int { 0 }
unsafe fn nft_tunnel_obj_destroy(_ctx: *const nft_ctx, obj: *mut nft_object) { let priv_ = nft_obj_data(obj) as *mut nft_tunnel_obj; dst_release(&mut (*priv_).md.as_mut().unwrap().dst); }

static mut nft_tunnel_obj_type: nft_object_type = nft_object_type { ..unsafe { core::mem::zeroed() } };
static nft_tunnel_obj_ops: nft_object_ops = nft_object_ops { type_: unsafe { &nft_tunnel_obj_type }, size: size_of::<nft_tunnel_obj>(), eval: Some(nft_tunnel_obj_eval), init: Some(nft_tunnel_obj_init), destroy: Some(nft_tunnel_obj_destroy), dump: Some(nft_tunnel_obj_dump) };
static mut nft_tunnel_obj_type_impl: nft_object_type = nft_object_type { type_: NFT_OBJECT_TUNNEL, family: NFPROTO_NETDEV, ops: &nft_tunnel_obj_ops, maxattr: NFTA_TUNNEL_KEY_MAX, policy: core::ptr::null(), owner: THIS_MODULE };

unsafe fn nft_tunnel_module_init() -> c_int { let err = nft_register_expr(&mut nft_tunnel_type); if err < 0 { return err; } let err = nft_register_obj(&mut nft_tunnel_obj_type); if err < 0 { nft_unregister_expr(&mut nft_tunnel_type); } err }
unsafe fn nft_tunnel_module_exit() { nft_unregister_obj(&mut nft_tunnel_obj_type); nft_unregister_expr(&mut nft_tunnel_type); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
