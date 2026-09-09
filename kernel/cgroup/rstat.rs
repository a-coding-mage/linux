// SPDX-License-Identifier: GPL-2.0-only
// Translated from rstat.c. Kernel-provided types, functions, and macros are
// intentionally referenced as external dependencies.

static mut rstat_base_lock: spinlock_t = unsafe { core::mem::zeroed() };
static mut rstat_backlog_list: per_cpu<llist_head> = unsafe { core::mem::zeroed() };

unsafe fn css_uses_rstat(css: *mut cgroup_subsys_state) -> bool {
    css_is_self(css) || !(*(*css).ss).css_rstat_flush.is_null()
}

unsafe fn css_rstat_cpu(css: *mut cgroup_subsys_state, cpu: i32) -> *mut css_rstat_cpu {
    per_cpu_ptr((*css).rstat_cpu, cpu)
}
unsafe fn cgroup_rstat_base_cpu(cgrp: *mut cgroup, cpu: i32) -> *mut cgroup_rstat_base_cpu {
    per_cpu_ptr((*cgrp).rstat_base_cpu, cpu)
}
unsafe fn ss_rstat_lock(ss: *mut cgroup_subsys) -> *mut spinlock_t {
    if !ss.is_null() { &mut (*ss).rstat_ss_lock } else { &raw mut rstat_base_lock }
}
unsafe fn ss_lhead_cpu(ss: *mut cgroup_subsys, cpu: i32) -> *mut llist_head {
    if !ss.is_null() { per_cpu_ptr((*ss).lhead, cpu) } else { per_cpu_ptr(&raw mut rstat_backlog_list, cpu) }
}

pub unsafe fn __css_rstat_updated(css: *mut cgroup_subsys_state, cpu: i32) {
    if !css_uses_rstat(css) { return; }
    lockdep_assert_preemption_disabled();
    if !IS_ENABLED(CONFIG_ARCH_HAVE_NMI_SAFE_CMPXCHG) && in_nmi() { return; }
    let rstatc = css_rstat_cpu(css, cpu);
    if llist_on_list(&mut (*rstatc).lnode) { return; }
    let mut self_node: *mut llist_node = &mut (*rstatc).lnode;
    if !try_cmpxchg(&mut (*rstatc).lnode.next, &mut self_node, core::ptr::null_mut()) { return; }
    llist_add(&mut (*rstatc).lnode, ss_lhead_cpu((*css).ss, cpu));
}

pub unsafe fn css_rstat_updated(css: *mut cgroup_subsys_state, cpu: i32) {
    if unlikely(cpu < 0 || cpu >= nr_cpu_ids || !cpu_possible(cpu)) { return; }
    __css_rstat_updated(css, cpu);
}

unsafe fn __css_process_update_tree(mut css: *mut cgroup_subsys_state, cpu: i32) {
    loop {
        let rstatc = css_rstat_cpu(css, cpu);
        let parent = (*css).parent;
        if !(*rstatc).updated_next.is_null() { break; }
        if parent.is_null() { (*rstatc).updated_next = css; break; }
        let prstatc = css_rstat_cpu(parent, cpu);
        (*rstatc).updated_next = (*prstatc).updated_children;
        (*prstatc).updated_children = css;
        css = parent;
    }
}

unsafe fn css_process_update_tree(ss: *mut cgroup_subsys, cpu: i32) {
    let lhead = ss_lhead_cpu(ss, cpu);
    while { let lnode = llist_del_first_init(lhead); if lnode.is_null() { false } else {
        let rstatc = container_of(lnode, css_rstat_cpu, lnode);
        __css_process_update_tree((*rstatc).owner, cpu); true
    }} {}
}

unsafe fn css_rstat_push_children(mut head: *mut cgroup_subsys_state, mut child: *mut cgroup_subsys_state, cpu: i32) -> *mut cgroup_subsys_state {
    let mut cnext = child;
    let mut ghead: *mut cgroup_subsys_state = core::ptr::null_mut();
    let mut parent: *mut cgroup_subsys_state;
    let mut grandchild: *mut cgroup_subsys_state;
    (*child).rstat_flush_next = core::ptr::null_mut();
    lockdep_assert_held(ss_rstat_lock((*head).ss));
    'next_level: loop {
        while !cnext.is_null() {
            child = cnext; cnext = (*child).rstat_flush_next; parent = (*child).parent;
            while child != parent {
                (*child).rstat_flush_next = head; head = child;
                let crstatc = css_rstat_cpu(child, cpu); grandchild = (*crstatc).updated_children;
                if grandchild != child { (*crstatc).updated_children = child; (*grandchild).rstat_flush_next = ghead; ghead = grandchild; }
                child = (*crstatc).updated_next; (*crstatc).updated_next = core::ptr::null_mut();
            }
        }
        if ghead.is_null() { break 'next_level; }
        cnext = ghead; ghead = core::ptr::null_mut();
    }
    head
}

