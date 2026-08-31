// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

/* Dependencies in the original C source:
 * #include <linux/types.h>
 * #include <bpf/bpf_helpers.h>
 * #include <linux/bpf.h>
 * #include <stdint.h>
 */

pub const TWFW_MAX_TIERS: u32 = 64;
/*
 * load is successful
 * #define TWFW_MAX_TIERS (64u)$
 */

#[repr(C)]
pub struct twfw_tier_value {
    pub mask: [::core::ffi::c_ulong; 1],
}

#[repr(C)]
pub struct rule {
    pub seqnum: u8,
}

/* Original BPF map declaration:
 * struct rules_map {
 *     __uint(type, BPF_MAP_TYPE_ARRAY);
 *     __type(key, __u32);
 *     __type(value, struct rule);
 *     __uint(max_entries, 1);
 * };
 */
#[repr(C)]
pub struct rules_map {
    _private: [u8; 0],
}

/* Original BPF map declaration:
 * struct tiers_map {
 *     __uint(type, BPF_MAP_TYPE_ARRAY);
 *     __type(key, __u32);
 *     __type(value, struct twfw_tier_value);
 *     __uint(max_entries, 1);
 * };
 */
#[repr(C)]
pub struct tiers_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut rules: rules_map;
    static mut tiers: tiers_map;

    fn bpf_map_lookup_elem(
        map: *const ::core::ffi::c_void,
        key: *const ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
}

/* SEC(".maps") */
/* struct rules_map rules SEC(".maps"); */
/* struct tiers_map tiers SEC(".maps"); */

/* SEC("cgroup_skb/ingress") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn twfw_verifier(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let key: u32 = 0;
    let tier: *const twfw_tier_value = unsafe {
        bpf_map_lookup_elem(
            ::core::ptr::addr_of!(tiers).cast::<::core::ffi::c_void>(),
            ::core::ptr::addr_of!(key).cast::<::core::ffi::c_void>(),
        )
    }
    .cast::<twfw_tier_value>();
    if tier.is_null() {
        return 1;
    }

    let rule: *mut rule = unsafe {
        bpf_map_lookup_elem(
            ::core::ptr::addr_of!(rules).cast::<::core::ffi::c_void>(),
            ::core::ptr::addr_of!(key).cast::<::core::ffi::c_void>(),
        )
    }
    .cast::<rule>();
    if rule.is_null() {
        return 1;
    }

    if !rule.is_null() && unsafe { (*rule).seqnum } < TWFW_MAX_TIERS as u8 {
        /* rule->seqnum / 64 should always be 0 */
        let mask: ::core::ffi::c_ulong =
            unsafe { (*tier).mask[(unsafe { (*rule).seqnum } / 64) as usize] };
        if mask != 0 {
            return 0;
        }
    }
    1
}
