// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2016 Mellanox Technologies. All rights reserved.
 * Copyright (c) 2016 Jiri Pirko <jiri@mellanox.com>
 */

// Dependencies supplied by the surrounding devlink/kernel translation.

static DEVLINK_PARAM_GENERIC: [devlink_param; 23] = [
    devlink_param { id: DEVLINK_PARAM_GENERIC_ID_INT_ERR_RESET, name: DEVLINK_PARAM_GENERIC_INT_ERR_RESET_NAME, type_: DEVLINK_PARAM_GENERIC_INT_ERR_RESET_TYPE, ..unsafe { core::mem::zeroed() } },
    devlink_param { id: DEVLINK_PARAM_GENERIC_ID_MAX_MACS, name: DEVLINK_PARAM_GENERIC_MAX_MACS_NAME, type_: DEVLINK_PARAM_GENERIC_MAX_MACS_TYPE, ..unsafe { core::mem::zeroed() } },
    devlink_param { id: DEVLINK_PARAM_GENERIC_ID_ENABLE_SRIOV, name: DEVLINK_PARAM_GENERIC_ENABLE_SRIOV_NAME, type_: DEVLINK_PARAM_GENERIC_ENABLE_SRIOV_TYPE, ..unsafe { core::mem::zeroed() } },
    devlink_param { id: DEVLINK_PARAM_GENERIC_ID_REGION_SNAPSHOT, name: DEVLINK_PARAM_GENERIC_REGION_SNAPSHOT_NAME, type_: DEVLINK_PARAM_GENERIC_REGION_SNAPSHOT_TYPE, ..unsafe { core::mem::zeroed() } },
    devlink_param { id: DEVLINK_PARAM_GENERIC_ID_IGNORE_ARI, name: DEVLINK_PARAM_GENERIC_IGNORE_ARI_NAME, type_: DEVLINK_PARAM_GENERIC_IGNORE_ARI_TYPE, ..unsafe { core::mem::zeroed() } },
    devlink_param { id: DEVLINK_PARAM_GENERIC_ID_MSIX_VEC_PER_PF_MAX, name: DEVLINK_PARAM_GENERIC_MSIX_VEC_PER_PF_MAX_NAME, type_: DEVLINK_PARAM_GENERIC_MSIX_VEC_PER_PF_MAX_TYPE, ..unsafe { core::mem::zeroed() } },
    devlink_param { id: DEVLINK_PARAM_GENERIC_ID_MSIX_VEC_PER_PF_MIN, name: DEVLINK_PARAM_GENERIC_MSIX_VEC_PER_PF_MIN_NAME, type_: DEVLINK_PARAM_GENERIC_MSIX_VEC_PER_PF_MIN_TYPE, ..unsafe { core::mem::zeroed() } },
    devlink_param { id: DEVLINK_PARAM_GENERIC_ID_FW_LOAD_POLICY, name: DEVLINK_PARAM_GENERIC_FW_LOAD_POLICY_NAME, type_: DEVLINK_PARAM_GENERIC_FW_LOAD_POLICY_TYPE, ..unsafe { core::mem::zeroed() } },
    devlink_param { id: DEVLINK_PARAM_GENERIC_ID_RESET_DEV_ON_DRV_PROBE, name: DEVLINK_PARAM_GENERIC_RESET_DEV_ON_DRV_PROBE_NAME, type_: DEVLINK_PARAM_GENERIC_RESET_DEV_ON_DRV_PROBE_TYPE, ..unsafe { core::mem::zeroed() } },
    devlink_param { id: DEVLINK_PARAM_GENERIC_ID_ENABLE_ROCE, name: DEVLINK_PARAM_GENERIC_ENABLE_ROCE_NAME, type_: DEVLINK_PARAM_GENERIC_ENABLE_ROCE_TYPE, ..unsafe { core::mem::zeroed() } },
    devlink_param { id: DEVLINK_PARAM_GENERIC_ID_ENABLE_REMOTE_DEV_RESET, name: DEVLINK_PARAM_GENERIC_ENABLE_REMOTE_DEV_RESET_NAME, type_: DEVLINK_PARAM_GENERIC_ENABLE_REMOTE_DEV_RESET_TYPE, ..unsafe { core::mem::zeroed() } },
    devlink_param { id: DEVLINK_PARAM_GENERIC_ID_ENABLE_ETH, name: DEVLINK_PARAM_GENERIC_ENABLE_ETH_NAME, type_: DEVLINK_PARAM_GENERIC_ENABLE_ETH_TYPE, ..unsafe { core::mem::zeroed() } },
    devlink_param { id: DEVLINK_PARAM_GENERIC_ID_ENABLE_RDMA, name: DEVLINK_PARAM_GENERIC_ENABLE_RDMA_NAME, type_: DEVLINK_PARAM_GENERIC_ENABLE_RDMA_TYPE, ..unsafe { core::mem::zeroed() } },
    devlink_param { id: DEVLINK_PARAM_GENERIC_ID_ENABLE_VNET, name: DEVLINK_PARAM_GENERIC_ENABLE_VNET_NAME, type_: DEVLINK_PARAM_GENERIC_ENABLE_VNET_TYPE, ..unsafe { core::mem::zeroed() } },
    devlink_param { id: DEVLINK_PARAM_GENERIC_ID_ENABLE_IWARP, name: DEVLINK_PARAM_GENERIC_ENABLE_IWARP_NAME, type_: DEVLINK_PARAM_GENERIC_ENABLE_IWARP_TYPE, ..unsafe { core::mem::zeroed() } },
    devlink_param { id: DEVLINK_PARAM_GENERIC_ID_IO_EQ_SIZE, name: DEVLINK_PARAM_GENERIC_IO_EQ_SIZE_NAME, type_: DEVLINK_PARAM_GENERIC_IO_EQ_SIZE_TYPE, ..unsafe { core::mem::zeroed() } },
    devlink_param { id: DEVLINK_PARAM_GENERIC_ID_EVENT_EQ_SIZE, name: DEVLINK_PARAM_GENERIC_EVENT_EQ_SIZE_NAME, type_: DEVLINK_PARAM_GENERIC_EVENT_EQ_SIZE_TYPE, ..unsafe { core::mem::zeroed() } },
    devlink_param { id: DEVLINK_PARAM_GENERIC_ID_ENABLE_PHC, name: DEVLINK_PARAM_GENERIC_ENABLE_PHC_NAME, type_: DEVLINK_PARAM_GENERIC_ENABLE_PHC_TYPE, ..unsafe { core::mem::zeroed() } },
    devlink_param { id: DEVLINK_PARAM_GENERIC_ID_CLOCK_ID, name: DEVLINK_PARAM_GENERIC_CLOCK_ID_NAME, type_: DEVLINK_PARAM_GENERIC_CLOCK_ID_TYPE, ..unsafe { core::mem::zeroed() } },
    devlink_param { id: DEVLINK_PARAM_GENERIC_ID_TOTAL_VFS, name: DEVLINK_PARAM_GENERIC_TOTAL_VFS_NAME, type_: DEVLINK_PARAM_GENERIC_TOTAL_VFS_TYPE, ..unsafe { core::mem::zeroed() } },
    devlink_param { id: DEVLINK_PARAM_GENERIC_ID_NUM_DOORBELLS, name: DEVLINK_PARAM_GENERIC_NUM_DOORBELLS_NAME, type_: DEVLINK_PARAM_GENERIC_NUM_DOORBELLS_TYPE, ..unsafe { core::mem::zeroed() } },
    devlink_param { id: DEVLINK_PARAM_GENERIC_ID_MAX_MAC_PER_VF, name: DEVLINK_PARAM_GENERIC_MAX_MAC_PER_VF_NAME, type_: DEVLINK_PARAM_GENERIC_MAX_MAC_PER_VF_TYPE, ..unsafe { core::mem::zeroed() } },
    devlink_param { id: DEVLINK_PARAM_GENERIC_ID_MAX_SFS, name: DEVLINK_PARAM_GENERIC_MAX_SFS_NAME, type_: DEVLINK_PARAM_GENERIC_MAX_SFS_TYPE, ..unsafe { core::mem::zeroed() } },
];