unsafe fn css_rstat_updated_list(root: *mut cgroup_subsys_state, cpu: i32) -> *mut cgroup_subsys_state {
    let rstatc = css_rstat_cpu(root, cpu); css_process_update_tree((*root).ss, cpu);
    if (*rstatc).updated_next.is_null() { return core::ptr::null_mut(); }
    let parent = (*root).parent;
    if !parent.is_null() {
        let prstatc = css_rstat_cpu(parent, cpu); let mut nextp = &mut (*prstatc).updated_children as *mut *mut cgroup_subsys_state;
        while *nextp != root { let nrstatc = css_rstat_cpu(*nextp, cpu); WARN_ON_ONCE(*nextp == parent); nextp = &mut (*nrstatc).updated_next; }
        *nextp = (*rstatc).updated_next;
    }
    (*rstatc).updated_next = core::ptr::null_mut();
    let mut head = root; (*root).rstat_flush_next = core::ptr::null_mut();
    let child = (*rstatc).updated_children; (*rstatc).updated_children = root;
    if child != root { head = css_rstat_push_children(head, child, cpu); }
    head
}

#[no_mangle] pub unsafe extern "C" fn bpf_rstat_flush(_cgrp: *mut cgroup, _parent: *mut cgroup, _cpu: i32) {}

unsafe fn __css_rstat_lock(css: *mut cgroup_subsys_state, cpu_in_loop: i32) {
    let cgrp = (*css).cgroup; let lock = ss_rstat_lock((*css).ss); let contended = !spin_trylock_irq(lock);
    if contended { trace_cgroup_rstat_lock_contended(cgrp, cpu_in_loop, contended); spin_lock_irq(lock); }
    trace_cgroup_rstat_locked(cgrp, cpu_in_loop, contended);
}
unsafe fn __css_rstat_unlock(css: *mut cgroup_subsys_state, cpu_in_loop: i32) {
    let cgrp = (*css).cgroup; let lock = ss_rstat_lock((*css).ss);
    trace_cgroup_rstat_unlock(cgrp, cpu_in_loop, false); spin_unlock_irq(lock);
}

pub unsafe fn css_rstat_flush(css: *mut cgroup_subsys_state) {
    if !css_uses_rstat(css) { return; } might_sleep(); let is_self = css_is_self(css);
    for_each_possible_cpu!(cpu, { __css_rstat_lock(css, cpu); let mut pos = css_rstat_updated_list(css, cpu);
        while !pos.is_null() { if is_self { cgroup_base_stat_flush((*pos).cgroup, cpu); bpf_rstat_flush((*pos).cgroup, cgroup_parent((*pos).cgroup), cpu); } else { ((*(*pos).ss).css_rstat_flush)(pos, cpu); } pos = (*pos).rstat_flush_next; }
        __css_rstat_unlock(css, cpu); if !cond_resched() { cpu_relax(); }
    });
}

pub unsafe fn css_rstat_init(css: *mut cgroup_subsys_state) -> i32 {
    let cgrp = (*css).cgroup; let is_self = css_is_self(css);
    if is_self { if (*cgrp).rstat_base_cpu.is_null() { (*cgrp).rstat_base_cpu = alloc_percpu::<cgroup_rstat_base_cpu>(); if (*cgrp).rstat_base_cpu.is_null() { return -ENOMEM; } } }
    else if (*(*css).ss).css_rstat_flush.is_null() { return 0; }
    if (*css).rstat_cpu.is_null() { (*css).rstat_cpu = alloc_percpu::<css_rstat_cpu>(); if (*css).rstat_cpu.is_null() { if is_self { free_percpu((*cgrp).rstat_base_cpu); } return -ENOMEM; } }
    for_each_possible_cpu!(cpu, { let rstatc = css_rstat_cpu(css, cpu); (*rstatc).owner = css; (*rstatc).updated_children = css; init_llist_node(&mut (*rstatc).lnode); if is_self { u64_stats_init(&mut (*cgroup_rstat_base_cpu(cgrp, cpu)).bsync); } }); 0
}

