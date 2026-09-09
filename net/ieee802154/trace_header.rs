/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of net/wireless-style cfg802154 trace declarations. */

// The C preprocessor selects TRACE_SYSTEM and includes the kernel tracepoint
// declaration machinery.  Those build-time conditions have no direct local
// Rust equivalent and are retained here as documentation.
// TRACE_SYSTEM = cfg802154
// __RDEV_CFG802154_OPS_TRACE / TRACE_HEADER_MULTI_READ guard

pub const MAXNAME: usize = 32;

#[inline]
pub const fn bool_to_str(value: bool) -> &'static str {
    if value { "true" } else { "false" }
}

// The following declarations preserve the kernel tracepoint interface.  The
// tracepoint implementation and the referenced kernel types are supplied by
// the surrounding system, just as they are for the original header.
macro_rules! trace_declaration {
    ($($item:tt)*) => {};
}

trace_declaration! {
    event_class wpan_phy_only_evt(
        wpan_phy: *mut wpan_phy,
        entry: { wpan_phy_name: [c_char; MAXNAME] },
        print: "{}", wpan_phy_name
    );
    event 802154_rdev_suspend from wpan_phy_only_evt;
    event 802154_rdev_resume from wpan_phy_only_evt;

    event 802154_rdev_add_virtual_intf(
        wpan_phy: *mut wpan_phy,
        name: *mut c_char,
        type_: nl802154_iftype,
        extended_addr: __le64,
        entry: {
            wpan_phy_name: [c_char; MAXNAME],
            vir_intf_name: *const c_char,
            type_: nl802154_iftype,
            extended_addr: __le64,
        },
        print: "{}, virtual intf name: {}, type: {}, extended addr: 0x{:x}"
    );

    event 802154_rdev_del_virtual_intf(
        wpan_phy: *mut wpan_phy,
        wpan_dev: *mut wpan_dev,
        entry: { wpan_phy_name: [c_char; MAXNAME], identifier: u32 },
        print: "{}, wpan_dev({})"
    );

    event 802154_rdev_set_channel(
        wpan_phy: *mut wpan_phy, page: u8, channel: u8,
        entry: { wpan_phy_name: [c_char; MAXNAME], page: u8, channel: u8 },
        print: "{}, page: {}, channel: {}"
    );
    event 802154_rdev_set_tx_power(
        wpan_phy: *mut wpan_phy, power: i32,
        entry: { wpan_phy_name: [c_char; MAXNAME], power: i32 },
        print: "{}, mbm: {}"
    );
    event 802154_rdev_set_cca_mode(
        wpan_phy: *mut wpan_phy, cca: *const wpan_phy_cca,
        entry: { wpan_phy_name: [c_char; MAXNAME], cca_mode: nl802154_cca_modes, cca_opt: nl802154_cca_opts },
        print: "{}, cca_mode: {}, cca_opt: {}"
    );
    event 802154_rdev_set_cca_ed_level(
        wpan_phy: *mut wpan_phy, ed_level: i32,
        entry: { wpan_phy_name: [c_char; MAXNAME], ed_level: i32 },
        print: "{}, ed level: {}"
    );

    event_class 802154_le16_template(
        wpan_phy: *mut wpan_phy, wpan_dev: *mut wpan_dev, le16arg: __le16,
        entry: { wpan_phy_name: [c_char; MAXNAME], identifier: u32, le16arg: __le16 },
        print: "{}, wpan_dev({}), pan id: 0x{:04x}"
    );
    event 802154_rdev_set_pan_id from 802154_le16_template;
    event 802154_rdev_set_short_addr from 802154_le16_template,
        print: "{}, wpan_dev({}), short addr: 0x{:04x}";

    event 802154_rdev_set_backoff_exponent(
        wpan_phy: *mut wpan_phy, wpan_dev: *mut wpan_dev, min_be: u8, max_be: u8,
        entry: { wpan_phy_name: [c_char; MAXNAME], identifier: u32, min_be: u8, max_be: u8 },
        print: "{}, wpan_dev({}), min be: {}, max be: {}"
    );
    event 802154_rdev_set_csma_backoffs(
        wpan_phy: *mut wpan_phy, wpan_dev: *mut wpan_dev, max_csma_backoffs: u8,
        entry: { wpan_phy_name: [c_char; MAXNAME], identifier: u32, max_csma_backoffs: u8 },
        print: "{}, wpan_dev({}), max csma backoffs: {}"
    );
    event 802154_rdev_set_max_frame_retries(
        wpan_phy: *mut wpan_phy, wpan_dev: *mut wpan_dev, max_frame_retries: i8,
        entry: { wpan_phy_name: [c_char; MAXNAME], identifier: u32, max_frame_retries: i8 },
        print: "{}, wpan_dev({}), max frame retries: {}"
    );
    event 802154_rdev_set_lbt_mode(
        wpan_phy: *mut wpan_phy, wpan_dev: *mut wpan_dev, mode: bool,
        entry: { wpan_phy_name: [c_char; MAXNAME], identifier: u32, mode: bool },
        print: "{}, wpan_dev({}), lbt mode: {}"
    );
    event 802154_rdev_set_ackreq_default(
        wpan_phy: *mut wpan_phy, wpan_dev: *mut wpan_dev, ackreq: bool,
        entry: { wpan_phy_name: [c_char; MAXNAME], identifier: u32, ackreq: bool },
        print: "{}, wpan_dev({}), ackreq default: {}"
    );
    event 802154_rdev_trigger_scan(
        wpan_phy: *mut wpan_phy, request: *mut cfg802154_scan_request,
        entry: { wpan_phy_name: [c_char; MAXNAME], page: u8, channels: u32, duration: u8 },
        print: "{}, scan, page: {}, channels: {:x}, duration {}"
    );
    event 802154_rdev_send_beacons(
        wpan_phy: *mut wpan_phy, request: *mut cfg802154_beacon_request,
        entry: { wpan_phy_name: [c_char; MAXNAME], interval: u8 },
        print: "{}, sending beacons (interval order: {})"
    );

    event_class 802154_wdev_template(
        wpan_phy: *mut wpan_phy, wpan_dev: *mut wpan_dev,
        entry: { wpan_phy_name: [c_char; MAXNAME], identifier: u32 },
        print: "{}, wpan_dev({})"
    );
    event 802154_rdev_abort_scan from 802154_wdev_template;
    event 802154_rdev_stop_beacons from 802154_wdev_template;

    event 802154_rdev_associate(
        wpan_phy: *mut wpan_phy, wpan_dev: *mut wpan_dev, coord: *mut ieee802154_addr,
        entry: { wpan_phy_name: [c_char; MAXNAME], identifier: u32, addr: __le64 },
        print: "{}, wpan_dev({}), associating with: 0x{:x}"
    );
    event 802154_rdev_disassociate(
        wpan_phy: *mut wpan_phy, wpan_dev: *mut wpan_dev, target: *mut ieee802154_addr,
        entry: { wpan_phy_name: [c_char; MAXNAME], identifier: u32, addr: __le64 },
        print: "{}, wpan_dev({}), disassociating with: 0x{:x}"
    );
    event 802154_rdev_return_int(
        wpan_phy: *mut wpan_phy, ret: i32,
        entry: { wpan_phy_name: [c_char; MAXNAME], ret: i32 },
        print: "{}, returned: {}"
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
