/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Copyright 2020 NXP */

/* C dependencies: <net/act_api.h>, <linux/tc_act/tc_gate.h> */

#[repr(C)]
pub struct action_gate_entry {
    pub gate_state: u8,
    pub interval: u32,
    pub ipv: i32,
    pub maxoctets: i32,
}

#[repr(C)]
pub struct tcfg_gate_entry {
    pub index: i32,
    pub gate_state: u8,
    pub interval: u32,
    pub ipv: i32,
    pub maxoctets: i32,
    pub list: list_head,
}

#[repr(C)]
pub struct tcf_gate_params {
    pub tcfg_priority: i32,
    pub tcfg_basetime: u64,
    pub tcfg_cycletime: u64,
    pub tcfg_cycletime_ext: u64,
    pub tcfg_flags: u32,
    pub tcfg_clockid: i32,
    pub num_entries: usize,
    pub entries: list_head,
    pub rcu: rcu_head,
}

pub const GATE_ACT_GATE_OPEN: u32 = 1 << 0;
pub const GATE_ACT_PENDING: u32 = 1 << 1;

#[repr(C)]
pub struct tcf_gate {
    pub common: tc_action,
    pub param: *mut tcf_gate_params, // __rcu
    pub current_gate_status: u8,
    pub current_close_time: ktime_t,
    pub current_entry_octets: u32,
    pub current_max_octets: i32,
    pub next_entry: *mut tcfg_gate_entry,
    pub hitimer: hrtimer,
    pub tk_offset: tk_offsets,
}

#[inline]
pub unsafe fn to_gate(a: *mut tc_action) -> *mut tcf_gate {
    a as *mut tcf_gate
}

#[inline]
pub unsafe fn tcf_gate_params_locked(a: *const tc_action) -> *mut tcf_gate_params {
    let gact = to_gate(a as *mut tc_action);
    rcu_dereference_protected(
        (*gact).param,
        lockdep_is_held(&(*gact).tcf_lock),
    )
}

#[inline]
pub unsafe fn tcf_gate_prio(a: *const tc_action) -> i32 {
    let p = tcf_gate_params_locked(a);
    (*p).tcfg_priority
}

#[inline]
pub unsafe fn tcf_gate_basetime(a: *const tc_action) -> u64 {
    let p = tcf_gate_params_locked(a);
    (*p).tcfg_basetime
}

#[inline]
pub unsafe fn tcf_gate_cycletime(a: *const tc_action) -> u64 {
    let p = tcf_gate_params_locked(a);
    (*p).tcfg_cycletime
}

#[inline]
pub unsafe fn tcf_gate_cycletimeext(a: *const tc_action) -> u64 {
    let p = tcf_gate_params_locked(a);
    (*p).tcfg_cycletime_ext
}

#[inline]
pub unsafe fn tcf_gate_num_entries(a: *const tc_action) -> u32 {
    let p = tcf_gate_params_locked(a);
    (*p).num_entries as u32
}

#[inline]
pub unsafe fn tcf_gate_get_list(a: *const tc_action) -> *mut action_gate_entry {
    let p = tcf_gate_params_locked(a);
    let mut i: i32 = 0;
    let mut entry: *mut tcfg_gate_entry = core::ptr::null_mut();

    // Equivalent to list_for_each_entry(entry, &p->entries, list).
    list_for_each_entry!(entry, &(*p).entries, list, { i += 1; });

    if i != (*p).num_entries as i32 {
        return core::ptr::null_mut();
    }

    let oe = kzalloc_objs::<action_gate_entry>((*p).num_entries, GFP_ATOMIC);
    if oe.is_null() {
        return core::ptr::null_mut();
    }

    i = 0;
    // Equivalent to list_for_each_entry(entry, &p->entries, list).
    list_for_each_entry!(entry, &(*p).entries, list, {
        (*oe.add(i as usize)).gate_state = (*entry).gate_state;
        (*oe.add(i as usize)).interval = (*entry).interval;
        (*oe.add(i as usize)).ipv = (*entry).ipv;
        (*oe.add(i as usize)).maxoctets = (*entry).maxoctets;
        i += 1;
    });

    oe
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
