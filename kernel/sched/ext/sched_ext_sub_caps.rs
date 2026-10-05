// SPDX-License-Identifier: GPL-2.0

struct ScxSubRcuGuard(core::marker::PhantomData<*mut ()>);

impl ScxSubRcuGuard {
    unsafe fn lock() -> Self {
        // SAFETY: Caller is a synchronous native kfunc/recursion callback whose
        // context permits ordinary RCU read locking; guard never crosses tasks.
        unsafe { lupos_scx_sub_rcu_lock() };
        Self(core::marker::PhantomData)
    }
}
impl Drop for ScxSubRcuGuard {
    fn drop(&mut self) {
        // SAFETY: This non-Send/non-Sync guard owns exactly one read lock on the
        // same synchronous stack; no path transfers or forgets its ownership.
        unsafe { lupos_scx_sub_rcu_unlock() };
    }
}

unsafe fn scx_pstack_recursion(prog: *mut bpf_prog, op: *const c_char) {
    // SAFETY: The callback pins prog/aux; RCU pins a resolved scheduler through
    // fixed native diagnostics. op is one of the native static string literals.
    unsafe {
        let _rcu = ScxSubRcuGuard::lock();
        let sch = lupos_scx_sub_prog_sched((*prog).aux);
        if lupos_scx_sub_unlikely_recursion(sch.is_null()) {
            return;
        }
        lupos_scx_sub_error_recursion(sch, op);
    }
}

/// Report recursive dispatch through the pstack callback.
///
/// # Safety
/// Native pstack handling pins prog and its aux for this synchronous callback.
#[no_mangle]
pub unsafe extern "C" fn scx_pstack_recursion_on_dispatch(prog: *mut bpf_prog) {
    // SAFETY: Native string has static lifetime; recursion helper pins sch.
    unsafe { scx_pstack_recursion(prog, lupos_scx_sub_dispatch_name()) };
}

/// Report recursive capability notification through the pstack callback.
///
/// # Safety
/// Native pstack handling pins prog and its aux for this synchronous callback.
#[no_mangle]
pub unsafe extern "C" fn scx_pstack_recursion_on_caps_updated(prog: *mut bpf_prog) {
    // SAFETY: Native string has static lifetime; recursion helper pins sch.
    unsafe { scx_pstack_recursion(prog, lupos_scx_sub_caps_name()) };
}

/// Trigger dispatch of an immediate child with effective CPU access.
///
/// # Safety
/// Invoked by the native BPF kfunc entry with verified implicit aux in a valid
/// locked dispatch context. The native locked rq and previous task remain live.
#[export_name = "lupos_scx_sub_dispatch"]
pub unsafe extern "C" fn scx_bpf_sub_dispatch(cgroup_id: u64, aux: *const bpf_prog_aux) -> bool {
    // SAFETY: Native context validation supplies rq; RCU pins resolved parent
    // and child while the separate ext dispatch owner performs its operation.
    unsafe {
        let rq = lupos_scx_sub_locked_rq();
        let _rcu = ScxSubRcuGuard::lock();
        let parent = lupos_scx_sub_prog_sched(aux);
        if lupos_scx_sub_unlikely_dispatch_parent(parent.is_null()) {
            return false;
        }
        let child = scx_find_sub_sched(cgroup_id);
        if lupos_scx_sub_unlikely_dispatch_child(child.is_null()) {
            return false;
        }
        if lupos_scx_sub_unlikely_dispatch_distant(lupos_scx_sub_parent(child) != parent) {
            lupos_scx_sub_error_dispatch(parent, cgroup_id);
            return false;
        }
        if lupos_scx_sub_missing_caps(child, lupos_scx_sub_cpu(rq), SCX_CAP_BASE as u64) != 0 {
            return false;
        }
        lupos_scx_sub_dispatch_sched(child, rq, (*rq).scx.sub_dispatch_prev, true) != SCX_DSP_NONE
    }
}

