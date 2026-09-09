/* SPDX-License-Identifier: GPL-2.0-only */

// Dependency provided by the surrounding translation unit: netlink.h

// static const struct nla_policy
// ethnl_ts_hwtst_prov_policy[ETHTOOL_A_TS_HWTSTAMP_PROVIDER_MAX + 1] = {
//     [ETHTOOL_A_TS_HWTSTAMP_PROVIDER_INDEX] = { .type = NLA_U32 },
//     [ETHTOOL_A_TS_HWTSTAMP_PROVIDER_QUALIFIER] =
//         NLA_POLICY_MAX(NLA_U32, HWTSTAMP_PROVIDER_QUALIFIER_CNT - 1)
// };
//
// The designated-index initializer and NLA_POLICY_MAX are retained above
// because their concrete Rust representations are supplied by netlink.h.

extern "C" {
    pub fn ts_parse_hwtst_provider(
        nest: *const nlattr,
        hwprov_desc: *mut hwtstamp_provider_desc,
        extack: *mut netlink_ext_ack,
        mod_: *mut bool,
    ) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
