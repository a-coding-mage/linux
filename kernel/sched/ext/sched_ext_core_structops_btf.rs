// SPDX-License-Identifier: GPL-2.0
// F13 additive source repair against ext.c at
// 126a30fae3bba11420ec2fcbde51a0a01bab1b5b. Native metadata/CFI shells and
// primitive leaves remain explicit unqualified C runtime, not Rust coverage.
compile_error!("SOURCE ONLY HOLD: sched_ext struct_ops/BTF ABI and protection remain unqualified");

use super::*;
use kernel::ffi::{c_int, c_ulong, c_void};

/// Validate a verifier context read before the native BTF context checker.
///
/// # Safety
/// The BPF verifier supplies valid prog/info and a positive access size, as at
/// the original C callback. No additional accepted access class is introduced.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_st_valid_access_body(
    off: c_int, size: c_int, type_: bpf_access_type,
    prog: *const bpf_prog, info: *mut bpf_insn_access_aux,
) -> bool {
    unsafe {
        if type_ != BPF_READ { return false; }
        if off < 0 || off as usize >= LUPOS_SCX_ST_CTX_BYTES as usize { return false; }
        if off % size != 0 { return false; }
        lupos_scx_st_ctx_access(off, size, type_, prog, info)
    }
}

/// Common write permission: task_struct.scx.disallow only.
///
/// # Safety
/// reg/its BTF object are verifier-owned and live. Native off+size is defined
/// on the verifier's input domain. The conversions preserve C's unsigned
/// offsetof comparisons, including negative off rejecting the lower bound.
unsafe fn bpf_scx_btf_struct_access_common(
    reg: *const bpf_reg_state, off: c_int, size: c_int,
) -> c_int {
    unsafe {
        let t = lupos_scx_st_reg_type(reg);
        if t == lupos_scx_st_task_type()
            && off as usize >= LUPOS_SCX_ST_DISALLOW as usize
            && off.wrapping_add(size) as usize <= LUPOS_SCX_ST_DISALLOW_END as usize
        {
            return SCALAR_VALUE as c_int;
        }
        -(EACCES as c_int)
    }
}

/// CPU-form additionally permits the original direct slice/vtime BPF stores.
///
/// # Safety
/// Same verifier input domain as the common helper. This checks metadata only;
/// it does not read/write task fields or assert exclusivity against BPF stores.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_st_struct_access_body(
    _log: *mut bpf_verifier_log, reg: *const bpf_reg_state, off: c_int, size: c_int,
) -> c_int {
    unsafe {
        let t = lupos_scx_st_reg_type(reg);
        if t == lupos_scx_st_task_type() {
            if (off as usize >= LUPOS_SCX_ST_SLICE as usize
                && off.wrapping_add(size) as usize <= LUPOS_SCX_ST_SLICE_END as usize)
                || (off as usize >= LUPOS_SCX_ST_VTIME as usize
                    && off.wrapping_add(size) as usize <= LUPOS_SCX_ST_VTIME_END as usize)
            {
                return SCALAR_VALUE as c_int;
            }
        }
        // Preserve the common helper's second native BTF lookup.
        bpf_scx_btf_struct_access_common(reg, off, size)
    }
}

/// CID-form keeps slice/vtime updates on the kfunc path.
///
/// # Safety
/// The native verifier callback supplies live metadata and valid access sizes.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_st_cid_struct_access_body(
    _log: *mut bpf_verifier_log, reg: *const bpf_reg_state, off: c_int, size: c_int,
) -> c_int {
    unsafe { bpf_scx_btf_struct_access_common(reg, off, size) }
}

