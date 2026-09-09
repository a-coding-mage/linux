// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * net/core/netprio_cgroup.c\tPriority Control Group
 *
 * Authors:\tNeil Horman <nhorman@tuxdriver.com>
 */

// #define pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// C dependencies are supplied by the surrounding kernel translation.

const NETPRIO_ID_MAX: u32 = u16::MAX as u32;
const PRIOMAP_MIN_SZ: usize = 128;

/*
 * Extend @dev->priomap so that it's large enough to accommodate
 * @target_idx.  @dev->priomap.priomap_len > @target_idx after successful
 * return.  Must be called under rtnl lock.
 */
unsafe fn extend_netdev_table(dev: *mut net_device, target_idx: u32) -> i32 {
    let old: *mut netprio_map = rtnl_dereference((*dev).priomap);
    if !old.is_null() && (*old).priomap_len > target_idx {
        return 0;
    }

    let mut new_sz = PRIOMAP_MIN_SZ;
    let new_len: usize;
    loop {
        new_len = (new_sz - core::mem::offset_of!(netprio_map, priomap))
            / core::mem::size_of::<u32>();
        if new_len > target_idx as usize {
            break;
        }
        new_sz = new_sz.wrapping_mul(2);
        if new_sz < PRIOMAP_MIN_SZ {
            if WARN_ON(true) {
                return -ENOSPC;
            }
        }
    }

    let new: *mut netprio_map = kzalloc(new_sz, GFP_KERNEL);
    if new.is_null() {
        return -ENOMEM;
    }

    if !old.is_null() {
        core::ptr::copy_nonoverlapping(
            (*old).priomap.as_ptr(),
            (*new).priomap.as_mut_ptr(),
            (*old).priomap_len as usize,
        );
    }

    (*new).priomap_len = new_len as u32;
    rcu_assign_pointer((*dev).priomap, new);
    if !old.is_null() {
        kfree_rcu(old, rcu);
    }
    0
}

/// Return the effective netprio of a cgroup-net_device pair.
unsafe fn netprio_prio(css: *mut cgroup_subsys_state, dev: *mut net_device) -> u32 {
    let map: *mut netprio_map = rcu_dereference_rtnl((*dev).priomap);
    let id = (*css).id;
    if !map.is_null() && id < (*map).priomap_len {
        return (*map).priomap[id as usize];
    }
    0
}

/// Set netprio on a cgroup-net_device pair.
unsafe fn netprio_set_prio(
    css: *mut cgroup_subsys_state,
    dev: *mut net_device,
    prio: u32,
) -> i32 {
    let mut map: *mut netprio_map;
    let id = (*css).id;

    map = rtnl_dereference((*dev).priomap);
    if prio == 0 && (map.is_null() || (*map).priomap_len <= id) {
        return 0;
    }

    let ret = extend_netdev_table(dev, id);
    if ret != 0 {
        return ret;
    }
    map = rtnl_dereference((*dev).priomap);
    (*map).priomap[id as usize] = prio;
    0
}

unsafe fn cgrp_css_alloc(parent_css: *mut cgroup_subsys_state) -> *mut cgroup_subsys_state {
    let css = kzalloc_obj::<cgroup_subsys_state>();
    if css.is_null() { ERR_PTR(-ENOMEM) } else { css }
}

unsafe fn cgrp_css_online(css: *mut cgroup_subsys_state) -> i32 {
    let parent_css = (*css).parent;
    let mut ret = 0;
    if (*css).id > NETPRIO_ID_MAX { return -ENOSPC; }
    if parent_css.is_null() { return 0; }

    rtnl_lock();
    // Inherit prios from the parent.  As all prios are set during onlining,
    // there is no need to clear them on offline.
    for_each_netdev(&init_net, |dev: *mut net_device| {
        let prio = netprio_prio(parent_css, dev);
        ret = netprio_set_prio(css, dev, prio);
        ret == 0
    });
    rtnl_unlock();
    ret
}

unsafe fn cgrp_css_free(css: *mut cgroup_subsys_state) { kfree(css); }

unsafe fn read_prioidx(css: *mut cgroup_subsys_state, _cft: *mut cftype) -> u64 { (*css).id as u64 }

unsafe fn read_priomap(sf: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    rcu_read_lock();
    for_each_netdev_rcu(&init_net, |dev: *mut net_device| {
        seq_printf(sf, "%s %u\n", (*dev).name.as_ptr(), netprio_prio(seq_css(sf), dev));
        true
    });
    rcu_read_unlock();
    0
}

unsafe fn write_priomap(
    of: *mut kernfs_open_file, buf: *mut u8, nbytes: usize, _off: loff_t,
) -> isize {
    let mut devname = [0u8; IFNAMSIZ + 1];
    let mut prio = 0u32;
    if sscanf(buf, "%" /* __stringify(IFNAMSIZ) */ "s %u", devname.as_mut_ptr(), &mut prio) != 2 {
        return -EINVAL as isize;
    }
    let dev = dev_get_by_name(&init_net, devname.as_ptr());
    if dev.is_null() { return -ENODEV as isize; }
    rtnl_lock();
    let ret = netprio_set_prio(of_css(of), dev, prio);
    rtnl_unlock();
    dev_put(dev);
    if ret != 0 { ret as isize } else { nbytes as isize }
}

unsafe fn update_netprio(v: *const core::ffi::c_void, file: *mut file, _n: u32) -> i32 {
    let sock = sock_from_file(file);
    if !sock.is_null() { sock_cgroup_set_prioidx(&mut (*sock).sk.sk_cgrp_data, v as usize as u64); }
    0
}

unsafe fn net_prio_attach(tset: *mut cgroup_taskset) {
    cgroup_taskset_for_each(tset, |p: *mut task_struct, css: *mut cgroup_subsys_state| {
        let v = css_id(css) as usize as *const core::ffi::c_void;
        task_lock(p);
        iterate_fd((*p).files, 0, update_netprio, v);
        task_unlock(p);
        true
    });
}

static mut SS_FILES: [cftype; 3] = [
    cftype { name: "prioidx", read_u64: Some(read_prioidx), ..CFTYPE_ZERO },
    cftype { name: "ifpriomap", seq_show: Some(read_priomap), write: Some(write_priomap), ..CFTYPE_ZERO },
    CFTYPE_ZERO,
];

pub static mut net_prio_cgrp_subsys: cgroup_subsys = cgroup_subsys {
    css_alloc: Some(cgrp_css_alloc), css_online: Some(cgrp_css_online), css_free: Some(cgrp_css_free),
    attach: Some(net_prio_attach), legacy_cftypes: SS_FILES.as_ptr(), ..CGROUP_SUBSYS_ZERO
};

unsafe fn netprio_device_event(_unused: *mut notifier_block, event: usize, ptr: *mut core::ffi::c_void) -> i32 {
    let dev = netdev_notifier_info_to_dev(ptr);
    if event == NETDEV_UNREGISTER {
        let old = rtnl_dereference((*dev).priomap);
        RCU_INIT_POINTER((*dev).priomap, core::ptr::null_mut());
        if !old.is_null() { kfree_rcu(old, rcu); }
    }
    NOTIFY_DONE
}

static mut netprio_device_notifier: notifier_block = notifier_block { notifier_call: Some(netprio_device_event), ..NOTIFIER_BLOCK_ZERO };

unsafe fn init_cgroup_netprio() -> i32 {
    register_netdevice_notifier(&mut netprio_device_notifier);
    0
}

// subsys_initcall(init_cgroup_netprio);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
