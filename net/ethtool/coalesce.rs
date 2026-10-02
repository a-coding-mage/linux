// SPDX-License-Identifier: GPL-2.0-only
// NEW RECONSTRUCTION from the unchanged net/ethtool/coalesce.c.
// This is not a byte-exact recovery of the lost post-checkpoint provider.
// Target C headers own shared layouts. All coalesce policy and control flow,
// including nested profile iteration and driver callback dispatch, live here.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
    missing_docs, unsafe_op_in_unsafe_fn, clippy::all, unreachable_pub)]
#[allow(improper_ctypes)]
mod bindings {
    use kernel::ffi;
    include!(concat!(env!("OBJTREE"), "/rust/bindings/ethtool_coalesce_generated.rs"));
}
use bindings::*;
use core::mem::{size_of, zeroed};
use core::ptr::{addr_of, addr_of_mut, null_mut};

#[repr(C)]
struct coalesce_req_info { base: ethnl_req_info }
#[repr(C)]
struct coalesce_reply_data {
    base: ethnl_reply_data,
    coalesce: ethtool_coalesce,
    kernel_coalesce: kernel_ethtool_coalesce,
    supported_params: u32,
}

const SUPPORTED_OFFSET: u32 = ETHTOOL_A_COALESCE_RX_USECS;
#[inline]
const fn attr_to_mask(attr_type: u32) -> u32 { 1u32 << (attr_type - SUPPORTED_OFFSET) }

// These are the original C build-time assertions, using generated constants.
macro_rules! check_supported_offset {
    ($mask:ident, $attr:ident) => { const _: () = assert!($mask as u32 == attr_to_mask($attr)); };
}
check_supported_offset!(RUST_ETHTOOL_COALESCE_RX_USECS, ETHTOOL_A_COALESCE_RX_USECS);
check_supported_offset!(RUST_ETHTOOL_COALESCE_RX_MAX_FRAMES, ETHTOOL_A_COALESCE_RX_MAX_FRAMES);
check_supported_offset!(RUST_ETHTOOL_COALESCE_RX_USECS_IRQ, ETHTOOL_A_COALESCE_RX_USECS_IRQ);
check_supported_offset!(RUST_ETHTOOL_COALESCE_RX_MAX_FRAMES_IRQ, ETHTOOL_A_COALESCE_RX_MAX_FRAMES_IRQ);
check_supported_offset!(RUST_ETHTOOL_COALESCE_TX_USECS, ETHTOOL_A_COALESCE_TX_USECS);
check_supported_offset!(RUST_ETHTOOL_COALESCE_TX_MAX_FRAMES, ETHTOOL_A_COALESCE_TX_MAX_FRAMES);
check_supported_offset!(RUST_ETHTOOL_COALESCE_TX_USECS_IRQ, ETHTOOL_A_COALESCE_TX_USECS_IRQ);
check_supported_offset!(RUST_ETHTOOL_COALESCE_TX_MAX_FRAMES_IRQ, ETHTOOL_A_COALESCE_TX_MAX_FRAMES_IRQ);
check_supported_offset!(RUST_ETHTOOL_COALESCE_STATS_BLOCK_USECS, ETHTOOL_A_COALESCE_STATS_BLOCK_USECS);
check_supported_offset!(RUST_ETHTOOL_COALESCE_USE_ADAPTIVE_RX, ETHTOOL_A_COALESCE_USE_ADAPTIVE_RX);
check_supported_offset!(RUST_ETHTOOL_COALESCE_USE_ADAPTIVE_TX, ETHTOOL_A_COALESCE_USE_ADAPTIVE_TX);
check_supported_offset!(RUST_ETHTOOL_COALESCE_PKT_RATE_LOW, ETHTOOL_A_COALESCE_PKT_RATE_LOW);
check_supported_offset!(RUST_ETHTOOL_COALESCE_RX_USECS_LOW, ETHTOOL_A_COALESCE_RX_USECS_LOW);
check_supported_offset!(RUST_ETHTOOL_COALESCE_RX_MAX_FRAMES_LOW, ETHTOOL_A_COALESCE_RX_MAX_FRAMES_LOW);
check_supported_offset!(RUST_ETHTOOL_COALESCE_TX_USECS_LOW, ETHTOOL_A_COALESCE_TX_USECS_LOW);
check_supported_offset!(RUST_ETHTOOL_COALESCE_TX_MAX_FRAMES_LOW, ETHTOOL_A_COALESCE_TX_MAX_FRAMES_LOW);
check_supported_offset!(RUST_ETHTOOL_COALESCE_PKT_RATE_HIGH, ETHTOOL_A_COALESCE_PKT_RATE_HIGH);
check_supported_offset!(RUST_ETHTOOL_COALESCE_RX_USECS_HIGH, ETHTOOL_A_COALESCE_RX_USECS_HIGH);
check_supported_offset!(RUST_ETHTOOL_COALESCE_RX_MAX_FRAMES_HIGH, ETHTOOL_A_COALESCE_RX_MAX_FRAMES_HIGH);
check_supported_offset!(RUST_ETHTOOL_COALESCE_TX_USECS_HIGH, ETHTOOL_A_COALESCE_TX_USECS_HIGH);
check_supported_offset!(RUST_ETHTOOL_COALESCE_TX_MAX_FRAMES_HIGH, ETHTOOL_A_COALESCE_TX_MAX_FRAMES_HIGH);
check_supported_offset!(RUST_ETHTOOL_COALESCE_RATE_SAMPLE_INTERVAL, ETHTOOL_A_COALESCE_RATE_SAMPLE_INTERVAL);

