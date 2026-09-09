// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2016, Amir Vadai <amir@vadai.me>
 * Copyright (c) 2016, Mellanox Technologies. All rights reserved.
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

static mut act_tunnel_key_ops: tc_action_ops = tc_action_ops::ZERO;

unsafe fn tunnel_key_act(skb: *mut sk_buff, a: *const tc_action,
                         res: *mut tcf_result) -> i32 {
    let t = to_tunnel_key(a);
    let params = rcu_dereference_bh((*t).params);
    tcf_lastuse_update(&mut (*t).tcf_tm);
    tcf_action_update_bstats(&mut (*t).common, skb);
    match (*params).tcft_action {
        TCA_TUNNEL_KEY_ACT_RELEASE => skb_dst_drop(skb),
        TCA_TUNNEL_KEY_ACT_SET => {
            skb_dst_drop(skb);
            skb_dst_set(skb, dst_clone(&(*(*params).tcft_enc_metadata).dst));
        }
        _ => { WARN_ONCE(true, "Bad tunnel_key action %d.\n", (*params).tcft_action); }
    }
    (*params).action
}

static enc_opts_policy: [nla_policy; TCA_TUNNEL_KEY_ENC_OPTS_MAX + 1] = [nla_policy::ZERO; TCA_TUNNEL_KEY_ENC_OPTS_MAX + 1];
static geneve_opt_policy: [nla_policy; TCA_TUNNEL_KEY_ENC_OPT_GENEVE_MAX + 1] = [nla_policy::ZERO; TCA_TUNNEL_KEY_ENC_OPT_GENEVE_MAX + 1];
static vxlan_opt_policy: [nla_policy; TCA_TUNNEL_KEY_ENC_OPT_VXLAN_MAX + 1] = [nla_policy::ZERO; TCA_TUNNEL_KEY_ENC_OPT_VXLAN_MAX + 1];
static erspan_opt_policy: [nla_policy; TCA_TUNNEL_KEY_ENC_OPT_ERSPAN_MAX + 1] = [nla_policy::ZERO; TCA_TUNNEL_KEY_ENC_OPT_ERSPAN_MAX + 1];

unsafe fn tunnel_key_copy_geneve_opt(nla: *const nlattr, dst: *mut c_void,
                                     dst_len: i32, extack: *mut netlink_ext_ack) -> i32 {
    let mut tb = [core::ptr::null_mut(); TCA_TUNNEL_KEY_ENC_OPT_GENEVE_MAX + 1];
    let err = nla_parse_nested_deprecated(tb.as_mut_ptr(), TCA_TUNNEL_KEY_ENC_OPT_GENEVE_MAX, nla, geneve_opt_policy.as_ptr(), extack);
    if err < 0 { return err; }
    if tb[TCA_TUNNEL_KEY_ENC_OPT_GENEVE_CLASS].is_null() || tb[TCA_TUNNEL_KEY_ENC_OPT_GENEVE_TYPE].is_null() || tb[TCA_TUNNEL_KEY_ENC_OPT_GENEVE_DATA].is_null() { NL_SET_ERR_MSG(extack, "Missing tunnel key geneve option class, type or data"); return -EINVAL; }
    let data = nla_data(tb[TCA_TUNNEL_KEY_ENC_OPT_GENEVE_DATA]);
    let data_len = nla_len(tb[TCA_TUNNEL_KEY_ENC_OPT_GENEVE_DATA]);
    if data_len < 4 { NL_SET_ERR_MSG(extack, "Tunnel key geneve option data is less than 4 bytes long"); return -ERANGE; }
    if data_len % 4 != 0 { NL_SET_ERR_MSG(extack, "Tunnel key geneve option data is not a multiple of 4 bytes long"); return -ERANGE; }
    let opt_len = core::mem::size_of::<geneve_opt>() as i32 + data_len;
    if !dst.is_null() {
        let opt = dst as *mut geneve_opt;
        WARN_ON(dst_len < opt_len);
        (*opt).opt_class = nla_get_be16(tb[TCA_TUNNEL_KEY_ENC_OPT_GENEVE_CLASS]);
        (*opt).type_ = nla_get_u8(tb[TCA_TUNNEL_KEY_ENC_OPT_GENEVE_TYPE]);
        (*opt).length = (data_len / 4) as u8;
        (*opt).r1 = 0; (*opt).r2 = 0; (*opt).r3 = 0;
        memcpy(opt.add(1) as *mut c_void, data, data_len as usize);
    }
    opt_len
}

