/* SPDX-License-Identifier: GPL-2.0 */
// C dependencies and build-time kernel configuration are supplied externally.

unsafe fn nft_flow_rule_alloc(num_actions: i32) -> *mut nft_flow_rule {
    let flow = kzalloc_obj::<nft_flow_rule>();
    if flow.is_null() { return core::ptr::null_mut(); }
    (*flow).rule = flow_rule_alloc(num_actions);
    if (*flow).rule.is_null() { kfree(flow); return core::ptr::null_mut(); }
    (*flow).rule.as_mut().unwrap().r#match.dissector = &mut (*flow).r#match.dissector;
    (*flow).rule.as_mut().unwrap().r#match.mask = &mut (*flow).r#match.mask;
    (*flow).rule.as_mut().unwrap().r#match.key = &mut (*flow).r#match.key;
    flow
}

pub unsafe fn nft_flow_rule_set_addr_type(flow: *mut nft_flow_rule, addr_type: flow_dissector_key_id) {
    let m = &mut (*flow).r#match;
    let mask = &mut m.mask; let key = &mut m.key;
    if m.dissector.used_keys & BIT_ULL(FLOW_DISSECTOR_KEY_CONTROL) != 0 { return; }
    key.control.addr_type = addr_type; mask.control.addr_type = 0xffff;
    m.dissector.used_keys |= BIT_ULL(FLOW_DISSECTOR_KEY_CONTROL);
    m.dissector.offset[FLOW_DISSECTOR_KEY_CONTROL] = core::mem::offset_of!(nft_flow_key, control);
}

#[repr(C)] struct nft_offload_ethertype { value: __be16, mask: __be16 }

unsafe fn nft_flow_rule_transfer_vlan(_ctx: *mut nft_offload_ctx, flow: *mut nft_flow_rule) {
    let m = &mut (*flow).r#match;
    let eth = nft_offload_ethertype { value: m.key.basic.n_proto, mask: m.mask.basic.n_proto };
    if m.dissector.used_keys & BIT_ULL(FLOW_DISSECTOR_KEY_VLAN) != 0 &&
       (m.key.vlan.vlan_tpid == htons(ETH_P_8021Q) || m.key.vlan.vlan_tpid == htons(ETH_P_8021AD)) {
        m.key.basic.n_proto=m.key.cvlan.vlan_tpid; m.mask.basic.n_proto=m.mask.cvlan.vlan_tpid;
        m.key.cvlan.vlan_tpid=m.key.vlan.vlan_tpid; m.mask.cvlan.vlan_tpid=m.mask.vlan.vlan_tpid;
        m.key.vlan.vlan_tpid=eth.value; m.mask.vlan.vlan_tpid=eth.mask;
        m.dissector.offset[FLOW_DISSECTOR_KEY_CVLAN]=core::mem::offset_of!(nft_flow_key,cvlan);
        m.dissector.used_keys |= BIT_ULL(FLOW_DISSECTOR_KEY_CVLAN);
    } else if m.dissector.used_keys & BIT_ULL(FLOW_DISSECTOR_KEY_BASIC) != 0 &&
       (m.key.basic.n_proto == htons(ETH_P_8021Q) || m.key.basic.n_proto == htons(ETH_P_8021AD)) {
        m.key.basic.n_proto=m.key.vlan.vlan_tpid; m.mask.basic.n_proto=m.mask.vlan.vlan_tpid;
        m.key.vlan.vlan_tpid=eth.value; m.mask.vlan.vlan_tpid=eth.mask;
        m.dissector.offset[FLOW_DISSECTOR_KEY_VLAN]=core::mem::offset_of!(nft_flow_key,vlan);
        m.dissector.used_keys |= BIT_ULL(FLOW_DISSECTOR_KEY_VLAN);
    }
}

