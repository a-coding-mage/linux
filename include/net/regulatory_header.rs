/*
 * regulatory support structures
 *
 * Translated from regulatory.h.  Linux declarations referenced by this
 * header are intentionally left as external crate-level dependencies.
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum environment_cap {
    ENVIRON_ANY,
    ENVIRON_INDOOR,
    ENVIRON_OUTDOOR,
}

#[repr(C)]
pub struct regulatory_request {
    pub rcu_head: crate::rcu_head,
    pub wiphy_idx: ::core::ffi::c_int,
    pub initiator: crate::nl80211_reg_initiator,
    pub user_reg_hint_type: crate::nl80211_user_reg_hint_type,
    pub alpha2: [::core::ffi::c_char; 3],
    pub dfs_region: crate::nl80211_dfs_regions,
    pub intersect: bool,
    pub processed: bool,
    pub country_ie_env: environment_cap,
    pub list: crate::list_head,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ieee80211_regulatory_flags {
    REGULATORY_CUSTOM_REG = 1 << 0,
    REGULATORY_STRICT_REG = 1 << 1,
    REGULATORY_DISABLE_BEACON_HINTS = 1 << 2,
    REGULATORY_COUNTRY_IE_FOLLOW_POWER = 1 << 3,
    REGULATORY_COUNTRY_IE_IGNORE = 1 << 4,
    REGULATORY_ENABLE_RELAX_NO_IR = 1 << 5,
    /* reuse bit 6 next time */
    REGULATORY_WIPHY_SELF_MANAGED = 1 << 7,
}

#[repr(C)]
pub struct ieee80211_freq_range {
    pub start_freq_khz: u32,
    pub end_freq_khz: u32,
    pub max_bandwidth_khz: u32,
}

#[repr(C)]
pub struct ieee80211_power_rule {
    pub max_antenna_gain: u32,
    pub max_eirp: u32,
}

#[repr(C)]
pub struct ieee80211_wmm_ac {
    pub cw_min: u16,
    pub cw_max: u16,
    pub cot: u16,
    pub aifsn: u8,
}

#[repr(C)]
pub struct ieee80211_wmm_rule {
    pub client: [ieee80211_wmm_ac; crate::IEEE80211_NUM_ACS],
    pub ap: [ieee80211_wmm_ac; crate::IEEE80211_NUM_ACS],
}

#[repr(C)]
pub struct ieee80211_reg_rule {
    pub freq_range: ieee80211_freq_range,
    pub power_rule: ieee80211_power_rule,
    pub wmm_rule: ieee80211_wmm_rule,
    pub flags: u32,
    pub dfs_cac_ms: u32,
    pub has_wmm: bool,
    pub psd: i8,
}

#[repr(C)]
pub struct ieee80211_regdomain {
    pub rcu_head: crate::rcu_head,
    pub n_reg_rules: u32,
    pub alpha2: [::core::ffi::c_char; 3],
    pub dfs_region: crate::nl80211_dfs_regions,
    pub reg_rules: [ieee80211_reg_rule; 0],
}

/* C macros, retained as Rust construction macros. */
#[macro_export]
macro_rules! REG_RULE_EXT {
    ($start:expr, $end:expr, $bw:expr, $gain:expr, $eirp:expr, $dfs_cac:expr, $reg_flags:expr) => {{
        $crate::ieee80211_reg_rule {
            freq_range: $crate::ieee80211_freq_range {
                start_freq_khz: $crate::MHZ_TO_KHZ!($start),
                end_freq_khz: $crate::MHZ_TO_KHZ!($end),
                max_bandwidth_khz: $crate::MHZ_TO_KHZ!($bw),
            },
            power_rule: $crate::ieee80211_power_rule {
                max_antenna_gain: $crate::DBI_TO_MBI!($gain),
                max_eirp: $crate::DBM_TO_MBM!($eirp),
            },
            wmm_rule: $crate::ieee80211_wmm_rule {
                client: [unsafe { ::core::mem::zeroed() }; $crate::IEEE80211_NUM_ACS],
                ap: [unsafe { ::core::mem::zeroed() }; $crate::IEEE80211_NUM_ACS],
            },
            flags: $reg_flags,
            dfs_cac_ms: $dfs_cac,
            has_wmm: false,
            psd: 0,
        }
    }};
}

#[macro_export]
macro_rules! REG_RULE {
    ($start:expr, $end:expr, $bw:expr, $gain:expr, $eirp:expr, $reg_flags:expr) => {
        $crate::REG_RULE_EXT!($start, $end, $bw, $gain, $eirp, 0, $reg_flags)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