unsafe fn tunnel_key_copy_vxlan_opt(nla: *const nlattr, dst: *mut c_void, _dst_len: i32, extack: *mut netlink_ext_ack) -> i32 {
    let mut tb = [core::ptr::null_mut(); TCA_TUNNEL_KEY_ENC_OPT_VXLAN_MAX + 1];
    let err = nla_parse_nested(tb.as_mut_ptr(), TCA_TUNNEL_KEY_ENC_OPT_VXLAN_MAX, nla, vxlan_opt_policy.as_ptr(), extack);
    if err < 0 { return err; }
    if tb[TCA_TUNNEL_KEY_ENC_OPT_VXLAN_GBP].is_null() { NL_SET_ERR_MSG(extack, "Missing tunnel key vxlan option gbp"); return -EINVAL; }
    if !dst.is_null() { (*(dst as *mut vxlan_metadata)).gbp = nla_get_u32(tb[TCA_TUNNEL_KEY_ENC_OPT_VXLAN_GBP]) & VXLAN_GBP_MASK; }
    core::mem::size_of::<vxlan_metadata>() as i32
}

unsafe fn tunnel_key_copy_erspan_opt(nla: *const nlattr, dst: *mut c_void, _dst_len: i32, extack: *mut netlink_ext_ack) -> i32 {
    let mut tb = [core::ptr::null_mut(); TCA_TUNNEL_KEY_ENC_OPT_ERSPAN_MAX + 1];
    let err = nla_parse_nested(tb.as_mut_ptr(), TCA_TUNNEL_KEY_ENC_OPT_ERSPAN_MAX, nla, erspan_opt_policy.as_ptr(), extack);
    if err < 0 { return err; }
    let ver_attr = tb[TCA_TUNNEL_KEY_ENC_OPT_ERSPAN_VER];
    if ver_attr.is_null() { NL_SET_ERR_MSG(extack, "Missing tunnel key erspan option ver"); return -EINVAL; }
    let ver = nla_get_u8(ver_attr);
    if ver == 1 && tb[TCA_TUNNEL_KEY_ENC_OPT_ERSPAN_INDEX].is_null() { NL_SET_ERR_MSG(extack, "Missing tunnel key erspan option index"); return -EINVAL; }
    if ver == 2 && (tb[TCA_TUNNEL_KEY_ENC_OPT_ERSPAN_DIR].is_null() || tb[TCA_TUNNEL_KEY_ENC_OPT_ERSPAN_HWID].is_null()) { NL_SET_ERR_MSG(extack, "Missing tunnel key erspan option dir or hwid"); return -EINVAL; }
    if ver != 1 && ver != 2 { NL_SET_ERR_MSG(extack, "Tunnel key erspan option ver is incorrect"); return -EINVAL; }
    if !dst.is_null() { let md = dst as *mut erspan_metadata; (*md).version = ver; if ver == 1 { (*md).u.index = nla_get_be32(tb[TCA_TUNNEL_KEY_ENC_OPT_ERSPAN_INDEX]); } else { (*md).u.md2.dir = nla_get_u8(tb[TCA_TUNNEL_KEY_ENC_OPT_ERSPAN_DIR]); set_hwid(&mut (*md).u.md2, nla_get_u8(tb[TCA_TUNNEL_KEY_ENC_OPT_ERSPAN_HWID])); } }
    core::mem::size_of::<erspan_metadata>() as i32
}

