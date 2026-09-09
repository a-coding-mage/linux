// SPDX-License-Identifier: GPL-2.0
/*
 * Debug controller
 *
 * WARNING: This controller is for cgroup core debugging only.
 * Its interfaces are unstable and subject to changes at any time.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

unsafe fn debug_css_alloc(parent_css: *mut cgroup_subsys_state) -> *mut cgroup_subsys_state {
    let css = kzalloc_obj::<cgroup_subsys_state>();
    if css.is_null() {
        return ERR_PTR(-ENOMEM);
    }
    css
}

unsafe fn debug_css_free(css: *mut cgroup_subsys_state) {
    kfree(css);
}

/*
 * debug_taskcount_read - return the number of tasks in a cgroup.
 * @cgrp: the cgroup in question
 */
unsafe fn debug_taskcount_read(
    css: *mut cgroup_subsys_state,
    cft: *mut cftype,
) -> u64 {
    cgroup_task_count((*css).cgroup)
}

unsafe fn current_css_set_read(seq: *mut seq_file, v: *mut core::ffi::c_void) -> i32 {
    let of = (*seq).private as *mut kernfs_open_file;
    let mut cset: *mut css_set;
    let mut ss: *mut cgroup_subsys;
    let mut css: *mut cgroup_subsys_state;
    let mut i: i32 = 0;
    let refcnt: i32;

    if !cgroup_kn_lock_live((*of).kn, false) {
        return -ENODEV;
    }

    spin_lock_irq(&mut css_set_lock);
    cset = task_css_set(current);
    refcnt = refcount_read(&(*cset).refcount);
    seq_printf(seq, "css_set %pK %d", cset, refcnt);
    if refcnt > (*cset).nr_tasks {
        seq_printf(seq, " +%d", refcnt - (*cset).nr_tasks);
    }
    seq_puts(seq, "\n");

    /* Print the css'es stored in the current css_set. */
    for_each_subsys!(ss, i) {
        css = (*cset).subsys[(*ss).id as usize];
        if css.is_null() {
            continue;
        }
        seq_printf(seq, "%2d: %-4s\t- %p[%d]\n", (*ss).id, (*ss).name, css, (*css).id);
    }
    spin_unlock_irq(&mut css_set_lock);
    cgroup_kn_unlock((*of).kn);
    0
}

unsafe fn current_css_set_refcount_read(css: *mut cgroup_subsys_state, cft: *mut cftype) -> u64 {
    let count: u64;
    rcu_read_lock();
    count = refcount_read(&(*task_css_set(current)).refcount) as u64;
    rcu_read_unlock();
    count
}

unsafe fn current_css_set_cg_links_read(seq: *mut seq_file, v: *mut core::ffi::c_void) -> i32 {
    let mut name_buf = kmalloc(NAME_MAX + 1, GFP_KERNEL);
    if name_buf.is_null() { return -ENOMEM; }
    spin_lock_irq(&mut css_set_lock);
    let cset = task_css_set(current);
    list_for_each_entry!(link, &(*cset).cgrp_links, cgrp_link) {
        let c = (*link).cgrp;
        cgroup_name(c, name_buf, NAME_MAX + 1);
        seq_printf(seq, "Root %d group %s\n", (*(*c).root).hierarchy_id, name_buf);
    }
    spin_unlock_irq(&mut css_set_lock);
    kfree(name_buf);
    0
}

const MAX_TASKS_SHOWN_PER_CSS: i32 = 25;

