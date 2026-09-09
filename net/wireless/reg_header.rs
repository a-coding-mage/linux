/* SPDX-License-Identifier: ISC */

// C dependency: <net/cfg80211.h>

/*
 * Copyright 2008-2011 Luis R. Rodriguez <mcgrof@qca.qualcomm.com>
 * Copyright (C) 2019, 2023 Intel Corporation
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ieee80211_regd_source {
    REGD_SOURCE_INTERNAL_DB,
    REGD_SOURCE_CRDA,
    REGD_SOURCE_CACHED,
}

extern "C" {
    pub static cfg80211_regdomain: *const ieee80211_regdomain;

    pub fn reg_is_valid_request(alpha2: *const ::core::ffi::c_char) -> bool;
    pub fn is_world_regdom(alpha2: *const ::core::ffi::c_char) -> bool;
    pub fn reg_supported_dfs_region(dfs_region: nl80211_dfs_regions) -> bool;
    pub fn reg_get_dfs_region(wiphy: *mut wiphy) -> nl80211_dfs_regions;

    pub fn regulatory_hint_user(
        alpha2: *const ::core::ffi::c_char,
        user_reg_hint_type: nl80211_user_reg_hint_type,
    ) -> ::core::ffi::c_int;

    /**
     * regulatory_hint_indoor - hint operation in indoor env. or not
     * @is_indoor: if true indicates that user space thinks that the
     * device is operating in an indoor environment.
     * @portid: the netlink port ID on which the hint was given.
     */
    pub fn regulatory_hint_indoor(is_indoor: bool, portid: u32);

    /**
     * regulatory_netlink_notify - notify on released netlink socket
     * @portid: the netlink socket port ID
     */
    pub fn regulatory_netlink_notify(portid: u32);

    pub fn wiphy_regulatory_register(wiphy: *mut wiphy);
    pub fn wiphy_regulatory_deregister(wiphy: *mut wiphy);

    pub fn regulatory_init() -> ::core::ffi::c_int;
    pub fn regulatory_exit();

    pub fn set_regdom(
        rd: *const ieee80211_regdomain,
        regd_src: ieee80211_regd_source,
    ) -> ::core::ffi::c_int;

    pub fn reg_get_max_bandwidth(
        rd: *const ieee80211_regdomain,
        rule: *const ieee80211_reg_rule,
    ) -> ::core::ffi::c_uint;

    pub fn reg_last_request_cell_base() -> bool;

    /**
     * regulatory_hint_found_beacon - hints a beacon was found on a channel
     * @wiphy: the wireless device where the beacon was found on
     * @beacon_chan: the channel on which the beacon was found on
     * @gfp: context flags
     *
     * This informs the wireless core that a beacon from an AP was found on
     * the channel provided. This allows the wireless core to make educated
     * guesses on regulatory to help with world roaming. This is only used for
     * world roaming -- when we do not know our current location. This is
     * only useful on channels 12, 13 and 14 on the 2 GHz band as channels
     * 1-11 are already enabled by the world regulatory domain; and on
     * non-radar 5 GHz channels.
     *
     * Drivers do not need to call this, cfg80211 will do it for after a scan
     * on a newly found BSS. If you cannot make use of this feature you can
     * set the wiphy->disable_beacon_hints to true.
     */
    pub fn regulatory_hint_found_beacon(
        wiphy: *mut wiphy,
        beacon_chan: *mut ieee80211_channel,
        gfp: gfp_t,
    );

    /**
     * regulatory_hint_country_ie - hints a country IE as a regulatory domain
     * @wiphy: the wireless device giving the hint (used only for reporting
     * conflicts)
     * @band: the band on which the country IE was received on. This determines
     * the band we'll process the country IE channel triplets for.
     * @country_ie: pointer to the country IE
     * @country_ie_len: length of the country IE
     *
     * We will intersect the rd with the what CRDA tells us should apply
     * for the alpha2 this country IE belongs to, this prevents APs from
     * sending us incorrect or outdated information against a country.
     *
     * The AP is expected to provide Country IE channel triplets for the
     * band it is on. It is technically possible for APs to send channel
     * country IE triplets even for channels outside of the band they are
     * in but for that they would have to use the regulatory extension
     * in combination with a triplet but this behaviour is currently
     * not observed. For this reason if a triplet is seen with channel
     * information for a band the BSS is not present in it will be ignored.
     */
    pub fn regulatory_hint_country_ie(
        wiphy: *mut wiphy,
        band: nl80211_band,
        country_ie: *const u8,
        country_ie_len: u8,
    );

    /**
     * regulatory_hint_disconnect - informs all devices have been disconnected
     *
     * Regulatory rules can be enhanced further upon scanning and upon
     * connection to an AP. These rules become stale if we disconnect
     * and go to another country, whether or not we suspend and resume.
     * If we suspend, go to another country and resume we'll automatically
     * get disconnected shortly after resuming and things will be reset as well.
     * This routine is a helper to restore regulatory settings to how they were
     * prior to our first connect attempt. This includes ignoring country IE and
     * beacon regulatory hints. The ieee80211_regdom module parameter will always
     * be respected but if a user had set the regulatory domain that will take
     * precedence.
     *
     * Must be called from process context.
     */
    pub fn regulatory_hint_disconnect();

    /**
     * cfg80211_get_unii - get the U-NII band for the frequency
     * @freq: the frequency for which we want to get the UNII band.
     *
     * Get a value specifying the U-NII band frequency belongs to.
     * U-NII bands are defined by the FCC in C.F.R 47 part 15.
     *
     * Return: -EINVAL if freq is invalid, 0 for UNII-1, 1 for UNII-2A,
     * 2 for UNII-2B, 3 for UNII-2C and 4 for UNII-3.
     */
    pub fn cfg80211_get_unii(freq: ::core::ffi::c_int) -> ::core::ffi::c_int;

    /**
     * regulatory_indoor_allowed - is indoor operation allowed
     * Return: %true if indoor operation is allowed, %false otherwise
     */
    pub fn regulatory_indoor_allowed() -> bool;

    /**
     * regulatory_propagate_dfs_state - Propagate DFS channel state to other wiphys
     * @wiphy: wiphy on which radar is detected and the event will be propagated
     * to other available wiphys having the same DFS domain
     * @chandef: Channel definition of radar detected channel
     * @dfs_state: DFS channel state to be set
     * @event: Type of radar event which triggered this DFS state change
     *
     * This function should be called with rtnl lock held.
     */
    pub fn regulatory_propagate_dfs_state(
        wiphy: *mut wiphy,
        chandef: *mut cfg80211_chan_def,
        dfs_state: nl80211_dfs_state,
        event: nl80211_radar_event,
    );

    /**
     * reg_dfs_domain_same - Checks if both wiphy have same DFS domain configured
     * @wiphy1: wiphy it's dfs_region to be checked against that of wiphy2
     * @wiphy2: wiphy it's dfs_region to be checked against that of wiphy1
     * Return: %true if both wiphys have the same DFS domain, %false otherwise
     */
    pub fn reg_dfs_domain_same(wiphy1: *mut wiphy, wiphy2: *mut wiphy) -> bool;

    /**
     * reg_reload_regdb - reload the regulatory.db firmware file
     * Return: 0 for success, an error code otherwise
     */
    pub fn reg_reload_regdb() -> ::core::ffi::c_int;

    /**
     * reg_check_channels - schedule regulatory enforcement
     */
    pub fn reg_check_channels();

    pub static shipped_regdb_certs: *const u8;
    pub static shipped_regdb_certs_len: ::core::ffi::c_uint;
    pub static extra_regdb_certs: *const u8;
    pub static extra_regdb_certs_len: ::core::ffi::c_uint;
}

// Grace period to timeout pre-CAC results on the dfs channels. This timeout
// value is used for Non-ETSI domain.
// TODO: May be make this timeout available through regdb?
pub const REG_PRE_CAC_EXPIRY_GRACE_MS: u32 = 2000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
