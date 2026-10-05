// SPDX-License-Identifier: GPL-2.0
// fair.c:1-7842, e1d84f501551943a11f4c5271e9f5c85d7e15168.
// All caller-held rq locks, RCU, IRQ and preemption requirements are unchanged.
#[no_mangle]
pub static mut sysctl_sched_tunable_scaling: c_uint = b::SCHED_TUNABLESCALING_LOG;
#[no_mangle]
pub static mut sysctl_sched_base_slice: c_uint = 700000;
static mut normalized_sysctl_sched_base_slice: c_uint = 700000;
#[no_mangle]
#[link_section = ".data..read_mostly"]
pub static mut sysctl_sched_migration_cost: c_uint = 500000;
#[cfg(CONFIG_CFS_BANDWIDTH)]
#[no_mangle]
static mut sysctl_sched_cfs_bandwidth_slice: c_uint = 5000;
#[cfg(CONFIG_NUMA_BALANCING)]
#[no_mangle]
static mut sysctl_numa_balancing_promote_rate_limit: c_uint = 65536;
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn setup_sched_thermal_decay_shift(_str: *mut kernel::ffi::c_char) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_warn_thermal_decay();
        1
    }
}
// Weak linkage remains in the native declaration veneer; the arithmetic is Rust.
#[no_mangle]
pub unsafe extern "C" fn rust_fair_arch_asym_cpu_priority(cpu: c_int) -> c_int {
    cpu.wrapping_neg()
}
#[inline]
unsafe fn fits_capacity(cap: c_ulong, capacity: c_ulong) -> bool {
    cap.wrapping_mul(1280) < capacity.wrapping_mul(1024)
}
#[inline]
unsafe fn capacity_greater(cap1: c_ulong, cap2: c_ulong) -> bool {
    cap1.wrapping_mul(1024) > cap2.wrapping_mul(1078)
}
#[inline]
unsafe fn update_load_add(lw: *mut b::load_weight, inc: c_ulong) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (*lw).weight = (*lw).weight.wrapping_add(inc);
        (*lw).inv_weight = 0;
    }
}
#[inline]
unsafe fn update_load_sub(lw: *mut b::load_weight, dec: c_ulong) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (*lw).weight = (*lw).weight.wrapping_sub(dec);
        (*lw).inv_weight = 0;
    }
}
#[inline]
unsafe fn update_load_set(lw: *mut b::load_weight, weight: c_ulong) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (*lw).weight = weight;
        (*lw).inv_weight = 0;
    }
}
unsafe fn get_update_sysctl_factor() -> c_uint {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let cpus = min(b::rust_fair_num_online_cpus(), 8);
        match sysctl_sched_tunable_scaling {
            b::SCHED_TUNABLESCALING_NONE => 1,
            b::SCHED_TUNABLESCALING_LINEAR => cpus,
            _ => 32u32.wrapping_sub(cpus.leading_zeros()),
        }
    }
}
unsafe fn update_sysctl() {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        sysctl_sched_base_slice =
            get_update_sysctl_factor().wrapping_mul(normalized_sysctl_sched_base_slice);
    }
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn sched_init_granularity() {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        update_sysctl();
    }
}
#[no_mangle]
pub unsafe extern "C" fn sched_update_scaling() -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        normalized_sysctl_sched_base_slice = sysctl_sched_base_slice / get_update_sysctl_factor();
        0
    }
}
#[cfg(not(CONFIG_64BIT))]
unsafe fn __update_inv_weight(lw: *mut b::load_weight) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if (*lw).inv_weight != 0 {
            return;
        }
        let w = b::rust_fair_scale_load_down((*lw).weight);
        (*lw).inv_weight = if b::BITS_PER_LONG > 32 && w >= c_uint::MAX as c_ulong {
            1
        } else if w == 0 {
            c_uint::MAX
        } else {
            (c_uint::MAX as c_ulong / w) as c_uint
        };
    }
}
#[cfg(not(CONFIG_64BIT))]
unsafe fn __calc_delta(delta_exec: u64, weight: c_ulong, lw: *mut b::load_weight) -> u64 {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut fact = b::rust_fair_scale_load_down(weight) as u64;
        let mut shift: c_int = 32;
        __update_inv_weight(lw);
        let hi = (fact >> 32) as u32;
        if hi != 0 {
            let fs = 32 - hi.leading_zeros();
            shift -= fs as c_int;
            fact >>= fs;
        }
        fact = (fact as u32 as u64).wrapping_mul((*lw).inv_weight as u64);
        let hi = (fact >> 32) as u32;
        if hi != 0 {
            let fs = 32 - hi.leading_zeros();
            shift -= fs as c_int;
            fact >>= fs;
        }
        b::rust_fair_mul_u64_u32_shr(delta_exec, fact as u32, shift as c_uint)
    }
}
#[cfg(CONFIG_64BIT)]
unsafe fn __calc_delta(delta_exec: u64, weight: c_ulong, lw: *mut b::load_weight) -> u64 {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        delta_exec.wrapping_mul(weight as u64) / (*lw).weight as u64
    }
}
#[inline]
unsafe fn calc_delta_fair(mut delta: u64, se: *mut b::sched_entity) -> u64 {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if (*se).h_load.weight != b::RUST_FAIR_NICE_0_LOAD as c_ulong {
            delta = __calc_delta(delta, b::RUST_FAIR_NICE_0_LOAD as c_ulong, addr_of_mut!((*se).h_load));
        }
        delta
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
unsafe fn list_add_leaf_cfs_rq(cfs: *mut b::cfs_rq) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let rq = b::rust_fair_rq_of(cfs);
        let cpu = b::rust_fair_cpu_of(rq);
        if (*cfs).on_list != 0 {
            return (*rq).tmp_alone_branch == addr_of_mut!((*rq).leaf_cfs_rq_list);
        }
        (*cfs).on_list = 1;
        let parent = (*(*cfs).tg).parent;
        if !parent.is_null() && (*b::rust_fair_tg_cfs_rq(parent, cpu)).on_list != 0 {
            b::rust_fair_list_add_tail_rcu(
                addr_of_mut!((*cfs).leaf_cfs_rq_list),
                addr_of_mut!((*b::rust_fair_tg_cfs_rq(parent, cpu)).leaf_cfs_rq_list),
            );
            (*rq).tmp_alone_branch = addr_of_mut!((*rq).leaf_cfs_rq_list);
            return true;
        }
        if parent.is_null() {
            b::rust_fair_list_add_tail_rcu(
                addr_of_mut!((*cfs).leaf_cfs_rq_list),
                addr_of_mut!((*rq).leaf_cfs_rq_list),
            );
            (*rq).tmp_alone_branch = addr_of_mut!((*rq).leaf_cfs_rq_list);
            return true;
        }
        b::rust_fair_list_add_rcu(
            addr_of_mut!((*cfs).leaf_cfs_rq_list),
            (*rq).tmp_alone_branch,
        );
        (*rq).tmp_alone_branch = addr_of_mut!((*cfs).leaf_cfs_rq_list);
        false
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
unsafe fn list_del_leaf_cfs_rq(cfs: *mut b::cfs_rq) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if (*cfs).on_list != 0 {
            let rq = b::rust_fair_rq_of(cfs);
            if (*rq).tmp_alone_branch == addr_of_mut!((*cfs).leaf_cfs_rq_list) {
                (*rq).tmp_alone_branch = (*cfs).leaf_cfs_rq_list.prev;
            }
            b::rust_fair_list_del_rcu(addr_of_mut!((*cfs).leaf_cfs_rq_list));
            (*cfs).on_list = 0;
        }
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
unsafe fn assert_list_leaf_cfs_rq(rq: *mut b::rq) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_warn_assert_list_leaf_cfs_rq_1(
            (*rq).tmp_alone_branch != addr_of_mut!((*rq).leaf_cfs_rq_list),
        );
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
unsafe fn is_same_group(se: *mut b::sched_entity, pse: *mut b::sched_entity) -> *mut b::cfs_rq {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if (*se).cfs_rq == (*pse).cfs_rq {
            (*se).cfs_rq
        } else {
            null_mut()
        }
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
unsafe fn parent_entity(se: *mut b::sched_entity) -> *mut b::sched_entity {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (*se).parent
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
unsafe fn tg_is_idle(tg: *mut b::task_group) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        ((*tg).idle > 0) as c_int
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
unsafe fn cfs_rq_is_idle(cfs: *mut b::cfs_rq) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        ((*cfs).idle > 0) as c_int
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
unsafe fn se_is_idle(se: *mut b::sched_entity) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if b::rust_fair_entity_is_task(se) {
            b::rust_fair_task_has_idle_policy(b::rust_fair_task_of(se)) as c_int
        } else {
            cfs_rq_is_idle(b::rust_fair_group_cfs_rq(se))
        }
    }
}
// These empty bodies are the exact original !CONFIG_FAIR_GROUP_SCHED branch.
#[cfg(not(CONFIG_FAIR_GROUP_SCHED))]
unsafe fn list_add_leaf_cfs_rq(_: *mut b::cfs_rq) -> bool {
    true
}
#[cfg(not(CONFIG_FAIR_GROUP_SCHED))]
unsafe fn list_del_leaf_cfs_rq(_: *mut b::cfs_rq) {}
#[cfg(not(CONFIG_FAIR_GROUP_SCHED))]
unsafe fn assert_list_leaf_cfs_rq(_: *mut b::rq) {}
#[cfg(not(CONFIG_FAIR_GROUP_SCHED))]
unsafe fn parent_entity(_: *mut b::sched_entity) -> *mut b::sched_entity {
    null_mut()
}
#[cfg(not(CONFIG_FAIR_GROUP_SCHED))]
unsafe fn tg_is_idle(_: *mut b::task_group) -> c_int {
    0
}
#[cfg(not(CONFIG_FAIR_GROUP_SCHED))]
unsafe fn cfs_rq_is_idle(_: *mut b::cfs_rq) -> c_int {
    0
}
#[cfg(not(CONFIG_FAIR_GROUP_SCHED))]
unsafe fn se_is_idle(se: *mut b::sched_entity) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_task_has_idle_policy(b::rust_fair_task_of(se)) as c_int
    }
}
// C vruntime_cmp/op are modular u64 subtraction interpreted as signed s64.
#[inline]
fn vruntime_delta(a: u64, b: u64) -> i64 {
    a.wrapping_sub(b) as i64
}
#[inline]
fn min_vruntime(a: u64, b: u64) -> u64 {
    if vruntime_delta(b, a) < 0 {
        b
    } else {
        a
    }
}
#[inline]
fn max_vruntime(a: u64, b: u64) -> u64 {
    if vruntime_delta(b, a) > 0 {
        b
    } else {
        a
    }
}
#[inline]
unsafe fn entity_before(a: *const b::sched_entity, bb: *const b::sched_entity) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        vruntime_delta((*a).deadline, (*bb).deadline) < 0
    }
}
#[inline]
unsafe fn entity_key(cfs: *mut b::cfs_rq, se: *mut b::sched_entity) -> i64 {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        vruntime_delta((*se).vruntime, (*cfs).zero_vruntime)
    }
}
#[inline]
unsafe fn node_2_se(node: *const b::rb_node) -> *mut b::sched_entity {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        node.cast::<u8>()
            .sub(core::mem::offset_of!(b::sched_entity, run_node))
            .cast_mut()
            .cast()
    }
}
#[inline]
unsafe fn avg_vruntime_weight(cfs: *mut b::cfs_rq, mut w: c_ulong) -> c_ulong {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        #[cfg(CONFIG_64BIT)]
        if (*cfs).sum_shift != 0 {
            w = max(2, w >> (*cfs).sum_shift);
        }
        w
    }
}
unsafe fn __sum_w_vruntime_add(cfs: *mut b::cfs_rq, se: *mut b::sched_entity) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let weight = avg_vruntime_weight(cfs, (*se).h_load.weight);
        let product = entity_key(cfs, se).wrapping_mul(weight as i64);
        b::rust_fair_warn___sum_w_vruntime_add_1((product >> 63) != (product >> 62));
        (*cfs).sum_w_vruntime = (*cfs).sum_w_vruntime.wrapping_add(product);
        (*cfs).sum_weight = (*cfs).sum_weight.wrapping_add(weight as u64);
    }
}
unsafe fn sum_w_vruntime_add_paranoid(cfs: *mut b::cfs_rq, se: *mut b::sched_entity) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        loop {
            let weight = avg_vruntime_weight(cfs, (*se).h_load.weight);
            // check_mul_overflow has an s64 result even though weight is unsigned.
            let wide = (entity_key(cfs, se) as i128) * (weight as i128);
            if wide >= i64::MIN as i128 && wide <= i64::MAX as i128 {
                if let Some(sum) = (*cfs).sum_w_vruntime.checked_add(wide as i64) {
                    (*cfs).sum_w_vruntime = sum;
                    (*cfs).sum_weight = (*cfs).sum_weight.wrapping_add(weight as u64);
                    return;
                }
            }
            b::rust_fair_bug_on((*cfs).sum_shift >= 10);
            (*cfs).sum_shift += 1;
            (*cfs).sum_w_vruntime = 0;
            (*cfs).sum_weight = 0;
            let mut node = (*cfs).tasks_timeline.rb_leftmost;
            while !node.is_null() {
                __sum_w_vruntime_add(cfs, node_2_se(node));
                node = b::rb_next(node);
            }
        }
    }
}
unsafe fn sum_w_vruntime_add(cfs: *mut b::cfs_rq, se: *mut b::sched_entity) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if feat!(rust_fair_feat_PARANOID_AVG) {
            sum_w_vruntime_add_paranoid(cfs, se);
        } else {
            __sum_w_vruntime_add(cfs, se);
        }
    }
}
unsafe fn sum_w_vruntime_sub(cfs: *mut b::cfs_rq, se: *mut b::sched_entity) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let weight = avg_vruntime_weight(cfs, (*se).h_load.weight);
        (*cfs).sum_w_vruntime = (*cfs)
            .sum_w_vruntime
            .wrapping_sub(entity_key(cfs, se).wrapping_mul(weight as i64));
        (*cfs).sum_weight = (*cfs).sum_weight.wrapping_sub(weight as u64);
    }
}
unsafe fn update_zero_vruntime(cfs: *mut b::cfs_rq, delta: i64) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (*cfs).sum_w_vruntime = (*cfs)
            .sum_w_vruntime
            .wrapping_sub(((*cfs).sum_weight as i64).wrapping_mul(delta));
        (*cfs).zero_vruntime = (*cfs).zero_vruntime.wrapping_add(delta as u64);
    }
}
#[no_mangle]
pub unsafe extern "C" fn avg_vruntime(cfs: *mut b::cfs_rq) -> u64 {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut curr = (*cfs).curr;
        let mut weight = (*cfs).sum_weight as c_long;
        let mut delta: i64 = 0;
        if !curr.is_null() && (*curr).on_rq == 0 {
            curr = null_mut();
        }
        if weight != 0 {
            let mut runtime = (*cfs).sum_w_vruntime;
            if !curr.is_null() {
                let w = avg_vruntime_weight(cfs, (*curr).h_load.weight);
                runtime = runtime.wrapping_add(entity_key(cfs, curr).wrapping_mul(w as i64));
                weight = weight.wrapping_add(w as c_long);
            }
            if runtime < 0 {
                runtime = runtime.wrapping_sub(weight.wrapping_sub(1) as i64);
            }
            delta = b::rust_fair_div64_long(runtime, weight);
        } else if !curr.is_null() {
            delta = vruntime_delta((*curr).vruntime, (*cfs).zero_vruntime);
        }
        update_zero_vruntime(cfs, delta);
        (*cfs).zero_vruntime
    }
}
unsafe fn ineligible_vruntime(cfs: *mut b::cfs_rq) -> u64 {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut curr = (*cfs).curr;
        let weight = (*cfs).sum_weight as c_long;
        let mut delta = 0;
        if !curr.is_null() && (*curr).on_rq == 0 {
            curr = null_mut();
        }
        b::rust_fair_warn_ineligible_vruntime_1(curr.is_null());
        if weight != 0 {
            let mut runtime = (*cfs).sum_w_vruntime;
            if runtime < 0 {
                runtime = runtime.wrapping_sub(weight.wrapping_sub(1) as i64);
            }
            delta = b::rust_fair_div64_long(runtime, weight);
        }
        (*cfs)
            .zero_vruntime
            .wrapping_add(delta as u64)
            .wrapping_add(1)
    }
}
unsafe fn entity_lag(cfs: *mut b::cfs_rq, se: *mut b::sched_entity, avruntime: u64) -> i64 {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let max_slice = cfs_rq_max_slice(cfs).wrapping_add(b::RUST_FAIR_TICK_NSEC as u64);
        let limit = calc_delta_fair(max_slice, se) as i64;
        {
            let value = vruntime_delta(avruntime, (*se).vruntime);
            let lower = limit.wrapping_neg();
            if value >= limit {
                limit
            } else if value <= lower {
                lower
            } else {
                value
            }
        }
    }
}
unsafe fn update_entity_lag(cfs: *mut b::cfs_rq, se: *mut b::sched_entity) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let avg = avg_vruntime(cfs);
        let mut vlag = entity_lag(cfs, se, avg);
        if (*se).sched_delayed != 0 {
            vlag = max(vlag, (*se).vlag);
            if feat!(rust_fair_feat_DELAY_ZERO) {
                vlag = min(vlag, 0);
            }
        }
        (*se).vlag = vlag;
        avg.wrapping_sub(vlag as u64) != (*se).vruntime
    }
}
unsafe fn vruntime_eligible(cfs: *mut b::cfs_rq, vruntime: u64) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let curr = (*cfs).curr;
        let mut avg = (*cfs).sum_w_vruntime;
        let mut load = (*cfs).sum_weight as c_long;
        if !curr.is_null() && (*curr).on_rq != 0 {
            let w = avg_vruntime_weight(cfs, (*curr).h_load.weight);
            avg = avg.wrapping_add(entity_key(cfs, curr).wrapping_mul(w as i64));
            load = load.wrapping_add(w as c_long);
        }
        let key = vruntime_delta(vruntime, (*cfs).zero_vruntime);
        #[cfg(all(CONFIG_64BIT, CONFIG_ARCH_SUPPORTS_INT128))]
        {
            return ((avg as i128) >= (key as i128) * (load as i128)) as c_int;
        }
        #[cfg(all(CONFIG_64BIT, not(CONFIG_ARCH_SUPPORTS_INT128)))]
        {
            return match key.checked_mul(load as i64) {
                Some(rhs) => (avg >= rhs) as c_int,
                None => (key <= 0) as c_int,
            };
        }
        #[cfg(not(CONFIG_64BIT))]
        {
            (avg >= key.wrapping_mul(load as i64)) as c_int
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn entity_eligible(cfs: *mut b::cfs_rq, se: *mut b::sched_entity) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        vruntime_eligible(cfs, (*se).vruntime)
    }
}
unsafe fn cfs_rq_min_slice(cfs: *mut b::cfs_rq) -> u64 {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let root = __pick_root_entity(cfs);
        let curr = (*cfs).curr;
        let mut slice = u64::MAX;
        if !curr.is_null() && (*curr).on_rq != 0 {
            slice = (*curr).slice;
        }
        if !root.is_null() {
            slice = min(slice, (*root).min_slice);
        }
        slice
    }
}
unsafe fn cfs_rq_max_slice(cfs: *mut b::cfs_rq) -> u64 {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let root = __pick_root_entity(cfs);
        let curr = (*cfs).curr;
        let mut slice = 0;
        if !curr.is_null() && (*curr).on_rq != 0 {
            slice = (*curr).slice;
        }
        if !root.is_null() {
            slice = max(slice, (*root).max_slice);
        }
        slice
    }
}
unsafe extern "C" fn __entity_less(a: *mut b::rb_node, bb: *const b::rb_node) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        entity_before(node_2_se(a), node_2_se(bb))
    }
}
unsafe fn __min_vruntime_update(se: *mut b::sched_entity, node: *mut b::rb_node) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if !node.is_null() {
            (*se).min_vruntime = min_vruntime((*se).min_vruntime, (*node_2_se(node)).min_vruntime);
        }
    }
}
unsafe fn __min_slice_update(se: *mut b::sched_entity, node: *mut b::rb_node) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if !node.is_null() {
            (*se).min_slice = min((*se).min_slice, (*node_2_se(node)).min_slice);
        }
    }
}
unsafe fn __max_slice_update(se: *mut b::sched_entity, node: *mut b::rb_node) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if !node.is_null() {
            (*se).max_slice = max((*se).max_slice, (*node_2_se(node)).max_slice);
        }
    }
}
unsafe fn min_vruntime_update(se: *mut b::sched_entity, _exit: bool) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let old = ((*se).min_vruntime, (*se).min_slice, (*se).max_slice);
        (*se).min_vruntime = (*se).vruntime;
        (*se).min_slice = (*se).slice;
        (*se).max_slice = (*se).slice;
        for node in [(*se).run_node.rb_right, (*se).run_node.rb_left] {
            __min_vruntime_update(se, node);
            __min_slice_update(se, node);
            __max_slice_update(se, node);
        }
        old == ((*se).min_vruntime, (*se).min_slice, (*se).max_slice)
    }
}
// Direct expansion of RB_DECLARE_CALLBACKS: metadata aggregation stays in Rust.
unsafe extern "C" fn min_vruntime_propagate(mut node: *mut b::rb_node, stop: *mut b::rb_node) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        while node != stop {
            if min_vruntime_update(node_2_se(node), true) {
                break;
            }
            node = b::rust_fair_rb_parent(node);
        }
    }
}
unsafe extern "C" fn min_vruntime_copy(old: *mut b::rb_node, new: *mut b::rb_node) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (*node_2_se(new)).min_vruntime = (*node_2_se(old)).min_vruntime;
    }
}
unsafe extern "C" fn min_vruntime_rotate(old: *mut b::rb_node, new: *mut b::rb_node) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (*node_2_se(new)).min_vruntime = (*node_2_se(old)).min_vruntime;
        min_vruntime_update(node_2_se(old), false);
    }
}
static min_vruntime_cb: b::rb_augment_callbacks = b::rb_augment_callbacks {
    propagate: Some(min_vruntime_propagate),
    copy: Some(min_vruntime_copy),
    rotate: Some(min_vruntime_rotate),
};
unsafe fn __enqueue_entity(cfs: *mut b::cfs_rq, se: *mut b::sched_entity) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_warn___enqueue_entity_1(addr_of_mut!((*b::rust_fair_rq_of(cfs)).cfs) != cfs);
        b::rust_fair_warn___enqueue_entity_2(!b::rust_fair_entity_is_task(se));
        sum_w_vruntime_add(cfs, se);
        (*se).min_vruntime = (*se).vruntime;
        (*se).min_slice = (*se).slice;
        b::rust_fair_rb_add_augmented_cached(
            addr_of_mut!((*se).run_node),
            addr_of_mut!((*cfs).tasks_timeline),
            Some(__entity_less),
            addr_of!(min_vruntime_cb),
        );
    }
}
unsafe fn __dequeue_entity(cfs: *mut b::cfs_rq, se: *mut b::sched_entity) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_warn___dequeue_entity_1(addr_of_mut!((*b::rust_fair_rq_of(cfs)).cfs) != cfs);
        b::rust_fair_warn___dequeue_entity_2(!b::rust_fair_entity_is_task(se));
        b::rust_fair_rb_erase_augmented_cached(
            addr_of_mut!((*se).run_node),
            addr_of_mut!((*cfs).tasks_timeline),
            addr_of!(min_vruntime_cb),
        );
        sum_w_vruntime_sub(cfs, se);
    }
}
#[no_mangle]
pub unsafe extern "C" fn __pick_root_entity(cfs: *mut b::cfs_rq) -> *mut b::sched_entity {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let root = (*cfs).tasks_timeline.rb_root.rb_node;
        if root.is_null() {
            null_mut()
        } else {
            node_2_se(root)
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn __pick_first_entity(cfs: *mut b::cfs_rq) -> *mut b::sched_entity {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let root = (*cfs).tasks_timeline.rb_leftmost;
        if root.is_null() {
            null_mut()
        } else {
            node_2_se(root)
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn __pick_last_entity(cfs: *mut b::cfs_rq) -> *mut b::sched_entity {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let root = b::rb_last(addr_of!((*cfs).tasks_timeline.rb_root));
        if root.is_null() {
            null_mut()
        } else {
            node_2_se(root)
        }
    }
}
unsafe fn set_protect_slice(cfs: *mut b::cfs_rq, se: *mut b::sched_entity) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut slice = normalized_sysctl_sched_base_slice as u64;
        let mut vprot = (*se).deadline;
        if feat!(rust_fair_feat_RUN_TO_PARITY) {
            slice = cfs_rq_min_slice(cfs);
        }
        slice = min(slice, (*se).slice);
        if slice != (*se).slice {
            vprot = if feat!(rust_fair_feat_PREEMPT_SHORT) {
                min_vruntime(vprot, ineligible_vruntime(cfs))
            } else {
                min_vruntime(
                    vprot,
                    (*se).vruntime.wrapping_add(calc_delta_fair(slice, se)),
                )
            };
        }
        (*se).vprot = vprot;
    }
}
unsafe fn update_protect_slice(cfs: *mut b::cfs_rq, se: *mut b::sched_entity) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let slice = cfs_rq_min_slice(cfs);
        let runtime = min_vruntime((*se).vruntime, avg_vruntime(cfs));
        (*se).vprot = min_vruntime(
            (*se).vprot,
            runtime.wrapping_add(calc_delta_fair(slice, se)),
        );
    }
}
unsafe fn protect_slice(se: *mut b::sched_entity) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        vruntime_delta((*se).vruntime, (*se).vprot) < 0
    }
}
unsafe fn cancel_protect_slice(se: *mut b::sched_entity) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if protect_slice(se) {
            (*se).vprot = (*se).vruntime;
        }
    }
}
unsafe fn pick_eevdf(cfs: *mut b::cfs_rq, protect: bool) -> *mut b::sched_entity {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut node = (*cfs).tasks_timeline.rb_root.rb_node;
        let mut se = __pick_first_entity(cfs);
        let mut curr = (*cfs).curr;
        let mut best = null_mut();
        if (*cfs).h_nr_queued == 1 {
            return if !curr.is_null() && (*curr).on_rq != 0 {
                curr
            } else {
                se
            };
        }
        if feat!(rust_fair_feat_PICK_BUDDY)
            && protect
            && !(*cfs).next.is_null()
            && entity_eligible(cfs, (*cfs).next) != 0
        {
            b::rust_fair_warn_pick_eevdf_1((*(*cfs).next).sched_delayed != 0);
            return (*cfs).next;
        }
        if !curr.is_null() && ((*curr).on_rq == 0 || entity_eligible(cfs, curr) == 0) {
            curr = null_mut();
        }
        if !curr.is_null() && protect && protect_slice(curr) {
            return curr;
        }
        if !se.is_null() && entity_eligible(cfs, se) != 0 {
            best = se;
        } else {
            while !node.is_null() {
                let left = (*node).rb_left;
                if !left.is_null() && vruntime_eligible(cfs, (*node_2_se(left)).min_vruntime) != 0 {
                    node = left;
                    continue;
                }
                se = node_2_se(node);
                if entity_eligible(cfs, se) != 0 {
                    best = se;
                    break;
                }
                node = (*node).rb_right;
            }
        }
        if best.is_null() || (!curr.is_null() && entity_before(curr, best)) {
            best = curr;
        }
        best
    }
}
unsafe fn update_deadline(cfs: *mut b::cfs_rq, se: *mut b::sched_entity) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if vruntime_delta((*se).vruntime, (*se).deadline) < 0 {
            return false;
        }
        if (*se).custom_slice == 0 {
            (*se).slice = sysctl_sched_base_slice as u64;
        }
        (*se).deadline = (*se)
            .vruntime
            .wrapping_add(calc_delta_fair((*se).slice, se));
        avg_vruntime(cfs);
        true
    }
}
#[no_mangle]
pub unsafe extern "C" fn init_entity_runnable_average(se: *mut b::sched_entity) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        core::ptr::write_bytes(addr_of_mut!((*se).avg), 0, 1);
        if b::rust_fair_entity_is_task(se) {
            (*se).avg.load_avg = b::rust_fair_scale_load_down((*se).load.weight);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn post_init_entity_util_avg(p: *mut b::task_struct) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let se = addr_of_mut!((*p).se);
        let cfs = b::rust_fair_cfs_rq_of(se);
        let sa = addr_of_mut!((*se).avg);
        let cpu_scale =
            b::rust_fair_arch_scale_cpu_capacity(b::rust_fair_cpu_of(b::rust_fair_rq_of(cfs)))
                as c_long;
        let cap = cpu_scale.wrapping_sub((*cfs).avg.util_avg as c_long) / 2;
        if (*p).sched_class != addr_of!(fair_sched_class) {
            (*se).avg.last_update_time = b::rust_fair_cfs_rq_clock_pelt(cfs);
            return;
        }
        if cap > 0 {
            if (*cfs).avg.util_avg != 0 {
                (*sa).util_avg = (*cfs)
                    .avg
                    .util_avg
                    .wrapping_mul(b::rust_fair_se_weight(se) as c_ulong);
                (*sa).util_avg /= (*cfs).avg.load_avg.wrapping_add(1);
                if (*sa).util_avg > cap as c_ulong {
                    (*sa).util_avg = cap as c_ulong;
                }
            } else {
                (*sa).util_avg = cap as c_ulong;
            }
        }
        (*sa).runnable_avg = (*sa).util_avg;
    }
}