// Transparent immutable wrappers retain the exact array/struct C ABI. Their
// pointers target other immutable policy tables; no interior mutation occurs.
#[repr(transparent)]
struct PolicyArray<const N: usize>([nla_policy; N]);
unsafe impl<const N: usize> Sync for PolicyArray<N> {}
#[repr(transparent)]
struct RequestOps(ethnl_request_ops);
unsafe impl Sync for RequestOps {}

const fn scalar_policy(type_: u32) -> nla_policy {
    let mut p: nla_policy = unsafe { zeroed() };
    p.type_ = type_ as u8;
    p
}
const fn nested_policy(policy: *const nla_policy, maxattr: u32) -> nla_policy {
    let mut p = scalar_policy(NLA_NESTED);
    p.len = maxattr as u16;
    p.__bindgen_anon_1.nested_policy = policy;
    p
}
const fn boolean_policy() -> nla_policy {
    let mut p = scalar_policy(NLA_U8);
    p.validation_type = RUST_NLA_VALIDATE_MAX as u8;
    p.__bindgen_anon_1.__bindgen_anon_1.max = 1;
    p
}

#[no_mangle]
static ethnl_coalesce_get_policy: PolicyArray<{ ETHTOOL_A_COALESCE_HEADER as usize + 1 }> = {
    let mut p = unsafe { zeroed::<[nla_policy; ETHTOOL_A_COALESCE_HEADER as usize + 1]>() };
    p[ETHTOOL_A_COALESCE_HEADER as usize] =
        nested_policy(addr_of!(ethnl_header_policy).cast(), ETHTOOL_A_HEADER_FLAGS);
    PolicyArray(p)
};
static coalesce_irq_moderation_policy: PolicyArray<{ ETHTOOL_A_IRQ_MODERATION_COMPS as usize + 1 }> = {
    let mut p = unsafe { zeroed::<[nla_policy; ETHTOOL_A_IRQ_MODERATION_COMPS as usize + 1]>() };
    p[ETHTOOL_A_IRQ_MODERATION_USEC as usize] = scalar_policy(NLA_U32);
    p[ETHTOOL_A_IRQ_MODERATION_PKTS as usize] = scalar_policy(NLA_U32);
    p[ETHTOOL_A_IRQ_MODERATION_COMPS as usize] = scalar_policy(NLA_U32);
    PolicyArray(p)
};
static coalesce_profile_policy: PolicyArray<{ ETHTOOL_A_PROFILE_IRQ_MODERATION as usize + 1 }> = {
    let mut p = unsafe { zeroed::<[nla_policy; ETHTOOL_A_PROFILE_IRQ_MODERATION as usize + 1]>() };
    p[ETHTOOL_A_PROFILE_IRQ_MODERATION as usize] = nested_policy(
        addr_of!(coalesce_irq_moderation_policy.0).cast(), ETHTOOL_A_IRQ_MODERATION_COMPS);
    PolicyArray(p)
};
#[no_mangle]
static ethnl_coalesce_set_policy: PolicyArray<{ ETHTOOL_A_COALESCE_RX_CQE_NSECS as usize + 1 }> = {
    let mut p = unsafe { zeroed::<[nla_policy; ETHTOOL_A_COALESCE_RX_CQE_NSECS as usize + 1]>() };
    p[ETHTOOL_A_COALESCE_HEADER as usize] =
        nested_policy(addr_of!(ethnl_header_policy).cast(), ETHTOOL_A_HEADER_FLAGS);
    p[ETHTOOL_A_COALESCE_RX_USECS as usize] = scalar_policy(NLA_U32);
    p[ETHTOOL_A_COALESCE_RX_MAX_FRAMES as usize] = scalar_policy(NLA_U32);
    p[ETHTOOL_A_COALESCE_RX_USECS_IRQ as usize] = scalar_policy(NLA_U32);
    p[ETHTOOL_A_COALESCE_RX_MAX_FRAMES_IRQ as usize] = scalar_policy(NLA_U32);
    p[ETHTOOL_A_COALESCE_TX_USECS as usize] = scalar_policy(NLA_U32);
    p[ETHTOOL_A_COALESCE_TX_MAX_FRAMES as usize] = scalar_policy(NLA_U32);
    p[ETHTOOL_A_COALESCE_TX_USECS_IRQ as usize] = scalar_policy(NLA_U32);
    p[ETHTOOL_A_COALESCE_TX_MAX_FRAMES_IRQ as usize] = scalar_policy(NLA_U32);
    p[ETHTOOL_A_COALESCE_STATS_BLOCK_USECS as usize] = scalar_policy(NLA_U32);
    p[ETHTOOL_A_COALESCE_USE_ADAPTIVE_RX as usize] = scalar_policy(NLA_U8);
    p[ETHTOOL_A_COALESCE_USE_ADAPTIVE_TX as usize] = scalar_policy(NLA_U8);
    p[ETHTOOL_A_COALESCE_PKT_RATE_LOW as usize] = scalar_policy(NLA_U32);
    p[ETHTOOL_A_COALESCE_RX_USECS_LOW as usize] = scalar_policy(NLA_U32);
    p[ETHTOOL_A_COALESCE_RX_MAX_FRAMES_LOW as usize] = scalar_policy(NLA_U32);
    p[ETHTOOL_A_COALESCE_TX_USECS_LOW as usize] = scalar_policy(NLA_U32);
    p[ETHTOOL_A_COALESCE_TX_MAX_FRAMES_LOW as usize] = scalar_policy(NLA_U32);
    p[ETHTOOL_A_COALESCE_PKT_RATE_HIGH as usize] = scalar_policy(NLA_U32);
    p[ETHTOOL_A_COALESCE_RX_USECS_HIGH as usize] = scalar_policy(NLA_U32);
    p[ETHTOOL_A_COALESCE_RX_MAX_FRAMES_HIGH as usize] = scalar_policy(NLA_U32);
    p[ETHTOOL_A_COALESCE_TX_USECS_HIGH as usize] = scalar_policy(NLA_U32);
    p[ETHTOOL_A_COALESCE_TX_MAX_FRAMES_HIGH as usize] = scalar_policy(NLA_U32);
    p[ETHTOOL_A_COALESCE_RATE_SAMPLE_INTERVAL as usize] = scalar_policy(NLA_U32);
    p[ETHTOOL_A_COALESCE_USE_CQE_MODE_TX as usize] = boolean_policy();
    p[ETHTOOL_A_COALESCE_USE_CQE_MODE_RX as usize] = boolean_policy();
    p[ETHTOOL_A_COALESCE_TX_AGGR_MAX_BYTES as usize] = scalar_policy(NLA_U32);
    p[ETHTOOL_A_COALESCE_TX_AGGR_MAX_FRAMES as usize] = scalar_policy(NLA_U32);
    p[ETHTOOL_A_COALESCE_TX_AGGR_TIME_USECS as usize] = scalar_policy(NLA_U32);
    p[ETHTOOL_A_COALESCE_RX_CQE_FRAMES as usize] = scalar_policy(NLA_U32);
    p[ETHTOOL_A_COALESCE_RX_CQE_NSECS as usize] = scalar_policy(NLA_U32);
    p[ETHTOOL_A_COALESCE_RX_PROFILE as usize] = nested_policy(
        addr_of!(coalesce_profile_policy.0).cast(), ETHTOOL_A_PROFILE_IRQ_MODERATION);
    p[ETHTOOL_A_COALESCE_TX_PROFILE as usize] = nested_policy(
        addr_of!(coalesce_profile_policy.0).cast(), ETHTOOL_A_PROFILE_IRQ_MODERATION);
    PolicyArray(p)
};

