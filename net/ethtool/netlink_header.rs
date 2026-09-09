/* SPDX-License-Identifier: GPL-2.0-only */

/* Translated from netlink.h. Kernel-provided types and functions are external dependencies. */

unsafe extern "C" {
    pub fn ethnl_parse_header_dev_get(
        req_info: *mut ethnl_req_info,
        nest: *const nlattr,
        net: *mut net,
        extack: *mut netlink_ext_ack,
        require_dev: bool,
    ) -> i32;
    pub fn ethnl_fill_reply_header(skb: *mut sk_buff, dev: *mut net_device, attrtype: u16) -> i32;
    pub fn ethnl_reply_init(
        payload: usize,
        dev: *mut net_device,
        cmd: u8,
        hdr_attrtype: u16,
        info: *mut genl_info,
        ehdrp: *mut *mut core::ffi::c_void,
    ) -> *mut sk_buff;
    pub fn ethnl_dump_put(skb: *mut sk_buff, cb: *mut netlink_callback, cmd: u8) -> *mut core::ffi::c_void;
    pub fn ethnl_bcastmsg_put(skb: *mut sk_buff, cmd: u8) -> *mut core::ffi::c_void;
    pub fn ethnl_unicast_put(skb: *mut sk_buff, portid: u32, seq: u32, cmd: u8) -> *mut core::ffi::c_void;
    pub fn ethnl_multicast(skb: *mut sk_buff, dev: *mut net_device) -> i32;
    pub fn ethnl_notify(dev: *mut net_device, cmd: u32, req_info: *const ethnl_req_info);

    pub fn nla_total_size(payload: usize) -> u32;
    pub fn strnlen(s: *const core::ffi::c_char, maxlen: usize) -> usize;
    pub fn nla_reserve(skb: *mut sk_buff, attrtype: u16, attrlen: usize) -> *mut nlattr;
    pub fn nla_data(attr: *mut nlattr) -> *mut core::ffi::c_void;
    pub fn nla_get_u32(attr: *const nlattr) -> u32;
    pub fn nla_get_u8(attr: *const nlattr) -> u8;
    pub fn nla_len(attr: *const nlattr) -> usize;
    pub fn nla_get_bitfield32(attr: *const nlattr) -> nla_bitfield32;
    pub fn netdev_put(dev: *mut net_device, tracker: *mut netdevice_tracker);
    pub fn ethnl_req_get_phydev(req_info: *const ethnl_req_info, tb: *mut *mut nlattr, header: u32, extack: *mut netlink_ext_ack) -> *mut phy_device;
    pub fn ethnl_ops_begin(dev: *mut net_device) -> i32;
    pub fn ethnl_ops_complete(dev: *mut net_device);
    pub fn ethnl_sock_priv_set(skb: *mut sk_buff, net: *mut net, portid: u32, ty: ethnl_sock_type) -> i32;
}

#[inline]
pub unsafe fn ethnl_strz_size(s: *const core::ffi::c_char) -> i32 {
    nla_total_size(strnlen(s, ETH_GSTRING_LEN as usize) + 1) as i32
}

#[inline]
pub unsafe fn ethnl_put_strz(skb: *mut sk_buff, attrtype: u16, s: *const core::ffi::c_char) -> i32 {
    let len = strnlen(s, ETH_GSTRING_LEN as usize);
    let attr = nla_reserve(skb, attrtype, len + 1);
    if attr.is_null() { return -EMSGSIZE; }
    core::ptr::copy_nonoverlapping(s as *const u8, nla_data(attr) as *mut u8, len);
    *(nla_data(attr) as *mut u8).add(len) = 0;
    0
}

#[inline]
pub unsafe fn ethnl_update_u32(dst: *mut u32, attr: *const nlattr, mod_: *mut bool) {
    if attr.is_null() { return; }
    let val = nla_get_u32(attr);
    if *dst == val { return; }
    *dst = val; *mod_ = true;
}

#[inline]
pub unsafe fn ethnl_update_u8(dst: *mut u8, attr: *const nlattr, mod_: *mut bool) {
    if attr.is_null() { return; }
    let val = nla_get_u8(attr);
    if *dst == val { return; }
    *dst = val; *mod_ = true;
}

#[inline]
pub unsafe fn ethnl_update_u8_u32(dst: *mut u8, attr: *const nlattr, mod_: *mut bool) {
    if attr.is_null() { return; }
    let val = nla_get_u32(attr);
    if val > u8::MAX as u32 { DEBUG_NET_WARN_ON_ONCE(val > u8::MAX as u32); }
    if *dst as u32 == val { return; }
    *dst = val as u8; *mod_ = true;
}

#[inline]
pub unsafe fn ethnl_update_bool32(dst: *mut u32, attr: *const nlattr, mod_: *mut bool) {
    if attr.is_null() { return; }
    let val = (nla_get_u8(attr) != 0) as u8;
    if (*dst != 0) == (val != 0) { return; }
    *dst = val as u32; *mod_ = true;
}

#[inline]
pub unsafe fn ethnl_update_bool(dst: *mut bool, attr: *const nlattr, mod_: *mut bool) {
    if attr.is_null() { return; }
    let val = nla_get_u8(attr) != 0;
    if *dst == val { return; }
    *dst = val; *mod_ = true;
}