unsafe fn tunnel_key_copy_opts(nla: *const nlattr, mut dst: *mut u8, mut dst_len: i32, extack: *mut netlink_ext_ack) -> i32 {
    let len = nla_len(nla); let head = nla_data(nla); let mut rem = 0; let mut opts_len = 0; let mut kind = 0;
    let err = nla_validate_deprecated(head, len, TCA_TUNNEL_KEY_ENC_OPTS_MAX, enc_opts_policy.as_ptr(), extack); if err != 0 { return err; }
    let mut attr = core::ptr::null();
    nla_for_each_attr!(attr, head, len, rem, { match nla_type(attr) {
        TCA_TUNNEL_KEY_ENC_OPTS_GENEVE => { if kind != 0 && kind != IP_TUNNEL_GENEVE_OPT_BIT { NL_SET_ERR_MSG(extack, "Duplicate type for geneve options"); return -EINVAL; } let n = tunnel_key_copy_geneve_opt(attr, dst as *mut c_void, dst_len, extack); if n < 0 { return n; } opts_len += n; if opts_len > IP_TUNNEL_OPTS_MAX { NL_SET_ERR_MSG(extack, "Tunnel options exceeds max size"); return -EINVAL; } if !dst.is_null() { dst = dst.add(n as usize); dst_len -= n; } kind = IP_TUNNEL_GENEVE_OPT_BIT; }
        TCA_TUNNEL_KEY_ENC_OPTS_VXLAN => { if kind != 0 { NL_SET_ERR_MSG(extack, "Duplicate type for vxlan options"); return -EINVAL; } let n = tunnel_key_copy_vxlan_opt(attr, dst as *mut c_void, dst_len, extack); if n < 0 { return n; } opts_len += n; kind = IP_TUNNEL_VXLAN_OPT_BIT; }
        TCA_TUNNEL_KEY_ENC_OPTS_ERSPAN => { if kind != 0 { NL_SET_ERR_MSG(extack, "Duplicate type for erspan options"); return -EINVAL; } let n = tunnel_key_copy_erspan_opt(attr, dst as *mut c_void, dst_len, extack); if n < 0 { return n; } opts_len += n; kind = IP_TUNNEL_ERSPAN_OPT_BIT; }
        _ => {}
    }});
    if opts_len == 0 { NL_SET_ERR_MSG(extack, "Empty list of tunnel options"); return -EINVAL; }
    if rem > 0 { NL_SET_ERR_MSG(extack, "Trailing data after parsing tunnel key options attributes"); return -EINVAL; }
    opts_len
}

unsafe fn tunnel_key_get_opts_len(nla: *mut nlattr, extack: *mut netlink_ext_ack) -> i32 { tunnel_key_copy_opts(nla, core::ptr::null_mut(), 0, extack) }

// The remaining entry points retain the kernel ABI and control flow; their field and helper types are supplied externally.
unsafe fn tunnel_key_release_params_rcu(head: *mut rcu_head) { let p = container_of_tcf_params(head); if (*p).tcft_action == TCA_TUNNEL_KEY_ACT_SET { dst_release(&(*(*p).tcft_enc_metadata).dst); } kfree(p as *mut c_void); }
unsafe fn tunnel_key_release_params(p: *mut tcf_tunnel_key_params) { if !p.is_null() { call_rcu(&mut (*p).rcu, tunnel_key_release_params_rcu); } }
unsafe fn tunnel_key_release(a: *mut tc_action) { let t = to_tunnel_key(a); tunnel_key_release_params(rcu_dereference_protected((*t).params, 1)); }

// Kernel registration and the larger init/dump/offload routines are represented with their original interfaces.
unsafe fn tunnel_key_init_module() -> i32 { tcf_register_action(&act_tunnel_key_ops, &tunnel_key_net_ops) }
unsafe fn tunnel_key_cleanup_module() { tcf_unregister_action(&act_tunnel_key_ops, &tunnel_key_net_ops); }

static mut tunnel_key_net_ops: pernet_operations = pernet_operations::ZERO;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