unsafe fn update_se(rq: *mut b::rq, se: *mut b::sched_entity) -> i64 {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let now = b::rust_fair_rq_clock_task(rq);
        let delta = now.wrapping_sub((*se).exec_start) as i64;
        if delta <= 0 {
            return delta;
        }
        (*se).exec_start = now;
        if b::rust_fair_entity_is_task(se) {
            let donor = b::rust_fair_task_of(se);
            let running = b::rust_fair_rq_curr(rq);
            (*running).se.exec_start = now;
            (*running).se.sum_exec_runtime = (*running).se.sum_exec_runtime.wrapping_add(delta as u64);
            b::rust_fair_trace_sched_stat_runtime(running, delta as u64);
            b::rust_fair_account_group_exec_runtime(running, delta as u64);
            account_mm_sched(rq, running, delta);
            b::rust_fair_cgroup_account_cputime(donor, delta as u64);
        } else {
            (*se).sum_exec_runtime = (*se).sum_exec_runtime.wrapping_add(delta as u64);
        }
        #[cfg(CONFIG_SCHEDSTATS)]
        if b::rust_fair_schedstat_enabled() {
            let stats = b::rust_fair_schedstats_from_se(se);
            (*stats).exec_max = max(delta, (*stats).exec_max);
        }
        delta
    }
}
#[no_mangle]
pub unsafe extern "C" fn update_curr_common(rq: *mut b::rq) -> i64 {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        update_se(rq, addr_of_mut!((*b::rust_fair_rq_donor(rq)).se))
    }
}
unsafe fn update_curr(mut cfs: *mut b::cfs_rq) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let curr = (*cfs).h_curr;
        let rq = b::rust_fair_rq_of(cfs);
        if curr.is_null() {
            return;
        }
        let delta = update_se(rq, curr);
        if delta <= 0 {
            return;
        }
        account_cfs_rq_runtime(cfs, delta as u64);
        if !b::rust_fair_entity_is_task(curr) {
            return;
        }
        cfs = addr_of_mut!((*rq).cfs);
        (*curr).vruntime = (*curr)
            .vruntime
            .wrapping_add(calc_delta_fair(delta as u64, curr));
        let resched = update_deadline(cfs, curr);
        b::dl_server_update(addr_of_mut!((*rq).fair_server), delta);
        if (*cfs).h_nr_queued == 1 {
            return;
        }
        if resched || !protect_slice(curr) {
            b::resched_curr_lazy(rq);
            clear_buddies(cfs, curr);
        }
    }
}
#[export_name = "rust_fair_update_curr_fair"]
unsafe extern "C" fn update_curr_fair(rq: *mut b::rq) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut se = addr_of_mut!((*b::rust_fair_rq_donor(rq)).se);
        while !se.is_null() {
            update_curr(b::rust_fair_cfs_rq_of(se));
            se = parent_entity(se);
        }
    }
}
unsafe fn update_stats_wait_start_fair(cfs: *mut b::cfs_rq, se: *mut b::sched_entity) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if !b::rust_fair_schedstat_enabled() {
            return;
        }
        let stats = b::rust_fair_schedstats_from_se(se);
        let p = if b::rust_fair_entity_is_task(se) {
            b::rust_fair_task_of(se)
        } else {
            null_mut()
        };
        b::rust_fair_update_stats_wait_start(b::rust_fair_rq_of(cfs), p, stats);
    }
}
unsafe fn update_stats_wait_end_fair(cfs: *mut b::cfs_rq, se: *mut b::sched_entity) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if !b::rust_fair_schedstat_enabled() {
            return;
        }
        let stats = b::rust_fair_schedstats_from_se(se);
        if b::rust_fair_schedstat_wait_start(stats) == 0 {
            return;
        }
        let p = if b::rust_fair_entity_is_task(se) {
            b::rust_fair_task_of(se)
        } else {
            null_mut()
        };
        b::rust_fair_update_stats_wait_end(b::rust_fair_rq_of(cfs), p, stats);
    }
}
unsafe fn update_stats_enqueue_sleeper_fair(cfs: *mut b::cfs_rq, se: *mut b::sched_entity) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if !b::rust_fair_schedstat_enabled() {
            return;
        }
        let stats = b::rust_fair_schedstats_from_se(se);
        let p = if b::rust_fair_entity_is_task(se) {
            b::rust_fair_task_of(se)
        } else {
            null_mut()
        };
        b::rust_fair_update_stats_enqueue_sleeper(b::rust_fair_rq_of(cfs), p, stats);
    }
}
unsafe fn update_stats_enqueue_fair(cfs: *mut b::cfs_rq, se: *mut b::sched_entity, flags: c_int) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if !b::rust_fair_schedstat_enabled() {
            return;
        }
        if se != (*cfs).h_curr {
            update_stats_wait_start_fair(cfs, se);
        }
        if flags & b::ENQUEUE_WAKEUP as c_int != 0 {
            update_stats_enqueue_sleeper_fair(cfs, se);
        }
    }
}
unsafe fn update_stats_dequeue_fair(cfs: *mut b::cfs_rq, se: *mut b::sched_entity, flags: c_int) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if !b::rust_fair_schedstat_enabled() {
            return;
        }
        if se != (*cfs).h_curr {
            update_stats_wait_end_fair(cfs, se);
        }
        #[cfg(CONFIG_SCHEDSTATS)]
        if flags & b::DEQUEUE_SLEEP as c_int != 0 && b::rust_fair_entity_is_task(se) {
            let task = b::rust_fair_task_of(se);
            let state = read_once!((*task).__state);
            if state & b::TASK_INTERRUPTIBLE != 0 {
                (*task).stats.sleep_start = b::rust_fair_rq_clock(b::rust_fair_rq_of(cfs));
            }
            if state & b::TASK_UNINTERRUPTIBLE != 0 {
                (*task).stats.block_start = b::rust_fair_rq_clock(b::rust_fair_rq_of(cfs));
            }
        }
    }
}
unsafe fn update_stats_curr_start(cfs: *mut b::cfs_rq, se: *mut b::sched_entity) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (*se).exec_start = b::rust_fair_rq_clock_task(b::rust_fair_rq_of(cfs));
    }
}
unsafe fn is_core_idle(cpu: c_int) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mask = b::rust_fair_cpu_smt_mask(cpu);
        let mut sibling = b::rust_fair_cpumask_first(mask);
        while sibling < b::rust_fair_small_cpumask_bits() {
            if cpu != sibling as c_int && b::idle_cpu(sibling as c_int) == 0 {
                return false;
            }
            sibling = b::rust_fair_cpumask_next(sibling as c_int, mask);
        }
        true
    }
}
#[cfg(CONFIG_NUMA)]
unsafe fn adjust_numa_imbalance(
    imbalance: c_int,
    dst_running: c_int,
    imb_numa_nr: c_int,
) -> c_long {
    if dst_running > imb_numa_nr {
        return imbalance as c_long;
    }
    if imbalance <= b::RUST_FAIR_NUMA_IMBALANCE_MIN as c_int {
        0
    } else {
        imbalance as c_long
    }
}
unsafe fn account_entity_enqueue(cfs: *mut b::cfs_rq, se: *mut b::sched_entity) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_warn_account_entity_enqueue_1(cfs != b::rust_fair_cfs_rq_of(se));
        update_load_add(addr_of_mut!((*cfs).load), (*se).load.weight);
        if b::rust_fair_entity_is_task(se) {
            let p = b::rust_fair_task_of(se);
            let rq = b::rust_fair_rq_of(cfs);
            account_numa_enqueue(rq, p);
            account_llc_enqueue(rq, p);
            b::rust_fair_list_add(
                addr_of_mut!((*se).group_node),
                addr_of_mut!((*rq).cfs_tasks),
            );
        }
        (*cfs).nr_queued = (*cfs).nr_queued.wrapping_add(1);
    }
}
unsafe fn account_entity_dequeue(cfs: *mut b::cfs_rq, se: *mut b::sched_entity) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_warn_account_entity_dequeue_1(cfs != b::rust_fair_cfs_rq_of(se));
        update_load_sub(addr_of_mut!((*cfs).load), (*se).load.weight);
        if b::rust_fair_entity_is_task(se) {
            let p = b::rust_fair_task_of(se);
            let rq = b::rust_fair_rq_of(cfs);
            account_numa_dequeue(rq, p);
            account_llc_dequeue(rq, p);
            b::rust_fair_list_del_init(addr_of_mut!((*se).group_node));
        }
        (*cfs).nr_queued = (*cfs).nr_queued.wrapping_sub(1);
    }
}
// Same-width signed add with a single READ_ONCE/WRITE_ONCE and saturation only
// on negative underflow. Positive wrapping is deliberately not saturated.
macro_rules! add_positive {
    ($field:expr, $delta:expr, $signed:ty) => {{
        let value = read_once!($field);
        let delta = $delta as $signed;
        let mut result = value.wrapping_add(delta as _);
        if delta < 0 && result > value {
            result = 0;
        }
        write_once!($field, result);
    }};
}
macro_rules! update_sa_load {
    ($sa:expr, $avg:expr, $sum:expr) => {{
        let sa = $sa;
        add_positive!((*sa).load_avg, $avg, c_long);
        add_positive!((*sa).load_sum, $sum, i64);
        (*sa).load_sum = max(
            (*sa).load_sum,
            (*sa)
                .load_avg
                .wrapping_mul(b::RUST_FAIR_PELT_MIN_DIVIDER as c_ulong) as u64,
        );
    }};
}
macro_rules! update_sa_util {
    ($sa:expr, $avg:expr, $sum:expr) => {{
        let sa = $sa;
        add_positive!((*sa).util_avg, $avg, c_long);
        add_positive!((*sa).util_sum, $sum, i32);
        (*sa).util_sum = max(
            (*sa).util_sum,
            (*sa)
                .util_avg
                .wrapping_mul(b::RUST_FAIR_PELT_MIN_DIVIDER as c_ulong) as u32,
        );
    }};
}
macro_rules! update_sa_runnable {
    ($sa:expr, $avg:expr, $sum:expr) => {{
        let sa = $sa;
        add_positive!((*sa).runnable_avg, $avg, c_long);
        add_positive!((*sa).runnable_sum, $sum, i64);
        (*sa).runnable_sum = max(
            (*sa).runnable_sum,
            (*sa)
                .runnable_avg
                .wrapping_mul(b::RUST_FAIR_PELT_MIN_DIVIDER as c_ulong) as u64,
        );
    }};
}
unsafe fn enqueue_load_avg(cfs: *mut b::cfs_rq, se: *mut b::sched_entity) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        update_sa_load!(
            addr_of_mut!((*cfs).avg),
            (*se).avg.load_avg,
            (b::rust_fair_se_weight(se) as u64).wrapping_mul((*se).avg.load_sum)
        );
    }
}
unsafe fn dequeue_load_avg(cfs: *mut b::cfs_rq, se: *mut b::sched_entity) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        update_sa_load!(
            addr_of_mut!((*cfs).avg),
            (*se).avg.load_avg.wrapping_neg(),
            (b::rust_fair_se_weight(se) as u64).wrapping_mul((*se).avg.load_sum.wrapping_neg())
        );
    }
}
unsafe fn rescale_entity(se: *mut b::sched_entity, weight: c_ulong, rel_vprot: bool) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let old = (*se).h_load.weight as c_long;
        (*se).vlag = b::rust_fair_div64_long((*se).vlag.wrapping_mul(old as i64), weight as c_long);
        if (*se).rel_deadline != 0 {
            (*se).deadline = b::rust_fair_div64_long(
                (*se).deadline.wrapping_mul(old as u64) as i64,
                weight as c_long,
            ) as u64;
        }
        if rel_vprot {
            (*se).vprot = b::rust_fair_div64_long(
                (*se).vprot.wrapping_mul(old as u64) as i64,
                weight as c_long,
            ) as u64;
        }
    }
}
unsafe fn reweight_eevdf(
    cfs: *mut b::cfs_rq,
    se: *mut b::sched_entity,
    weight: c_ulong,
    on_rq: bool,
) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let curr = (*cfs).curr == se;
        let mut rel = false;
        let mut avg = 0;
        if (*se).h_load.weight == weight {
            return;
        }
        if on_rq {
            avg = avg_vruntime(cfs);
            (*se).vlag = entity_lag(cfs, se, avg);
            (*se).deadline = (*se).deadline.wrapping_sub(avg);
            (*se).rel_deadline = 1;
            if curr && protect_slice(se) {
                (*se).vprot = (*se).vprot.wrapping_sub(avg);
                rel = true;
            }
            (*cfs).h_nr_queued = (*cfs).h_nr_queued.wrapping_sub(1);
            if !curr {
                __dequeue_entity(cfs, se);
            }
        }
        rescale_entity(se, weight, rel);
        update_load_set(addr_of_mut!((*se).h_load), weight);
        if on_rq {
            if rel {
                (*se).vprot = (*se).vprot.wrapping_add(avg);
            }
            (*se).deadline = (*se).deadline.wrapping_add(avg);
            (*se).rel_deadline = 0;
            (*se).vruntime = avg.wrapping_sub((*se).vlag as u64);
            if !curr {
                __enqueue_entity(cfs, se);
            }
            (*cfs).h_nr_queued = (*cfs).h_nr_queued.wrapping_add(1);
        }
    }
}
unsafe fn reweight_entity(cfs: *mut b::cfs_rq, se: *mut b::sched_entity, weight: c_ulong) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if (*se).load.weight == weight {
            return;
        }
        if (*se).on_rq != 0 {
            b::rust_fair_warn_reweight_entity_1(cfs != b::rust_fair_cfs_rq_of(se));
            update_load_sub(addr_of_mut!((*cfs).load), (*se).load.weight);
        }
        dequeue_load_avg(cfs, se);
        update_load_set(addr_of_mut!((*se).load), weight);
        let divider = b::rust_fair_get_pelt_divider(addr_of!((*se).avg));
        (*se).avg.load_avg = ((b::rust_fair_se_weight(se) as u64).wrapping_mul((*se).avg.load_sum)
            / divider as u64) as c_ulong;
        enqueue_load_avg(cfs, se);
        if (*se).on_rq != 0 {
            update_load_add(addr_of_mut!((*cfs).load), (*se).load.weight);
        }
    }
}
unsafe fn __calc_prop_weight(
    cfs: *mut b::cfs_rq,
    se: *mut b::sched_entity,
    mut weight: c_ulong,
) -> c_ulong {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        weight = weight.wrapping_mul((*se).load.weight);
        weight /= if !parent_entity(se).is_null() {
            (*cfs).load.weight
        } else {
            b::RUST_FAIR_NICE_0_LOAD as c_ulong
        };
        max(weight, b::MIN_SHARES as c_ulong)
    }
}
#[export_name = "rust_fair_reweight_task_fair"]
unsafe extern "C" fn reweight_task_fair(
    rq: *mut b::rq,
    p: *mut b::task_struct,
    lw: *const b::load_weight,
) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut se = addr_of_mut!((*p).se);
        let mut weight = b::RUST_FAIR_NICE_0_LOAD as c_ulong;
        if (*se).on_rq != 0 {
            update_curr_fair(rq);
        }
        reweight_entity(b::rust_fair_cfs_rq_of(se), se, (*lw).weight);
        (*se).load.inv_weight = (*lw).inv_weight;
        if (*se).on_rq == 0 {
            return;
        }
        while !se.is_null() {
            weight = __calc_prop_weight(b::rust_fair_cfs_rq_of(se), se, weight);
            se = parent_entity(se);
        }
        reweight_eevdf(
            addr_of_mut!((*rq).cfs),
            addr_of_mut!((*p).se),
            weight,
            (*p).se.on_rq != 0,
        );
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
unsafe fn __calc_smp_shares(cfs: *mut b::cfs_rq, tg_shares: c_long, shares_max: c_long) -> c_long {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let tg = (*cfs).tg;
        let load = max(
            b::rust_fair_scale_load_down((*cfs).load.weight),
            (*cfs).avg.load_avg,
        ) as c_long;
        let mut tg_weight = b::rust_fair_atomic_long_read(addr_of!((*tg).load_avg));
        tg_weight = tg_weight
            .wrapping_sub((*cfs).tg_load_avg_contrib as c_long)
            .wrapping_add(load);
        let mut shares = tg_shares.wrapping_mul(load);
        if tg_weight != 0 {
            shares /= tg_weight;
        }
        // Linux clamp_t permits a reversed range, unlike Rust clamp().
        if shares >= shares_max {
            shares_max
        } else if shares <= b::MIN_SHARES as c_long {
            b::MIN_SHARES as c_long
        } else {
            shares
        }
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
unsafe fn tg_cpus(tg: *mut b::task_group) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut nr = b::rust_fair_num_online_cpus() as c_int;
        if b::rust_fair_cpusets_enabled() {
            let cgrp = (*tg).css.cgroup;
            if !cgrp.is_null() {
                nr = b::rust_fair_cpuset_num_cpus(cgrp);
            }
        }
        max(nr, 1)
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
unsafe fn tg_tasks(tg: *mut b::task_group) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        max(
            1,
            b::rust_fair_atomic_long_read(addr_of!((*tg).runnable_avg)) >> b::SCHED_CAPACITY_SHIFT,
        ) as c_int
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn calc_tasks_shares(cfs: *mut b::cfs_rq) -> c_long {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let tg = (*cfs).tg;
        let nr = tg_tasks(tg) as c_long;
        let shares = read_once!((*tg).shares) as c_long;
        __calc_smp_shares(cfs, nr.wrapping_mul(shares), nr.wrapping_mul(shares))
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn calc_max_shares(cfs: *mut b::cfs_rq) -> c_long {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let tg = (*cfs).tg;
        let nr = tg_cpus(tg) as c_long;
        let shares = read_once!((*tg).shares) as c_long;
        let cap = b::rust_fair_scale_load(b::sched_prio_to_weight[0] as c_ulong) as c_long;
        __calc_smp_shares(cfs, shares.wrapping_mul(nr), cap)
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn calc_concur_shares(cfs: *mut b::cfs_rq) -> c_long {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let tg = (*cfs).tg;
        let nr = min(tg_tasks(tg), tg_cpus(tg)) as c_long;
        let shares = read_once!((*tg).shares) as c_long;
        __calc_smp_shares(cfs, nr.wrapping_mul(shares), nr.wrapping_mul(shares))
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn calc_smp_shares(cfs: *mut b::cfs_rq) -> c_long {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let shares = read_once!((*(*cfs).tg).shares) as c_long;
        __calc_smp_shares(cfs, shares, shares)
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn calc_up_shares(cfs: *mut b::cfs_rq) -> c_long {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        read_once!((*(*cfs).tg).shares) as c_long
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn __sched_cgroup_mode_update(mode: c_int) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let function: unsafe extern "C" fn(*mut b::cfs_rq) -> c_long = match mode {
            0 => calc_up_shares,
            1 => calc_smp_shares,
            3 => calc_max_shares,
            4 => calc_tasks_shares,
            _ => calc_concur_shares,
        };
        b::rust_fair_static_call_update_group_shares(Some(function));
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
unsafe fn update_cfs_group(se: *mut b::sched_entity) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let gcfs = b::rust_fair_group_cfs_rq(se);
        if gcfs.is_null() || (*gcfs).load.weight == 0 {
            return;
        }
        let shares = b::rust_fair_static_call_group_shares(gcfs);
        reweight_entity(b::rust_fair_cfs_rq_of(se), se, shares as c_ulong);
    }
}
#[cfg(not(CONFIG_FAIR_GROUP_SCHED))]
unsafe fn update_cfs_group(_: *mut b::sched_entity) {}
unsafe fn cfs_rq_util_change(cfs: *mut b::cfs_rq, flags: c_int) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let rq = b::rust_fair_rq_of(cfs);
        if addr_of_mut!((*rq).cfs) == cfs {
            b::rust_fair_cpufreq_update_util(rq, flags as c_uint);
        }
    }
}
unsafe fn load_avg_is_decayed(sa: *mut b::sched_avg) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if (*sa).load_sum != 0 || (*sa).util_sum != 0 || (*sa).runnable_sum != 0 {
            return false;
        }
        b::rust_fair_warn_load_avg_is_decayed_1(
            (*sa).load_avg != 0 || (*sa).util_avg != 0 || (*sa).runnable_avg != 0,
        );
        true
    }
}
unsafe fn cfs_rq_last_update_time(cfs: *mut b::cfs_rq) -> u64 {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_cfs_rq_last_update_time_copy(cfs)
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
unsafe fn child_cfs_rq_on_list(cfs: *mut b::cfs_rq) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let rq = b::rust_fair_rq_of(cfs);
        let prev = if (*cfs).on_list != 0 {
            (*cfs).leaf_cfs_rq_list.prev
        } else {
            (*rq).tmp_alone_branch
        };
        if prev == addr_of_mut!((*rq).leaf_cfs_rq_list) {
            return false;
        }
        let prev_cfs = prev
            .cast::<u8>()
            .sub(core::mem::offset_of!(b::cfs_rq, leaf_cfs_rq_list))
            .cast::<b::cfs_rq>();
        (*(*prev_cfs).tg).parent == (*cfs).tg
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
unsafe fn cfs_rq_is_decayed(cfs: *mut b::cfs_rq) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (*cfs).load.weight == 0
            && load_avg_is_decayed(addr_of_mut!((*cfs).avg))
            && !child_cfs_rq_on_list(cfs)
            && (*cfs).tg_load_avg_contrib == 0
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
unsafe fn update_tg_load_avg(cfs: *mut b::cfs_rq) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if (*cfs).tg == addr_of_mut!(b::root_task_group)
            || !b::rust_fair_cpu_active(b::rust_fair_cpu_of(b::rust_fair_rq_of(cfs)))
        {
            return;
        }
        let now = b::rust_fair_rq_clock(b::rust_fair_rq_of(cfs));
        if now.wrapping_sub((*cfs).last_update_tg_load_avg) < b::NSEC_PER_MSEC as u64 {
            return;
        }
        let dl = (*cfs).avg.load_avg.wrapping_sub((*cfs).tg_load_avg_contrib) as c_long;
        let dr = (*cfs)
            .avg
            .runnable_avg
            .wrapping_sub((*cfs).tg_runnable_avg_contrib) as c_long;
        if dl.wrapping_abs() as c_ulong > (*cfs).tg_load_avg_contrib / 64
            || dr.wrapping_abs() as c_ulong > (*cfs).tg_runnable_avg_contrib / 64
        {
            b::rust_fair_atomic_long_add(dl, addr_of_mut!((*(*cfs).tg).load_avg));
            b::rust_fair_atomic_long_add(dr, addr_of_mut!((*(*cfs).tg).runnable_avg));
            (*cfs).tg_load_avg_contrib = (*cfs).avg.load_avg;
            (*cfs).tg_runnable_avg_contrib = (*cfs).avg.runnable_avg;
            (*cfs).last_update_tg_load_avg = now;
        }
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
unsafe fn clear_tg_load_avg(cfs: *mut b::cfs_rq) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if (*cfs).tg == addr_of_mut!(b::root_task_group) {
            return;
        }
        let now = b::rust_fair_rq_clock(b::rust_fair_rq_of(cfs));
        b::rust_fair_atomic_long_add(
            (*cfs).tg_load_avg_contrib.wrapping_neg() as c_long,
            addr_of_mut!((*(*cfs).tg).load_avg),
        );
        b::rust_fair_atomic_long_add(
            (*cfs).tg_runnable_avg_contrib.wrapping_neg() as c_long,
            addr_of_mut!((*(*cfs).tg).runnable_avg),
        );
        (*cfs).tg_load_avg_contrib = 0;
        (*cfs).tg_runnable_avg_contrib = 0;
        (*cfs).last_update_tg_load_avg = now;
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
unsafe fn clear_tg_offline_cfs_rqs(rq: *mut b::rq) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_lockdep_assert_rq_held(rq);
        b::rust_fair_rq_clock_start_loop_update(rq);
        b::rust_fair_rcu_read_lock();
        let head = addr_of_mut!(b::task_groups);
        let mut node = b::rust_fair_list_next_rcu(head);
        while node != head {
            let tg = node
                .cast::<u8>()
                .sub(core::mem::offset_of!(b::task_group, list))
                .cast::<b::task_group>();
            clear_tg_load_avg(b::rust_fair_tg_cfs_rq(tg, b::rust_fair_cpu_of(rq)));
            node = b::rust_fair_list_next_rcu(node);
        }
        b::rust_fair_rq_clock_stop_loop_update(rq);
        b::rust_fair_rcu_read_unlock();
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn set_task_rq_fair(
    se: *mut b::sched_entity,
    prev: *mut b::cfs_rq,
    next: *mut b::cfs_rq,
) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if !feat!(rust_fair_feat_ATTACH_AGE_LOAD) || (*se).avg.last_update_time == 0 || prev.is_null() {
            return;
        }
        let old = cfs_rq_last_update_time(prev);
        let new = cfs_rq_last_update_time(next);
        b::__update_load_avg_blocked_se(old, se);
        (*se).avg.last_update_time = new;
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
unsafe fn update_tg_cfs_util(cfs: *mut b::cfs_rq, se: *mut b::sched_entity, gcfs: *mut b::cfs_rq) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let delta_avg = (*gcfs).avg.util_avg.wrapping_sub((*se).avg.util_avg) as c_long;
        if delta_avg == 0 {
            return;
        }
        let divider = b::rust_fair_get_pelt_divider(addr_of!((*cfs).avg));
        (*se).avg.util_avg = (*gcfs).avg.util_avg;
        let new_sum = (*se).avg.util_avg.wrapping_mul(divider as c_ulong) as u32;
        let delta_sum = (new_sum as c_long).wrapping_sub((*se).avg.util_sum as c_long);
        (*se).avg.util_sum = new_sum;
        update_sa_util!(addr_of_mut!((*cfs).avg), delta_avg, delta_sum);
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
unsafe fn update_tg_cfs_runnable(
    cfs: *mut b::cfs_rq,
    se: *mut b::sched_entity,
    gcfs: *mut b::cfs_rq,
) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let delta_avg = (*gcfs)
            .avg
            .runnable_avg
            .wrapping_sub((*se).avg.runnable_avg) as c_long;
        if delta_avg == 0 {
            return;
        }
        let divider = b::rust_fair_get_pelt_divider(addr_of!((*cfs).avg));
        (*se).avg.runnable_avg = (*gcfs).avg.runnable_avg;
        let new_sum = ((*se).avg.runnable_avg as u64).wrapping_mul(divider as u64);
        let delta_sum = (new_sum as c_long).wrapping_sub((*se).avg.runnable_sum as c_long);
        (*se).avg.runnable_sum = new_sum;
        update_sa_runnable!(addr_of_mut!((*cfs).avg), delta_avg, delta_sum);
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
unsafe fn update_tg_cfs_load(cfs: *mut b::cfs_rq, se: *mut b::sched_entity, gcfs: *mut b::cfs_rq) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut runnable = (*gcfs).prop_runnable_sum;
        if runnable == 0 {
            return;
        }
        (*gcfs).prop_runnable_sum = 0;
        let divider = b::rust_fair_get_pelt_divider(addr_of!((*cfs).avg));
        let mut load_sum = 0u64;
        if runnable >= 0 {
            runnable = min(
                runnable.wrapping_add((*se).avg.load_sum as c_long),
                divider as c_long,
            );
        } else {
            let weight = b::rust_fair_scale_load_down((*gcfs).load.weight);
            if weight != 0 {
                load_sum = (*gcfs).avg.load_sum / weight as u32 as u64;
            }
            runnable = min((*se).avg.load_sum, load_sum) as c_long;
        }
        runnable = max(
            runnable,
            ((*se).avg.util_sum >> b::SCHED_CAPACITY_SHIFT) as c_long,
        );
        load_sum = b::rust_fair_se_weight(se).wrapping_mul(runnable) as u64;
        let load_avg = (load_sum / divider as u64) as c_ulong;
        let delta_avg = load_avg.wrapping_sub((*se).avg.load_avg) as c_long;
        if delta_avg == 0 {
            return;
        }
        let delta_sum = load_sum
            .wrapping_sub((b::rust_fair_se_weight(se) as u64).wrapping_mul((*se).avg.load_sum))
            as i64;
        (*se).avg.load_sum = runnable as u64;
        (*se).avg.load_avg = load_avg;
        update_sa_load!(addr_of_mut!((*cfs).avg), delta_avg, delta_sum);
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
unsafe fn add_tg_cfs_propagate(cfs: *mut b::cfs_rq, sum: c_long) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (*cfs).propagate = 1;
        (*cfs).prop_runnable_sum = (*cfs).prop_runnable_sum.wrapping_add(sum);
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
unsafe fn propagate_entity_load_avg(se: *mut b::sched_entity) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if b::rust_fair_entity_is_task(se) {
            return 0;
        }
        let gcfs = b::rust_fair_group_cfs_rq(se);
        if (*gcfs).propagate == 0 {
            return 0;
        }
        (*gcfs).propagate = 0;
        let cfs = b::rust_fair_cfs_rq_of(se);
        add_tg_cfs_propagate(cfs, (*gcfs).prop_runnable_sum);
        update_tg_cfs_util(cfs, se, gcfs);
        update_tg_cfs_runnable(cfs, se, gcfs);
        update_tg_cfs_load(cfs, se, gcfs);
        b::rust_fair_trace_pelt_cfs(cfs);
        b::rust_fair_trace_pelt_se(se);
        1
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
unsafe fn skip_blocked_update(se: *mut b::sched_entity) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (*se).avg.load_avg == 0
            && (*se).avg.util_avg == 0
            && (*b::rust_fair_group_cfs_rq(se)).propagate == 0
    }
}
#[cfg(not(CONFIG_FAIR_GROUP_SCHED))]
unsafe fn update_tg_load_avg(_: *mut b::cfs_rq) {}
#[cfg(not(CONFIG_FAIR_GROUP_SCHED))]
unsafe fn clear_tg_offline_cfs_rqs(_: *mut b::rq) {}
#[cfg(not(CONFIG_FAIR_GROUP_SCHED))]
unsafe fn propagate_entity_load_avg(_: *mut b::sched_entity) -> c_int {
    0
}
#[cfg(not(CONFIG_FAIR_GROUP_SCHED))]
unsafe fn add_tg_cfs_propagate(_: *mut b::cfs_rq, _: c_long) {}
unsafe fn update_cfs_rq_load_avg(now: u64, cfs: *mut b::cfs_rq) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let sa = addr_of_mut!((*cfs).avg);
        let mut decayed = 0;
        if (*cfs).removed.nr != 0 {
            let divider = b::rust_fair_get_pelt_divider(sa);
            b::rust_fair_raw_spin_lock(addr_of_mut!((*cfs).removed.lock));
            let util = (*cfs).removed.util_avg;
            (*cfs).removed.util_avg = 0;
            let load = (*cfs).removed.load_avg;
            (*cfs).removed.load_avg = 0;
            let runnable = (*cfs).removed.runnable_avg;
            (*cfs).removed.runnable_avg = 0;
            (*cfs).removed.nr = 0;
            b::rust_fair_raw_spin_unlock(addr_of_mut!((*cfs).removed.lock));
            update_sa_load!(
                sa,
                load.wrapping_neg(),
                load.wrapping_neg().wrapping_mul(divider as c_ulong)
            );
            update_sa_util!(
                sa,
                util.wrapping_neg(),
                util.wrapping_neg().wrapping_mul(divider as c_ulong)
            );
            update_sa_runnable!(
                sa,
                runnable.wrapping_neg(),
                runnable.wrapping_neg().wrapping_mul(divider as c_ulong)
            );
            add_tg_cfs_propagate(
                cfs,
                (runnable.wrapping_mul(divider as c_ulong) as c_long).wrapping_neg()
                    >> b::SCHED_CAPACITY_SHIFT,
            );
            decayed = 1;
        }
        decayed |= b::__update_load_avg_cfs_rq(now, cfs);
        b::rust_fair_cfs_rq_store_last_update_time_copy(cfs, (*sa).last_update_time);
        decayed
    }
}
unsafe fn attach_entity_load_avg(cfs: *mut b::cfs_rq, se: *mut b::sched_entity) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let divider = b::rust_fair_get_pelt_divider(addr_of!((*cfs).avg));
        (*se).avg.last_update_time = (*cfs).avg.last_update_time;
        (*se).avg.period_contrib = (*cfs).avg.period_contrib;
        (*se).avg.util_sum = (*se).avg.util_avg.wrapping_mul(divider as c_ulong) as u32;
        (*se).avg.runnable_sum = (*se).avg.runnable_avg.wrapping_mul(divider as c_ulong) as u64;
        (*se).avg.load_sum = (*se).avg.load_avg.wrapping_mul(divider as c_ulong) as u64;
        let weight = b::rust_fair_se_weight(se) as u64;
        if weight < (*se).avg.load_sum {
            (*se).avg.load_sum /= weight as u32 as u64;
        } else {
            (*se).avg.load_sum = 1;
        }
        enqueue_load_avg(cfs, se);
        (*cfs).avg.util_avg = (*cfs).avg.util_avg.wrapping_add((*se).avg.util_avg);
        (*cfs).avg.util_sum = (*cfs).avg.util_sum.wrapping_add((*se).avg.util_sum);
        (*cfs).avg.runnable_avg = (*cfs).avg.runnable_avg.wrapping_add((*se).avg.runnable_avg);
        (*cfs).avg.runnable_sum = (*cfs).avg.runnable_sum.wrapping_add((*se).avg.runnable_sum);
        add_tg_cfs_propagate(cfs, (*se).avg.load_sum as c_long);
        cfs_rq_util_change(cfs, 0);
        b::rust_fair_trace_pelt_cfs(cfs);
    }
}
unsafe fn detach_entity_load_avg(cfs: *mut b::cfs_rq, se: *mut b::sched_entity) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        dequeue_load_avg(cfs, se);
        update_sa_util!(
            addr_of_mut!((*cfs).avg),
            (*se).avg.util_avg.wrapping_neg(),
            (*se).avg.util_sum.wrapping_neg()
        );
        update_sa_runnable!(
            addr_of_mut!((*cfs).avg),
            (*se).avg.runnable_avg.wrapping_neg(),
            (*se).avg.runnable_sum.wrapping_neg()
        );
        add_tg_cfs_propagate(cfs, (*se).avg.load_sum.wrapping_neg() as c_long);
        cfs_rq_util_change(cfs, 0);
        b::rust_fair_trace_pelt_cfs(cfs);
    }
}
unsafe fn util_est_update(se: *mut b::sched_entity) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if !feat!(rust_fair_feat_UTIL_EST) {
            return;
        }
        let mut ewma = read_once!((*se).avg.util_est);
        if ewma & b::UTIL_AVG_UNCHANGED != 0 {
            return;
        }
        let dequeued = read_once!((*se).avg.util_avg) as c_uint;
        if ewma <= dequeued {
            ewma = dequeued;
        } else {
            let diff = ewma.wrapping_sub(dequeued);
            if diff as c_ulong >= b::RUST_FAIR_UTIL_EST_MARGIN as c_ulong
                && (dequeued as c_ulong).wrapping_add(b::RUST_FAIR_UTIL_EST_MARGIN as c_ulong)
                    >= read_once!((*se).avg.runnable_avg)
            {
                ewma = ewma
                    .wrapping_shl(b::UTIL_EST_WEIGHT_SHIFT)
                    .wrapping_sub(diff)
                    >> b::UTIL_EST_WEIGHT_SHIFT;
            }
        }
        ewma |= b::UTIL_AVG_UNCHANGED;
        write_once!((*se).avg.util_est, ewma);
        b::rust_fair_trace_util_est_se(se);
    }
}
const UPDATE_TG: c_int = 0x01;
const SKIP_AGE_LOAD: c_int = 0x02;
const DO_ATTACH: c_int = 0x04;
const DO_DETACH: c_int = 0x08;
const UPDATE_UTIL_EST: c_int = 0x10;
unsafe fn update_load_avg(cfs: *mut b::cfs_rq, se: *mut b::sched_entity, flags: c_int) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let now = b::rust_fair_cfs_rq_clock_pelt(cfs);
        if (*se).avg.last_update_time != 0 && flags & SKIP_AGE_LOAD == 0 {
            b::__update_load_avg_se(now, cfs, se);
        }
        let decayed = update_cfs_rq_load_avg(now, cfs) | propagate_entity_load_avg(se);
        if (*se).avg.last_update_time == 0 && flags & DO_ATTACH != 0 {
            attach_entity_load_avg(cfs, se);
            update_tg_load_avg(cfs);
        } else if flags & DO_DETACH != 0 {
            detach_entity_load_avg(cfs, se);
            update_tg_load_avg(cfs);
        } else if decayed != 0 {
            cfs_rq_util_change(cfs, 0);
            if flags & UPDATE_TG != 0 {
                update_tg_load_avg(cfs);
            }
        }
        if flags & UPDATE_UTIL_EST != 0 {
            util_est_update(se);
        }
    }
}
unsafe fn sync_entity_load_avg(se: *mut b::sched_entity) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::__update_load_avg_blocked_se(cfs_rq_last_update_time(b::rust_fair_cfs_rq_of(se)), se);
    }
}
unsafe fn remove_entity_load_avg(se: *mut b::sched_entity) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let cfs = b::rust_fair_cfs_rq_of(se);
        sync_entity_load_avg(se);
        let flags = b::rust_fair_raw_spin_lock_irqsave(addr_of_mut!((*cfs).removed.lock));
        (*cfs).removed.nr = (*cfs).removed.nr.wrapping_add(1);
        (*cfs).removed.util_avg = (*cfs).removed.util_avg.wrapping_add((*se).avg.util_avg);
        (*cfs).removed.load_avg = (*cfs).removed.load_avg.wrapping_add((*se).avg.load_avg);
        (*cfs).removed.runnable_avg = (*cfs)
            .removed
            .runnable_avg
            .wrapping_add((*se).avg.runnable_avg);
        b::rust_fair_raw_spin_unlock_irqrestore(addr_of_mut!((*cfs).removed.lock), flags);
    }
}
unsafe fn cfs_rq_runnable_avg(cfs: *mut b::cfs_rq) -> c_ulong {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (*cfs).avg.runnable_avg
    }
}
unsafe fn cfs_rq_load_avg(cfs: *mut b::cfs_rq) -> c_ulong {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (*cfs).avg.load_avg
    }
}
unsafe fn task_util(p: *mut b::task_struct) -> c_ulong {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        read_once!((*p).se.avg.util_avg)
    }
}
unsafe fn _task_util_est(p: *mut b::task_struct) -> c_ulong {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (read_once!((*p).se.avg.util_est) & !b::UTIL_AVG_UNCHANGED) as c_ulong
    }
}
unsafe fn task_util_est(p: *mut b::task_struct) -> c_ulong {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        max(task_util(p), _task_util_est(p))
    }
}
unsafe fn util_est_enqueue(cfs: *mut b::cfs_rq, p: *mut b::task_struct) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if !feat!(rust_fair_feat_UTIL_EST) {
            return;
        }
        let enqueued = (*cfs)
            .avg
            .util_est
            .wrapping_add(_task_util_est(p) as c_uint);
        write_once!((*cfs).avg.util_est, enqueued);
        b::rust_fair_trace_util_est_cfs(cfs);
    }
}
unsafe fn util_est_dequeue(cfs: *mut b::cfs_rq, p: *mut b::task_struct) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if !feat!(rust_fair_feat_UTIL_EST) {
            return;
        }
        let enqueued = (*cfs)
            .avg
            .util_est
            .saturating_sub(_task_util_est(p) as c_uint);
        write_once!((*cfs).avg.util_est, enqueued);
        b::rust_fair_trace_util_est_cfs(cfs);
    }
}
unsafe fn get_actual_cpu_capacity(cpu: c_int) -> c_ulong {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (b::rust_fair_arch_scale_cpu_capacity(cpu) as u64).wrapping_sub(max(
            b::rust_fair_hw_load_avg(b::rust_fair_cpu_rq(cpu)),
            b::rust_fair_cpufreq_get_pressure(cpu) as u64,
        )) as c_ulong
    }
}
unsafe fn util_fits_cpu(util: c_ulong, mut umin: c_ulong, umax: c_ulong, cpu: c_int) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let capacity = capacity_of(cpu);
        let mut fits = fits_capacity(util, capacity);
        if !b::rust_fair_uclamp_is_used() {
            return fits as c_int;
        }
        let original = b::rust_fair_arch_scale_cpu_capacity(cpu);
        let mut max_fits = original == b::SCHED_CAPACITY_SCALE as c_ulong
            && umax == b::SCHED_CAPACITY_SCALE as c_ulong;
        max_fits = !max_fits && umax <= original;
        fits |= max_fits;
        umin = min(umin, umax);
        if fits && util < umin && umin > get_actual_cpu_capacity(cpu) {
            return -1;
        }
        fits as c_int
    }
}
unsafe fn task_fits_cpu(p: *mut b::task_struct, cpu: c_int) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (util_fits_cpu(
            task_util_est(p),
            b::rust_fair_uclamp_eff_value(p, b::UCLAMP_MIN),
            b::rust_fair_uclamp_eff_value(p, b::UCLAMP_MAX),
            cpu,
        ) > 0) as c_int
    }
}
unsafe fn update_misfit_status(p: *mut b::task_struct, rq: *mut b::rq) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let cpu = b::rust_fair_cpu_of(rq);
        if !b::rust_fair_sched_asym_cpucap_active() {
            return;
        }
        if p.is_null()
            || (*p).nr_cpus_allowed == 1
            || b::rust_fair_arch_scale_cpu_capacity(cpu) == (*p).max_allowed_capacity
            || task_fits_cpu(p, cpu) != 0
        {
            (*rq).misfit_task_load = 0;
            return;
        }
        (*rq).misfit_task_load = max(task_h_load(p), 1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn __setparam_fair(p: *mut b::task_struct, attr: *const b::sched_attr) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let se = addr_of_mut!((*p).se);
        (*p).static_prio = b::rust_fair_nice_to_prio((*attr).sched_nice);
        if (*attr).sched_runtime != 0 {
            (*se).custom_slice = 1;
            (*se).slice = (*attr)
                .sched_runtime
                .clamp(b::NSEC_PER_MSEC as u64 / 10, b::NSEC_PER_MSEC as u64 * 100);
        } else {
            (*se).custom_slice = 0;
            (*se).slice = sysctl_sched_base_slice as u64;
        }
    }
}
unsafe fn place_entity(cfs: *mut b::cfs_rq, se: *mut b::sched_entity, flags: c_int) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let vruntime = avg_vruntime(cfs);
        let mut nr = (*cfs).h_nr_queued;
        let mut update_zero = false;
        let mut lag = 0i64;
        if (*se).custom_slice == 0 {
            (*se).slice = sysctl_sched_base_slice as u64;
        }
        let mut vslice = calc_delta_fair((*se).slice, se);
        if flags & b::ENQUEUE_QUEUED as c_int != 0 {
            nr = nr.wrapping_sub(1);
        }
        if feat!(rust_fair_feat_PLACE_LAG) && nr != 0 && (*se).vlag != 0 {
            let curr = (*cfs).curr;
            lag = (*se).vlag;
            let mut load = (*cfs).sum_weight as c_long;
            if !curr.is_null() && (*curr).on_rq != 0 {
                load = load.wrapping_add(avg_vruntime_weight(cfs, (*curr).h_load.weight) as c_long);
            }
            let weight = avg_vruntime_weight(cfs, (*se).h_load.weight) as c_long;
            lag = lag.wrapping_mul(load.wrapping_add(weight) as i64);
            if load == 0 {
                b::rust_fair_warn_place_entity_1(true);
                load = 1;
            }
            lag = b::rust_fair_div64_long(lag, load);
            if weight > load {
                update_zero = true;
            }
        }
        (*se).vruntime = vruntime.wrapping_sub(lag as u64);
        if update_zero {
            update_zero_vruntime(cfs, lag.wrapping_neg());
        }
        if feat!(rust_fair_feat_PLACE_REL_DEADLINE) && (*se).rel_deadline != 0 {
            (*se).deadline = (*se).deadline.wrapping_add((*se).vruntime);
            (*se).rel_deadline = 0;
            return;
        }
        if feat!(rust_fair_feat_PLACE_DEADLINE_INITIAL) && flags & b::ENQUEUE_INITIAL as c_int != 0 {
            vslice /= 2;
        }
        (*se).deadline = (*se).vruntime.wrapping_add(vslice);
    }
}
unsafe fn enqueue_entity(cfs: *mut b::cfs_rq, se: *mut b::sched_entity, flags: c_int) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        update_load_avg(cfs, se, UPDATE_TG | DO_ATTACH);
        b::rust_fair_se_update_runnable(se);
        update_cfs_group(se);
        account_entity_enqueue(cfs, se);
        if flags & b::ENQUEUE_MIGRATED as c_int != 0 {
            (*se).exec_start = 0;
        }
        b::rust_fair_check_schedstat_required();
        update_stats_enqueue_fair(cfs, se, flags);
        (*se).on_rq = 1;
        if (*cfs).nr_queued == 1 {
            check_enqueue_throttle(cfs);
            list_add_leaf_cfs_rq(cfs);
            #[cfg(CONFIG_CFS_BANDWIDTH)]
            if b::rust_fair_cfs_pelt_clock_throttled(cfs) {
                let rq = b::rust_fair_rq_of(cfs);
                (*cfs).throttled_clock_pelt_time = (*cfs).throttled_clock_pelt_time.wrapping_add(
                    b::rust_fair_rq_clock_pelt(rq).wrapping_sub((*cfs).throttled_clock_pelt),
                );
                b::rust_fair_cfs_set_pelt_clock_throttled(cfs, false);
            }
        }
    }
}
unsafe fn set_next_buddy(cfs: *mut b::cfs_rq, se: *mut b::sched_entity) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if (*se).on_rq == 0 || (*se).sched_delayed != 0 {
            b::rust_fair_warn_set_next_buddy_1(true);
            return;
        }
        if se_is_idle(se) != 0 {
            return;
        }
        (*cfs).next = se;
    }
}
unsafe fn clear_buddies(cfs: *mut b::cfs_rq, se: *mut b::sched_entity) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if (*cfs).next == se {
            (*cfs).next = null_mut();
        }
    }
}
unsafe fn set_delayed(mut se: *mut b::sched_entity) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (*se).sched_delayed = 1;
        if !b::rust_fair_entity_is_task(se) {
            return;
        }
        while !se.is_null() {
            let cfs = b::rust_fair_cfs_rq_of(se);
            (*cfs).h_nr_runnable = (*cfs).h_nr_runnable.wrapping_sub(1);
            se = parent_entity(se);
        }
    }
}
unsafe fn clear_delayed(mut se: *mut b::sched_entity) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (*se).sched_delayed = 0;
        if !b::rust_fair_entity_is_task(se) {
            return;
        }
        while !se.is_null() {
            let cfs = b::rust_fair_cfs_rq_of(se);
            (*cfs).h_nr_runnable = (*cfs).h_nr_runnable.wrapping_add(1);
            se = parent_entity(se);
        }
    }
}
unsafe fn dequeue_entity(cfs: *mut b::cfs_rq, se: *mut b::sched_entity, flags: c_int) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut action = UPDATE_TG;
        if b::rust_fair_entity_is_task(se) {
            if b::rust_fair_task_on_rq_migrating(b::rust_fair_task_of(se)) {
                action |= DO_DETACH;
            }
            if flags & b::DEQUEUE_SLEEP as c_int != 0 && flags & b::DEQUEUE_DELAYED as c_int == 0 {
                action |= UPDATE_UTIL_EST;
            }
        }
        update_load_avg(cfs, se, action);
        b::rust_fair_se_update_runnable(se);
        update_stats_dequeue_fair(cfs, se, flags);
        (*se).on_rq = 0;
        account_entity_dequeue(cfs, se);
        return_cfs_rq_runtime(cfs);
        update_cfs_group(se);
        if (*cfs).nr_queued == 0 {
            b::rust_fair_update_idle_cfs_rq_clock_pelt(cfs);
            #[cfg(CONFIG_CFS_BANDWIDTH)]
            if throttled_hierarchy(cfs) != 0 {
                list_del_leaf_cfs_rq(cfs);
                (*cfs).throttled_clock_pelt = b::rust_fair_rq_clock_pelt(b::rust_fair_rq_of(cfs));
                b::rust_fair_cfs_set_pelt_clock_throttled(cfs, true);
            }
        }
    }
}
unsafe fn set_next_entity(cfs: *mut b::cfs_rq, se: *mut b::sched_entity) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if (*se).on_rq != 0 {
            update_stats_wait_end_fair(cfs, se);
            update_load_avg(cfs, se, UPDATE_TG);
        }
        update_stats_curr_start(cfs, se);
        b::rust_fair_warn_set_next_entity_1(!(*cfs).h_curr.is_null());
        (*cfs).h_curr = se;
        #[cfg(CONFIG_SCHEDSTATS)]
        if b::rust_fair_schedstat_enabled()
            && (*b::rust_fair_rq_of(cfs)).cfs.load.weight >= (*se).load.weight.wrapping_mul(2)
        {
            let stats = b::rust_fair_schedstats_from_se(se);
            (*stats).slice_max = max(
                (*stats).slice_max,
                (*se)
                    .sum_exec_runtime
                    .wrapping_sub((*se).prev_sum_exec_runtime),
            );
        }
        (*se).prev_sum_exec_runtime = (*se).sum_exec_runtime;
    }
}
unsafe fn pick_next_entity(rq: *mut b::rq, protect: bool) -> *mut b::sched_entity {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let se = pick_eevdf(addr_of_mut!((*rq).cfs), protect);
        if (*se).sched_delayed != 0 {
            __dequeue_task(
                rq,
                b::rust_fair_task_of(se),
                (b::DEQUEUE_SLEEP | b::DEQUEUE_DELAYED) as c_int,
            );
            return null_mut();
        }
        se
    }
}
unsafe fn put_prev_entity(cfs: *mut b::cfs_rq, prev: *mut b::sched_entity) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if (*prev).on_rq != 0 {
            update_curr(cfs);
        }
        if (*prev).on_rq != 0 {
            update_stats_wait_start_fair(cfs, prev);
            update_load_avg(cfs, prev, 0);
        }
        b::rust_fair_warn_put_prev_entity_1((*cfs).h_curr != prev);
        (*cfs).h_curr = null_mut();
    }
}
unsafe fn entity_tick(cfs: *mut b::cfs_rq, curr: *mut b::sched_entity, queued: c_int) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        update_curr(cfs);
        update_load_avg(cfs, curr, UPDATE_TG);
        update_cfs_group(curr);
        #[cfg(CONFIG_SCHED_HRTICK)]
        if queued != 0 {
            b::resched_curr(b::rust_fair_rq_of(cfs));
        }
    }
}
#[cfg(CONFIG_SCHED_HRTICK)]
unsafe fn hrtick_start_fair(rq: *mut b::rq, p: *mut b::task_struct) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let se = addr_of_mut!((*p).se);
        let mut scale: c_ulong = 1024;
        let mut util: c_ulong = 0;
        b::rust_fair_warn_hrtick_start_fair_1(b::rust_fair_task_rq(p) != rq);
        if (*rq).cfs.h_nr_queued <= 1 {
            return;
        }
        let vdelta = (*se).deadline.wrapping_sub((*se).vruntime);
        if (vdelta as i64) < 0 {
            if b::rust_fair_task_current_donor(rq, p) {
                b::resched_curr(rq);
            }
            return;
        }
        let delta = ((*se).h_load.weight as u64).wrapping_mul(vdelta) / b::RUST_FAIR_NICE_0_LOAD as u64;
        util = util.wrapping_add(b::rust_fair_cpu_util_irq(rq));
        if util != 0 && util < 1024 {
            scale = scale.wrapping_mul(1024) / (1024 - util);
        }
        b::hrtick_start(rq, (scale as u64).wrapping_mul(delta) / 1024);
    }
}
#[cfg(CONFIG_SCHED_HRTICK)]
unsafe fn hrtick_update(rq: *mut b::rq) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let donor = b::rust_fair_rq_donor(rq);
        if !b::rust_fair_hrtick_enabled_fair(rq)
            || (*donor).sched_class != addr_of!(fair_sched_class)
            || b::rust_fair_hrtick_active(rq)
        {
            return;
        }
        hrtick_start_fair(rq, donor);
    }
}
#[cfg(not(CONFIG_SCHED_HRTICK))]
unsafe fn hrtick_start_fair(_: *mut b::rq, _: *mut b::task_struct) {}
#[cfg(not(CONFIG_SCHED_HRTICK))]
unsafe fn hrtick_update(_: *mut b::rq) {}