unsafe fn sub_cap_preamble(cgroup_id: u64, caps: u64, aux: *const bpf_prog_aux,
    parent_out: *mut *mut scx_sched, child_out: *mut *mut scx_sched) -> c_int
{
    // SAFETY: IRQ-disabled kfunc context pins RCU scheduler/hash readers. Output
    // slots belong exclusively to the caller and are only read after success.
    unsafe {
        let parent = lupos_scx_sub_prog_sched(aux);
        if lupos_scx_sub_unlikely_preamble_parent(parent.is_null()) {
            return -(LUPOS_SCX_SUB_ENODEV as c_int);
        }
        if !lupos_scx_sub_is_cid_type() {
            lupos_scx_sub_error_preamble_cid_form(parent);
            return -(LUPOS_SCX_SUB_EOPNOTSUPP as c_int);
        }
        let child = scx_find_sub_sched(cgroup_id);
        if lupos_scx_sub_unlikely_preamble_child(child.is_null()) {
            return -(LUPOS_SCX_SUB_ENODEV as c_int);
        }
        if lupos_scx_sub_unlikely_preamble_distant(lupos_scx_sub_parent(child) != parent) {
            lupos_scx_sub_error_preamble_direct_child(parent, cgroup_id);
            return -(LUPOS_SCX_SUB_EINVAL as c_int);
        }
        if lupos_scx_sub_unlikely_preamble_caps(caps & !(__SCX_CAP_ALL as u64) != 0) {
            lupos_scx_sub_error_preamble_caps(parent, caps);
            return -(LUPOS_SCX_SUB_EINVAL as c_int);
        }
        *parent_out = parent;
        *child_out = child;
        0
    }
}

unsafe fn queue_changed_cids(sch: *mut scx_sched, cmask: *const scx_cmask) {
    // SAFETY: Kernel scratch mask geometry and zero padding are trusted. Caller
    // holds the shard lock. Snapshot each word exactly once, like the C iterator.
    unsafe {
        let base = ((*cmask).base & !63u32) as u64;
        for wi in 0..lupos_scx_sub_cmask_used_words(cmask) {
            let mut word = lupos_scx_sub_cmask_read_word(cmask, wi);
            while word != 0 {
                let cid = base + wi as u64 * 64 + word.trailing_zeros() as u64;
                queue_sync_ecaps(sch, cid as c_int);
                word &= word - 1;
            }
        }
    }
}

unsafe fn frame_scratch(mask: *mut scx_cmask, slice: *const scx_cmask) {
    // SAFETY: Both are native fixed-capacity scratch masks. slice's geometry
    // came from a validated snapshot and is no larger than one CID shard.
    unsafe {
        lupos_scx_sub_cmask_init_capacity(mask, (*slice).base, (*slice).nr_cids,
            LUPOS_SCX_SUB_SHARD_MAX_CPUS);
    }
}

/// Grant a direct child only CIDs on which its parent holds every requested cap.
///
/// # Safety
/// Called through the verified native BPF wrapper. aux is pinned; cmask and
/// optional denied are verifier-approved arena addresses belonging to the caller.
/// The native adapter supplies irqsave protection and bounded automatic masks.
#[export_name = "lupos_scx_sub_grant"]
pub unsafe extern "C" fn scx_bpf_sub_grant(cgroup_id: u64, caps: u64,
    cmask: *const scx_cmask, denied: *mut scx_cmask, aux: *const bpf_prog_aux) -> c_int
{
    // SAFETY: The synchronous native storage adapter neither dereferences raw
    // arena input nor implements delegation. It returns the Rust body result.
    unsafe { lupos_scx_sub_with_grant_masks(cgroup_id, caps, cmask, denied, aux) }
}

