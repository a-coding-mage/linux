// SPDX-License-Identifier: GPL-2.0
// kernel/sched/fair.c:7843-15642, frozen e1d84f501551943a11f4c5271e9f5c85d7e15168.
// Included in fair.rs; b::* is generated from the original configured C ABI.
// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
// RECONCILED-SOURCE-COMMIT: e1d84f501551943a11f4c5271e9f5c85d7e15168
// Native wrappers below are header/macro/architecture operations, not original
// fair.c algorithm bodies. Their executable work remains native and is inventoried.

#[inline]
fn fair_clamp_ulong(val: c_ulong, lo: c_ulong, hi: c_ulong) -> c_ulong {
    // include/linux/minmax.h:181: __clamp tests the upper bound first and
    // does not introduce a runtime assertion when bounds are dynamic.
    if val >= hi {
        hi
    } else if val <= lo {
        lo
    } else {
        val
    }
}

// Advance after the body, including after continue: the body may clear CPUs
// from the mask (select_idle_core), which must affect the next iteration.
struct FairCpuIter {
    mask: *const b::cpumask,
    and_mask: *const b::cpumask,
    last: c_int,
    start: c_int,
    wrap: bool,
    first: bool,
}
impl Iterator for FairCpuIter {
    type Item = c_int;
    fn next(&mut self) -> Option<c_int> {
        unsafe {
            let n = if self.wrap && self.first {
                b::rust_fair_cpu_iter_first_wrap(self.mask, self.start as c_uint)
            } else if self.wrap {
                b::rust_fair_cpu_iter_next_wrap(
                    self.mask,
                    self.start as c_uint,
                    (self.last.wrapping_add(1)) as c_uint,
                )
            } else if !self.and_mask.is_null() {
                b::rust_fair_cpu_iter_next_and(
                    self.mask,
                    self.and_mask,
                    (self.last.wrapping_add(1)) as c_uint,
                )
            } else {
                b::rust_fair_cpu_iter_next(self.mask, (self.last.wrapping_add(1)) as c_uint)
            } as c_int;
            self.first = false;
            self.last = n;
            if (n as c_uint) < b::rust_fair_small_cpumask_bits() {
                Some(n)
            } else {
                None
            }
        }
    }
}
macro_rules! fair_each_cpu {
    ($cpu:ident, $mask:expr, $body:block) => {{
        for $cpu in (FairCpuIter {
            mask: $mask,
            and_mask: core::ptr::null(),
            last: -1,
            start: 0,
            wrap: false,
            first: true,
        }) {
            $body
        }
    }};
}
macro_rules! fair_each_cpu_and {
    ($cpu:ident, $a:expr, $b:expr, $body:block) => {{
        for $cpu in (FairCpuIter {
            mask: $a,
            and_mask: $b,
            last: -1,
            start: 0,
            wrap: false,
            first: true,
        }) {
            $body
        }
    }};
}
macro_rules! fair_each_cpu_wrap {
    ($cpu:ident, $mask:expr, $start:expr, $body:block) => {{
        let start = $start;
        for $cpu in (FairCpuIter {
            mask: $mask,
            and_mask: core::ptr::null(),
            last: start.wrapping_sub(1),
            start,
            wrap: true,
            first: true,
        }) {
            $body
        }
    }};
}

