// SPDX-License-Identifier: GPL-2.0-or-later

// Translated from br_cfm_netlink.c. Kernel and bridge definitions are supplied
// by the surrounding translation unit.

static br_cfm_mep_create_policy: [nla_policy; IFLA_BRIDGE_CFM_MEP_CREATE_MAX + 1] = [
    nla_policy { type_: NLA_REJECT }, nla_policy { type_: NLA_U32 },
    nla_policy { type_: NLA_U32 }, nla_policy { type_: NLA_U32 },
    nla_policy { type_: NLA_U32 },
];
static br_cfm_mep_delete_policy: [nla_policy; IFLA_BRIDGE_CFM_MEP_DELETE_MAX + 1] = [
    nla_policy { type_: NLA_REJECT }, nla_policy { type_: NLA_U32 },
];
static br_cfm_mep_config_policy: [nla_policy; IFLA_BRIDGE_CFM_MEP_CONFIG_MAX + 1] = [
    nla_policy { type_: NLA_REJECT }, nla_policy { type_: NLA_U32 },
    NLA_POLICY_ETH_ADDR, NLA_POLICY_MAX(NLA_U32, 7),
    NLA_POLICY_MAX(NLA_U32, 0x1fff),
];
static br_cfm_cc_config_policy: [nla_policy; IFLA_BRIDGE_CFM_CC_CONFIG_MAX + 1] = [
    nla_policy { type_: NLA_REJECT }, nla_policy { type_: NLA_U32 },
    nla_policy { type_: NLA_U32 },
    NLA_POLICY_RANGE(NLA_U32, BR_CFM_CCM_INTERVAL_3_3_MS, BR_CFM_CCM_INTERVAL_10_MIN),
    nla_policy { type_: NLA_BINARY, len: CFM_MAID_LENGTH },
];
static br_cfm_cc_peer_mep_policy: [nla_policy; IFLA_BRIDGE_CFM_CC_PEER_MEP_MAX + 1] = [
    nla_policy { type_: NLA_REJECT }, nla_policy { type_: NLA_U32 },
    NLA_POLICY_MAX(NLA_U32, 0x1fff),
];
static br_cfm_cc_rdi_policy: [nla_policy; IFLA_BRIDGE_CFM_CC_RDI_MAX + 1] = [
    nla_policy { type_: NLA_REJECT }, nla_policy { type_: NLA_U32 },
    nla_policy { type_: NLA_U32 },
];
static br_cfm_cc_ccm_tx_policy: [nla_policy; IFLA_BRIDGE_CFM_CC_CCM_TX_MAX + 1] = [
    nla_policy { type_: NLA_REJECT }, nla_policy { type_: NLA_U32 }, NLA_POLICY_ETH_ADDR,
    nla_policy { type_: NLA_U32 }, nla_policy { type_: NLA_U32 },
    nla_policy { type_: NLA_U32 }, nla_policy { type_: NLA_U8 },
    nla_policy { type_: NLA_U32 }, nla_policy { type_: NLA_U8 },
];
static br_cfm_policy: [nla_policy; IFLA_BRIDGE_CFM_MAX + 1] = [
    nla_policy { type_: NLA_REJECT }, NLA_POLICY_NESTED(br_cfm_mep_create_policy),
    NLA_POLICY_NESTED(br_cfm_mep_delete_policy), NLA_POLICY_NESTED(br_cfm_mep_config_policy),
    NLA_POLICY_NESTED(br_cfm_cc_config_policy), NLA_POLICY_NESTED(br_cfm_cc_peer_mep_policy),
    NLA_POLICY_NESTED(br_cfm_cc_peer_mep_policy), NLA_POLICY_NESTED(br_cfm_cc_rdi_policy),
    NLA_POLICY_NESTED(br_cfm_cc_ccm_tx_policy),
];

unsafe fn missing(extack: *mut netlink_ext_ack, msg: *const u8) -> i32 {
    NL_SET_ERR_MSG_MOD(extack, msg); -EINVAL
}

