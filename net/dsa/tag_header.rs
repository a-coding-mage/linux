/* SPDX-License-Identifier: GPL-2.0-or-later */

// Dependencies supplied by the surrounding kernel/DSA translation.

#[repr(C)]
pub struct dsa_tag_driver {
    pub ops: *const dsa_device_ops,
    pub list: list_head,
    pub owner: *mut module,
}

extern "C" {
    pub static mut dsa_pack_type: packet_type;

    pub fn dsa_tag_driver_get_by_id(tag_protocol: ::core::ffi::c_int)
        -> *const dsa_device_ops;
    pub fn dsa_tag_driver_get_by_name(name: *const ::core::ffi::c_char)
        -> *const dsa_device_ops;
    pub fn dsa_tag_driver_put(ops: *const dsa_device_ops);
    pub fn dsa_tag_protocol_to_str(ops: *const dsa_device_ops)
        -> *const ::core::ffi::c_char;
}

#[inline]
pub unsafe fn dsa_tag_protocol_overhead(ops: *const dsa_device_ops) -> ::core::ffi::c_int {
    (*ops).needed_headroom + (*ops).needed_tailroom
}

#[inline]
pub unsafe fn dsa_conduit_find_user(
    dev: *mut net_device,
    device: ::core::ffi::c_int,
    port: ::core::ffi::c_int,
) -> *mut net_device {
    let cpu_dp = (*dev).dsa_ptr;
    let dst = (*cpu_dp).dst;
    let mut dp: *mut dsa_port;

    list_for_each_entry!(dp, &mut (*dst).ports, list);
    if (*dp).ds.index == device && (*dp).index == port
        && (*dp).type_ == DSA_PORT_TYPE_USER
    {
        return (*dp).user;
    }

    core::ptr::null_mut()
}

#[inline]
pub unsafe fn dsa_software_untag_vlan_aware_bridge(
    skb: *mut sk_buff,
    br: *mut net_device,
    vid: u16,
) {
    let mut pvid: u16 = 0;
    let mut proto: u16 = 0;
    let mut err: ::core::ffi::c_int;

    err = br_vlan_get_proto(br, &mut proto);
    if err != 0 { return; }

    err = br_vlan_get_pvid_rcu((*skb).dev, &mut pvid);
    if err != 0 { return; }

    if vid == pvid && (*skb).vlan_proto == htons(proto) {
        __vlan_hwaccel_clear_tag(skb);
    }
}

#[inline]
pub unsafe fn dsa_software_untag_vlan_unaware_bridge(
    skb: *mut sk_buff,
    br: *mut net_device,
    vid: u16,
) {
    let mut upper_dev: *mut net_device;
    let mut pvid: u16 = 0;
    let mut proto: u16 = 0;
    let mut err: ::core::ffi::c_int;

    err = br_vlan_get_proto(br, &mut proto);
    if err != 0 { return; }

    err = br_vlan_get_pvid_rcu((*skb).dev, &mut pvid);
    if err != 0 { return; }

    if vid != pvid || (*skb).vlan_proto != htons(proto) { return; }

    // Preserve the source commentary: retain the tag if an 8021q upper would
    // steal VLAN-tagged traffic from the bridge data path.
    upper_dev = __vlan_find_dev_deep_rcu(br, htons(proto), vid);
    if upper_dev.is_null() {
        __vlan_hwaccel_clear_tag(skb);
    }
}

#[inline]
pub unsafe fn dsa_software_vlan_untag(mut skb: *mut sk_buff) -> *mut sk_buff {
    let dp = dsa_user_to_port((*skb).dev);
    let br = dsa_port_bridge_dev_get(dp);
    let mut vid: u16 = 0;
    let mut proto: u16 = 0;
    let mut err: ::core::ffi::c_int;

    if br.is_null() { return skb; }

    err = br_vlan_get_proto(br, &mut proto);
    if err != 0 { return skb; }

    if !skb_vlan_tag_present(skb) && (*skb).protocol == htons(proto) {
        skb = skb_vlan_untag(skb);
        if skb.is_null() { return core::ptr::null_mut(); }
    }

    if !skb_vlan_tag_present(skb) { return skb; }
    vid = skb_vlan_tag_get_id(skb);

    if br_vlan_enabled(br) {
        if (*(*dp).ds).untag_vlan_aware_bridge_pvid {
            dsa_software_untag_vlan_aware_bridge(skb, br, vid);
        }
    } else if (*(*dp).ds).untag_bridge_pvid {
        dsa_software_untag_vlan_unaware_bridge(skb, br, vid);
    }

    skb
}