unsafe fn devlink_param_generic_verify(param: *const devlink_param) -> i32 {
    if (*param).id > DEVLINK_PARAM_GENERIC_ID_MAX { return -EINVAL; }
    if strcmp((*param).name, DEVLINK_PARAM_GENERIC[(*param).id as usize].name) != 0 { return -ENOENT; }
    WARN_ON((*param).type_ != DEVLINK_PARAM_GENERIC[(*param).id as usize].type_);
    0
}

unsafe fn devlink_param_driver_verify(param: *const devlink_param) -> i32 {
    if (*param).id <= DEVLINK_PARAM_GENERIC_ID_MAX { return -EINVAL; }
    for i in 0..=DEVLINK_PARAM_GENERIC_ID_MAX {
        if strcmp((*param).name, DEVLINK_PARAM_GENERIC[i as usize].name) == 0 { return -EEXIST; }
    }
    0
}

unsafe fn devlink_param_find_by_name(params: *mut xarray, param_name: *const c_char) -> *mut devlink_param_item {
    let mut param_item: *mut devlink_param_item = core::ptr::null_mut();
    let mut param_id: c_ulong = 0;
    xa_for_each!(params, param_id, param_item, {
        if strcmp((*param_item).param.as_ref().unwrap().name, param_name) == 0 { return param_item; }
    });
    core::ptr::null_mut()
}