/// Rust delegation body using native fixed-capacity stack storage.
///
/// # Safety
/// Only the native grant adapter calls this with IRQs disabled and five distinct
/// initialized kernel scratch masks plus an empty list. All remain live through
/// recursive notifications; none of their pointers may escape this call.
#[export_name = "lupos_scx_sub_grant_locked"]
pub unsafe extern "C" fn scx_sub_grant_locked(cgroup_id: u64, caps: u64,
    cmask: *const scx_cmask, denied: *mut scx_cmask, aux: *const bpf_prog_aux,
    scratch: *const lupos_scx_sub_grant_scratch) -> c_int
{
    // SAFETY: Arena ref constructors snapshot and validate geometry before any
    // copy. The native stack owns all flexible-array capacity; nested shard
    // locks protect hierarchy subset invariants and are released before BPF ops.
    unsafe {
        let mut parent = ptr::null_mut();
        let mut child = ptr::null_mut();
        let ret = sub_cap_preamble(cgroup_id, caps, aux, &mut parent, &mut child);
        if ret != 0 {
            return ret;
        }
        let mut ref_storage = core::mem::MaybeUninit::<scx_cmask_ref>::uninit();
        let reference = ref_storage.as_mut_ptr();
        let ret = scx_cmask_ref_init(parent, cmask, reference);
        if ret != 0 {
            lupos_scx_sub_error_grant_cmask(parent, ret);
            return ret;
        }
        let mut denied_storage = core::mem::MaybeUninit::<scx_cmask_ref>::uninit();
        let denied_ref = denied_storage.as_mut_ptr();
        if !denied.is_null() {
            let ret = scx_cmask_ref_init(parent, denied, denied_ref);
            if ret != 0 {
                lupos_scx_sub_error_denied(parent, ret);
                return ret;
            }
        }
        let slice = (*scratch).slice;
        let granted = (*scratch).granted;
        let changed = (*scratch).changed;
        let delta = (*scratch).delta;
        let to_deliver = (*scratch).to_deliver;
        let mut any_denied = false;
        for si in (*reference).shard_first..(*reference).shard_end {
            let pps = *(*parent).pshard.add(si as usize);
            let cps = *(*child).pshard.add(si as usize);
            scx_cmask_ref_shard(reference, si, slice);
            if scx_cmask_empty(slice) {
                continue;
            }
            frame_scratch(granted, slice);
            frame_scratch(changed, slice);
            frame_scratch(delta, slice);
            scx_cmask_copy(granted, slice);
            lupos_scx_sub_ps_lock(pps);
            lupos_scx_sub_ps_lock_nested(cps);
            let mut bits = caps;
            while bits != 0 {
                let bit = bits.trailing_zeros();
                scx_cmask_and(granted, lupos_scx_sub_cap_cmask(pps, bit));
                bits &= bits - 1;
            }
            let mut granted_caps = 0;
            bits = caps;
            while bits != 0 {
                let bit = bits.trailing_zeros();
                let ccm = lupos_scx_sub_cap_cmask(cps, bit);
                scx_cmask_copy(delta, granted);
                scx_cmask_andnot(delta, ccm);
                if !scx_cmask_empty(delta) {
                    scx_cmask_or(ccm, delta);
                    scx_cmask_or(changed, delta);
                    granted_caps |= 1u64 << bit;
                }
                bits &= bits - 1;
            }
            if granted_caps != 0 {
                caps_updated_record(cps, changed, granted_caps, to_deliver);
                queue_changed_cids(child, changed);
            }
            lupos_scx_sub_ps_unlock(cps);
            lupos_scx_sub_ps_unlock(pps);
            if !scx_cmask_subset(slice, granted) {
                any_denied = true;
                if !denied.is_null() {
                    let local_denied = (*scratch).denied;
                    frame_scratch(local_denied, slice);
                    scx_cmask_copy(local_denied, slice);
                    scx_cmask_andnot(local_denied, granted);
                    scx_cmask_ref_or(denied_ref, local_denied);
                }
            }
        }
        caps_updated_deliver(to_deliver);
        if any_denied { -(LUPOS_SCX_SUB_EPERM as c_int) } else { 0 }
    }
}

/// Revoke requested caps from a direct child and every affected descendant.
///
/// # Safety
/// Called through the verified native BPF wrapper with pinned aux and an arena
/// mask authorized by that caller. Native adapter disables IRQs and owns scratch.
#[export_name = "lupos_scx_sub_revoke"]
pub unsafe extern "C" fn scx_bpf_sub_revoke(cgroup_id: u64, caps: u64,
    cmask: *const scx_cmask, aux: *const bpf_prog_aux)
{
    // SAFETY: The native adapter only establishes automatic storage and IRQ
    // lifetime, then synchronously enters the Rust subtree-revocation owner.
    unsafe { lupos_scx_sub_with_revoke_masks(cgroup_id, caps, cmask, aux) };
}