/// Validate and copy each recognized data member; leave unknown members alone.
///
/// # Safety
/// Native struct_ops supplies suitably sized/aligned live kdata/udata. CID
/// common-byte interpretation depends on F17's exact native offset/tail
/// assertions. No Rust cast between CPU/CID lookalike layouts is used.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_st_init_member_body(
    t: *const btf_type, member: *const btf_member,
    kdata: *mut c_void, udata: *const c_void,
) -> c_int {
    unsafe {
        let moff = lupos_scx_st_member_bit_offset(t, member) / 8;
        match moff {
            LUPOS_SCX_ST_DISPATCH_MAX_BATCH => {
                if lupos_scx_st_udata_u32(udata, moff) > INT_MAX as u32 {
                    return -(E2BIG as c_int);
                }
                lupos_scx_st_set_dispatch_max_batch(kdata, lupos_scx_st_udata_u32(udata, moff));
            }
            LUPOS_SCX_ST_FLAGS => {
                if lupos_scx_st_udata_u64(udata, moff) & !(SCX_OPS_ALL_FLAGS as u64) != 0 {
                    return -(EINVAL as c_int);
                }
                lupos_scx_st_set_flags(kdata, lupos_scx_st_udata_u64(udata, moff));
            }
            LUPOS_SCX_ST_NAME => {
                let ret = lupos_scx_st_copy_name(kdata, udata);
                if ret < 0 { return ret; }
                if ret == 0 { return -(EINVAL as c_int); }
            }
            LUPOS_SCX_ST_TIMEOUT_MS => {
                if lupos_scx_st_msecs_to_jiffies(lupos_scx_st_udata_u32(udata, moff))
                    > SCX_WATCHDOG_MAX_TIMEOUT as c_ulong
                {
                    return -(E2BIG as c_int);
                }
                lupos_scx_st_set_timeout_ms(kdata, lupos_scx_st_udata_u32(udata, moff));
            }
            LUPOS_SCX_ST_EXIT_DUMP_LEN => {
                let len = lupos_scx_st_udata_u32(udata, moff);
                lupos_scx_st_set_exit_dump_len(kdata,
                    if len != 0 { len } else { SCX_EXIT_DUMP_DFL_LEN as u32 });
            }
            LUPOS_SCX_ST_HOTPLUG_SEQ => {
                lupos_scx_st_set_hotplug_seq(kdata, lupos_scx_st_udata_u64(udata, moff));
            }
            LUPOS_SCX_ST_CID_SHARD_SIZE => {
                lupos_scx_st_set_cid_shard_size(kdata, lupos_scx_st_udata_u32(udata, moff));
            }
            LUPOS_SCX_ST_RESCUE_BANDWIDTH_PPT => {
                let bw_ppt = lupos_scx_st_udata_u32(udata, moff);
                if bw_ppt > SCX_RESCUE_MAX_BW_PPT as u32 && bw_ppt != SCX_RESCUE_DISABLE as u32 {
                    return -(E2BIG as c_int);
                }
                lupos_scx_st_set_rescue_bandwidth_ppt(kdata, bw_ppt);
            }
            LUPOS_SCX_ST_RESCUE_QUANTUM_US => {
                let quantum_us = lupos_scx_st_udata_u32(udata, moff);
                if quantum_us > SCX_RESCUE_MAX_QUANTUM_US as u32 { return -(E2BIG as c_int); }
                if quantum_us != 0 && quantum_us < SCX_RESCUE_MIN_QUANTUM_US as u32 {
                    return -(EINVAL as c_int);
                }
                lupos_scx_st_set_rescue_quantum_us(kdata, quantum_us);
            }
            #[cfg(CONFIG_EXT_SUB_SCHED)]
            LUPOS_SCX_ST_SUB_CGROUP_ID => {
                lupos_scx_st_set_sub_cgroup_id(kdata, lupos_scx_st_udata_u64(udata, moff));
            }
            _ => return 0,
        }
        1
    }
}

/// Check sleepability and retain private-stack recursion callback identities.
///
/// # Safety
/// prog/aux are live and owned by verifier initialization. Native leaves retain
/// real bitfield and recursion-handler types; they do not manufacture a layout.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_st_check_member_body(
    t: *const btf_type, member: *const btf_member, prog: *const bpf_prog,
) -> c_int {
    unsafe {
        let moff = lupos_scx_st_member_bit_offset(t, member) / 8;
        match moff {
            LUPOS_SCX_ST_INIT_TASK | LUPOS_SCX_ST_CPU_ONLINE | LUPOS_SCX_ST_CPU_OFFLINE
            | LUPOS_SCX_ST_INIT_CIDS | LUPOS_SCX_ST_INIT | LUPOS_SCX_ST_EXIT
            | LUPOS_SCX_ST_SUB_ATTACH | LUPOS_SCX_ST_SUB_DETACH => {}
            #[cfg(CONFIG_EXT_GROUP_SCHED)]
            LUPOS_SCX_ST_CGROUP_INIT | LUPOS_SCX_ST_CGROUP_EXIT | LUPOS_SCX_ST_CGROUP_PREP_MOVE => {}
            _ => {
                if lupos_scx_st_prog_sleepable(prog) { return -(EINVAL as c_int); }
            }
        }
        #[cfg(CONFIG_EXT_SUB_SCHED)]
        match moff {
            LUPOS_SCX_ST_DISPATCH => lupos_scx_st_request_dispatch_stack(prog),
            LUPOS_SCX_ST_SUB_CAPS_UPDATED => lupos_scx_st_request_caps_stack(prog),
            _ => {}
        }
        0
    }
}