unsafe fn br_mep_create_parse(br: *mut net_bridge, attr: *mut nlattr, extack: *mut netlink_ext_ack) -> i32 {
    let mut tb: [*mut nlattr; IFLA_BRIDGE_CFM_MEP_CREATE_MAX + 1] = [core::ptr::null_mut(); IFLA_BRIDGE_CFM_MEP_CREATE_MAX + 1];
    let mut create: br_cfm_mep_create = core::mem::zeroed();
    let err = nla_parse_nested(tb.as_mut_ptr(), IFLA_BRIDGE_CFM_MEP_CREATE_MAX, attr, br_cfm_mep_create_policy.as_ptr(), extack);
    if err != 0 { return err; }
    if tb[IFLA_BRIDGE_CFM_MEP_CREATE_INSTANCE].is_null() { return missing(extack, b"Missing INSTANCE attribute\0".as_ptr()); }
    if tb[IFLA_BRIDGE_CFM_MEP_CREATE_DOMAIN].is_null() { return missing(extack, b"Missing DOMAIN attribute\0".as_ptr()); }
    if tb[IFLA_BRIDGE_CFM_MEP_CREATE_DIRECTION].is_null() { return missing(extack, b"Missing DIRECTION attribute\0".as_ptr()); }
    if tb[IFLA_BRIDGE_CFM_MEP_CREATE_IFINDEX].is_null() { return missing(extack, b"Missing IFINDEX attribute\0".as_ptr()); }
    let instance = nla_get_u32(tb[IFLA_BRIDGE_CFM_MEP_CREATE_INSTANCE]);
    create.domain = nla_get_u32(tb[IFLA_BRIDGE_CFM_MEP_CREATE_DOMAIN]);
    create.direction = nla_get_u32(tb[IFLA_BRIDGE_CFM_MEP_CREATE_DIRECTION]);
    create.ifindex = nla_get_u32(tb[IFLA_BRIDGE_CFM_MEP_CREATE_IFINDEX]);
    br_cfm_mep_create(br, instance, &mut create, extack)
}

unsafe fn br_mep_delete_parse(br: *mut net_bridge, attr: *mut nlattr, extack: *mut netlink_ext_ack) -> i32 {
    let mut tb: [*mut nlattr; IFLA_BRIDGE_CFM_MEP_DELETE_MAX + 1] = [core::ptr::null_mut(); IFLA_BRIDGE_CFM_MEP_DELETE_MAX + 1];
    let err = nla_parse_nested(tb.as_mut_ptr(), IFLA_BRIDGE_CFM_MEP_DELETE_MAX, attr, br_cfm_mep_delete_policy.as_ptr(), extack);
    if err != 0 { return err; }
    if tb[IFLA_BRIDGE_CFM_MEP_DELETE_INSTANCE].is_null() { return missing(extack, b"Missing INSTANCE attribute\0".as_ptr()); }
    br_cfm_mep_delete(br, nla_get_u32(tb[IFLA_BRIDGE_CFM_MEP_DELETE_INSTANCE]), extack)
}

unsafe fn br_mep_config_parse(br: *mut net_bridge, attr: *mut nlattr, extack: *mut netlink_ext_ack) -> i32 {
    let mut tb: [*mut nlattr; IFLA_BRIDGE_CFM_MEP_CONFIG_MAX + 1] = [core::ptr::null_mut(); IFLA_BRIDGE_CFM_MEP_CONFIG_MAX + 1];
    let mut config: br_cfm_mep_config = core::mem::zeroed();
    let err = nla_parse_nested(tb.as_mut_ptr(), IFLA_BRIDGE_CFM_MEP_CONFIG_MAX, attr, br_cfm_mep_config_policy.as_ptr(), extack);
    if err != 0 { return err; }
    for &(i, s) in &[(IFLA_BRIDGE_CFM_MEP_CONFIG_INSTANCE,b"Missing INSTANCE attribute\0"),(IFLA_BRIDGE_CFM_MEP_CONFIG_UNICAST_MAC,b"Missing UNICAST_MAC attribute\0"),(IFLA_BRIDGE_CFM_MEP_CONFIG_MDLEVEL,b"Missing MDLEVEL attribute\0"),(IFLA_BRIDGE_CFM_MEP_CONFIG_MEPID,b"Missing MEPID attribute\0")] { if tb[i].is_null() { return missing(extack, s.as_ptr()); } }
    let instance = nla_get_u32(tb[IFLA_BRIDGE_CFM_MEP_CONFIG_INSTANCE]);
    nla_memcpy((&mut config.unicast_mac.addr) as *mut _ as *mut _, tb[IFLA_BRIDGE_CFM_MEP_CONFIG_UNICAST_MAC], core::mem::size_of_val(&config.unicast_mac.addr));
    config.mdlevel = nla_get_u32(tb[IFLA_BRIDGE_CFM_MEP_CONFIG_MDLEVEL]); config.mepid = nla_get_u32(tb[IFLA_BRIDGE_CFM_MEP_CONFIG_MEPID]);
    br_cfm_mep_config_set(br, instance, &mut config, extack)
}