#[cfg(all(CONFIG_CFS_BANDWIDTH, CONFIG_JUMP_LABEL))]
unsafe fn cfs_bandwidth_used() -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_cfs_bandwidth_key_enabled()
    }
}
#[cfg(all(CONFIG_CFS_BANDWIDTH, not(CONFIG_JUMP_LABEL)))]
unsafe fn cfs_bandwidth_used() -> bool {
    true
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
#[no_mangle]
pub unsafe extern "C" fn cfs_bandwidth_usage_inc() {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        #[cfg(CONFIG_JUMP_LABEL)]
        b::rust_fair_cfs_bandwidth_key_inc();
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
#[no_mangle]
pub unsafe extern "C" fn cfs_bandwidth_usage_dec() {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        #[cfg(CONFIG_JUMP_LABEL)]
        b::rust_fair_cfs_bandwidth_key_dec();
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
unsafe fn sched_cfs_bandwidth_slice() -> u64 {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (sysctl_sched_cfs_bandwidth_slice as u64).wrapping_mul(b::NSEC_PER_USEC as u64)
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
#[no_mangle]
pub unsafe extern "C" fn __refill_cfs_bandwidth_runtime(cfs_b: *mut b::cfs_bandwidth) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if (*cfs_b).quota == b::RUST_FAIR_RUNTIME_INF {
            return;
        }
        (*cfs_b).runtime = (*cfs_b).runtime.wrapping_add((*cfs_b).quota);
        let runtime = (*cfs_b).runtime_snap.wrapping_sub((*cfs_b).runtime) as i64;
        if runtime > 0 {
            (*cfs_b).burst_time = (*cfs_b).burst_time.wrapping_add(runtime as u64);
            (*cfs_b).nr_burst = (*cfs_b).nr_burst.wrapping_add(1);
        }
        (*cfs_b).runtime = min(
            (*cfs_b).runtime,
            (*cfs_b).quota.wrapping_add((*cfs_b).burst),
        );
        (*cfs_b).runtime_snap = (*cfs_b).runtime;
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
unsafe fn tg_cfs_bandwidth(tg: *mut b::task_group) -> *mut b::cfs_bandwidth {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        addr_of_mut!((*tg).cfs_bandwidth)
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
unsafe fn __assign_cfs_rq_runtime(
    cfs_b: *mut b::cfs_bandwidth,
    cfs: *mut b::cfs_rq,
    target: u64,
) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_lockdep_assert_raw_spin_held(addr_of_mut!((*cfs_b).lock));
        let minimum = target.wrapping_sub((*cfs).runtime_remaining as u64);
        let mut amount = 0;
        if (*cfs_b).quota == b::RUST_FAIR_RUNTIME_INF {
            amount = minimum;
        } else {
            start_cfs_bandwidth(cfs_b);
            if (*cfs_b).runtime > 0 {
                amount = min((*cfs_b).runtime, minimum);
                (*cfs_b).runtime -= amount;
                (*cfs_b).idle = 0;
            }
        }
        (*cfs).runtime_remaining = (*cfs).runtime_remaining.wrapping_add(amount as i64);
        ((*cfs).runtime_remaining > 0) as c_int
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
unsafe fn __account_cfs_rq_runtime(cfs: *mut b::cfs_rq, delta: u64) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (*cfs).runtime_remaining = (*cfs).runtime_remaining.wrapping_sub(delta as i64);
        if (*cfs).runtime_remaining > 0 {
            return false;
        }
        if b::rust_fair_cfs_throttled(cfs) {
            return true;
        }
        throttle_cfs_rq(cfs)
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
unsafe fn account_cfs_rq_runtime(cfs: *mut b::cfs_rq, delta: u64) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if !cfs_bandwidth_used() || (*cfs).runtime_enabled == 0 {
            return false;
        }
        __account_cfs_rq_runtime(cfs, delta)
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
unsafe fn cfs_rq_throttled(cfs: *mut b::cfs_rq) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (cfs_bandwidth_used() && b::rust_fair_cfs_throttled(cfs)) as c_int
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
unsafe fn cfs_rq_pelt_clock_throttled(cfs: *mut b::cfs_rq) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        cfs_bandwidth_used() && b::rust_fair_cfs_pelt_clock_throttled(cfs)
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
unsafe fn throttled_hierarchy(cfs: *mut b::cfs_rq) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (cfs_bandwidth_used() && (*cfs).throttle_count != 0) as c_int
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
unsafe fn lb_throttled_hierarchy(p: *mut b::task_struct, cpu: c_int) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        throttled_hierarchy(b::rust_fair_tg_cfs_rq(b::rust_fair_task_group(p), cpu))
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
unsafe fn task_is_throttled(p: *mut b::task_struct) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        cfs_bandwidth_used() && (*p).throttled
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
unsafe extern "C" fn throttle_cfs_rq_work(work: *mut b::callback_head) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let p = work
            .cast::<u8>()
            .sub(core::mem::offset_of!(b::task_struct, sched_throttle_work))
            .cast::<b::task_struct>();
        b::rust_fair_warn_throttle_cfs_rq_work_1(p != b::rust_fair_current());
        (*p).sched_throttle_work.next = addr_of_mut!((*p).sched_throttle_work);
        if (*p).flags & b::PF_EXITING != 0 {
            return;
        }
        let mut rf: b::rq_flags = core::mem::zeroed();
        let rq = b::rust_fair_task_rq_lock(p, addr_of_mut!(rf));
        let cfs = b::rust_fair_cfs_rq_of(addr_of_mut!((*p).se));
        if (*p).sched_class == addr_of!(fair_sched_class) && (*cfs).throttle_count != 0 {
            b::update_rq_clock(rq);
            b::rust_fair_warn_throttle_cfs_rq_work_2(
                (*p).throttled || !b::rust_fair_list_empty(addr_of!((*p).throttle_node)),
            );
            dequeue_task_fair(rq, p, (b::DEQUEUE_SLEEP | b::DEQUEUE_THROTTLE) as c_int);
            b::rust_fair_list_add(
                addr_of_mut!((*p).throttle_node),
                addr_of_mut!((*cfs).throttled_limbo_list),
            );
            (*p).throttled = true;
            b::resched_curr(rq);
        }
        b::rust_fair_task_rq_unlock(rq, p, addr_of_mut!(rf));
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
#[no_mangle]
pub unsafe extern "C" fn init_cfs_throttle_work(p: *mut b::task_struct) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_init_task_work(
            addr_of_mut!((*p).sched_throttle_work),
            Some(throttle_cfs_rq_work),
        );
        (*p).sched_throttle_work.next = addr_of_mut!((*p).sched_throttle_work);
        b::rust_fair_init_list_head(addr_of_mut!((*p).throttle_node));
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
unsafe fn dequeue_throttled_task(p: *mut b::task_struct, flags: c_int) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_warn_dequeue_throttled_task_1((*p).se.on_rq != 0);
        b::rust_fair_list_del_init(addr_of_mut!((*p).throttle_node));
        if flags & b::DEQUEUE_SLEEP as c_int != 0 {
            (*p).throttled = false;
            return;
        }
        if b::rust_fair_task_on_rq_migrating(p) {
            detach_task_cfs_rq(p);
        }
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
unsafe fn enqueue_throttled_task(p: *mut b::task_struct) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let cfs = b::rust_fair_cfs_rq_of(addr_of_mut!((*p).se));
        b::rust_fair_warn_enqueue_throttled_task_1(!b::rust_fair_list_empty(addr_of!(
            (*p).throttle_node
        )));
        if throttled_hierarchy(cfs) != 0 && !b::rust_fair_task_current_donor(b::rust_fair_rq_of(cfs), p)
        {
            b::rust_fair_list_add(
                addr_of_mut!((*p).throttle_node),
                addr_of_mut!((*cfs).throttled_limbo_list),
            );
            return true;
        }
        (*p).throttled = false;
        false
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
unsafe extern "C" fn tg_unthrottle_up(
    tg: *mut b::task_group,
    data: *mut core::ffi::c_void,
) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let rq = data.cast::<b::rq>();
        let cfs = b::rust_fair_tg_cfs_rq(tg, b::rust_fair_cpu_of(rq));
        let mut tasks: b::list_head = core::mem::zeroed();
        b::rust_fair_init_list_head(addr_of_mut!(tasks));
        update_curr(cfs);
        (*cfs).throttle_count = (*cfs).throttle_count.wrapping_sub(1);
        if (*cfs).throttle_count != 0 {
            return 0;
        }
        if b::rust_fair_cfs_pelt_clock_throttled(cfs) {
            (*cfs).throttled_clock_pelt_time = (*cfs)
                .throttled_clock_pelt_time
                .wrapping_add(b::rust_fair_rq_clock_pelt(rq).wrapping_sub((*cfs).throttled_clock_pelt));
            b::rust_fair_cfs_set_pelt_clock_throttled(cfs, false);
        }
        if (*cfs).throttled_clock_self != 0 {
            let mut delta = b::rust_fair_rq_clock(rq).wrapping_sub((*cfs).throttled_clock_self);
            (*cfs).throttled_clock_self = 0;
            if (delta as i64) < 0 {
                b::rust_fair_warn_tg_unthrottle_up_1(true);
                delta = 0;
            }
            (*cfs).throttled_clock_self_time = (*cfs).throttled_clock_self_time.wrapping_add(delta);
        }
        b::rust_fair_list_splice_init(
            addr_of_mut!((*cfs).throttled_limbo_list),
            addr_of_mut!(tasks),
        );
        let mut node = tasks.next;
        while node != addr_of_mut!(tasks) {
            let next = (*node).next;
            if (*cfs).throttle_count != 0 {
                break;
            }
            let p = node
                .cast::<u8>()
                .sub(core::mem::offset_of!(b::task_struct, throttle_node))
                .cast::<b::task_struct>();
            b::rust_fair_list_del_init(addr_of_mut!((*p).throttle_node));
            (*p).throttled = false;
            enqueue_task_fair(rq, p, b::ENQUEUE_WAKEUP as c_int);
            node = next;
        }
        b::rust_fair_list_splice(
            addr_of_mut!(tasks),
            addr_of_mut!((*cfs).throttled_limbo_list),
        );
        if !cfs_rq_is_decayed(cfs) {
            list_add_leaf_cfs_rq(cfs);
        }
        0
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
unsafe fn task_has_throttle_work(p: *mut b::task_struct) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (*p).sched_throttle_work.next != addr_of_mut!((*p).sched_throttle_work)
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
unsafe fn task_throttle_setup_work(p: *mut b::task_struct) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if task_has_throttle_work(p) || (*p).flags & (b::PF_EXITING | b::PF_KTHREAD) != 0 {
            return;
        }
        b::task_work_add(p, addr_of_mut!((*p).sched_throttle_work), b::TWA_RESUME);
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
unsafe fn record_throttle_clock(cfs: *mut b::cfs_rq) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let rq = b::rust_fair_rq_of(cfs);
        if cfs_rq_throttled(cfs) != 0 && (*cfs).throttled_clock == 0 {
            (*cfs).throttled_clock = b::rust_fair_rq_clock(rq);
        }
        if (*cfs).throttled_clock_self == 0 {
            (*cfs).throttled_clock_self = b::rust_fair_rq_clock(rq);
        }
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
unsafe extern "C" fn tg_throttle_down(
    tg: *mut b::task_group,
    data: *mut core::ffi::c_void,
) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let rq = data.cast::<b::rq>();
        let cfs = b::rust_fair_tg_cfs_rq(tg, b::rust_fair_cpu_of(rq));
        let old = (*cfs).throttle_count;
        (*cfs).throttle_count = old.wrapping_add(1);
        if old != 0 {
            return 0;
        }
        if (*cfs).nr_queued == 0 {
            list_del_leaf_cfs_rq(cfs);
            (*cfs).throttled_clock_pelt = b::rust_fair_rq_clock_pelt(rq);
            b::rust_fair_cfs_set_pelt_clock_throttled(cfs, true);
        }
        b::rust_fair_warn_tg_throttle_down_1((*cfs).throttled_clock_self != 0);
        b::rust_fair_warn_tg_throttle_down_2(!b::rust_fair_list_empty(addr_of!(
            (*cfs).throttled_limbo_list
        )));
        0
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
unsafe fn throttle_cfs_rq(cfs: *mut b::cfs_rq) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let cb = tg_cfs_bandwidth((*cfs).tg);
        let curr = (*cfs).curr;
        let rq = b::rust_fair_rq_of(cfs);
        b::rust_fair_raw_spin_lock(addr_of_mut!((*cb).lock));
        let target = if !curr.is_null() && (*curr).on_rq != 0 {
            sched_cfs_bandwidth_slice()
        } else {
            1
        };
        if __assign_cfs_rq_runtime(cb, cfs, target) != 0 {
            b::rust_fair_raw_spin_unlock(addr_of_mut!((*cb).lock));
            return false;
        }
        b::rust_fair_list_add_tail_rcu(
            addr_of_mut!((*cfs).throttled_list),
            addr_of_mut!((*cb).throttled_cfs_rq),
        );
        b::rust_fair_raw_spin_unlock(addr_of_mut!((*cb).lock));
        b::rust_fair_rcu_read_lock();
        b::walk_tg_tree_from(
            (*cfs).tg,
            Some(tg_throttle_down),
            Some(b::tg_nop),
            rq.cast(),
        );
        b::rust_fair_rcu_read_unlock();
        b::rust_fair_cfs_set_throttled(cfs, true);
        b::rust_fair_warn_throttle_cfs_rq_1((*cfs).throttled_clock != 0);
        if !curr.is_null() && (*curr).on_rq != 0 {
            task_throttle_setup_work(b::rust_fair_rq_donor(rq));
        }
        true
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
#[no_mangle]
pub unsafe extern "C" fn unthrottle_cfs_rq(cfs: *mut b::cfs_rq) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let rq = b::rust_fair_rq_of(cfs);
        let cb = tg_cfs_bandwidth((*cfs).tg);
        let mut se = b::rust_fair_cfs_rq_se(cfs);
        update_curr(cfs);
        if (*cfs).runtime_enabled != 0 && (*cfs).runtime_remaining <= 0 {
            return;
        }
        b::rust_fair_cfs_set_throttled(cfs, false);
        b::rust_fair_raw_spin_lock(addr_of_mut!((*cb).lock));
        b::rust_fair_list_del_rcu(addr_of_mut!((*cfs).throttled_list));
        if (*cfs).throttled_clock != 0 {
            (*cb).throttled_time = (*cb)
                .throttled_time
                .wrapping_add(b::rust_fair_rq_clock(rq).wrapping_sub((*cfs).throttled_clock));
            (*cfs).throttled_clock = 0;
        }
        b::rust_fair_raw_spin_unlock(addr_of_mut!((*cb).lock));
        b::walk_tg_tree_from(
            (*cfs).tg,
            Some(b::tg_nop),
            Some(tg_unthrottle_up),
            rq.cast(),
        );
        if (*cfs).load.weight == 0 {
            if (*cfs).on_list == 0 {
                return;
            }
            while !se.is_null() {
                if list_add_leaf_cfs_rq(b::rust_fair_cfs_rq_of(se)) {
                    break;
                }
                se = parent_entity(se);
            }
        }
        assert_list_leaf_cfs_rq(rq);
        if b::rust_fair_rq_curr(rq) == (*rq).idle && (*rq).cfs.h_nr_queued != 0 {
            b::resched_curr(rq);
        }
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
#[no_mangle]
pub unsafe extern "C" fn __cfsb_csd_unthrottle(arg: *mut core::ffi::c_void) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let rq = arg.cast::<b::rq>();
        let mut rf: b::rq_flags = core::mem::zeroed();
        b::rust_fair_rq_lock(rq, addr_of_mut!(rf));
        b::update_rq_clock(rq);
        b::rust_fair_rq_clock_start_loop_update(rq);
        b::rust_fair_rcu_read_lock();
        let head = addr_of_mut!((*rq).cfsb_csd_list);
        let mut node = (*head).next;
        while node != head {
            let next = (*node).next;
            let cfs = node
                .cast::<u8>()
                .sub(core::mem::offset_of!(b::cfs_rq, throttled_csd_list))
                .cast::<b::cfs_rq>();
            b::rust_fair_list_del_init(node);
            if cfs_rq_throttled(cfs) != 0 {
                unthrottle_cfs_rq(cfs);
            }
            node = next;
        }
        b::rust_fair_rq_clock_stop_loop_update(rq);
        b::rust_fair_rcu_read_unlock();
        b::rust_fair_rq_unlock(rq, addr_of_mut!(rf));
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
unsafe fn __unthrottle_cfs_rq_async(cfs: *mut b::cfs_rq) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let rq = b::rust_fair_rq_of(cfs);
        if rq == b::rust_fair_this_rq() {
            b::update_rq_clock(rq);
            unthrottle_cfs_rq(cfs);
            return;
        }
        if !b::rust_fair_list_empty(addr_of!((*cfs).throttled_csd_list)) {
            b::rust_fair_warn___unthrottle_cfs_rq_async_1(true);
            return;
        }
        let first = b::rust_fair_list_empty(addr_of!((*rq).cfsb_csd_list));
        b::rust_fair_list_add_tail(
            addr_of_mut!((*cfs).throttled_csd_list),
            addr_of_mut!((*rq).cfsb_csd_list),
        );
        if first {
            b::smp_call_function_single_async(b::rust_fair_cpu_of(rq), addr_of_mut!((*rq).cfsb_csd));
        }
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
unsafe fn unthrottle_cfs_rq_async(cfs: *mut b::cfs_rq) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_lockdep_assert_rq_held(b::rust_fair_rq_of(cfs));
        if cfs_rq_throttled(cfs) == 0 || (*cfs).runtime_remaining <= 0 {
            b::rust_fair_warn_unthrottle_cfs_rq_async_1(true);
            return;
        }
        __unthrottle_cfs_rq_async(cfs);
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
unsafe fn distribute_cfs_runtime(cb: *mut b::cfs_bandwidth) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut throttled = false;
        let mut unthrottle_local = false;
        let cpu = b::rust_fair_smp_processor_id();
        let mut remaining = 1u64;
        b::rust_fair_rcu_read_lock();
        let head = addr_of_mut!((*cb).throttled_cfs_rq);
        let mut node = b::rust_fair_list_next_rcu(head);
        while node != head {
            let cfs = node
                .cast::<u8>()
                .sub(core::mem::offset_of!(b::cfs_rq, throttled_list))
                .cast::<b::cfs_rq>();
            let rq = b::rust_fair_rq_of(cfs);
            if remaining == 0 {
                throttled = true;
                break;
            }
            let mut rf: b::rq_flags = core::mem::zeroed();
            b::rust_fair_rq_lock_irqsave(rq, addr_of_mut!(rf));
            if cfs_rq_throttled(cfs) == 0
                || !b::rust_fair_list_empty(addr_of!((*cfs).throttled_csd_list))
            {
                b::rust_fair_rq_unlock_irqrestore(rq, addr_of_mut!(rf));
                node = b::rust_fair_list_next_rcu(node);
                continue;
            }
            if !(*cfs).curr.is_null() {
                b::update_rq_clock(rq);
                update_curr(cfs);
            }
            b::rust_fair_warn_distribute_cfs_runtime_1((*cfs).runtime_remaining > 0);
            b::rust_fair_raw_spin_lock(addr_of_mut!((*cb).lock));
            let runtime = min(
                (*cfs).runtime_remaining.wrapping_neg().wrapping_add(1) as u64,
                (*cb).runtime,
            );
            (*cb).runtime = (*cb).runtime.wrapping_sub(runtime);
            remaining = (*cb).runtime;
            b::rust_fair_raw_spin_unlock(addr_of_mut!((*cb).lock));
            (*cfs).runtime_remaining = (*cfs).runtime_remaining.wrapping_add(runtime as i64);
            if (*cfs).runtime_remaining <= 0 {
                throttled = true;
                b::rust_fair_rq_unlock_irqrestore(rq, addr_of_mut!(rf));
                break;
            }
            if b::rust_fair_cpu_of(rq) != cpu {
                unthrottle_cfs_rq_async(cfs);
            } else {
                unthrottle_local = b::rust_fair_list_empty(addr_of!((*rq).cfsb_csd_list));
                b::rust_fair_list_add_tail(
                    addr_of_mut!((*cfs).throttled_csd_list),
                    addr_of_mut!((*rq).cfsb_csd_list),
                );
            }
            b::rust_fair_rq_unlock_irqrestore(rq, addr_of_mut!(rf));
            node = b::rust_fair_list_next_rcu(node);
        }
        if unthrottle_local {
            let flags = b::rust_fair_local_irq_save();
            __cfsb_csd_unthrottle(b::rust_fair_cpu_rq(cpu).cast());
            b::rust_fair_local_irq_restore(flags);
        }
        b::rust_fair_rcu_read_unlock();
        throttled
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
unsafe fn do_sched_cfs_period_timer(cb: *mut b::cfs_bandwidth, overrun: c_int) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if (*cb).quota == b::RUST_FAIR_RUNTIME_INF {
            return 1;
        }
        let mut throttled = !b::rust_fair_list_empty(addr_of!((*cb).throttled_cfs_rq));
        (*cb).nr_periods = (*cb).nr_periods.wrapping_add(overrun as _);
        __refill_cfs_bandwidth_runtime(cb);
        if (*cb).idle != 0 && !throttled {
            return 1;
        }
        if !throttled {
            (*cb).idle = 1;
            return 0;
        }
        (*cb).nr_throttled = (*cb).nr_throttled.wrapping_add(overrun as _);
        while throttled && (*cb).runtime > 0 {
            b::rust_fair_raw_spin_unlock_irq_enable(addr_of_mut!((*cb).lock));
            throttled = distribute_cfs_runtime(cb);
            b::rust_fair_raw_spin_lock_irq_disable(addr_of_mut!((*cb).lock));
        }
        (*cb).idle = 0;
        0
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
const min_cfs_rq_runtime: u64 = b::NSEC_PER_MSEC as u64;
#[cfg(CONFIG_CFS_BANDWIDTH)]
const min_bandwidth_expiration: u64 = 2 * b::NSEC_PER_MSEC as u64;
#[cfg(CONFIG_CFS_BANDWIDTH)]
const cfs_bandwidth_slack_period: u64 = 5 * b::NSEC_PER_MSEC as u64;
#[cfg(CONFIG_CFS_BANDWIDTH)]
unsafe fn runtime_refresh_within(cb: *mut b::cfs_bandwidth, expire: u64) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let timer = addr_of_mut!((*cb).period_timer);
        if b::rust_fair_hrtimer_callback_running(timer) {
            return 1;
        }
        (b::rust_fair_hrtimer_expires_remaining_ns(timer) < expire as i64) as c_int
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
unsafe fn start_cfs_slack_bandwidth(cb: *mut b::cfs_bandwidth) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if runtime_refresh_within(cb, cfs_bandwidth_slack_period + min_bandwidth_expiration) != 0
            || (*cb).slack_started != 0
        {
            return;
        }
        (*cb).slack_started = 1;
        b::rust_fair_hrtimer_start_ns(
            addr_of_mut!((*cb).slack_timer),
            cfs_bandwidth_slack_period,
            b::HRTIMER_MODE_REL,
        );
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
unsafe fn __return_cfs_rq_runtime(cfs: *mut b::cfs_rq) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let cb = tg_cfs_bandwidth((*cfs).tg);
        let slack = (*cfs)
            .runtime_remaining
            .wrapping_sub(min_cfs_rq_runtime as i64);
        if slack <= 0 {
            return;
        }
        b::rust_fair_raw_spin_lock(addr_of_mut!((*cb).lock));
        if (*cb).quota != b::RUST_FAIR_RUNTIME_INF {
            (*cb).runtime = (*cb).runtime.wrapping_add(slack as u64);
            if (*cb).runtime > sched_cfs_bandwidth_slice()
                && !b::rust_fair_list_empty(addr_of!((*cb).throttled_cfs_rq))
            {
                start_cfs_slack_bandwidth(cb);
            }
        }
        (*cfs).runtime_remaining = (*cfs).runtime_remaining.wrapping_sub(slack);
        b::rust_fair_raw_spin_unlock(addr_of_mut!((*cb).lock));
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
unsafe fn return_cfs_rq_runtime(cfs: *mut b::cfs_rq) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if !cfs_bandwidth_used() || (*cfs).runtime_enabled == 0 || (*cfs).nr_queued != 0 {
            return;
        }
        __return_cfs_rq_runtime(cfs);
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
unsafe fn do_sched_cfs_slack_timer(cb: *mut b::cfs_bandwidth) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_raw_spin_lock_irq(addr_of_mut!((*cb).lock));
        (*cb).slack_started = 0;
        let refresh = runtime_refresh_within(cb, min_bandwidth_expiration) != 0;
        let runtime =
            if (*cb).quota != b::RUST_FAIR_RUNTIME_INF && (*cb).runtime > sched_cfs_bandwidth_slice() {
                (*cb).runtime
            } else {
                0
            };
        b::rust_fair_raw_spin_unlock_irq(addr_of_mut!((*cb).lock));
        if refresh || runtime == 0 {
            return;
        }
        distribute_cfs_runtime(cb);
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
unsafe fn check_enqueue_throttle(cfs: *mut b::cfs_rq) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if !cfs_bandwidth_used()
            || (*cfs).runtime_enabled == 0
            || !(*cfs).h_curr.is_null()
            || cfs_rq_throttled(cfs) != 0
        {
            return;
        }
        account_cfs_rq_runtime(cfs, 0);
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
unsafe fn sync_throttle(tg: *mut b::task_group, cpu: c_int) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if !cfs_bandwidth_used() || (*tg).parent.is_null() {
            return;
        }
        let cfs = b::rust_fair_tg_cfs_rq(tg, cpu);
        let parent = b::rust_fair_tg_cfs_rq((*tg).parent, cpu);
        (*cfs).throttle_count = (*parent).throttle_count;
        (*cfs).throttled_clock_pelt = b::rust_fair_rq_clock_pelt(b::rust_fair_cpu_rq(cpu));
        if (*cfs).throttle_count != 0 {
            b::rust_fair_cfs_set_pelt_clock_throttled(cfs, true);
        }
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
unsafe extern "C" fn sched_cfs_slack_timer(timer: *mut b::hrtimer) -> b::hrtimer_restart {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let cb = timer
            .cast::<u8>()
            .sub(core::mem::offset_of!(b::cfs_bandwidth, slack_timer))
            .cast::<b::cfs_bandwidth>();
        do_sched_cfs_slack_timer(cb);
        b::HRTIMER_NORESTART
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
unsafe extern "C" fn sched_cfs_period_timer(timer: *mut b::hrtimer) -> b::hrtimer_restart {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let cb = timer
            .cast::<u8>()
            .sub(core::mem::offset_of!(b::cfs_bandwidth, period_timer))
            .cast::<b::cfs_bandwidth>();
        let mut idle = 0;
        let mut count: c_int = 0;
        b::rust_fair_raw_spin_lock_irq(addr_of_mut!((*cb).lock));
        loop {
            let overrun = b::rust_fair_hrtimer_forward_now(timer, (*cb).period) as c_int;
            if overrun == 0 {
                break;
            }
            idle = do_sched_cfs_period_timer(cb, overrun);
            count += 1;
            if count > 3 {
                let old = b::rust_fair_ktime_to_ns((*cb).period) as u64;
                let new = old.wrapping_mul(2);
                if new < (b::max_bw_quota_period_us as u64).wrapping_mul(b::NSEC_PER_USEC as u64) {
                    (*cb).period = b::rust_fair_ns_to_ktime(new);
                    (*cb).quota = (*cb).quota.wrapping_mul(2);
                    (*cb).burst = (*cb).burst.wrapping_mul(2);
                    b::rust_fair_warn_period_scaled(
                        b::rust_fair_smp_processor_id(),
                        new / b::NSEC_PER_USEC as u64,
                        (*cb).quota / b::NSEC_PER_USEC as u64,
                    );
                } else {
                    b::rust_fair_warn_period_unscalable(
                        b::rust_fair_smp_processor_id(),
                        old / b::NSEC_PER_USEC as u64,
                        (*cb).quota / b::NSEC_PER_USEC as u64,
                    );
                }
                count = 0;
            }
        }
        if idle != 0 {
            (*cb).period_active = 0;
        }
        b::rust_fair_raw_spin_unlock_irq(addr_of_mut!((*cb).lock));
        if idle != 0 {
            b::HRTIMER_NORESTART
        } else {
            b::HRTIMER_RESTART
        }
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
#[no_mangle]
pub unsafe extern "C" fn init_cfs_bandwidth(
    cb: *mut b::cfs_bandwidth,
    parent: *mut b::cfs_bandwidth,
) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_init_cfs_bandwidth_lock(addr_of_mut!((*cb).lock));
        (*cb).runtime = 0;
        (*cb).quota = b::RUST_FAIR_RUNTIME_INF;
        (*cb).period = b::rust_fair_us_to_ktime(b::rust_fair_default_bw_period_us());
        (*cb).burst = 0;
        (*cb).hierarchical_quota = if parent.is_null() {
            b::RUST_FAIR_RUNTIME_INF as i64
        } else {
            (*parent).hierarchical_quota
        };
        b::rust_fair_init_list_head(addr_of_mut!((*cb).throttled_cfs_rq));
        b::hrtimer_setup(
            addr_of_mut!((*cb).period_timer),
            Some(sched_cfs_period_timer),
            b::CLOCK_MONOTONIC as c_int,
            b::HRTIMER_MODE_ABS_PINNED,
        );
        let expires = b::rust_fair_get_random_u32_below((*cb).period as u32);
        b::rust_fair_hrtimer_set_expires(addr_of_mut!((*cb).period_timer), expires as b::ktime_t);
        b::hrtimer_setup(
            addr_of_mut!((*cb).slack_timer),
            Some(sched_cfs_slack_timer),
            b::CLOCK_MONOTONIC as c_int,
            b::HRTIMER_MODE_REL,
        );
        (*cb).slack_started = 0;
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
unsafe fn init_cfs_rq_runtime(cfs: *mut b::cfs_rq) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (*cfs).runtime_enabled = 0;
        b::rust_fair_init_list_head(addr_of_mut!((*cfs).throttled_list));
        b::rust_fair_init_list_head(addr_of_mut!((*cfs).throttled_csd_list));
        b::rust_fair_init_list_head(addr_of_mut!((*cfs).throttled_limbo_list));
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
#[no_mangle]
pub unsafe extern "C" fn start_cfs_bandwidth(cb: *mut b::cfs_bandwidth) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_lockdep_assert_raw_spin_held(addr_of_mut!((*cb).lock));
        if (*cb).period_active != 0 {
            return;
        }
        (*cb).period_active = 1;
        b::rust_fair_hrtimer_forward_now(addr_of_mut!((*cb).period_timer), (*cb).period);
        b::rust_fair_hrtimer_start_expires(
            addr_of_mut!((*cb).period_timer),
            b::HRTIMER_MODE_ABS_PINNED,
        );
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
unsafe fn destroy_cfs_bandwidth(cb: *mut b::cfs_bandwidth) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if (*cb).throttled_cfs_rq.next.is_null() {
            return;
        }
        b::hrtimer_cancel(addr_of_mut!((*cb).period_timer));
        b::hrtimer_cancel(addr_of_mut!((*cb).slack_timer));
        let mask = b::rust_fair_cpu_possible_mask();
        let mut cpu = b::rust_fair_cpumask_first(mask);
        while cpu < b::rust_fair_small_cpumask_bits() {
            let rq = b::rust_fair_cpu_rq(cpu as c_int);
            if !b::rust_fair_list_empty(addr_of!((*rq).cfsb_csd_list)) {
                let flags = b::rust_fair_local_irq_save();
                __cfsb_csd_unthrottle(rq.cast());
                b::rust_fair_local_irq_restore(flags);
            }
            cpu = b::rust_fair_cpumask_next(cpu as c_int, mask);
        }
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
unsafe fn update_runtime_enabled(rq: *mut b::rq) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_lockdep_assert_rq_held(rq);
        b::rust_fair_rcu_read_lock();
        let head = addr_of_mut!(b::task_groups);
        let mut node = b::rust_fair_list_next_rcu(head);
        while node != head {
            let tg = node
                .cast::<u8>()
                .sub(core::mem::offset_of!(b::task_group, list))
                .cast::<b::task_group>();
            let cb = tg_cfs_bandwidth(tg);
            let cfs = b::rust_fair_tg_cfs_rq(tg, b::rust_fair_cpu_of(rq));
            b::rust_fair_raw_spin_lock(addr_of_mut!((*cb).lock));
            (*cfs).runtime_enabled = ((*cb).quota != b::RUST_FAIR_RUNTIME_INF) as _;
            b::rust_fair_raw_spin_unlock(addr_of_mut!((*cb).lock));
            node = b::rust_fair_list_next_rcu(node);
        }
        b::rust_fair_rcu_read_unlock();
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
unsafe fn unthrottle_offline_cfs_rqs(rq: *mut b::rq) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_lockdep_assert_rq_held(rq);
        if b::rust_fair_cpu_active(b::rust_fair_cpu_of(rq)) {
            return;
        }
        b::rust_fair_rq_clock_start_loop_update(rq);
        b::rust_fair_rcu_read_lock();
        let head = addr_of_mut!(b::task_groups);
        let mut node = b::rust_fair_list_next_rcu(head);
        while node != head {
            let tg = node
                .cast::<u8>()
                .sub(core::mem::offset_of!(b::task_group, list))
                .cast::<b::task_group>();
            let cfs = b::rust_fair_tg_cfs_rq(tg, b::rust_fair_cpu_of(rq));
            if (*cfs).runtime_enabled != 0 {
                (*cfs).runtime_enabled = 0;
                if cfs_rq_throttled(cfs) != 0 {
                    (*cfs).runtime_remaining = 1;
                    unthrottle_cfs_rq(cfs);
                }
            }
            node = b::rust_fair_list_next_rcu(node);
        }
        b::rust_fair_rq_clock_stop_loop_update(rq);
        b::rust_fair_rcu_read_unlock();
    }
}
#[cfg(CONFIG_CFS_BANDWIDTH)]
#[no_mangle]
pub unsafe extern "C" fn cfs_task_bw_constrained(p: *mut b::task_struct) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let cfs = b::rust_fair_task_cfs_rq(p);
        if !cfs_bandwidth_used() {
            return false;
        }
        (*cfs).runtime_enabled != 0
            || (*tg_cfs_bandwidth((*cfs).tg)).hierarchical_quota != b::RUST_FAIR_RUNTIME_INF as i64
    }
}
#[cfg(all(CONFIG_CFS_BANDWIDTH, CONFIG_NO_HZ_FULL))]
unsafe fn sched_fair_update_stop_tick(rq: *mut b::rq, p: *mut b::task_struct) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let cpu = b::rust_fair_cpu_of(rq);
        if !cfs_bandwidth_used() || !b::rust_fair_tick_nohz_full_cpu(cpu) || (*rq).nr_running != 1 {
            return;
        }
        if cfs_task_bw_constrained(p) {
            b::tick_nohz_dep_set_cpu(cpu, b::TICK_DEP_BIT_SCHED);
        }
    }
}
// The following are original disabled-configuration implementations, not gaps.
#[cfg(not(CONFIG_CFS_BANDWIDTH))]
unsafe fn account_cfs_rq_runtime(_: *mut b::cfs_rq, _: u64) -> bool {
    false
}
#[cfg(not(CONFIG_CFS_BANDWIDTH))]
unsafe fn check_enqueue_throttle(_: *mut b::cfs_rq) {}
#[cfg(not(CONFIG_CFS_BANDWIDTH))]
unsafe fn sync_throttle(_: *mut b::task_group, _: c_int) {}
#[cfg(not(CONFIG_CFS_BANDWIDTH))]
unsafe fn return_cfs_rq_runtime(_: *mut b::cfs_rq) {}
#[cfg(not(CONFIG_CFS_BANDWIDTH))]
unsafe fn task_throttle_setup_work(_: *mut b::task_struct) {}
#[cfg(not(CONFIG_CFS_BANDWIDTH))]
unsafe fn task_is_throttled(_: *mut b::task_struct) -> bool {
    false
}
#[cfg(not(CONFIG_CFS_BANDWIDTH))]
unsafe fn dequeue_throttled_task(_: *mut b::task_struct, _: c_int) {}
#[cfg(not(CONFIG_CFS_BANDWIDTH))]
unsafe fn enqueue_throttled_task(_: *mut b::task_struct) -> bool {
    false
}
#[cfg(not(CONFIG_CFS_BANDWIDTH))]
unsafe fn record_throttle_clock(_: *mut b::cfs_rq) {}
#[cfg(not(CONFIG_CFS_BANDWIDTH))]
unsafe fn cfs_rq_throttled(_: *mut b::cfs_rq) -> c_int {
    0
}
#[cfg(not(CONFIG_CFS_BANDWIDTH))]
unsafe fn cfs_rq_pelt_clock_throttled(_: *mut b::cfs_rq) -> bool {
    false
}
#[cfg(not(CONFIG_CFS_BANDWIDTH))]
unsafe fn throttled_hierarchy(_: *mut b::cfs_rq) -> c_int {
    0
}
#[cfg(not(CONFIG_CFS_BANDWIDTH))]
unsafe fn lb_throttled_hierarchy(_: *mut b::task_struct, _: c_int) -> c_int {
    0
}
#[cfg(all(not(CONFIG_CFS_BANDWIDTH), CONFIG_FAIR_GROUP_SCHED))]
#[no_mangle]
pub unsafe extern "C" fn init_cfs_bandwidth(_: *mut b::cfs_bandwidth, _: *mut b::cfs_bandwidth) {}
#[cfg(all(not(CONFIG_CFS_BANDWIDTH), CONFIG_FAIR_GROUP_SCHED))]
unsafe fn init_cfs_rq_runtime(_: *mut b::cfs_rq) {}
#[cfg(not(CONFIG_CFS_BANDWIDTH))]
unsafe fn tg_cfs_bandwidth(_: *mut b::task_group) -> *mut b::cfs_bandwidth {
    null_mut()
}
#[cfg(not(CONFIG_CFS_BANDWIDTH))]
unsafe fn destroy_cfs_bandwidth(_: *mut b::cfs_bandwidth) {}
#[cfg(not(CONFIG_CFS_BANDWIDTH))]
unsafe fn update_runtime_enabled(_: *mut b::rq) {}
#[cfg(not(CONFIG_CFS_BANDWIDTH))]
unsafe fn unthrottle_offline_cfs_rqs(_: *mut b::rq) {}
#[cfg(all(not(CONFIG_CFS_BANDWIDTH), CONFIG_CGROUP_SCHED))]
#[no_mangle]
pub unsafe extern "C" fn cfs_task_bw_constrained(_: *mut b::task_struct) -> bool {
    false
}
#[cfg(any(not(CONFIG_CFS_BANDWIDTH), not(CONFIG_NO_HZ_FULL)))]
unsafe fn sched_fair_update_stop_tick(_: *mut b::rq, _: *mut b::task_struct) {}

#[cfg(CONFIG_NO_HZ_COMMON)]
unsafe fn migrate_se_pelt_lag(se: *mut b::sched_entity) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if load_avg_is_decayed(addr_of_mut!((*se).avg)) {
            return;
        }
        let cfs = b::rust_fair_cfs_rq_of(se);
        let rq = b::rust_fair_rq_of(cfs);
        b::rust_fair_rcu_read_lock();
        let idle = b::rust_fair_is_idle_task(b::rust_fair_rq_curr_rcu(rq));
        b::rust_fair_rcu_read_unlock();
        if !idle {
            return;
        }
        let mut throttled = 0;
        #[cfg(CONFIG_CFS_BANDWIDTH)]
        {
            throttled = b::rust_fair_load_throttled_pelt_idle(cfs);
            if throttled == u64::MAX {
                return;
            }
        }
        let mut now = b::rust_fair_load_clock_pelt_idle(rq);
        b::rust_fair_smp_rmb();
        let lut = cfs_rq_last_update_time(cfs);
        now = now.wrapping_sub(throttled);
        if now < lut {
            now = lut;
        } else {
            now = now.wrapping_add(
                b::sched_clock_cpu(b::rust_fair_cpu_of(rq))
                    .wrapping_sub(b::rust_fair_load_clock_idle(rq)),
            );
        }
        b::__update_load_avg_blocked_se(now, se);
    }
}
#[cfg(not(CONFIG_NO_HZ_COMMON))]
unsafe fn migrate_se_pelt_lag(_: *mut b::sched_entity) {}
#[cfg(CONFIG_SCHED_CACHE)]
#[no_mangle]
#[link_section = ".data..read_mostly"]
pub static mut llc_aggr_tolerance: c_uint = 1;
#[cfg(CONFIG_SCHED_CACHE)]
#[no_mangle]
#[link_section = ".data..read_mostly"]
pub static mut llc_epoch_period: c_uint = b::HZ / 100;
#[cfg(CONFIG_SCHED_CACHE)]
#[no_mangle]
#[link_section = ".data..read_mostly"]
pub static mut llc_epoch_affinity_timeout: c_uint = 5;
#[cfg(CONFIG_SCHED_CACHE)]
#[no_mangle]
#[link_section = ".data..read_mostly"]
pub static mut llc_imb_pct: c_uint = 20;
#[cfg(CONFIG_SCHED_CACHE)]
#[no_mangle]
#[link_section = ".data..read_mostly"]
pub static mut llc_overaggr_pct: c_uint = 50;
#[cfg(CONFIG_SCHED_CACHE)]
unsafe fn llc_id(cpu: c_int) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if cpu < 0 {
            -1
        } else {
            b::rust_fair_sd_llc_id(cpu)
        }
    }
}
#[cfg(CONFIG_SCHED_CACHE)]
unsafe fn get_sched_cache_scale(mul: c_int) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let tol = read_once!(llc_aggr_tolerance);
        if tol == 0 {
            return 0;
        }
        if tol >= 100 {
            return c_int::MAX;
        }
        1u32.wrapping_add(tol.wrapping_sub(1).wrapping_mul(mul as c_uint)) as c_int
    }
}
#[cfg(CONFIG_SCHED_CACHE)]
unsafe fn exceed_llc_capacity(mm: *mut b::mm_struct, cpu: c_int) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        #[cfg(CONFIG_NUMA_BALANCING)]
        {
            b::rust_fair_rcu_read_lock();
            let sd = b::rust_fair_rcu_sched_domain_rq_sd(b::rust_fair_cpu_rq(cpu));
            if sd.is_null() {
                b::rust_fair_rcu_read_unlock();
                return true;
            }
            if b::rust_fair_numa_balancing_enabled() {
                let llc = (*sd).llc_bytes;
                let footprint = read_once!((*mm).sc_stat.footprint);
                let scale = get_sched_cache_scale(256);
                let exceeds = scale != c_int::MAX
                    && (llc as u64).wrapping_mul(scale as u64)
                        < footprint.wrapping_mul(b::RUST_FAIR_PAGE_SIZE) as u64;
                b::rust_fair_rcu_read_unlock();
                return exceeds;
            }
            b::rust_fair_rcu_read_unlock();
        }
        false
    }
}
#[cfg(CONFIG_SCHED_CACHE)]
unsafe fn invalid_llc_nr(mm: *mut b::mm_struct, p: *mut b::task_struct, cpu: c_int) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if b::rust_fair_get_nr_threads(p) <= 1 {
            return true;
        }
        let scale = get_sched_cache_scale(1);
        if scale == c_int::MAX {
            return false;
        }
        // The original macro's lhs is u64; do not narrow it to unsigned long on 32b.
        let lhs = (*mm)
            .sc_stat
            .nr_running_avg
            .wrapping_mul(b::cpu_smt_num_threads as u64)
            .wrapping_mul(1280);
        let rhs = scale
            .wrapping_mul(b::rust_fair_sd_llc_size(cpu))
            .wrapping_mul(1024) as u64;
        !(lhs < rhs)
    }
}
#[cfg(CONFIG_SCHED_CACHE)]
unsafe fn account_llc_enqueue(rq: *mut b::rq, p: *mut b::task_struct) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let pref = (*p).preferred_llc;
        if pref < 0 {
            return;
        }
        let queued = pref == b::rust_fair_task_llc(p);
        (*rq).nr_llc_running = (*rq).nr_llc_running.wrapping_add(1);
        (*rq).nr_pref_llc_running = (*rq).nr_pref_llc_running.wrapping_add(queued as _);
        (*p).pref_llc_queued = queued as _;
        let sd = b::rust_fair_rq_sd_rcu(rq);
        if !sd.is_null() && (pref as c_uint) < (*sd).llc_max {
            let count = (*sd).llc_counts.add(pref as usize);
            *count = (*count).wrapping_add(1);
        }
    }
}
#[cfg(CONFIG_SCHED_CACHE)]
unsafe fn account_llc_dequeue(rq: *mut b::rq, p: *mut b::task_struct) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let pref = (*p).preferred_llc;
        if pref < 0 {
            return;
        }
        (*rq).nr_llc_running = (*rq).nr_llc_running.wrapping_sub(1);
        if (*p).pref_llc_queued != 0 {
            (*rq).nr_pref_llc_running = (*rq).nr_pref_llc_running.wrapping_sub(1);
            (*p).pref_llc_queued = 0;
        }
        let sd = b::rust_fair_rq_sd_rcu(rq);
        if !sd.is_null() && (pref as c_uint) < (*sd).llc_max {
            let count = (*sd).llc_counts.add(pref as usize);
            if *count != 0 {
                *count -= 1;
            }
        }
    }
}
#[cfg(CONFIG_SCHED_CACHE)]
#[no_mangle]
pub unsafe extern "C" fn mm_init_sched(mm: *mut b::mm_struct, pcpu: *mut b::sched_cache_time) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut epoch = 0;
        let mask = b::rust_fair_cpu_possible_mask();
        let mut cpu = b::rust_fair_cpumask_first(mask);
        while cpu < b::rust_fair_small_cpumask_bits() {
            let local = b::rust_fair_per_cpu_cache_time(pcpu, cpu as c_int);
            let rq = b::rust_fair_cpu_rq(cpu as c_int);
            (*local).runtime = 0;
            (*local).epoch = (*rq).cpu_epoch;
            epoch = (*rq).cpu_epoch;
            cpu = b::rust_fair_cpumask_next(cpu as c_int, mask);
        }
        b::rust_fair_init_mm_sched_lock(addr_of_mut!((*mm).sc_stat.lock));
        (*mm).sc_stat.epoch = epoch;
        (*mm).sc_stat.cpu = -1;
        (*mm).sc_stat.next_scan = b::rust_fair_jiffies();
        (*mm).sc_stat.nr_running_avg = 0;
        (*mm).sc_stat.footprint = 0;
        b::rust_fair_store_release_cache_time(addr_of_mut!((*mm).sc_stat.pcpu_sched), pcpu);
    }
}
#[cfg(CONFIG_SCHED_CACHE)]
unsafe fn __shr_u64(value: *mut u64, shift: c_uint) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        *value = if shift >= 64 { 0 } else { *value >> shift };
    }
}
#[cfg(CONFIG_SCHED_CACHE)]
unsafe fn __update_mm_sched(rq: *mut b::rq, pcpu: *mut b::sched_cache_time) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_lockdep_assert_raw_spin_held(addr_of_mut!((*rq).cpu_epoch_lock));
        let period = max(read_once!(llc_epoch_period), 1);
        let now = b::rust_fair_jiffies();
        let delta = now.wrapping_sub((*rq).cpu_epoch_next) as c_long;
        if delta > 0 {
            let n = delta.wrapping_add(period as c_long).wrapping_sub(1) / period as c_long;
            (*rq).cpu_epoch = (*rq).cpu_epoch.wrapping_add(n as c_ulong);
            (*rq).cpu_epoch_next = (*rq)
                .cpu_epoch_next
                .wrapping_add((n as c_ulong).wrapping_mul(period as c_ulong));
            __shr_u64(addr_of_mut!((*rq).cpu_runtime), n as c_uint);
        }
        let n = (*rq).cpu_epoch.wrapping_sub((*pcpu).epoch);
        if n != 0 {
            (*pcpu).epoch = (*pcpu).epoch.wrapping_add(n);
            __shr_u64(addr_of_mut!((*pcpu).runtime), n as c_uint);
        }
    }
}
#[cfg(CONFIG_SCHED_CACHE)]
unsafe fn fraction_mm_sched(rq: *mut b::rq, pcpu: *mut b::sched_cache_time) -> c_ulong {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let flags = b::rust_fair_raw_spin_lock_irqsave(addr_of_mut!((*rq).cpu_epoch_lock));
        __update_mm_sched(rq, pcpu);
        let result = (b::RUST_FAIR_NICE_0_LOAD as u64).wrapping_mul((*pcpu).runtime)
            / (*rq).cpu_runtime.wrapping_add(1);
        b::rust_fair_raw_spin_unlock_irqrestore(addr_of_mut!((*rq).cpu_epoch_lock), flags);
        result as c_ulong
    }
}
#[cfg(CONFIG_SCHED_CACHE)]
unsafe fn get_pref_llc(p: *mut b::task_struct, mm: *mut b::mm_struct) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if mm.is_null() {
            return -1;
        }
        let cpu = read_once!((*mm).sc_stat.cpu);
        let mut llc = -1;
        if cpu != -1 {
            llc = llc_id(cpu);
            #[cfg(CONFIG_NUMA_BALANCING)]
            if b::rust_fair_numa_balancing_enabled()
                && (*p).numa_preferred_nid >= 0
                && b::rust_fair_cpu_to_node(cpu) != (*p).numa_preferred_nid
            {
                llc = -1;
            }
        }
        llc
    }
}
#[cfg(CONFIG_SCHED_CACHE)]
unsafe fn account_mm_sched(rq: *mut b::rq, p: *mut b::task_struct, delta: i64) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mm = (*p).mm;
        if !b::rust_fair_sched_cache_enabled()
            || (*p).sched_class != addr_of!(fair_sched_class)
            || mm.is_null()
            || (*mm).sc_stat.pcpu_sched.is_null()
        {
            return;
        }
        let pcpu = b::rust_fair_per_cpu_cache_time((*mm).sc_stat.pcpu_sched, b::rust_fair_cpu_of(rq));
        b::rust_fair_raw_spin_lock(addr_of_mut!((*rq).cpu_epoch_lock));
        __update_mm_sched(rq, pcpu);
        (*pcpu).runtime = (*pcpu).runtime.wrapping_add(delta as u64);
        (*rq).cpu_runtime = (*rq).cpu_runtime.wrapping_add(delta as u64);
        let epoch = (*rq).cpu_epoch;
        b::rust_fair_raw_spin_unlock(addr_of_mut!((*rq).cpu_epoch_lock));
        if epoch.wrapping_sub(read_once!((*mm).sc_stat.epoch)) as c_long
            > llc_epoch_affinity_timeout as c_long
            || invalid_llc_nr(mm, p, b::rust_fair_cpu_of(rq))
            || exceed_llc_capacity(mm, b::rust_fair_cpu_of(rq))
        {
            if read_once!((*mm).sc_stat.cpu) != -1 {
                write_once!((*mm).sc_stat.cpu, -1);
            }
        }
        let pref = get_pref_llc(p, mm);
        if task_running_on_cpu((*rq).cpu, p) != 0 && read_once!((*p).preferred_llc) != pref {
            account_llc_dequeue(rq, p);
            write_once!((*p).preferred_llc, pref);
            account_llc_enqueue(rq, p);
        }
    }
}
#[cfg(CONFIG_SCHED_CACHE)]
unsafe fn task_tick_cache(rq: *mut b::rq, p: *mut b::task_struct) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let work = addr_of_mut!((*p).cache_work);
        let mm = (*p).mm;
        if !b::rust_fair_sched_cache_enabled()
            || mm.is_null()
            || (*p).flags & b::PF_KTHREAD != 0
            || (*mm).sc_stat.pcpu_sched.is_null()
        {
            return;
        }
        let epoch = (*rq).cpu_epoch;
        if ((*mm).sc_stat.epoch.wrapping_sub(epoch) as c_long) >= 0 {
            return;
        }
        b::rust_fair_raw_spin_lock(addr_of_mut!((*mm).sc_stat.lock));
        if (*work).next == work {
            b::task_work_add(p, work, b::TWA_RESUME);
            write_once!((*mm).sc_stat.epoch, epoch);
        }
        b::rust_fair_raw_spin_unlock(addr_of_mut!((*mm).sc_stat.lock));
    }
}
#[cfg(CONFIG_SCHED_CACHE)]
unsafe fn get_scan_cpumasks(cpus: *mut b::cpumask, p: *mut b::task_struct) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        #[cfg(CONFIG_NUMA_BALANCING)]
        if b::rust_fair_numa_balancing_enabled() {
            let cpu = read_once!((*(*p).mm).sc_stat.cpu);
            let current = b::rust_fair_task_cpu(p);
            let preferred = (*p).numa_preferred_nid;
            if preferred != b::NUMA_NO_NODE {
                b::rust_fair_cpumask_or(cpus, cpus, b::rust_fair_cpumask_of_node(preferred));
                if cpu != -1 && !b::rust_fair_cpumask_test_cpu(cpu, cpus) {
                    let nid = b::rust_fair_cpu_to_node(cpu);
                    if nid != b::NUMA_NO_NODE {
                        b::rust_fair_cpumask_or(cpus, cpus, b::rust_fair_cpumask_of_node(nid));
                    }
                }
                if !b::rust_fair_cpumask_test_cpu(current, cpus) {
                    b::rust_fair_cpumask_or(
                        cpus,
                        cpus,
                        b::rust_fair_cpumask_of_node(b::rust_fair_cpu_to_node(current)),
                    );
                }
                return;
            }
        }
        b::rust_fair_cpumask_copy(cpus, b::rust_fair_cpu_online_mask());
    }
}
#[cfg(CONFIG_SCHED_CACHE)]
unsafe fn update_avg_scale(avg: *mut u64, sample: u64) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let factor = b::rust_fair_sd_llc_size(b::rust_fair_raw_smp_processor_id());
        let diff = sample.wrapping_sub(*avg) as i64;
        let divisor = ((factor >> 2) as u32).clamp(2, 8);
        *avg = (*avg).wrapping_add((diff / divisor as i64) as u64);
    }
}
#[cfg(CONFIG_SCHED_CACHE)]
unsafe extern "C" fn task_cache_work(work: *mut b::callback_head) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let p = b::rust_fair_current();
        let mm = (*p).mm;
        let now = b::rust_fair_jiffies();
        b::rust_fair_warn_task_cache_work_1(work != addr_of_mut!((*p).cache_work));
        (*work).next = work;
        if (*p).flags & b::PF_EXITING != 0 {
            return;
        }
        let mut next = read_once!((*mm).sc_stat.next_scan);
        if (now.wrapping_sub(next) as c_long) < 0 {
            return;
        }
        if !b::rust_fair_try_cmpxchg_ulong(
            addr_of_mut!((*mm).sc_stat.next_scan),
            addr_of_mut!(next),
            now.wrapping_add(max(read_once!(llc_epoch_period) as c_ulong, 1)),
        ) {
            return;
        }
        let current = b::rust_fair_task_cpu(p);
        if invalid_llc_nr(mm, p, current) || exceed_llc_capacity(mm, current) {
            if read_once!((*mm).sc_stat.cpu) != -1 {
                write_once!((*mm).sc_stat.cpu, -1);
            }
            return;
        }
        let mut cpus: b::cpumask_var_t = core::mem::zeroed();
        if !b::rust_fair_zalloc_cpumask_var(addr_of_mut!(cpus), b::RUST_FAIR_GFP_KERNEL) {
            return;
        }
        let mask = b::rust_fair_cpumask_var_ptr(addr_of_mut!(cpus));
        let mut max_cpu = -1;
        let mut running: c_int = 0;
        let mut curr_occ: c_ulong = 0;
        let mut max_occ: c_ulong = 0;
        b::rust_fair_cpus_read_lock();
        b::rust_fair_rcu_read_lock();
        get_scan_cpumasks(mask, p);
        let mut cpu = b::rust_fair_cpumask_first(mask);
        while cpu < b::rust_fair_small_cpumask_bits() {
            let sd = b::rust_fair_sd_llc_rcu(cpu as c_int);
            if !sd.is_null() {
                let span = b::rust_fair_sched_domain_span(sd);
                let mut local_max = 0;
                let mut sum: c_ulong = 0;
                let mut local_cpu = -1;
                let mut i = b::rust_fair_cpumask_first(span);
                while i < b::rust_fair_small_cpumask_bits() {
                    let rq = b::rust_fair_cpu_rq(i as c_int);
                    let occ = fraction_mm_sched(
                        rq,
                        b::rust_fair_per_cpu_cache_time((*mm).sc_stat.pcpu_sched, i as c_int),
                    );
                    sum = sum.wrapping_add(occ);
                    if occ > local_max {
                        local_max = occ;
                        local_cpu = i as c_int;
                    }
                    let cur = b::rust_fair_rq_curr_rcu(rq);
                    if !cur.is_null()
                        && (*cur).flags & (b::PF_EXITING | b::PF_KTHREAD) == 0
                        && (*cur).mm == mm
                    {
                        running = running.wrapping_add(1);
                    }
                    i = b::rust_fair_cpumask_next(i as c_int, span);
                }
                if sum > max_occ {
                    max_occ = sum;
                    max_cpu = local_cpu;
                }
                if llc_id(cpu as c_int) == llc_id(read_once!((*mm).sc_stat.cpu)) {
                    curr_occ = sum;
                }
                b::rust_fair_cpumask_andnot(mask, mask, span);
            }
            cpu = b::rust_fair_cpumask_next(cpu as c_int, mask);
        }
        b::rust_fair_rcu_read_unlock();
        b::rust_fair_cpus_read_unlock();
        if max_occ > curr_occ.wrapping_mul(2) {
            write_once!((*mm).sc_stat.cpu, max_cpu);
        }
        update_avg_scale(addr_of_mut!((*mm).sc_stat.nr_running_avg), running as u64);
        b::rust_fair_free_cpumask_var(addr_of_mut!(cpus));
    }
}
#[cfg(CONFIG_SCHED_CACHE)]
#[no_mangle]
pub unsafe extern "C" fn init_sched_mm(p: *mut b::task_struct) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let work = addr_of_mut!((*p).cache_work);
        b::rust_fair_init_task_work(work, Some(task_cache_work));
        (*work).next = work;
        (*p).preferred_llc = -1;
    }
}
#[cfg(not(CONFIG_SCHED_CACHE))]
unsafe fn account_mm_sched(_: *mut b::rq, _: *mut b::task_struct, _: i64) {}
#[cfg(not(CONFIG_SCHED_CACHE))]
#[no_mangle]
pub unsafe extern "C" fn init_sched_mm(_: *mut b::task_struct) {}
#[cfg(not(CONFIG_SCHED_CACHE))]
unsafe fn task_tick_cache(_: *mut b::rq, _: *mut b::task_struct) {}
#[cfg(not(CONFIG_SCHED_CACHE))]
unsafe fn get_pref_llc(_: *mut b::task_struct, _: *mut b::mm_struct) -> c_int {
    -1
}
#[cfg(not(CONFIG_SCHED_CACHE))]
unsafe fn account_llc_enqueue(_: *mut b::rq, _: *mut b::task_struct) {}
#[cfg(not(CONFIG_SCHED_CACHE))]
unsafe fn account_llc_dequeue(_: *mut b::rq, _: *mut b::task_struct) {}