/// Rust revoke body with native masks and a deferred notification list.
///
/// # Safety
/// Only the native revoke adapter calls this under irqsave with three distinct
/// fixed-capacity masks and an empty initialized list live for the full call.
#[export_name = "lupos_scx_sub_revoke_locked"]
pub unsafe extern "C" fn scx_sub_revoke_locked(cgroup_id: u64, caps: u64,
    cmask: *const scx_cmask, aux: *const bpf_prog_aux,
    slice: *mut scx_cmask, changed: *mut scx_cmask, delta: *mut scx_cmask,
    to_deliver: *mut list_head)
{
    // SAFETY: Validated snapshot geometry bounds all arena access. Origin
    // parent lock spans each subtree walk; nested descendant locks serialize
    // actual mutation. No callback is delivered until every shard lock is gone.
    unsafe {
        let mut parent = ptr::null_mut();
        let mut child = ptr::null_mut();
        if sub_cap_preamble(cgroup_id, caps, aux, &mut parent, &mut child) != 0 {
            return;
        }
        let mut storage = core::mem::MaybeUninit::<scx_cmask_ref>::uninit();
        let reference = storage.as_mut_ptr();
        let ret = scx_cmask_ref_init(parent, cmask, reference);
        if ret != 0 {
            lupos_scx_sub_error_revoke_cmask(parent, ret);
            return;
        }
        for si in (*reference).shard_first..(*reference).shard_end {
            scx_cmask_ref_shard(reference, si, slice);
            if scx_cmask_empty(slice) {
                continue;
            }
            let pps = *(*parent).pshard.add(si as usize);
            lupos_scx_sub_ps_lock(pps);
            let mut pos = scx_next_descendant_pre(ptr::null_mut(), child);
            while !pos.is_null() {
                let ps = *(*pos).pshard.add(si as usize);
                frame_scratch(changed, slice);
                frame_scratch(delta, slice);
                let mut revoked = 0;
                lupos_scx_sub_ps_lock_nested(ps);
                let mut bits = caps;
                while bits != 0 {
                    let bit = bits.trailing_zeros();
                    let cm = lupos_scx_sub_cap_cmask(ps, bit);
                    scx_cmask_copy(delta, cm);
                    scx_cmask_and(delta, slice);
                    if !scx_cmask_empty(delta) {
                        scx_cmask_andnot(cm, delta);
                        scx_cmask_or(changed, delta);
                        revoked |= 1u64 << bit;
                    }
                    bits &= bits - 1;
                }
                if revoked != 0 {
                    caps_updated_record(ps, changed, revoked, to_deliver);
                    queue_changed_cids(pos, changed);
                }
                lupos_scx_sub_ps_unlock(ps);
                pos = if revoked != 0 { scx_next_descendant_pre(pos, child) }
                    else { scx_skip_subtree_pre(pos, child) };
            }
            lupos_scx_sub_ps_unlock(pps);
        }
        caps_updated_deliver(to_deliver);
    }
}

/// Read the literal cap union of self or one immediate child into an arena mask.
///
/// # Safety
/// Native BPF wrapper verifies the implicit aux and arena output argument.
/// The adapter pins execution with irqsave and provides bounded scratch storage.
#[export_name = "lupos_scx_sub_caps"]
pub unsafe extern "C" fn scx_bpf_sub_caps(cgroup_id: u64, caps: u64,
    out: *mut scx_cmask, aux: *const bpf_prog_aux) -> c_int
{
    // SAFETY: No dereference precedes the Rust body's snapshot validation.
    unsafe { lupos_scx_sub_with_caps_mask(cgroup_id, caps, out, aux) }
}