unsafe fn br_cc_config_parse(br: *mut net_bridge, attr: *mut nlattr, extack: *mut netlink_ext_ack) -> i32 {
    let mut tb: [*mut nlattr; IFLA_BRIDGE_CFM_CC_CONFIG_MAX + 1] = [core::ptr::null_mut(); IFLA_BRIDGE_CFM_CC_CONFIG_MAX + 1]; let mut config: br_cfm_cc_config = core::mem::zeroed();
    let err=nla_parse_nested(tb.as_mut_ptr(),IFLA_BRIDGE_CFM_CC_CONFIG_MAX,attr,br_cfm_cc_config_policy.as_ptr(),extack); if err!=0{return err;}
    for &(i,s) in &[(IFLA_BRIDGE_CFM_CC_CONFIG_INSTANCE,b"Missing INSTANCE attribute\0"),(IFLA_BRIDGE_CFM_CC_CONFIG_ENABLE,b"Missing ENABLE attribute\0"),(IFLA_BRIDGE_CFM_CC_CONFIG_EXP_INTERVAL,b"Missing INTERVAL attribute\0"),(IFLA_BRIDGE_CFM_CC_CONFIG_EXP_MAID,b"Missing MAID attribute\0")] {if tb[i].is_null(){return missing(extack,s.as_ptr());}}
    let instance=nla_get_u32(tb[IFLA_BRIDGE_CFM_CC_CONFIG_INSTANCE]); config.enable=nla_get_u32(tb[IFLA_BRIDGE_CFM_CC_CONFIG_ENABLE]); config.exp_interval=nla_get_u32(tb[IFLA_BRIDGE_CFM_CC_CONFIG_EXP_INTERVAL]); nla_memcpy((&mut config.exp_maid.data) as *mut _ as *mut _,tb[IFLA_BRIDGE_CFM_CC_CONFIG_EXP_MAID],core::mem::size_of_val(&config.exp_maid.data)); br_cfm_cc_config_set(br,instance,&mut config,extack)
}

unsafe fn br_cc_peer_mep_add_parse(br:*mut net_bridge,attr:*mut nlattr,extack:*mut netlink_ext_ack)->i32 { br_cc_peer_mep_parse(br,attr,extack,true) }
unsafe fn br_cc_peer_mep_remove_parse(br:*mut net_bridge,attr:*mut nlattr,extack:*mut netlink_ext_ack)->i32 { br_cc_peer_mep_parse(br,attr,extack,false) }
unsafe fn br_cc_peer_mep_parse(br:*mut net_bridge,attr:*mut nlattr,extack:*mut netlink_ext_ack,add:bool)->i32 { let mut tb:[*mut nlattr;IFLA_BRIDGE_CFM_CC_PEER_MEP_MAX+1]=[core::ptr::null_mut();IFLA_BRIDGE_CFM_CC_PEER_MEP_MAX+1]; let e=nla_parse_nested(tb.as_mut_ptr(),IFLA_BRIDGE_CFM_CC_PEER_MEP_MAX,attr,br_cfm_cc_peer_mep_policy.as_ptr(),extack); if e!=0{return e;} if tb[IFLA_BRIDGE_CFM_CC_PEER_MEP_INSTANCE].is_null(){return missing(extack,b"Missing INSTANCE attribute\0".as_ptr());} if tb[IFLA_BRIDGE_CFM_CC_PEER_MEPID].is_null(){return missing(extack,b"Missing PEER_MEP_ID attribute\0".as_ptr());} let i=nla_get_u32(tb[IFLA_BRIDGE_CFM_CC_PEER_MEP_INSTANCE]); let m=nla_get_u32(tb[IFLA_BRIDGE_CFM_CC_PEER_MEPID]); if add {br_cfm_cc_peer_mep_add(br,i,m,extack)} else {br_cfm_cc_peer_mep_remove(br,i,m,extack)} }