unsafe fn devlink_param_find_by_id(params: *mut xarray, param_id: u32) -> *mut devlink_param_item { xa_load(params, param_id) }
unsafe fn devlink_param_cmode_is_supported(param: *const devlink_param, cmode: devlink_param_cmode) -> bool { test_bit(cmode, &(*param).supported_cmodes) }

unsafe fn devlink_param_get(devlink: *mut devlink, param: *const devlink_param, ctx: *mut devlink_param_gset_ctx, extack: *mut netlink_ext_ack) -> i32 {
    if (*param).get.is_none() { return -EOPNOTSUPP; }
    (*param).get.unwrap()(devlink, (*param).id, ctx, extack)
}
unsafe fn devlink_param_set(devlink: *mut devlink, param: *const devlink_param, ctx: *mut devlink_param_gset_ctx, extack: *mut netlink_ext_ack) -> i32 {
    if (*param).set.is_none() { return -EOPNOTSUPP; }
    (*param).set.unwrap()(devlink, (*param).id, ctx, extack)
}
unsafe fn devlink_param_get_default(devlink: *mut devlink, param: *const devlink_param, ctx: *mut devlink_param_gset_ctx, extack: *mut netlink_ext_ack) -> i32 {
    if (*param).get_default.is_none() { return -EOPNOTSUPP; }
    (*param).get_default.unwrap()(devlink, (*param).id, ctx, extack)
}
unsafe fn devlink_param_reset_default(devlink: *mut devlink, param: *const devlink_param, cmode: devlink_param_cmode, extack: *mut netlink_ext_ack) -> i32 {
    if (*param).reset_default.is_none() { return -EOPNOTSUPP; }
    (*param).reset_default.unwrap()(devlink, (*param).id, cmode, extack)
}