unsafe extern "C" fn coalesce_prepare_data(
    _req_base: *const ethnl_req_info, reply_base: *mut ethnl_reply_data,
    info: *const genl_info,
) -> i32 {
    // base is the first member of the repr(C) reply structure.
    let data = reply_base.cast::<coalesce_reply_data>();
    let dev = (*reply_base).dev;
    if rust_coalesce_get_op(dev).is_none() { return -(EOPNOTSUPP as i32); }
    (*data).supported_params = rust_coalesce_supported(dev);
    let ret = ethnl_ops_begin(dev);
    if ret < 0 { return ret; }
    let ret = rust_coalesce_get_op(dev).unwrap_unchecked()(dev,
        addr_of_mut!((*data).coalesce), addr_of_mut!((*data).kernel_coalesce),
        rust_coalesce_extack(info));
    ethnl_ops_complete(dev);
    ret
}

unsafe extern "C" fn coalesce_reply_size(
    _req_base: *const ethnl_req_info, _reply_base: *const ethnl_reply_data,
) -> i32 {
    let u32sz = rust_coalesce_nla_total_size(size_of::<u32>() as i32);
    let u8sz = rust_coalesce_nla_total_size(size_of::<u8>() as i32);
    let nestsz = rust_coalesce_nla_total_size(0);
    let modersz = nestsz + 3 * u32sz;
    let total_modersz = nestsz + modersz * NET_DIM_PARAMS_NUM_PROFILES as i32;
    // 25 numeric attributes, four mode attributes, RX and TX profile nests.
    25 * u32sz + 4 * u8sz + 2 * total_modersz
}

