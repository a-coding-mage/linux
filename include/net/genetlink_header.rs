/* SPDX-License-Identifier: GPL-2.0 */

// External kernel dependencies are supplied by the surrounding translation.

pub const GENLMSG_DEFAULT_SIZE: usize = NLMSG_DEFAULT_SIZE - GENL_HDRLEN;

/* Non-parallel generic netlink requests are serialized by a global lock. */
unsafe extern "C" {
    pub fn genl_lock();
    pub fn genl_unlock();
}

pub const GENL_MCAST_CAP_NET_ADMIN: u8 = BIT(0);
pub const GENL_MCAST_CAP_SYS_ADMIN: u8 = BIT(1);

#[repr(C)]
pub struct genl_multicast_group {
    pub name: [core::ffi::c_char; GENL_NAMSIZ],
    pub flags: u8,
}

#[repr(C)]
pub struct genl_family {
    pub hdrsize: core::ffi::c_uint,
    pub name: [core::ffi::c_char; GENL_NAMSIZ],
    pub version: core::ffi::c_uint,
    pub maxattr: core::ffi::c_uint,
    pub netnsok: u8,
    pub parallel_ops: u8,
    pub n_ops: u8,
    pub n_small_ops: u8,
    pub n_split_ops: u8,
    pub n_mcgrps: u8,
    pub resv_start_op: u8,
    pub policy: *const nla_policy,
    pub pre_doit: Option<unsafe extern "C" fn(*const genl_split_ops, *mut sk_buff, *mut genl_info) -> core::ffi::c_int>,
    pub post_doit: Option<unsafe extern "C" fn(*const genl_split_ops, *mut sk_buff, *mut genl_info)>,
    pub bind: Option<unsafe extern "C" fn(core::ffi::c_int) -> core::ffi::c_int>,
    pub unbind: Option<unsafe extern "C" fn(core::ffi::c_int)>,
    pub ops: *const genl_ops,
    pub small_ops: *const genl_small_ops,
    pub split_ops: *const genl_split_ops,
    pub mcgrps: *const genl_multicast_group,
    pub module: *mut module,
    pub sock_priv_size: usize,
    pub sock_priv_init: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    pub sock_priv_destroy: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    pub id: core::ffi::c_int,
    pub mcgrp_offset: core::ffi::c_uint,
    pub sock_privs: *mut xarray,
}

#[repr(C)]
pub struct genl_info {
    pub snd_seq: u32,
    pub snd_portid: u32,
    pub family: *const genl_family,
    pub nlhdr: *const nlmsghdr,
    pub genlhdr: *mut genlmsghdr,
    pub attrs: *mut *mut nlattr,
    pub _net: possible_net_t,
    pub user: genl_info_user_union,
    pub extack: *mut netlink_ext_ack,
}

#[repr(C)]
pub union genl_info_user_union {
    pub ctx: [u8; NETLINK_CTX_SIZE],
    pub user_ptr: [*mut core::ffi::c_void; 2],
}

#[inline]
pub unsafe fn genl_info_net(info: *const genl_info) -> *mut net {
    read_pnet(unsafe { &(*info)._net })
}

#[inline]
pub unsafe fn genl_info_net_set(info: *mut genl_info, net: *mut net) {
    write_pnet(unsafe { &mut (*info)._net }, net);
}

#[inline]
pub unsafe fn genl_info_userhdr(info: *const genl_info) -> *mut core::ffi::c_void {
    (unsafe { (*info).genlhdr.cast::<u8>().add(GENL_HDRLEN) }).cast()
}

#[inline]
pub unsafe fn genl_req_attr_check(info: *mut genl_info, attr: core::ffi::c_int) -> core::ffi::c_int {
    let i = info;
    NL_REQ_ATTR_CHECK(unsafe { (*i).extack }, core::ptr::null_mut(), unsafe { (*i).attrs }, attr)
}