unsafe fn devlink_nl_param_value_put(msg: *mut sk_buff, type_: devlink_param_type, nla_type: i32, val: *mut devlink_param_value, flag_as_u8: bool) -> i32 {
    match type_ {
        DEVLINK_PARAM_TYPE_U8 => if nla_put_u8(msg,nla_type,(*val).vu8)!=0{-EMSGSIZE},
        DEVLINK_PARAM_TYPE_U16 => if nla_put_u16(msg,nla_type,(*val).vu16)!=0{-EMSGSIZE},
        DEVLINK_PARAM_TYPE_U32 => if nla_put_u32(msg,nla_type,(*val).vu32)!=0{-EMSGSIZE},
        DEVLINK_PARAM_TYPE_U64 => if devlink_nl_put_u64(msg,nla_type,(*val).vu64)!=0{-EMSGSIZE},
        DEVLINK_PARAM_TYPE_STRING => if nla_put_string(msg,nla_type,(*val).vstr)!=0{-EMSGSIZE},
        DEVLINK_PARAM_TYPE_BOOL => { if flag_as_u8 { if nla_put_u8(msg,nla_type,(*val).vbool)!=0{-EMSGSIZE} } else if (*val).vbool && nla_put_flag(msg,nla_type)!=0{-EMSGSIZE} },
        DEVLINK_PARAM_TYPE_U64_ARRAY => { if (*val).u64arr.size > __DEVLINK_PARAM_MAX_ARRAY_SIZE{return -EMSGSIZE;} for i in 0..(*val).u64arr.size { if nla_put_uint(msg,nla_type,(*val).u64arr.val[i as usize])!=0{return -EMSGSIZE;} } },
        _ => 0,
    }
}

unsafe fn devlink_nl_param_value_fill_one(msg:*mut sk_buff,type_:devlink_param_type,cmode:devlink_param_cmode,val:*mut devlink_param_value,default_val:*mut devlink_param_value,has_default:bool)->i32 {
    let attr=nla_nest_start_noflag(msg,DEVLINK_ATTR_PARAM_VALUE); if attr.is_null(){return -EMSGSIZE;}
    if nla_put_u8(msg,DEVLINK_ATTR_PARAM_VALUE_CMODE,cmode)!=0 { nla_nest_cancel(msg,attr); return -EMSGSIZE; }
    let mut err=devlink_nl_param_value_put(msg,type_,DEVLINK_ATTR_PARAM_VALUE_DATA,val,false);
    if err==0 && has_default {err=devlink_nl_param_value_put(msg,type_,DEVLINK_ATTR_PARAM_VALUE_DEFAULT,default_val,true);}
    if err!=0 {nla_nest_cancel(msg,attr); return err;} nla_nest_end(msg,attr); 0
}

unsafe fn devlink_param_notify(devlink:*mut devlink,port_index:u32,param_item:*mut devlink_param_item,cmd:devlink_command){
    WARN_ON(cmd!=DEVLINK_CMD_PARAM_NEW&&cmd!=DEVLINK_CMD_PARAM_DEL&&cmd!=DEVLINK_CMD_PORT_PARAM_NEW&&cmd!=DEVLINK_CMD_PORT_PARAM_DEL);
    if !devl_is_registered(devlink)||!devlink_nl_notify_need(devlink){return;}
    let msg=nlmsg_new(NLMSG_DEFAULT_SIZE,GFP_KERNEL); if msg.is_null(){return;}
    if devlink_nl_param_fill(msg,devlink,port_index,param_item,cmd,0,0,0,core::ptr::null_mut())!=0 {nlmsg_free(msg);return;} devlink_nl_notify_send(devlink,msg);
}

unsafe fn devlink_param_verify(param:*const devlink_param)->i32 { if param.is_null()||(*param).name.is_null()||(*param).supported_cmodes==0{return -EINVAL;} if (*param).generic{devlink_param_generic_verify(param)}else{devlink_param_driver_verify(param)} }

unsafe fn devlink_param_register(devlink:*mut devlink,param:*const devlink_param)->i32 { if devlink_param_verify(param)!=0{return -EINVAL;} let item=kzalloc_obj::<devlink_param_item>();if item.is_null(){return -ENOMEM;}(*item).param=param;let e=xa_insert(&mut (*devlink).params,(*param).id,item,GFP_KERNEL);if e!=0{kfree(item);return e;}devlink_param_notify(devlink,0,item,DEVLINK_CMD_PARAM_NEW);0 }
unsafe fn devlink_param_unregister(devlink:*mut devlink,param:*const devlink_param){let item=devlink_param_find_by_id(&mut (*devlink).params,(*param).id);if item.is_null(){return;}devlink_param_notify(devlink,0,item,DEVLINK_CMD_PARAM_DEL);xa_erase(&mut (*devlink).params,(*param).id);kfree(item)}
pub unsafe fn devlink_params_notify_register(devlink:*mut devlink){let mut id=0;let mut p=core::ptr::null_mut();xa_for_each!(&mut (*devlink).params,id,p,{devlink_param_notify(devlink,0,p,DEVLINK_CMD_PARAM_NEW);})}
pub unsafe fn devlink_params_notify_unregister(devlink:*mut devlink){let mut id=0;let mut p=core::ptr::null_mut();xa_for_each!(&mut (*devlink).params,id,p,{devlink_param_notify(devlink,0,p,DEVLINK_CMD_PARAM_DEL);})}