unsafe fn cgroup_css_links_read(seq: *mut seq_file, v: *mut core::ffi::c_void) -> i32 {
    let css = seq_css(seq);
    let mut dead_cnt = 0;
    let mut extra_refs = 0;
    let mut threaded_csets = 0;
    spin_lock_irq(&mut css_set_lock);
    list_for_each_entry!(link, &(*(*css).cgroup).cset_links, cset_link) {
        let cset = (*link).cset;
        let mut count = 0;
        let refcnt = refcount_read(&(*cset).refcount);
        seq_printf(seq, "css_set %pK", cset);
        if rcu_dereference_protected!((*cset).dom_cset, 1) != cset {
            threaded_csets += 1;
            seq_printf(seq, "=>%pK", (*cset).dom_cset);
        }
        if !list_empty(&(*cset).threaded_csets) {
            let mut idx = 0;
            list_for_each_entry!(tcset, &(*cset).threaded_csets, threaded_csets_node) {
                seq_puts(seq, if idx != 0 { "," } else { "<=" });
                seq_printf(seq, "%pK", tcset);
                idx += 1;
            }
        } else {
            seq_printf(seq, " %d", refcnt);
            if refcnt - (*cset).nr_tasks > 0 {
                let mut extra = refcnt - (*cset).nr_tasks;
                seq_printf(seq, " +%d", extra);
                /* Take out the one additional reference in init_css_set. */
                if cset == &mut init_css_set { extra -= 1; }
                extra_refs += extra;
            }
        }
        seq_puts(seq, "\n");
        list_for_each_entry!(task, &(*cset).tasks, cg_list) {
            if count <= MAX_TASKS_SHOWN_PER_CSS { seq_printf(seq, "  task %d\n", task_pid_vnr(task)); }
            count += 1;
        }
        list_for_each_entry!(task, &(*cset).mg_tasks, cg_list) {
            if count <= MAX_TASKS_SHOWN_PER_CSS { seq_printf(seq, "  task %d\n", task_pid_vnr(task)); }
            count += 1;
        }
        if count > MAX_TASKS_SHOWN_PER_CSS { seq_printf(seq, "  ... (%d)\n", count - MAX_TASKS_SHOWN_PER_CSS); }
        if (*cset).dead { seq_puts(seq, "    [dead]\n"); dead_cnt += 1; }
        WARN_ON(count != (*cset).nr_tasks);
    }
    spin_unlock_irq(&mut css_set_lock);
    if dead_cnt == 0 && extra_refs == 0 && threaded_csets == 0 { return 0; }
    seq_puts(seq, "\n");
    if threaded_csets != 0 { seq_printf(seq, "threaded css_sets = %d\n", threaded_csets); }
    if extra_refs != 0 { seq_printf(seq, "extra references = %d\n", extra_refs); }
    if dead_cnt != 0 { seq_printf(seq, "dead css_sets = %d\n", dead_cnt); }
    0
}

unsafe fn cgroup_subsys_states_read(seq: *mut seq_file, v: *mut core::ffi::c_void) -> i32 {
    let of = (*seq).private as *mut kernfs_open_file;
    let cgrp = cgroup_kn_lock_live((*of).kn, false);
    if cgrp.is_null() { return -ENODEV; }
    let mut pbuf = [0i8; 16];
    let mut i = 0;
    for_each_subsys!(ss, i) {
        let css = rcu_dereference_check!((*cgrp).subsys[(*ss).id as usize], true);
        if css.is_null() { continue; }
        pbuf[0] = 0;
        if !(*css).parent.is_null() { snprintf(pbuf.as_mut_ptr(), pbuf.len() - 1, " P=%d", (*(*css).parent).id); }
        seq_printf(seq, "%2d: %-4s\t- %p[%d] %d%s\n", (*ss).id, (*ss).name, css, (*css).id, atomic_read(&(*css).online_cnt), pbuf.as_ptr());
    }
    cgroup_kn_unlock((*of).kn);
    0
}

unsafe fn cgroup_masks_read_one(seq: *mut seq_file, name: *const i8, mask: u32) {
    let mut first = true;
    seq_printf(seq, "%-17s: ", name);
    let mut ssid = 0;
    for_each_subsys!(ss, ssid) {
        if mask & (1u32 << ssid) == 0 { continue; }
        if !first { seq_puts(seq, ", "); }
        seq_puts(seq, (*ss).name);
        first = false;
    }
    seq_putc(seq, '\n' as i32);
}

