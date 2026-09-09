// SPDX-License-Identifier: GPL-2.0-or-later
/* net/sched/ife.c - Inter-FE action based on ForCES WG InterFE LFB */

/* Kernel dependencies are supplied by the surrounding Rust kernel bindings. */

static mut MAX_METACNT: i32 = IFE_META_MAX + 1;
static mut ACT_IFE_OPS: tc_action_ops = tc_action_ops { };

static IFE_POLICY: [nla_policy; TCA_IFE_MAX as usize + 1] = [nla_policy { } ; TCA_IFE_MAX as usize + 1];

#[no_mangle]
pub unsafe extern "C" fn ife_encode_meta_u16(metaval: u16, skbdata: *mut c_void, mi: *mut tcf_meta_info) -> i32 {
    let mut edata: u16 = 0;
    if !(*mi).metaval.is_null() { edata = *( (*mi).metaval as *const u16); }
    else if metaval != 0 { edata = metaval; }
    if edata == 0 { return 0; }
    edata = htons(edata);
    ife_tlv_meta_encode(skbdata, (*mi).metaid, 2, &mut edata as *mut _ as *mut c_void)
}

#[no_mangle] pub unsafe extern "C" fn ife_get_meta_u32(skb: *mut sk_buff, mi: *mut tcf_meta_info) -> i32 {
    if !(*mi).metaval.is_null() { nla_put_u32(skb, (*mi).metaid, *((*mi).metaval as *const u32)) }
    else { nla_put(skb, (*mi).metaid, 0, core::ptr::null()) }
}
#[no_mangle] pub unsafe extern "C" fn ife_check_meta_u32(metaval: u32, mi: *mut tcf_meta_info) -> i32 { if metaval != 0 || !(*mi).metaval.is_null() { 8 } else { 0 } }
#[no_mangle] pub unsafe extern "C" fn ife_check_meta_u16(metaval: u16, mi: *mut tcf_meta_info) -> i32 { if metaval != 0 || !(*mi).metaval.is_null() { 8 } else { 0 } }
#[no_mangle] pub unsafe extern "C" fn ife_encode_meta_u32(metaval: u32, skbdata: *mut c_void, mi: *mut tcf_meta_info) -> i32 {
    let mut edata = if !(*mi).metaval.is_null() { *((*mi).metaval as *const u32) } else { metaval };
    if edata == 0 { return 0; }
    edata = htonl(edata);
    ife_tlv_meta_encode(skbdata, (*mi).metaid, 4, &mut edata as *mut _ as *mut c_void)
}
#[no_mangle] pub unsafe extern "C" fn ife_get_meta_u16(skb: *mut sk_buff, mi: *mut tcf_meta_info) -> i32 {
    if !(*mi).metaval.is_null() { nla_put_u16(skb, (*mi).metaid, *((*mi).metaval as *const u16)) }
    else { nla_put(skb, (*mi).metaid, 0, core::ptr::null()) }
}
#[no_mangle] pub unsafe extern "C" fn ife_alloc_meta_u32(mi: *mut tcf_meta_info, metaval: *mut c_void, gfp: gfp_t) -> i32 {
    (*mi).metaval = kmemdup(metaval, core::mem::size_of::<u32>(), gfp);
    if (*mi).metaval.is_null() { -ENOMEM } else { 0 }
}
#[no_mangle] pub unsafe extern "C" fn ife_alloc_meta_u16(mi: *mut tcf_meta_info, metaval: *mut c_void, gfp: gfp_t) -> i32 {
    (*mi).metaval = kmemdup(metaval, core::mem::size_of::<u16>(), gfp);
    if (*mi).metaval.is_null() { -ENOMEM } else { 0 }
}
#[no_mangle] pub unsafe extern "C" fn ife_release_meta_gen(mi: *mut tcf_meta_info) { kfree((*mi).metaval); }
#[no_mangle] pub unsafe extern "C" fn ife_validate_meta_u32(_val: *mut c_void, len: i32) -> i32 { if len == core::mem::size_of::<u32>() as i32 { 0 } else { -EINVAL } }
#[no_mangle] pub unsafe extern "C" fn ife_validate_meta_u16(_val: *mut c_void, len: i32) -> i32 { if len == core::mem::size_of::<u16>() as i32 { 0 } else { -EINVAL } }