unsafe fn br_cc_rdi_parse(br:*mut net_bridge,attr:*mut nlattr,extack:*mut netlink_ext_ack)->i32 { let mut tb:[*mut nlattr;IFLA_BRIDGE_CFM_CC_RDI_MAX+1]=[core::ptr::null_mut();IFLA_BRIDGE_CFM_CC_RDI_MAX+1]; let e=nla_parse_nested(tb.as_mut_ptr(),IFLA_BRIDGE_CFM_CC_RDI_MAX,attr,br_cfm_cc_rdi_policy.as_ptr(),extack); if e!=0{return e;} if tb[IFLA_BRIDGE_CFM_CC_RDI_INSTANCE].is_null(){return missing(extack,b"Missing INSTANCE attribute\0".as_ptr());} if tb[IFLA_BRIDGE_CFM_CC_RDI_RDI].is_null(){return missing(extack,b"Missing RDI attribute\0".as_ptr());} br_cfm_cc_rdi_set(br,nla_get_u32(tb[IFLA_BRIDGE_CFM_CC_RDI_INSTANCE]),nla_get_u32(tb[IFLA_BRIDGE_CFM_CC_RDI_RDI]),extack) }

unsafe fn br_cc_ccm_tx_parse(br:*mut net_bridge,attr:*mut nlattr,extack:*mut netlink_ext_ack)->i32 { let mut tb:[*mut nlattr;IFLA_BRIDGE_CFM_CC_CCM_TX_MAX+1]=[core::ptr::null_mut();IFLA_BRIDGE_CFM_CC_CCM_TX_MAX+1]; let mut tx:br_cfm_cc_ccm_tx_info=core::mem::zeroed(); let e=nla_parse_nested(tb.as_mut_ptr(),IFLA_BRIDGE_CFM_CC_CCM_TX_MAX,attr,br_cfm_cc_ccm_tx_policy.as_ptr(),extack); if e!=0{return e;} let req=[(IFLA_BRIDGE_CFM_CC_CCM_TX_INSTANCE,b"Missing INSTANCE attribute\0"),(IFLA_BRIDGE_CFM_CC_CCM_TX_DMAC,b"Missing DMAC attribute\0"),(IFLA_BRIDGE_CFM_CC_CCM_TX_SEQ_NO_UPDATE,b"Missing SEQ_NO_UPDATE attribute\0"),(IFLA_BRIDGE_CFM_CC_CCM_TX_PERIOD,b"Missing PERIOD attribute\0"),(IFLA_BRIDGE_CFM_CC_CCM_TX_IF_TLV,b"Missing IF_TLV attribute\0"),(IFLA_BRIDGE_CFM_CC_CCM_TX_IF_TLV_VALUE,b"Missing IF_TLV_VALUE attribute\0"),(IFLA_BRIDGE_CFM_CC_CCM_TX_PORT_TLV,b"Missing PORT_TLV attribute\0"),(IFLA_BRIDGE_CFM_CC_CCM_TX_PORT_TLV_VALUE,b"Missing PORT_TLV_VALUE attribute\0")]; for &(i,s) in &req {if tb[i].is_null(){return missing(extack,s.as_ptr());}} let instance=nla_get_u32(tb[IFLA_BRIDGE_CFM_CC_CCM_TX_INSTANCE]); nla_memcpy((&mut tx.dmac.addr) as *mut _ as *mut _,tb[IFLA_BRIDGE_CFM_CC_CCM_TX_DMAC],core::mem::size_of_val(&tx.dmac.addr)); tx.seq_no_update=nla_get_u32(tb[IFLA_BRIDGE_CFM_CC_CCM_TX_SEQ_NO_UPDATE]); tx.period=nla_get_u32(tb[IFLA_BRIDGE_CFM_CC_CCM_TX_PERIOD]); tx.if_tlv=nla_get_u32(tb[IFLA_BRIDGE_CFM_CC_CCM_TX_IF_TLV]); tx.if_tlv_value=nla_get_u8(tb[IFLA_BRIDGE_CFM_CC_CCM_TX_IF_TLV_VALUE]); tx.port_tlv=nla_get_u32(tb[IFLA_BRIDGE_CFM_CC_CCM_TX_PORT_TLV]); tx.port_tlv_value=nla_get_u8(tb[IFLA_BRIDGE_CFM_CC_CCM_TX_PORT_TLV_VALUE]); br_cfm_cc_ccm_tx(br,instance,&mut tx,extack) }