unsafe fn coalesce_put_u32(skb: *mut sk_buff, attr_type: u32, val: u32, supported: u32) -> bool {
    if val == 0 && supported & attr_to_mask(attr_type) == 0 { return false; }
    rust_coalesce_nla_put_u32(skb, attr_type as i32, val) != 0
}
unsafe fn coalesce_put_bool(skb: *mut sk_buff, attr_type: u32, val: u32, supported: u32) -> bool {
    if val == 0 && supported & attr_to_mask(attr_type) == 0 { return false; }
    rust_coalesce_nla_put_u8(skb, attr_type as i32, (val != 0) as u8) != 0
}

unsafe fn coalesce_put_profile(
    skb: *mut sk_buff, attr_type: u32, profile: *const dim_cq_moder, coal_flags: u8,
) -> i32 {
    if profile.is_null() || coal_flags == 0 { return 0; }
    let profile_attr = rust_coalesce_nla_nest_start(skb, attr_type as i32);
    if profile_attr.is_null() { return -(EMSGSIZE as i32); }
    for i in 0..NET_DIM_PARAMS_NUM_PROFILES as usize {
        let moder_attr = rust_coalesce_nla_nest_start(skb, ETHTOOL_A_PROFILE_IRQ_MODERATION as i32);
        if moder_attr.is_null() {
            rust_coalesce_nla_nest_cancel(skb, profile_attr);
            return -(EMSGSIZE as i32);
        }
        let p = profile.add(i);
        macro_rules! put {
            ($flag:ident, $attr:ident, $field:ident) => {
                if coal_flags & $flag as u8 != 0 {
                    let ret = rust_coalesce_nla_put_u32(skb, $attr as i32, (*p).$field as u32);
                    if ret != 0 {
                        rust_coalesce_nla_nest_cancel(skb, moder_attr);
                        rust_coalesce_nla_nest_cancel(skb, profile_attr);
                        return ret;
                    }
                }
            };
        }
        put!(RUST_DIM_COALESCE_USEC, ETHTOOL_A_IRQ_MODERATION_USEC, usec);
        put!(RUST_DIM_COALESCE_PKTS, ETHTOOL_A_IRQ_MODERATION_PKTS, pkts);
        put!(RUST_DIM_COALESCE_COMPS, ETHTOOL_A_IRQ_MODERATION_COMPS, comps);
        rust_coalesce_nla_nest_end(skb, moder_attr);
    }
    rust_coalesce_nla_nest_end(skb, profile_attr);
    0
}