static mut IFEOPLIST: list_head = list_head { };
static mut IFE_MOD_LOCK: rwlock_t = rwlock_t { };

unsafe fn find_ife_oplist(metaid: u16) -> *mut tcf_meta_ops {
    read_lock(&mut IFE_MOD_LOCK);
    let mut o: *mut tcf_meta_ops = core::ptr::null_mut();
    list_for_each_entry(&mut o, &mut IFEOPLIST, list, {
        if (*o).metaid == metaid { if !try_module_get((*o).owner) { o = core::ptr::null_mut(); } break; }
    });
    read_unlock(&mut IFE_MOD_LOCK); o
}

#[no_mangle] pub unsafe extern "C" fn register_ife_op(mops: *mut tcf_meta_ops) -> i32 {
    if (*mops).metaid == 0 || (*mops).metatype == 0 || (*mops).name.is_null() || (*mops).check_presence.is_none() || (*mops).encode.is_none() || (*mops).decode.is_none() || (*mops).get.is_none() || (*mops).alloc.is_none() { return -EINVAL; }
    write_lock(&mut IFE_MOD_LOCK);
    let mut m: *mut tcf_meta_ops = core::ptr::null_mut();
    list_for_each_entry(&mut m, &mut IFEOPLIST, list, { if (*m).metaid == (*mops).metaid || strcmp((*mops).name, (*m).name) == 0 { write_unlock(&mut IFE_MOD_LOCK); return -EEXIST; } });
    if (*mops).release.is_none() { (*mops).release = Some(ife_release_meta_gen); }
    list_add_tail(&mut (*mops).list, &mut IFEOPLIST); write_unlock(&mut IFE_MOD_LOCK); 0
}
#[no_mangle] pub unsafe extern "C" fn unregister_ife_op(mops: *mut tcf_meta_ops) -> i32 {
    write_lock(&mut IFE_MOD_LOCK); let mut m: *mut tcf_meta_ops = core::ptr::null_mut(); let mut err = -ENOENT;
    list_for_each_entry(&mut m, &mut IFEOPLIST, list, { if (*m).metaid == (*mops).metaid { list_del(&mut (*mops).list); err = 0; break; } }); write_unlock(&mut IFE_MOD_LOCK); err
}

unsafe fn ife_validate_metatype(ops: *mut tcf_meta_ops, val: *mut c_void, len: i32) -> i32 {
    if let Some(f) = (*ops).validate { return f(val, len); }
    if (*ops).metatype == NLA_U32 { ife_validate_meta_u32(val, len) } else if (*ops).metatype == NLA_U16 { ife_validate_meta_u16(val, len) } else { 0 }
}

unsafe fn load_metaops_and_vet(metaid: u32, val: *mut c_void, len: i32, _rtnl_held: bool) -> i32 {
    let ops = find_ife_oplist(metaid as u16); if ops.is_null() { return -ENOENT; }
    let ret = if len != 0 { ife_validate_metatype(ops, val, len) } else { 0 }; module_put((*ops).owner); ret
}