pub unsafe fn nft_flow_rule_create(net: *mut net, rule: *const nft_rule) -> *mut nft_flow_rule {
    let mut n=0; let mut expr=nft_expr_first(rule);
    while nft_expr_more(rule,expr) { if (*(*expr).ops).offload_action.is_some() && ((*(*expr).ops).offload_action.unwrap())(expr)!=0 { n+=1; } expr=nft_expr_next(expr); }
    if n==0 { return ERR_PTR(-EOPNOTSUPP); }
    let flow=nft_flow_rule_alloc(n); if flow.is_null() { return ERR_PTR(-ENOMEM); }
    expr=nft_expr_first(rule); let ctx=kzalloc_obj::<nft_offload_ctx>();
    if ctx.is_null() { nft_flow_rule_destroy(flow); return ERR_PTR(-ENOMEM); }
    (*ctx).net=net; (*ctx).dep.r#type=NFT_OFFLOAD_DEP_UNSPEC;
    while nft_expr_more(rule,expr) { if (*expr).ops.offload.is_none() { kfree(ctx); nft_flow_rule_destroy(flow); return ERR_PTR(-EOPNOTSUPP); }
        let err=((*expr).ops.offload.unwrap())(ctx,flow,expr); if err<0 { kfree(ctx); nft_flow_rule_destroy(flow); return ERR_PTR(err); } expr=nft_expr_next(expr); }
    nft_flow_rule_transfer_vlan(ctx,flow); (*flow).proto=(*ctx).dep.l3num; kfree(ctx); flow
}