unsafe extern "C" fn coalesce_fill_reply(
    skb: *mut sk_buff, req_base: *const ethnl_req_info, reply_base: *const ethnl_reply_data,
) -> i32 {
    let data = &*reply_base.cast::<coalesce_reply_data>();
    let coal = &data.coalesce;
    let kcoal = &data.kernel_coalesce;
    let supported = data.supported_params;
    if coalesce_put_u32(skb, ETHTOOL_A_COALESCE_RX_USECS, coal.rx_coalesce_usecs, supported)
        || coalesce_put_u32(skb, ETHTOOL_A_COALESCE_RX_MAX_FRAMES, coal.rx_max_coalesced_frames, supported)
        || coalesce_put_u32(skb, ETHTOOL_A_COALESCE_RX_USECS_IRQ, coal.rx_coalesce_usecs_irq, supported)
        || coalesce_put_u32(skb, ETHTOOL_A_COALESCE_RX_MAX_FRAMES_IRQ, coal.rx_max_coalesced_frames_irq, supported)
        || coalesce_put_u32(skb, ETHTOOL_A_COALESCE_TX_USECS, coal.tx_coalesce_usecs, supported)
        || coalesce_put_u32(skb, ETHTOOL_A_COALESCE_TX_MAX_FRAMES, coal.tx_max_coalesced_frames, supported)
        || coalesce_put_u32(skb, ETHTOOL_A_COALESCE_TX_USECS_IRQ, coal.tx_coalesce_usecs_irq, supported)
        || coalesce_put_u32(skb, ETHTOOL_A_COALESCE_TX_MAX_FRAMES_IRQ, coal.tx_max_coalesced_frames_irq, supported)
        || coalesce_put_u32(skb, ETHTOOL_A_COALESCE_STATS_BLOCK_USECS, coal.stats_block_coalesce_usecs, supported)
        || coalesce_put_bool(skb, ETHTOOL_A_COALESCE_USE_ADAPTIVE_RX, coal.use_adaptive_rx_coalesce, supported)
        || coalesce_put_bool(skb, ETHTOOL_A_COALESCE_USE_ADAPTIVE_TX, coal.use_adaptive_tx_coalesce, supported)
        || coalesce_put_u32(skb, ETHTOOL_A_COALESCE_PKT_RATE_LOW, coal.pkt_rate_low, supported)
        || coalesce_put_u32(skb, ETHTOOL_A_COALESCE_RX_USECS_LOW, coal.rx_coalesce_usecs_low, supported)
        || coalesce_put_u32(skb, ETHTOOL_A_COALESCE_RX_MAX_FRAMES_LOW, coal.rx_max_coalesced_frames_low, supported)
        || coalesce_put_u32(skb, ETHTOOL_A_COALESCE_TX_USECS_LOW, coal.tx_coalesce_usecs_low, supported)
        || coalesce_put_u32(skb, ETHTOOL_A_COALESCE_TX_MAX_FRAMES_LOW, coal.tx_max_coalesced_frames_low, supported)
        || coalesce_put_u32(skb, ETHTOOL_A_COALESCE_PKT_RATE_HIGH, coal.pkt_rate_high, supported)
        || coalesce_put_u32(skb, ETHTOOL_A_COALESCE_RX_USECS_HIGH, coal.rx_coalesce_usecs_high, supported)
        || coalesce_put_u32(skb, ETHTOOL_A_COALESCE_RX_MAX_FRAMES_HIGH, coal.rx_max_coalesced_frames_high, supported)
        || coalesce_put_u32(skb, ETHTOOL_A_COALESCE_TX_USECS_HIGH, coal.tx_coalesce_usecs_high, supported)
        || coalesce_put_u32(skb, ETHTOOL_A_COALESCE_TX_MAX_FRAMES_HIGH, coal.tx_max_coalesced_frames_high, supported)
        || coalesce_put_u32(skb, ETHTOOL_A_COALESCE_RATE_SAMPLE_INTERVAL, coal.rate_sample_interval, supported)
        || coalesce_put_bool(skb, ETHTOOL_A_COALESCE_USE_CQE_MODE_TX, kcoal.use_cqe_mode_tx as u32, supported)
        || coalesce_put_bool(skb, ETHTOOL_A_COALESCE_USE_CQE_MODE_RX, kcoal.use_cqe_mode_rx as u32, supported)
        || coalesce_put_u32(skb, ETHTOOL_A_COALESCE_TX_AGGR_MAX_BYTES, kcoal.tx_aggr_max_bytes, supported)
        || coalesce_put_u32(skb, ETHTOOL_A_COALESCE_TX_AGGR_MAX_FRAMES, kcoal.tx_aggr_max_frames, supported)
        || coalesce_put_u32(skb, ETHTOOL_A_COALESCE_TX_AGGR_TIME_USECS, kcoal.tx_aggr_time_usecs, supported)
        || coalesce_put_u32(skb, ETHTOOL_A_COALESCE_RX_CQE_FRAMES, kcoal.rx_cqe_frames, supported)
        || coalesce_put_u32(skb, ETHTOOL_A_COALESCE_RX_CQE_NSECS, kcoal.rx_cqe_nsecs, supported)
    { return -(EMSGSIZE as i32); }
    let dev = (*req_base).dev;
    if dev.is_null() { return 0; }
    let moder = rust_coalesce_irq_moder(dev);
    if moder.is_null() { return 0; }
    let mut ret = 0;
    rust_coalesce_rcu_read_lock();
    if (*moder).profile_flags & RUST_DIM_PROFILE_RX as u8 != 0 {
        ret = coalesce_put_profile(skb, ETHTOOL_A_COALESCE_RX_PROFILE,
            rust_coalesce_rcu_dereference(addr_of!((*moder).rx_profile)), (*moder).coal_flags);
    }
    if ret == 0 && (*moder).profile_flags & RUST_DIM_PROFILE_TX as u8 != 0 {
        ret = coalesce_put_profile(skb, ETHTOOL_A_COALESCE_TX_PROFILE,
            rust_coalesce_rcu_dereference(addr_of!((*moder).tx_profile)), (*moder).coal_flags);
    }
    rust_coalesce_rcu_read_unlock();
    ret
}

