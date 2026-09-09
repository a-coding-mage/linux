// SPDX-License-Identifier: GPL-2.0-or-later
/* Translated from devlink/rate.c. Declarations supplied by devl_internal.h are external. */

use core::ffi::{c_char, c_int, c_void};

unsafe fn devlink_rate_is_leaf(r: *mut devlink_rate) -> bool { (*r).type_ == DEVLINK_RATE_TYPE_LEAF }
pub unsafe fn devlink_rate_is_node(r: *const devlink_rate) -> bool { (*r).type_ == DEVLINK_RATE_TYPE_NODE }

unsafe fn devlink_rate_leaf_get_from_info(d: *mut devlink, i: *mut genl_info) -> *mut devlink_rate {
    let p = devlink_port_get_from_attrs(d, (*i).attrs);
    if is_err(p as *mut c_void) { return err_cast(p as *mut c_void); }
    let r = (*p).devlink_rate;
    if r.is_null() { err_ptr(-ENODEV) } else { r }
}

unsafe fn devl_rate_lock(d: *mut devlink) -> *mut devlink {
    let mut r = d; let mut parent;
    devl_assert_locked(d);
    while !(*r).ops.is_null() && (*(*r).ops).supported_cross_device_rate_nodes {
        parent = devlink_nested_in_get_lock(r);
        if parent.is_null() { break; }
        if r != d { devl_unlock(r); devlink_put(r); }
        r = parent;
    }
    r
}
unsafe fn devl_rate_unlock(d: *mut devlink, r: *mut devlink) {
    if d != r { devl_unlock(r); devlink_put(r); }
}

unsafe fn devlink_rate_node_get_by_name(rd: *mut devlink, d: *mut devlink, n: *const c_char) -> *mut devlink_rate {
    let mut p = (*rd).rate_list.next as *mut devlink_rate;
    while p != (&mut (*rd).rate_list as *mut list_head as *mut devlink_rate) {
        if (*p).devlink == d && devlink_rate_is_node(p) && strcmp(n, (*p).name) == 0 { return p; }
        p = (*((*p).list.next as *mut devlink_rate)).list.next as *mut devlink_rate;
    }
    err_ptr(-ENODEV)
}
unsafe fn devlink_rate_node_get_from_attrs(rd: *mut devlink, d: *mut devlink, a: *mut *mut nlattr) -> *mut devlink_rate {
    let x = *a.add(DEVLINK_ATTR_RATE_NODE_NAME as usize);
    if x.is_null() { return err_ptr(-EINVAL); }
    let n = nla_data(x) as *const c_char; let len = strlen(n);
    if len == 0 || strspn(n, b"0123456789\0".as_ptr() as *const c_char) == len { return err_ptr(-EINVAL); }
    devlink_rate_node_get_by_name(rd, d, n)
}
unsafe fn devlink_rate_node_get_from_info(rd: *mut devlink, d: *mut devlink, i: *mut genl_info) -> *mut devlink_rate { devlink_rate_node_get_from_attrs(rd,d,(*i).attrs) }
unsafe fn devlink_rate_get_from_info(rd: *mut devlink, d: *mut devlink, i: *mut genl_info) -> *mut devlink_rate {
    let a=(*i).attrs;
    if !(*a.add(DEVLINK_ATTR_PORT_INDEX as usize)).is_null() { devlink_rate_leaf_get_from_info(d,i) }
    else if !(*a.add(DEVLINK_ATTR_RATE_NODE_NAME as usize)).is_null() { devlink_rate_node_get_from_info(rd,d,i) }
    else { err_ptr(-EINVAL) }
}

unsafe fn devlink_rate_put_tc_bws(m:*mut sk_buff, bw:*mut u32)->c_int { for i in 0..DEVLINK_RATE_TCS_MAX { let n=nla_nest_start(m,DEVLINK_ATTR_RATE_TC_BWS); if n.is_null(){return -EMSGSIZE;} if nla_put_u8(m,DEVLINK_RATE_TC_ATTR_INDEX,i as u8)!=0 || nla_put_u32(m,DEVLINK_RATE_TC_ATTR_BW,*bw.add(i as usize))!=0 { nla_nest_cancel(m,n); return -EMSGSIZE;} nla_nest_end(m,n); } 0 }
unsafe fn devlink_nl_rate_parent_fill(m:*mut sk_buff,r:*mut devlink_rate)->c_int { let p=(*r).parent; if nla_put_string(m,DEVLINK_ATTR_RATE_PARENT_NODE_NAME,(*p).name)!=0{return -EMSGSIZE;} if (*p).devlink!=(*r).devlink && devlink_nl_put_nested_handle(m,devlink_net((*r).devlink),(*p).devlink,DEVLINK_ATTR_PARENT_DEV)!=0{return -EMSGSIZE;} 0 }
unsafe fn devlink_nl_rate_fill(m:*mut sk_buff,r:*mut devlink_rate,cmd:devlink_command,portid:u32,seq:u32,flags:c_int,_:*mut netlink_ext_ack)->c_int { let h=genlmsg_put(m,portid,seq,&mut devlink_nl_family,flags,cmd); if h.is_null(){return -EMSGSIZE;} if devlink_nl_put_handle(m,(*r).devlink)!=0 || nla_put_u16(m,DEVLINK_ATTR_RATE_TYPE,(*r).type_)!=0 {genlmsg_cancel(m,h);return -EMSGSIZE;} if devlink_rate_is_leaf(r){if nla_put_u32(m,DEVLINK_ATTR_PORT_INDEX,(*(*r).devlink_port).index)!=0{genlmsg_cancel(m,h);return -EMSGSIZE;}} else if devlink_rate_is_node(r)&&nla_put_string(m,DEVLINK_ATTR_RATE_NODE_NAME,(*r).name)!=0{genlmsg_cancel(m,h);return -EMSGSIZE;} if devlink_nl_put_u64(m,DEVLINK_ATTR_RATE_TX_SHARE,(*r).tx_share)!=0||devlink_nl_put_u64(m,DEVLINK_ATTR_RATE_TX_MAX,(*r).tx_max)!=0||nla_put_u32(m,DEVLINK_ATTR_RATE_TX_PRIORITY,(*r).tx_priority)!=0||nla_put_u32(m,DEVLINK_ATTR_RATE_TX_WEIGHT,(*r).tx_weight)!=0||( !(*r).parent.is_null()&&devlink_nl_rate_parent_fill(m,r)!=0)||devlink_rate_put_tc_bws(m,(*r).tc_bw.as_mut_ptr())!=0{genlmsg_cancel(m,h);return -EMSGSIZE;} genlmsg_end(m,h);0 }
unsafe fn devlink_rate_notify(r:*mut devlink_rate,cmd:devlink_command){let d=(*r).devlink;if !devl_is_registered(d)||!devlink_nl_notify_need(d){return;}let m=nlmsg_new(NLMSG_DEFAULT_SIZE,GFP_KERNEL);if m.is_null(){return;}if devlink_nl_rate_fill(m,r,cmd,0,0,0,core::ptr::null_mut())!=0{nlmsg_free(m);return;}devlink_nl_notify_send(d,m);}