#[inline]
pub unsafe fn dsa_find_designated_bridge_port_by_vid(
    conduit: *mut net_device,
    vid: u16,
) -> *mut net_device {
    let cpu_dp = (*conduit).dsa_ptr;
    let dst = (*cpu_dp).dst;
    let mut vinfo: bridge_vlan_info;
    let mut user: *mut net_device;
    let mut dp: *mut dsa_port;
    let mut err: ::core::ffi::c_int;

    list_for_each_entry!(dp, &mut (*dst).ports, list);
    if (*dp).type_ != DSA_PORT_TYPE_USER || (*dp).bridge.is_null() { continue; }
    if (*dp).stp_state != BR_STATE_LEARNING && (*dp).stp_state != BR_STATE_FORWARDING { continue; }
    if (*dp).cpu_dp != cpu_dp { continue; }

    user = (*dp).user;
    err = br_vlan_get_info_rcu(user, vid, &mut vinfo);
    if err != 0 { continue; }
    return user;

    core::ptr::null_mut()
}

#[inline]
pub unsafe fn dsa_default_offload_fwd_mark(skb: *mut sk_buff) {
    let dp = dsa_user_to_port((*skb).dev);
    (*skb).offload_fwd_mark = if !(*dp).bridge.is_null() { 1 } else { 0 };
}

#[inline]
pub unsafe fn dsa_strip_etype_header(skb: *mut sk_buff, len: ::core::ffi::c_int) {
    memmove((*skb).data.offset(-(ETH_HLEN as isize)),
            (*skb).data.offset(-(ETH_HLEN as isize) - len as isize),
            (2 * ETH_ALEN) as usize);
}

#[inline]
pub unsafe fn dsa_alloc_etype_header(skb: *mut sk_buff, len: ::core::ffi::c_int) {
    memmove((*skb).data, (*skb).data.offset(len as isize), (2 * ETH_ALEN) as usize);
}

#[inline]
pub unsafe fn dsa_etype_header_pos_rx(skb: *mut sk_buff) -> *mut ::core::ffi::c_void {
    (*skb).data.offset(-2) as *mut _
}

#[inline]
pub unsafe fn dsa_etype_header_pos_tx(skb: *mut sk_buff) -> *mut ::core::ffi::c_void {
    (*skb).data.add(2 * ETH_ALEN) as *mut _
}

#[inline]
pub unsafe fn dsa_xmit_port_mask(skb: *const sk_buff, dev: *const net_device) -> ::core::ffi::c_ulong {
    let dp = dsa_user_to_port(dev as *mut _);
    let mut mask = BIT((*dp).index);
    if IS_ENABLED!(CONFIG_HSR) && unlikely!((*dev).features & NETIF_F_HW_HSR_DUP != 0) {
        let hsr_dev = (*dp).hsr_dev;
        let mut other_dp: *mut dsa_port;
        dsa_hsr_foreach_port!(other_dp, (*dp).ds, hsr_dev);
        mask |= BIT((*other_dp).index);
    }
    mask
}

pub const DSA_TAG_DRIVER_ALIAS: &str = "dsa_tag:";

#[macro_export]
macro_rules! MODULE_ALIAS_DSA_TAG_DRIVER {
    ($proto:ident, $name:literal) => {
        MODULE_ALIAS!(concat!("dsa_tag:", $name));
        MODULE_ALIAS!(concat!("dsa_tag:id-", stringify!($proto), "_VALUE"));
    };
}

extern "C" {
    pub fn dsa_tag_drivers_register(
        dsa_tag_driver_array: *mut *mut dsa_tag_driver,
        count: ::core::ffi::c_uint,
        owner: *mut module,
    );
    pub fn dsa_tag_drivers_unregister(
        dsa_tag_driver_array: *mut *mut dsa_tag_driver,
        count: ::core::ffi::c_uint,
    );
}

// The remaining registration helpers are retained as declarative macro intent
// because their C module-init/module-exit machinery is supplied by the build.
#[macro_export]
macro_rules! DSA_TAG_DRIVER_NAME { ($ops:ident) => { paste::paste! { [<dsa_tag_driver_ $ops>] } }; }

#[macro_export]
macro_rules! DSA_TAG_DRIVER {
    ($ops:ident) => { static mut DSA_TAG_DRIVER_NAME!($ops): dsa_tag_driver = dsa_tag_driver { ops: &$ops, list: core::mem::zeroed(), owner: core::ptr::null_mut() }; };
}

#[macro_export]
macro_rules! module_dsa_tag_drivers { ($array:expr) => { dsa_tag_drivers_register($array.as_mut_ptr(), $array.len() as _, THIS_MODULE); }; }

#[macro_export]
macro_rules! module_dsa_tag_driver { ($ops:ident) => { DSA_TAG_DRIVER!($ops); }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
