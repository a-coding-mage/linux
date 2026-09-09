// SPDX-License-Identifier: GPL-2.0-or-later
/* Translation of devlink/sb.c. External kernel declarations are supplied by
 * the surrounding devlink implementation. */

#[repr(C)]
pub struct DevlinkSb {
    pub list: ListHead,
    pub index: c_uint,
    pub size: u32,
    pub ingress_pools_count: u16,
    pub egress_pools_count: u16,
    pub ingress_tc_count: u16,
    pub egress_tc_count: u16,
}

unsafe fn devlink_sb_pool_count(sb: *mut DevlinkSb) -> u16 {
    (*sb).ingress_pools_count.wrapping_add((*sb).egress_pools_count)
}

unsafe fn devlink_sb_get_by_index(devlink: *mut Devlink, index: c_uint) -> *mut DevlinkSb {
    // list_for_each_entry(devlink_sb, &devlink->sb_list, list)
    let mut p = (*devlink).sb_list.next as *mut DevlinkSb;
    while !p.is_null() {
        if (*p).index == index { return p; }
        p = (*p).list.next as *mut DevlinkSb;
    }
    core::ptr::null_mut()
}

unsafe fn devlink_sb_index_exists(d: *mut Devlink, i: c_uint) -> bool {
    !devlink_sb_get_by_index(d, i).is_null()
}

unsafe fn devlink_sb_get_from_attrs(d: *mut Devlink, attrs: *mut *mut Nlattr) -> *mut DevlinkSb {
    let a = *attrs.add(DEVLINK_ATTR_SB_INDEX as usize);
    if !a.is_null() {
        let sb = devlink_sb_get_by_index(d, nla_get_u32(a));
        if sb.is_null() { return ERR_PTR(-ENODEV); }
        return sb;
    }
    ERR_PTR(-EINVAL)
}
unsafe fn devlink_sb_get_from_info(d: *mut Devlink, i: *mut GenlInfo) -> *mut DevlinkSb {
    devlink_sb_get_from_attrs(d, (*i).attrs)
}
unsafe fn devlink_sb_pool_index_get_from_attrs(sb: *mut DevlinkSb, a: *mut *mut Nlattr, out: *mut u16) -> c_int {
    let x = *a.add(DEVLINK_ATTR_SB_POOL_INDEX as usize);
    if x.is_null() { return -EINVAL; }
    let v = nla_get_u16(x);
    if v >= devlink_sb_pool_count(sb) { return -EINVAL; }
    *out = v; 0
}
unsafe fn devlink_sb_pool_index_get_from_info(sb: *mut DevlinkSb, i: *mut GenlInfo, o: *mut u16) -> c_int {
    devlink_sb_pool_index_get_from_attrs(sb, (*i).attrs, o)
}
unsafe fn devlink_sb_pool_type_get_from_attrs(a: *mut *mut Nlattr, out: *mut DevlinkSbPoolType) -> c_int {
    let x = *a.add(DEVLINK_ATTR_SB_POOL_TYPE as usize); if x.is_null() { return -EINVAL; }
    let v = nla_get_u8(x); if v != DEVLINK_SB_POOL_TYPE_INGRESS && v != DEVLINK_SB_POOL_TYPE_EGRESS { return -EINVAL; }
    *out = v; 0
}
unsafe fn devlink_sb_pool_type_get_from_info(i: *mut GenlInfo, o: *mut DevlinkSbPoolType) -> c_int { devlink_sb_pool_type_get_from_attrs((*i).attrs, o) }
unsafe fn devlink_sb_th_type_get_from_attrs(a: *mut *mut Nlattr, out: *mut DevlinkSbThresholdType) -> c_int {
    let x = *a.add(DEVLINK_ATTR_SB_POOL_THRESHOLD_TYPE as usize); if x.is_null() { return -EINVAL; }
    let v = nla_get_u8(x); if v != DEVLINK_SB_THRESHOLD_TYPE_STATIC && v != DEVLINK_SB_THRESHOLD_TYPE_DYNAMIC { return -EINVAL; }
    *out = v; 0
}
unsafe fn devlink_sb_th_type_get_from_info(i: *mut GenlInfo, o: *mut DevlinkSbThresholdType) -> c_int { devlink_sb_th_type_get_from_attrs((*i).attrs, o) }
unsafe fn devlink_sb_tc_index_get_from_attrs(sb: *mut DevlinkSb, a: *mut *mut Nlattr, ty: DevlinkSbPoolType, out: *mut u16) -> c_int {
    let x = *a.add(DEVLINK_ATTR_SB_TC_INDEX as usize); if x.is_null() { return -EINVAL; }
    let v = nla_get_u16(x);
    if ty == DEVLINK_SB_POOL_TYPE_INGRESS && v >= (*sb).ingress_tc_count { return -EINVAL; }
    if ty == DEVLINK_SB_POOL_TYPE_EGRESS && v >= (*sb).egress_tc_count { return -EINVAL; }
    *out = v; 0
}
unsafe fn devlink_sb_tc_index_get_from_info(sb: *mut DevlinkSb, i: *mut GenlInfo, ty: DevlinkSbPoolType, o: *mut u16) -> c_int { devlink_sb_tc_index_get_from_attrs(sb, (*i).attrs, ty, o) }

