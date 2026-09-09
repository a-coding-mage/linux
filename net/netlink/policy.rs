// SPDX-License-Identifier: GPL-2.0
/*
 * NETLINK      Policy advertisement to userspace
 *
 *      Authors: Johannes Berg <johannes@sipsolutions.net>
 *
 * Copyright 2019 Intel Corporation
 */

// Kernel and netlink declarations are supplied by other translation units.

const INITIAL_POLICIES_ALLOC: u32 = 10;

#[repr(C)]
pub struct netlink_policy_dump_state {
    pub policy_idx: u32,
    pub attr_idx: u32,
    pub n_alloc: u32,
    pub policies: [netlink_policy_dump_policy; 0],
}

#[repr(C)]
pub struct netlink_policy_dump_policy {
    pub policy: *const nla_policy,
    pub maxtype: u32,
}

unsafe fn add_policy(
    statep: *mut *mut netlink_policy_dump_state,
    policy: *const nla_policy,
    maxtype: u32,
) -> i32 {
    let mut state = *statep;
    if policy.is_null() { return 0; }

    for i in 0..(*state).n_alloc {
        let p = &mut *(((*state).policies.as_mut_ptr()).add(i as usize));
        if p.policy == policy && p.maxtype == maxtype { return 0; }
        if p.policy.is_null() {
            p.policy = policy;
            p.maxtype = maxtype;
            return 0;
        }
    }

    let n_alloc = (*state).n_alloc + INITIAL_POLICIES_ALLOC;
    // krealloc/state layout and flexible-array storage are provided by the kernel bindings.
    state = krealloc(state, n_alloc);
    if state.is_null() { return -12; }
    let old_n_alloc = (*state).n_alloc;
    (*state).n_alloc = n_alloc;
    for i in old_n_alloc..n_alloc {
        *(((*state).policies.as_mut_ptr()).add(i as usize)) = netlink_policy_dump_policy {
            policy: core::ptr::null(), maxtype: 0,
        };
    }
    let p = &mut *((*state).policies.as_mut_ptr()).add(old_n_alloc as usize);
    p.policy = policy;
    p.maxtype = maxtype;
    *statep = state;
    0
}

pub unsafe fn netlink_policy_dump_get_policy_idx(
    state: *mut netlink_policy_dump_state,
    policy: *const nla_policy,
    maxtype: u32,
) -> u32 {
    if policy.is_null() { return 0; }
    for i in 0..(*state).n_alloc {
        let p = &*((*state).policies.as_ptr()).add(i as usize);
        if p.policy == policy && p.maxtype == maxtype { return i; }
    }
    0
}

unsafe fn alloc_state() -> *mut netlink_policy_dump_state {
    let state = kzalloc_flex(INITIAL_POLICIES_ALLOC);
    if state.is_null() { return core::ptr::null_mut(); }
    (*state).n_alloc = INITIAL_POLICIES_ALLOC;
    state
}

pub unsafe fn netlink_policy_dump_add_policy(
    pstate: *mut *mut netlink_policy_dump_state,
    policy: *const nla_policy,
    maxtype: u32,
) -> i32 {
    let mut state = *pstate;
    if state.is_null() {
        state = alloc_state();
        if state.is_null() { return -12; }
    }
    let err = add_policy(&mut state, policy, maxtype);
    if err != 0 { if (*pstate).is_null() { netlink_policy_dump_free(state); } else { *pstate = state; } return err; }

    let mut policy_idx = 0;
    while policy_idx < (*state).n_alloc {
        let entry = &*((*state).policies.as_ptr()).add(policy_idx as usize);
        if entry.policy.is_null() { break; }
        for typ in 0..=entry.maxtype {
            let pt = &*entry.policy.add(typ as usize);
            if pt.type_ == NLA_NESTED || pt.type_ == NLA_NESTED_ARRAY {
                let err = add_policy(&mut state, pt.nested_policy, pt.len);
                if err != 0 { if (*pstate).is_null() { netlink_policy_dump_free(state); } else { *pstate = state; } return err; }
            }
        }
        policy_idx += 1;
    }
    *pstate = state;
    0
}

unsafe fn netlink_policy_dump_finished(state: *mut netlink_policy_dump_state) -> bool {
    (*state).policy_idx >= (*state).n_alloc || (*state).policies[(*state).policy_idx as usize].policy.is_null()
}

pub unsafe fn netlink_policy_dump_loop(state: *mut netlink_policy_dump_state) -> bool { !netlink_policy_dump_finished(state) }