/// Forward the stack-native CPU-form command to F12's real helper.
///
/// # Safety
/// cmd is initialized by the original native designated initializer and remains
/// live until scx_enable completes its synchronous queue/flush transaction.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_st_reg_body(cmd: *mut scx_enable_cmd, link: *mut bpf_link) -> c_int {
    unsafe { lupos_scx_st_enable(cmd, link) }
}

/// Require every non-NULL program arena contribution to name the same map.
///
/// # Safety
/// The native iterator pins prog and synchronously borrows the registration
/// shell's zero-initialized scan. arena.o is only referenced for MMU && 64BIT.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_st_scan_prog_body(prog: *mut bpf_prog, s: *mut scx_arena_scan) -> c_int {
    unsafe {
        #[cfg(all(CONFIG_MMU, CONFIG_64BIT))]
        let arena = lupos_scx_st_prog_arena(prog);
        #[cfg(not(all(CONFIG_MMU, CONFIG_64BIT)))]
        let arena: *mut bpf_map = { let _ = prog; core::ptr::null_mut() };
        if arena.is_null() { return 0; }
        if !(*s).arena.is_null() && (*s).arena != arena {
            (*s).err = -(EINVAL as c_int);
            return 1;
        }
        (*s).arena = arena;
        0
    }
}

/// Scan, pin and transfer CID arena ownership through the real enable command.
///
/// # Safety
/// kdata/link are valid struct_ops registration inputs; cmd/scan are native
/// stack values. F09 consumes cmd.arena_map by clearing it on transfer, while
/// F12 retains the real helper lifetime and synchronous completion protocol.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_st_reg_cid_body(
    kdata: *mut c_void, link: *mut bpf_link,
    cmd: *mut scx_enable_cmd, scan: *mut scx_arena_scan,
) -> c_int {
    unsafe {
        lupos_scx_st_for_each_prog(kdata, scan);
        if (*scan).err != 0 {
            lupos_scx_st_error_multiple_arenas();
            return (*scan).err;
        }
        if (*scan).arena.is_null() {
            lupos_scx_st_error_missing_arena();
            return -(EINVAL as c_int);
        }
        lupos_scx_st_map_inc((*scan).arena);
        lupos_scx_st_cmd_set_arena(cmd, (*scan).arena);
        let ret = lupos_scx_st_enable(cmd, link);
        if !lupos_scx_st_cmd_arena(cmd).is_null() {
            lupos_scx_st_map_put(lupos_scx_st_cmd_arena(cmd));
        }
        ret
    }
}

/// Drain disable before clearing the protected association and releasing kobj.
///
/// # Safety
/// Struct_ops unregister provides the original lifetime/exclusion. The native
/// leaf retains rcu_dereference_protected(..., true) and RCU_INIT_POINTER.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_st_unreg_body(kdata: *mut c_void, _link: *mut bpf_link) {
    unsafe {
        let sch = lupos_scx_st_ops_priv_protected(kdata);
        scx_disable(sch, SCX_EXIT_UNREG);
        scx_flush_disable_work(sch);
        lupos_scx_st_ops_priv_clear(kdata);
        lupos_scx_st_sched_kobject_put(sch);
    }
}

/// Populate the one F13-native task_struct_type identity.
///
/// # Safety
/// btf is the registration-time native BTF object retained by the BPF core.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_st_init_body(btf: *mut btf) -> c_int {
    unsafe { lupos_scx_st_set_task_type(lupos_scx_st_tracing_task_type(btf)); }
    0
}