#[cfg(CONFIG_NUMA_BALANCING)]
#[no_mangle]
pub static mut sysctl_numa_balancing_scan_period_min: c_uint = 1000;
#[cfg(CONFIG_NUMA_BALANCING)]
#[no_mangle]
pub static mut sysctl_numa_balancing_scan_period_max: c_uint = 60000;
#[cfg(CONFIG_NUMA_BALANCING)]
#[no_mangle]
pub static mut sysctl_numa_balancing_scan_size: c_uint = 256;
#[cfg(CONFIG_NUMA_BALANCING)]
#[no_mangle]
pub static mut sysctl_numa_balancing_scan_delay: c_uint = 1000;
#[cfg(CONFIG_NUMA_BALANCING)]
#[no_mangle]
pub static mut sysctl_numa_balancing_hot_threshold: c_uint = b::MSEC_PER_SEC;
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn deref_task_numa_group(p: *mut b::task_struct) -> *mut b::numa_group {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_deref_task_numa_group(p)
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn deref_curr_numa_group(p: *mut b::task_struct) -> *mut b::numa_group {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_deref_curr_numa_group(p)
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn task_nr_scan_windows(p: *mut b::task_struct) -> c_uint {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let pages = b::rust_fair_mb_to_pages(sysctl_numa_balancing_scan_size as c_ulong);
        let mut rss = b::rust_fair_get_mm_rss((*p).mm);
        if rss == 0 {
            rss = pages;
        }
        // Preserve include/linux/math.h round_up, including its source expression.
        rss = (rss.wrapping_sub(1) | pages.wrapping_sub(1)).wrapping_add(1);
        (rss / pages) as c_uint
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn task_scan_min(p: *mut b::task_struct) -> c_uint {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let size = read_once!(sysctl_numa_balancing_scan_size);
        let windows = if size < 2560 { 2560 / size } else { 1 };
        max(
            1000 / windows,
            sysctl_numa_balancing_scan_period_min / task_nr_scan_windows(p),
        )
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn task_scan_start(p: *mut b::task_struct) -> c_uint {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let smin = task_scan_min(p) as c_ulong;
        let mut period = smin;
        b::rust_fair_rcu_read_lock();
        let ng = b::rust_fair_numa_group_rcu(p);
        if !ng.is_null() {
            let shared = group_faults_shared(ng);
            let private = group_faults_priv(ng);
            period = period
                .wrapping_mul(b::rust_fair_refcount_read(addr_of!((*ng).refcount)) as c_ulong)
                .wrapping_mul(shared.wrapping_add(1))
                / private.wrapping_add(shared).wrapping_add(1);
        }
        b::rust_fair_rcu_read_unlock();
        max(smin, period) as c_uint
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn task_scan_max(p: *mut b::task_struct) -> c_uint {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let smin = task_scan_min(p) as c_ulong;
        let mut smax = (sysctl_numa_balancing_scan_period_max / task_nr_scan_windows(p)) as c_ulong;
        let ng = deref_curr_numa_group(p);
        if !ng.is_null() {
            let shared = group_faults_shared(ng);
            let private = group_faults_priv(ng);
            let period = smax
                .wrapping_mul(b::rust_fair_refcount_read(addr_of!((*ng).refcount)) as c_ulong)
                .wrapping_mul(shared.wrapping_add(1))
                / private.wrapping_add(shared).wrapping_add(1);
            smax = max(smax, period);
        }
        max(smin, smax) as c_uint
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn account_numa_enqueue(rq: *mut b::rq, p: *mut b::task_struct) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (*rq).nr_numa_running = (*rq)
            .nr_numa_running
            .wrapping_add(((*p).numa_preferred_nid != b::NUMA_NO_NODE) as _);
        (*rq).nr_preferred_running = (*rq)
            .nr_preferred_running
            .wrapping_add(((*p).numa_preferred_nid == b::rust_fair_task_node(p)) as _);
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn account_numa_dequeue(rq: *mut b::rq, p: *mut b::task_struct) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (*rq).nr_numa_running = (*rq)
            .nr_numa_running
            .wrapping_sub(((*p).numa_preferred_nid != b::NUMA_NO_NODE) as _);
        (*rq).nr_preferred_running = (*rq)
            .nr_preferred_running
            .wrapping_sub(((*p).numa_preferred_nid == b::rust_fair_task_node(p)) as _);
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
#[no_mangle]
pub unsafe extern "C" fn task_numa_group_id(p: *mut b::task_struct) -> b::pid_t {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_rcu_read_lock();
        let ng = b::rust_fair_numa_group_rcu(p);
        let gid = if ng.is_null() { 0 } else { (*ng).gid };
        b::rust_fair_rcu_read_unlock();
        gid
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn task_faults_idx(s: b::numa_faults_stats, nid: c_int, private: c_int) -> c_int {
    2i32.wrapping_mul(
        (s as c_int)
            .wrapping_mul(b::nr_node_ids as c_int)
            .wrapping_add(nid),
    )
    .wrapping_add(private)
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn task_faults(p: *mut b::task_struct, nid: c_int) -> c_ulong {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if (*p).numa_faults.is_null() {
            return 0;
        }
        (*(*p)
            .numa_faults
            .offset(task_faults_idx(b::NUMA_MEM, nid, 0) as isize))
        .wrapping_add(
            *(*p)
                .numa_faults
                .offset(task_faults_idx(b::NUMA_MEM, nid, 1) as isize),
        )
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn group_fault_ptr(
    ng: *mut b::numa_group,
    s: b::numa_faults_stats,
    nid: c_int,
    private: c_int,
) -> *mut c_ulong {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (*ng)
            .faults
            .as_mut_ptr()
            .offset(task_faults_idx(s, nid, private) as isize)
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn group_faults(p: *mut b::task_struct, nid: c_int) -> c_ulong {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let ng = deref_task_numa_group(p);
        if ng.is_null() {
            return 0;
        }
        (*group_fault_ptr(ng, b::NUMA_MEM, nid, 0)).wrapping_add(*group_fault_ptr(
            ng,
            b::NUMA_MEM,
            nid,
            1,
        ))
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn group_faults_cpu(ng: *mut b::numa_group, nid: c_int) -> c_ulong {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (*group_fault_ptr(ng, b::NUMA_CPU, nid, 0)).wrapping_add(*group_fault_ptr(
            ng,
            b::NUMA_CPU,
            nid,
            1,
        ))
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn group_faults_priv(ng: *mut b::numa_group) -> c_ulong {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut total: c_ulong = 0;
        let mut node = b::rust_fair_first_online_node();
        while node < b::MAX_NUMNODES as c_int {
            total = total.wrapping_add(*group_fault_ptr(ng, b::NUMA_MEM, node, 1));
            node = b::rust_fair_next_online_node(node);
        }
        total
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn group_faults_shared(ng: *mut b::numa_group) -> c_ulong {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut total: c_ulong = 0;
        let mut node = b::rust_fair_first_online_node();
        while node < b::MAX_NUMNODES as c_int {
            total = total.wrapping_add(*group_fault_ptr(ng, b::NUMA_MEM, node, 0));
            node = b::rust_fair_next_online_node(node);
        }
        total
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn numa_is_active_node(nid: c_int, ng: *mut b::numa_group) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        group_faults_cpu(ng, nid).wrapping_mul(3) > (*ng).max_faults_cpu
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn score_nearby_nodes(
    p: *mut b::task_struct,
    nid: c_int,
    lim: c_int,
    task: bool,
) -> c_ulong {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if b::sched_numa_topology_type == b::NUMA_DIRECT {
            return 0;
        }
        let maximum = read_once!(b::sched_max_numa_distance);
        let mut score: c_ulong = 0;
        let mut node = b::rust_fair_first_online_node();
        while node < b::MAX_NUMNODES as c_int {
            let dist = b::rust_fair_node_distance(nid, node);
            if dist < maximum
                && node != nid
                && !(b::sched_numa_topology_type == b::NUMA_BACKPLANE && dist >= lim)
            {
                let mut faults = if task {
                    task_faults(p, node)
                } else {
                    group_faults(p, node)
                };
                if b::sched_numa_topology_type == b::NUMA_GLUELESS_MESH {
                    faults = faults.wrapping_mul(maximum.wrapping_sub(dist) as c_ulong)
                        / maximum.wrapping_sub(b::LOCAL_DISTANCE as c_int) as c_ulong;
                }
                score = score.wrapping_add(faults);
            }
            node = b::rust_fair_next_online_node(node);
        }
        score
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn task_weight(p: *mut b::task_struct, nid: c_int, dist: c_int) -> c_ulong {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if (*p).numa_faults.is_null() || (*p).total_numa_faults == 0 {
            return 0;
        }
        task_faults(p, nid)
            .wrapping_add(score_nearby_nodes(p, nid, dist, true))
            .wrapping_mul(1000)
            / (*p).total_numa_faults
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn group_weight(p: *mut b::task_struct, nid: c_int, dist: c_int) -> c_ulong {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let ng = deref_task_numa_group(p);
        if ng.is_null() || (*ng).total_faults == 0 {
            return 0;
        }
        group_faults(p, nid)
            .wrapping_add(score_nearby_nodes(p, nid, dist, false))
            .wrapping_mul(1000)
            / (*ng).total_faults
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn cpupid_valid(cpupid: c_int) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_cpupid_to_cpu(cpupid) < b::nr_cpu_ids as c_int
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn pgdat_free_space_enough(pgdat: *mut b::pglist_data) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let watermark = max(
            (1024 as c_ulong * 1024 * 1024) >> b::PAGE_SHIFT,
            (*pgdat).node_present_pages >> 4,
        );
        let mut z = (*pgdat).nr_zones - 1;
        while z >= 0 {
            let zone = (*pgdat).node_zones.as_mut_ptr().offset(z as isize);
            if b::rust_fair_populated_zone(zone)
                && b::zone_watermark_ok(
                    zone,
                    0,
                    b::rust_fair_promo_wmark_pages(zone).wrapping_add(watermark),
                    b::ZONE_MOVABLE as c_int,
                    0,
                )
            {
                return true;
            }
            z -= 1;
        }
        false
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn numa_hint_fault_latency(folio: *mut b::folio) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let time = b::rust_fair_jiffies_to_msecs(b::rust_fair_jiffies()) as c_int;
        let previous = b::rust_fair_folio_xchg_access_time(folio, time);
        time.wrapping_sub(previous) & b::RUST_FAIR_PAGE_ACCESS_TIME_MASK as c_int
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn numa_promotion_rate_limit(pgdat: *mut b::pglist_data, limit: c_ulong, nr: c_int) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let now = b::rust_fair_jiffies_to_msecs(b::rust_fair_jiffies());
        b::mod_node_page_state(pgdat, b::PGPROMOTE_CANDIDATE, nr as c_long);
        let candidates = b::rust_fair_node_page_state(pgdat, b::PGPROMOTE_CANDIDATE);
        let start = (*pgdat).nbp_rl_start;
        if now.wrapping_sub(start) > b::MSEC_PER_SEC
            && b::rust_fair_cmpxchg_uint(addr_of_mut!((*pgdat).nbp_rl_start), start, now) == start
        {
            (*pgdat).nbp_rl_nr_cand = candidates;
        }
        candidates.wrapping_sub((*pgdat).nbp_rl_nr_cand) >= limit
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn numa_promotion_adjust_threshold(
    pgdat: *mut b::pglist_data,
    limit: c_ulong,
    reference: c_uint,
) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let now = b::rust_fair_jiffies_to_msecs(b::rust_fair_jiffies());
        let start = (*pgdat).nbp_th_start;
        if now.wrapping_sub(start) > sysctl_numa_balancing_scan_period_max
            && b::rust_fair_cmpxchg_uint(addr_of_mut!((*pgdat).nbp_th_start), start, now) == start
        {
            let reference_count = limit.wrapping_mul(sysctl_numa_balancing_scan_period_max as c_ulong)
                / b::MSEC_PER_SEC as c_ulong;
            let count = b::rust_fair_node_page_state(pgdat, b::PGPROMOTE_CANDIDATE);
            let diff = count.wrapping_sub((*pgdat).nbp_th_nr_cand);
            let unit = reference.wrapping_mul(2) / 16;
            let mut threshold = if (*pgdat).nbp_threshold != 0 {
                (*pgdat).nbp_threshold
            } else {
                reference
            };
            if diff > reference_count.wrapping_mul(11) / 10 {
                threshold = max(threshold.wrapping_sub(unit), unit);
            } else if diff < reference_count.wrapping_mul(9) / 10 {
                threshold = min(threshold.wrapping_add(unit), reference.wrapping_mul(2));
            }
            (*pgdat).nbp_th_nr_cand = count;
            (*pgdat).nbp_threshold = threshold;
        }
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
#[no_mangle]
pub unsafe extern "C" fn should_numa_migrate_memory(
    p: *mut b::task_struct,
    folio: *mut b::folio,
    src: c_int,
    cpu: c_int,
) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let ng = deref_curr_numa_group(p);
        let dst = b::rust_fair_cpu_to_node(cpu);
        if !b::rust_fair_node_state(dst, b::N_MEMORY) {
            return false;
        }
        if b::rust_fair_folio_use_access_time(folio) {
            let nr = b::rust_fair_folio_nr_pages(folio) as c_long;
            let pgdat = b::rust_fair_node_data(dst);
            if pgdat_free_space_enough(pgdat) {
                (*pgdat).nbp_threshold = 0;
                b::mod_node_page_state(pgdat, b::PGPROMOTE_CANDIDATE_NRL, nr);
                return true;
            }
            let default = sysctl_numa_balancing_hot_threshold;
            let limit = b::rust_fair_mb_to_pages(sysctl_numa_balancing_promote_rate_limit as c_ulong);
            numa_promotion_adjust_threshold(pgdat, limit, default);
            let threshold = if (*pgdat).nbp_threshold != 0 {
                (*pgdat).nbp_threshold
            } else {
                default
            };
            if numa_hint_fault_latency(folio) as c_uint >= threshold {
                return false;
            }
            return !numa_promotion_rate_limit(pgdat, limit, nr as c_int);
        }
        let this = b::rust_fair_cpu_pid_to_cpupid(cpu, (*b::rust_fair_current()).pid);
        let last = b::rust_fair_folio_xchg_last_cpupid(folio, this);
        if b::sysctl_numa_balancing_mode & b::NUMA_BALANCING_MEMORY_TIERING == 0
            && !b::rust_fair_node_is_toptier(src)
            && !cpupid_valid(last)
        {
            return false;
        }
        if ((*p).numa_preferred_nid == b::NUMA_NO_NODE || (*p).numa_scan_seq <= 4)
            && (b::rust_fair_cpupid_pid_unset(last) || b::rust_fair_cpupid_match_pid(p, last))
        {
            return true;
        }
        if !b::rust_fair_cpupid_pid_unset(last) && b::rust_fair_cpupid_to_nid(last) != dst {
            return false;
        }
        if b::rust_fair_cpupid_match_pid(p, last) || ng.is_null() {
            return true;
        }
        if group_faults_cpu(ng, dst) > group_faults_cpu(ng, src).wrapping_mul(3) {
            return true;
        }
        group_faults_cpu(ng, dst)
            .wrapping_mul(group_faults(p, src))
            .wrapping_mul(3)
            > group_faults_cpu(ng, src)
                .wrapping_mul(group_faults(p, dst))
                .wrapping_mul(4)
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn numa_classify(imbalance: c_uint, ns: *mut b::numa_stats) -> b::numa_type {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let cap = (*ns).compute_capacity;
        if (*ns).nr_running > (*ns).weight
            && (cap.wrapping_mul(100) < (*ns).util.wrapping_mul(imbalance as c_ulong)
                || cap.wrapping_mul(imbalance as c_ulong) < (*ns).runnable.wrapping_mul(100))
        {
            return b::node_overloaded;
        }
        if (*ns).nr_running < (*ns).weight
            || (cap.wrapping_mul(100) > (*ns).util.wrapping_mul(imbalance as c_ulong)
                && cap.wrapping_mul(imbalance as c_ulong) > (*ns).runnable.wrapping_mul(100))
        {
            return b::node_has_spare;
        }
        b::node_fully_busy
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn numa_idle_core(mut idle: c_int, cpu: c_int) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if !b::rust_fair_sched_smt_active() || idle >= 0 || !test_idle_cores(cpu) {
            return idle;
        }
        if is_core_idle(cpu) {
            idle = cpu;
        }
        idle
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn update_numa_stats(
    env: *mut b::task_numa_env,
    ns: *mut b::numa_stats,
    nid: c_int,
    find_idle: bool,
) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        core::ptr::write_bytes(ns, 0, 1);
        (*ns).idle_cpu = -1;
        let mut idle_core = -1;
        b::rust_fair_rcu_read_lock();
        let mask = b::rust_fair_cpumask_of_node(nid);
        let mut cpu = b::rust_fair_cpumask_first(mask);
        while cpu < b::rust_fair_small_cpumask_bits() {
            let rq = b::rust_fair_cpu_rq(cpu as c_int);
            (*ns).load = (*ns).load.wrapping_add(cpu_load(rq));
            (*ns).runnable = (*ns).runnable.wrapping_add(cpu_runnable(rq));
            (*ns).util = (*ns).util.wrapping_add(cpu_util_cfs(cpu as c_int));
            (*ns).nr_running = (*ns).nr_running.wrapping_add((*rq).cfs.h_nr_runnable);
            (*ns).compute_capacity = (*ns)
                .compute_capacity
                .wrapping_add(capacity_of(cpu as c_int));
            if find_idle
                && idle_core < 0
                && (*rq).nr_running == 0
                && b::idle_cpu(cpu as c_int) != 0
                && read_once!((*rq).numa_migrate_on) == 0
                && b::rust_fair_cpumask_test_cpu(cpu as c_int, (*(*env).p).cpus_ptr)
            {
                if (*ns).idle_cpu == -1 {
                    (*ns).idle_cpu = cpu as c_int;
                }
                idle_core = numa_idle_core(idle_core, cpu as c_int);
            }
            cpu = b::rust_fair_cpumask_next(cpu as c_int, mask);
        }
        b::rust_fair_rcu_read_unlock();
        (*ns).weight = b::rust_fair_cpumask_weight(mask);
        (*ns).node_type = numa_classify((*env).imbalance_pct as c_uint, ns);
        if idle_core >= 0 {
            (*ns).idle_cpu = idle_core;
        }
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn task_numa_assign(env: *mut b::task_numa_env, p: *mut b::task_struct, imp: c_long) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut rq = b::rust_fair_cpu_rq((*env).dst_cpu);
        if (*env).best_cpu != (*env).dst_cpu && b::rust_fair_xchg_numa_migrate_on(rq, 1) != 0 {
            let start = (*env).dst_cpu.wrapping_add(1);
            let mask = b::rust_fair_cpumask_of_node((*env).dst_nid);
            let mut selected = false;
            let mut cpu = b::rust_fair_cpu_iter_first_wrap(mask, start as c_uint);
            while cpu < b::rust_fair_small_cpumask_bits() {
                if cpu as c_int != (*env).best_cpu
                    && b::idle_cpu(cpu as c_int) != 0
                    && b::rust_fair_cpumask_test_cpu(cpu as c_int, (*(*env).p).cpus_ptr)
                {
                    (*env).dst_cpu = cpu as c_int;
                    rq = b::rust_fair_cpu_rq((*env).dst_cpu);
                    if b::rust_fair_xchg_numa_migrate_on(rq, 1) == 0 {
                        selected = true;
                        break;
                    }
                }
                cpu = b::rust_fair_cpu_iter_next_wrap(mask, start as c_uint, cpu.wrapping_add(1));
            }
            if !selected {
                return;
            }
        }
        if (*env).best_cpu != -1 && (*env).best_cpu != (*env).dst_cpu {
            rq = b::rust_fair_cpu_rq((*env).best_cpu);
            write_once!((*rq).numa_migrate_on, 0);
        }
        if !(*env).best_task.is_null() {
            b::rust_fair_put_task_struct((*env).best_task);
        }
        if !p.is_null() {
            b::rust_fair_get_task_struct(p);
        }
        (*env).best_task = p;
        (*env).best_imp = imp;
        (*env).best_cpu = (*env).dst_cpu;
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn load_too_imbalanced(src: c_long, dst: c_long, env: *mut b::task_numa_env) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let src_cap = (*env).src_stats.compute_capacity as c_long;
        let dst_cap = (*env).dst_stats.compute_capacity as c_long;
        let imbalance = dst
            .wrapping_mul(src_cap)
            .wrapping_sub(src.wrapping_mul(dst_cap))
            .wrapping_abs();
        let old = ((*env).dst_stats.load as c_long)
            .wrapping_mul(src_cap)
            .wrapping_sub(((*env).src_stats.load as c_long).wrapping_mul(dst_cap))
            .wrapping_abs();
        imbalance > old
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn task_numa_compare(
    env: *mut b::task_numa_env,
    taskimp: c_long,
    groupimp: c_long,
    maymove: bool,
) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let p_ng = deref_curr_numa_group((*env).p);
        let dst_rq = b::rust_fair_cpu_rq((*env).dst_cpu);
        let mut imp = if p_ng.is_null() { taskimp } else { groupimp };
        let dist = (*env).dist;
        let moveimp = imp;
        let mut stop = false;
        if read_once!((*dst_rq).numa_migrate_on) != 0 {
            return false;
        }
        b::rust_fair_rcu_read_lock();
        'unlock: {
            let mut cur = b::rust_fair_rq_curr_rcu(dst_rq);
            if !cur.is_null()
                && ((*cur).flags & (b::PF_EXITING | b::PF_KTHREAD) != 0 || (*cur).mm.is_null())
            {
                cur = null_mut();
            }
            if cur == (*env).p {
                stop = true;
                break 'unlock;
            }
            'assign: {
                if cur.is_null() {
                    if maymove && moveimp >= (*env).best_imp {
                        break 'assign;
                    } else {
                        break 'unlock;
                    }
                }
                if !b::rust_fair_cpumask_test_cpu((*env).src_cpu, (*cur).cpus_ptr) {
                    break 'unlock;
                }
                if !(*env).best_task.is_null()
                    && (*(*env).best_task).numa_preferred_nid == (*env).src_nid
                    && (*cur).numa_preferred_nid != (*env).src_nid
                {
                    break 'unlock;
                }
                let cur_ng = b::rust_fair_numa_group_rcu(cur);
                if cur_ng == p_ng {
                    if (*env).dst_stats.node_type == b::node_has_spare {
                        break 'unlock;
                    }
                    imp = taskimp
                        .wrapping_add(task_weight(cur, (*env).src_nid, dist) as c_long)
                        .wrapping_sub(task_weight(cur, (*env).dst_nid, dist) as c_long);
                    if !cur_ng.is_null() {
                        imp = imp.wrapping_sub(imp / 16);
                    }
                } else if !cur_ng.is_null() && !p_ng.is_null() {
                    imp = imp.wrapping_add(
                        group_weight(cur, (*env).src_nid, dist).wrapping_sub(group_weight(
                            cur,
                            (*env).dst_nid,
                            dist,
                        )) as c_long,
                    );
                } else {
                    imp = imp.wrapping_add(
                        task_weight(cur, (*env).src_nid, dist).wrapping_sub(task_weight(
                            cur,
                            (*env).dst_nid,
                            dist,
                        )) as c_long,
                    );
                }
                if (*cur).numa_preferred_nid == (*env).dst_nid {
                    imp = imp.wrapping_sub(imp / 16);
                }
                if (*cur).numa_preferred_nid == (*env).src_nid {
                    imp = imp.wrapping_add(imp / 8);
                }
                if maymove && moveimp > imp && moveimp > (*env).best_imp {
                    imp = moveimp;
                    cur = null_mut();
                    break 'assign;
                }
                if !(*env).best_task.is_null()
                    && (*cur).numa_preferred_nid == (*env).src_nid
                    && (*(*env).best_task).numa_preferred_nid != (*env).src_nid
                {
                    break 'assign;
                }
                if imp < 30 || imp <= (*env).best_imp.wrapping_add(15) {
                    break 'unlock;
                }
                let load = task_h_load((*env).p).wrapping_sub(task_h_load(cur)) as c_long;
                if load != 0
                    && load_too_imbalanced(
                        (*env).src_stats.load.wrapping_sub(load as c_ulong) as c_long,
                        (*env).dst_stats.load.wrapping_add(load as c_ulong) as c_long,
                        env,
                    )
                {
                    break 'unlock;
                }
            }
            if cur.is_null() {
                let mut cpu = (*env).dst_stats.idle_cpu;
                if cpu < 0 {
                    cpu = (*env).dst_cpu;
                }
                if b::idle_cpu(cpu) == 0 && (*env).best_cpu >= 0 && b::idle_cpu((*env).best_cpu) != 0 {
                    cpu = (*env).best_cpu;
                }
                (*env).dst_cpu = cpu;
            }
            task_numa_assign(env, cur, imp);
            if maymove && cur.is_null() && (*env).best_cpu >= 0 && b::idle_cpu((*env).best_cpu) != 0 {
                stop = true;
            }
            if !maymove
                && !(*env).best_task.is_null()
                && (*(*env).best_task).numa_preferred_nid == (*env).src_nid
            {
                stop = true;
            }
        }
        b::rust_fair_rcu_read_unlock();
        stop
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn task_numa_find_cpu(env: *mut b::task_numa_env, taskimp: c_long, groupimp: c_long) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut maymove = false;
        if (*env).dst_stats.node_type == b::node_has_spare {
            let src = (*env).src_stats.nr_running.wrapping_sub(1) as c_int;
            let dst = (*env).dst_stats.nr_running.wrapping_add(1) as c_int;
            let imbalance = max(0, dst.wrapping_sub(src));
            if adjust_numa_imbalance(imbalance, dst, (*env).imb_numa_nr) == 0 {
                maymove = true;
                if (*env).dst_stats.idle_cpu >= 0 {
                    (*env).dst_cpu = (*env).dst_stats.idle_cpu;
                    task_numa_assign(env, null_mut(), 0);
                    return;
                }
            }
        } else {
            let load = task_h_load((*env).p);
            maymove = !load_too_imbalanced(
                (*env).src_stats.load.wrapping_sub(load) as c_long,
                (*env).dst_stats.load.wrapping_add(load) as c_long,
                env,
            );
        }
        let mask = b::rust_fair_cpumask_of_node((*env).dst_nid);
        let allowed = (*(*env).p).cpus_ptr;
        let mut cpu = b::rust_fair_cpumask_first_and(mask, allowed);
        while cpu < b::rust_fair_small_cpumask_bits() {
            (*env).dst_cpu = cpu as c_int;
            if task_numa_compare(env, taskimp, groupimp, maymove) {
                break;
            }
            cpu = b::rust_fair_cpumask_next_and(cpu as c_int, mask, allowed);
        }
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn task_numa_migrate(p: *mut b::task_struct) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut env: b::task_numa_env = core::mem::zeroed();
        env.p = p;
        env.src_cpu = b::rust_fair_task_cpu(p);
        env.src_nid = b::rust_fair_task_node(p);
        env.imbalance_pct = 112;
        env.best_cpu = -1;
        b::rust_fair_rcu_read_lock();
        let sd = b::rust_fair_sd_numa_rcu(env.src_cpu);
        if !sd.is_null() {
            env.imbalance_pct = (100 + ((*sd).imbalance_pct - 100) / 2) as c_int;
            env.imb_numa_nr = (*sd).imb_numa_nr as c_int;
        }
        b::rust_fair_rcu_read_unlock();
        if sd.is_null() {
            b::sched_setnuma(p, b::rust_fair_task_node(p));
            return -(b::EINVAL as c_int);
        }
        env.dst_nid = (*p).numa_preferred_nid;
        env.dist = b::rust_fair_node_distance(env.src_nid, env.dst_nid);
        let mut dist = env.dist;
        let mut taskweight = task_weight(p, env.src_nid, dist);
        let mut groupweight = group_weight(p, env.src_nid, dist);
        update_numa_stats(
            addr_of_mut!(env),
            addr_of_mut!(env.src_stats),
            env.src_nid,
            false,
        );
        let taskimp = task_weight(p, env.dst_nid, dist).wrapping_sub(taskweight) as c_long;
        let groupimp = group_weight(p, env.dst_nid, dist).wrapping_sub(groupweight) as c_long;
        update_numa_stats(
            addr_of_mut!(env),
            addr_of_mut!(env.dst_stats),
            env.dst_nid,
            true,
        );
        task_numa_find_cpu(addr_of_mut!(env), taskimp, groupimp);
        let ng = deref_curr_numa_group(p);
        if env.best_cpu == -1 || (!ng.is_null() && (*ng).active_nodes > 1) {
            let mut nid = b::rust_fair_first_node_state(b::N_CPU);
            while nid < b::MAX_NUMNODES as c_int {
                if nid != env.src_nid && nid != (*p).numa_preferred_nid {
                    // Preserve original use of env.dst_nid here (before assigning nid).
                    dist = b::rust_fair_node_distance(env.src_nid, env.dst_nid);
                    if b::sched_numa_topology_type == b::NUMA_BACKPLANE && dist != env.dist {
                        taskweight = task_weight(p, env.src_nid, dist);
                        groupweight = group_weight(p, env.src_nid, dist);
                    }
                    let ti = task_weight(p, nid, dist).wrapping_sub(taskweight) as c_long;
                    let gi = group_weight(p, nid, dist).wrapping_sub(groupweight) as c_long;
                    if ti >= 0 || gi >= 0 {
                        env.dist = dist;
                        env.dst_nid = nid;
                        update_numa_stats(addr_of_mut!(env), addr_of_mut!(env.dst_stats), nid, true);
                        task_numa_find_cpu(addr_of_mut!(env), ti, gi);
                    }
                }
                nid = b::rust_fair_next_node_state(nid, b::N_CPU);
            }
        }
        if !ng.is_null() {
            let nid = if env.best_cpu == -1 {
                env.src_nid
            } else {
                b::rust_fair_cpu_to_node(env.best_cpu)
            };
            if nid != (*p).numa_preferred_nid {
                b::sched_setnuma(p, nid);
            }
        }
        if env.best_cpu == -1 {
            b::rust_fair_trace_sched_stick_numa(p, env.src_cpu, null_mut(), -1);
            return -(b::EAGAIN as c_int);
        }
        let best_rq = b::rust_fair_cpu_rq(env.best_cpu);
        let result = if env.best_task.is_null() {
            b::migrate_task_to(p, env.best_cpu)
        } else {
            b::migrate_swap(p, env.best_task, env.best_cpu, env.src_cpu)
        };
        write_once!((*best_rq).numa_migrate_on, 0);
        if result != 0 {
            b::rust_fair_trace_sched_stick_numa(p, env.src_cpu, env.best_task, env.best_cpu);
        }
        if !env.best_task.is_null() {
            b::rust_fair_put_task_struct(env.best_task);
        }
        result
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn numa_migrate_preferred(p: *mut b::task_struct) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if (*p).numa_preferred_nid == b::NUMA_NO_NODE || (*p).numa_faults.is_null() {
            return;
        }
        let interval = min(
            b::HZ as c_ulong,
            b::rust_fair_msecs_to_jiffies((*p).numa_scan_period) / 16,
        );
        (*p).numa_migrate_retry = b::rust_fair_jiffies().wrapping_add(interval);
        if b::rust_fair_task_node(p) != (*p).numa_preferred_nid {
            task_numa_migrate(p);
        }
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn numa_group_count_active_nodes(ng: *mut b::numa_group) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut maximum = 0;
        let mut active: c_int = 0;
        let mut nid = b::rust_fair_first_node_state(b::N_CPU);
        while nid < b::MAX_NUMNODES as c_int {
            maximum = max(maximum, group_faults_cpu(ng, nid));
            nid = b::rust_fair_next_node_state(nid, b::N_CPU);
        }
        nid = b::rust_fair_first_node_state(b::N_CPU);
        while nid < b::MAX_NUMNODES as c_int {
            if group_faults_cpu(ng, nid).wrapping_mul(3) > maximum {
                active += 1;
            }
            nid = b::rust_fair_next_node_state(nid, b::N_CPU);
        }
        (*ng).max_faults_cpu = maximum;
        (*ng).active_nodes = active;
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn update_task_scan_period(p: *mut b::task_struct, shared: c_ulong, private: c_ulong) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let remote = (*p).numa_faults_locality[0];
        let local = (*p).numa_faults_locality[1];
        if local.wrapping_add(shared) == 0 || (*p).numa_faults_locality[2] != 0 {
            (*p).numa_scan_period = min(
                (*p).numa_scan_period_max,
                (*p).numa_scan_period.wrapping_shl(1),
            );
            (*(*p).mm).numa_next_scan = b::rust_fair_jiffies()
                .wrapping_add(b::rust_fair_msecs_to_jiffies((*p).numa_scan_period));
            return;
        }
        let slot = (*p).numa_scan_period.wrapping_add(9) / 10;
        let lr = (local.wrapping_mul(10) / local.wrapping_add(remote)) as c_int;
        let ps = (private.wrapping_mul(10) / private.wrapping_add(shared)) as c_int;
        let diff = if ps >= 7 {
            max(ps - 7, 1).wrapping_mul(slot as c_int)
        } else if lr >= 7 {
            max(lr - 7, 1).wrapping_mul(slot as c_int)
        } else {
            (7 - max(lr, ps)).wrapping_neg().wrapping_mul(slot as c_int)
        };
        let period = (*p).numa_scan_period.wrapping_add(diff as c_uint);
        let lower = task_scan_min(p);
        let upper = task_scan_max(p);
        (*p).numa_scan_period = if period >= upper {
            upper
        } else if period <= lower {
            lower
        } else {
            period
        };
        core::ptr::write_bytes(addr_of_mut!((*p).numa_faults_locality), 0, 1);
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn numa_get_avg_runtime(p: *mut b::task_struct, period: *mut u64) -> u64 {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let now = (*p).se.exec_start;
        let runtime = (*p).se.sum_exec_runtime;
        let delta;
        if (*p).last_task_numa_placement != 0 {
            delta = runtime.wrapping_sub((*p).last_sum_exec_runtime);
            *period = now.wrapping_sub((*p).last_task_numa_placement);
            if (*period as i64) < 0 {
                *period = 0;
            }
        } else {
            delta = (*p).se.avg.load_sum;
            *period = b::LOAD_AVG_MAX as u64;
        }
        (*p).last_sum_exec_runtime = runtime;
        (*p).last_task_numa_placement = now;
        delta
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn preferred_group_nid(p: *mut b::task_struct, mut nid: c_int) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if b::sched_numa_topology_type == b::NUMA_DIRECT {
            return nid;
        }
        if b::sched_numa_topology_type == b::NUMA_GLUELESS_MESH {
            let mut maximum = 0;
            let mut best = nid;
            let dist = b::sched_max_numa_distance;
            let mut node = b::rust_fair_first_node_state(b::N_CPU);
            while node < b::MAX_NUMNODES as c_int {
                let score = group_weight(p, node, dist);
                if score > maximum {
                    maximum = score;
                    best = node;
                }
                node = b::rust_fair_next_node_state(node, b::N_CPU);
            }
            return best;
        }
        let mut nodes = b::node_states[b::N_CPU as usize];
        let mut dist = b::sched_max_numa_distance;
        while dist > b::LOCAL_DISTANCE as c_int {
            if b::find_numa_distance(dist) {
                let mut maximum = 0;
                let mut max_group: b::nodemask_t = core::mem::zeroed();
                let mut a = b::rust_fair_first_node_mask(addr_of!(nodes));
                while a < b::MAX_NUMNODES as c_int {
                    let mut faults: c_ulong = 0;
                    let mut group: b::nodemask_t = core::mem::zeroed();
                    let mut node = b::rust_fair_first_node_mask(addr_of!(nodes));
                    while node < b::MAX_NUMNODES as c_int {
                        if b::rust_fair_node_distance(a, node) < dist {
                            faults = faults.wrapping_add(group_faults(p, node));
                            b::rust_fair_node_set(node, addr_of_mut!(group));
                            b::rust_fair_node_clear(node, addr_of_mut!(nodes));
                        }
                        node = b::rust_fair_next_node_mask(node, addr_of!(nodes));
                    }
                    if faults > maximum {
                        maximum = faults;
                        max_group = group;
                        nid = a;
                    }
                    a = b::rust_fair_next_node_mask(a, addr_of!(nodes));
                }
                if maximum == 0 {
                    break;
                }
                nodes = max_group;
            }
            dist -= 1;
        }
        nid
    }
}

#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn task_numa_placement(p: *mut b::task_struct) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let seq = read_once!((*(*p).mm).numa_scan_seq);
        if (*p).numa_scan_seq == seq {
            return;
        }
        (*p).numa_scan_seq = seq;
        (*p).numa_scan_period_max = task_scan_max(p);
        let total = (*p).numa_faults_locality[0].wrapping_add((*p).numa_faults_locality[1]);
        let mut period = 0;
        let runtime = numa_get_avg_runtime(p, addr_of_mut!(period));
        let ng = deref_curr_numa_group(p);
        if !ng.is_null() {
            b::rust_fair_spin_lock_irq(addr_of_mut!((*ng).lock));
        }
        let mut maximum: c_ulong = 0;
        let mut best = b::NUMA_NO_NODE;
        let mut types = [0 as c_ulong; 2];
        let mut nid = b::rust_fair_first_online_node();
        while nid < b::MAX_NUMNODES as c_int {
            let mut faults: c_ulong = 0;
            let mut group_sum: c_ulong = 0;
            for private in 0..2 {
                let mem_idx = task_faults_idx(b::NUMA_MEM, nid, private) as isize;
                let buf_idx = task_faults_idx(b::NUMA_MEMBUF, nid, private) as isize;
                let cpu_idx = task_faults_idx(b::NUMA_CPU, nid, private) as isize;
                let cpu_buf_idx = task_faults_idx(b::NUMA_CPUBUF, nid, private) as isize;
                let f = (*p).numa_faults;
                let diff = (*f.offset(buf_idx)).wrapping_sub(*f.offset(mem_idx) / 2) as c_long;
                types[private as usize] = types[private as usize].wrapping_add(*f.offset(buf_idx));
                *f.offset(buf_idx) = 0;
                let mut weight = (runtime.wrapping_shl(16) / period.wrapping_add(1)) as c_long;
                weight = (weight as c_ulong)
                    .wrapping_mul(*f.offset(cpu_buf_idx))
                    .wrapping_div(total.wrapping_add(1)) as c_long;
                let fdiff = (weight as c_ulong).wrapping_sub(*f.offset(cpu_idx) / 2) as c_long;
                *f.offset(cpu_buf_idx) = 0;
                *f.offset(mem_idx) = (*f.offset(mem_idx)).wrapping_add(diff as c_ulong);
                *f.offset(cpu_idx) = (*f.offset(cpu_idx)).wrapping_add(fdiff as c_ulong);
                faults = faults.wrapping_add(*f.offset(mem_idx));
                (*p).total_numa_faults = (*p).total_numa_faults.wrapping_add(diff as c_ulong);
                if !ng.is_null() {
                    let nf = (*ng).faults.as_mut_ptr();
                    *nf.offset(mem_idx) = (*nf.offset(mem_idx)).wrapping_add(diff as c_ulong);
                    *nf.offset(cpu_idx) = (*nf.offset(cpu_idx)).wrapping_add(fdiff as c_ulong);
                    (*ng).total_faults = (*ng).total_faults.wrapping_add(diff as c_ulong);
                    group_sum = group_sum.wrapping_add(*nf.offset(mem_idx));
                }
                #[cfg(CONFIG_SCHED_CACHE)]
                {
                    let footprint =
                        (read_once!((*(*p).mm).sc_stat.footprint) as c_long).wrapping_add(diff);
                    write_once!((*(*p).mm).sc_stat.footprint, max(footprint, 0) as c_ulong);
                }
            }
            let count = if ng.is_null() { faults } else { group_sum };
            if count > maximum {
                maximum = count;
                best = nid;
            }
            nid = b::rust_fair_next_online_node(nid);
        }
        best = b::numa_nearest_node(best, b::N_CPU);
        if !ng.is_null() {
            numa_group_count_active_nodes(ng);
            b::rust_fair_spin_unlock_irq(addr_of_mut!((*ng).lock));
            best = preferred_group_nid(p, best);
        }
        if maximum != 0 && best != (*p).numa_preferred_nid {
            b::sched_setnuma(p, best);
        }
        update_task_scan_period(p, types[0], types[1]);
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn get_numa_group(group: *mut b::numa_group) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_refcount_inc_not_zero(addr_of_mut!((*group).refcount)) as c_int
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn put_numa_group(group: *mut b::numa_group) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if b::rust_fair_refcount_dec_and_test(addr_of_mut!((*group).refcount)) {
            b::rust_fair_kfree_numa_group_rcu(group);
        }
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn task_numa_group(
    p: *mut b::task_struct,
    cpupid: c_int,
    flags: c_int,
    private: *mut c_int,
) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let cpu = b::rust_fair_cpupid_to_cpu(cpupid);
        if deref_curr_numa_group(p).is_null() {
            let count = 4i32.wrapping_mul(b::nr_node_ids as c_int);
            let size = core::mem::size_of::<b::numa_group>()
                .wrapping_add((count as usize).wrapping_mul(core::mem::size_of::<c_ulong>()))
                as c_uint;
            let group = b::rust_fair_alloc_numa_group(size as usize);
            if group.is_null() {
                return;
            }
            b::rust_fair_refcount_set(addr_of_mut!((*group).refcount), 1);
            (*group).active_nodes = 1;
            (*group).max_faults_cpu = 0;
            b::rust_fair_init_numa_group_lock(addr_of_mut!((*group).lock));
            (*group).gid = (*p).pid;
            for i in 0..count {
                *(*group).faults.as_mut_ptr().offset(i as isize) = *(*p).numa_faults.offset(i as isize);
            }
            (*group).total_faults = (*p).total_numa_faults;
            (*group).nr_tasks = (*group).nr_tasks.wrapping_add(1);
            b::rust_fair_rcu_assign_numa_group(p, group);
        }
        let mut join = false;
        let mut group: *mut b::numa_group = null_mut();
        let mut mine: *mut b::numa_group = null_mut();
        b::rust_fair_rcu_read_lock();
        'no_join: {
            let task = b::rust_fair_rq_curr_once(b::rust_fair_cpu_rq(cpu));
            if !b::rust_fair_cpupid_match_pid(task, cpupid) {
                break 'no_join;
            }
            group = b::rust_fair_numa_group_rcu(task);
            if group.is_null() {
                break 'no_join;
            }
            mine = deref_curr_numa_group(p);
            if group == mine
                || (*mine).nr_tasks > (*group).nr_tasks
                || ((*mine).nr_tasks == (*group).nr_tasks && (mine as usize) > group as usize)
            {
                break 'no_join;
            }
            join = (*task).mm == (*b::rust_fair_current()).mm || flags & b::TNF_SHARED as c_int != 0;
            *private = (!join) as c_int;
            if join && get_numa_group(group) == 0 {
                join = false;
            }
        }
        b::rust_fair_rcu_read_unlock();
        if !join {
            return;
        }
        b::rust_fair_warn_task_numa_group_1(b::rust_fair_irqs_disabled());
        b::rust_fair_double_lock_irq(addr_of_mut!((*mine).lock), addr_of_mut!((*group).lock));
        for i in 0..4i32.wrapping_mul(b::nr_node_ids as c_int) {
            let amount = *(*p).numa_faults.offset(i as isize);
            let old = (*mine).faults.as_mut_ptr().offset(i as isize);
            let new = (*group).faults.as_mut_ptr().offset(i as isize);
            *old = (*old).wrapping_sub(amount);
            *new = (*new).wrapping_add(amount);
        }
        (*mine).total_faults = (*mine).total_faults.wrapping_sub((*p).total_numa_faults);
        (*group).total_faults = (*group).total_faults.wrapping_add((*p).total_numa_faults);
        (*mine).nr_tasks = (*mine).nr_tasks.wrapping_sub(1);
        (*group).nr_tasks = (*group).nr_tasks.wrapping_add(1);
        b::rust_fair_spin_unlock(addr_of_mut!((*mine).lock));
        b::rust_fair_spin_unlock_irq(addr_of_mut!((*group).lock));
        b::rust_fair_rcu_assign_numa_group(p, group);
        put_numa_group(mine);
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
#[no_mangle]
pub unsafe extern "C" fn task_numa_free(p: *mut b::task_struct, final_free: bool) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let group = b::rust_fair_numa_group_rcu_raw(p);
        let faults = (*p).numa_faults;
        if faults.is_null() {
            return;
        }
        let count = 4i32.wrapping_mul(b::nr_node_ids as c_int);
        if !group.is_null() {
            let flags = b::rust_fair_spin_lock_irqsave(addr_of_mut!((*group).lock));
            for i in 0..count {
                let slot = (*group).faults.as_mut_ptr().offset(i as isize);
                *slot = (*slot).wrapping_sub(*(*p).numa_faults.offset(i as isize));
            }
            (*group).total_faults = (*group).total_faults.wrapping_sub((*p).total_numa_faults);
            (*group).nr_tasks = (*group).nr_tasks.wrapping_sub(1);
            b::rust_fair_spin_unlock_irqrestore(addr_of_mut!((*group).lock), flags);
            b::rust_fair_rcu_init_numa_group(p, null_mut());
            put_numa_group(group);
        }
        if final_free {
            (*p).numa_faults = null_mut();
            b::kfree(faults.cast());
        } else {
            (*p).total_numa_faults = 0;
            for i in 0..count {
                *faults.offset(i as isize) = 0;
            }
        }
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
#[no_mangle]
pub unsafe extern "C" fn task_numa_fault(last: c_int, mem: c_int, pages: c_int, flags: c_int) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let p = b::rust_fair_current();
        let cpu = b::rust_fair_task_node(p);
        let mut local = (flags & b::TNF_FAULT_LOCAL as c_int != 0) as usize;
        if !b::rust_fair_numa_balancing_enabled() || (*p).mm.is_null() {
            return;
        }
        if !b::rust_fair_node_is_toptier(mem)
            && (b::sysctl_numa_balancing_mode & b::NUMA_BALANCING_MEMORY_TIERING != 0
                || !cpupid_valid(last))
        {
            return;
        }
        if (*p).numa_faults.is_null() {
            let size = core::mem::size_of::<c_ulong>()
                .wrapping_mul(8)
                .wrapping_mul(b::nr_node_ids as usize) as c_int;
            (*p).numa_faults = b::rust_fair_alloc_numa_faults(size as usize);
            if (*p).numa_faults.is_null() {
                return;
            }
            (*p).total_numa_faults = 0;
            core::ptr::write_bytes(addr_of_mut!((*p).numa_faults_locality), 0, 1);
        }
        let mut private = if last == (-1i32 & b::RUST_FAIR_LAST_CPUPID_MASK as c_int) {
            1
        } else {
            b::rust_fair_cpupid_match_pid(p, last) as c_int
        };
        if last != (-1i32 & b::RUST_FAIR_LAST_CPUPID_MASK as c_int)
            && private == 0
            && flags & b::TNF_NO_GROUP as c_int == 0
        {
            task_numa_group(p, last, flags, addr_of_mut!(private));
        }
        let ng = deref_curr_numa_group(p);
        if private == 0
            && local == 0
            && !ng.is_null()
            && (*ng).active_nodes > 1
            && numa_is_active_node(cpu, ng)
            && numa_is_active_node(mem, ng)
        {
            local = 1;
        }
        if b::rust_fair_jiffies().wrapping_sub((*p).numa_migrate_retry) as c_long > 0 {
            task_numa_placement(p);
            numa_migrate_preferred(p);
        }
        if flags & b::TNF_MIGRATED as c_int != 0 {
            (*p).numa_pages_migrated = (*p).numa_pages_migrated.wrapping_add(pages as c_ulong);
        }
        if flags & b::TNF_MIGRATE_FAIL as c_int != 0 {
            (*p).numa_faults_locality[2] = (*p).numa_faults_locality[2].wrapping_add(pages as c_ulong);
        }
        for (stat, nid) in [(b::NUMA_MEMBUF, mem), (b::NUMA_CPUBUF, cpu)] {
            let ptr = (*p)
                .numa_faults
                .offset(task_faults_idx(stat, nid, private) as isize);
            *ptr = (*ptr).wrapping_add(pages as c_ulong);
        }
        (*p).numa_faults_locality[local] =
            (*p).numa_faults_locality[local].wrapping_add(pages as c_ulong);
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn reset_ptenuma_scan(p: *mut b::task_struct) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        write_once!(
            (*(*p).mm).numa_scan_seq,
            read_once!((*(*p).mm).numa_scan_seq).wrapping_add(1)
        );
        (*(*p).mm).numa_scan_offset = 0;
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn vma_is_accessed(mm: *mut b::mm_struct, vma: *mut b::vm_area_struct) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let current = b::rust_fair_current();
        let state = (*vma).numab_state;
        if read_once!((*(*current).mm).numa_scan_seq).wrapping_sub((*state).start_scan_seq) < 2 {
            return true;
        }
        let pids = (*state).pids_active[0] | (*state).pids_active[1];
        if b::rust_fair_test_bit(b::rust_fair_hash_pid((*current).pid), addr_of!(pids)) {
            return true;
        }
        if (*mm).numa_scan_offset > (*vma).vm_start {
            b::rust_fair_trace_skip_vma_numa(mm, vma, b::NUMAB_SKIP_IGNORE_PID);
            return true;
        }
        read_once!((*mm).numa_scan_seq)
            > (*state)
                .prev_scan_seq
                .wrapping_add(b::rust_fair_get_nr_threads(current) as c_uint)
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe extern "C" fn task_numa_work(work: *mut b::callback_head) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let now = b::rust_fair_jiffies();
        let p = b::rust_fair_current();
        let mm = (*p).mm;
        let runtime = (*p).se.sum_exec_runtime;
        let expected = work
            .cast::<u8>()
            .sub(core::mem::offset_of!(b::task_struct, numa_work))
            .cast::<b::task_struct>();
        b::rust_fair_warn_task_numa_work_1(p != expected);
        (*work).next = work;
        if (*p).flags & b::PF_EXITING != 0 {
            return;
        }
        if b::rust_fair_cpusets_enabled() && b::rust_fair_current_mems_weight() == 1 {
            b::rust_fair_trace_skip_current_cpuset_numa(p);
            return;
        }
        if (*mm).numa_next_scan == 0 {
            (*mm).numa_next_scan = now.wrapping_add(b::rust_fair_msecs_to_jiffies(
                sysctl_numa_balancing_scan_delay,
            ));
        }
        let mut migrate = (*mm).numa_next_scan;
        if (now.wrapping_sub(migrate) as c_long) < 0 {
            return;
        }
        if (*p).numa_scan_period == 0 {
            (*p).numa_scan_period_max = task_scan_max(p);
            (*p).numa_scan_period = task_scan_start(p);
        }
        let next_scan = now.wrapping_add(b::rust_fair_msecs_to_jiffies((*p).numa_scan_period));
        if !b::rust_fair_try_cmpxchg_ulong(
            addr_of_mut!((*mm).numa_next_scan),
            addr_of_mut!(migrate),
            next_scan,
        ) {
            return;
        }
        (*p).node_stamp = (*p).node_stamp.wrapping_add((2 * b::RUST_FAIR_TICK_NSEC) as u64);
        let mut pages = (sysctl_numa_balancing_scan_size as c_long).wrapping_shl(20 - b::PAGE_SHIFT);
        let mut virtpages = pages.wrapping_mul(8);
        if pages == 0 || !b::rust_fair_mmap_read_trylock(mm) {
            return;
        }
        let mut skipped = false;
        let mut forced = false;
        let mut start: c_ulong;
        let mut vma: *mut b::vm_area_struct;
        'retry: loop {
            start = (*mm).numa_scan_offset;
            let mut iterator: b::vma_iterator = core::mem::zeroed();
            b::rust_fair_vma_iter_init(addr_of_mut!(iterator), mm, start);
            vma = b::rust_fair_vma_next(addr_of_mut!(iterator));
            if vma.is_null() {
                reset_ptenuma_scan(p);
                start = 0;
                b::rust_fair_vma_iter_set(addr_of_mut!(iterator), start);
                vma = b::rust_fair_vma_next(addr_of_mut!(iterator));
            }
            while !vma.is_null() {
                'next_vma: {
                    if !b::rust_fair_vma_migratable(vma)
                        || !b::rust_fair_vma_policy_mof(vma)
                        || b::rust_fair_is_vm_hugetlb_page(vma)
                        || (*vma).vm_flags & b::RUST_FAIR_VM_MIXEDMAP != 0
                    {
                        b::rust_fair_trace_skip_vma_numa(mm, vma, b::NUMAB_SKIP_UNSUITABLE);
                        break 'next_vma;
                    }
                    if (*vma).vm_mm.is_null()
                        || (!(*vma).vm_file.is_null()
                            && (*vma).vm_flags & (b::RUST_FAIR_VM_READ | b::RUST_FAIR_VM_WRITE)
                                == b::RUST_FAIR_VM_READ)
                    {
                        b::rust_fair_trace_skip_vma_numa(mm, vma, b::NUMAB_SKIP_SHARED_RO);
                        break 'next_vma;
                    }
                    if !b::rust_fair_vma_is_accessible(vma) {
                        b::rust_fair_trace_skip_vma_numa(mm, vma, b::NUMAB_SKIP_INACCESSIBLE);
                        break 'next_vma;
                    }
                    if (*vma).numab_state.is_null() {
                        let ptr = b::rust_fair_alloc_vma_numab_state();
                        if ptr.is_null() {
                            break 'next_vma;
                        }
                        if !b::rust_fair_cmpxchg_vma_numab_state(vma, null_mut(), ptr).is_null() {
                            b::kfree(ptr.cast());
                            break 'next_vma;
                        }
                        let state = (*vma).numab_state;
                        (*state).start_scan_seq = (*mm).numa_scan_seq;
                        (*state).next_scan = now.wrapping_add(b::rust_fair_msecs_to_jiffies(
                            sysctl_numa_balancing_scan_delay,
                        ));
                        (*state).pids_active_reset =
                            (*state)
                                .next_scan
                                .wrapping_add(b::rust_fair_msecs_to_jiffies(
                                    sysctl_numa_balancing_scan_delay.wrapping_mul(4),
                                ));
                        (*state).prev_scan_seq = (*mm).numa_scan_seq.wrapping_sub(1);
                    }
                    let state = (*vma).numab_state;
                    if (*mm).numa_scan_seq != 0
                        && (b::rust_fair_jiffies().wrapping_sub((*state).next_scan) as c_long) < 0
                    {
                        b::rust_fair_trace_skip_vma_numa(mm, vma, b::NUMAB_SKIP_SCAN_DELAY);
                        break 'next_vma;
                    }
                    if (*mm).numa_scan_seq != 0
                        && b::rust_fair_jiffies().wrapping_sub((*state).pids_active_reset) as c_long > 0
                    {
                        (*state).pids_active_reset =
                            (*state)
                                .pids_active_reset
                                .wrapping_add(b::rust_fair_msecs_to_jiffies(
                                    sysctl_numa_balancing_scan_delay.wrapping_mul(4),
                                ));
                        (*state).pids_active[0] = read_once!((*state).pids_active[1]);
                        (*state).pids_active[1] = 0;
                    }
                    if (*state).prev_scan_seq == (*mm).numa_scan_seq {
                        (*mm).numa_scan_offset = (*vma).vm_end;
                        b::rust_fair_trace_skip_vma_numa(mm, vma, b::NUMAB_SKIP_SEQ_COMPLETED);
                        break 'next_vma;
                    }
                    if !forced && !vma_is_accessed(mm, vma) {
                        skipped = true;
                        b::rust_fair_trace_skip_vma_numa(mm, vma, b::NUMAB_SKIP_PID_INACTIVE);
                        break 'next_vma;
                    }
                    loop {
                        start = max(start, (*vma).vm_start);
                        let candidate =
                            start.wrapping_add(pages.wrapping_shl(b::PAGE_SHIFT) as c_ulong);
                        let huge_mask = b::RUST_FAIR_HPAGE_SIZE - 1;
                        let end = min(
                            candidate.wrapping_add(huge_mask) & !huge_mask,
                            (*vma).vm_end,
                        );
                        let updates = b::change_prot_numa(vma, start, end);
                        let scanned = end.wrapping_sub(start) >> b::PAGE_SHIFT;
                        if updates != 0 {
                            pages = pages.wrapping_sub(scanned as c_long);
                        }
                        virtpages = virtpages.wrapping_sub(scanned as c_long);
                        start = end;
                        if pages <= 0 || virtpages <= 0 {
                            break 'retry;
                        }
                        b::rust_fair_cond_resched();
                        if end == (*vma).vm_end {
                            break;
                        }
                    }
                    (*state).prev_scan_seq = (*mm).numa_scan_seq;
                    if forced {
                        break;
                    }
                }
                vma = b::rust_fair_vma_next(addr_of_mut!(iterator));
            }
            if vma.is_null() && !forced && skipped {
                forced = true;
                continue 'retry;
            }
            break;
        }
        if !vma.is_null() {
            (*mm).numa_scan_offset = start;
        } else {
            reset_ptenuma_scan(p);
        }
        b::rust_fair_mmap_read_unlock(mm);
        if (*p).se.sum_exec_runtime != runtime {
            let diff = (*p).se.sum_exec_runtime.wrapping_sub(runtime);
            (*p).node_stamp = (*p).node_stamp.wrapping_add(diff.wrapping_mul(32));
        }
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
#[no_mangle]
pub unsafe extern "C" fn init_numa_balancing(clone_flags: u64, p: *mut b::task_struct) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mm = (*p).mm;
        let mut users: c_int = 0;
        if !mm.is_null() {
            users = b::rust_fair_atomic_read(addr_of!((*mm).mm_users));
            if users == 1 {
                (*mm).numa_next_scan = b::rust_fair_jiffies().wrapping_add(
                    b::rust_fair_msecs_to_jiffies(sysctl_numa_balancing_scan_delay),
                );
                (*mm).numa_scan_seq = 0;
            }
        }
        (*p).node_stamp = 0;
        (*p).numa_scan_seq = if mm.is_null() { 0 } else { (*mm).numa_scan_seq };
        (*p).numa_scan_period = sysctl_numa_balancing_scan_delay;
        (*p).numa_migrate_retry = 0;
        (*p).numa_work.next = addr_of_mut!((*p).numa_work);
        (*p).numa_faults = null_mut();
        (*p).numa_pages_migrated = 0;
        (*p).total_numa_faults = 0;
        b::rust_fair_rcu_init_numa_group(p, null_mut());
        (*p).last_task_numa_placement = 0;
        (*p).last_sum_exec_runtime = 0;
        b::rust_fair_init_task_work(addr_of_mut!((*p).numa_work), Some(task_numa_work));
        if clone_flags & b::CLONE_VM as u64 == 0 {
            (*p).numa_preferred_nid = b::NUMA_NO_NODE;
            return;
        }
        if !mm.is_null() {
            let current = b::rust_fair_current();
            let mut delay = min(
                task_scan_max(current),
                (*current)
                    .numa_scan_period
                    .wrapping_mul(users as c_uint)
                    .wrapping_mul(b::NSEC_PER_MSEC as c_uint),
            );
            delay = delay.wrapping_add((2 * b::RUST_FAIR_TICK_NSEC) as c_uint);
            (*p).node_stamp = delay as u64;
        }
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn task_tick_numa(_rq: *mut b::rq, current: *mut b::task_struct) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let work = addr_of_mut!((*current).numa_work);
        if (*current).mm.is_null()
            || (*current).flags & (b::PF_EXITING | b::PF_KTHREAD) != 0
            || (*work).next != work
        {
            return;
        }
        let now = (*current).se.sum_exec_runtime;
        let period = ((*current).numa_scan_period as u64).wrapping_mul(b::NSEC_PER_MSEC as u64);
        if now > (*current).node_stamp.wrapping_add(period) {
            if (*current).node_stamp == 0 {
                (*current).numa_scan_period = task_scan_start(current);
            }
            (*current).node_stamp = (*current).node_stamp.wrapping_add(period);
            if b::rust_fair_jiffies().wrapping_sub((*(*current).mm).numa_next_scan) as c_long >= 0 {
                b::task_work_add(current, work, b::TWA_RESUME);
            }
        }
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn update_scan_period(p: *mut b::task_struct, new_cpu: c_int) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let src = b::rust_fair_cpu_to_node(b::rust_fair_task_cpu(p));
        let dst = b::rust_fair_cpu_to_node(new_cpu);
        if !b::rust_fair_numa_balancing_enabled()
            || (*p).mm.is_null()
            || (*p).numa_faults.is_null()
            || (*p).flags & b::PF_EXITING != 0
            || src == dst
        {
            return;
        }
        if (*p).numa_scan_seq != 0
            && (dst == (*p).numa_preferred_nid
                || ((*p).numa_preferred_nid != b::NUMA_NO_NODE && src != (*p).numa_preferred_nid))
        {
            return;
        }
        (*p).numa_scan_period = task_scan_start(p);
    }
}
#[cfg(not(CONFIG_NUMA_BALANCING))]
unsafe fn task_tick_numa(_: *mut b::rq, _: *mut b::task_struct) {}
#[cfg(not(CONFIG_NUMA_BALANCING))]
unsafe fn account_numa_enqueue(_: *mut b::rq, _: *mut b::task_struct) {}
#[cfg(not(CONFIG_NUMA_BALANCING))]
unsafe fn account_numa_dequeue(_: *mut b::rq, _: *mut b::task_struct) {}
#[cfg(not(CONFIG_NUMA_BALANCING))]
unsafe fn update_scan_period(_: *mut b::task_struct, _: c_int) {}
#[cfg(CONFIG_SYSCTL)]
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn sched_fair_sysctl_init() -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_register_sysctl_init();
        0
    }
}