pub unsafe fn nft_flow_rule_destroy(flow:*mut nft_flow_rule) { let mut entry: *mut flow_action_entry=core::ptr::null_mut(); let mut i=0; flow_action_for_each(i,entry,&(*(*flow).rule).action) { match (*entry).id { FLOW_ACTION_REDIRECT|FLOW_ACTION_MIRRED=>dev_put((*entry).dev), _=>{} } } kfree((*flow).rule); kfree(flow); }
pub unsafe fn nft_offload_set_dependency(ctx:*mut nft_offload_ctx, t:nft_offload_dep_type){(*ctx).dep.r#type=t;}
pub unsafe fn nft_offload_update_dependency(ctx:*mut nft_offload_ctx,data:*const core::ffi::c_void,len:u32){match (*ctx).dep.r#type { NFT_OFFLOAD_DEP_NETWORK=>{WARN_ON(len!=core::mem::size_of::<__u16>() as u32); core::ptr::copy_nonoverlapping(data as *const u8,&mut (*ctx).dep.l3num as *mut _ as *mut u8,2);}, NFT_OFFLOAD_DEP_TRANSPORT=>{WARN_ON(len!=1);core::ptr::copy_nonoverlapping(data as *const u8,&mut (*ctx).dep.protonum as *mut _ as *mut u8,1);}, _=>{}} (*ctx).dep.r#type=NFT_OFFLOAD_DEP_UNSPEC;}

unsafe fn nft_flow_offload_common_init(c:*mut flow_cls_common_offload,p:__be16,prio:i32,e:*mut netlink_ext_ack){(*c).protocol=p;(*c).prio=prio;(*c).extack=e;}
unsafe fn nft_setup_cb_call(t:tc_setup_type,d:*mut core::ffi::c_void,l:*mut list_head)->i32{let mut b:*mut flow_block_cb;list_for_each_entry!(b,l,list){let e=((*b).cb)(t,d,(*b).cb_priv);if e<0{return e;}}0}
unsafe fn nft_chain_offload_priority(b:*const nft_base_chain)->i32{if (*b).ops.priority<=0||(*b).ops.priority>USHRT_MAX as i32{-1}else{0}}
pub unsafe fn nft_chain_offload_support(b:*const nft_base_chain)->bool{if nft_chain_offload_priority(b)<0{return false;}let mut h:*mut nft_hook;let mut o:*mut nf_hook_ops;list_for_each_entry!(h,&(*b).hook_list,list){list_for_each_entry!(o,&(*h).ops_list,list){if (*o).pf!=NFPROTO_NETDEV||(*o).hooknum!=NF_NETDEV_INGRESS{return false;}let d=(*o).dev;if (*(*d).netdev_ops).ndo_setup_tc.is_none()&&!flow_indr_dev_exists(){return false;}}}true}

unsafe fn nft_flow_cls_offload_setup(c:*mut flow_cls_offload,b:*const nft_base_chain,r:*const nft_rule,f:*const nft_flow_rule,e:*mut netlink_ext_ack,cmd:flow_cls_command){core::ptr::write_bytes(c as *mut u8,0,core::mem::size_of::<flow_cls_offload>());let mut p=ETH_P_ALL;if !f.is_null(){p=(*f).proto;}nft_flow_offload_common_init(&mut (*c).common,p,(*b).ops.priority,e);(*c).command=cmd;(*c).cookie=r as usize; if !f.is_null(){(*c).rule=(*f).rule;}}
unsafe fn nft_flow_offload_cmd(ch:*const nft_chain,r:*const nft_rule,f:*mut nft_flow_rule,cmd:flow_cls_command,c:*mut flow_cls_offload)->i32{if !nft_is_base_chain(ch){return -EOPNOTSUPP;}let b=nft_base_chain(ch);let mut e=core::mem::zeroed();nft_flow_cls_offload_setup(c,b,r,f,&mut e,cmd);nft_setup_cb_call(TC_SETUP_CLSFLOWER,c,&mut (*b).flow_block.cb_list)}
unsafe fn nft_flow_offload_rule(ch:*const nft_chain,r:*mut nft_rule,f:*mut nft_flow_rule,cmd:flow_cls_command)->i32{let mut c=core::mem::zeroed();nft_flow_offload_cmd(ch,r,f,cmd,&mut c)}
pub unsafe fn nft_flow_rule_stats(ch:*const nft_chain,r:*const nft_rule)->i32{let mut c=core::mem::zeroed();let e=nft_flow_offload_cmd(ch,r,core::ptr::null_mut(),FLOW_CLS_STATS,&mut c);if e<0{return e;}let(mut x,mut next)=(nft_expr_first(r),core::ptr::null_mut());nft_rule_for_each_expr!(x,next,r){if (*x).ops.offload_stats.is_some(){((*x).ops.offload_stats.unwrap())(x,&mut c.stats);}}0}
unsafe fn nft_flow_offload_bind(bo:*mut flow_block_offload,b:*mut nft_base_chain)->i32{list_splice(&mut (*bo).cb_list,&mut (*b).flow_block.cb_list);0}
unsafe fn nft_flow_offload_unbind(bo:*mut flow_block_offload,b:*mut nft_base_chain)->i32{let ch=&mut (*b).chain;let mut r:*mut nft_rule;list_for_each_entry!(r,&ch.rules,list){let mut e=core::mem::zeroed();let mut c=core::mem::zeroed();nft_flow_cls_offload_setup(&mut c,b,r,core::ptr::null(),&mut e,FLOW_CLS_DESTROY);nft_setup_cb_call(TC_SETUP_CLSFLOWER,&mut c,&mut (*bo).cb_list);}let(mut x,mut next)=(core::ptr::null_mut(),core::ptr::null_mut());list_for_each_entry_safe!(x,next,&mut (*bo).cb_list,list){list_del(&mut (*x).list);flow_block_cb_free(x);}0}
unsafe fn nft_block_setup(b:*mut nft_base_chain,bo:*mut flow_block_offload,cmd:flow_block_command)->i32{match cmd{FLOW_BLOCK_BIND=>nft_flow_offload_bind(bo,b),FLOW_BLOCK_UNBIND=>nft_flow_offload_unbind(bo,b),_=>{DEBUG_NET_WARN_ON_ONCE(1);-EOPNOTSUPP}}}
unsafe fn nft_flow_block_offload_init(bo:*mut flow_block_offload,net:*mut net,cmd:flow_block_command,b:*mut nft_base_chain,e:*mut netlink_ext_ack){core::ptr::write_bytes(bo as *mut u8,0,core::mem::size_of::<flow_block_offload>());(*bo).net=net;(*bo).block=&mut (*b).flow_block;(*bo).command=cmd;(*bo).binder_type=FLOW_BLOCK_BINDER_TYPE_CLSACT_INGRESS;(*bo).extack=e;(*bo).cb_list_head=&mut (*b).flow_block.cb_list;INIT_LIST_HEAD(&mut (*bo).cb_list);}
unsafe fn nft_block_offload_cmd(b:*mut nft_base_chain,d:*mut net_device,cmd:flow_block_command)->i32{let mut e=core::mem::zeroed();let mut bo=core::mem::zeroed();nft_flow_block_offload_init(&mut bo,dev_net(d),cmd,b,&mut e);let x=((*(*d).netdev_ops).ndo_setup_tc.unwrap())(d,TC_SETUP_BLOCK,&mut bo);if x<0{x}else{nft_block_setup(b,&mut bo,cmd)}}
unsafe fn nft_chain_offload_cmd(b:*mut nft_base_chain,d:*mut net_device,cmd:flow_block_command)->i32{if (*(*d).netdev_ops).ndo_setup_tc.is_some(){nft_block_offload_cmd(b,d,cmd)}else{-EOPNOTSUPP}}
pub unsafe fn nft_flow_rule_offload_commit(_net:*mut net)->i32{0}
pub unsafe fn nft_offload_init()->i32{register_netdevice_notifier(&mut nft_offload_netdev_notifier)}
pub unsafe fn nft_offload_exit(){unregister_netdevice_notifier(&mut nft_offload_netdev_notifier);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