unsafe fn cgroup_masks_read(seq: *mut seq_file, v: *mut core::ffi::c_void) -> i32 {
    let of = (*seq).private as *mut kernfs_open_file;
    let cgrp = cgroup_kn_lock_live((*of).kn, false);
    if cgrp.is_null() { return -ENODEV; }
    cgroup_masks_read_one(seq, c"subtree_control".as_ptr(), (*cgrp).subtree_control);
    cgroup_masks_read_one(seq, c"subtree_ss_mask".as_ptr(), (*cgrp).subtree_ss_mask);
    cgroup_kn_unlock((*of).kn);
    0
}

unsafe fn releasable_read(css: *mut cgroup_subsys_state, cft: *mut cftype) -> u64 {
    (!cgroup_is_populated((*css).cgroup) && !css_has_online_children(&(*(*css).cgroup).self_)) as u64
}

// The following tables preserve the C cftype layout and callback wiring.
static mut debug_legacy_files: [cftype; 9] = [
    cftype { name: c"taskcount".as_ptr(), read_u64: Some(debug_taskcount_read), ..CFTYPE_ZERO },
    cftype { name: c"current_css_set".as_ptr(), seq_show: Some(current_css_set_read), flags: CFTYPE_ONLY_ON_ROOT, ..CFTYPE_ZERO },
    cftype { name: c"current_css_set_refcount".as_ptr(), read_u64: Some(current_css_set_refcount_read), flags: CFTYPE_ONLY_ON_ROOT, ..CFTYPE_ZERO },
    cftype { name: c"current_css_set_cg_links".as_ptr(), seq_show: Some(current_css_set_cg_links_read), flags: CFTYPE_ONLY_ON_ROOT, ..CFTYPE_ZERO },
    cftype { name: c"cgroup_css_links".as_ptr(), seq_show: Some(cgroup_css_links_read), ..CFTYPE_ZERO },
    cftype { name: c"cgroup_subsys_states".as_ptr(), seq_show: Some(cgroup_subsys_states_read), ..CFTYPE_ZERO },
    cftype { name: c"cgroup_masks".as_ptr(), seq_show: Some(cgroup_masks_read), ..CFTYPE_ZERO },
    cftype { name: c"releasable".as_ptr(), read_u64: Some(releasable_read), ..CFTYPE_ZERO },
    CFTYPE_ZERO,
];

static mut debug_files: [cftype; 8] = [
    cftype { name: c"taskcount".as_ptr(), read_u64: Some(debug_taskcount_read), ..CFTYPE_ZERO },
    cftype { name: c"current_css_set".as_ptr(), seq_show: Some(current_css_set_read), flags: CFTYPE_ONLY_ON_ROOT, ..CFTYPE_ZERO },
    cftype { name: c"current_css_set_refcount".as_ptr(), read_u64: Some(current_css_set_refcount_read), flags: CFTYPE_ONLY_ON_ROOT, ..CFTYPE_ZERO },
    cftype { name: c"current_css_set_cg_links".as_ptr(), seq_show: Some(current_css_set_cg_links_read), flags: CFTYPE_ONLY_ON_ROOT, ..CFTYPE_ZERO },
    cftype { name: c"css_links".as_ptr(), seq_show: Some(cgroup_css_links_read), ..CFTYPE_ZERO },
    cftype { name: c"csses".as_ptr(), seq_show: Some(cgroup_subsys_states_read), ..CFTYPE_ZERO },
    cftype { name: c"masks".as_ptr(), seq_show: Some(cgroup_masks_read), ..CFTYPE_ZERO },
    CFTYPE_ZERO,
];

static mut debug_cgrp_subsys: cgroup_subsys = cgroup_subsys {
    css_alloc: Some(debug_css_alloc),
    css_free: Some(debug_css_free),
    legacy_cftypes: debug_legacy_files.as_mut_ptr(),
    ..CGROUP_SUBSYS_ZERO
};

/* On v2, debug is an implicit controller enabled by "cgroup_debug" boot parameter. */
unsafe fn enable_debug_cgroup() {
    debug_cgrp_subsys.dfl_cftypes = debug_files.as_mut_ptr();
    debug_cgrp_subsys.implicit_on_dfl = true;
    debug_cgrp_subsys.threaded = true;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