#[inline]
unsafe fn cpu_overutilized(cpu: c_int) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if !b::rust_fair_sched_energy_enabled() {
            return false;
        }
        let rq_util_max = b::rust_fair_uclamp_rq_get(b::rust_fair_cpu_rq(cpu), b::UCLAMP_MAX);
        util_fits_cpu(cpu_util_cfs(cpu), 0, rq_util_max, cpu) == 0
    }
}
#[inline]
unsafe fn is_rd_overutilized(rd: *mut b::root_domain) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        !b::rust_fair_sched_energy_enabled() || read_once!((*rd).overutilized)
    }
}
#[inline]
unsafe fn set_rd_overutilized(rd: *mut b::root_domain, flag: bool) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if !b::rust_fair_sched_energy_enabled() {
            return;
        }
        write_once!((*rd).overutilized, flag as _);
        b::rust_fair_trace_sched_overutilized_tp(rd, flag);
    }
}
#[inline]
unsafe fn check_update_overutilized_status(rq: *mut b::rq) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if !is_rd_overutilized((*rq).rd) && cpu_overutilized((*rq).cpu) {
            set_rd_overutilized((*rq).rd, true);
        }
    }
}
unsafe fn sched_idle_rq(rq: *mut b::rq) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (*rq).nr_running == (*rq).cfs.h_nr_idle && (*rq).nr_running != 0
    }
}
unsafe fn choose_sched_idle_rq(rq: *mut b::rq, p: *mut b::task_struct) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        sched_idle_rq(rq) && !b::rust_fair_task_has_idle_policy(p)
    }
}
unsafe fn choose_idle_cpu(cpu: c_int, p: *mut b::task_struct) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_available_idle_cpu(cpu) != 0 || choose_sched_idle_rq(b::rust_fair_cpu_rq(cpu), p)
    }
}
unsafe fn requeue_delayed_entity(cfs_rq: *mut b::cfs_rq, se: *mut b::sched_entity) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_warn_requeue_delayed_entity_1(!((*se).sched_delayed != 0));
        b::rust_fair_warn_requeue_delayed_entity_2((*se).on_rq == 0);
        if update_entity_lag(cfs_rq, se) {
            (*cfs_rq).h_nr_queued = (*cfs_rq).h_nr_queued.wrapping_sub(1);
            if se != (*cfs_rq).curr {
                __dequeue_entity(cfs_rq, se);
            }
            place_entity(cfs_rq, se, 0);
            if se != (*cfs_rq).curr {
                __enqueue_entity(cfs_rq, se);
            }
            (*cfs_rq).h_nr_queued = (*cfs_rq).h_nr_queued.wrapping_add(1);
        }
        update_load_avg(cfs_rq, se, 0);
        clear_delayed(se);
    }
}
unsafe fn enqueue_hierarchy(p: *mut b::task_struct, mut flags: c_int) -> c_ulong {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut weight = b::RUST_FAIR_NICE_0_LOAD as c_ulong;
        let task_new = flags & b::RUST_FAIR_ENQUEUE_WAKEUP == 0;
        let mut se = addr_of_mut!((*p).se);
        let mut h_nr_idle = b::rust_fair_task_has_idle_policy(p) as c_uint;
        let h_nr_runnable = (!(task_new && ((*se).sched_delayed != 0))) as c_uint;
        while !se.is_null() {
            let cfs_rq = b::rust_fair_cfs_rq_of(se);
            update_curr(cfs_rq);
            if (*se).on_rq == 0 {
                enqueue_entity(cfs_rq, se, flags);
            } else {
                update_load_avg(cfs_rq, se, b::RUST_FAIR_UPDATE_TG);
                b::rust_fair_se_update_runnable(se);
                update_cfs_group(se);
            }
            (*cfs_rq).h_nr_runnable = (*cfs_rq).h_nr_runnable.wrapping_add(h_nr_runnable);
            (*cfs_rq).h_nr_queued = (*cfs_rq).h_nr_queued.wrapping_add(1);
            (*cfs_rq).h_nr_idle = (*cfs_rq).h_nr_idle.wrapping_add(h_nr_idle);
            if cfs_rq_is_idle(cfs_rq) != 0 {
                h_nr_idle = 1;
            }
            weight = __calc_prop_weight(cfs_rq, se, weight);
            flags = b::RUST_FAIR_ENQUEUE_WAKEUP;
            se = parent_entity(se);
        }
        weight
    }
}
#[inline]
unsafe fn update_curr_eevdf(cfs_rq: *mut b::cfs_rq) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if !(*cfs_rq).curr.is_null() {
            update_curr(b::rust_fair_cfs_rq_of((*cfs_rq).curr));
        }
    }
}
#[export_name = "rust_fair_enqueue_task_fair"]
unsafe extern "C" fn enqueue_task_fair(rq: *mut b::rq, p: *mut b::task_struct, flags: c_int) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let rq_h_nr_queued = (*rq).cfs.h_nr_queued;
        let task_new = flags & b::RUST_FAIR_ENQUEUE_WAKEUP == 0;
        let se = addr_of_mut!((*p).se);
        let cfs_rq = addr_of_mut!((*rq).cfs);
        if task_is_throttled(p) && enqueue_throttled_task(p) {
            return;
        }
        if !((*se).sched_delayed != 0) || flags & b::RUST_FAIR_ENQUEUE_DELAYED != 0 {
            util_est_enqueue(cfs_rq, p);
        }
        update_curr_eevdf(cfs_rq);
        if flags & b::RUST_FAIR_ENQUEUE_DELAYED != 0 {
            requeue_delayed_entity(cfs_rq, se);
            return;
        }
        if b::rust_fair_task_in_iowait(p) {
            b::rust_fair_cpufreq_update_util(rq, b::RUST_FAIR_SCHED_CPUFREQ_IOWAIT);
        }
        let curr = (*cfs_rq).curr == se;
        if curr {
            place_entity(cfs_rq, se, flags);
        }
        if (*se).on_rq != 0 && ((*se).sched_delayed != 0) {
            requeue_delayed_entity(cfs_rq, se);
        }
        let weight = enqueue_hierarchy(p, flags);
        if !curr {
            reweight_eevdf(cfs_rq, se, weight, false);
            place_entity(cfs_rq, se, flags | b::RUST_FAIR_ENQUEUE_QUEUED);
            __enqueue_entity(cfs_rq, se);
        }
        if rq_h_nr_queued == 0 && (*rq).cfs.h_nr_queued != 0 {
            b::dl_server_start(addr_of_mut!((*rq).fair_server));
        }
        b::rust_fair_add_nr_running(rq, 1);
        if !task_new {
            check_update_overutilized_status(rq);
        }
        assert_list_leaf_cfs_rq(rq);
        hrtick_update(rq);
    }
}
unsafe fn dequeue_hierarchy(p: *mut b::task_struct, mut flags: c_int) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut se = addr_of_mut!((*p).se);
        let task_sleep = flags & b::RUST_FAIR_DEQUEUE_SLEEP != 0;
        let task_delayed = flags & b::RUST_FAIR_DEQUEUE_DELAYED != 0;
        let task_throttled = flags & b::RUST_FAIR_DEQUEUE_THROTTLE != 0;
        let h_nr_runnable = (task_sleep || task_delayed || !((*se).sched_delayed != 0)) as c_uint;
        let mut h_nr_idle = b::rust_fair_task_has_idle_policy(p) as c_uint;
        let mut dequeue = true;
        while !se.is_null() {
            let cfs_rq = b::rust_fair_cfs_rq_of(se);
            update_curr(cfs_rq);
            if dequeue {
                dequeue_entity(cfs_rq, se, flags);
                if (*cfs_rq).load.weight != 0 {
                    dequeue = false;
                }
            } else {
                update_load_avg(cfs_rq, se, b::RUST_FAIR_UPDATE_TG);
                b::rust_fair_se_update_runnable(se);
                update_cfs_group(se);
            }
            (*cfs_rq).h_nr_runnable = (*cfs_rq).h_nr_runnable.wrapping_sub(h_nr_runnable);
            (*cfs_rq).h_nr_queued = (*cfs_rq).h_nr_queued.wrapping_sub(1);
            (*cfs_rq).h_nr_idle = (*cfs_rq).h_nr_idle.wrapping_sub(h_nr_idle);
            if cfs_rq_is_idle(cfs_rq) != 0 {
                h_nr_idle = 1;
            }
            if throttled_hierarchy(cfs_rq) != 0 && task_throttled {
                record_throttle_clock(cfs_rq);
            }
            flags |= b::RUST_FAIR_DEQUEUE_SLEEP;
            flags &= !(b::RUST_FAIR_DEQUEUE_DELAYED | b::RUST_FAIR_DEQUEUE_SPECIAL);
            se = parent_entity(se);
        }
    }
}
unsafe fn __dequeue_task(rq: *mut b::rq, p: *mut b::task_struct, flags: c_int) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let se = addr_of_mut!((*p).se);
        let cfs_rq = addr_of_mut!((*rq).cfs);
        let was_sched_idle = sched_idle_rq(rq);
        let task_sleep = flags & b::RUST_FAIR_DEQUEUE_SLEEP != 0;
        let task_delayed = flags & b::RUST_FAIR_DEQUEUE_DELAYED != 0;
        clear_buddies(cfs_rq, se);
        update_curr_eevdf(cfs_rq);
        update_entity_lag(cfs_rq, se);
        if task_delayed {
            b::rust_fair_warn___dequeue_task_1(!((*se).sched_delayed != 0));
        } else {
            let delay = task_sleep
                && flags & (b::RUST_FAIR_DEQUEUE_SPECIAL | b::RUST_FAIR_DEQUEUE_THROTTLE) == 0;
            b::rust_fair_warn___dequeue_task_2(delay && ((*se).sched_delayed != 0));
            if feat!(rust_fair_feat_DELAY_DEQUEUE) && delay && entity_eligible(cfs_rq, se) == 0 {
                update_load_avg(b::rust_fair_cfs_rq_of(se), se, b::RUST_FAIR_UPDATE_UTIL_EST);
                set_delayed(se);
                return false;
            }
        }
        dequeue_hierarchy(p, flags);
        if feat!(rust_fair_feat_PLACE_REL_DEADLINE) && !task_sleep {
            (*se).deadline = (*se).deadline.wrapping_sub((*se).vruntime);
            (*se).rel_deadline = 1;
        }
        if se != (*cfs_rq).curr {
            __dequeue_entity(cfs_rq, se);
        }
        b::rust_fair_sub_nr_running(rq, 1);
        if !was_sched_idle && sched_idle_rq(rq) {
            (*rq).next_balance = b::rust_fair_jiffies();
        }
        if task_delayed {
            clear_delayed(se);
            b::rust_fair_warn___dequeue_task_3(!task_sleep);
            b::rust_fair_warn___dequeue_task_4((*p).on_rq != 1);
            // Last use of p: __block_task may release the task's last reference.
            b::rust_fair___block_task(rq, p);
        }
        true
    }
}
#[export_name = "rust_fair_dequeue_task_fair"]
unsafe extern "C" fn dequeue_task_fair(
    rq: *mut b::rq,
    p: *mut b::task_struct,
    flags: c_int,
) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if task_is_throttled(p) {
            dequeue_throttled_task(p, flags);
            return true;
        }
        if !((*p).se.sched_delayed != 0) {
            util_est_dequeue(addr_of_mut!((*rq).cfs), p);
        }
        __dequeue_task(rq, p, flags)
    }
}
#[inline]
unsafe fn cfs_h_nr_delayed(rq: *mut b::rq) -> c_uint {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (*rq).cfs.h_nr_queued.wrapping_sub((*rq).cfs.h_nr_runnable)
    }
}
// load_balance_mask/select_rq_mask/should_we_balance_tmpmask retain native
// DEFINE_PER_CPU identity; nohz retains native cacheline alignment and layout.
unsafe fn cpu_load(rq: *mut b::rq) -> c_ulong {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        cfs_rq_load_avg(addr_of_mut!((*rq).cfs))
    }
}
unsafe fn cpu_load_without(rq: *mut b::rq, p: *mut b::task_struct) -> c_ulong {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if b::rust_fair_cpu_of(rq) != b::rust_fair_task_cpu(p)
            || read_once!((*p).se.avg.last_update_time) == 0
        {
            return cpu_load(rq);
        }
        // lsub_positive uses min_t(typeof(*ptr)): truncate the task load to
        // unsigned int before the minimum and subtraction, as in fair.c:4570.
        let load = read_once!((*rq).cfs.avg.load_avg) as c_uint;
        let sub = task_h_load(p) as c_uint;
        load.saturating_sub(sub) as c_ulong
    }
}
unsafe fn cpu_runnable(rq: *mut b::rq) -> c_ulong {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        cfs_rq_runnable_avg(addr_of_mut!((*rq).cfs))
    }
}
unsafe fn cpu_runnable_without(rq: *mut b::rq, p: *mut b::task_struct) -> c_ulong {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if b::rust_fair_cpu_of(rq) != b::rust_fair_task_cpu(p)
            || read_once!((*p).se.avg.last_update_time) == 0
        {
            return cpu_runnable(rq);
        }
        (read_once!((*rq).cfs.avg.runnable_avg) as c_uint)
            .saturating_sub((*p).se.avg.runnable_avg as c_uint) as c_ulong
    }
}
unsafe fn capacity_of(cpu: c_int) -> c_ulong {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (*b::rust_fair_cpu_rq(cpu)).cpu_capacity
    }
}
unsafe fn record_wakee(p: *mut b::task_struct) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let current = b::rust_fair_current();
        if b::rust_fair_time_after(
            b::rust_fair_jiffies(),
            (*current)
                .wakee_flip_decay_ts
                .wrapping_add(b::RUST_FAIR_HZ as c_ulong),
        ) {
            (*current).wakee_flips >>= 1;
            (*current).wakee_flip_decay_ts = b::rust_fair_jiffies();
        }
        if (*current).last_wakee != p {
            (*current).last_wakee = p;
            (*current).wakee_flips = (*current).wakee_flips.wrapping_add(1);
        }
    }
}
unsafe fn wake_wide(p: *mut b::task_struct) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut master = (*b::rust_fair_current()).wakee_flips;
        let mut slave = (*p).wakee_flips;
        let factor = b::rust_fair_this_sd_llc_size() as c_uint;
        if master < slave {
            core::mem::swap(&mut master, &mut slave);
        }
        !(slave < factor || master < slave.wrapping_mul(factor))
    }
}
unsafe fn wake_affine_idle(this_cpu: c_int, prev_cpu: c_int, sync: bool) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if b::rust_fair_available_idle_cpu(this_cpu) != 0
            && b::rust_fair_cpus_share_cache(this_cpu, prev_cpu)
        {
            return if b::rust_fair_available_idle_cpu(prev_cpu) != 0 {
                prev_cpu
            } else {
                this_cpu
            };
        }
        if sync {
            let rq = b::rust_fair_cpu_rq(this_cpu);
            if (*rq).nr_running.wrapping_sub(cfs_h_nr_delayed(rq)) == 1 {
                return this_cpu;
            }
        }
        if b::rust_fair_available_idle_cpu(prev_cpu) != 0 {
            return prev_cpu;
        }
        b::rust_fair_nr_cpumask_bits() as c_int
    }
}
unsafe fn wake_affine_weight(
    sd: *mut b::sched_domain,
    p: *mut b::task_struct,
    this_cpu: c_int,
    prev_cpu: c_int,
    sync: bool,
) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut this_eff_load = cpu_load(b::rust_fair_cpu_rq(this_cpu)) as i64;
        if sync {
            let current_load = task_h_load(b::rust_fair_current());
            if current_load as u64 > this_eff_load as u64 {
                return this_cpu;
            }
            this_eff_load = this_eff_load.wrapping_sub(current_load as i64);
        }
        let task_load = task_h_load(p);
        this_eff_load = this_eff_load.wrapping_add(task_load as i64);
        if feat!(rust_fair_feat_WA_BIAS) {
            this_eff_load = this_eff_load.wrapping_mul(100);
        }
        this_eff_load = this_eff_load.wrapping_mul(capacity_of(prev_cpu) as i64);
        let mut prev_eff_load =
            (cpu_load(b::rust_fair_cpu_rq(prev_cpu)) as i64).wrapping_sub(task_load as i64);
        if feat!(rust_fair_feat_WA_BIAS) {
            prev_eff_load = prev_eff_load
                .wrapping_mul(100u32.wrapping_add((*sd).imbalance_pct.wrapping_sub(100) / 2) as i64);
        }
        prev_eff_load = prev_eff_load.wrapping_mul(capacity_of(this_cpu) as i64);
        if sync {
            prev_eff_load = prev_eff_load.wrapping_add(1);
        }
        if this_eff_load < prev_eff_load {
            this_cpu
        } else {
            b::rust_fair_nr_cpumask_bits() as c_int
        }
    }
}
unsafe fn wake_affine(
    sd: *mut b::sched_domain,
    p: *mut b::task_struct,
    this_cpu: c_int,
    prev_cpu: c_int,
    sync: bool,
) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut target = b::rust_fair_nr_cpumask_bits() as c_int;
        if feat!(rust_fair_feat_WA_IDLE) {
            target = wake_affine_idle(this_cpu, prev_cpu, sync);
        }
        if feat!(rust_fair_feat_WA_WEIGHT) && target == b::rust_fair_nr_cpumask_bits() as c_int {
            target = wake_affine_weight(sd, p, this_cpu, prev_cpu, sync);
        }
        b::rust_fair_stat_wakeups_affine_attempts(p);
        if target != this_cpu {
            return prev_cpu;
        }
        b::rust_fair_stat_ttwu_move_affine(sd);
        b::rust_fair_stat_wakeups_affine(p);
        target
    }
}
unsafe fn sched_balance_find_dst_group_cpu(
    group: *mut b::sched_group,
    p: *mut b::task_struct,
    this_cpu: c_int,
) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut min_load = c_ulong::MAX;
        let mut min_exit_latency = c_uint::MAX;
        let mut latest_idle_timestamp = 0u64;
        let mut least_loaded_cpu = this_cpu;
        let mut shallowest_idle_cpu = -1;
        if (*group).group_weight == 1 {
            return b::rust_fair_cpumask_first(b::rust_fair_sched_group_span(group)) as c_int;
        }
        fair_each_cpu_and!(i, b::rust_fair_sched_group_span(group), (*p).cpus_ptr, {
            let rq = b::rust_fair_cpu_rq(i);
            if !b::rust_fair_sched_core_cookie_match(rq, p) {
                continue;
            }
            if choose_sched_idle_rq(rq, p) {
                return i;
            }
            if b::rust_fair_available_idle_cpu(i) != 0 {
                let idle = b::rust_fair_idle_get_state(rq);
                if !idle.is_null() && (*idle).exit_latency < min_exit_latency {
                    min_exit_latency = (*idle).exit_latency;
                    latest_idle_timestamp = (*rq).idle_stamp;
                    shallowest_idle_cpu = i;
                } else if (idle.is_null() || (*idle).exit_latency == min_exit_latency)
                    && (*rq).idle_stamp > latest_idle_timestamp
                {
                    latest_idle_timestamp = (*rq).idle_stamp;
                    shallowest_idle_cpu = i;
                }
            } else if shallowest_idle_cpu == -1 {
                let load = cpu_load(b::rust_fair_cpu_rq(i));
                if load < min_load {
                    min_load = load;
                    least_loaded_cpu = i;
                }
            }
        });
        if shallowest_idle_cpu != -1 {
            shallowest_idle_cpu
        } else {
            least_loaded_cpu
        }
    }
}
#[inline]
unsafe fn sched_balance_find_dst_cpu(
    mut sd: *mut b::sched_domain,
    p: *mut b::task_struct,
    mut cpu: c_int,
    prev_cpu: c_int,
    sd_flag: c_int,
) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut new_cpu = cpu;
        if !b::rust_fair_cpumask_intersects(b::rust_fair_sched_domain_span(sd), (*p).cpus_ptr) {
            return prev_cpu;
        }
        if sd_flag & b::RUST_FAIR_SD_BALANCE_FORK == 0 {
            sync_entity_load_avg(addr_of_mut!((*p).se));
        }
        while !sd.is_null() {
            if (*sd).flags & sd_flag == 0 {
                sd = (*sd).child;
                continue;
            }
            let group = sched_balance_find_dst_group(sd, p, cpu);
            if group.is_null() {
                sd = (*sd).child;
                continue;
            }
            new_cpu = sched_balance_find_dst_group_cpu(group, p, cpu);
            if new_cpu == cpu {
                sd = (*sd).child;
                continue;
            }
            cpu = new_cpu;
            let weight = (*sd).span_weight;
            sd = null_mut();
            let mut tmp = b::rust_fair_cpu_sched_domain(cpu);
            while !tmp.is_null() {
                if weight <= (*tmp).span_weight {
                    break;
                }
                if (*tmp).flags & sd_flag != 0 {
                    sd = tmp;
                }
                tmp = (*tmp).parent;
            }
        }
        new_cpu
    }
}
#[inline]
unsafe fn __select_idle_cpu(cpu: c_int, p: *mut b::task_struct) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if choose_idle_cpu(cpu, p) && b::rust_fair_sched_cpu_cookie_match(b::rust_fair_cpu_rq(cpu), p) {
            cpu
        } else {
            -1
        }
    }
}
#[inline]
unsafe fn set_idle_cores(cpu: c_int, val: bool) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let sds = b::rust_fair_rcu_sd_balance_shared(cpu);
        if !sds.is_null() {
            write_once!((*sds).has_idle_cores, val as _);
        }
    }
}
#[inline]
unsafe fn test_idle_cores(cpu: c_int) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let sds = b::rust_fair_rcu_sd_balance_shared(cpu);
        !sds.is_null() && read_once!((*sds).has_idle_cores) != 0
    }
}
#[no_mangle]
pub unsafe extern "C" fn __update_idle_core(rq: *mut b::rq) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let core = b::rust_fair_cpu_of(rq);
        b::rust_fair_rcu_read_lock();
        'scan: {
            if test_idle_cores(core) {
                break 'scan;
            }
            fair_each_cpu!(cpu, b::rust_fair_cpu_smt_mask(core), {
                if cpu == core {
                    continue;
                }
                if b::rust_fair_available_idle_cpu(cpu) == 0 {
                    break 'scan;
                }
            });
            set_idle_cores(core, true);
        }
        b::rust_fair_rcu_read_unlock();
    }
}
unsafe fn select_idle_core(
    p: *mut b::task_struct,
    core: c_int,
    cpus: *mut b::cpumask,
    idle_cpu: *mut c_int,
) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut idle = true;
        fair_each_cpu!(cpu, b::rust_fair_cpu_smt_mask(core), {
            if b::rust_fair_available_idle_cpu(cpu) == 0 {
                idle = false;
                if *idle_cpu == -1 {
                    if choose_sched_idle_rq(b::rust_fair_cpu_rq(cpu), p)
                        && b::rust_fair_cpumask_test_cpu(cpu, cpus)
                    {
                        *idle_cpu = cpu;
                        break;
                    }
                    continue;
                }
                break;
            }
            if *idle_cpu == -1 && b::rust_fair_cpumask_test_cpu(cpu, cpus) {
                *idle_cpu = cpu;
            }
        });
        if idle {
            return core;
        }
        b::rust_fair_cpumask_andnot(cpus, cpus, b::rust_fair_cpu_smt_mask(core));
        -1
    }
}
unsafe fn select_idle_smt(
    p: *mut b::task_struct,
    sd: *mut b::sched_domain,
    target: c_int,
) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        fair_each_cpu_and!(cpu, b::rust_fair_cpu_smt_mask(target), (*p).cpus_ptr, {
            if cpu == target {
                continue;
            }
            if !b::rust_fair_cpumask_test_cpu(cpu, b::rust_fair_sched_domain_span(sd)) {
                continue;
            }
            if choose_idle_cpu(cpu, p) {
                return cpu;
            }
        });
        -1
    }
}
unsafe fn select_idle_cpu(
    p: *mut b::task_struct,
    sd: *mut b::sched_domain,
    has_idle_core: bool,
    target: c_int,
) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let cpus = b::rust_fair_this_select_rq_mask();
        let mut idle_cpu = -1;
        let mut nr = c_int::MAX;
        if feat!(rust_fair_feat_SIS_UTIL) && !(*sd).shared.is_null() {
            nr = b::rust_fair_sd_nr_idle_scan((*sd).shared).wrapping_add(1) as c_int;
            if nr == 1 {
                return -1;
            }
        }
        if !b::rust_fair_cpumask_and(cpus, b::rust_fair_sched_domain_span(sd), (*p).cpus_ptr) {
            return -1;
        }
        if b::rust_fair_sched_cluster_active() {
            let sg = (*sd).groups;
            if (*sg).flags & b::RUST_FAIR_SD_CLUSTER != 0 {
                fair_each_cpu_wrap!(
                    cpu,
                    b::rust_fair_sched_group_span(sg),
                    target.wrapping_add(1),
                    {
                        if !b::rust_fair_cpumask_test_cpu(cpu, cpus) {
                            continue;
                        }
                        if has_idle_core {
                            let i = select_idle_core(p, cpu, cpus, &mut idle_cpu);
                            if (i as c_uint) < b::rust_fair_nr_cpumask_bits() {
                                return i;
                            }
                        } else {
                            nr = nr.wrapping_sub(1);
                            if nr <= 0 {
                                return -1;
                            }
                            idle_cpu = __select_idle_cpu(cpu, p);
                            if (idle_cpu as c_uint) < b::rust_fair_nr_cpumask_bits() {
                                return idle_cpu;
                            }
                        }
                    }
                );
                b::rust_fair_cpumask_andnot(cpus, cpus, b::rust_fair_sched_group_span(sg));
            }
        }
        fair_each_cpu_wrap!(cpu, cpus, target.wrapping_add(1), {
            if has_idle_core {
                let i = select_idle_core(p, cpu, cpus, &mut idle_cpu);
                if (i as c_uint) < b::rust_fair_nr_cpumask_bits() {
                    return i;
                }
            } else {
                nr = nr.wrapping_sub(1);
                if nr <= 0 {
                    return -1;
                }
                idle_cpu = __select_idle_cpu(cpu, p);
                if (idle_cpu as c_uint) < b::rust_fair_nr_cpumask_bits() {
                    break;
                }
            }
        });
        if has_idle_core {
            set_idle_cores(target, false);
        }
        idle_cpu
    }
}
unsafe fn select_idle_capacity(
    p: *mut b::task_struct,
    sd: *mut b::sched_domain,
    target: c_int,
) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let has_idle_core = b::rust_fair_sched_smt_active() && test_idle_cores(target);
        let mut best_cap: c_ulong = 0;
        let mut best_fits = b::ASYM_IDLE_THREAD_MISFIT as c_int;
        let mut best_cpu = -1;
        let mut nr = c_int::MAX;
        let cpus = b::rust_fair_this_select_rq_mask();
        b::rust_fair_cpumask_and(cpus, b::rust_fair_sched_domain_span(sd), (*p).cpus_ptr);
        let task_util = task_util_est(p);
        let util_min = b::rust_fair_uclamp_eff_value(p, b::UCLAMP_MIN);
        let util_max = b::rust_fair_uclamp_eff_value(p, b::UCLAMP_MAX);
        if feat!(rust_fair_feat_SIS_UTIL) && !(*sd).shared.is_null() {
            nr = b::rust_fair_sd_nr_idle_scan((*sd).shared).wrapping_add(1) as c_int;
            if nr == 1 {
                return -1;
            }
        }
        fair_each_cpu_wrap!(cpu, cpus, target, {
            let preferred_core = !has_idle_core || is_core_idle(cpu);
            let mut cpu_cap = capacity_of(cpu);
            if !has_idle_core {
                nr = nr.wrapping_sub(1);
                if nr <= 0 {
                    return best_cpu;
                }
            }
            if !choose_idle_cpu(cpu, p) {
                continue;
            }
            let mut fits = util_fits_cpu(task_util, util_min, util_max, cpu);
            if fits > 0 && preferred_core {
                return cpu;
            } else if fits < 0 {
                cpu_cap = get_actual_cpu_capacity(cpu);
            } else if fits > 0 {
                fits = b::ASYM_IDLE_THREAD_FITS as c_int;
            }
            if preferred_core {
                fits = fits.wrapping_add(b::ASYM_IDLE_CORE_BIAS as c_int);
            }
            if fits < best_fits || (fits == best_fits && cpu_cap > best_cap) {
                best_cap = cpu_cap;
                best_cpu = cpu;
                best_fits = fits;
            }
        });
        if has_idle_core && best_fits > b::ASYM_IDLE_COMPLETE_MISFIT as c_int {
            set_idle_cores(target, false);
        }
        best_cpu
    }
}
#[inline]
unsafe fn asym_fits_cpu(util: c_ulong, util_min: c_ulong, util_max: c_ulong, cpu: c_int) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if b::rust_fair_sched_asym_cpucap_active() {
            return (!b::rust_fair_sched_smt_active() || is_core_idle(cpu))
                && util_fits_cpu(util, util_min, util_max, cpu) > 0;
        }
        true
    }
}
unsafe fn select_idle_sibling(p: *mut b::task_struct, prev: c_int, target: c_int) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut has_idle_core = false;
        let mut prev_aff = -1;
        // Native C only reads these on the asymmetric branch; zero initialization
        // gives the symmetric call arguments defined Rust values with identical use.
        let (mut task_util, mut util_min, mut util_max) = (0, 0, 0);
        if b::rust_fair_sched_asym_cpucap_active() {
            sync_entity_load_avg(addr_of_mut!((*p).se));
            task_util = task_util_est(p);
            util_min = b::rust_fair_uclamp_eff_value(p, b::UCLAMP_MIN);
            util_max = b::rust_fair_uclamp_eff_value(p, b::UCLAMP_MAX);
        }
        b::rust_fair_lockdep_assert_irqs_disabled();
        if choose_idle_cpu(target, p) && asym_fits_cpu(task_util, util_min, util_max, target) {
            return target;
        }
        if prev != target
            && b::rust_fair_cpus_share_cache(prev, target)
            && choose_idle_cpu(prev, p)
            && asym_fits_cpu(task_util, util_min, util_max, prev)
        {
            if !b::rust_fair_sched_cluster_active() || b::rust_fair_cpus_share_resources(prev, target) {
                return prev;
            }
            prev_aff = prev;
        }
        if b::rust_fair_is_per_cpu_kthread(b::rust_fair_current())
            && b::rust_fair_in_task()
            && prev == b::rust_fair_smp_processor_id()
            && (*b::rust_fair_this_rq()).nr_running <= 1
            && asym_fits_cpu(task_util, util_min, util_max, prev)
        {
            return prev;
        }
        let mut recent_used_cpu = (*p).recent_used_cpu;
        (*p).recent_used_cpu = prev;
        if recent_used_cpu != prev
            && recent_used_cpu != target
            && b::rust_fair_cpus_share_cache(recent_used_cpu, target)
            && choose_idle_cpu(recent_used_cpu, p)
            && b::rust_fair_cpumask_test_cpu(recent_used_cpu, (*p).cpus_ptr)
            && asym_fits_cpu(task_util, util_min, util_max, recent_used_cpu)
        {
            if !b::rust_fair_sched_cluster_active()
                || b::rust_fair_cpus_share_resources(recent_used_cpu, target)
            {
                return recent_used_cpu;
            }
        } else {
            recent_used_cpu = -1;
        }
        if b::rust_fair_sched_asym_cpucap_active() {
            let sd = b::rust_fair_rcu_sd_asym_cpucapacity(target);
            if !sd.is_null() {
                let i = select_idle_capacity(p, sd, target);
                return if (i as c_uint) < b::rust_fair_nr_cpumask_bits() {
                    i
                } else {
                    target
                };
            }
        }
        let sd = b::rust_fair_rcu_sd_llc(target);
        if sd.is_null() {
            return target;
        }
        if b::rust_fair_sched_smt_active() {
            has_idle_core = test_idle_cores(target);
            if !has_idle_core && b::rust_fair_cpus_share_cache(prev, target) {
                let i = select_idle_smt(p, sd, prev);
                if (i as c_uint) < b::rust_fair_nr_cpumask_bits() {
                    return i;
                }
            }
        }
        let i = select_idle_cpu(p, sd, has_idle_core, target);
        if (i as c_uint) < b::rust_fair_nr_cpumask_bits() {
            return i;
        }
        if (prev_aff as c_uint) < b::rust_fair_nr_cpumask_bits() {
            return prev_aff;
        }
        if (recent_used_cpu as c_uint) < b::rust_fair_nr_cpumask_bits() {
            return recent_used_cpu;
        }
        target
    }
}
unsafe fn cpu_util(cpu: c_int, p: *mut b::task_struct, dst_cpu: c_int, boost: bool) -> c_ulong {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let add_task = !p.is_null() && b::rust_fair_task_cpu(p) != cpu && dst_cpu == cpu;
        let sub_task = !p.is_null() && b::rust_fair_task_cpu(p) == cpu && dst_cpu != cpu;
        let cfs_rq = addr_of_mut!((*b::rust_fair_cpu_rq(cpu)).cfs);
        let mut util = read_once!((*cfs_rq).avg.util_avg) as c_ulong;
        if add_task {
            util = util.wrapping_add(task_util(p));
        } else if sub_task {
            util = util.saturating_sub(task_util(p));
        }
        if boost {
            let mut runnable = read_once!((*cfs_rq).avg.runnable_avg) as c_ulong;
            if add_task {
                runnable = runnable.wrapping_add(read_once!((*p).se.avg.runnable_avg) as c_ulong);
            } else if sub_task {
                runnable = runnable.saturating_sub(read_once!((*p).se.avg.runnable_avg) as c_ulong);
            }
            util = max(util, runnable);
        }
        if feat!(rust_fair_feat_UTIL_EST) {
            let mut util_est = read_once!((*cfs_rq).avg.util_est) as c_ulong;
            if dst_cpu == cpu {
                util_est = util_est.wrapping_add(_task_util_est(p));
            } else if !p.is_null() && (b::rust_fair_task_on_rq_queued(p) || b::rust_fair_current() == p)
            {
                util_est = util_est.saturating_sub(_task_util_est(p));
            }
            util = max(util, util_est);
        }
        min(util, b::rust_fair_arch_scale_cpu_capacity(cpu))
    }
}
#[no_mangle]
pub unsafe extern "C" fn cpu_util_cfs(cpu: c_int) -> c_ulong {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        cpu_util(cpu, null_mut(), -1, false)
    }
}
#[no_mangle]
pub unsafe extern "C" fn cpu_util_cfs_boost(cpu: c_int) -> c_ulong {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        cpu_util(cpu, null_mut(), -1, true)
    }
}
unsafe fn cpu_util_without(cpu: c_int, mut p: *mut b::task_struct) -> c_ulong {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if cpu != b::rust_fair_task_cpu(p) || read_once!((*p).se.avg.last_update_time) == 0 {
            p = null_mut();
        }
        cpu_util(cpu, p, -1, false)
    }
}
#[no_mangle]
pub unsafe extern "C" fn effective_cpu_util(
    cpu: c_int,
    util_cfs: c_ulong,
    min_out: *mut c_ulong,
    max_out: *mut c_ulong,
) -> c_ulong {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let rq = b::rust_fair_cpu_rq(cpu);
        let scale = b::rust_fair_arch_scale_cpu_capacity(cpu);
        let irq = b::rust_fair_cpu_util_irq(rq);
        if irq >= scale {
            if !min_out.is_null() {
                *min_out = scale;
            }
            if !max_out.is_null() {
                *max_out = scale;
            }
            return scale;
        }
        if !min_out.is_null() {
            *min_out = max(
                irq.wrapping_add(b::rust_fair_cpu_bw_dl(rq)),
                b::rust_fair_uclamp_rq_get(rq, b::UCLAMP_MIN),
            );
            if !b::rust_fair_uclamp_is_used() && b::rust_fair_rt_rq_is_runnable(addr_of_mut!((*rq).rt))
            {
                *min_out = max(*min_out, scale);
            }
        }
        let mut util = util_cfs.wrapping_add(b::rust_fair_cpu_util_rt(rq));
        util = util.wrapping_add(b::rust_fair_cpu_util_dl(rq));
        if !max_out.is_null() {
            *max_out = min(scale, b::rust_fair_uclamp_rq_get(rq, b::UCLAMP_MAX));
        }
        if util >= scale {
            return scale;
        }
        util = b::rust_fair_scale_irq_capacity(util, irq, scale).wrapping_add(irq);
        min(scale, util)
    }
}
#[no_mangle]
pub unsafe extern "C" fn sched_cpu_util(cpu: c_int) -> c_ulong {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        effective_cpu_util(cpu, cpu_util_cfs(cpu), null_mut(), null_mut())
    }
}
#[inline]
unsafe fn eenv_task_busy_time(eenv: *mut b::energy_env, p: *mut b::task_struct, prev_cpu: c_int) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let max_cap = b::rust_fair_arch_scale_cpu_capacity(prev_cpu);
        let irq = b::rust_fair_cpu_util_irq(b::rust_fair_cpu_rq(prev_cpu));
        (*eenv).task_busy_time = if irq >= max_cap {
            max_cap
        } else {
            b::rust_fair_scale_irq_capacity(task_util_est(p), irq, max_cap)
        };
    }
}
#[inline]
unsafe fn eenv_pd_busy_time(
    eenv: *mut b::energy_env,
    pd_cpus: *mut b::cpumask,
    p: *mut b::task_struct,
) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut busy_time: c_ulong = 0;
        fair_each_cpu!(cpu, pd_cpus, {
            let util = cpu_util(cpu, p, -1, false);
            busy_time = busy_time.wrapping_add(effective_cpu_util(cpu, util, null_mut(), null_mut()));
        });
        (*eenv).pd_busy_time = min((*eenv).pd_cap, busy_time);
    }
}
#[inline]
unsafe fn eenv_pd_max_util(
    eenv: *mut b::energy_env,
    pd_cpus: *mut b::cpumask,
    p: *mut b::task_struct,
    dst_cpu: c_int,
) -> c_ulong {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut max_util = 0;
        fair_each_cpu!(cpu, pd_cpus, {
            let tsk = if cpu == dst_cpu { p } else { null_mut() };
            let util = cpu_util(cpu, p, dst_cpu, true);
            let (mut min_hint, mut max_hint) = (0, 0);
            let mut eff_util = effective_cpu_util(cpu, util, &mut min_hint, &mut max_hint);
            if !tsk.is_null() && b::rust_fair_uclamp_is_used() {
                min_hint = max(min_hint, b::rust_fair_uclamp_eff_value(p, b::UCLAMP_MIN));
                max_hint = if b::rust_fair_uclamp_rq_is_idle(b::rust_fair_cpu_rq(cpu)) {
                    b::rust_fair_uclamp_eff_value(p, b::UCLAMP_MAX)
                } else {
                    max(max_hint, b::rust_fair_uclamp_eff_value(p, b::UCLAMP_MAX))
                };
            }
            eff_util = b::rust_fair_sugov_effective_cpu_perf(cpu, eff_util, min_hint, max_hint);
            max_util = max(max_util, eff_util);
        });
        min(max_util, (*eenv).cpu_cap)
    }
}
#[inline]
unsafe fn compute_energy(
    eenv: *mut b::energy_env,
    pd: *mut b::perf_domain,
    pd_cpus: *mut b::cpumask,
    p: *mut b::task_struct,
    dst_cpu: c_int,
) -> c_ulong {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let max_util = eenv_pd_max_util(eenv, pd_cpus, p, dst_cpu);
        let mut busy_time = (*eenv).pd_busy_time;
        if dst_cpu >= 0 {
            busy_time = min(
                (*eenv).pd_cap,
                busy_time.wrapping_add((*eenv).task_busy_time),
            );
        }
        let energy = b::rust_fair_em_cpu_energy((*pd).em_pd, max_util, busy_time, (*eenv).cpu_cap);
        b::rust_fair_trace_sched_compute_energy_tp(p, dst_cpu, energy, max_util, busy_time);
        energy
    }
}
unsafe fn find_energy_efficient_cpu(p: *mut b::task_struct, prev_cpu: c_int) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let cpus = b::rust_fair_this_select_rq_mask();
        let mut prev_delta = c_ulong::MAX;
        let mut best_delta = c_ulong::MAX;
        let p_util_min = if b::rust_fair_uclamp_is_used() {
            b::rust_fair_uclamp_eff_value(p, b::UCLAMP_MIN)
        } else {
            0
        };
        let p_util_max = if b::rust_fair_uclamp_is_used() {
            b::rust_fair_uclamp_eff_value(p, b::UCLAMP_MAX)
        } else {
            1024
        };
        let rd = (*b::rust_fair_this_rq()).rd;
        let mut target = -1;
        let mut best_energy_cpu = -1;
        let (mut prev_fits, mut best_fits) = (-1, -1);
        let (mut best_actual_cap, mut prev_actual_cap) = (0, 0);
        let mut eenv: b::energy_env = core::mem::zeroed();
        let mut pd = b::rust_fair_rcu_rd_pd(rd);
        if pd.is_null() {
            return target;
        }
        let mut sd = b::rust_fair_rcu_sd_asym_cpucapacity(b::rust_fair_smp_processor_id());
        while !sd.is_null()
            && !b::rust_fair_cpumask_test_cpu(prev_cpu, b::rust_fair_sched_domain_span(sd))
        {
            sd = (*sd).parent;
        }
        if sd.is_null() {
            return target;
        }
        target = prev_cpu;
        sync_entity_load_avg(addr_of_mut!((*p).se));
        if task_util_est(p) == 0 && p_util_min == 0 {
            return target;
        }
        eenv_task_busy_time(&mut eenv, p, prev_cpu);
        while !pd.is_null() {
            let current_pd = pd;
            pd = (*pd).next;
            let pd = current_pd;
            let (mut util_min, mut util_max) = (p_util_min, p_util_max);
            let (mut prev_spare_cap, mut max_spare_cap): (c_long, c_long) = (-1, -1);
            let mut max_spare_cap_cpu = -1;
            let mut max_fits = -1;
            if !b::rust_fair_cpumask_and(
                cpus,
                b::rust_fair_perf_domain_span(pd),
                b::rust_fair_cpu_online_mask(),
            ) {
                continue;
            }
            let cpu = b::rust_fair_cpumask_first(cpus) as c_int;
            let cpu_actual_cap = get_actual_cpu_capacity(cpu);
            eenv.cpu_cap = cpu_actual_cap;
            eenv.pd_cap = 0;
            fair_each_cpu!(cpu, cpus, {
                let rq = b::rust_fair_cpu_rq(cpu);
                eenv.pd_cap = eenv.pd_cap.wrapping_add(cpu_actual_cap);
                if !b::rust_fair_cpumask_test_cpu(cpu, b::rust_fair_sched_domain_span(sd)) {
                    continue;
                }
                if !b::rust_fair_cpumask_test_cpu(cpu, (*p).cpus_ptr) {
                    continue;
                }
                let util = cpu_util(cpu, p, cpu, false);
                let mut cpu_cap = capacity_of(cpu);
                if b::rust_fair_uclamp_is_used() && !b::rust_fair_uclamp_rq_is_idle(rq) {
                    let rq_util_min = b::rust_fair_uclamp_rq_get(rq, b::UCLAMP_MIN);
                    let rq_util_max = b::rust_fair_uclamp_rq_get(rq, b::UCLAMP_MAX);
                    util_min = max(rq_util_min, p_util_min);
                    util_max = max(rq_util_max, p_util_max);
                }
                let fits = util_fits_cpu(util, util_min, util_max, cpu);
                if fits == 0 {
                    continue;
                }
                cpu_cap = cpu_cap.saturating_sub(util);
                if cpu == prev_cpu {
                    prev_spare_cap = cpu_cap as c_long;
                    prev_fits = fits;
                } else if fits > max_fits || (fits == max_fits && cpu_cap as c_long > max_spare_cap) {
                    max_spare_cap = cpu_cap as c_long;
                    max_spare_cap_cpu = cpu;
                    max_fits = fits;
                }
            });
            if max_spare_cap_cpu < 0 && prev_spare_cap < 0 {
                continue;
            }
            eenv_pd_busy_time(&mut eenv, cpus, p);
            let base_energy = compute_energy(&mut eenv, pd, cpus, p, -1);
            if prev_spare_cap > -1 {
                prev_delta = compute_energy(&mut eenv, pd, cpus, p, prev_cpu);
                if prev_delta < base_energy {
                    return target;
                }
                prev_delta = prev_delta.wrapping_sub(base_energy);
                prev_actual_cap = cpu_actual_cap;
                best_delta = min(best_delta, prev_delta);
            }
            if max_spare_cap_cpu >= 0 && max_spare_cap > prev_spare_cap {
                if max_fits < best_fits {
                    continue;
                }
                if max_fits < 0 && cpu_actual_cap <= best_actual_cap {
                    continue;
                }
                let mut cur_delta = compute_energy(&mut eenv, pd, cpus, p, max_spare_cap_cpu);
                if cur_delta < base_energy {
                    return target;
                }
                cur_delta = cur_delta.wrapping_sub(base_energy);
                if max_fits > 0 && best_fits > 0 && cur_delta >= best_delta {
                    continue;
                }
                best_delta = cur_delta;
                best_energy_cpu = max_spare_cap_cpu;
                best_fits = max_fits;
                best_actual_cap = cpu_actual_cap;
            }
        }
        if best_fits > prev_fits
            || (best_fits > 0 && best_delta < prev_delta)
            || (best_fits < 0 && best_actual_cap > prev_actual_cap)
        {
            target = best_energy_cpu;
        }
        target
    }
}
#[export_name = "rust_fair_select_task_rq_fair"]
unsafe extern "C" fn select_task_rq_fair(
    p: *mut b::task_struct,
    prev_cpu: c_int,
    wake_flags: c_int,
) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let sync = wake_flags & b::RUST_FAIR_WF_SYNC != 0
            && (*b::rust_fair_current()).flags & b::RUST_FAIR_PF_EXITING == 0;
        let mut sd = null_mut();
        let cpu = b::rust_fair_smp_processor_id();
        let mut new_cpu = prev_cpu;
        let mut want_affine = false;
        let sd_flag = wake_flags & 0xF;
        b::rust_fair_lockdep_assert_pi_lock(p);
        if wake_flags & b::RUST_FAIR_WF_TTWU != 0 {
            record_wakee(p);
            if wake_flags & b::RUST_FAIR_WF_CURRENT_CPU != 0
                && b::rust_fair_cpumask_test_cpu(cpu, (*p).cpus_ptr)
            {
                return cpu;
            }
            if !is_rd_overutilized((*b::rust_fair_this_rq()).rd) {
                new_cpu = find_energy_efficient_cpu(p, prev_cpu);
                if new_cpu >= 0 {
                    return new_cpu;
                }
                new_cpu = prev_cpu;
            }
            want_affine = !wake_wide(p) && b::rust_fair_cpumask_test_cpu(cpu, (*p).cpus_ptr);
        }
        let mut tmp = b::rust_fair_cpu_sched_domain(cpu);
        while !tmp.is_null() {
            if want_affine
                && (*tmp).flags & b::RUST_FAIR_SD_WAKE_AFFINE != 0
                && b::rust_fair_cpumask_test_cpu(prev_cpu, b::rust_fair_sched_domain_span(tmp))
            {
                if cpu != prev_cpu {
                    new_cpu = wake_affine(tmp, p, cpu, prev_cpu, sync);
                }
                sd = null_mut();
                break;
            }
            if (*tmp).flags & sd_flag != 0 {
                sd = tmp;
            } else if !want_affine {
                break;
            }
            tmp = (*tmp).parent;
        }
        if !sd.is_null() {
            return sched_balance_find_dst_cpu(sd, p, cpu, prev_cpu, sd_flag);
        }
        if wake_flags & b::RUST_FAIR_WF_TTWU != 0 {
            return select_idle_sibling(p, prev_cpu, new_cpu);
        }
        new_cpu
    }
}
#[export_name = "rust_fair_migrate_task_rq_fair"]
unsafe extern "C" fn migrate_task_rq_fair(p: *mut b::task_struct, new_cpu: c_int) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let se = addr_of_mut!((*p).se);
        if !b::rust_fair_task_on_rq_migrating(p) {
            remove_entity_load_avg(se);
            migrate_se_pelt_lag(se);
        }
        (*se).avg.last_update_time = 0;
        update_scan_period(p, new_cpu);
    }
}
#[export_name = "rust_fair_task_dead_fair"]
unsafe extern "C" fn task_dead_fair(p: *mut b::task_struct) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        remove_entity_load_avg(addr_of_mut!((*p).se));
    }
}
unsafe fn set_task_max_allowed_capacity(p: *mut b::task_struct) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if !b::rust_fair_sched_asym_cpucap_active() {
            return;
        }
        b::rust_fair_rcu_read_lock();
        let mut entry = b::rust_fair_asym_cap_first_rcu();
        while !entry.is_null() {
            let cpumask = b::rust_fair_cpu_capacity_span(entry);
            if b::rust_fair_cpumask_intersects((*p).cpus_ptr, cpumask) {
                (*p).max_allowed_capacity = (*entry).capacity;
                break;
            }
            entry = b::rust_fair_asym_cap_next_rcu(entry);
        }
        b::rust_fair_rcu_read_unlock();
    }
}
#[export_name = "rust_fair_set_cpus_allowed_fair"]
unsafe extern "C" fn set_cpus_allowed_fair(p: *mut b::task_struct, ctx: *mut b::affinity_context) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::set_cpus_allowed_common(p, ctx);
        set_task_max_allowed_capacity(p);
    }
}
#[inline]
unsafe fn set_preempt_buddy(cfs_rq: *mut b::cfs_rq, pse: *mut b::sched_entity) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if !(*cfs_rq).next.is_null() && entity_before((*cfs_rq).next, pse) {
            return false;
        }
        set_next_buddy(cfs_rq, pse);
        true
    }
}
#[inline]
unsafe fn set_short_buddy(cfs_rq: *mut b::cfs_rq, pse: *mut b::sched_entity) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if !(*cfs_rq).next.is_null() && (*(*cfs_rq).next).slice < (*pse).slice {
            return false;
        }
        set_next_buddy(cfs_rq, pse);
        true
    }
}
#[inline]
unsafe fn preempt_sync(
    rq: *mut b::rq,
    wake_flags: c_int,
    pse: *mut b::sched_entity,
    se: *mut b::sched_entity,
) -> b::preempt_wakeup_action {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_warn_preempt_sync_1(wake_flags & b::RUST_FAIR_WF_TTWU == 0);
        let mut threshold = sysctl_sched_migration_cost as u64;
        let mut delta = b::rust_fair_rq_clock_task(rq).wrapping_sub((*se).exec_start);
        if (delta as i64) < 0 {
            delta = 0;
        }
        if wake_flags & b::RUST_FAIR_WF_RQ_SELECTED != 0 {
            threshold >>= 2;
        }
        if entity_before(pse, se) && delta >= threshold {
            b::PREEMPT_WAKEUP_RESCHED
        } else {
            b::PREEMPT_WAKEUP_NONE
        }
    }
}
#[export_name = "rust_fair_wakeup_preempt_fair"]
unsafe extern "C" fn wakeup_preempt_fair(
    rq: *mut b::rq,
    p: *mut b::task_struct,
    wake_flags: c_int,
) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut action = b::PREEMPT_WAKEUP_PICK;
        let donor = (*rq).donor;
        let se = addr_of_mut!((*donor).se);
        let pse = addr_of_mut!((*p).se);
        let cfs_rq = addr_of_mut!((*rq).cfs);
        if (*p).sched_class != addr_of!(b::fair_sched_class)
            || (*donor).sched_class != addr_of!(b::fair_sched_class)
        {
            return;
        }
        if se == pse || task_is_throttled(p) {
            return;
        }
        if !feat!(rust_fair_feat_PREEMPT_SHORT) && b::rust_fair_test_tsk_need_resched((*rq).curr) {
            return;
        }
        if !feat!(rust_fair_feat_WAKEUP_PREEMPTION) {
            return;
        }
        b::rust_fair_warn_wakeup_preempt_fair_1(pse.is_null());
        let cse_is_idle = se_is_idle(se);
        let pse_is_idle = se_is_idle(pse);
        let preempt = 'decision: {
            if cse_is_idle != 0 && pse_is_idle == 0 {
                break 'decision true;
            }
            update_curr_fair(rq);
            if cse_is_idle != pse_is_idle
                || !b::rust_fair_normal_policy((*p).policy as c_int)
                || ((*pse).sched_delayed != 0)
            {
                break 'decision false;
            }
            if feat!(rust_fair_feat_PREEMPT_SHORT) && (*pse).slice < (*se).slice {
                action = b::PREEMPT_WAKEUP_SHORT;
            } else {
                if wake_flags & b::RUST_FAIR_WF_FORK != 0 {
                    break 'decision false;
                }
                if feat!(rust_fair_feat_NEXT_BUDDY)
                    && set_preempt_buddy(cfs_rq, pse)
                    && wake_flags & b::RUST_FAIR_WF_SYNC != 0
                {
                    action = preempt_sync(rq, wake_flags, pse, se);
                }
                if action == b::PREEMPT_WAKEUP_NONE {
                    return;
                }
                if action == b::PREEMPT_WAKEUP_RESCHED {
                    break 'decision true;
                }
            }
            while (*cfs_rq).h_nr_queued != 0 {
                let nse = pick_next_entity(rq, action != b::PREEMPT_WAKEUP_SHORT);
                if nse.is_null() {
                    continue;
                }
                if nse == pse {
                    break 'decision true;
                }
                break;
            }
            action == b::PREEMPT_WAKEUP_SHORT && entity_eligible(cfs_rq, pse) != 0
        };
        if preempt {
            cancel_protect_slice(se);
            if action == b::PREEMPT_WAKEUP_SHORT {
                set_short_buddy(cfs_rq, pse);
            }
            b::resched_curr_lazy(rq);
        } else if feat!(rust_fair_feat_RUN_TO_PARITY) {
            update_protect_slice(cfs_rq, se);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn pick_task_fair(
    rq: *mut b::rq,
    rf: *mut b::rq_flags,
) -> *mut b::task_struct {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let cfs_rq = addr_of_mut!((*rq).cfs);
        loop {
            if (*cfs_rq).h_nr_queued != 0 {
                if !(*cfs_rq).curr.is_null() && (*(*cfs_rq).curr).on_rq != 0 {
                    update_curr(cfs_rq);
                }
                let se = pick_next_entity(rq, true);
                if se.is_null() {
                    continue;
                }
                return b::rust_fair_task_of(se);
            }
            if b::rust_fair_sched_core_enabled(rq) {
                return null_mut();
            }
            let new_tasks = sched_balance_newidle(rq, rf);
            if new_tasks < 0 {
                return b::rust_fair_retry_task();
            }
            if new_tasks > 0 {
                continue;
            }
            return null_mut();
        }
    }
}
unsafe extern "C" fn fair_server_pick_task(
    dl_se: *mut b::sched_dl_entity,
    rf: *mut b::rq_flags,
) -> *mut b::task_struct {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        pick_task_fair((*dl_se).rq, rf)
    }
}
#[no_mangle]
pub unsafe extern "C" fn fair_server_init(rq: *mut b::rq) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let dl_se = addr_of_mut!((*rq).fair_server);
        b::init_dl_entity(dl_se);
        b::dl_server_init(dl_se, rq, Some(fair_server_pick_task));
    }
}
#[export_name = "rust_fair_put_prev_task_fair"]
unsafe extern "C" fn put_prev_task_fair(
    rq: *mut b::rq,
    prev: *mut b::task_struct,
    next: *mut b::task_struct,
) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut se = addr_of_mut!((*prev).se);
        let mut nse: *mut b::sched_entity = null_mut();
        #[cfg(CONFIG_FAIR_GROUP_SCHED)]
        if !next.is_null() && (*next).sched_class == addr_of!(b::fair_sched_class) {
            nse = addr_of_mut!((*next).se);
        }
        while !se.is_null() {
            let cfs_rq = b::rust_fair_cfs_rq_of(se);
            if nse.is_null() || !(*cfs_rq).h_curr.is_null() {
                put_prev_entity(cfs_rq, se);
            }
            #[cfg(CONFIG_FAIR_GROUP_SCHED)]
            if !nse.is_null() {
                if !is_same_group(se, nse).is_null() {
                    break;
                }
                let d = (*nse).depth.wrapping_sub((*se).depth);
                if d >= 0 {
                    nse = parent_entity(nse);
                    if d > 0 {
                        continue;
                    }
                }
            }
            se = parent_entity(se);
        }
        let cfs_rq = addr_of_mut!((*rq).cfs);
        se = addr_of_mut!((*prev).se);
        b::rust_fair_warn_put_prev_task_fair_1((*cfs_rq).curr != se);
        (*cfs_rq).curr = null_mut();
        if (*se).on_rq != 0 {
            __enqueue_entity(cfs_rq, se);
        }
    }
}
#[export_name = "rust_fair_yield_task_fair"]
unsafe extern "C" fn yield_task_fair(rq: *mut b::rq) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let se = addr_of_mut!((*(*rq).donor).se);
        let cfs_rq = addr_of_mut!((*rq).cfs);
        if (*rq).nr_running == 1 {
            return;
        }
        clear_buddies(cfs_rq, se);
        b::update_rq_clock(rq);
        update_curr(cfs_rq);
        b::rust_fair_rq_clock_skip_update(rq);
        if entity_eligible(cfs_rq, se) != 0 {
            (*se).vruntime = (*se).deadline;
            update_deadline(cfs_rq, se);
        }
    }
}
#[export_name = "rust_fair_yield_to_task_fair"]
unsafe extern "C" fn yield_to_task_fair(rq: *mut b::rq, p: *mut b::task_struct) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let se = addr_of_mut!((*p).se);
        if (*se).on_rq == 0 || ((*se).sched_delayed != 0) {
            return false;
        }
        set_next_buddy(addr_of_mut!((*b::rust_fair_task_rq(p)).cfs), se);
        yield_task_fair(rq);
        true
    }
}
#[link_section = ".data..read_mostly"]
static mut max_load_balance_interval: c_ulong = b::RUST_FAIR_HZ as c_ulong / 10;
unsafe fn task_hot(p: *mut b::task_struct, env: *mut b::lb_env) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_lockdep_assert_rq_held((*env).src_rq);
        if (*p).sched_class != addr_of!(b::fair_sched_class)
            || b::rust_fair_task_has_idle_policy(p)
            || (*(*env).sd).flags & b::RUST_FAIR_SD_SHARE_CPUCAPACITY != 0
        {
            return false;
        }
        if feat!(rust_fair_feat_CACHE_HOT_BUDDY)
            && (*(*env).dst_rq).nr_running != 0
            && addr_of_mut!((*p).se) == (*b::rust_fair_cfs_rq_of(addr_of_mut!((*p).se))).next
        {
            return true;
        }
        if sysctl_sched_migration_cost == c_uint::MAX {
            return true;
        }
        if !b::rust_fair_sched_core_cookie_match(b::rust_fair_cpu_rq((*env).dst_cpu), p) {
            return true;
        }
        if sysctl_sched_migration_cost == 0 {
            return false;
        }
        let delta = b::rust_fair_rq_clock_task((*env).src_rq).wrapping_sub((*p).se.exec_start) as i64;
        delta < sysctl_sched_migration_cost as i64
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn migrate_degrades_locality(p: *mut b::task_struct, env: *mut b::lb_env) -> c_long {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let numa_group = b::rust_fair_rcu_task_numa_group(p);
        if !b::rust_fair_sched_numa_balancing()
            || (*p).numa_faults.is_null()
            || (*(*env).sd).flags & b::RUST_FAIR_SD_NUMA == 0
        {
            return 0;
        }
        let src_nid = b::rust_fair_cpu_to_node((*env).src_cpu);
        let dst_nid = b::rust_fair_cpu_to_node((*env).dst_cpu);
        if src_nid == dst_nid {
            return 0;
        }
        if src_nid == (*p).numa_preferred_nid {
            return ((*(*env).src_rq).nr_running > (*(*env).src_rq).nr_preferred_running) as c_long;
        }
        if dst_nid == (*p).numa_preferred_nid {
            return -1;
        }
        if (*env).idle == b::CPU_IDLE {
            return 0;
        }
        let dist = b::rust_fair_node_distance(src_nid, dst_nid);
        let (src_weight, dst_weight) = if !numa_group.is_null() {
            (
                group_weight(p, src_nid, dist),
                group_weight(p, dst_nid, dist),
            )
        } else {
            (task_weight(p, src_nid, dist), task_weight(p, dst_nid, dist))
        };
        src_weight.wrapping_sub(dst_weight) as c_long
    }
}
#[cfg(not(CONFIG_NUMA_BALANCING))]
unsafe fn migrate_degrades_locality(_p: *mut b::task_struct, _env: *mut b::lb_env) -> c_long {
    0
}
#[inline]
unsafe fn task_is_ineligible_on_dst_cpu(p: *mut b::task_struct, dest_cpu: c_int) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let dst_cfs_rq = addr_of_mut!((*b::rust_fair_cpu_rq(dest_cpu)).cfs);
        feat!(rust_fair_feat_PLACE_LAG)
            && (*dst_cfs_rq).h_nr_queued != 0
            && entity_eligible(
                addr_of_mut!((*b::rust_fair_task_rq(p)).cfs),
                addr_of_mut!((*p).se),
            ) == 0
    }
}
#[cfg(CONFIG_SCHED_CACHE)]
unsafe fn fits_llc_capacity(util: c_ulong, capacity: c_ulong) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut aggr_pct = read_once!(llc_overaggr_pct);
        if b::rust_fair_cpu_smt_num_threads() == 1 {
            aggr_pct = aggr_pct.wrapping_mul(3) / 2;
        }
        util.wrapping_mul(100) < capacity.wrapping_mul(aggr_pct as c_ulong)
    }
}
#[cfg(CONFIG_SCHED_CACHE)]
unsafe fn fair_util_greater(util1: c_ulong, util2: c_ulong) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        util1.wrapping_mul(100)
            > util2.wrapping_mul(100u32.wrapping_add(read_once!(llc_imb_pct)) as c_ulong)
    }
}
#[cfg(CONFIG_SCHED_CACHE)]
unsafe fn get_llc_stats(cpu: c_int, util: *mut c_ulong, cap: *mut c_ulong) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let sd_share = b::rust_fair_rcu_sd_llc_shared(cpu);
        if sd_share.is_null() {
            return false;
        }
        *util = read_once!((*sd_share).util_avg);
        *cap = read_once!((*sd_share).capacity);
        true
    }
}
#[cfg(CONFIG_SCHED_CACHE)]
unsafe fn can_migrate_llc(
    src_cpu: c_int,
    dst_cpu: c_int,
    tsk_util: c_ulong,
    to_pref: bool,
) -> b::llc_mig {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let (mut src_util, mut dst_util, mut src_cap, mut dst_cap) = (0, 0, 0, 0);
        if !get_llc_stats(src_cpu, &mut src_util, &mut src_cap)
            || !get_llc_stats(dst_cpu, &mut dst_util, &mut dst_cap)
        {
            return b::mig_unrestricted;
        }
        src_util = src_util.saturating_sub(tsk_util);
        dst_util = dst_util.wrapping_add(tsk_util);
        if !fits_llc_capacity(dst_util, dst_cap) && !fits_llc_capacity(src_util, src_cap) {
            return b::mig_unrestricted;
        }
        if to_pref {
            if !fits_llc_capacity(dst_util, dst_cap) && fair_util_greater(dst_util, src_util) {
                return b::mig_forbid;
            }
        } else if fits_llc_capacity(src_util, src_cap) || !fair_util_greater(src_util, dst_util) {
            return b::mig_forbid;
        }
        b::mig_llc
    }
}
#[cfg(CONFIG_SCHED_CACHE)]
unsafe fn can_migrate_llc_task(
    src_cpu: c_int,
    dst_cpu: c_int,
    p: *mut b::task_struct,
) -> b::llc_mig {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mm = (*p).mm;
        if mm.is_null() {
            return b::mig_unrestricted;
        }
        let cpu = read_once!((*mm).sc_stat.cpu);
        if cpu < 0 || b::rust_fair_cpus_share_cache(src_cpu, dst_cpu) {
            return b::mig_unrestricted;
        }
        if invalid_llc_nr(mm, p, dst_cpu) || exceed_llc_capacity(mm, dst_cpu) {
            if read_once!((*mm).sc_stat.cpu) != -1 {
                write_once!((*mm).sc_stat.cpu, -1);
            }
            return b::mig_unrestricted;
        }
        let to_pref = if b::rust_fair_cpus_share_cache(dst_cpu, cpu) {
            true
        } else if b::rust_fair_cpus_share_cache(src_cpu, cpu) {
            false
        } else {
            return b::mig_unrestricted;
        };
        can_migrate_llc(src_cpu, dst_cpu, task_util(p), to_pref)
    }
}
#[cfg(CONFIG_SCHED_CACHE)]
unsafe fn alb_break_llc(env: *mut b::lb_env) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if !b::rust_fair_sched_cache_enabled()
            || b::rust_fair_cpus_share_cache((*env).src_cpu, (*env).dst_cpu)
        {
            return false;
        }
        let src_rq = (*env).src_rq;
        if (*src_rq).nr_pref_llc_running != 0
            && (*src_rq).nr_pref_llc_running == (*src_rq).cfs.h_nr_runnable
        {
            if (*src_rq).nr_running <= 1 {
                return true;
            }
            let cur = b::rust_fair_rcu_rq_curr(src_rq);
            let util = if !cur.is_null() && (*cur).sched_class == addr_of!(b::fair_sched_class) {
                task_util(cur)
            } else {
                0
            };
            if can_migrate_llc((*env).src_cpu, (*env).dst_cpu, util, false) == b::mig_forbid {
                return true;
            }
        }
        false
    }
}
#[cfg(CONFIG_SCHED_CACHE)]
unsafe fn migrate_degrades_llc(p: *mut b::task_struct, env: *mut b::lb_env) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if !b::rust_fair_sched_cache_enabled()
            || b::rust_fair_task_has_sched_core(p)
            || (*(*env).sd).nr_balance_failed >= (*(*env).sd).cache_nice_tries.wrapping_add(1)
        {
            return false;
        }
        if (*env).migration_type == b::migrate_llc_task
            && read_once!((*p).preferred_llc) != llc_id((*env).dst_cpu)
        {
            return true;
        }
        can_migrate_llc_task((*env).src_cpu, (*env).dst_cpu, p) == b::mig_forbid
    }
}
#[cfg(not(CONFIG_SCHED_CACHE))]
unsafe fn get_llc_stats(_cpu: c_int, _util: *mut c_ulong, _cap: *mut c_ulong) -> bool {
    false
}
#[cfg(not(CONFIG_SCHED_CACHE))]
unsafe fn alb_break_llc(_env: *mut b::lb_env) -> bool {
    false
}
#[cfg(not(CONFIG_SCHED_CACHE))]
unsafe fn migrate_degrades_llc(_p: *mut b::task_struct, _env: *mut b::lb_env) -> bool {
    false
}
unsafe fn can_migrate_task(p: *mut b::task_struct, env: *mut b::lb_env) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_lockdep_assert_rq_held((*env).src_rq);
        if b::rust_fair_task_sched_task_hot(p) {
            b::rust_fair_task_set_sched_task_hot(p, false);
        }
        if ((*p).se.sched_delayed != 0) && (*env).migration_type != b::migrate_load {
            return false;
        }
        if lb_throttled_hierarchy(p, (*env).dst_cpu) != 0 {
            return false;
        }
        if (*(*env).sd).nr_balance_failed == 0 && task_is_ineligible_on_dst_cpu(p, (*env).dst_cpu) {
            return false;
        }
        if b::rust_fair_kthread_is_per_cpu(p) || b::rust_fair_task_is_blocked(p) {
            return false;
        }
        if !b::rust_fair_cpumask_test_cpu((*env).dst_cpu, (*p).cpus_ptr) {
            b::rust_fair_stat_failed_migrations_affine(p);
            (*env).flags |= b::RUST_FAIR_LBF_SOME_PINNED as c_uint;
            if (*env).idle == b::CPU_NEWLY_IDLE
                || (*env).flags & ((b::RUST_FAIR_LBF_DST_PINNED | b::RUST_FAIR_LBF_ACTIVE_LB) as c_uint) != 0
            {
                return false;
            }
            let cpu = b::rust_fair_cpumask_first_and_and((*env).dst_grpmask, (*env).cpus, (*p).cpus_ptr)
                as c_int;
            if (cpu as c_uint) < b::rust_fair_nr_cpu_ids() {
                (*env).flags |= b::RUST_FAIR_LBF_DST_PINNED as c_uint;
                (*env).new_dst_cpu = cpu;
            }
            return false;
        }
        (*env).flags &= !(b::RUST_FAIR_LBF_ALL_PINNED as c_uint);
        if b::rust_fair_task_on_cpu((*env).src_rq, p)
            || b::rust_fair_task_current_donor((*env).src_rq, p)
        {
            b::rust_fair_stat_failed_migrations_running(p);
            return false;
        }
        if (*env).flags & (b::RUST_FAIR_LBF_ACTIVE_LB as c_uint) != 0 {
            return true;
        }
        let degrades = migrate_degrades_locality(p, env);
        let hot = if degrades == 0 {
            if migrate_degrades_llc(p, env) {
                if (*env).migration_type != b::migrate_llc_task {
                    (*env).flags |= b::RUST_FAIR_LBF_LLC_PINNED as c_uint;
                }
                return false;
            }
            task_hot(p, env)
        } else {
            degrades > 0
        };
        if !hot || (*(*env).sd).nr_balance_failed > (*(*env).sd).cache_nice_tries {
            if hot {
                b::rust_fair_task_set_sched_task_hot(p, true);
            }
            return true;
        }
        b::rust_fair_stat_failed_migrations_hot(p);
        false
    }
}
unsafe fn detach_task(p: *mut b::task_struct, env: *mut b::lb_env) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_lockdep_assert_rq_held((*env).src_rq);
        if b::rust_fair_task_sched_task_hot(p) {
            b::rust_fair_task_set_sched_task_hot(p, false);
            b::rust_fair_stat_lb_hot_gained((*env).sd, (*env).idle);
            b::rust_fair_stat_forced_migrations(p);
        }
        // These are WARN_ON, not WARN_ON_ONCE, in the original source.
        b::rust_fair_warn_detach_task_1(b::rust_fair_task_current((*env).src_rq, p));
        b::rust_fair_warn_detach_task_2(b::rust_fair_task_current_donor((*env).src_rq, p));
        b::deactivate_task((*env).src_rq, p, b::RUST_FAIR_DEQUEUE_NOCLOCK);
        b::set_task_cpu(p, (*env).dst_cpu);
    }
}
unsafe fn detach_one_task(env: *mut b::lb_env) -> *mut b::task_struct {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_lockdep_assert_rq_held((*env).src_rq);
        let head = addr_of_mut!((*(*env).src_rq).cfs_tasks);
        let mut node = (*head).prev;
        while node != head {
            let p = b::rust_fair_task_from_group_node(node);
            if can_migrate_task(p, env) {
                detach_task(p, env);
                b::rust_fair_stat_lb_gained((*env).sd, (*env).idle, 1);
                return p;
            }
            node = (*node).prev;
        }
        null_mut()
    }
}
unsafe fn detach_tasks(env: *mut b::lb_env) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let tasks = addr_of_mut!((*(*env).src_rq).cfs_tasks);
        let mut detached: c_int = 0;
        b::rust_fair_lockdep_assert_rq_held((*env).src_rq);
        if (*(*env).src_rq).nr_running <= 1 {
            (*env).flags &= !(b::RUST_FAIR_LBF_ALL_PINNED as c_uint);
            return 0;
        }
        if (*env).imbalance <= 0 {
            return 0;
        }
        while !b::rust_fair_list_empty(tasks) {
            if (*env).idle != 0 && (*(*env).src_rq).nr_running <= 1 {
                break;
            }
            (*env).loop_ = (*env).loop_.wrapping_add(1);
            if (*env).loop_ > (*env).loop_max {
                break;
            }
            if (*env).loop_ > (*env).loop_break {
                (*env).loop_break = (*env)
                    .loop_break
                    .wrapping_add(b::RUST_FAIR_SCHED_NR_MIGRATE_BREAK as c_uint);
                (*env).flags |= b::RUST_FAIR_LBF_NEED_BREAK as c_uint;
                break;
            }
            let p = b::rust_fair_task_from_group_node((*tasks).prev);
            let migrate = 'candidate: {
                if !can_migrate_task(p, env) {
                    break 'candidate false;
                }
                match (*env).migration_type {
                    b::migrate_load => {
                        let load = max(task_h_load(p), 1);
                        if feat!(rust_fair_feat_LB_MIN)
                            && load < 16
                            && (*(*env).sd).nr_balance_failed == 0
                        {
                            break 'candidate false;
                        }
                        if b::rust_fair_shr_bound(load, (*(*env).sd).nr_balance_failed)
                            > (*env).imbalance as c_ulong
                        {
                            break 'candidate false;
                        }
                        (*env).imbalance = (*env).imbalance.wrapping_sub(load as c_long);
                    }
                    b::migrate_util => {
                        let util = task_util_est(p);
                        if b::rust_fair_shr_bound(util, (*(*env).sd).nr_balance_failed)
                            > (*env).imbalance as c_ulong
                        {
                            break 'candidate false;
                        }
                        (*env).imbalance = (*env).imbalance.wrapping_sub(util as c_long);
                    }
                    b::migrate_task | b::migrate_llc_task => {
                        (*env).imbalance = (*env).imbalance.wrapping_sub(1);
                    }
                    b::migrate_misfit => {
                        if task_fits_cpu(p, (*env).src_cpu) != 0 {
                            break 'candidate false;
                        }
                        (*env).imbalance = 0;
                    }
                    _ => {}
                }
                true
            };
            if !migrate {
                if b::rust_fair_task_sched_task_hot(p) {
                    b::rust_fair_stat_failed_migrations_hot(p);
                }
                b::rust_fair_list_move(addr_of_mut!((*p).se.group_node), tasks);
                continue;
            }
            detach_task(p, env);
            b::rust_fair_list_add(addr_of_mut!((*p).se.group_node), addr_of_mut!((*env).tasks));
            detached = detached.wrapping_add(1);
            #[cfg(CONFIG_PREEMPTION)]
            if (*env).idle == b::CPU_NEWLY_IDLE {
                break;
            }
            if (*env).imbalance <= 0 {
                break;
            }
        }
        b::rust_fair_stat_lb_gained((*env).sd, (*env).idle, detached as c_uint);
        detached
    }
}
unsafe fn attach_tasks(env: *mut b::lb_env) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let tasks = addr_of_mut!((*env).tasks);
        let mut rf: b::rq_flags = core::mem::zeroed();
        b::rust_fair_rq_lock((*env).dst_rq, &mut rf);
        b::update_rq_clock((*env).dst_rq);
        while !b::rust_fair_list_empty(tasks) {
            let p = b::rust_fair_task_from_group_node((*tasks).next);
            b::rust_fair_list_del_init(addr_of_mut!((*p).se.group_node));
            b::rust_fair_attach_task((*env).dst_rq, p);
        }
        b::rust_fair_rq_unlock((*env).dst_rq, &mut rf);
    }
}
#[cfg(CONFIG_NO_HZ_COMMON)]
unsafe fn cfs_rq_has_blocked_load(cfs_rq: *mut b::cfs_rq) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (*cfs_rq).avg.load_avg != 0 || (*cfs_rq).avg.util_avg != 0
    }
}
#[cfg(CONFIG_NO_HZ_COMMON)]
unsafe fn others_have_blocked(rq: *mut b::rq) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_cpu_util_rt(rq) != 0
            || b::rust_fair_cpu_util_dl(rq) != 0
            || b::rust_fair_hw_load_avg(rq) != 0
            || b::rust_fair_cpu_util_irq(rq) != 0
    }
}
#[cfg(CONFIG_NO_HZ_COMMON)]
unsafe fn update_blocked_load_tick(rq: *mut b::rq) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        write_once!((*rq).last_blocked_load_update_tick, b::rust_fair_jiffies());
    }
}
#[cfg(CONFIG_NO_HZ_COMMON)]
unsafe fn update_has_blocked_load_status(rq: *mut b::rq, has_blocked_load: bool) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if !has_blocked_load {
            (*rq).has_blocked_load = 0;
        }
    }
}
#[cfg(not(CONFIG_NO_HZ_COMMON))]
unsafe fn cfs_rq_has_blocked_load(_cfs_rq: *mut b::cfs_rq) -> bool {
    false
}
#[cfg(not(CONFIG_NO_HZ_COMMON))]
unsafe fn others_have_blocked(_rq: *mut b::rq) -> bool {
    false
}
#[cfg(not(CONFIG_NO_HZ_COMMON))]
unsafe fn update_blocked_load_tick(_rq: *mut b::rq) {}
#[cfg(not(CONFIG_NO_HZ_COMMON))]
unsafe fn update_has_blocked_load_status(_rq: *mut b::rq, _has_blocked_load: bool) {}
unsafe fn __update_blocked_others(rq: *mut b::rq, done: *mut bool) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let updated = b::rust_fair_update_other_load_avgs(rq);
        if others_have_blocked(rq) {
            *done = false;
        }
        updated
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
unsafe fn __update_blocked_fair(rq: *mut b::rq, done: *mut bool) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let head = addr_of_mut!((*rq).leaf_cfs_rq_list);
        let mut node = (*head).next;
        let mut decayed = false;
        while node != head {
            let cfs_rq = b::rust_fair_cfs_rq_from_leaf(node);
            node = (*node).next;
            if update_cfs_rq_load_avg(b::rust_fair_cfs_rq_clock_pelt(cfs_rq), cfs_rq) != 0 {
                update_tg_load_avg(cfs_rq);
                if (*cfs_rq).nr_queued == 0 {
                    b::rust_fair_update_idle_cfs_rq_clock_pelt(cfs_rq);
                }
                if cfs_rq == addr_of_mut!((*rq).cfs) {
                    decayed = true;
                }
            }
            let se = b::rust_fair_cfs_rq_se(cfs_rq);
            if !se.is_null() && !skip_blocked_update(se) {
                update_load_avg(b::rust_fair_cfs_rq_of(se), se, b::RUST_FAIR_UPDATE_TG);
            }
            if cfs_rq_is_decayed(cfs_rq) {
                list_del_leaf_cfs_rq(cfs_rq);
            }
            if cfs_rq_has_blocked_load(cfs_rq) {
                *done = false;
            }
        }
        decayed
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
unsafe fn update_cfs_rq_h_load(mut cfs_rq: *mut b::cfs_rq) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut se = b::rust_fair_cfs_rq_se(cfs_rq);
        let now = b::rust_fair_jiffies();
        if (*cfs_rq).last_h_load_update == now as u64 {
            return;
        }
        write_once!((*cfs_rq).h_load_next, null_mut());
        while !se.is_null() {
            cfs_rq = b::rust_fair_cfs_rq_of(se);
            write_once!((*cfs_rq).h_load_next, se);
            if (*cfs_rq).last_h_load_update == now as u64 {
                break;
            }
            se = parent_entity(se);
        }
        if se.is_null() {
            (*cfs_rq).h_load = cfs_rq_load_avg(cfs_rq);
            (*cfs_rq).last_h_load_update = now as u64;
        }
        loop {
            se = read_once!((*cfs_rq).h_load_next);
            if se.is_null() {
                break;
            }
            // Multiplication is unsigned long in C before div64_ul widens it.
            let load = b::rust_fair_div64_ul(
                (*cfs_rq).h_load.wrapping_mul((*se).avg.load_avg as c_ulong) as u64,
                cfs_rq_load_avg(cfs_rq).wrapping_add(1),
            );
            cfs_rq = b::rust_fair_group_cfs_rq(se);
            (*cfs_rq).h_load = load as c_ulong;
            (*cfs_rq).last_h_load_update = now as u64;
        }
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
unsafe fn task_h_load(p: *mut b::task_struct) -> c_ulong {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let cfs_rq = b::rust_fair_task_cfs_rq(p);
        update_cfs_rq_h_load(cfs_rq);
        b::rust_fair_div64_ul(
            ((*p).se.avg.load_avg as c_ulong).wrapping_mul((*cfs_rq).h_load) as u64,
            cfs_rq_load_avg(cfs_rq).wrapping_add(1),
        ) as c_ulong
    }
}
#[cfg(not(CONFIG_FAIR_GROUP_SCHED))]
unsafe fn __update_blocked_fair(rq: *mut b::rq, done: *mut bool) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let cfs_rq = addr_of_mut!((*rq).cfs);
        let decayed = update_cfs_rq_load_avg(b::rust_fair_cfs_rq_clock_pelt(cfs_rq), cfs_rq) != 0;
        if cfs_rq_has_blocked_load(cfs_rq) {
            *done = false;
        }
        decayed
    }
}
#[cfg(not(CONFIG_FAIR_GROUP_SCHED))]
unsafe fn task_h_load(p: *mut b::task_struct) -> c_ulong {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (*p).se.avg.load_avg as c_ulong
    }
}
unsafe fn __sched_balance_update_blocked_averages(rq: *mut b::rq) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut done = true;
        update_blocked_load_tick(rq);
        let mut decayed = __update_blocked_others(rq, &mut done);
        decayed |= __update_blocked_fair(rq, &mut done);
        update_has_blocked_load_status(rq, !done);
        if decayed {
            b::rust_fair_cpufreq_update_util(rq, 0);
        }
    }
}
unsafe fn sched_balance_update_blocked_averages(cpu: c_int) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let rq = b::rust_fair_cpu_rq(cpu);
        let mut rf: b::rq_flags = core::mem::zeroed();
        b::rust_fair_rq_lock_irqsave(rq, &mut rf);
        b::update_rq_clock(rq);
        __sched_balance_update_blocked_averages(rq);
        b::rust_fair_rq_unlock_irqrestore(rq, &mut rf);
    }
}
#[inline]
unsafe fn init_sd_lb_stats(sds: *mut b::sd_lb_stats) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        core::ptr::write_bytes(sds, 0, 1);
        (*sds).busiest_stat.idle_cpus = c_uint::MAX;
        (*sds).busiest_stat.group_type = b::group_has_spare;
    }
}
unsafe fn scale_rt_capacity(cpu: c_int) -> c_ulong {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let capacity = get_actual_cpu_capacity(cpu);
        let rq = b::rust_fair_cpu_rq(cpu);
        let irq = b::rust_fair_cpu_util_irq(rq);
        if irq >= capacity {
            return 1;
        }
        let used = b::rust_fair_cpu_util_rt(rq).wrapping_add(b::rust_fair_cpu_util_dl(rq));
        if used >= capacity {
            return 1;
        }
        b::rust_fair_scale_irq_capacity(capacity - used, irq, capacity)
    }
}
unsafe fn update_cpu_capacity(sd: *mut b::sched_domain, cpu: c_int) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let capacity = max(scale_rt_capacity(cpu), 1);
        let sdg = (*sd).groups;
        let rq = b::rust_fair_cpu_rq(cpu);
        (*rq).cpu_capacity = capacity;
        b::rust_fair_trace_sched_cpu_capacity_tp(rq);
        (*(*sdg).sgc).capacity = capacity;
        (*(*sdg).sgc).min_capacity = capacity;
        (*(*sdg).sgc).max_capacity = capacity;
    }
}
#[no_mangle]
pub unsafe extern "C" fn update_group_capacity(sd: *mut b::sched_domain, cpu: c_int) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let child = (*sd).child;
        let sdg = (*sd).groups;
        let interval = fair_clamp_ulong(
            b::rust_fair_msecs_to_jiffies((*sd).balance_interval as c_uint),
            1,
            max_load_balance_interval,
        );
        (*(*sdg).sgc).next_update = b::rust_fair_jiffies().wrapping_add(interval);
        if child.is_null() {
            update_cpu_capacity(sd, cpu);
            return;
        }
        let mut capacity: c_ulong = 0;
        let mut min_capacity = c_ulong::MAX;
        let mut max_capacity = 0;
        if (*child).flags & b::RUST_FAIR_SD_NUMA != 0 {
            fair_each_cpu!(cpu, b::rust_fair_sched_group_span(sdg), {
                let cpu_cap = capacity_of(cpu);
                capacity = capacity.wrapping_add(cpu_cap);
                min_capacity = min(cpu_cap, min_capacity);
                max_capacity = max(cpu_cap, max_capacity);
            });
        } else {
            let mut group = (*child).groups;
            loop {
                let sgc = (*group).sgc;
                capacity = capacity.wrapping_add((*sgc).capacity);
                min_capacity = min((*sgc).min_capacity, min_capacity);
                max_capacity = max((*sgc).max_capacity, max_capacity);
                group = (*group).next;
                if group == (*child).groups {
                    break;
                }
            }
        }
        (*(*sdg).sgc).capacity = capacity;
        (*(*sdg).sgc).min_capacity = min_capacity;
        (*(*sdg).sgc).max_capacity = max_capacity;
    }
}
#[inline]
unsafe fn check_cpu_capacity(rq: *mut b::rq, sd: *mut b::sched_domain) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (*rq)
            .cpu_capacity
            .wrapping_mul((*sd).imbalance_pct as c_ulong)
            < b::rust_fair_arch_scale_cpu_capacity(b::rust_fair_cpu_of(rq)).wrapping_mul(100)
    }
}
#[inline]
unsafe fn check_misfit_status(rq: *mut b::rq) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (*rq).misfit_task_load != 0
    }
}
#[inline]
unsafe fn sg_imbalanced(group: *mut b::sched_group) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (*(*group).sgc).imbalance != 0
    }
}
#[inline]
unsafe fn group_has_capacity(imbalance_pct: c_uint, sgs: *mut b::sg_lb_stats) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if (*sgs).sum_nr_running < (*sgs).group_weight {
            return true;
        }
        if (*sgs).group_capacity.wrapping_mul(imbalance_pct as c_ulong)
            < (*sgs).group_runnable.wrapping_mul(100)
        {
            return false;
        }
        (*sgs).group_capacity.wrapping_mul(100)
            > (*sgs).group_util.wrapping_mul(imbalance_pct as c_ulong)
    }
}
#[inline]
unsafe fn group_is_overloaded(imbalance_pct: c_uint, sgs: *mut b::sg_lb_stats) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if b::rust_fair_sched_energy_enabled() && (*sgs).group_overutilized == 0 {
            return false;
        }
        if (*sgs).sum_nr_running <= (*sgs).group_weight {
            return false;
        }
        (*sgs).group_capacity.wrapping_mul(100)
            < (*sgs).group_util.wrapping_mul(imbalance_pct as c_ulong)
            || (*sgs).group_capacity.wrapping_mul(imbalance_pct as c_ulong)
                < (*sgs).group_runnable.wrapping_mul(100)
    }
}
#[inline]
unsafe fn group_classify(
    imbalance_pct: c_uint,
    group: *mut b::sched_group,
    sgs: *mut b::sg_lb_stats,
) -> b::group_type {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if group_is_overloaded(imbalance_pct, sgs) {
            b::group_overloaded
        } else if (*sgs).group_llc_balance != 0 {
            b::group_llc_balance
        } else if sg_imbalanced(group) {
            b::group_imbalanced
        } else if (*sgs).group_asym_packing != 0 {
            b::group_asym_packing
        } else if (*sgs).group_smt_balance != 0 {
            b::group_smt_balance
        } else if (*sgs).group_misfit_task_load != 0 {
            b::group_misfit_task
        } else if !group_has_capacity(imbalance_pct, sgs) {
            b::group_fully_busy
        } else {
            b::group_has_spare
        }
    }
}
unsafe fn sched_use_asym_prio(sd: *mut b::sched_domain, cpu: c_int) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if (*sd).flags & b::RUST_FAIR_SD_ASYM_PACKING == 0 {
            return false;
        }
        if !b::rust_fair_sched_smt_active() {
            return true;
        }
        (*sd).flags & b::RUST_FAIR_SD_SHARE_CPUCAPACITY != 0 || is_core_idle(cpu)
    }
}
#[inline]
unsafe fn sched_asym(sd: *mut b::sched_domain, dst_cpu: c_int, src_cpu: c_int) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        sched_use_asym_prio(sd, dst_cpu) && b::rust_fair_sched_asym_prefer(dst_cpu, src_cpu)
    }
}
#[inline]
unsafe fn sched_group_asym(
    env: *mut b::lb_env,
    sgs: *mut b::sg_lb_stats,
    group: *mut b::sched_group,
) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if (*group).flags & b::RUST_FAIR_SD_SHARE_CPUCAPACITY != 0
            && (*sgs).group_weight.wrapping_sub((*sgs).idle_cpus) != 1
        {
            return false;
        }
        sched_asym(
            (*env).sd,
            (*env).dst_cpu,
            read_once!((*group).asym_prefer_cpu),
        )
    }
}
#[inline]
unsafe fn smt_vs_nonsmt_groups(sg1: *mut b::sched_group, sg2: *mut b::sched_group) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        !sg1.is_null()
            && !sg2.is_null()
            && ((*sg1).flags & b::RUST_FAIR_SD_SHARE_CPUCAPACITY)
                != ((*sg2).flags & b::RUST_FAIR_SD_SHARE_CPUCAPACITY)
    }
}
#[inline]
unsafe fn smt_balance(
    env: *mut b::lb_env,
    sgs: *mut b::sg_lb_stats,
    group: *mut b::sched_group,
) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (*env).idle != 0
            && (*group).flags & b::RUST_FAIR_SD_SHARE_CPUCAPACITY != 0
            && (*sgs).sum_h_nr_running > 1
    }
}
#[inline]
unsafe fn sibling_imbalance(
    env: *mut b::lb_env,
    sds: *mut b::sd_lb_stats,
    busiest: *mut b::sg_lb_stats,
    local: *mut b::sg_lb_stats,
) -> c_long {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if (*env).idle == 0 || (*busiest).sum_nr_running == 0 {
            return 0;
        }
        let ncores_busiest = (*(*sds).busiest).cores as c_int;
        let ncores_local = (*(*sds).local).cores as c_int;
        if ncores_busiest == ncores_local {
            let imbalance = (*busiest).sum_nr_running as c_long;
            return imbalance.wrapping_sub(min(imbalance, (*local).sum_nr_running as c_long));
        }
        // C int * unsigned int is unsigned int before assignment to long.
        let mut imbalance = (ncores_local as c_uint).wrapping_mul((*busiest).sum_nr_running) as c_long;
        imbalance = imbalance.wrapping_sub(min(
            imbalance,
            (ncores_busiest as c_uint).wrapping_mul((*local).sum_nr_running) as c_long,
        ));
        imbalance = imbalance
            .wrapping_mul(2)
            .wrapping_add(ncores_local as c_long)
            .wrapping_add(ncores_busiest as c_long);
        imbalance /= ncores_local.wrapping_add(ncores_busiest) as c_long;
        if imbalance <= 1 && (*local).sum_nr_running == 0 && (*busiest).sum_nr_running > 1 {
            imbalance = 2;
        }
        imbalance
    }
}
#[inline]
unsafe fn sched_reduced_capacity(rq: *mut b::rq, sd: *mut b::sched_domain) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (*rq).cfs.h_nr_runnable == 1 && check_cpu_capacity(rq, sd)
    }
}
#[cfg(CONFIG_SCHED_CACHE)]
unsafe fn record_sg_llc_stats(
    env: *mut b::lb_env,
    sgs: *mut b::sg_lb_stats,
    group: *mut b::sched_group,
) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if !b::rust_fair_sched_cache_enabled()
            || (*env).idle == b::CPU_NEWLY_IDLE
            || (*(*env).sd).child != b::rust_fair_rcu_sd_llc((*env).dst_cpu)
        {
            return;
        }
        let cpu = b::rust_fair_cpumask_first(b::rust_fair_sched_group_span(group)) as c_int;
        let sd_share = b::rust_fair_rcu_sd_llc_shared(cpu);
        if sd_share.is_null() {
            return;
        }
        if read_once!((*sd_share).util_avg) != (*sgs).group_util {
            write_once!((*sd_share).util_avg, (*sgs).group_util);
        }
        if read_once!((*sd_share).capacity) != (*sgs).group_capacity {
            write_once!((*sd_share).capacity, (*sgs).group_capacity);
        }
    }
}
#[cfg(CONFIG_SCHED_CACHE)]
unsafe fn llc_balance(
    env: *mut b::lb_env,
    sgs: *mut b::sg_lb_stats,
    group: *mut b::sched_group,
) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if !b::rust_fair_sched_cache_enabled()
            || (*(*env).sd).flags & b::RUST_FAIR_SD_SHARE_LLC != 0
            || (*(*env).sd).nr_balance_failed >= (*(*env).sd).cache_nice_tries.wrapping_add(1)
        {
            return false;
        }
        (*sgs).nr_pref_dst_llc != 0
            && can_migrate_llc(
                b::rust_fair_cpumask_first(b::rust_fair_sched_group_span(group)) as c_int,
                (*env).dst_cpu,
                0,
                true,
            ) == b::mig_llc
    }
}
#[cfg(CONFIG_SCHED_CACHE)]
unsafe fn update_llc_busiest(
    _env: *mut b::lb_env,
    busiest: *mut b::sg_lb_stats,
    sgs: *mut b::sg_lb_stats,
) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (*sgs).nr_pref_dst_llc > (*busiest).nr_pref_dst_llc
    }
}
#[cfg(not(CONFIG_SCHED_CACHE))]
unsafe fn record_sg_llc_stats(
    _env: *mut b::lb_env,
    _sgs: *mut b::sg_lb_stats,
    _group: *mut b::sched_group,
) {
}
#[cfg(not(CONFIG_SCHED_CACHE))]
unsafe fn llc_balance(
    _env: *mut b::lb_env,
    _sgs: *mut b::sg_lb_stats,
    _group: *mut b::sched_group,
) -> bool {
    false
}
#[cfg(not(CONFIG_SCHED_CACHE))]
unsafe fn update_llc_busiest(
    _env: *mut b::lb_env,
    _busiest: *mut b::sg_lb_stats,
    _sgs: *mut b::sg_lb_stats,
) -> bool {
    false
}
#[inline]
unsafe fn update_sg_lb_stats(
    env: *mut b::lb_env,
    sds: *mut b::sd_lb_stats,
    group: *mut b::sched_group,
    sgs: *mut b::sg_lb_stats,
    sg_overloaded: *mut bool,
) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let sd_flags = (*(*env).sd).flags;
        let balancing_at_rd = (*(*env).sd).parent.is_null();
        core::ptr::write_bytes(sgs, 0, 1);
        let local_group = group == (*sds).local;
        fair_each_cpu_and!(i, b::rust_fair_sched_group_span(group), (*env).cpus, {
            let rq = b::rust_fair_cpu_rq(i);
            let load = cpu_load(rq);
            (*sgs).group_load = (*sgs).group_load.wrapping_add(load);
            (*sgs).group_util = (*sgs).group_util.wrapping_add(cpu_util_cfs(i));
            (*sgs).group_runnable = (*sgs).group_runnable.wrapping_add(cpu_runnable(rq));
            (*sgs).sum_h_nr_running = (*sgs)
                .sum_h_nr_running
                .wrapping_add((*rq).cfs.h_nr_runnable);
            let nr_running = (*rq).nr_running as c_int;
            (*sgs).sum_nr_running = (*sgs).sum_nr_running.wrapping_add(nr_running as c_uint);
            if cpu_overutilized(i) {
                (*sgs).group_overutilized = 1;
            }
            #[cfg(CONFIG_SCHED_CACHE)]
            if b::rust_fair_sched_cache_enabled() {
                let dst_llc = llc_id((*env).dst_cpu);
                if llc_id(i) != dst_llc {
                    let sd_tmp = b::rust_fair_rcu_rq_sd(rq);
                    if !sd_tmp.is_null() && (dst_llc as c_uint) < (*sd_tmp).llc_max {
                        (*sgs).nr_pref_dst_llc = (*sgs)
                            .nr_pref_dst_llc
                            .wrapping_add(*(*sd_tmp).llc_counts.add(dst_llc as usize));
                    }
                }
            }
            if nr_running == 0 && b::rust_fair_idle_cpu(i) != 0 {
                (*sgs).idle_cpus = (*sgs).idle_cpus.wrapping_add(1);
                continue;
            }
            if balancing_at_rd && nr_running > 1 {
                *sg_overloaded = true;
            }
            #[cfg(CONFIG_NUMA_BALANCING)]
            if sd_flags & b::RUST_FAIR_SD_NUMA != 0 {
                (*sgs).nr_numa_running = (*sgs).nr_numa_running.wrapping_add((*rq).nr_numa_running);
                (*sgs).nr_preferred_running = (*sgs)
                    .nr_preferred_running
                    .wrapping_add((*rq).nr_preferred_running);
            }
            if local_group {
                continue;
            }
            if sd_flags & b::RUST_FAIR_SD_ASYM_CPUCAPACITY != 0 {
                if (*rq).misfit_task_load != 0 {
                    if balancing_at_rd {
                        *sg_overloaded = true;
                    }
                    if capacity_greater(capacity_of((*env).dst_cpu), (*(*group).sgc).max_capacity)
                        && (*sgs).group_misfit_task_load < (*rq).misfit_task_load
                    {
                        (*sgs).group_misfit_task_load = (*rq).misfit_task_load;
                    }
                }
            } else if (*env).idle != 0
                && sched_reduced_capacity(rq, (*env).sd)
                && (*sgs).group_misfit_task_load < load
            {
                (*sgs).group_misfit_task_load = load;
            }
        });
        (*sgs).group_capacity = (*(*group).sgc).capacity;
        (*sgs).group_weight = (*group).group_weight;
        if !local_group {
            if (*env).idle != 0 && (*sgs).sum_h_nr_running != 0 && sched_group_asym(env, sgs, group) {
                (*sgs).group_asym_packing = 1;
            }
            if smt_balance(env, sgs, group) {
                (*sgs).group_smt_balance = 1;
            }
            if llc_balance(env, sgs, group) {
                (*sgs).group_llc_balance = 1;
            }
        }
        (*sgs).group_type = group_classify((*(*env).sd).imbalance_pct, group, sgs);
        record_sg_llc_stats(env, sgs, group);
        if (*sgs).group_type == b::group_overloaded {
            (*sgs).avg_load = (*sgs)
                .group_load
                .wrapping_mul(b::RUST_FAIR_SCHED_CAPACITY_SCALE as c_ulong)
                / (*sgs).group_capacity;
        }
    }
}
unsafe fn update_sd_pick_busiest(
    env: *mut b::lb_env,
    sds: *mut b::sd_lb_stats,
    sg: *mut b::sched_group,
    sgs: *mut b::sg_lb_stats,
) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let busiest = addr_of_mut!((*sds).busiest_stat);
        if (*sgs).sum_h_nr_running == 0 {
            return false;
        }
        if (*(*env).sd).flags & b::RUST_FAIR_SD_ASYM_CPUCAPACITY != 0
            && (*sgs).group_type == b::group_misfit_task
            && (!(*env).dst_core_idle
                || !capacity_greater(capacity_of((*env).dst_cpu), (*(*sg).sgc).max_capacity)
                || (*sds).local_stat.group_type != b::group_has_spare)
        {
            return false;
        }
        if (*(*env).sd).flags & b::RUST_FAIR_SD_ASYM_CPUCAPACITY != 0
            && (*sgs).group_type <= b::group_fully_busy
            && capacity_greater((*(*sg).sgc).min_capacity, capacity_of((*env).dst_cpu))
        {
            return false;
        }
        if (*sgs).group_type > (*busiest).group_type {
            return true;
        }
        if (*sgs).group_type < (*busiest).group_type {
            return false;
        }
        match (*sgs).group_type {
            b::group_overloaded => return (*sgs).avg_load > (*busiest).avg_load,
            b::group_llc_balance => return update_llc_busiest(env, busiest, sgs),
            b::group_imbalanced => return false,
            b::group_asym_packing => {
                return b::rust_fair_sched_asym_prefer(
                    read_once!((*(*sds).busiest).asym_prefer_cpu),
                    read_once!((*sg).asym_prefer_cpu),
                )
            }
            b::group_misfit_task => {
                return (*sgs).group_misfit_task_load > (*busiest).group_misfit_task_load
            }
            b::group_smt_balance if (*sgs).idle_cpus != 0 || (*busiest).idle_cpus != 0 => {}
            b::group_smt_balance | b::group_fully_busy => {
                if (*sgs).avg_load < (*busiest).avg_load {
                    return false;
                }
                if (*sgs).avg_load == (*busiest).avg_load
                    && (*(*sds).busiest).flags & b::RUST_FAIR_SD_SHARE_CPUCAPACITY != 0
                {
                    return false;
                }
                return true;
            }
            b::group_has_spare => {
                if smt_vs_nonsmt_groups((*sds).busiest, sg) {
                    return !((*sg).flags & b::RUST_FAIR_SD_SHARE_CPUCAPACITY != 0
                        && (*sgs).sum_h_nr_running <= 1);
                }
            }
            _ => return true,
        }
        !((*sgs).idle_cpus > (*busiest).idle_cpus
            || ((*sgs).idle_cpus == (*busiest).idle_cpus
                && (*sgs).sum_nr_running <= (*busiest).sum_nr_running))
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn fbq_classify_group(sgs: *mut b::sg_lb_stats) -> b::fbq_type {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if (*sgs).sum_h_nr_running > (*sgs).nr_numa_running {
            b::regular
        } else if (*sgs).sum_h_nr_running > (*sgs).nr_preferred_running {
            b::remote
        } else {
            b::all
        }
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn fbq_classify_rq(rq: *mut b::rq) -> b::fbq_type {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if (*rq).nr_running > (*rq).nr_numa_running {
            b::regular
        } else if (*rq).nr_running > (*rq).nr_preferred_running {
            b::remote
        } else {
            b::all
        }
    }
}
#[cfg(not(CONFIG_NUMA_BALANCING))]
unsafe fn fbq_classify_group(_sgs: *mut b::sg_lb_stats) -> b::fbq_type {
    b::all
}
#[cfg(not(CONFIG_NUMA_BALANCING))]
unsafe fn fbq_classify_rq(_rq: *mut b::rq) -> b::fbq_type {
    b::regular
}
unsafe fn task_running_on_cpu(cpu: c_int, p: *mut b::task_struct) -> c_uint {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (cpu == b::rust_fair_task_cpu(p)
            && read_once!((*p).se.avg.last_update_time) != 0
            && b::rust_fair_task_on_rq_queued(p)) as c_uint
    }
}
unsafe fn idle_cpu_without(cpu: c_int, p: *mut b::task_struct) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let rq = b::rust_fair_cpu_rq(cpu);
        ((*rq).curr == (*rq).idle || (*rq).curr == p) && !b::rust_fair_rq_ttwu_pending(rq)
    }
}
#[inline]
unsafe fn update_sg_wakeup_stats(
    sd: *mut b::sched_domain,
    group: *mut b::sched_group,
    sgs: *mut b::sg_lb_stats,
    p: *mut b::task_struct,
) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        core::ptr::write_bytes(sgs, 0, 1);
        if (*sd).flags & b::RUST_FAIR_SD_ASYM_CPUCAPACITY != 0 {
            (*sgs).group_misfit_task_load = 1;
        }
        fair_each_cpu_and!(i, b::rust_fair_sched_group_span(group), (*p).cpus_ptr, {
            let rq = b::rust_fair_cpu_rq(i);
            (*sgs).group_load = (*sgs).group_load.wrapping_add(cpu_load_without(rq, p));
            (*sgs).group_util = (*sgs).group_util.wrapping_add(cpu_util_without(i, p));
            (*sgs).group_runnable = (*sgs)
                .group_runnable
                .wrapping_add(cpu_runnable_without(rq, p));
            let local = task_running_on_cpu(i, p);
            (*sgs).sum_h_nr_running = (*sgs)
                .sum_h_nr_running
                .wrapping_add((*rq).cfs.h_nr_runnable.wrapping_sub(local));
            let nr_running = (*rq).nr_running.wrapping_sub(local) as c_int;
            (*sgs).sum_nr_running = (*sgs).sum_nr_running.wrapping_add(nr_running as c_uint);
            if nr_running == 0 && idle_cpu_without(i, p) {
                (*sgs).idle_cpus = (*sgs).idle_cpus.wrapping_add(1);
            }
            if (*sd).flags & b::RUST_FAIR_SD_ASYM_CPUCAPACITY != 0
                && (*sgs).group_misfit_task_load != 0
                && task_fits_cpu(p, i) != 0
            {
                (*sgs).group_misfit_task_load = 0;
            }
        });
        (*sgs).group_capacity = (*(*group).sgc).capacity;
        (*sgs).group_weight = (*group).group_weight;
        (*sgs).group_type = group_classify((*sd).imbalance_pct, group, sgs);
        if (*sgs).group_type == b::group_fully_busy || (*sgs).group_type == b::group_overloaded {
            (*sgs).avg_load = (*sgs)
                .group_load
                .wrapping_mul(b::RUST_FAIR_SCHED_CAPACITY_SCALE as c_ulong)
                / (*sgs).group_capacity;
        }
    }
}
unsafe fn update_pick_idlest(
    idlest: *mut b::sched_group,
    idlest_sgs: *mut b::sg_lb_stats,
    group: *mut b::sched_group,
    sgs: *mut b::sg_lb_stats,
) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if (*sgs).group_type < (*idlest_sgs).group_type {
            return true;
        }
        if (*sgs).group_type > (*idlest_sgs).group_type {
            return false;
        }
        match (*sgs).group_type {
            b::group_overloaded | b::group_fully_busy => {
                if (*idlest_sgs).avg_load <= (*sgs).avg_load {
                    return false;
                }
            }
            b::group_llc_balance
            | b::group_imbalanced
            | b::group_asym_packing
            | b::group_smt_balance => return false,
            b::group_misfit_task => {
                if (*(*idlest).sgc).max_capacity >= (*(*group).sgc).max_capacity {
                    return false;
                }
            }
            b::group_has_spare => {
                if (*idlest_sgs).idle_cpus > (*sgs).idle_cpus
                    || ((*idlest_sgs).idle_cpus == (*sgs).idle_cpus
                        && (*idlest_sgs).group_util <= (*sgs).group_util)
                {
                    return false;
                }
            }
            _ => {}
        }
        true
    }
}
unsafe fn sched_balance_find_dst_group(
    sd: *mut b::sched_domain,
    p: *mut b::task_struct,
    this_cpu: c_int,
) -> *mut b::sched_group {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut idlest: *mut b::sched_group = null_mut();
        let mut local: *mut b::sched_group = null_mut();
        let mut group = (*sd).groups;
        let mut local_sgs: b::sg_lb_stats = core::mem::zeroed();
        let mut tmp_sgs: b::sg_lb_stats = core::mem::zeroed();
        let mut idlest_sgs: b::sg_lb_stats = core::mem::zeroed();
        idlest_sgs.avg_load = c_uint::MAX as c_ulong;
        idlest_sgs.group_type = b::group_overloaded;
        loop {
            if b::rust_fair_cpumask_intersects(b::rust_fair_sched_group_span(group), (*p).cpus_ptr)
                && b::rust_fair_sched_group_cookie_match(b::rust_fair_cpu_rq(this_cpu), p, group)
            {
                let local_group =
                    b::rust_fair_cpumask_test_cpu(this_cpu, b::rust_fair_sched_group_span(group));
                let sgs = if local_group {
                    local = group;
                    &mut local_sgs
                } else {
                    &mut tmp_sgs
                } as *mut b::sg_lb_stats;
                update_sg_wakeup_stats(sd, group, sgs, p);
                if !local_group && update_pick_idlest(idlest, &mut idlest_sgs, group, sgs) {
                    idlest = group;
                    core::ptr::copy_nonoverlapping(sgs, &mut idlest_sgs, 1);
                }
            }
            group = (*group).next;
            if group == (*sd).groups {
                break;
            }
        }
        if idlest.is_null() {
            return null_mut();
        }
        if local.is_null() {
            return idlest;
        }
        if local_sgs.group_type < idlest_sgs.group_type {
            return null_mut();
        }
        if local_sgs.group_type > idlest_sgs.group_type {
            return idlest;
        }
        match local_sgs.group_type {
            b::group_overloaded | b::group_fully_busy => {
                let imbalance = b::rust_fair_scale_load_down(b::RUST_FAIR_NICE_0_LOAD as c_ulong)
                    .wrapping_mul((*sd).imbalance_pct.wrapping_sub(100) as c_ulong)
                    / 100;
                if (*sd).flags & b::RUST_FAIR_SD_NUMA != 0
                    && idlest_sgs.avg_load.wrapping_add(imbalance) >= local_sgs.avg_load
                {
                    return null_mut();
                }
                if idlest_sgs.avg_load >= local_sgs.avg_load.wrapping_add(imbalance)
                    || local_sgs.avg_load.wrapping_mul(100)
                        <= idlest_sgs
                            .avg_load
                            .wrapping_mul((*sd).imbalance_pct as c_ulong)
                {
                    return null_mut();
                }
            }
            b::group_llc_balance
            | b::group_imbalanced
            | b::group_asym_packing
            | b::group_smt_balance => return null_mut(),
            b::group_misfit_task => {
                if (*(*local).sgc).max_capacity >= (*(*idlest).sgc).max_capacity {
                    return null_mut();
                }
            }
            b::group_has_spare => {
                #[cfg(CONFIG_NUMA)]
                if (*sd).flags & b::RUST_FAIR_SD_NUMA != 0 {
                    let mut imb_numa_nr = (*sd).imb_numa_nr as c_int;
                    #[cfg(CONFIG_NUMA_BALANCING)]
                    {
                        if b::rust_fair_cpu_to_node(this_cpu) == (*p).numa_preferred_nid {
                            return null_mut();
                        }
                        let idlest_cpu =
                            b::rust_fair_cpumask_first(b::rust_fair_sched_group_span(idlest)) as c_int;
                        if b::rust_fair_cpu_to_node(idlest_cpu) == (*p).numa_preferred_nid {
                            return idlest;
                        }
                    }
                    if (*p).nr_cpus_allowed != b::RUST_FAIR_NR_CPUS {
                        let w = b::rust_fair_cpumask_weight_and(
                            (*p).cpus_ptr,
                            b::rust_fair_sched_group_span(local),
                        );
                        imb_numa_nr = min(w, (*sd).imb_numa_nr as c_uint) as c_int;
                    }
                    let imbalance = (local_sgs.idle_cpus.wrapping_sub(idlest_sgs.idle_cpus) as c_int)
                        .wrapping_abs() as c_ulong;
                    if adjust_numa_imbalance(
                        imbalance as c_int,
                        local_sgs.sum_nr_running.wrapping_add(1) as _,
                        imb_numa_nr,
                    ) == 0
                    {
                        return null_mut();
                    }
                }
                if local_sgs.idle_cpus >= idlest_sgs.idle_cpus {
                    return null_mut();
                }
            }
            _ => {}
        }
        idlest
    }
}
unsafe fn update_idle_cpu_scan(env: *mut b::lb_env, sum_util: c_ulong) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let sd = (*env).sd;
        if !feat!(rust_fair_feat_SIS_UTIL) || (*env).idle == b::CPU_NEWLY_IDLE {
            return;
        }
        let sd_share = (*sd).shared;
        if sd_share.is_null() {
            return;
        }
        let llc_weight = (*sd).span_weight as c_int;
        let x = sum_util as u64 / llc_weight as u32 as u64;
        let pct = (*sd).imbalance_pct as c_int;
        let mut tmp = x
            .wrapping_mul(x)
            .wrapping_mul(pct as u64)
            .wrapping_mul(pct as u64);
        tmp /= (10000 * b::RUST_FAIR_SCHED_CAPACITY_SCALE) as u32 as u64;
        tmp = min(tmp as c_long, b::RUST_FAIR_SCHED_CAPACITY_SCALE as c_long) as u64;
        let mut y = (b::RUST_FAIR_SCHED_CAPACITY_SCALE as u64).wrapping_sub(tmp);
        y = y.wrapping_mul(llc_weight as u64) / b::RUST_FAIR_SCHED_CAPACITY_SCALE as u64;
        if y as c_int != b::rust_fair_sd_nr_idle_scan_plain(sd_share) {
            b::rust_fair_sd_set_nr_idle_scan(sd_share, y as c_int);
        }
    }
}
#[inline]
unsafe fn update_sd_lb_stats(env: *mut b::lb_env, sds: *mut b::sd_lb_stats) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut sg = (*(*env).sd).groups;
        let local = addr_of_mut!((*sds).local_stat);
        let mut tmp_sgs: b::sg_lb_stats = core::mem::zeroed();
        let mut sum_util: c_ulong = 0;
        let mut sg_overloaded = false;
        let mut sg_overutilized = false;
        (*env).dst_core_idle = !b::rust_fair_sched_smt_active() || is_core_idle((*env).dst_cpu);
        loop {
            let mut sgs = &mut tmp_sgs as *mut b::sg_lb_stats;
            let local_group =
                b::rust_fair_cpumask_test_cpu((*env).dst_cpu, b::rust_fair_sched_group_span(sg));
            if local_group {
                (*sds).local = sg;
                sgs = local;
                if (*env).idle != b::CPU_NEWLY_IDLE
                    || b::rust_fair_time_after_eq(b::rust_fair_jiffies(), (*(*sg).sgc).next_update)
                {
                    update_group_capacity((*env).sd, (*env).dst_cpu);
                }
            }
            update_sg_lb_stats(env, sds, sg, sgs, &mut sg_overloaded);
            if !local_group && update_sd_pick_busiest(env, sds, sg, sgs) {
                (*sds).busiest = sg;
                core::ptr::copy_nonoverlapping(sgs, addr_of_mut!((*sds).busiest_stat), 1);
            }
            sg_overutilized |= (*sgs).group_overutilized != 0;
            (*sds).total_load = (*sds).total_load.wrapping_add((*sgs).group_load);
            (*sds).total_capacity = (*sds).total_capacity.wrapping_add((*sgs).group_capacity);
            sum_util = sum_util.wrapping_add((*sgs).group_util);
            sg = (*sg).next;
            if sg == (*(*env).sd).groups {
                break;
            }
        }
        if !(*sds).busiest.is_null() {
            (*sds).prefer_sibling =
                ((*(*sds).busiest).flags & b::RUST_FAIR_SD_PREFER_SIBLING != 0) as c_uint;
        }
        if (*(*env).sd).flags & b::RUST_FAIR_SD_NUMA != 0 {
            (*env).fbq_type = fbq_classify_group(addr_of_mut!((*sds).busiest_stat));
        }
        if (*(*env).sd).parent.is_null() {
            b::rust_fair_set_rd_overloaded((*(*env).dst_rq).rd, sg_overloaded as c_int);
            set_rd_overutilized((*(*env).dst_rq).rd, sg_overutilized);
        } else if sg_overutilized {
            set_rd_overutilized((*(*env).dst_rq).rd, sg_overutilized);
        }
        update_idle_cpu_scan(env, sum_util);
    }
}
#[inline]
unsafe fn calculate_imbalance(env: *mut b::lb_env, sds: *mut b::sd_lb_stats) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let local = addr_of_mut!((*sds).local_stat);
        let busiest = addr_of_mut!((*sds).busiest_stat);
        if (*busiest).group_type == b::group_misfit_task {
            if (*(*env).sd).flags & b::RUST_FAIR_SD_ASYM_CPUCAPACITY != 0 {
                (*env).migration_type = b::migrate_misfit;
                (*env).imbalance = 1;
            } else {
                (*env).migration_type = b::migrate_load;
                (*env).imbalance = (*busiest).group_misfit_task_load as c_long;
            }
            return;
        }
        if (*busiest).group_type == b::group_asym_packing {
            (*env).migration_type = b::migrate_task;
            (*env).imbalance = (*busiest).sum_h_nr_running as c_long;
            return;
        }
        if (*busiest).group_type == b::group_smt_balance {
            (*env).migration_type = b::migrate_task;
            (*env).imbalance = 1;
            return;
        }
        #[cfg(CONFIG_SCHED_CACHE)]
        if (*busiest).group_type == b::group_llc_balance {
            (*env).migration_type = b::migrate_llc_task;
            (*env).imbalance = 1;
            return;
        }
        if (*busiest).group_type == b::group_imbalanced {
            (*env).migration_type = b::migrate_task;
            (*env).imbalance = 1;
            return;
        }
        if (*local).group_type == b::group_has_spare {
            if (*busiest).group_type > b::group_fully_busy
                && (*(*env).sd).flags & b::RUST_FAIR_SD_SHARE_LLC == 0
            {
                (*env).migration_type = b::migrate_util;
                (*env).imbalance =
                    (max((*local).group_capacity, (*local).group_util) - (*local).group_util) as c_long;
                if (*env).idle != 0 && (*env).imbalance == 0 {
                    (*env).migration_type = b::migrate_task;
                    (*env).imbalance = 1;
                }
                return;
            }
            (*env).migration_type = b::migrate_task;
            (*env).imbalance = if (*busiest).group_weight == 1 || (*sds).prefer_sibling != 0 {
                sibling_imbalance(env, sds, busiest, local)
            } else {
                max(
                    0,
                    (*local).idle_cpus.wrapping_sub((*busiest).idle_cpus) as c_long,
                )
            };
            #[cfg(CONFIG_NUMA)]
            if (*(*env).sd).flags & b::RUST_FAIR_SD_NUMA != 0 {
                (*env).imbalance = adjust_numa_imbalance(
                    (*env).imbalance as c_int,
                    (*local).sum_nr_running.wrapping_add(1) as _,
                    (*(*env).sd).imb_numa_nr as _,
                ) as c_long;
            }
            (*env).imbalance >>= 1;
            return;
        }
        if (*local).group_type < b::group_overloaded {
            (*local).avg_load = (*local)
                .group_load
                .wrapping_mul(b::RUST_FAIR_SCHED_CAPACITY_SCALE as c_ulong)
                / (*local).group_capacity;
            if (*local).avg_load >= (*busiest).avg_load {
                (*env).imbalance = 0;
                return;
            }
            (*sds).avg_load = (*sds)
                .total_load
                .wrapping_mul(b::RUST_FAIR_SCHED_CAPACITY_SCALE as c_ulong)
                / (*sds).total_capacity;
            if (*local).avg_load >= (*sds).avg_load {
                (*env).imbalance = 0;
                return;
            }
        }
        (*env).migration_type = b::migrate_load;
        (*env).imbalance = (min(
            (*busiest)
                .avg_load
                .wrapping_sub((*sds).avg_load)
                .wrapping_mul((*busiest).group_capacity),
            (*sds)
                .avg_load
                .wrapping_sub((*local).avg_load)
                .wrapping_mul((*local).group_capacity),
        ) / b::RUST_FAIR_SCHED_CAPACITY_SCALE as c_ulong) as c_long;
    }
}
unsafe fn sched_balance_find_src_group(env: *mut b::lb_env) -> *mut b::sched_group {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut sds: b::sd_lb_stats = core::mem::zeroed();
        init_sd_lb_stats(&mut sds);
        update_sd_lb_stats(env, &mut sds);
        let balance = 'decision: {
            if sds.busiest.is_null() {
                break 'decision false;
            }
            let busiest = addr_of_mut!(sds.busiest_stat);
            if (*busiest).group_type == b::group_misfit_task {
                break 'decision true;
            }
            if !is_rd_overutilized((*(*env).dst_rq).rd)
                && !b::rust_fair_rcu_rd_pd((*(*env).dst_rq).rd).is_null()
            {
                break 'decision false;
            }
            if (*busiest).group_type == b::group_asym_packing
                || (*busiest).group_type == b::group_imbalanced
            {
                break 'decision true;
            }
            let local = addr_of_mut!(sds.local_stat);
            if (*local).group_type > (*busiest).group_type {
                break 'decision false;
            }
            if (*local).group_type == b::group_overloaded {
                if (*local).avg_load >= (*busiest).avg_load {
                    break 'decision false;
                }
                sds.avg_load = sds
                    .total_load
                    .wrapping_mul(b::RUST_FAIR_SCHED_CAPACITY_SCALE as c_ulong)
                    / sds.total_capacity;
                if (*local).avg_load >= sds.avg_load
                    || (*busiest).avg_load.wrapping_mul(100)
                        <= (*local)
                            .avg_load
                            .wrapping_mul((*(*env).sd).imbalance_pct as c_ulong)
                {
                    break 'decision false;
                }
            }
            if sds.prefer_sibling != 0
                && (*local).group_type == b::group_has_spare
                && ((*busiest).group_type == b::group_llc_balance
                    || sibling_imbalance(env, &mut sds, busiest, local) > 1)
            {
                break 'decision true;
            }
            if (*busiest).group_type != b::group_overloaded {
                if (*env).idle == 0 {
                    break 'decision false;
                }
                if (*busiest).group_type == b::group_smt_balance
                    && smt_vs_nonsmt_groups(sds.local, sds.busiest)
                {
                    break 'decision true;
                }
                if (*busiest).group_weight > 1
                    && (*local).idle_cpus <= (*busiest).idle_cpus.wrapping_add(1)
                {
                    break 'decision false;
                }
                if (*busiest).sum_h_nr_running == 1 {
                    break 'decision false;
                }
            }
            true
        };
        if !balance {
            (*env).imbalance = 0;
            return null_mut();
        }
        calculate_imbalance(env, &mut sds);
        if (*env).imbalance != 0 {
            sds.busiest
        } else {
            null_mut()
        }
    }
}
unsafe fn sched_balance_find_src_rq(env: *mut b::lb_env, group: *mut b::sched_group) -> *mut b::rq {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut busiest = null_mut();
        let mut busiest_util = 0;
        let mut busiest_load: c_ulong = 0;
        let mut busiest_capacity: c_ulong = 1;
        let mut busiest_nr = 0;
        #[cfg(CONFIG_SCHED_CACHE)]
        let mut busiest_pref_llc = 0;
        fair_each_cpu_and!(i, b::rust_fair_sched_group_span(group), (*env).cpus, {
            let rq = b::rust_fair_cpu_rq(i);
            if fbq_classify_rq(rq) > (*env).fbq_type {
                continue;
            }
            let nr_running = (*rq).cfs.h_nr_runnable;
            if nr_running == 0 {
                continue;
            }
            let capacity = capacity_of(i);
            if (*(*env).sd).flags & b::RUST_FAIR_SD_ASYM_CPUCAPACITY != 0 && nr_running == 1 {
                let cluster_equal_cap = b::rust_fair_sched_cluster_active()
                    && get_actual_cpu_capacity((*env).dst_cpu) == get_actual_cpu_capacity(i);
                let smt_degraded_cap = b::rust_fair_sched_smt_active() && !is_core_idle(i);
                if !smt_degraded_cap
                    && !cluster_equal_cap
                    && !capacity_greater(capacity_of((*env).dst_cpu), capacity)
                {
                    continue;
                }
            }
            if sched_asym((*env).sd, i, (*env).dst_cpu) && nr_running == 1 {
                continue;
            }
            match (*env).migration_type {
                b::migrate_load => {
                    let load = cpu_load(rq);
                    if nr_running == 1
                        && load > (*env).imbalance as c_ulong
                        && !check_cpu_capacity(rq, (*env).sd)
                    {
                        continue;
                    }
                    if load.wrapping_mul(busiest_capacity) > busiest_load.wrapping_mul(capacity) {
                        busiest_load = load;
                        busiest_capacity = capacity;
                        busiest = rq;
                    }
                }
                b::migrate_util => {
                    let util = cpu_util_cfs_boost(i);
                    if nr_running <= 1 {
                        continue;
                    }
                    if busiest_util < util {
                        busiest_util = util;
                        busiest = rq;
                    }
                }
                b::migrate_task => {
                    if busiest_nr < nr_running {
                        busiest_nr = nr_running;
                        busiest = rq;
                    }
                }
                b::migrate_misfit => {
                    if (*rq).misfit_task_load > busiest_load {
                        busiest_load = (*rq).misfit_task_load;
                        busiest = rq;
                    }
                }
                b::migrate_llc_task => {
                    #[cfg(CONFIG_SCHED_CACHE)]
                    {
                        let sd_tmp = b::rust_fair_rcu_rq_sd(rq);
                        let dst_llc = llc_id((*env).dst_cpu);
                        if !sd_tmp.is_null() && (dst_llc as c_uint) < (*sd_tmp).llc_max {
                            let pref = *(*sd_tmp).llc_counts.add(dst_llc as usize);
                            if busiest_pref_llc < pref {
                                busiest_pref_llc = pref;
                                busiest = rq;
                            }
                        }
                    }
                }
                _ => {}
            }
        });
        busiest
    }
}
#[inline]
unsafe fn asym_active_balance(env: *mut b::lb_env) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (*env).idle != 0
            && sched_use_asym_prio((*env).sd, (*env).dst_cpu)
            && (b::rust_fair_sched_asym_prefer((*env).dst_cpu, (*env).src_cpu)
                || !sched_use_asym_prio((*env).sd, (*env).src_cpu))
    }
}
#[inline]
unsafe fn imbalanced_active_balance(env: *mut b::lb_env) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (*env).migration_type == b::migrate_task
            && (*(*env).sd).nr_balance_failed > (*(*env).sd).cache_nice_tries.wrapping_add(2)
    }
}
unsafe fn need_active_balance(env: *mut b::lb_env) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let sd = (*env).sd;
        if alb_break_llc(env) {
            return false;
        }
        if asym_active_balance(env) || imbalanced_active_balance(env) {
            return true;
        }
        if (*env).idle != 0
            && (*(*env).src_rq).cfs.h_nr_runnable == 1
            && check_cpu_capacity((*env).src_rq, sd)
            && capacity_of((*env).src_cpu).wrapping_mul((*sd).imbalance_pct as c_ulong)
                < capacity_of((*env).dst_cpu).wrapping_mul(100)
        {
            return true;
        }
        (*env).migration_type == b::migrate_misfit || (*env).migration_type == b::migrate_llc_task
    }
}
unsafe fn should_we_balance(env: *mut b::lb_env) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let swb_cpus = b::rust_fair_this_should_we_balance_tmpmask();
        let sg = (*(*env).sd).groups;
        let mut idle_smt = -1;
        if !b::rust_fair_cpumask_test_cpu((*env).dst_cpu, (*env).cpus) {
            return false;
        }
        if (*env).idle == b::CPU_NEWLY_IDLE {
            return (*(*env).dst_rq).nr_running == 0 && !b::rust_fair_rq_ttwu_pending((*env).dst_rq);
        }
        b::rust_fair_cpumask_copy(swb_cpus, b::rust_fair_group_balance_mask(sg));
        fair_each_cpu_and!(cpu, swb_cpus, (*env).cpus, {
            if b::rust_fair_idle_cpu(cpu) == 0 {
                continue;
            }
            if b::rust_fair_sched_smt_active()
                && (*(*env).sd).flags & b::RUST_FAIR_SD_SHARE_CPUCAPACITY == 0
                && !is_core_idle(cpu)
            {
                if idle_smt == -1 {
                    idle_smt = cpu;
                }
                b::rust_fair_cpumask_andnot(swb_cpus, swb_cpus, b::rust_fair_cpu_smt_mask(cpu));
                continue;
            }
            return cpu == (*env).dst_cpu;
        });
        if idle_smt != -1 {
            idle_smt == (*env).dst_cpu
        } else {
            b::rust_fair_group_balance_cpu(sg) == (*env).dst_cpu
        }
    }
}
unsafe fn update_lb_imbalance_stat(
    env: *mut b::lb_env,
    sd: *mut b::sched_domain,
    idle: b::cpu_idle_type,
) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if !b::rust_fair_schedstat_enabled() {
            return;
        }
        match (*env).migration_type {
            b::migrate_load => b::rust_fair_stat_lb_imbalance_load(sd, idle, (*env).imbalance),
            b::migrate_util => b::rust_fair_stat_lb_imbalance_util(sd, idle, (*env).imbalance),
            b::migrate_task => b::rust_fair_stat_lb_imbalance_task(sd, idle, (*env).imbalance),
            b::migrate_misfit => b::rust_fair_stat_lb_imbalance_misfit(sd, idle, (*env).imbalance),
            _ => {}
        }
    }
}
// Rust-only control state replacing the four shared C exit labels, no ABI use.
enum FairBalanceExit {
    Balanced,
    AllPinned,
    OnePinned,
    Unbalanced,
}
unsafe fn sched_balance_rq(
    this_cpu: c_int,
    this_rq: *mut b::rq,
    sd: *mut b::sched_domain,
    idle: b::cpu_idle_type,
    continue_balancing: *mut c_int,
) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let sd_parent = (*sd).parent;
        let cpus = b::rust_fair_this_load_balance_mask();
        let mut env: b::lb_env = core::mem::zeroed();
        let mut rf: b::rq_flags = core::mem::zeroed();
        env.sd = sd;
        env.dst_cpu = this_cpu;
        env.dst_rq = this_rq;
        env.dst_grpmask = b::rust_fair_group_balance_mask((*sd).groups);
        env.idle = idle;
        env.loop_break = b::RUST_FAIR_SCHED_NR_MIGRATE_BREAK as c_uint;
        env.cpus = cpus;
        env.fbq_type = b::all;
        b::rust_fair_init_list_head(addr_of_mut!(env.tasks));
        let mut need_unlock = false;
        let mut ld_moved: c_int = 0;
        b::rust_fair_cpumask_and(
            cpus,
            b::rust_fair_sched_domain_span(sd),
            b::rust_fair_cpu_active_mask(),
        );
        b::rust_fair_stat_lb_count(sd, idle);
        let finish = 'redo: loop {
            if !should_we_balance(&mut env) {
                *continue_balancing = 0;
                break 'redo FairBalanceExit::Balanced;
            }
            if !need_unlock && (*sd).flags & b::RUST_FAIR_SD_SERIALIZE != 0 {
                if !b::rust_fair_balance_try_acquire() {
                    break 'redo FairBalanceExit::Balanced;
                }
                need_unlock = true;
            }
            let group = sched_balance_find_src_group(&mut env);
            if group.is_null() {
                b::rust_fair_stat_lb_nobusyg(sd, idle);
                break 'redo FairBalanceExit::Balanced;
            }
            let busiest = sched_balance_find_src_rq(&mut env, group);
            if busiest.is_null() {
                b::rust_fair_stat_lb_nobusyq(sd, idle);
                break 'redo FairBalanceExit::Balanced;
            }
            b::rust_fair_warn_sched_balance_rq_1(busiest == env.dst_rq);
            update_lb_imbalance_stat(&mut env, sd, idle);
            env.src_cpu = (*busiest).cpu;
            env.src_rq = busiest;
            ld_moved = 0;
            env.flags |= b::RUST_FAIR_LBF_ALL_PINNED as c_uint;
            if (*busiest).nr_running > 1 {
                env.loop_max = min(
                    b::rust_fair_sysctl_sched_nr_migrate(),
                    (*busiest).nr_running,
                );
                loop {
                    b::rust_fair_rq_lock_irqsave(busiest, &mut rf);
                    b::update_rq_clock(busiest);
                    let cur_ld_moved = detach_tasks(&mut env);
                    b::rust_fair_rq_unlock(busiest, &mut rf);
                    if cur_ld_moved != 0 {
                        attach_tasks(&mut env);
                        ld_moved = ld_moved.wrapping_add(cur_ld_moved);
                    }
                    // Attach with IRQs still disabled, restore only after destination unlock.
                    b::rust_fair_local_irq_restore(rf.flags);
                    if env.flags & (b::RUST_FAIR_LBF_NEED_BREAK as c_uint) != 0 {
                        env.flags &= !(b::RUST_FAIR_LBF_NEED_BREAK as c_uint);
                        continue;
                    }
                    if env.flags & (b::RUST_FAIR_LBF_DST_PINNED as c_uint) != 0 && env.imbalance > 0 {
                        b::rust_fair___cpumask_clear_cpu(env.dst_cpu, env.cpus);
                        env.dst_rq = b::rust_fair_cpu_rq(env.new_dst_cpu);
                        env.dst_cpu = env.new_dst_cpu;
                        env.flags &= !(b::RUST_FAIR_LBF_DST_PINNED as c_uint);
                        env.loop_ = 0;
                        env.loop_break = b::RUST_FAIR_SCHED_NR_MIGRATE_BREAK as c_uint;
                        continue;
                    }
                    break;
                }
                if !sd_parent.is_null()
                    && env.flags & (b::RUST_FAIR_LBF_SOME_PINNED as c_uint) != 0
                    && env.imbalance > 0
                {
                    (*(*(*sd_parent).groups).sgc).imbalance = 1;
                }
                if env.flags & (b::RUST_FAIR_LBF_ALL_PINNED as c_uint) != 0 {
                    b::rust_fair___cpumask_clear_cpu(b::rust_fair_cpu_of(busiest), cpus);
                    if !b::rust_fair_cpumask_subset(cpus, env.dst_grpmask) {
                        env.loop_ = 0;
                        env.loop_break = b::RUST_FAIR_SCHED_NR_MIGRATE_BREAK as c_uint;
                        continue 'redo;
                    }
                    break 'redo FairBalanceExit::AllPinned;
                }
            }
            if ld_moved != 0 {
                (*sd).nr_balance_failed = 0;
                break 'redo FairBalanceExit::Unbalanced;
            }
            b::rust_fair_stat_lb_failed(sd, idle);
            if idle != b::CPU_NEWLY_IDLE
                && env.migration_type != b::migrate_misfit
                && env.flags & (b::RUST_FAIR_LBF_LLC_PINNED as c_uint) == 0
            {
                (*sd).nr_balance_failed = (*sd).nr_balance_failed.wrapping_add(1);
            }
            if !need_active_balance(&mut env) {
                break 'redo FairBalanceExit::Unbalanced;
            }
            let irq_flags = b::rust_fair_raw_spin_rq_lock_irqsave(busiest);
            if !b::rust_fair_cpumask_test_cpu(this_cpu, (*(*busiest).curr).cpus_ptr) {
                b::rust_fair_raw_spin_rq_unlock_irqrestore(busiest, irq_flags);
                break 'redo FairBalanceExit::OnePinned;
            }
            env.flags &= !(b::RUST_FAIR_LBF_ALL_PINNED as c_uint);
            if (*busiest).active_balance != 0 || (*(*busiest).curr).on_rq == 0 {
                b::rust_fair_raw_spin_rq_unlock_irqrestore(busiest, irq_flags);
                break 'redo FairBalanceExit::Unbalanced;
            }
            (*busiest).active_balance = 1;
            (*busiest).push_cpu = this_cpu;
            b::rust_fair_preempt_disable();
            b::rust_fair_raw_spin_rq_unlock_irqrestore(busiest, irq_flags);
            b::stop_one_cpu_nowait(
                b::rust_fair_cpu_of(busiest) as c_uint,
                Some(active_load_balance_cpu_stop),
                busiest.cast(),
                addr_of_mut!((*busiest).active_balance_work),
            );
            b::rust_fair_preempt_enable();
            break 'redo FairBalanceExit::Unbalanced;
        };
        match finish {
            FairBalanceExit::Unbalanced => {
                (*sd).balance_interval = (*sd).min_interval as c_uint;
            }
            finish => {
                if matches!(finish, FairBalanceExit::Balanced)
                    && !sd_parent.is_null()
                    && env.flags & (b::RUST_FAIR_LBF_ALL_PINNED as c_uint) == 0
                {
                    let group_imbalance = addr_of_mut!((*(*(*sd_parent).groups).sgc).imbalance);
                    if *group_imbalance != 0 {
                        *group_imbalance = 0;
                    }
                }
                if !matches!(finish, FairBalanceExit::OnePinned) {
                    b::rust_fair_stat_lb_balanced(sd, idle);
                    (*sd).nr_balance_failed = 0;
                }
                ld_moved = 0;
                if env.idle != b::CPU_NEWLY_IDLE
                    && env.migration_type != b::migrate_misfit
                    && ((env.flags & (b::RUST_FAIR_LBF_ALL_PINNED as c_uint) != 0
                        && (*sd).balance_interval < b::RUST_FAIR_MAX_PINNED_INTERVAL as c_uint)
                        || ((*sd).balance_interval as c_ulong) < (*sd).max_interval)
                {
                    (*sd).balance_interval = (*sd).balance_interval.wrapping_mul(2);
                }
            }
        }
        if need_unlock {
            b::rust_fair_balance_release();
        }
        ld_moved
    }
}
#[inline]
unsafe fn get_sd_balance_interval(sd: *mut b::sched_domain, cpu_busy: bool) -> c_ulong {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut interval = (*sd).balance_interval as c_ulong;
        if cpu_busy {
            interval = interval.wrapping_mul((*sd).busy_factor as c_ulong);
        }
        interval = b::rust_fair_msecs_to_jiffies(interval as c_uint);
        if cpu_busy {
            interval = interval.wrapping_sub(1);
        }
        fair_clamp_ulong(interval, 1, max_load_balance_interval)
    }
}
#[inline]
unsafe fn update_next_balance(sd: *mut b::sched_domain, next_balance: *mut c_ulong) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let next = (*sd)
            .last_balance
            .wrapping_add(get_sd_balance_interval(sd, false));
        if b::rust_fair_time_after(*next_balance, next) {
            *next_balance = next;
        }
    }
}
unsafe extern "C" fn active_load_balance_cpu_stop(data: *mut core::ffi::c_void) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let busiest_rq = data.cast::<b::rq>();
        let busiest_cpu = b::rust_fair_cpu_of(busiest_rq);
        let target_cpu = (*busiest_rq).push_cpu;
        let target_rq = b::rust_fair_cpu_rq(target_cpu);
        let mut p: *mut b::task_struct = null_mut();
        let mut rf: b::rq_flags = core::mem::zeroed();
        b::rust_fair_rq_lock_irq(busiest_rq, &mut rf);
        'locked: {
            if !b::rust_fair_cpu_active(busiest_cpu)
                || !b::rust_fair_cpu_active(target_cpu)
                || busiest_cpu != b::rust_fair_smp_processor_id()
                || (*busiest_rq).active_balance == 0
                || (*busiest_rq).nr_running <= 1
            {
                break 'locked;
            }
            b::rust_fair_warn_active_load_balance_cpu_stop_1(busiest_rq == target_rq);
            b::rust_fair_rcu_read_lock();
            let mut sd = b::rust_fair_cpu_sched_domain(target_cpu);
            while !sd.is_null() {
                if b::rust_fair_cpumask_test_cpu(busiest_cpu, b::rust_fair_sched_domain_span(sd)) {
                    break;
                }
                sd = (*sd).parent;
            }
            if !sd.is_null() {
                let mut env: b::lb_env = core::mem::zeroed();
                env.sd = sd;
                env.dst_cpu = target_cpu;
                env.dst_rq = target_rq;
                env.src_cpu = (*busiest_rq).cpu;
                env.src_rq = busiest_rq;
                env.idle = b::CPU_IDLE;
                env.flags = b::RUST_FAIR_LBF_ACTIVE_LB as c_uint;
                b::rust_fair_stat_alb_count(sd);
                b::update_rq_clock(busiest_rq);
                p = detach_one_task(&mut env);
                if !p.is_null() {
                    b::rust_fair_stat_alb_pushed(sd);
                    (*sd).nr_balance_failed = 0;
                } else {
                    b::rust_fair_stat_alb_failed(sd);
                }
            }
            b::rust_fair_rcu_read_unlock();
        }
        (*busiest_rq).active_balance = 0;
        b::rust_fair_rq_unlock(busiest_rq, &mut rf);
        if !p.is_null() {
            b::rust_fair_attach_one_task(target_rq, p);
        }
        b::rust_fair_local_irq_enable();
        0
    }
}
#[no_mangle]
pub unsafe extern "C" fn update_max_interval() {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        max_load_balance_interval =
            (b::RUST_FAIR_HZ as c_uint).wrapping_mul(b::rust_fair_num_online_cpus()) as c_ulong / 10;
    }
}
#[inline]
unsafe fn update_newidle_stats(sd: *mut b::sched_domain, success: c_uint) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (*sd).newidle_call = (*sd).newidle_call.wrapping_add(1);
        (*sd).newidle_success = (*sd).newidle_success.wrapping_add(success);
        if (*sd).newidle_call >= 1024 {
            let now = b::sched_clock();
            let mut delta = now.wrapping_sub((*sd).newidle_stamp) as i64;
            (*sd).newidle_stamp = now;
            let mut ratio = 0i32;
            if delta < 0 {
                delta = 0;
            }
            if feat!(rust_fair_feat_NI_RATE) {
                ratio = (delta >> 22) as c_int;
            }
            ratio = ratio.wrapping_add((*sd).newidle_success as c_int);
            (*sd).newidle_ratio = min(1024, ratio) as _;
            (*sd).newidle_call /= 2;
            (*sd).newidle_success /= 2;
        }
    }
}
#[inline]
unsafe fn update_newidle_cost(sd: *mut b::sched_domain, cost: u64, success: c_uint) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let next_decay = (*sd)
            .last_decay_max_lb_cost
            .wrapping_add(b::RUST_FAIR_HZ as c_ulong);
        let now = b::rust_fair_jiffies();
        if cost != 0 {
            update_newidle_stats(sd, success);
        }
        if cost > (*sd).max_newidle_lb_cost {
            (*sd).max_newidle_lb_cost = cost;
            (*sd).last_decay_max_lb_cost = now;
        } else if b::rust_fair_time_after(now, next_decay) {
            (*sd).max_newidle_lb_cost = (*sd).max_newidle_lb_cost.wrapping_mul(253) / 256;
            (*sd).last_decay_max_lb_cost = now;
            return true;
        }
        false
    }
}
unsafe fn sched_balance_domains(rq: *mut b::rq, mut idle: b::cpu_idle_type) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut continue_balancing = 1;
        let cpu = (*rq).cpu;
        let mut busy = idle != b::CPU_IDLE && !sched_idle_rq(rq);
        let mut next_balance = b::rust_fair_jiffies().wrapping_add(60 * b::RUST_FAIR_HZ as c_ulong);
        let mut update_next = false;
        let mut need_decay = false;
        let mut max_cost = 0u64;
        b::rust_fair_rcu_read_lock();
        let mut sd = b::rust_fair_cpu_sched_domain(cpu);
        while !sd.is_null() {
            need_decay = update_newidle_cost(sd, 0, 0);
            max_cost = max_cost.wrapping_add((*sd).max_newidle_lb_cost);
            if continue_balancing == 0 {
                if need_decay {
                    sd = (*sd).parent;
                    continue;
                }
                break;
            }
            let mut interval = get_sd_balance_interval(sd, busy);
            if b::rust_fair_time_after_eq(
                b::rust_fair_jiffies(),
                (*sd).last_balance.wrapping_add(interval),
            ) {
                if sched_balance_rq(cpu, rq, sd, idle, &mut continue_balancing) != 0 {
                    idle = b::rust_fair_idle_cpu(cpu) as b::cpu_idle_type;
                    busy = idle == 0 && !sched_idle_rq(rq);
                }
                (*sd).last_balance = b::rust_fair_jiffies();
                interval = get_sd_balance_interval(sd, busy);
            }
            if b::rust_fair_time_after(next_balance, (*sd).last_balance.wrapping_add(interval)) {
                next_balance = (*sd).last_balance.wrapping_add(interval);
                update_next = true;
            }
            sd = (*sd).parent;
        }
        if need_decay {
            (*rq).max_idle_balance_cost = max(sysctl_sched_migration_cost as u64, max_cost);
        }
        b::rust_fair_rcu_read_unlock();
        if update_next {
            (*rq).next_balance = next_balance;
        }
    }
}
#[inline]
unsafe fn on_null_domain(rq: *mut b::rq) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_rcu_sched_rq_sd(rq).is_null()
    }
}
#[cfg(CONFIG_NO_HZ_COMMON)]
unsafe fn find_new_ilb() -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut fallback = -1;
        b::rust_fair_lockdep_assert_irqs_disabled();
        let ilb_cpus = b::rust_fair_this_select_rq_mask();
        let nohz = b::rust_fair_nohz();
        b::rust_fair_cpumask_and(
            ilb_cpus,
            b::rust_fair_nohz_idle_cpus_mask(),
            b::rust_fair_housekeeping_kernel_noise_mask(),
        );
        fair_each_cpu!(ilb_cpu, ilb_cpus, {
            if b::rust_fair_idle_cpu(ilb_cpu) == 0 {
                if b::rust_fair_sched_smt_active() && fallback >= 0 {
                    b::rust_fair_cpumask_andnot(ilb_cpus, ilb_cpus, b::rust_fair_cpu_smt_mask(ilb_cpu));
                }
                continue;
            }
            if b::rust_fair_sched_smt_active() && !is_core_idle(ilb_cpu) {
                if fallback < 0 {
                    fallback = ilb_cpu;
                }
                b::rust_fair_cpumask_andnot(ilb_cpus, ilb_cpus, b::rust_fair_cpu_smt_mask(ilb_cpu));
                continue;
            }
            return ilb_cpu;
        });
        fallback
    }
}
#[cfg(CONFIG_NO_HZ_COMMON)]
unsafe fn kick_ilb(mut flags: c_uint) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let nohz = b::rust_fair_nohz();
        if flags & (b::RUST_FAIR_NOHZ_BALANCE_KICK as c_uint) != 0 {
            (*nohz).next_balance = b::rust_fair_jiffies().wrapping_add(1);
        }
        let ilb_cpu = find_new_ilb();
        if ilb_cpu < 0 {
            return;
        }
        if b::rust_fair_nohz_flags_read(ilb_cpu) as c_uint & flags == flags {
            return;
        }
        flags = b::rust_fair_nohz_flags_fetch_or(ilb_cpu, flags) as c_uint;
        if flags & (b::RUST_FAIR_NOHZ_KICK_MASK as c_uint) != 0 {
            return;
        }
        b::smp_call_function_single_async(
            ilb_cpu,
            addr_of_mut!((*b::rust_fair_cpu_rq(ilb_cpu)).nohz_csd),
        );
    }
}
#[cfg(CONFIG_NO_HZ_COMMON)]
unsafe fn nohz_balancer_kick(rq: *mut b::rq) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let now = b::rust_fair_jiffies();
        let cpu = (*rq).cpu;
        let mut flags = 0;
        let nohz = b::rust_fair_nohz();
        if (*rq).idle_balance != 0 {
            return;
        }
        nohz_balance_exit_idle(rq);
        if read_once!((*nohz).has_blocked_load) != 0
            && b::rust_fair_time_after(now, read_once!((*nohz).next_blocked))
        {
            flags = (b::RUST_FAIR_NOHZ_STATS_KICK as c_uint);
        }
        'decide: {
            if b::rust_fair_time_before(now, (*nohz).next_balance) {
                break 'decide;
            }
            if b::rust_fair_cpumask_empty(b::rust_fair_nohz_idle_cpus_mask()) {
                return;
            }
            if (*rq).nr_running >= 2 {
                flags = (b::RUST_FAIR_NOHZ_STATS_KICK as c_uint) | (b::RUST_FAIR_NOHZ_BALANCE_KICK as c_uint);
                break 'decide;
            }
            let sd = b::rust_fair_rcu_rq_sd(rq);
            if !sd.is_null() && (*rq).cfs.h_nr_runnable >= 1 && check_cpu_capacity(rq, sd) {
                flags |= (b::RUST_FAIR_NOHZ_STATS_KICK as c_uint) | (b::RUST_FAIR_NOHZ_BALANCE_KICK as c_uint);
                break 'decide;
            }
            let sd = b::rust_fair_rcu_sd_asym_packing(cpu);
            if !sd.is_null() {
                fair_each_cpu_and!(
                    i,
                    b::rust_fair_sched_domain_span(sd),
                    b::rust_fair_nohz_idle_cpus_mask(),
                    {
                        if sched_asym(sd, i, cpu) {
                            flags |= (b::RUST_FAIR_NOHZ_STATS_KICK as c_uint) | (b::RUST_FAIR_NOHZ_BALANCE_KICK as c_uint);
                            break 'decide;
                        }
                    }
                );
            }
            let sd = b::rust_fair_rcu_sd_asym_cpucapacity(cpu);
            if !sd.is_null() {
                if check_misfit_status(rq) {
                    flags |= (b::RUST_FAIR_NOHZ_STATS_KICK as c_uint) | (b::RUST_FAIR_NOHZ_BALANCE_KICK as c_uint);
                }
                break 'decide;
            }
            let sds = b::rust_fair_rcu_sd_balance_shared(cpu);
            if !sds.is_null() && b::rust_fair_atomic_read(addr_of!((*sds).nr_busy_cpus)) > 1 {
                flags |= (b::RUST_FAIR_NOHZ_STATS_KICK as c_uint) | (b::RUST_FAIR_NOHZ_BALANCE_KICK as c_uint);
            }
        }
        if read_once!((*nohz).needs_update) != 0 {
            flags |= (b::RUST_FAIR_NOHZ_NEXT_KICK as c_uint);
        }
        if flags != 0 {
            kick_ilb(flags);
        }
    }
}
#[cfg(CONFIG_NO_HZ_COMMON)]
unsafe fn set_cpu_sd_state_busy(cpu: c_int) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let sd = b::rust_fair_rcu_sd_llc(cpu);
        if sd.is_null() || (*sd).shared.is_null() || (*sd).nohz_idle == 0 {
            return;
        }
        (*sd).nohz_idle = 0;
        b::rust_fair_atomic_inc(addr_of_mut!((*(*sd).shared).nr_busy_cpus));
    }
}
#[cfg(CONFIG_NO_HZ_COMMON)]
#[no_mangle]
pub unsafe extern "C" fn nohz_balance_exit_idle(rq: *mut b::rq) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_warn_nohz_balance_exit_idle_1(rq != b::rust_fair_this_rq());
        if (*rq).nohz_tick_stopped == 0 {
            return;
        }
        (*rq).nohz_tick_stopped = 0;
        b::rust_fair_cpumask_clear_cpu((*rq).cpu, b::rust_fair_nohz_idle_cpus_mask());
        set_cpu_sd_state_busy((*rq).cpu);
    }
}
#[cfg(CONFIG_NO_HZ_COMMON)]
unsafe fn set_cpu_sd_state_idle(cpu: c_int) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let sd = b::rust_fair_rcu_sd_llc(cpu);
        if sd.is_null() || (*sd).shared.is_null() || (*sd).nohz_idle != 0 {
            return;
        }
        (*sd).nohz_idle = 1;
        b::rust_fair_atomic_dec(addr_of_mut!((*(*sd).shared).nr_busy_cpus));
    }
}
#[cfg(CONFIG_NO_HZ_COMMON)]
#[no_mangle]
pub unsafe extern "C" fn nohz_balance_enter_idle(cpu: c_int) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let rq = b::rust_fair_cpu_rq(cpu);
        let nohz = b::rust_fair_nohz();
        b::rust_fair_warn_nohz_balance_enter_idle_1(cpu != b::rust_fair_smp_processor_id());
        if !b::rust_fair_cpu_active(cpu) {
            return;
        }
        (*rq).has_blocked_load = 1;
        if (*rq).nohz_tick_stopped == 0 {
            if on_null_domain(rq) {
                return;
            }
            (*rq).nohz_tick_stopped = 1;
            b::rust_fair_cpumask_set_cpu(cpu, b::rust_fair_nohz_idle_cpus_mask());
            b::rust_fair_smp_mb_after_atomic();
            set_cpu_sd_state_idle(cpu);
            write_once!((*nohz).needs_update, 1);
        }
        write_once!((*nohz).has_blocked_load, 1);
    }
}
#[cfg(CONFIG_NO_HZ_COMMON)]
unsafe fn update_nohz_stats(rq: *mut b::rq) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let cpu = (*rq).cpu;
        if (*rq).has_blocked_load == 0
            || !b::rust_fair_cpumask_test_cpu(cpu, b::rust_fair_nohz_idle_cpus_mask())
        {
            return false;
        }
        if !b::rust_fair_time_after(
            b::rust_fair_jiffies(),
            read_once!((*rq).last_blocked_load_update_tick),
        ) {
            return true;
        }
        sched_balance_update_blocked_averages(cpu);
        (*rq).has_blocked_load != 0
    }
}
#[cfg(CONFIG_NO_HZ_COMMON)]
unsafe fn _nohz_idle_balance(this_rq: *mut b::rq, flags: c_uint) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let now = b::rust_fair_jiffies();
        let mut next_balance = now.wrapping_add(60 * b::RUST_FAIR_HZ as c_ulong);
        let mut has_blocked_load = false;
        let mut update_next = false;
        let this_cpu = (*this_rq).cpu;
        let nohz = b::rust_fair_nohz();
        b::rust_fair_warn__nohz_idle_balance_1(
            flags & (b::RUST_FAIR_NOHZ_KICK_MASK as c_uint) == (b::RUST_FAIR_NOHZ_BALANCE_KICK as c_uint),
        );
        if flags & (b::RUST_FAIR_NOHZ_STATS_KICK as c_uint) != 0 {
            write_once!((*nohz).has_blocked_load, 0);
        }
        if flags & (b::RUST_FAIR_NOHZ_NEXT_KICK as c_uint) != 0 {
            write_once!((*nohz).needs_update, 0);
        }
        b::rust_fair_smp_mb();
        'scan: {
            fair_each_cpu_wrap!(
                balance_cpu,
                b::rust_fair_nohz_idle_cpus_mask(),
                this_cpu.wrapping_add(1),
                {
                    if b::rust_fair_idle_cpu(balance_cpu) == 0 {
                        continue;
                    }
                    if b::rust_fair_idle_cpu(this_cpu) == 0 && b::rust_fair_need_resched() {
                        if flags & (b::RUST_FAIR_NOHZ_STATS_KICK as c_uint) != 0 {
                            has_blocked_load = true;
                        }
                        if flags & (b::RUST_FAIR_NOHZ_NEXT_KICK as c_uint) != 0 {
                            write_once!((*nohz).needs_update, 1);
                        }
                        break 'scan;
                    }
                    let rq = b::rust_fair_cpu_rq(balance_cpu);
                    if flags & (b::RUST_FAIR_NOHZ_STATS_KICK as c_uint) != 0 {
                        has_blocked_load |= update_nohz_stats(rq);
                    }
                    if b::rust_fair_time_after_eq(b::rust_fair_jiffies(), (*rq).next_balance) {
                        let mut rf: b::rq_flags = core::mem::zeroed();
                        b::rust_fair_rq_lock_irqsave(rq, &mut rf);
                        b::update_rq_clock(rq);
                        b::rust_fair_rq_unlock_irqrestore(rq, &mut rf);
                        if flags & (b::RUST_FAIR_NOHZ_BALANCE_KICK as c_uint) != 0 {
                            sched_balance_domains(rq, b::CPU_IDLE);
                        }
                    }
                    if b::rust_fair_time_after(next_balance, (*rq).next_balance) {
                        next_balance = (*rq).next_balance;
                        update_next = true;
                    }
                }
            );
            if update_next {
                (*nohz).next_balance = next_balance;
            }
            if flags & (b::RUST_FAIR_NOHZ_STATS_KICK as c_uint) != 0 {
                write_once!(
                    (*nohz).next_blocked,
                    now.wrapping_add(b::rust_fair_msecs_to_jiffies(b::RUST_FAIR_LOAD_AVG_PERIOD as c_uint))
                );
            }
        }
        if has_blocked_load {
            write_once!((*nohz).has_blocked_load, 1);
        }
    }
}
#[cfg(CONFIG_NO_HZ_COMMON)]
unsafe fn nohz_idle_balance(this_rq: *mut b::rq, idle: b::cpu_idle_type) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let flags = (*this_rq).nohz_idle_balance as c_uint;
        if flags == 0 {
            return false;
        }
        (*this_rq).nohz_idle_balance = 0;
        if idle != b::CPU_IDLE {
            return false;
        }
        _nohz_idle_balance(this_rq, flags);
        true
    }
}
#[cfg(CONFIG_NO_HZ_COMMON)]
#[no_mangle]
pub unsafe extern "C" fn nohz_run_idle_balance(cpu: c_int) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let flags = b::rust_fair_nohz_flags_fetch_andnot(cpu, (b::RUST_FAIR_NOHZ_NEWILB_KICK as c_uint)) as c_uint;
        if flags == (b::RUST_FAIR_NOHZ_NEWILB_KICK as c_uint) && !b::rust_fair_need_resched() {
            _nohz_idle_balance(b::rust_fair_cpu_rq(cpu), (b::RUST_FAIR_NOHZ_STATS_KICK as c_uint));
        }
    }
}
#[cfg(CONFIG_NO_HZ_COMMON)]
unsafe fn nohz_newidle_balance(this_rq: *mut b::rq) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let nohz = b::rust_fair_nohz();
        if (*this_rq).avg_idle < sysctl_sched_migration_cost as u64 {
            return;
        }
        if read_once!((*nohz).has_blocked_load) == 0
            || b::rust_fair_time_before(b::rust_fair_jiffies(), read_once!((*nohz).next_blocked))
        {
            return;
        }
        b::rust_fair_nohz_flags_or((*this_rq).cpu, (b::RUST_FAIR_NOHZ_NEWILB_KICK as c_uint));
    }
}
#[cfg(not(CONFIG_NO_HZ_COMMON))]
unsafe fn nohz_balancer_kick(_rq: *mut b::rq) {}
#[cfg(not(CONFIG_NO_HZ_COMMON))]
unsafe fn nohz_idle_balance(_this_rq: *mut b::rq, _idle: b::cpu_idle_type) -> bool {
    false
}
#[cfg(not(CONFIG_NO_HZ_COMMON))]
unsafe fn nohz_newidle_balance(_this_rq: *mut b::rq) {}
unsafe fn sched_balance_newidle(this_rq: *mut b::rq, rf: *mut b::rq_flags) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut next_balance = b::rust_fair_jiffies().wrapping_add(b::RUST_FAIR_HZ as c_ulong);
        let this_cpu = (*this_rq).cpu;
        let mut continue_balancing = 1;
        let mut curr_cost = 0u64;
        let mut pulled_task = 0;
        update_misfit_status(null_mut(), this_rq);
        if b::rust_fair_rq_ttwu_pending(this_rq) {
            return 0;
        }
        (*this_rq).idle_stamp = b::rust_fair_rq_clock(this_rq);
        if !b::rust_fair_cpu_active(this_cpu) {
            return 0;
        }
        b::rust_fair_rq_unpin_lock(this_rq, rf);
        'balance: {
            let sd = b::rust_fair_rcu_sched_domain_rq_sd(this_rq);
            if sd.is_null() {
                break 'balance;
            }
            if b::rust_fair_get_rd_overloaded((*this_rq).rd) == 0
                || (*this_rq).avg_idle < (*sd).max_newidle_lb_cost
            {
                update_next_balance(sd, &mut next_balance);
                break 'balance;
            }
            let mut t0 = b::sched_clock_cpu(this_cpu);
            __sched_balance_update_blocked_averages(this_rq);
            b::rust_fair_rq_modified_begin(this_rq, addr_of!(b::fair_sched_class));
            b::rust_fair_raw_spin_rq_unlock(this_rq);
            let mut sd = b::rust_fair_cpu_sched_domain(this_cpu);
            while !sd.is_null() {
                update_next_balance(sd, &mut next_balance);
                if (*this_rq).avg_idle < curr_cost.wrapping_add((*sd).max_newidle_lb_cost) {
                    break;
                }
                if (*sd).flags & b::RUST_FAIR_SD_BALANCE_NEWIDLE != 0 {
                    let mut weight = 1;
                    if feat!(rust_fair_feat_NI_RANDOM) && (*sd).newidle_ratio < 1024 {
                        let d1k = b::rust_fair_sched_rng() % 1024;
                        weight = 1u32.wrapping_add((*sd).newidle_ratio);
                        if d1k > weight {
                            update_newidle_stats(sd, 0);
                            sd = (*sd).parent;
                            continue;
                        }
                        weight = (1024 + weight / 2) / weight;
                    }
                    pulled_task = sched_balance_rq(
                        this_cpu,
                        this_rq,
                        sd,
                        b::CPU_NEWLY_IDLE,
                        &mut continue_balancing,
                    );
                    let t1 = b::sched_clock_cpu(this_cpu);
                    let domain_cost = t1.wrapping_sub(t0);
                    curr_cost = curr_cost.wrapping_add(domain_cost);
                    t0 = t1;
                    update_newidle_cost(
                        sd,
                        domain_cost,
                        weight.wrapping_mul((pulled_task != 0) as c_uint),
                    );
                }
                if pulled_task != 0 || continue_balancing == 0 {
                    break;
                }
                sd = (*sd).parent;
            }
            b::rust_fair_raw_spin_rq_lock(this_rq);
            if curr_cost > (*this_rq).max_idle_balance_cost {
                (*this_rq).max_idle_balance_cost = curr_cost;
            }
            if (*this_rq).cfs.h_nr_queued != 0 && pulled_task == 0 {
                pulled_task = 1;
            }
            if b::rust_fair_rq_modified_above(this_rq, addr_of!(b::fair_sched_class)) {
                pulled_task = -1;
            }
        }
        if b::rust_fair_time_after((*this_rq).next_balance, next_balance) {
            (*this_rq).next_balance = next_balance;
        }
        if pulled_task != 0 {
            (*this_rq).idle_stamp = 0;
        } else {
            nohz_newidle_balance(this_rq);
        }
        b::rust_fair_rq_repin_lock(this_rq, rf);
        pulled_task
    }
}
// Native __latent_entropy entry veneer must preserve the original plugin policy;
// scheduling body is Rust and the veneer may call only this callback.
#[no_mangle]
pub unsafe extern "C" fn rust_fair_sched_balance_softirq() {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let this_rq = b::rust_fair_this_rq();
        let idle = (*this_rq).idle_balance as b::cpu_idle_type;
        if nohz_idle_balance(this_rq, idle) {
            return;
        }
        sched_balance_update_blocked_averages((*this_rq).cpu);
        sched_balance_domains(this_rq, idle);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sched_balance_trigger(rq: *mut b::rq) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if on_null_domain(rq) || !b::rust_fair_cpu_active(b::rust_fair_cpu_of(rq)) {
            return;
        }
        if b::rust_fair_time_after_eq(b::rust_fair_jiffies(), (*rq).next_balance) {
            b::rust_fair_raise_sched_softirq();
        }
        nohz_balancer_kick(rq);
    }
}
#[export_name = "rust_fair_rq_online_fair"]
unsafe extern "C" fn rq_online_fair(rq: *mut b::rq) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        update_sysctl();
        update_runtime_enabled(rq);
    }
}
#[export_name = "rust_fair_rq_offline_fair"]
unsafe extern "C" fn rq_offline_fair(rq: *mut b::rq) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        update_sysctl();
        unthrottle_offline_cfs_rqs(rq);
        clear_tg_offline_cfs_rqs(rq);
    }
}
#[cfg(CONFIG_SCHED_CORE)]
#[inline]
unsafe fn __entity_slice_used(se: *mut b::sched_entity, min_nr_tasks: c_int) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        (*se)
            .sum_exec_runtime
            .wrapping_sub((*se).prev_sum_exec_runtime)
            .wrapping_mul(min_nr_tasks as u64)
            > (*se).slice
    }
}
#[cfg(CONFIG_SCHED_CORE)]
unsafe fn task_tick_core(rq: *mut b::rq, curr: *mut b::task_struct) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if !b::rust_fair_sched_core_enabled(rq) {
            return;
        }
        if (*(*rq).core).core_forceidle_count != 0
            && (*rq).cfs.h_nr_queued == 1
            && __entity_slice_used(
                addr_of_mut!((*curr).se),
                b::RUST_FAIR_MIN_NR_TASKS_DURING_FORCEIDLE,
            )
        {
            b::resched_curr(rq);
        }
    }
}
#[cfg(CONFIG_SCHED_CORE)]
unsafe fn se_fi_update(mut se: *const b::sched_entity, fi_seq: c_uint, forceidle: bool) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        while !se.is_null() {
            let cfs_rq = b::rust_fair_cfs_rq_of(se.cast_mut());
            if forceidle {
                if (*cfs_rq).forceidle_seq == fi_seq {
                    break;
                }
                (*cfs_rq).forceidle_seq = fi_seq;
            }
            (*cfs_rq).zero_vruntime_fi = (*cfs_rq).zero_vruntime;
            se = parent_entity(se.cast_mut());
        }
    }
}
#[cfg(CONFIG_SCHED_CORE)]
#[no_mangle]
pub unsafe extern "C" fn task_vruntime_update(rq: *mut b::rq, p: *mut b::task_struct, in_fi: bool) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if (*p).sched_class != addr_of!(b::fair_sched_class) {
            return;
        }
        se_fi_update(addr_of!((*p).se), (*(*rq).core).core_forceidle_seq, in_fi);
    }
}
#[cfg(CONFIG_SCHED_CORE)]
#[no_mangle]
pub unsafe extern "C" fn cfs_prio_less(
    a: *const b::task_struct,
    bb: *const b::task_struct,
    _in_fi: bool,
) -> bool {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let rq = b::rust_fair_task_rq(a.cast_mut());
        let sea = addr_of!((*a).se);
        let seb = addr_of!((*bb).se);
        b::rust_fair_warn_cfs_prio_less_1((*b::rust_fair_task_rq(bb.cast_mut())).core != (*rq).core);
        let cfs_rqa = addr_of!((*b::rust_fair_task_rq(a.cast_mut())).cfs);
        let cfs_rqb = addr_of!((*b::rust_fair_task_rq(bb.cast_mut())).cfs);
        let delta = ((*sea).vruntime.wrapping_sub((*seb).vruntime) as i64).wrapping_add(
            (*cfs_rqb)
                .zero_vruntime_fi
                .wrapping_sub((*cfs_rqa).zero_vruntime_fi) as i64,
        );
        delta > 0
    }
}
#[cfg(CONFIG_SCHED_CORE)]
#[export_name = "rust_fair_task_is_throttled_fair"]
unsafe extern "C" fn task_is_throttled_fair(p: *mut b::task_struct, cpu: c_int) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        #[cfg(CONFIG_FAIR_GROUP_SCHED)]
        let cfs_rq = b::rust_fair_tg_cfs_rq(b::rust_fair_task_group(p), cpu);
        #[cfg(not(CONFIG_FAIR_GROUP_SCHED))]
        let cfs_rq = addr_of_mut!((*b::rust_fair_cpu_rq(cpu)).cfs);
        throttled_hierarchy(cfs_rq)
    }
}
#[cfg(not(CONFIG_SCHED_CORE))]
unsafe fn task_tick_core(_rq: *mut b::rq, _curr: *mut b::task_struct) {}
#[export_name = "rust_fair_task_tick_fair"]
unsafe extern "C" fn task_tick_fair(rq: *mut b::rq, curr: *mut b::task_struct, queued: c_int) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut se = addr_of_mut!((*curr).se);
        if (*se).on_rq != 0 {
            let mut weight = b::RUST_FAIR_NICE_0_LOAD as c_ulong;
            let mut cfs_rq = null_mut();
            while !se.is_null() {
                cfs_rq = b::rust_fair_cfs_rq_of(se);
                entity_tick(cfs_rq, se, queued);
                weight = __calc_prop_weight(cfs_rq, se, weight);
                se = parent_entity(se);
            }
            se = addr_of_mut!((*curr).se);
            reweight_eevdf(cfs_rq, se, weight, (*se).on_rq != 0);
        }
        if queued != 0 {
            return;
        }
        if b::rust_fair_sched_numa_balancing_unlikely() {
            task_tick_numa(rq, curr);
        }
        task_tick_cache(rq, curr);
        update_misfit_status(curr, rq);
        check_update_overutilized_status(b::rust_fair_task_rq(curr));
        task_tick_core(rq, curr);
    }
}
#[export_name = "rust_fair_task_fork_fair"]
unsafe extern "C" fn task_fork_fair(p: *mut b::task_struct) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        set_task_max_allowed_capacity(p);
    }
}
#[export_name = "rust_fair_prio_changed_fair"]
unsafe extern "C" fn prio_changed_fair(rq: *mut b::rq, p: *mut b::task_struct, oldprio: u64) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if !b::rust_fair_task_on_rq_queued(p)
            || (*p).prio as u64 == oldprio
            || (*rq).cfs.h_nr_queued == 1
        {
            return;
        }
        if b::rust_fair_task_current_donor(rq, p) {
            if (*p).prio as u64 > oldprio {
                b::resched_curr(rq);
            }
        } else {
            b::wakeup_preempt(rq, p, 0);
        }
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
unsafe fn propagate_entity_cfs_rq(mut se: *mut b::sched_entity) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut cfs_rq = b::rust_fair_cfs_rq_of(se);
        if !cfs_rq_pelt_clock_throttled(cfs_rq) {
            list_add_leaf_cfs_rq(cfs_rq);
        }
        se = (*se).parent;
        while !se.is_null() {
            cfs_rq = b::rust_fair_cfs_rq_of(se);
            update_load_avg(cfs_rq, se, b::RUST_FAIR_UPDATE_TG);
            if !cfs_rq_pelt_clock_throttled(cfs_rq) {
                list_add_leaf_cfs_rq(cfs_rq);
            }
            se = parent_entity(se);
        }
        assert_list_leaf_cfs_rq(b::rust_fair_rq_of(cfs_rq));
    }
}
#[cfg(not(CONFIG_FAIR_GROUP_SCHED))]
unsafe fn propagate_entity_cfs_rq(_se: *mut b::sched_entity) {}
unsafe fn detach_entity_cfs_rq(se: *mut b::sched_entity) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let cfs_rq = b::rust_fair_cfs_rq_of(se);
        if (*se).avg.last_update_time == 0 {
            return;
        }
        update_load_avg(cfs_rq, se, 0);
        detach_entity_load_avg(cfs_rq, se);
        update_tg_load_avg(cfs_rq);
        propagate_entity_cfs_rq(se);
    }
}
unsafe fn attach_entity_cfs_rq(se: *mut b::sched_entity) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let cfs_rq = b::rust_fair_cfs_rq_of(se);
        update_load_avg(
            cfs_rq,
            se,
            if feat!(rust_fair_feat_ATTACH_AGE_LOAD) {
                0
            } else {
                b::RUST_FAIR_SKIP_AGE_LOAD
            },
        );
        attach_entity_load_avg(cfs_rq, se);
        update_tg_load_avg(cfs_rq);
        propagate_entity_cfs_rq(se);
    }
}
unsafe fn detach_task_cfs_rq(p: *mut b::task_struct) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        detach_entity_cfs_rq(addr_of_mut!((*p).se));
    }
}
unsafe fn attach_task_cfs_rq(p: *mut b::task_struct) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        attach_entity_cfs_rq(addr_of_mut!((*p).se));
    }
}
#[export_name = "rust_fair_switching_from_fair"]
unsafe extern "C" fn switching_from_fair(rq: *mut b::rq, p: *mut b::task_struct) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if (*p).se.sched_delayed != 0 {
            b::dequeue_task(
                rq,
                p,
                b::RUST_FAIR_DEQUEUE_SLEEP
                    | b::RUST_FAIR_DEQUEUE_DELAYED
                    | b::RUST_FAIR_DEQUEUE_NOCLOCK,
            );
        }
    }
}
#[export_name = "rust_fair_switched_from_fair"]
unsafe extern "C" fn switched_from_fair(_rq: *mut b::rq, p: *mut b::task_struct) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        detach_task_cfs_rq(p);
    }
}
#[export_name = "rust_fair_switched_to_fair"]
unsafe extern "C" fn switched_to_fair(rq: *mut b::rq, p: *mut b::task_struct) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_warn_switched_to_fair_1((*p).se.sched_delayed != 0);
        attach_task_cfs_rq(p);
        set_task_max_allowed_capacity(p);
        if b::rust_fair_task_on_rq_queued(p) {
            if b::rust_fair_task_current_donor(rq, p) {
                b::resched_curr(rq);
            } else {
                b::wakeup_preempt(rq, p, 0);
            }
        }
    }
}
#[export_name = "rust_fair_set_next_task_fair"]
unsafe extern "C" fn set_next_task_fair(rq: *mut b::rq, p: *mut b::task_struct, first: bool) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut se = addr_of_mut!((*p).se);
        let mut throttled = false;
        let mut cfs_rq = addr_of_mut!((*rq).cfs);
        let mut weight = b::RUST_FAIR_NICE_0_LOAD as c_ulong;
        let on_rq = (*se).on_rq != 0;
        clear_buddies(cfs_rq, se);
        if on_rq {
            __dequeue_entity(cfs_rq, se);
        }
        while !se.is_null() {
            cfs_rq = b::rust_fair_cfs_rq_of(se);
            if !cfg!(CONFIG_FAIR_GROUP_SCHED) || !first || (*cfs_rq).h_curr.is_null() {
                set_next_entity(cfs_rq, se);
            }
            throttled |= account_cfs_rq_runtime(cfs_rq, 0);
            if on_rq {
                weight = __calc_prop_weight(cfs_rq, se, weight);
            }
            se = parent_entity(se);
        }
        if throttled {
            task_throttle_setup_work(p);
        }
        se = addr_of_mut!((*p).se);
        (*cfs_rq).curr = se;
        if on_rq {
            reweight_eevdf(cfs_rq, se, weight, (*se).on_rq != 0);
            if first {
                set_protect_slice(cfs_rq, se);
            }
        }
        if b::rust_fair_task_on_rq_queued(p) {
            b::rust_fair_list_move(
                addr_of_mut!((*se).group_node),
                addr_of_mut!((*rq).cfs_tasks),
            );
        }
        if !first {
            return;
        }
        b::rust_fair_warn_set_next_task_fair_1((*se).sched_delayed != 0);
        if b::rust_fair_hrtick_enabled_fair(rq) {
            hrtick_start_fair(rq, p);
        }
        update_misfit_status(p, rq);
        sched_fair_update_stop_tick(rq, p);
    }
}
#[no_mangle]
pub unsafe extern "C" fn init_cfs_rq(cfs_rq: *mut b::cfs_rq) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_init_tasks_timeline(cfs_rq);
        (*cfs_rq).zero_vruntime = (-(1i64 << 20)) as u64;
        b::rust_fair_init_removed_lock(cfs_rq);
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
#[export_name = "rust_fair_task_change_group_fair"]
unsafe extern "C" fn task_change_group_fair(p: *mut b::task_struct) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if read_once!((*p).__state) == b::RUST_FAIR_TASK_NEW as c_uint {
            return;
        }
        detach_task_cfs_rq(p);
        (*p).se.avg.last_update_time = 0;
        b::rust_fair_set_task_rq(p, b::rust_fair_task_cpu(p) as c_uint);
        attach_task_cfs_rq(p);
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn free_fair_sched_group(tg: *mut b::task_group) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::free_percpu((*tg).cfs_rq.cast());
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn alloc_fair_sched_group(
    tg: *mut b::task_group,
    parent: *mut b::task_group,
) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let state = b::rust_fair_alloc_percpu_cfs_tg_state();
        if state.is_null() {
            return 0;
        }
        (*tg).cfs_rq = addr_of_mut!((*state).cfs_rq);
        (*tg).shares = b::RUST_FAIR_NICE_0_LOAD as c_ulong;
        init_cfs_bandwidth(tg_cfs_bandwidth(tg), tg_cfs_bandwidth(parent));
        fair_each_cpu!(i, b::rust_fair_cpu_possible_mask(), {
            let cfs_rq = b::rust_fair_tg_cfs_rq(tg, i);
            if cfs_rq.is_null() {
                return 0;
            }
            let se = b::rust_fair_tg_se(tg, i);
            init_cfs_rq(cfs_rq);
            init_tg_cfs_entry(tg, cfs_rq, se, i, b::rust_fair_tg_se(parent, i));
            init_entity_runnable_average(se);
        });
        1
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn online_fair_sched_group(tg: *mut b::task_group) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let mut rf: b::rq_flags = core::mem::zeroed();
        fair_each_cpu!(i, b::rust_fair_cpu_possible_mask(), {
            let rq = b::rust_fair_cpu_rq(i);
            let se = b::rust_fair_tg_se(tg, i);
            b::rust_fair_rq_lock_irq(rq, &mut rf);
            b::update_rq_clock(rq);
            attach_entity_cfs_rq(se);
            sync_throttle(tg, i);
            b::rust_fair_rq_unlock_irq(rq, &mut rf);
        });
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn unregister_fair_sched_group(tg: *mut b::task_group) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        destroy_cfs_bandwidth(tg_cfs_bandwidth(tg));
        fair_each_cpu!(cpu, b::rust_fair_cpu_possible_mask(), {
            let cfs_rq = b::rust_fair_tg_cfs_rq(tg, cpu);
            let se = b::rust_fair_tg_se(tg, cpu);
            let rq = b::rust_fair_cpu_rq(cpu);
            if !se.is_null() {
                remove_entity_load_avg(se);
            }
            if (*cfs_rq).on_list != 0 {
                let mut rf: b::rq_flags = core::mem::zeroed();
                b::rust_fair_rq_lock_irqsave(rq, &mut rf);
                list_del_leaf_cfs_rq(cfs_rq);
                b::rust_fair_rq_unlock_irqrestore(rq, &mut rf);
            }
        });
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn init_tg_cfs_entry(
    tg: *mut b::task_group,
    cfs_rq: *mut b::cfs_rq,
    se: *mut b::sched_entity,
    cpu: c_int,
    parent: *mut b::sched_entity,
) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let rq = b::rust_fair_cpu_rq(cpu);
        (*cfs_rq).tg = tg;
        (*cfs_rq).rq = rq;
        init_cfs_rq_runtime(cfs_rq);
        if se.is_null() {
            return;
        }
        if parent.is_null() {
            (*se).cfs_rq = addr_of_mut!((*rq).cfs);
            (*se).depth = 0;
        } else {
            (*se).cfs_rq = (*parent).my_q;
            (*se).depth = (*parent).depth.wrapping_add(1);
        }
        (*se).my_q = cfs_rq;
        update_load_set(addr_of_mut!((*se).load), b::RUST_FAIR_NICE_0_LOAD as c_ulong);
        (*se).parent = parent;
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
unsafe fn __sched_group_set_shares(tg: *mut b::task_group, mut shares: c_ulong) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_lockdep_assert_shares_mutex();
        if b::rust_fair_is_root_task_group(tg) {
            return -(b::EINVAL as c_int);
        }
        shares = fair_clamp_ulong(
            shares,
            b::rust_fair_scale_load(b::RUST_FAIR_MIN_SHARES),
            b::rust_fair_scale_load(b::RUST_FAIR_MAX_SHARES),
        );
        if (*tg).shares == shares {
            return 0;
        }
        (*tg).shares = shares;
        fair_each_cpu!(i, b::rust_fair_cpu_possible_mask(), {
            let rq = b::rust_fair_cpu_rq(i);
            let mut se = b::rust_fair_tg_se(tg, i);
            let mut rf: b::rq_flags = core::mem::zeroed();
            b::rust_fair_rq_lock_irqsave(rq, &mut rf);
            b::update_rq_clock(rq);
            while !se.is_null() {
                update_load_avg(b::rust_fair_cfs_rq_of(se), se, b::RUST_FAIR_UPDATE_TG);
                update_cfs_group(se);
                se = parent_entity(se);
            }
            b::rust_fair_rq_unlock_irqrestore(rq, &mut rf);
        });
        0
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn sched_group_set_shares(tg: *mut b::task_group, shares: c_ulong) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_shares_mutex_lock();
        let ret = if tg_is_idle(tg) != 0 {
            -(b::EINVAL as c_int)
        } else {
            __sched_group_set_shares(tg, shares)
        };
        b::rust_fair_shares_mutex_unlock();
        ret
    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn sched_group_set_idle(tg: *mut b::task_group, idle: c_long) -> c_int {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if tg == b::rust_fair_root_task_group() || idle < 0 || idle > 1 {
            return -(b::EINVAL as c_int);
        }
        b::rust_fair_shares_mutex_lock();
        if (*tg).idle as c_long == idle {
            b::rust_fair_shares_mutex_unlock();
            return 0;
        }
        (*tg).idle = idle as _;
        fair_each_cpu!(i, b::rust_fair_cpu_possible_mask(), {
            let rq = b::rust_fair_cpu_rq(i);
            let mut se = b::rust_fair_tg_se(tg, i);
            let grp_cfs_rq = b::rust_fair_tg_cfs_rq(tg, i);
            let was_idle = cfs_rq_is_idle(grp_cfs_rq);
            let mut rf: b::rq_flags = core::mem::zeroed();
            b::rust_fair_rq_lock_irqsave(rq, &mut rf);
            (*grp_cfs_rq).idle = idle as _;
            if !b::rust_fair_warn_sched_group_set_idle_1(was_idle == cfs_rq_is_idle(grp_cfs_rq)) {
                let mut idle_task_delta = (*grp_cfs_rq)
                    .h_nr_queued
                    .wrapping_sub((*grp_cfs_rq).h_nr_idle)
                    as c_long;
                if cfs_rq_is_idle(grp_cfs_rq) == 0 {
                    idle_task_delta = idle_task_delta.wrapping_neg();
                }
                while !se.is_null() {
                    let cfs_rq = b::rust_fair_cfs_rq_of(se);
                    if (*se).on_rq == 0 {
                        break;
                    }
                    (*cfs_rq).h_nr_idle =
                        ((*cfs_rq).h_nr_idle as c_long).wrapping_add(idle_task_delta) as c_uint;
                    if cfs_rq_is_idle(cfs_rq) != 0 {
                        break;
                    }
                    se = parent_entity(se);
                }
            }
            b::rust_fair_rq_unlock_irqrestore(rq, &mut rf);
        });
        if tg_is_idle(tg) != 0 {
            __sched_group_set_shares(tg, b::rust_fair_scale_load(b::RUST_FAIR_WEIGHT_IDLEPRIO as c_ulong));
        } else {
            __sched_group_set_shares(tg, b::RUST_FAIR_NICE_0_LOAD as c_ulong);
        }
        b::rust_fair_shares_mutex_unlock();
        0
    }
}
#[export_name = "rust_fair_get_rr_interval_fair"]
unsafe extern "C" fn get_rr_interval_fair(rq: *mut b::rq, task: *mut b::task_struct) -> c_uint {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        if (*rq).cfs.load.weight != 0 {
            b::rust_fair_ns_to_jiffies((*task).se.slice) as c_uint
        } else {
            0
        }
    }
}
// DEFINE_SCHED_CLASS(fair) stays a native constant initializer, preserving the
// original linker order/alignment and all CONFIG callback fields. Each supplied
// callback is the concrete Rust function above (reweight/update_curr foundation).
#[no_mangle]
pub unsafe extern "C" fn print_cfs_stats(m: *mut b::seq_file, cpu: c_int) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        b::rust_fair_rcu_read_lock();
        #[cfg(CONFIG_FAIR_GROUP_SCHED)]
        {
            let rq = b::rust_fair_cpu_rq(cpu);
            let head = addr_of_mut!((*rq).leaf_cfs_rq_list);
            let mut node = (*head).next;
            while node != head {
                let cfs_rq = b::rust_fair_cfs_rq_from_leaf(node);
                node = (*node).next;
                b::print_cfs_rq(m, cpu, cfs_rq);
            }
        }
        #[cfg(not(CONFIG_FAIR_GROUP_SCHED))]
        {
            b::print_cfs_rq(m, cpu, addr_of_mut!((*b::rust_fair_cpu_rq(cpu)).cfs));
        }
        b::rust_fair_rcu_read_unlock();
    }
}
#[cfg(CONFIG_NUMA_BALANCING)]
#[no_mangle]
pub unsafe extern "C" fn show_numa_stats(p: *mut b::task_struct, m: *mut b::seq_file) {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        let (mut tsf, mut tpf, mut gsf, mut gpf) = (0, 0, 0, 0);
        b::rust_fair_rcu_read_lock();
        let ng = b::rust_fair_rcu_task_numa_group(p);
        let mut node = b::rust_fair_first_online_node();
        while node < b::RUST_FAIR_MAX_NUMNODES {
            if !(*p).numa_faults.is_null() {
                tsf = *(*p)
                    .numa_faults
                    .add(task_faults_idx(b::NUMA_MEM, node, 0) as usize);
                tpf = *(*p)
                    .numa_faults
                    .add(task_faults_idx(b::NUMA_MEM, node, 1) as usize);
            }
            if !ng.is_null() {
                gsf = *b::rust_fair_numa_group_faults(ng)
                    .add(task_faults_idx(b::NUMA_MEM, node, 0) as usize);
                gpf = *b::rust_fair_numa_group_faults(ng)
                    .add(task_faults_idx(b::NUMA_MEM, node, 1) as usize);
            }
            b::print_numa_stats(m, node, tsf, tpf, gsf, gpf);
            node = b::rust_fair_next_online_node(node);
        }
        b::rust_fair_rcu_read_unlock();
    }
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn init_sched_fair_class() {
    // SAFETY: The unsafe caller supplies native object lifetimes and the
    // original C caller-side synchronization required by this operation.
    unsafe {
        fair_each_cpu!(i, b::rust_fair_cpu_possible_mask(), {
            let node = b::rust_fair_cpu_to_node(i);
            b::rust_fair_zalloc_load_balance_mask_node(i, node);
            b::rust_fair_zalloc_select_rq_mask_node(i, node);
            b::rust_fair_zalloc_should_we_balance_tmpmask_node(i, node);
            #[cfg(CONFIG_CFS_BANDWIDTH)]
            {
                let rq = b::rust_fair_cpu_rq(i);
                b::rust_fair_init_cfsb_csd(rq, Some(__cfsb_csd_unthrottle));
                b::rust_fair_init_list_head(addr_of_mut!((*rq).cfsb_csd_list));
            }
        });
        b::rust_fair_open_sched_softirq();
        #[cfg(CONFIG_NO_HZ_COMMON)]
        {
            let nohz = b::rust_fair_nohz();
            (*nohz).next_balance = b::rust_fair_jiffies();
            (*nohz).next_blocked = b::rust_fair_jiffies();
            b::rust_fair_zalloc_nohz_idle_mask();
        }
    }
}
