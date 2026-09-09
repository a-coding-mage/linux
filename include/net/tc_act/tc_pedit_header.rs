/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// <net/act_api.h>, <linux/tc_act/tc_pedit.h>, and <linux/types.h>.

#[repr(C)]
pub struct tcf_pedit_key_ex {
    pub htype: pedit_header_type,
    pub cmd: pedit_cmd,
}

#[repr(C)]
pub struct tcf_pedit_parms {
    pub tcfp_keys: *mut tc_pedit_key,
    pub tcfp_keys_ex: *mut tcf_pedit_key_ex,
    pub action: ::core::ffi::c_int,
    pub tcfp_nkeys: u8,
    pub tcfp_flags: u8,
    pub rcu: rcu_head,
}

#[repr(C)]
pub struct tcf_pedit {
    pub common: tc_action,
    pub parms: *mut tcf_pedit_parms,
}

#[inline]
pub unsafe fn to_pedit(a: *mut tc_action) -> *mut tcf_pedit {
    a as *mut tcf_pedit
}

#[inline]
pub unsafe fn to_pedit_parms(a: *mut tc_action) -> *mut tcf_pedit_parms {
    rcu_dereference((*to_pedit(a)).parms)
}

#[inline]
pub unsafe fn is_tcf_pedit(a: *const tc_action) -> bool {
    // CONFIG_NET_CLS_ACT controls this block at build time in the C source.
    if !(*a).ops.is_null() && (*(*a).ops).id == TCA_ID_PEDIT {
        return true;
    }
    false
}

/* Must be called with act->tcfa_lock held to ensure consistency of parallel
 * reads of the same action's pedit keys (e.g. flow_offload count vs fill).
 * Note, this is only used for pedit offload.
 */
#[inline]
pub unsafe fn tcf_pedit_nkeys_locked(a: *const tc_action) -> ::core::ffi::c_int {
    lockdep_assert_held(&(*a).tcfa_lock);
    rcu_dereference_protected(
        (*to_pedit(a as *mut tc_action)).parms,
        lockdep_is_held(&(*a).tcfa_lock),
    )
    .as_ref()
    .unwrap()
    .tcfp_nkeys as ::core::ffi::c_int
}

#[inline]
pub unsafe fn tcf_pedit_htype(a: *const tc_action, index: ::core::ffi::c_int) -> u32 {
    let mut htype = TCA_PEDIT_KEY_EX_HDR_TYPE_NETWORK;
    rcu_read_lock();
    let parms = to_pedit_parms(a as *mut tc_action);
    if !(*parms).tcfp_keys_ex.is_null() {
        htype = (*(*parms).tcfp_keys_ex.add(index as usize)).htype as u32;
    }
    rcu_read_unlock();
    htype
}

#[inline]
pub unsafe fn tcf_pedit_cmd(a: *const tc_action, index: ::core::ffi::c_int) -> u32 {
    let mut cmd = __PEDIT_CMD_MAX;
    rcu_read_lock();
    let parms = to_pedit_parms(a as *mut tc_action);
    if !(*parms).tcfp_keys_ex.is_null() {
        cmd = (*(*parms).tcfp_keys_ex.add(index as usize)).cmd as u32;
    }
    rcu_read_unlock();
    cmd
}

#[inline]
pub unsafe fn tcf_pedit_mask(a: *const tc_action, index: ::core::ffi::c_int) -> u32 {
    rcu_read_lock();
    let parms = to_pedit_parms(a as *mut tc_action);
    let mask = (*(*parms).tcfp_keys.add(index as usize)).mask;
    rcu_read_unlock();
    mask
}

#[inline]
pub unsafe fn tcf_pedit_val(a: *const tc_action, index: ::core::ffi::c_int) -> u32 {
    rcu_read_lock();
    let parms = to_pedit_parms(a as *mut tc_action);
    let val = (*(*parms).tcfp_keys.add(index as usize)).val;
    rcu_read_unlock();
    val
}

#[inline]
pub unsafe fn tcf_pedit_offset(a: *const tc_action, index: ::core::ffi::c_int) -> u32 {
    rcu_read_lock();
    let parms = to_pedit_parms(a as *mut tc_action);
    let off = (*(*parms).tcfp_keys.add(index as usize)).off;
    rcu_read_unlock();
    off
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