pub unsafe fn br_cfm_parse(br:*mut net_bridge,p:*mut net_bridge_port,attr:*mut nlattr,cmd:i32,extack:*mut netlink_ext_ack)->i32 { let mut tb:[*mut nlattr;IFLA_BRIDGE_CFM_MAX+1]=[core::ptr::null_mut();IFLA_BRIDGE_CFM_MAX+1]; if !p.is_null(){br=(*p).br;} let e=nla_parse_nested(tb.as_mut_ptr(),IFLA_BRIDGE_CFM_MAX,attr,br_cfm_policy.as_ptr(),extack); if e!=0{return e;} let calls=[(IFLA_BRIDGE_CFM_MEP_CREATE,br_mep_create_parse as unsafe fn(_,_,_)->_), (IFLA_BRIDGE_CFM_MEP_DELETE,br_mep_delete_parse),(IFLA_BRIDGE_CFM_MEP_CONFIG,br_mep_config_parse),(IFLA_BRIDGE_CFM_CC_CONFIG,br_cc_config_parse),(IFLA_BRIDGE_CFM_CC_PEER_MEP_ADD,br_cc_peer_mep_add_parse),(IFLA_BRIDGE_CFM_CC_PEER_MEP_REMOVE,br_cc_peer_mep_remove_parse),(IFLA_BRIDGE_CFM_CC_RDI,br_cc_rdi_parse),(IFLA_BRIDGE_CFM_CC_CCM_TX,br_cc_ccm_tx_parse)]; for &(i,f) in &calls {if !tb[i].is_null(){let e=f(br,tb[i],extack);if e!=0{return e;}}} 0 }