extern "C" { fn devlink_nl_param_fill(msg:*mut sk_buff,devlink:*mut devlink,port_index:u32,param_item:*mut devlink_param_item,cmd:devlink_command,portid:u32,seq:u32,flags:i32,extack:*mut netlink_ext_ack)->i32; }

// The remaining exported operations preserve the source API and delegate to the
// surrounding kernel/devlink translation for netlink filling and xarray access.
pub unsafe fn devl_params_register(devlink:*mut devlink,params:*const devlink_param,params_count:usize)->i32 { let mut i=0; while i<params_count { let err=devlink_param_register(devlink,params.add(i)); if err!=0 {while i>0{i-=1;devlink_param_unregister(devlink,params.add(i));}return err;}i+=1;}0 }
pub unsafe fn devlink_params_register(devlink:*mut devlink,params:*const devlink_param,params_count:usize)->i32 {devl_lock(devlink);let e=devl_params_register(devlink,params,params_count);devl_unlock(devlink);e}
pub unsafe fn devl_params_unregister(devlink:*mut devlink,params:*const devlink_param,params_count:usize){let mut i=0;while i<params_count{devlink_param_unregister(devlink,params.add(i));i+=1;}}
pub unsafe fn devlink_params_unregister(devlink:*mut devlink,params:*const devlink_param,params_count:usize){devl_lock(devlink);devl_params_unregister(devlink,params,params_count);devl_unlock(devlink)}

pub unsafe fn devl_param_driverinit_value_get(devlink:*mut devlink,param_id:u32,val:*mut devlink_param_value)->i32{if !devlink_reload_supported((*devlink).ops){return -EOPNOTSUPP;}let p=devlink_param_find_by_id(&mut (*devlink).params,param_id);if p.is_null()||!(*p).driverinit_value_valid{return -EINVAL;}if !devlink_param_cmode_is_supported((*p).param,DEVLINK_PARAM_CMODE_DRIVERINIT){return -EOPNOTSUPP;}*val=(*p).driverinit_value;0}
pub unsafe fn devl_param_driverinit_value_set(devlink:*mut devlink,param_id:u32,init_val:*mut devlink_param_value){let p=devlink_param_find_by_id(&mut (*devlink).params,param_id);if p.is_null(){return;}(*p).driverinit_value=*init_val;(*p).driverinit_value_valid=true;(*p).driverinit_default=*init_val;devlink_param_notify(devlink,0,p,DEVLINK_CMD_PARAM_NEW)}
pub unsafe fn devlink_params_driverinit_load_new(devlink:*mut devlink){let mut id=0;let mut p=core::ptr::null_mut();xa_for_each!(&mut (*devlink).params,id,p,{if devlink_param_cmode_is_supported((*p).param,DEVLINK_PARAM_CMODE_DRIVERINIT)&&(*p).driverinit_value_new_valid{(*p).driverinit_value=(*p).driverinit_value_new;(*p).driverinit_value_valid=true;(*p).driverinit_value_new_valid=false;}})}
pub unsafe fn devl_param_value_changed(devlink:*mut devlink,param_id:u32){let p=devlink_param_find_by_id(&mut (*devlink).params,param_id);WARN_ON(p.is_null());devlink_param_notify(devlink,0,p,DEVLINK_CMD_PARAM_NEW)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