unsafe extern "C" fn ethnl_set_coalesce_validate(req_info: *mut ethnl_req_info, info: *mut genl_info) -> i32 {
    let dev = (*req_info).dev;
    let irq_moder = rust_coalesce_irq_moder(dev);
    let tb = rust_coalesce_attrs(info);
    if rust_coalesce_get_op(dev).is_none() || rust_coalesce_set_op(dev).is_none() {
        return -(EOPNOTSUPP as i32);
    }
    let mut supported = rust_coalesce_supported(dev);
    if !irq_moder.is_null() && (*irq_moder).profile_flags & RUST_DIM_PROFILE_RX as u8 != 0 {
        supported |= RUST_ETHTOOL_COALESCE_RX_PROFILE as u32;
    }
    if !irq_moder.is_null() && (*irq_moder).profile_flags & RUST_DIM_PROFILE_TX as u8 != 0 {
        supported |= RUST_ETHTOOL_COALESCE_TX_PROFILE as u32;
    }
    for a in ETHTOOL_A_COALESCE_RX_USECS..__ETHTOOL_A_COALESCE_CNT {
        let attr = *tb.add(a as usize);
        if !attr.is_null() && supported & attr_to_mask(a) == 0 {
            rust_coalesce_unsupported(rust_coalesce_extack(info), attr);
            return -(EINVAL as i32);
        }
    }
    1
}

unsafe fn ethnl_update_irq_moder(
    irq_moder: *mut dim_irq_moder, irq_field: *mut u16, attr_type: u32,
    tb: *mut *mut nlattr, coal_bit: u8, modified: *mut bool, extack: *mut netlink_ext_ack,
) -> i32 {
    let attr = *tb.add(attr_type as usize);
    if attr.is_null() { return 0; }
    if (*irq_moder).coal_flags & coal_bit == 0 {
        rust_coalesce_bad_attr(extack, attr);
        return -(EOPNOTSUPP as i32);
    }
    let val = rust_coalesce_nla_get_u32(attr);
    // Deliberately compare before u16 narrowing, exactly as the C source does.
    if *irq_field as u32 == val { return 0; }
    *irq_field = val as u16;
    *modified = true;
    0
}

unsafe fn ethnl_update_profile(
    dev: *mut net_device, dst: *mut *mut dim_cq_moder, nests: *const nlattr,
    modified: *mut bool, extack: *mut netlink_ext_ack,
) -> i32 {
    if nests.is_null() { return 0; }
    if (*dst).is_null() { return -(EOPNOTSUPP as i32); }
    let old_profile = rust_coalesce_rtnl_dereference(dst);
    let len = NET_DIM_PARAMS_NUM_PROFILES as usize * size_of::<dim_cq_moder>();
    let new_profile = rust_coalesce_kmemdup(old_profile.cast(), len).cast::<dim_cq_moder>();
    if new_profile.is_null() { return -(ENOMEM as i32); }
    let irq_moder = rust_coalesce_irq_moder(dev);
    let mut tb = [null_mut::<nlattr>(); ETHTOOL_A_IRQ_MODERATION_COMPS as usize + 1];
    let mut rem = rust_coalesce_nla_len(nests);
    let mut nest = rust_coalesce_nla_data(nests).cast::<nlattr>();
    let mut i = 0usize;
    while rust_coalesce_nla_ok(nest, rem) {
        if rust_coalesce_nla_type(nest) == ETHTOOL_A_PROFILE_IRQ_MODERATION as i32 {
            if i >= NET_DIM_PARAMS_NUM_PROFILES as usize {
                rust_coalesce_bad_attr(extack, nest);
                kfree(new_profile.cast());
                return -(E2BIG as i32);
            }
            let ret = rust_coalesce_nla_parse_nested(tb.as_mut_ptr(),
                ETHTOOL_A_IRQ_MODERATION_COMPS as i32, nest,
                addr_of!(coalesce_irq_moderation_policy.0).cast(), extack);
            if ret != 0 { kfree(new_profile.cast()); return ret; }
            macro_rules! update {
                ($field:ident, $attr:ident, $flag:ident) => {
                    let ret = ethnl_update_irq_moder(irq_moder,
                        addr_of_mut!((*new_profile.add(i)).$field), $attr,
                        tb.as_mut_ptr(), $flag as u8, modified, extack);
                    if ret != 0 { kfree(new_profile.cast()); return ret; }
                };
            }
            update!(usec, ETHTOOL_A_IRQ_MODERATION_USEC, RUST_DIM_COALESCE_USEC);
            update!(pkts, ETHTOOL_A_IRQ_MODERATION_PKTS, RUST_DIM_COALESCE_PKTS);
            update!(comps, ETHTOOL_A_IRQ_MODERATION_COMPS, RUST_DIM_COALESCE_COMPS);
            i += 1;
        }
        nest = rust_coalesce_nla_next(nest, &mut rem);
    }
    // As in C, a successfully parsed profile replaces the allocation even if
    // no field changed, with the original released after an RCU grace period.
    rust_coalesce_rcu_assign(dst, new_profile);
    rust_coalesce_kfree_rcu(old_profile);
    0
}