#[inline]
pub unsafe fn ethnl_update_binary(dst: *mut core::ffi::c_void, mut len: usize, attr: *const nlattr, mod_: *mut bool) {
    if attr.is_null() { return; }
    let alen = nla_len(attr); if alen < len { len = alen; }
    if core::slice::from_raw_parts(dst as *const u8, len) == core::slice::from_raw_parts(nla_data(attr) as *const u8, len) { return; }
    core::ptr::copy_nonoverlapping(nla_data(attr) as *const u8, dst as *mut u8, len); *mod_ = true;
}

#[inline]
pub unsafe fn ethnl_update_bitfield32(dst: *mut u32, attr: *const nlattr, mod_: *mut bool) {
    if attr.is_null() { return; }
    let change = nla_get_bitfield32(attr);
    let newval = (*dst & !change.selector) | (change.value & change.selector);
    if *dst == newval { return; }
    *dst = newval; *mod_ = true;
}

#[inline]
pub unsafe fn ethnl_reply_header_size() -> u32 {
    nla_total_size((nla_total_size(core::mem::size_of::<u32>()) + nla_total_size(IFNAMSIZ as usize)) as usize)
}

#[repr(C)]
pub struct ethnl_req_info {
    pub dev: *mut net_device,
    pub dev_tracker: netdevice_tracker,
    pub flags: u32,
    pub phy_index: u32,
}

#[inline]
pub unsafe fn ethnl_parse_header_dev_put(req_info: *mut ethnl_req_info) {
    netdev_put((*req_info).dev, &mut (*req_info).dev_tracker);
}

#[repr(C)]
pub struct ethnl_reply_data { pub dev: *mut net_device }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ethnl_sock_type { ETHTOOL_SOCK_TYPE_MODULE_FW_FLASH }

#[repr(C)]
pub struct ethnl_sock_priv { pub net: *mut net, pub portid: u32, pub type_: ethnl_sock_type }

#[repr(C)]
pub struct ethnl_request_ops {
    pub request_cmd: u8, pub reply_cmd: u8, pub hdr_attr: u16,
    pub req_info_size: u32, pub reply_data_size: u32,
    pub allow_nodev_do: bool, pub set_ntf_cmd: u8,
    pub parse_request: Option<unsafe extern "C" fn(*mut ethnl_req_info, *const genl_info, *mut *mut nlattr, *mut netlink_ext_ack) -> i32>,
    pub prepare_data: Option<unsafe extern "C" fn(*const ethnl_req_info, *mut ethnl_reply_data, *const genl_info) -> i32>,
    pub reply_size: Option<unsafe extern "C" fn(*const ethnl_req_info, *const ethnl_reply_data) -> i32>,
    pub fill_reply: Option<unsafe extern "C" fn(*mut sk_buff, *const ethnl_req_info, *const ethnl_reply_data) -> i32>,
    pub cleanup_data: Option<unsafe extern "C" fn(*mut ethnl_reply_data)>,
    pub set_validate: Option<unsafe extern "C" fn(*mut ethnl_req_info, *mut genl_info) -> i32>,
    pub set: Option<unsafe extern "C" fn(*mut ethnl_req_info, *mut genl_info) -> i32>,
}

unsafe extern "C" {
    pub static ethnl_strset_request_ops: ethnl_request_ops;
    pub static ethnl_linkinfo_request_ops: ethnl_request_ops;
    pub static ethnl_linkmodes_request_ops: ethnl_request_ops;
    pub static ethnl_linkstate_request_ops: ethnl_request_ops;
    pub static ethnl_debug_request_ops: ethnl_request_ops;
    pub static ethnl_wol_request_ops: ethnl_request_ops;
    pub static ethnl_features_request_ops: ethnl_request_ops;
    pub static ethnl_privflags_request_ops: ethnl_request_ops;
    pub static ethnl_rings_request_ops: ethnl_request_ops;
    pub static ethnl_channels_request_ops: ethnl_request_ops;
    pub static ethnl_coalesce_request_ops: ethnl_request_ops;
    pub static ethnl_pause_request_ops: ethnl_request_ops;
    pub static ethnl_eee_request_ops: ethnl_request_ops;
    pub static ethnl_tsinfo_request_ops: ethnl_request_ops;
    pub static ethnl_fec_request_ops: ethnl_request_ops;
    pub static ethnl_module_eeprom_request_ops: ethnl_request_ops;
    pub static ethnl_stats_request_ops: ethnl_request_ops;
    pub static ethnl_phc_vclocks_request_ops: ethnl_request_ops;
    pub static ethnl_module_request_ops: ethnl_request_ops;
    pub static ethnl_pse_request_ops: ethnl_request_ops;
    pub static ethnl_rss_request_ops: ethnl_request_ops;
    pub static ethnl_plca_cfg_request_ops: ethnl_request_ops;
    pub static ethnl_plca_status_request_ops: ethnl_request_ops;
    pub static ethnl_mm_request_ops: ethnl_request_ops;
    pub static ethnl_phy_request_ops: ethnl_request_ops;
    pub static ethnl_tsconfig_request_ops: ethnl_request_ops;
    pub static ethnl_mse_request_ops: ethnl_request_ops;
    pub static ethnl_header_policy: [nla_policy; ETHTOOL_A_HEADER_FLAGS as usize + 1];
    pub static ethnl_header_policy_stats: [nla_policy; ETHTOOL_A_HEADER_FLAGS as usize + 1];
    pub static ethnl_header_policy_phy: [nla_policy; ETHTOOL_A_HEADER_PHY_INDEX as usize + 1];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