#[repr(i32)]
pub enum genl_validate_flags {
    GENL_DONT_VALIDATE_STRICT = BIT(0) as i32,
    GENL_DONT_VALIDATE_DUMP = BIT(1) as i32,
    GENL_DONT_VALIDATE_DUMP_STRICT = BIT(2) as i32,
}

#[repr(C)]
pub struct genl_small_ops {
    pub doit: Option<unsafe extern "C" fn(*mut sk_buff, *mut genl_info) -> core::ffi::c_int>,
    pub dumpit: Option<unsafe extern "C" fn(*mut sk_buff, *mut netlink_callback) -> core::ffi::c_int>,
    pub cmd: u8,
    pub internal_flags: u8,
    pub flags: u8,
    pub validate: u8,
}

#[repr(C)]
pub struct genl_ops {
    pub doit: Option<unsafe extern "C" fn(*mut sk_buff, *mut genl_info) -> core::ffi::c_int>,
    pub start: Option<unsafe extern "C" fn(*mut netlink_callback) -> core::ffi::c_int>,
    pub dumpit: Option<unsafe extern "C" fn(*mut sk_buff, *mut netlink_callback) -> core::ffi::c_int>,
    pub done: Option<unsafe extern "C" fn(*mut netlink_callback) -> core::ffi::c_int>,
    pub policy: *const nla_policy,
    pub maxattr: core::ffi::c_uint,
    pub cmd: u8,
    pub internal_flags: u8,
    pub flags: u8,
    pub validate: u8,
}

#[repr(C)]
pub union genl_split_ops_callbacks {
    pub doit: genl_split_ops_do,
    pub dump: genl_split_ops_dump,
}
#[repr(C)]
pub struct genl_split_ops_do {
    pub pre_doit: Option<unsafe extern "C" fn(*const genl_split_ops, *mut sk_buff, *mut genl_info) -> core::ffi::c_int>,
    pub doit: Option<unsafe extern "C" fn(*mut sk_buff, *mut genl_info) -> core::ffi::c_int>,
    pub post_doit: Option<unsafe extern "C" fn(*const genl_split_ops, *mut sk_buff, *mut genl_info)>,
}
#[repr(C)]
pub struct genl_split_ops_dump {
    pub start: Option<unsafe extern "C" fn(*mut netlink_callback) -> core::ffi::c_int>,
    pub dumpit: Option<unsafe extern "C" fn(*mut sk_buff, *mut netlink_callback) -> core::ffi::c_int>,
    pub done: Option<unsafe extern "C" fn(*mut netlink_callback) -> core::ffi::c_int>,
}
#[repr(C)]
pub struct genl_split_ops {
    pub callbacks: genl_split_ops_callbacks,
    pub policy: *const nla_policy,
    pub maxattr: core::ffi::c_uint,
    pub cmd: u8,
    pub internal_flags: u8,
    pub flags: u8,
    pub validate: u8,
}

#[repr(C)]
pub struct genl_dumpit_info { pub op: genl_split_ops, pub info: genl_info }

#[inline]
pub unsafe fn genl_dumpit_info(cb: *mut netlink_callback) -> *const genl_dumpit_info { unsafe { (*cb).data.cast() } }
#[inline]
pub unsafe fn genl_info_dump(cb: *mut netlink_callback) -> *const genl_info { unsafe { &(*genl_dumpit_info(cb)).info } }

#[inline]
pub unsafe fn genl_info_init_ntf(info: *mut genl_info, family: *const genl_family, cmd: u8) {
    let hdr = unsafe { (*info).user.user_ptr.as_mut_ptr().cast::<genlmsghdr>() };
    memset(info.cast(), 0, core::mem::size_of::<genl_info>());
    unsafe { (*info).family = family; (*info).genlhdr = hdr; (*hdr).cmd = cmd; }
}
#[inline]
pub unsafe fn genl_info_is_ntf(info: *const genl_info) -> bool { unsafe { (*info).nlhdr.is_null() } }

