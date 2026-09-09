// SPDX-License-Identifier: GPL-2.0-only
/*
 * Common code for control of lockd and nfsv4 grace periods.
 *
 * Transplanted from lockd code
 */

// C kernel includes and build-time attributes are supplied by other Rust
// translation units/dependencies.

static mut grace_net_id: ::core::primitive::u32 = 0;
static mut grace_lock: Spinlock = DEFINE_SPINLOCK!();

/**
 * locks_start_grace
 * @net: net namespace that this lock manager belongs to
 * @lm: who this grace period is for
 *
 * A grace period is a period during which locks should not be given
 * out.  Currently grace periods are only enforced by the two lock
 * managers (lockd and nfsd), using the locks_in_grace() function to
 * check when they are in a grace period.
 *
 * This function is called to start a grace period.
 */
pub unsafe fn locks_start_grace(net: *mut net, lm: *mut lock_manager) {
    let grace_list: *mut list_head = net_generic(net, grace_net_id);

    spin_lock(&raw mut grace_lock);
    if list_empty(unsafe { &(*lm).list }) {
        list_add(unsafe { &raw mut (*lm).list }, grace_list);
    } else {
        WARN!(
            1,
            "double list_add attempt detected in net %x %s\n",
            (*net).ns.inum,
            if net == &raw mut init_net {
                "(init_net)"
            } else {
                ""
            }
        );
    }
    spin_unlock(&raw mut grace_lock);
}

// EXPORT_SYMBOL_GPL(locks_start_grace);

/**
 * locks_end_grace
 * @lm: who this grace period is for
 *
 * Call this function to state that the given lock manager is ready to
 * resume regular locking.  The grace period will not end until all lock
 * managers that called locks_start_grace() also call locks_end_grace().
 * Note that callers count on it being safe to call this more than once,
 * and the second call should be a no-op.
 */
pub unsafe fn locks_end_grace(lm: *mut lock_manager) {
    spin_lock(&raw mut grace_lock);
    list_del_init(&raw mut (*lm).list);
    spin_unlock(&raw mut grace_lock);
}

// EXPORT_SYMBOL_GPL(locks_end_grace);

unsafe fn __state_in_grace(net: *mut net, open: bool) -> bool {
    let grace_list: *mut list_head = net_generic(net, grace_net_id);
    let mut lm: *mut lock_manager;

    if !open {
        return !list_empty(&*grace_list);
    }

    spin_lock(&raw mut grace_lock);
    list_for_each_entry!(lm, grace_list, list, {
        if (*lm).block_opens {
            spin_unlock(&raw mut grace_lock);
            return true;
        }
    });
    spin_unlock(&raw mut grace_lock);
    false
}

/**
 * locks_in_grace
 * @net: network namespace
 *
 * Lock managers call this function to determine when it is OK for them
 * to answer ordinary lock requests, and when they should accept only
 * lock reclaims.
 */
pub unsafe fn locks_in_grace(net: *mut net) -> bool {
    __state_in_grace(net, false)
}

// EXPORT_SYMBOL_GPL(locks_in_grace);

pub unsafe fn opens_in_grace(net: *mut net) -> bool {
    __state_in_grace(net, true)
}

// EXPORT_SYMBOL_GPL(opens_in_grace);

unsafe fn grace_init_net(net: *mut net) -> i32 {
    let grace_list: *mut list_head = net_generic(net, grace_net_id);

    INIT_LIST_HEAD(grace_list);
    0
}

unsafe fn grace_exit_net(net: *mut net) {
    let grace_list: *mut list_head = net_generic(net, grace_net_id);

    WARN_ONCE!(
        !list_empty(&*grace_list),
        "net %x %s: grace_list is not empty\n",
        (*net).ns.inum,
        __func__
    );
}

static mut grace_net_ops: pernet_operations = pernet_operations {
    init: Some(grace_init_net),
    exit: Some(grace_exit_net),
    id: &raw mut grace_net_id,
    size: core::mem::size_of::<list_head>(),
};

unsafe fn init_grace() -> i32 {
    register_pernet_subsys(&raw mut grace_net_ops)
}

unsafe fn exit_grace() {
    unregister_pernet_subsys(&raw mut grace_net_ops);
}

// MODULE_AUTHOR("Jeff Layton <jlayton@primarydata.com>");
// MODULE_DESCRIPTION("NFS client and server infrastructure");
// MODULE_LICENSE("GPL");
// module_init(init_grace)
// module_exit(exit_grace)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