/// Active replacement remains unsupported, including initialization failures
/// and concurrent unregister. This is the original real rejection behavior.
///
/// # Safety
/// Inputs are supplied by the native struct_ops update callback and unused.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_st_update_body(
    _kdata: *mut c_void, _old_kdata: *mut c_void, _link: *mut bpf_link,
) -> c_int {
    -(EOPNOTSUPP as c_int)
}

/// Original validation callback intentionally adds no checks of its own.
///
/// # Safety
/// kdata is the BPF core's valid callback input. This is not a fallback for
/// missing validation: member validation and F12 scx_validate_ops remain real.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_st_validate_body(_kdata: *mut c_void) -> c_int { 0 }

/// Enforce the native BTF groups and per-op table in the original order.
///
/// # Safety
/// Native BPF verifier pins prog/aux and its current st_ops association. All
/// membership reads refer to the original same-TU local BTF sets; CID/idle
/// metadata is owned elsewhere. moff has the verifier's valid member domain.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_st_context_filter_body(prog: *const bpf_prog, kfunc_id: u32) -> c_int {
    unsafe {
        // Eagerly retain all ten original membership evaluations in order.
        let in_unlocked = lupos_scx_st_in_unlocked(kfunc_id);
        let in_init_cids = lupos_scx_st_in_init_cids(kfunc_id);
        let in_select_cpu = lupos_scx_st_in_select_cpu(kfunc_id);
        let in_enqueue = lupos_scx_st_in_enqueue(kfunc_id);
        let in_dispatch = lupos_scx_st_in_dispatch(kfunc_id);
        let in_cpu_release = lupos_scx_st_in_cpu_release(kfunc_id);
        let in_idle = lupos_scx_st_in_idle(kfunc_id);
        let in_any = lupos_scx_st_in_any(kfunc_id);
        let in_cpu_only = lupos_scx_st_in_cpu_only(kfunc_id);
        let in_cid = lupos_scx_st_in_cid(kfunc_id);

        // cpu_only is an overlap classifier, not an independent SCX group.
        if !(in_unlocked || in_init_cids || in_select_cpu || in_enqueue || in_dispatch
            || in_cpu_release || in_idle || in_any || in_cid)
        {
            return 0;
        }
        if lupos_scx_st_prog_type(prog) == BPF_PROG_TYPE_SYSCALL {
            return if in_unlocked || in_select_cpu || in_idle || in_any || in_cid {
                0
            } else { -(EACCES as c_int) };
        }
        if lupos_scx_st_prog_type(prog) != BPF_PROG_TYPE_STRUCT_OPS {
            return if in_any || in_idle || in_cid { 0 } else { -(EACCES as c_int) };
        }
        // Collection can precede attach; do_check_main rechecks with st_ops.
        if lupos_scx_st_prog_ops(prog).is_null() { return 0; }
        if lupos_scx_st_prog_ops(prog) != lupos_scx_st_cpu_ops()
            && lupos_scx_st_prog_ops(prog) != lupos_scx_st_cid_ops()
        {
            return -(EACCES as c_int);
        }
        // This rejection precedes the unrestricted-group fast path.
        if lupos_scx_st_prog_ops(prog) == lupos_scx_st_cid_ops() && in_cpu_only {
            return -(EACCES as c_int);
        }
        if in_any || in_idle || in_cid { return 0; }
        let moff = lupos_scx_st_prog_member_off(prog);
        let flags = lupos_scx_st_allow_flags(moff);
        if flags & SCX_KF_ALLOW_UNLOCKED as u32 != 0 && in_unlocked { return 0; }
        if flags & SCX_KF_ALLOW_INIT_CIDS as u32 != 0 && in_init_cids { return 0; }
        if flags & SCX_KF_ALLOW_CPU_RELEASE as u32 != 0 && in_cpu_release { return 0; }
        if flags & SCX_KF_ALLOW_DISPATCH as u32 != 0 && in_dispatch { return 0; }
        if flags & SCX_KF_ALLOW_ENQUEUE as u32 != 0 && in_enqueue { return 0; }
        if flags & SCX_KF_ALLOW_SELECT_CPU as u32 != 0 && in_select_cpu { return 0; }
        -(EACCES as c_int)
    }
}