unsafe extern "C" {
    pub fn __genl_sk_priv_get(family: *mut genl_family, sk: *mut sock) -> *mut core::ffi::c_void;
    pub fn genl_sk_priv_get(family: *mut genl_family, sk: *mut sock) -> *mut core::ffi::c_void;
    pub fn genl_register_family(family: *mut genl_family) -> core::ffi::c_int;
    pub fn genl_unregister_family(family: *const genl_family) -> core::ffi::c_int;
    pub fn genl_notify(family: *const genl_family, skb: *mut sk_buff, info: *mut genl_info, group: u32, flags: gfp_t);
    pub fn genlmsg_put(skb: *mut sk_buff, portid: u32, seq: u32, family: *const genl_family, flags: core::ffi::c_int, cmd: u8) -> *mut core::ffi::c_void;
}

#[inline]
pub unsafe fn __genlmsg_iput(skb: *mut sk_buff, info: *const genl_info, flags: core::ffi::c_int) -> *mut core::ffi::c_void {
    unsafe { genlmsg_put(skb, (*info).snd_portid, (*info).snd_seq, (*info).family, flags, (*(*info).genlhdr).cmd) }
}
#[inline]
pub unsafe fn genlmsg_iput(skb: *mut sk_buff, info: *const genl_info) -> *mut core::ffi::c_void { unsafe { __genlmsg_iput(skb, info, 0) } }
#[inline]
pub unsafe fn genlmsg_nlhdr(user_hdr: *mut core::ffi::c_void) -> *mut nlmsghdr { unsafe { user_hdr.cast::<u8>().sub(GENL_HDRLEN + NLMSG_HDRLEN).cast() } }

#[inline]
pub unsafe fn genlmsg_parse_deprecated(nlh: *const nlmsghdr, family: *const genl_family, tb: *mut *mut nlattr, maxtype: core::ffi::c_int, policy: *const nla_policy, extack: *mut netlink_ext_ack) -> core::ffi::c_int {
    unsafe { __nlmsg_parse(nlh, (*family).hdrsize + GENL_HDRLEN, tb, maxtype, policy, NL_VALIDATE_LIBERAL, extack) }
}
#[inline]
pub unsafe fn genlmsg_parse(nlh: *const nlmsghdr, family: *const genl_family, tb: *mut *mut nlattr, maxtype: core::ffi::c_int, policy: *const nla_policy, extack: *mut netlink_ext_ack) -> core::ffi::c_int {
    unsafe { __nlmsg_parse(nlh, (*family).hdrsize + GENL_HDRLEN, tb, maxtype, policy, NL_VALIDATE_STRICT, extack) }
}
#[inline]
pub unsafe fn genl_dump_check_consistent(cb: *mut netlink_callback, user_hdr: *mut core::ffi::c_void) { unsafe { nl_dump_check_consistent(cb, genlmsg_nlhdr(user_hdr)); } }
#[inline]
pub unsafe fn genlmsg_put_reply(skb: *mut sk_buff, info: *mut genl_info, family: *const genl_family, flags: core::ffi::c_int, cmd: u8) -> *mut core::ffi::c_void { unsafe { genlmsg_put(skb, (*info).snd_portid, (*info).snd_seq, family, flags, cmd) } }
#[inline]
pub unsafe fn genlmsg_end(skb: *mut sk_buff, hdr: *mut core::ffi::c_void) { unsafe { nlmsg_end(skb, hdr.cast::<u8>().sub(GENL_HDRLEN + NLMSG_HDRLEN).cast()); } }
#[inline]
pub unsafe fn genlmsg_cancel(skb: *mut sk_buff, hdr: *mut core::ffi::c_void) { if !hdr.is_null() { unsafe { nlmsg_cancel(skb, hdr.cast::<u8>().sub(GENL_HDRLEN + NLMSG_HDRLEN).cast()); } } }