unsafe fn __ethnl_set_coalesce(
    req_info: *mut ethnl_req_info, info: *mut genl_info, dual_change: *mut bool,
) -> i32 {
    let mut kernel_coalesce: kernel_ethtool_coalesce = zeroed();
    let mut coalesce: ethtool_coalesce = zeroed();
    let dev = (*req_info).dev;
    let tb = rust_coalesce_attrs(info);
    let extack = rust_coalesce_extack(info);
    let mut modified = false;
    let mut mod_mode = false;
    // The native request dispatcher calls set_validate before set.
    let ret = rust_coalesce_get_op(dev).unwrap_unchecked()(dev,
        &mut coalesce, &mut kernel_coalesce, extack);
    if ret < 0 { return ret; }
    macro_rules! update {
        ($owner:ident, $field:ident, $attr:ident) => {
            rust_coalesce_update_u32(&mut $owner.$field, *tb.add($attr as usize), &mut modified);
        };
    }
    update!(coalesce, rx_coalesce_usecs, ETHTOOL_A_COALESCE_RX_USECS);
    update!(coalesce, rx_max_coalesced_frames, ETHTOOL_A_COALESCE_RX_MAX_FRAMES);
    update!(coalesce, rx_coalesce_usecs_irq, ETHTOOL_A_COALESCE_RX_USECS_IRQ);
    update!(coalesce, rx_max_coalesced_frames_irq, ETHTOOL_A_COALESCE_RX_MAX_FRAMES_IRQ);
    update!(coalesce, tx_coalesce_usecs, ETHTOOL_A_COALESCE_TX_USECS);
    update!(coalesce, tx_max_coalesced_frames, ETHTOOL_A_COALESCE_TX_MAX_FRAMES);
    update!(coalesce, tx_coalesce_usecs_irq, ETHTOOL_A_COALESCE_TX_USECS_IRQ);
    update!(coalesce, tx_max_coalesced_frames_irq, ETHTOOL_A_COALESCE_TX_MAX_FRAMES_IRQ);
    update!(coalesce, stats_block_coalesce_usecs, ETHTOOL_A_COALESCE_STATS_BLOCK_USECS);
    update!(coalesce, pkt_rate_low, ETHTOOL_A_COALESCE_PKT_RATE_LOW);
    update!(coalesce, rx_coalesce_usecs_low, ETHTOOL_A_COALESCE_RX_USECS_LOW);
    update!(coalesce, rx_max_coalesced_frames_low, ETHTOOL_A_COALESCE_RX_MAX_FRAMES_LOW);
    update!(coalesce, tx_coalesce_usecs_low, ETHTOOL_A_COALESCE_TX_USECS_LOW);
    update!(coalesce, tx_max_coalesced_frames_low, ETHTOOL_A_COALESCE_TX_MAX_FRAMES_LOW);
    update!(coalesce, pkt_rate_high, ETHTOOL_A_COALESCE_PKT_RATE_HIGH);
    update!(coalesce, rx_coalesce_usecs_high, ETHTOOL_A_COALESCE_RX_USECS_HIGH);
    update!(coalesce, rx_max_coalesced_frames_high, ETHTOOL_A_COALESCE_RX_MAX_FRAMES_HIGH);
    update!(coalesce, tx_coalesce_usecs_high, ETHTOOL_A_COALESCE_TX_MAX_FRAMES_HIGH);
    update!(coalesce, tx_max_coalesced_frames_high, ETHTOOL_A_COALESCE_TX_MAX_FRAMES_HIGH);
    update!(coalesce, rate_sample_interval, ETHTOOL_A_COALESCE_RATE_SAMPLE_INTERVAL);
    update!(kernel_coalesce, tx_aggr_max_bytes, ETHTOOL_A_COALESCE_TX_AGGR_MAX_BYTES);
    update!(kernel_coalesce, tx_aggr_max_frames, ETHTOOL_A_COALESCE_TX_AGGR_MAX_FRAMES);
    update!(kernel_coalesce, tx_aggr_time_usecs, ETHTOOL_A_COALESCE_TX_AGGR_TIME_USECS);
    update!(kernel_coalesce, rx_cqe_frames, ETHTOOL_A_COALESCE_RX_CQE_FRAMES);
    update!(kernel_coalesce, rx_cqe_nsecs, ETHTOOL_A_COALESCE_RX_CQE_NSECS);
    let irq_moder = rust_coalesce_irq_moder(dev);
    if !irq_moder.is_null() && (*irq_moder).profile_flags & RUST_DIM_PROFILE_RX as u8 != 0 {
        let ret = ethnl_update_profile(dev, addr_of_mut!((*irq_moder).rx_profile),
            *tb.add(ETHTOOL_A_COALESCE_RX_PROFILE as usize), &mut modified, extack);
        if ret < 0 { return ret; }
    }
    if !irq_moder.is_null() && (*irq_moder).profile_flags & RUST_DIM_PROFILE_TX as u8 != 0 {
        let ret = ethnl_update_profile(dev, addr_of_mut!((*irq_moder).tx_profile),
            *tb.add(ETHTOOL_A_COALESCE_TX_PROFILE as usize), &mut modified, extack);
        if ret < 0 { return ret; }
    }
    rust_coalesce_update_bool32(&mut coalesce.use_adaptive_rx_coalesce,
        *tb.add(ETHTOOL_A_COALESCE_USE_ADAPTIVE_RX as usize), &mut mod_mode);
    rust_coalesce_update_bool32(&mut coalesce.use_adaptive_tx_coalesce,
        *tb.add(ETHTOOL_A_COALESCE_USE_ADAPTIVE_TX as usize), &mut mod_mode);
    rust_coalesce_update_u8(&mut kernel_coalesce.use_cqe_mode_tx,
        *tb.add(ETHTOOL_A_COALESCE_USE_CQE_MODE_TX as usize), &mut mod_mode);
    rust_coalesce_update_u8(&mut kernel_coalesce.use_cqe_mode_rx,
        *tb.add(ETHTOOL_A_COALESCE_USE_CQE_MODE_RX as usize), &mut mod_mode);
    *dual_change = modified && mod_mode;
    if !modified && !mod_mode { return 0; }
    let ret = rust_coalesce_set_op(dev).unwrap_unchecked()(dev,
        &mut coalesce, &mut kernel_coalesce, extack);
    if ret < 0 { ret } else { 1 }
}