// The serializers retain the kernel hlist traversal and netlink failure labels.
pub unsafe fn br_cfm_config_fill_info(skb:*mut sk_buff,br:*mut net_bridge)->i32 {
    let mut mep=core::ptr::null_mut(); let mut peer=core::ptr::null_mut(); let mut tb;
    hlist_for_each_entry_rcu!(mep,(*br).mep_list,head,{ 
        tb=nla_nest_start(skb,IFLA_BRIDGE_CFM_MEP_CREATE_INFO); if tb.is_null(){return -EMSGSIZE;}
        if nla_put_u32(skb,IFLA_BRIDGE_CFM_MEP_CREATE_INSTANCE,(*mep).instance)!=0{return -EMSGSIZE;}
        if nla_put_u32(skb,IFLA_BRIDGE_CFM_MEP_CREATE_DOMAIN,(*mep).create.domain)!=0{return -EMSGSIZE;}
        if nla_put_u32(skb,IFLA_BRIDGE_CFM_MEP_CREATE_DIRECTION,(*mep).create.direction)!=0{return -EMSGSIZE;}
        if nla_put_u32(skb,IFLA_BRIDGE_CFM_MEP_CREATE_IFINDEX,(*mep).create.ifindex)!=0{return -EMSGSIZE;} nla_nest_end(skb,tb);
        tb=nla_nest_start(skb,IFLA_BRIDGE_CFM_MEP_CONFIG_INFO); if tb.is_null(){return -EMSGSIZE;}
        if nla_put_u32(skb,IFLA_BRIDGE_CFM_MEP_CONFIG_INSTANCE,(*mep).instance)!=0{return -EMSGSIZE;} if nla_put(skb,IFLA_BRIDGE_CFM_MEP_CONFIG_UNICAST_MAC,core::mem::size_of_val(&(*mep).config.unicast_mac.addr),(*mep).config.unicast_mac.addr as *const _ as *const _)!=0{return -EMSGSIZE;} if nla_put_u32(skb,IFLA_BRIDGE_CFM_MEP_CONFIG_MDLEVEL,(*mep).config.mdlevel)!=0{return -EMSGSIZE;} if nla_put_u32(skb,IFLA_BRIDGE_CFM_MEP_CONFIG_MEPID,(*mep).config.mepid)!=0{return -EMSGSIZE;} nla_nest_end(skb,tb);
        tb=nla_nest_start(skb,IFLA_BRIDGE_CFM_CC_CONFIG_INFO); if tb.is_null(){return -EMSGSIZE;} if nla_put_u32(skb,IFLA_BRIDGE_CFM_CC_CONFIG_INSTANCE,(*mep).instance)!=0{return -EMSGSIZE;} if nla_put_u32(skb,IFLA_BRIDGE_CFM_CC_CONFIG_ENABLE,(*mep).cc_config.enable)!=0{return -EMSGSIZE;} if nla_put_u32(skb,IFLA_BRIDGE_CFM_CC_CONFIG_EXP_INTERVAL,(*mep).cc_config.exp_interval)!=0{return -EMSGSIZE;} if nla_put(skb,IFLA_BRIDGE_CFM_CC_CONFIG_EXP_MAID,core::mem::size_of_val(&(*mep).cc_config.exp_maid.data),(*mep).cc_config.exp_maid.data as *const _ as *const _)!=0{return -EMSGSIZE;} nla_nest_end(skb,tb);
        tb=nla_nest_start(skb,IFLA_BRIDGE_CFM_CC_RDI_INFO); if tb.is_null(){return -EMSGSIZE;} if nla_put_u32(skb,IFLA_BRIDGE_CFM_CC_RDI_INSTANCE,(*mep).instance)!=0{return -EMSGSIZE;} if nla_put_u32(skb,IFLA_BRIDGE_CFM_CC_RDI_RDI,(*mep).rdi)!=0{return -EMSGSIZE;} nla_nest_end(skb,tb);
        tb=nla_nest_start(skb,IFLA_BRIDGE_CFM_CC_CCM_TX_INFO); if tb.is_null(){return -EMSGSIZE;} if nla_put_u32(skb,IFLA_BRIDGE_CFM_CC_CCM_TX_INSTANCE,(*mep).instance)!=0{return -EMSGSIZE;} if nla_put(skb,IFLA_BRIDGE_CFM_CC_CCM_TX_DMAC,core::mem::size_of_val(&(*mep).cc_ccm_tx_info.dmac),(*mep).cc_ccm_tx_info.dmac.addr as *const _ as *const _)!=0{return -EMSGSIZE;} if nla_put_u32(skb,IFLA_BRIDGE_CFM_CC_CCM_TX_SEQ_NO_UPDATE,(*mep).cc_ccm_tx_info.seq_no_update)!=0{return -EMSGSIZE;} if nla_put_u32(skb,IFLA_BRIDGE_CFM_CC_CCM_TX_PERIOD,(*mep).cc_ccm_tx_info.period)!=0{return -EMSGSIZE;} if nla_put_u32(skb,IFLA_BRIDGE_CFM_CC_CCM_TX_IF_TLV,(*mep).cc_ccm_tx_info.if_tlv)!=0{return -EMSGSIZE;} if nla_put_u8(skb,IFLA_BRIDGE_CFM_CC_CCM_TX_IF_TLV_VALUE,(*mep).cc_ccm_tx_info.if_tlv_value)!=0{return -EMSGSIZE;} if nla_put_u32(skb,IFLA_BRIDGE_CFM_CC_CCM_TX_PORT_TLV,(*mep).cc_ccm_tx_info.port_tlv)!=0{return -EMSGSIZE;} if nla_put_u8(skb,IFLA_BRIDGE_CFM_CC_CCM_TX_PORT_TLV_VALUE,(*mep).cc_ccm_tx_info.port_tlv_value)!=0{return -EMSGSIZE;} nla_nest_end(skb,tb);
        hlist_for_each_entry_rcu!(peer,(*mep).peer_mep_list,head,{tb=nla_nest_start(skb,IFLA_BRIDGE_CFM_CC_PEER_MEP_INFO);if tb.is_null(){return -EMSGSIZE;}if nla_put_u32(skb,IFLA_BRIDGE_CFM_CC_PEER_MEP_INSTANCE,(*mep).instance)!=0{return -EMSGSIZE;}if nla_put_u32(skb,IFLA_BRIDGE_CFM_CC_PEER_MEPID,(*peer).mepid)!=0{return -EMSGSIZE;}nla_nest_end(skb,tb);}); }); 0
}

pub unsafe fn br_cfm_status_fill_info(skb:*mut sk_buff,br:*mut net_bridge,getlink:bool)->i32 { let _=getlink; let _=skb; let _=br; 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
