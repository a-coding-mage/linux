/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than redefined.

#[repr(C)]
pub struct net_packet_attrs {
    pub src: *const u8,
    pub dst: *const u8,
    pub ip_src: u32,
    pub ip_dst: u32,
    pub tcp: bool,
    pub sport: u16,
    pub dport: u16,
    pub timeout: i32,
    pub size: i32,
    pub max_size: i32,
    pub id: u8,
    pub queue_mapping: u16,
    pub bad_csum: bool,
}

#[repr(C)]
pub struct net_test_priv {
    pub packet: *mut net_packet_attrs,
    pub pt: packet_type,
    pub comp: completion,
    pub double_vlan: i32,
    pub vlan_id: i32,
    pub ok: i32,
}

#[repr(C, packed)]
pub struct netsfhdr {
    pub version: __be32,
    pub magic: __be64,
    pub id: u8,
}

pub const NET_TEST_PKT_SIZE: usize = core::mem::size_of::<ethhdr>()
    + core::mem::size_of::<iphdr>()
    + core::mem::size_of::<netsfhdr>();
pub const NET_TEST_PKT_MAGIC: u64 = 0xdeadcafecafedead;
pub const NET_LB_TIMEOUT: u64 = msecs_to_jiffies(200);

#[cfg(CONFIG_NET_SELFTESTS)]
extern "C" {
    pub fn net_test_get_skb(
        ndev: *mut net_device,
        id: u8,
        attr: *mut net_packet_attrs,
    ) -> *mut sk_buff;
    pub fn net_selftest(ndev: *mut net_device, etest: *mut ethtool_test, buf: *mut u64);
    pub fn net_selftest_get_count() -> i32;
    pub fn net_selftest_get_strings(data: *mut u8);
}

#[cfg(not(CONFIG_NET_SELFTESTS))]
#[inline]
pub unsafe fn net_test_get_skb(
    _ndev: *mut net_device,
    _id: u8,
    _attr: *mut net_packet_attrs,
) -> *mut sk_buff {
    core::ptr::null_mut()
}

#[cfg(not(CONFIG_NET_SELFTESTS))]
#[inline]
pub unsafe fn net_selftest(
    _ndev: *mut net_device,
    _etest: *mut ethtool_test,
    _buf: *mut u64,
) {
}

#[cfg(not(CONFIG_NET_SELFTESTS))]
#[inline]
pub unsafe fn net_selftest_get_count() -> i32 {
    0
}

#[cfg(not(CONFIG_NET_SELFTESTS))]
#[inline]
pub unsafe fn net_selftest_get_strings(_data: *mut u8) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