/// Rust cap-read body using a native fixed-capacity output scratch mask.
///
/// # Safety
/// Only the native caps adapter calls this under irqsave; local_out is a live
/// distinct kernel mask with SCX_CID_SHARD_MAX_CPUS backing capacity.
#[export_name = "lupos_scx_sub_caps_locked"]
pub unsafe extern "C" fn scx_sub_caps_locked(cgroup_id: u64, caps: u64,
    out: *mut scx_cmask, aux: *const bpf_prog_aux, local_out: *mut scx_cmask) -> c_int
{
    // SAFETY: Acquire publication precedes every shard/CID read. Snapshot
    // geometry, never mutable live arena header values, bounds output writes.
    unsafe {
        let sch = lupos_scx_sub_prog_sched(aux);
        if lupos_scx_sub_unlikely_read_sched(sch.is_null()) {
            return -(LUPOS_SCX_SUB_ENODEV as c_int);
        }
        if !lupos_scx_sub_is_cid_type() {
            lupos_scx_sub_error_read_cid_form(sch);
            return -(LUPOS_SCX_SUB_EOPNOTSUPP as c_int);
        }
        if lupos_scx_sub_unlikely_read_caps(caps & !(__SCX_CAP_ALL as u64) != 0) {
            lupos_scx_sub_error_read_caps(sch, caps);
            return -(LUPOS_SCX_SUB_EINVAL as c_int);
        }
        let target = if cgroup_id != 0 {
            let child = scx_find_sub_sched(cgroup_id);
            if lupos_scx_sub_unlikely_read_child(child.is_null()) {
                return -(LUPOS_SCX_SUB_ENODEV as c_int);
            }
            if lupos_scx_sub_unlikely_read_distant(lupos_scx_sub_parent(child) != sch) {
                lupos_scx_sub_error_read_direct_child(sch, cgroup_id);
                return -(LUPOS_SCX_SUB_EINVAL as c_int);
            }
            child
        } else { sch };
        let pshard = lupos_scx_sub_acquire_pshards(target);
        if lupos_scx_sub_unlikely_read_uninitialized(pshard.is_null()) {
            lupos_scx_sub_error_uninitialized_caps(sch);
            return -(LUPOS_SCX_SUB_ENODEV as c_int);
        }
        let mut storage = core::mem::MaybeUninit::<scx_cmask_ref>::uninit();
        let reference = storage.as_mut_ptr();
        let ret = scx_cmask_ref_init(sch, out, reference);
        if ret != 0 {
            lupos_scx_sub_error_out(sch, ret);
            return ret;
        }
        for si in (*reference).shard_first..(*reference).shard_end {
            let shard = lupos_scx_sub_shard_range_all(si);
            lupos_scx_sub_cmask_init_capacity(local_out, (*shard).base_cid as u32,
                (*shard).nr_cids as u32, LUPOS_SCX_SUB_SHARD_MAX_CPUS);
            let mut bits = caps;
            while bits != 0 {
                let bit = bits.trailing_zeros();
                scx_cmask_or(local_out, lupos_scx_sub_cap_cmask(*pshard.add(si as usize), bit));
                bits &= bits - 1;
            }
            scx_cmask_ref_copy(reference, local_out);
        }
        0
    }
}

/// Request asynchronous eviction of an immediate child with a BPF format reason.
///
/// # Safety
/// Native BPF verifier supplies aux, readable format/data and their validated
/// length. RCU pins resolved schedulers; native formatting owns argument ABI.
#[export_name = "lupos_scx_sub_kill_bstr"]
pub unsafe extern "C" fn scx_bpf_sub_kill_bstr(cgroup_id: u64, fmt: *mut c_char,
    data: *mut kernel::ffi::c_ulonglong, data__sz: u32, aux: *const bpf_prog_aux) -> c_int
{
    // SAFETY: The RCU guard spans every resolver/exit access and the native leaf
    // preserves exit kind, zero exit code, parent attribution and BPF formatting.
    unsafe {
        let _rcu = ScxSubRcuGuard::lock();
        let parent = lupos_scx_sub_prog_sched(aux);
        if lupos_scx_sub_unlikely_kill_parent(parent.is_null()) {
            return -(LUPOS_SCX_SUB_ENODEV as c_int);
        }
        if !lupos_scx_sub_is_cid_type() {
            lupos_scx_sub_error_kill_cid_form(parent);
            return -(LUPOS_SCX_SUB_EOPNOTSUPP as c_int);
        }
        let child = scx_find_sub_sched(cgroup_id);
        if lupos_scx_sub_unlikely_kill_child(child.is_null()) {
            return -(LUPOS_SCX_SUB_ENODEV as c_int);
        }
        if lupos_scx_sub_unlikely_kill_distant(lupos_scx_sub_parent(child) != parent) {
            lupos_scx_sub_error_kill_direct_child(parent, cgroup_id);
            return -(LUPOS_SCX_SUB_EINVAL as c_int);
        }
        lupos_scx_sub_exit_bstr(child, parent, fmt, data, data__sz);
        0
    }
}
