/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct nfnl_info {
    pub net: *mut net,
    pub sk: *mut sock,
    pub nlh: *const nlmsghdr,
    pub nfmsg: *const nfgenmsg,
    pub extack: *mut netlink_ext_ack,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum nfnl_callback_type {
    NFNL_CB_UNSPEC = 0,
    NFNL_CB_MUTEX,
    NFNL_CB_RCU,
    NFNL_CB_BATCH,
}

#[repr(C)]
pub struct nfnl_callback {
    pub call: Option<unsafe extern "C" fn(skb: *mut sk_buff, info: *const nfnl_info, cda: *const *const nlattr) -> c_int>,
    pub policy: *const nla_policy,
    pub type_: nfnl_callback_type,
    pub attr_count: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum nfnl_abort_action {
    NFNL_ABORT_NONE = 0,
    NFNL_ABORT_AUTOLOAD,
    NFNL_ABORT_VALIDATE,
}

#[repr(C)]
pub struct nfnetlink_subsystem {
    pub name: *const c_char,
    pub subsys_id: __u8,
    pub cb_count: __u8,
    pub cb: *const nfnl_callback,
    pub owner: *mut module,
    pub commit: Option<unsafe extern "C" fn(net: *mut net, skb: *mut sk_buff) -> c_int>,
    pub abort: Option<unsafe extern "C" fn(net: *mut net, skb: *mut sk_buff, action: nfnl_abort_action) -> c_int>,
    pub valid_genid: Option<unsafe extern "C" fn(net: *mut net, genid: u32) -> bool>,
}

extern "C" {
    pub fn nfnetlink_subsys_register(n: *const nfnetlink_subsystem) -> c_int;
    pub fn nfnetlink_subsys_unregister(n: *const nfnetlink_subsystem) -> c_int;

    pub fn nfnetlink_has_listeners(net: *mut net, group: c_uint) -> c_int;
    pub fn nfnetlink_send(skb: *mut sk_buff, net: *mut net, portid: u32,
                          group: c_uint, echo: c_int, flags: gfp_t) -> c_int;
    pub fn nfnetlink_set_err(net: *mut net, portid: u32, group: u32, error: c_int) -> c_int;
    pub fn nfnetlink_unicast(skb: *mut sk_buff, net: *mut net, portid: u32) -> c_int;
    pub fn nfnetlink_broadcast(net: *mut net, skb: *mut sk_buff, portid: __u32,
                               group: __u32, allocation: gfp_t);

    pub fn nfnl_lock(subsys_id: __u8);
    pub fn nfnl_unlock(subsys_id: __u8);
}

#[inline]
pub unsafe fn nfnl_msg_type(subsys: u8, msg_type: u8) -> u16 {
    ((subsys as u16) << 8) | msg_type as u16
}

#[inline]
pub unsafe fn nfnl_fill_hdr(nlh: *mut nlmsghdr, family: u8, version: u8, res_id: __be16) {
    let nfmsg = nlmsg_data(nlh) as *mut nfgenmsg;
    (*nfmsg).nfgen_family = family;
    (*nfmsg).version = version;
    (*nfmsg).res_id = res_id;
}

#[inline]
pub unsafe fn nfnl_msg_put(skb: *mut sk_buff, portid: u32, seq: u32, type_: c_int,
                           flags: c_int, family: u8, version: u8, res_id: __be16) -> *mut nlmsghdr {
    let nlh = nlmsg_put(skb, portid, seq, type_, core::mem::size_of::<nfgenmsg>() as c_int, flags);
    if nlh.is_null() {
        return core::ptr::null_mut();
    }
    nfnl_fill_hdr(nlh, family, version, res_id);
    nlh
}

#[cfg(feature = "CONFIG_PROVE_LOCKING")]
extern "C" {
    pub fn lockdep_nfnl_is_held(subsys_id: __u8) -> bool;
}

#[cfg(not(feature = "CONFIG_PROVE_LOCKING"))]
#[inline]
pub unsafe fn lockdep_nfnl_is_held(_subsys_id: __u8) -> bool {
    true
}

// MODULE_ALIAS_NFNL_SUBSYS(subsys) expands to MODULE_ALIAS("nfnetlink-subsys-" __stringify(subsys)).

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