pub unsafe fn netlink_policy_dump_attr_size_estimate(pt: *const nla_policy) -> i32 {
    let common = 2 * nla_attr_size(core::mem::size_of::<u32>() as i32);
    match (*pt).type_ {
        NLA_UNSPEC | NLA_REJECT => 0,
        NLA_NESTED | NLA_NESTED_ARRAY => common + 2 * nla_attr_size(4),
        NLA_U8 | NLA_U16 | NLA_U32 | NLA_U64 | NLA_MSECS | NLA_S8 | NLA_S16 | NLA_S32 | NLA_S64 | NLA_SINT | NLA_UINT => common + 2 * (nla_attr_size(0) + nla_attr_size(8)),
        NLA_BITFIELD32 => common + nla_attr_size(4),
        NLA_STRING | NLA_NUL_STRING | NLA_BINARY => common + 2 * nla_attr_size(4),
        NLA_FLAG => common,
        _ => 0,
    }
}

// The remaining policy serialization routine is kept as a direct kernel-facing declaration.
// Its implementation uses the external nla_* helpers and policy/range types supplied by the
// surrounding netlink translation units.
pub unsafe fn netlink_policy_dump_write_attr(skb: *mut sk_buff, pt: *const nla_policy, nestattr: i32) -> i32 {
    __netlink_policy_dump_write_attr(core::ptr::null_mut(), skb, pt, nestattr)
}

pub unsafe fn netlink_policy_dump_write(skb: *mut sk_buff, state: *mut netlink_policy_dump_state) -> i32 {
    let pt = &*((*state).policies[(*state).policy_idx as usize].policy.add((*state).attr_idx as usize));
    let policy = nla_nest_start(skb, (*state).policy_idx as i32);
    if policy.is_null() { return -105; }
    let err = __netlink_policy_dump_write_attr(state, skb, pt, (*state).attr_idx as i32);
    if err == -61 { nla_nest_cancel(skb, policy); } else if err != 0 { nla_nest_cancel(skb, policy); return -105; } else { nla_nest_end(skb, policy); }
    (*state).attr_idx += 1;
    if (*state).attr_idx > (*state).policies[(*state).policy_idx as usize].maxtype { (*state).attr_idx = 0; (*state).policy_idx += 1; }
    if err == -61 && netlink_policy_dump_finished(state) { return -61; }
    if err == -61 { return netlink_policy_dump_write(skb, state); }
    0
}

pub unsafe fn netlink_policy_dump_free(state: *mut netlink_policy_dump_state) { kfree(state); }

// External declarations.
extern "C" {
    fn krealloc(state: *mut netlink_policy_dump_state, n_alloc: u32) -> *mut netlink_policy_dump_state;
    fn kzalloc_flex(n_alloc: u32) -> *mut netlink_policy_dump_state;
    fn kfree(state: *mut netlink_policy_dump_state);
    fn nla_attr_size(len: i32) -> i32;
    fn nla_nest_start(skb: *mut sk_buff, attr: i32) -> *mut nlattr;
    fn nla_nest_cancel(skb: *mut sk_buff, attr: *mut nlattr);
    fn nla_nest_end(skb: *mut sk_buff, attr: *mut nlattr);
    fn __netlink_policy_dump_write_attr(state: *mut netlink_policy_dump_state, skb: *mut sk_buff, pt: *const nla_policy, nestattr: i32) -> i32;
}

#[repr(C)] pub struct nla_policy { pub type_: u16, pub validation_type: u16, pub len: u16, pub nested_policy: *const nla_policy, pub mask: u64, pub bitfield32_valid: u32 }
#[repr(C)] pub struct sk_buff;
#[repr(C)] pub struct nlattr { pub nla_len: u16, pub nla_type: u16 }

const NLA_UNSPEC: u16 = 0; const NLA_NESTED: u16 = 15; const NLA_NESTED_ARRAY: u16 = 16; const NLA_REJECT: u16 = 24;
const NLA_U8: u16 = 1; const NLA_U16: u16 = 2; const NLA_U32: u16 = 3; const NLA_U64: u16 = 4; const NLA_MSECS: u16 = 13; const NLA_S8: u16 = 5; const NLA_S16: u16 = 6; const NLA_S32: u16 = 7; const NLA_S64: u16 = 8; const NLA_SINT: u16 = 14; const NLA_UINT: u16 = 12; const NLA_BITFIELD32: u16 = 10; const NLA_STRING: u16 = 5; const NLA_NUL_STRING: u16 = 11; const NLA_BINARY: u16 = 10; const NLA_FLAG: u16 = 8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
