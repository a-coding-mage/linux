// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2017 Facebook
 */

/*
 * C dependencies:
 * #include <stddef.h>
 * #include <string.h>
 * #include <linux/bpf.h>
 * #include <linux/pkt_cls.h>
 * #include <bpf/bpf_helpers.h>
 */

use core::ptr;

#[cfg(target_endian = "little")]
unsafe fn TEST_FIELD<TYPE>(field: *const __u32, mask: __u32) -> bool
where
    TYPE: Copy + PartialEq + FromMaskedU32,
{
    let tmp: TYPE = ptr::read_volatile(field as *const TYPE);
    tmp != TYPE::from_masked_u32(ptr::read_volatile(field) & mask)
}

#[cfg(target_endian = "big")]
unsafe fn TEST_FIELD<TYPE>(field: *const __u32, mask: __u32) -> bool
where
    TYPE: Copy + PartialEq + FromMaskedU32,
{
    let offset = (core::mem::size_of::<__u32>() - core::mem::size_of::<TYPE>())
        / core::mem::size_of::<TYPE>();
    let tmp: TYPE = ptr::read_volatile((field as *const TYPE).add(offset));
    tmp != TYPE::from_masked_u32(ptr::read_volatile(field) & mask)
}

trait FromMaskedU32 {
    fn from_masked_u32(value: __u32) -> Self;
}

impl FromMaskedU32 for __u8 {
    fn from_masked_u32(value: __u32) -> Self {
        value as __u8
    }
}

impl FromMaskedU32 for __u16 {
    fn from_masked_u32(value: __u32) -> Self {
        value as __u16
    }
}

impl FromMaskedU32 for __u32 {
    fn from_masked_u32(value: __u32) -> Self {
        value
    }
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn test_pkt_md_access(skb: *mut __sk_buff) -> i32 {
    if TEST_FIELD::<__u8>(ptr::addr_of!((*skb).len), 0xFF) {
        return TC_ACT_SHOT;
    }
    if TEST_FIELD::<__u16>(ptr::addr_of!((*skb).len), 0xFFFF) {
        return TC_ACT_SHOT;
    }
    if TEST_FIELD::<__u32>(ptr::addr_of!((*skb).len), 0xFFFFFFFF) {
        return TC_ACT_SHOT;
    }
    if TEST_FIELD::<__u16>(ptr::addr_of!((*skb).protocol), 0xFFFF) {
        return TC_ACT_SHOT;
    }
    if TEST_FIELD::<__u32>(ptr::addr_of!((*skb).protocol), 0xFFFFFFFF) {
        return TC_ACT_SHOT;
    }
    if TEST_FIELD::<__u8>(ptr::addr_of!((*skb).hash), 0xFF) {
        return TC_ACT_SHOT;
    }
    if TEST_FIELD::<__u16>(ptr::addr_of!((*skb).hash), 0xFFFF) {
        return TC_ACT_SHOT;
    }
    if TEST_FIELD::<__u32>(ptr::addr_of!((*skb).hash), 0xFFFFFFFF) {
        return TC_ACT_SHOT;
    }

    TC_ACT_OK
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