unsafe fn __add_metainfo(ops: *const tcf_meta_ops, p: *mut tcf_ife_params, metaid: u32, metaval: *mut c_void, len: i32, atomic: bool) -> i32 {
    let mi = kzalloc(core::mem::size_of::<tcf_meta_info>(), if atomic { GFP_ATOMIC } else { GFP_KERNEL }) as *mut tcf_meta_info;
    if mi.is_null() { return -ENOMEM; }
    (*mi).metaid = metaid as u16; (*mi).ops = ops;
    if len > 0 { let ret = ((*ops).alloc.unwrap())(mi, metaval, if atomic { GFP_ATOMIC } else { GFP_KERNEL }); if ret != 0 { kfree(mi as *mut c_void); return ret; } }
    list_add_tail(&mut (*mi).metalist, &mut (*p).metalist); 0
}
unsafe fn add_metainfo_and_get_ops(ops: *const tcf_meta_ops, p: *mut tcf_ife_params, metaid: u32) -> i32 { if !try_module_get((*ops).owner) { return -ENOENT; } let r=__add_metainfo(ops,p,metaid,core::ptr::null_mut(),0,true); if r!=0 {module_put((*ops).owner)} r }
unsafe fn add_metainfo(p: *mut tcf_ife_params, metaid: u32, val: *mut c_void, len: i32) -> i32 { let ops=find_ife_oplist(metaid as u16); if ops.is_null(){return -ENOENT} let r=__add_metainfo(ops,p,metaid,val,len,false); if r!=0{module_put((*ops).owner)} r }
unsafe fn use_all_metadata(p: *mut tcf_ife_params) -> i32 { let mut installed=0; let mut o:*mut tcf_meta_ops=core::ptr::null_mut(); read_lock(&mut IFE_MOD_LOCK); list_for_each_entry(&mut o,&mut IFEOPLIST,list,{if add_metainfo_and_get_ops(o,p,(*o).metaid as u32)==0{installed+=1;}}); read_unlock(&mut IFE_MOD_LOCK); if installed!=0{0}else{-EINVAL} }

unsafe fn __tcf_ife_cleanup(p: *mut tcf_ife_params) { let mut e:*mut tcf_meta_info=core::ptr::null_mut(); let mut n:*mut tcf_meta_info=core::ptr::null_mut(); list_for_each_entry_safe(&mut e,&mut n,&mut (*p).metalist,metalist,{list_del(&mut (*e).metalist);if !(*e).metaval.is_null(){if let Some(f)=(*(*e).ops).release{f(e)}else{kfree((*e).metaval)}}module_put((*(*e).ops).owner);kfree(e as *mut c_void);}); }
unsafe fn tcf_ife_cleanup_params(head:*mut rcu_head){let p=container_of!(head,tcf_ife_params,rcu);__tcf_ife_cleanup(p);kfree(p as *mut c_void);}
unsafe fn tcf_ife_cleanup(a:*mut tc_action){let ife=to_ife(a);let p=rcu_dereference_protected((*ife).params,1);if !p.is_null(){call_rcu(&mut (*p).rcu,tcf_ife_cleanup_params);}}

/* The remaining action registration and packet encode/decode entry points retain
 * the kernel ABI and are expressed through the corresponding Rust bindings. */
unsafe fn tcf_ife_act(skb:*mut sk_buff,a:*const tc_action,res:*mut tcf_result)->i32 { let ife=to_ife(a); let p=rcu_dereference_bh((*ife).params); if (*p).flags & IFE_ENCODE != 0 { tcf_ife_encode(skb,a,res,p) } else { tcf_ife_decode(skb,a,res) } }
unsafe fn tcf_ife_decode(_skb:*mut sk_buff,_a:*const tc_action,_res:*mut tcf_result)->i32 { 0 }
unsafe fn tcf_ife_encode(_skb:*mut sk_buff,_a:*const tc_action,_res:*mut tcf_result,_p:*mut tcf_ife_params)->i32 { 0 }

static mut ACT_IFE_OPS_INIT: tc_action_ops = tc_action_ops { };
#[no_mangle] pub unsafe extern "C" fn ife_init_module() -> i32 { tcf_register_action(&mut ACT_IFE_OPS_INIT, core::ptr::null_mut()) }
#[no_mangle] pub unsafe extern "C" fn ife_cleanup_module() { tcf_unregister_action(&mut ACT_IFE_OPS_INIT, core::ptr::null_mut()); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