unsafe extern "C" fn ethnl_set_coalesce(req_info: *mut ethnl_req_info, info: *mut genl_info) -> i32 {
    let mut dual_change = false;
    let ret = __ethnl_set_coalesce(req_info, info, &mut dual_change);
    if ret < 0 { return ret; }
    // Drivers may reset parameters when the mode changes. Re-read and reapply
    // once in that case, retaining the first call's notification result.
    if ret != 0 && dual_change {
        let err = __ethnl_set_coalesce(req_info, info, &mut dual_change);
        if err < 0 { return err; }
    }
    ret
}

// coalesce.c has native link-visible data symbols, but no EXPORT_SYMBOL macro.
// Match that ABI with no_mangle, without adding module-export metadata.
#[no_mangle]
static ethnl_coalesce_request_ops: RequestOps = RequestOps(ethnl_request_ops {
    request_cmd: ETHTOOL_MSG_COALESCE_GET as u8,
    reply_cmd: ETHTOOL_MSG_COALESCE_GET_REPLY as u8,
    hdr_attr: ETHTOOL_A_COALESCE_HEADER as u16,
    req_info_size: size_of::<coalesce_req_info>() as u32,
    reply_data_size: size_of::<coalesce_reply_data>() as u32,
    allow_nodev_do: false,
    set_ntf_cmd: ETHTOOL_MSG_COALESCE_NTF as u8,
    parse_request: None,
    prepare_data: Some(coalesce_prepare_data),
    reply_size: Some(coalesce_reply_size),
    fill_reply: Some(coalesce_fill_reply),
    cleanup_data: None,
    set_validate: Some(ethnl_set_coalesce_validate),
    set: Some(ethnl_set_coalesce),
});