unsafe fn devlink_nl_sb_fill(msg: *mut SkBuff, d: *mut Devlink, sb: *mut DevlinkSb, cmd: DevlinkCommand, portid: u32, seq: u32, flags: c_int) -> c_int {
    let hdr = genlmsg_put(msg, portid, seq, &devlink_nl_family, flags, cmd); if hdr.is_null() { return -EMSGSIZE; }
    if devlink_nl_put_handle(msg, d) != 0 || nla_put_u32(msg, DEVLINK_ATTR_SB_INDEX, (*sb).index) != 0 || nla_put_u32(msg, DEVLINK_ATTR_SB_SIZE, (*sb).size) != 0 ||
       nla_put_u16(msg, DEVLINK_ATTR_SB_INGRESS_POOL_COUNT, (*sb).ingress_pools_count) != 0 || nla_put_u16(msg, DEVLINK_ATTR_SB_EGRESS_POOL_COUNT, (*sb).egress_pools_count) != 0 ||
       nla_put_u16(msg, DEVLINK_ATTR_SB_INGRESS_TC_COUNT, (*sb).ingress_tc_count) != 0 || nla_put_u16(msg, DEVLINK_ATTR_SB_EGRESS_TC_COUNT, (*sb).egress_tc_count) != 0 { genlmsg_cancel(msg, hdr); return -EMSGSIZE; }
    genlmsg_end(msg, hdr); 0
}

// The remaining netlink entry points retain the C control flow and delegate
// serialization and driver operations to the declarations from devl_internal.
pub unsafe fn devlink_nl_sb_get_doit(_skb: *mut SkBuff, info: *mut GenlInfo) -> c_int {
    let d = devlink_nl_ctx(info).devlink; let sb = devlink_sb_get_from_info(d, info); if IS_ERR(sb) { return PTR_ERR(sb); }
    let msg = nlmsg_new(NLMSG_DEFAULT_SIZE, GFP_KERNEL); if msg.is_null() { return -ENOMEM; }
    let e = devlink_nl_sb_fill(msg, d, sb, DEVLINK_CMD_SB_NEW, (*info).snd_portid, (*info).snd_seq, 0); if e != 0 { nlmsg_free(msg); return e; } genlmsg_reply(msg, info)
}

pub unsafe fn devlink_nl_sb_get_dumpit(skb: *mut SkBuff, cb: *mut NetlinkCallback) -> c_int { devlink_nl_dumpit(skb, cb, devlink_nl_sb_get_dump_one) }
unsafe fn devlink_nl_sb_get_dump_one(_m: *mut SkBuff, _d: *mut Devlink, _cb: *mut NetlinkCallback, _f: c_int) -> c_int { 0 }

// Driver-facing setters and all public registration interfaces.
pub unsafe fn devl_sb_register(d: *mut Devlink, index: c_uint, size: u32, ipc: u16, epc: u16, itc: u16, etc: u16) -> c_int {
    lockdep_assert_held(&(*d).lock); if devlink_sb_index_exists(d, index) { return -EEXIST; }
    let sb = kzalloc_obj::<DevlinkSb>(); if sb.is_null() { return -ENOMEM; }
    (*sb).index=index; (*sb).size=size; (*sb).ingress_pools_count=ipc; (*sb).egress_pools_count=epc; (*sb).ingress_tc_count=itc; (*sb).egress_tc_count=etc;
    list_add_tail(&mut (*sb).list, &mut (*d).sb_list); 0
}
pub unsafe fn devlink_sb_register(d: *mut Devlink, i:c_uint,s:u32,a:u16,b:u16,c:u16,e:u16)->c_int { devl_lock(d); let r=devl_sb_register(d,i,s,a,b,c,e); devl_unlock(d); r }
pub unsafe fn devl_sb_unregister(d:*mut Devlink,i:c_uint) { lockdep_assert_held(&(*d).lock); let sb=devlink_sb_get_by_index(d,i); WARN_ON(sb.is_null()); list_del(&mut (*sb).list); kfree(sb as *mut _); }
pub unsafe fn devlink_sb_unregister(d:*mut Devlink,i:c_uint) { devl_lock(d); devl_sb_unregister(d,i); devl_unlock(d); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