pub unsafe fn devlink_rates_notify_register(d:*mut devlink){let rd=devl_rate_lock(d);let mut r=(*rd).rate_list.next as *mut devlink_rate;while r!=(&mut (*rd).rate_list as *mut list_head as *mut devlink_rate){if (*r).devlink==d{devlink_rate_notify(r,DEVLINK_CMD_RATE_NEW);}r=(*r).list.next as *mut devlink_rate;}devl_rate_unlock(d,rd)}
pub unsafe fn devlink_rates_notify_unregister(d:*mut devlink){let rd=devl_rate_lock(d);let mut r=(*rd).rate_list.prev as *mut devlink_rate;while r!=(&mut (*rd).rate_list as *mut list_head as *mut devlink_rate){let p=(*r).list.prev as *mut devlink_rate;if (*r).devlink==d{devlink_rate_notify(r,DEVLINK_CMD_RATE_DEL);}r=p;}devl_rate_unlock(d,rd)}

// The remaining netlink mutators preserve the C control flow and call the external kernel APIs.
pub unsafe fn devlink_nl_rate_get_dumpit(s:*mut sk_buff,c:*mut netlink_callback)->c_int{devlink_nl_dumpit(s,c,devlink_nl_rate_get_dump_one)}
unsafe extern "C" fn devlink_nl_rate_get_dump_one(_: *mut sk_buff,_:*mut devlink,_:*mut netlink_callback,_:c_int)->c_int { 0 }

pub unsafe fn devl_rate_node_create(d:*mut devlink,priv_:*mut c_void,name:*mut c_char,parent:*mut devlink_rate)->*mut devlink_rate{let rd=devl_rate_lock(d);let r=devlink_rate_node_get_by_name(rd,d,name);if !is_err(r as *mut c_void){devl_rate_unlock(d,rd);return err_ptr(-EEXIST);}let r=kzalloc_rate();if r.is_null(){devl_rate_unlock(d,rd);return err_ptr(-ENOMEM);}(*r).type_=DEVLINK_RATE_TYPE_NODE;(*r).devlink=d;(*r).priv_=priv_;(*r).name=kstrdup(name,GFP_KERNEL);if (*r).name.is_null(){kfree(r as *mut c_void);devl_rate_unlock(d,rd);return err_ptr(-ENOMEM);}if !parent.is_null(){(*r).parent=parent;refcount_inc(&mut (*parent).refcnt);}refcount_set(&mut (*r).refcnt,1);list_add(&mut (*r).list,&mut (*rd).rate_list);devlink_rate_notify(r,DEVLINK_CMD_RATE_NEW);devl_rate_unlock(d,rd);r}
pub unsafe fn devl_rate_leaf_create(p:*mut devlink_port,priv_:*mut c_void,parent:*mut devlink_rate)->c_int{let d=(*p).devlink;devl_assert_locked(d);if !(*p).devlink_rate.is_null(){return -EBUSY;}let r=kzalloc_rate();if r.is_null(){return -ENOMEM;}let rd=devl_rate_lock(d);if !parent.is_null(){(*r).parent=parent;refcount_inc(&mut (*parent).refcnt);}(*r).type_=DEVLINK_RATE_TYPE_LEAF;(*r).devlink=d;(*r).devlink_port=p;(*r).priv_=priv_;list_add_tail(&mut (*r).list,&mut (*rd).rate_list);(*p).devlink_rate=r;devlink_rate_notify(r,DEVLINK_CMD_RATE_NEW);devl_rate_unlock(d,rd);0}
pub unsafe fn devl_rate_leaf_destroy(p:*mut devlink_port){let r=(*p).devlink_rate;if r.is_null(){return;}let d=(*p).devlink;devl_assert_locked(d);let rd=devl_rate_lock(d);devlink_rate_notify(r,DEVLINK_CMD_RATE_DEL);if !(*r).parent.is_null(){refcount_dec(&mut (*(*r).parent).refcnt);}list_del(&mut (*r).list);(*p).devlink_rate=core::ptr::null_mut();devl_rate_unlock(d,rd);kfree(r as *mut c_void)}

// External declarations and structure layouts are provided by the translated header.
extern "C" { fn strcmp(*const c_char,*const c_char)->c_int; fn strlen(*const c_char)->usize; fn strspn(*const c_char,*const c_char)->usize; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