pub unsafe fn css_rstat_exit(css: *mut cgroup_subsys_state) {
    if !css_uses_rstat(css) || (*css).rstat_cpu.is_null() { return; } css_rstat_flush(css);
    for_each_possible_cpu!(cpu, { let rstatc = css_rstat_cpu(css, cpu); if WARN_ON_ONCE((*rstatc).updated_children != css) || WARN_ON_ONCE(!(*rstatc).updated_next.is_null()) { return; } });
    if css_is_self(css) { let cgrp = (*css).cgroup; free_percpu((*cgrp).rstat_base_cpu); (*cgrp).rstat_base_cpu = core::ptr::null_mut(); }
    free_percpu((*css).rstat_cpu); (*css).rstat_cpu = core::ptr::null_mut();
}

pub unsafe fn ss_rstat_init(ss: *mut cgroup_subsys) -> i32 {
    if !ss.is_null() { (*ss).lhead = alloc_percpu::<llist_head>(); if (*ss).lhead.is_null() { return -ENOMEM; } }
    spin_lock_init(ss_rstat_lock(ss)); for_each_possible_cpu!(cpu, { init_llist_head(ss_lhead_cpu(ss, cpu)); }); 0
}

unsafe fn cgroup_base_stat_add(dst: *mut cgroup_base_stat, src: *mut cgroup_base_stat) { (*dst).cputime.utime += (*src).cputime.utime; (*dst).cputime.stime += (*src).cputime.stime; (*dst).cputime.sum_exec_runtime += (*src).cputime.sum_exec_runtime; #[cfg(CONFIG_SCHED_CORE)] { (*dst).forceidle_sum += (*src).forceidle_sum; } (*dst).ntime += (*src).ntime; }
unsafe fn cgroup_base_stat_sub(dst: *mut cgroup_base_stat, src: *mut cgroup_base_stat) { (*dst).cputime.utime -= (*src).cputime.utime; (*dst).cputime.stime -= (*src).cputime.stime; (*dst).cputime.sum_exec_runtime -= (*src).cputime.sum_exec_runtime; #[cfg(CONFIG_SCHED_CORE)] { (*dst).forceidle_sum -= (*src).forceidle_sum; } (*dst).ntime -= (*src).ntime; }

unsafe fn cgroup_base_stat_flush(cgrp: *mut cgroup, cpu: i32) {
    let rstatbc = cgroup_rstat_base_cpu(cgrp, cpu); let parent = cgroup_parent(cgrp); if parent.is_null() { return; }
    let mut delta: cgroup_base_stat; let seq: u32; loop { seq = __u64_stats_fetch_begin(&(*rstatbc).bsync); delta = (*rstatbc).bstat; if !__u64_stats_fetch_retry(&(*rstatbc).bsync, seq) { break; } }
    cgroup_base_stat_sub(&mut delta, &mut (*rstatbc).last_bstat); cgroup_base_stat_add(&mut (*cgrp).bstat, &mut delta); cgroup_base_stat_add(&mut (*rstatbc).last_bstat, &mut delta); cgroup_base_stat_add(&mut (*rstatbc).subtree_bstat, &mut delta);
    if !cgroup_parent(parent).is_null() { delta = (*cgrp).bstat; cgroup_base_stat_sub(&mut delta, &mut (*cgrp).last_bstat); cgroup_base_stat_add(&mut (*parent).bstat, &mut delta); cgroup_base_stat_add(&mut (*cgrp).last_bstat, &mut delta); delta = (*rstatbc).subtree_bstat; let prstatbc = cgroup_rstat_base_cpu(parent, cpu); cgroup_base_stat_sub(&mut delta, &mut (*rstatbc).last_subtree_bstat); cgroup_base_stat_add(&mut (*prstatbc).subtree_bstat, &mut delta); cgroup_base_stat_add(&mut (*rstatbc).last_subtree_bstat, &mut delta); }
}

unsafe fn cgroup_base_stat_cputime_account_begin(cgrp: *mut cgroup, flags: *mut ulong) -> *mut cgroup_rstat_base_cpu { let r = get_cpu_ptr((*cgrp).rstat_base_cpu); *flags = u64_stats_update_begin_irqsave(&mut (*r).bsync); r }
unsafe fn cgroup_base_stat_cputime_account_end(cgrp: *mut cgroup, r: *mut cgroup_rstat_base_cpu, flags: ulong) { u64_stats_update_end_irqrestore(&mut (*r).bsync, flags); __css_rstat_updated(&mut (*cgrp).self_, smp_processor_id()); put_cpu_ptr(r); }
pub unsafe fn __cgroup_account_cputime(cgrp: *mut cgroup, delta_exec: u64) { let mut flags = 0; let r = cgroup_base_stat_cputime_account_begin(cgrp, &mut flags); (*r).bstat.cputime.sum_exec_runtime += delta_exec; cgroup_base_stat_cputime_account_end(cgrp, r, flags); }
pub unsafe fn __cgroup_account_cputime_field(cgrp: *mut cgroup, index: cpu_usage_stat, delta_exec: u64) { let mut flags = 0; let r = cgroup_base_stat_cputime_account_begin(cgrp, &mut flags); match index { CPUTIME_NICE => { (*r).bstat.ntime += delta_exec; (*r).bstat.cputime.utime += delta_exec; }, CPUTIME_USER => (*r).bstat.cputime.utime += delta_exec, CPUTIME_SYSTEM | CPUTIME_IRQ | CPUTIME_SOFTIRQ => (*r).bstat.cputime.stime += delta_exec, #[cfg(CONFIG_SCHED_CORE)] CPUTIME_FORCEIDLE => (*r).bstat.forceidle_sum += delta_exec, _ => {} } cgroup_base_stat_cputime_account_end(cgrp, r, flags); }

unsafe fn root_cgroup_cputime(bstat: *mut cgroup_base_stat) { let cputime = &mut (*bstat).cputime; core::ptr::write_bytes(bstat, 0, 1); for_each_possible_cpu!(i, { let mut kcpustat: kernel_cpustat = core::mem::zeroed(); kcpustat_cpu_fetch(&mut kcpustat, i); let user = kcpustat.cpustat[CPUTIME_USER] + kcpustat.cpustat[CPUTIME_NICE]; cputime.utime += user; let sys = kcpustat.cpustat[CPUTIME_SYSTEM] + kcpustat.cpustat[CPUTIME_IRQ] + kcpustat.cpustat[CPUTIME_SOFTIRQ]; cputime.stime += sys; cputime.sum_exec_runtime += user + sys; #[cfg(CONFIG_SCHED_CORE)] { (*bstat).forceidle_sum += kcpustat.cpustat[CPUTIME_FORCEIDLE]; } (*bstat).ntime += kcpustat.cpustat[CPUTIME_NICE]; }); }
unsafe fn cgroup_force_idle_show(seq: *mut seq_file, bstat: *mut cgroup_base_stat) { #[cfg(CONFIG_SCHED_CORE)] { let mut forceidle_time = (*bstat).forceidle_sum; do_div(&mut forceidle_time, NSEC_PER_USEC); seq_printf(seq, "core_sched.force_idle_usec %llu\n", forceidle_time); } }
pub unsafe fn cgroup_base_stat_cputime_show(seq: *mut seq_file) { let cgrp = (*seq_css(seq)).cgroup; let mut bstat: cgroup_base_stat; if !cgroup_parent(cgrp).is_null() { css_rstat_flush(&mut (*cgrp).self_); __css_rstat_lock(&mut (*cgrp).self_, -1); bstat = (*cgrp).bstat; cputime_adjust(&mut (*cgrp).bstat.cputime, &mut (*cgrp).prev_cputime, &mut bstat.cputime.utime, &mut bstat.cputime.stime); __css_rstat_unlock(&mut (*cgrp).self_, -1); } else { bstat = core::mem::zeroed(); root_cgroup_cputime(&mut bstat); } do_div(&mut bstat.cputime.sum_exec_runtime, NSEC_PER_USEC); do_div(&mut bstat.cputime.utime, NSEC_PER_USEC); do_div(&mut bstat.cputime.stime, NSEC_PER_USEC); do_div(&mut bstat.ntime, NSEC_PER_USEC); seq_printf(seq, "usage_usec %llu\nuser_usec %llu\nsystem_usec %llu\nnice_usec %llu\n", bstat.cputime.sum_exec_runtime, bstat.cputime.utime, bstat.cputime.stime, bstat.ntime); cgroup_force_idle_show(seq, &mut bstat); }

// BTF kfunc registration and late_initcall are supplied by the kernel build.
static bpf_rstat_kfunc_set: btf_kfunc_id_set = unsafe { core::mem::zeroed() };
unsafe fn bpf_rstat_kfunc_init() -> i32 { register_btf_kfunc_id_set(BPF_PROG_TYPE_TRACING, &bpf_rstat_kfunc_set) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