#[inline]
pub unsafe fn genlmsg_multicast_netns_filtered(family: *const genl_family, net: *mut net, skb: *mut sk_buff, portid: u32, mut group: core::ffi::c_uint, flags: gfp_t, filter: netlink_filter_fn, filter_data: *mut core::ffi::c_void) -> core::ffi::c_int {
    if WARN_ON_ONCE(group >= unsafe { (*family).n_mcgrps }) { unsafe { nlmsg_free(skb); } return -EINVAL; }
    group += unsafe { (*family).mcgrp_offset };
    unsafe { nlmsg_multicast_filtered((*net).genl_sock, skb, portid, group, flags, filter, filter_data) }
}
#[inline]
pub unsafe fn genlmsg_multicast_netns(family: *const genl_family, net: *mut net, skb: *mut sk_buff, portid: u32, group: core::ffi::c_uint, flags: gfp_t) -> core::ffi::c_int { unsafe { genlmsg_multicast_netns_filtered(family, net, skb, portid, group, flags, None, core::ptr::null_mut()) } }
#[inline]
pub unsafe fn genlmsg_multicast(family: *const genl_family, skb: *mut sk_buff, portid: u32, group: core::ffi::c_uint, flags: gfp_t) -> core::ffi::c_int { unsafe { genlmsg_multicast_netns(family, &mut init_net, skb, portid, group, flags) } }
unsafe extern "C" { pub fn genlmsg_multicast_allns(family: *const genl_family, skb: *mut sk_buff, portid: u32, group: core::ffi::c_uint) -> core::ffi::c_int; }
#[inline]
pub unsafe fn genlmsg_unicast(net: *mut net, skb: *mut sk_buff, portid: u32) -> core::ffi::c_int { unsafe { nlmsg_unicast((*net).genl_sock, skb, portid) } }
#[inline]
pub unsafe fn genlmsg_reply(skb: *mut sk_buff, info: *mut genl_info) -> core::ffi::c_int { unsafe { genlmsg_unicast(genl_info_net(info), skb, (*info).snd_portid) } }
#[inline]
pub unsafe fn genlmsg_data(gnlh: *const genlmsghdr) -> *mut core::ffi::c_void { unsafe { gnlh.cast::<u8>().add(GENL_HDRLEN).cast() } }
#[inline]
pub unsafe fn genlmsg_len(gnlh: *const genlmsghdr) -> core::ffi::c_int { unsafe { ((*gnlh.cast::<u8>().sub(NLMSG_HDRLEN).cast::<nlmsghdr>()).nlmsg_len - GENL_HDRLEN - NLMSG_HDRLEN) as core::ffi::c_int } }
#[inline]
pub const fn genlmsg_msg_size(payload: core::ffi::c_int) -> core::ffi::c_int { GENL_HDRLEN as core::ffi::c_int + payload }
#[inline]
pub fn genlmsg_total_size(payload: core::ffi::c_int) -> core::ffi::c_int { NLMSG_ALIGN(genlmsg_msg_size(payload)) }
#[inline]
pub unsafe fn genlmsg_new(payload: usize, flags: gfp_t) -> *mut sk_buff { unsafe { nlmsg_new(genlmsg_total_size(payload as core::ffi::c_int) as usize, flags) } }
#[inline]
pub unsafe fn genl_set_err(family: *const genl_family, net: *mut net, portid: u32, mut group: u32, code: core::ffi::c_int) -> core::ffi::c_int { if WARN_ON_ONCE(group >= unsafe { (*family).n_mcgrps as u32 }) { return -EINVAL; } group += unsafe { (*family).mcgrp_offset }; unsafe { netlink_set_err((*net).genl_sock, portid, group, code) } }
#[inline]
pub unsafe fn genl_has_listeners(family: *const genl_family, net: *mut net, mut group: core::ffi::c_uint) -> core::ffi::c_int { if WARN_ON_ONCE(group >= unsafe { (*family).n_mcgrps as core::ffi::c_uint }) { return -EINVAL; } group += unsafe { (*family).mcgrp_offset }; unsafe { netlink_has_listeners((*net).genl_sock, group) } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
