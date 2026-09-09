//! Low-level Rust translation boundary for mac80211 interface handling.
#![allow(dead_code, unused_variables, unused_mut)]

pub const SOURCE_ROLE: &str = "implementation source";

#[repr(C)]
pub struct ieee80211_link_data { _private: [u8; 0] }

pub unsafe fn __ieee80211_recalc_txpower(link: *mut ieee80211_link_data) -> bool {
    let _ = link;
    todo!("requires external mac80211 bindings")
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
