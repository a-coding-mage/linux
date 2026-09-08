// SPDX-License-Identifier: GPL-2.0-only
/*
 * Landlock LSM - Ruleset management
 *
 * Copyright © 2016-2020 Mickaël Salaün <mic@digikod.net>
 * Copyright © 2018-2020 ANSSI
 * Copyright © 2026 Cloudflare, Inc.
 */

// Dependencies from Linux and local Landlock headers:
// <linux/bits.h>, <linux/bug.h>, <linux/cleanup.h>,
// <linux/compiler_types.h>, <linux/err.h>, <linux/errno.h>,
// <linux/kernel.h>, <linux/lockdep.h>, <linux/mutex.h>,
// <linux/overflow.h>, <linux/rbtree.h>, <linux/refcount.h>,
// <linux/slab.h>, <linux/spinlock.h>, <uapi/linux/landlock.h>,
// "access.h", "id.h", "limits.h", "object.h", "ruleset.h",
// and <trace/events/landlock.h>.

pub unsafe fn landlock_create_ruleset(
    fs_access_mask: access_mask_t,
    net_access_mask: access_mask_t,
    scope_mask: access_mask_t,
) -> *mut landlock_ruleset {
    let new_ruleset: *mut landlock_ruleset;

    /* Informs about useless ruleset. */
    if fs_access_mask == 0 && net_access_mask == 0 && scope_mask == 0 {
        return ERR_PTR(-ENOMSG) as *mut landlock_ruleset;
    }

    new_ruleset = kzalloc_obj::<landlock_ruleset>(GFP_KERNEL_ACCOUNT);
    if new_ruleset.is_null() {
        return ERR_PTR(-ENOMEM) as *mut landlock_ruleset;
    }

    refcount_set(&mut (*new_ruleset).usage, 1);
    mutex_init(&mut (*new_ruleset).lock);
    (*new_ruleset).rules.root_inode = RB_ROOT;

    #[cfg(CONFIG_INET)]
    {
        (*new_ruleset).rules.root_net_port = RB_ROOT;
    } /* IS_ENABLED(CONFIG_INET) */

    #[cfg(CONFIG_TRACEPOINTS)]
    {
        (*new_ruleset).id = landlock_get_id_range(1);
    } /* CONFIG_TRACEPOINTS */

    /* Should already be checked in landlock_create_ruleset(). */
    if fs_access_mask != 0 {
        let mask: access_mask_t = fs_access_mask & LANDLOCK_MASK_ACCESS_FS;

        WARN_ON_ONCE(fs_access_mask != mask);
        (*new_ruleset).handled_masks.fs |= mask;
    }
    if net_access_mask != 0 {
        let mask: access_mask_t = net_access_mask & LANDLOCK_MASK_ACCESS_NET;

        WARN_ON_ONCE(net_access_mask != mask);
        (*new_ruleset).handled_masks.net |= mask;
    }
    if scope_mask != 0 {
        let mask: access_mask_t = scope_mask & LANDLOCK_MASK_SCOPE;

        WARN_ON_ONCE(scope_mask != mask);
        (*new_ruleset).handled_masks.scope |= mask;
    }
    new_ruleset
}

unsafe fn build_check_rule() {
    let rule = landlock_rule {
        num_layers: !0,
        ..core::mem::zeroed()
    };

    /*
     * Checks that .num_layers is large enough for at least
     * LANDLOCK_MAX_NUM_LAYERS layers.
     */
    BUILD_BUG_ON(rule.num_layers < LANDLOCK_MAX_NUM_LAYERS);
}

unsafe fn is_object_pointer(key_type: landlock_key_type) -> bool {
    match key_type {
        LANDLOCK_KEY_INODE => true,

        #[cfg(CONFIG_INET)]
        LANDLOCK_KEY_NET_PORT => false,

        _ => {
            WARN_ON_ONCE(1 != 0);
            false
        }
    }
}

unsafe fn create_rule(
    id: landlock_id,
    layers: *const [landlock_layer],
    num_layers: u32,
    new_layer: *const landlock_layer,
) -> *mut landlock_rule {
    let new_rule: *mut landlock_rule;
    let new_num_layers: u32;

    build_check_rule();
    if !new_layer.is_null() {
        /* Should already be checked by landlock_merge_ruleset(). */
        if WARN_ON_ONCE(num_layers >= LANDLOCK_MAX_NUM_LAYERS) {
            return ERR_PTR(-E2BIG) as *mut landlock_rule;
        }
        new_num_layers = num_layers + 1;
    } else {
        new_num_layers = num_layers;
    }
    new_rule = kzalloc_flex::<landlock_rule, landlock_layer>(
        GFP_KERNEL_ACCOUNT,
        new_num_layers as usize,
    );
    if new_rule.is_null() {
        return ERR_PTR(-ENOMEM) as *mut landlock_rule;
    }
    RB_CLEAR_NODE(&mut (*new_rule).node);
    if is_object_pointer(id.type_) {
        /* This should have been caught by landlock_store_rule(). */
        WARN_ON_ONCE((*id.key).object.is_null());
        landlock_get_object((*id.key).object);
    }

    (*new_rule).key = id.key;
    (*new_rule).num_layers = new_num_layers;
    /* Copies the original layer stack. */
    memcpy(
        (*new_rule).layers.as_mut_ptr() as *mut core::ffi::c_void,
        layers as *const core::ffi::c_void,
        flex_array_size(new_rule, (*new_rule).layers.as_ptr(), num_layers as usize),
    );
    if !new_layer.is_null() {
        /* Adds a copy of @new_layer on the layer stack. */
        *(*new_rule)
            .layers
            .as_mut_ptr()
            .add(((*new_rule).num_layers - 1) as usize) = *new_layer;
    }
    new_rule
}

unsafe fn free_rule(rule: *mut landlock_rule, key_type: landlock_key_type) {
    might_sleep();
    if rule.is_null() {
        return;
    }
    if is_object_pointer(key_type) {
        landlock_put_object((*rule).key.object);
    }
    kfree(rule as *mut core::ffi::c_void);
}

unsafe fn build_check_ruleset() {
    let rules = landlock_rules {
        num_rules: !0,
        ..core::mem::zeroed()
    };

    BUILD_BUG_ON(rules.num_rules < LANDLOCK_MAX_NUM_RULES);
}

/**
 * landlock_store_rule - Create and insert a rule into the rule storage
 *
 * @rules: The rule storage to be updated.  The caller is responsible for
 *         any required locking.  For rulesets, this means holding
 *         &landlock_ruleset.lock.  For domains under construction, no lock is
 *         needed because the domain is not yet visible to other tasks.
 * @id: The ID to build the new rule with.  The underlying kernel object, if
 *      any, must be held by the caller.
 * @layers: One or multiple layers to be copied into the new rule.
 * @num_layers: The number of @layers entries.
 *
 * When user space requests to add a new rule to a ruleset, @layers only
 * contains one entry and this entry is not assigned to any level.  In this
 * case, the new rule will extend @rules, similarly to a boolean OR between
 * access rights.
 *
 * When merging a ruleset in a domain, or copying a domain, @layers will be
 * added to @rules as new constraints, similarly to a boolean AND between access
 * rights.
 *
 * Return: 0 on success, -errno on failure.
 */
pub unsafe fn landlock_store_rule(
    rules: *mut landlock_rules,
    id: landlock_id,
    layers: *const [landlock_layer],
    num_layers: usize,
) -> core::ffi::c_int {
    let mut walker_node: *mut *mut rb_node;
    let mut parent_node: *mut rb_node = core::ptr::null_mut();
    let mut new_rule: *mut landlock_rule;
    let root: *mut rb_root;

    might_sleep();
    if WARN_ON_ONCE(layers.is_null()) {
        return -ENOENT;
    }

    if is_object_pointer(id.type_) && WARN_ON_ONCE((*id.key).object.is_null()) {
        return -ENOENT;
    }

    root = landlock_get_rule_root(rules, id.type_);
    if IS_ERR(root) {
        return PTR_ERR(root) as core::ffi::c_int;
    }

    walker_node = &mut (*root).rb_node;
    while !(*walker_node).is_null() {
        let this: *mut landlock_rule = rb_entry(*walker_node, landlock_rule, node);

        if (*this).key.data != (*id.key).data {
            parent_node = *walker_node;
            if (*this).key.data < (*id.key).data {
                walker_node = &mut (**walker_node).rb_right;
            } else {
                walker_node = &mut (**walker_node).rb_left;
            }
            continue;
        }

        /* Only a single-level layer should match an existing rule. */
        if WARN_ON_ONCE(num_layers != 1) {
            return -EINVAL;
        }

        /* If there is a matching rule, updates it. */
        if (*(*layers).as_ptr().add(0)).level == 0 {
            /*
             * Extends access rights when the request comes from
             * landlock_add_rule(2), i.e. @rules is not a domain.
             */
            if WARN_ON_ONCE((*this).num_layers != 1) {
                return -EINVAL;
            }
            if WARN_ON_ONCE((*(*this).layers.as_ptr().add(0)).level != 0) {
                return -EINVAL;
            }
            (*(*this).layers.as_mut_ptr().add(0)).access |= (*(*layers).as_ptr().add(0)).access;
            (*(*this).layers.as_mut_ptr().add(0)).flags.quiet |=
                (*(*layers).as_ptr().add(0)).flags.quiet;
            return 0;
        }

        if WARN_ON_ONCE((*(*this).layers.as_ptr().add(0)).level == 0) {
            return -EINVAL;
        }

        /*
         * Intersects access rights when it is a merge between a
         * ruleset and a domain.
         */
        new_rule = create_rule(
            id,
            (*this).layers.as_ptr() as *const [landlock_layer],
            (*this).num_layers,
            (*layers).as_ptr().add(0),
        );
        if IS_ERR(new_rule) {
            return PTR_ERR(new_rule) as core::ffi::c_int;
        }
        rb_replace_node(&mut (*this).node, &mut (*new_rule).node, root);
        free_rule(this, id.type_);
        return 0;
    }

    /* There is no match for @id. */
    build_check_ruleset();
    if (*rules).num_rules >= LANDLOCK_MAX_NUM_RULES {
        return -E2BIG;
    }
    new_rule = create_rule(id, layers, num_layers as u32, core::ptr::null());
    if IS_ERR(new_rule) {
        return PTR_ERR(new_rule) as core::ffi::c_int;
    }
    rb_link_node(&mut (*new_rule).node, parent_node, walker_node);
    rb_insert_color(&mut (*new_rule).node, root);
    (*rules).num_rules += 1;
    0
}

unsafe fn build_check_layer() {
    let layer = landlock_layer {
        level: !0,
        access: !0,
        ..core::mem::zeroed()
    };

    /*
     * Checks that .level and .access are large enough to contain their expected
     * maximum values.
     */
    BUILD_BUG_ON(layer.level < LANDLOCK_MAX_NUM_LAYERS);
    BUILD_BUG_ON(layer.access < LANDLOCK_MASK_ACCESS_FS);
}

/* @ruleset must be locked by the caller. */
pub unsafe fn landlock_insert_rule(
    ruleset: *mut landlock_ruleset,
    id: landlock_id,
    access: access_mask_t,
    flags: u32,
) -> core::ffi::c_int {
    let mut layers = [landlock_layer {
        access,
        /*
         * When @level is zero, landlock_store_rule() extends @ruleset.
         */
        level: 0,
        flags: landlock_layer_flags {
            quiet: ((flags & LANDLOCK_ADD_RULE_QUIET) != 0) as _,
        },
    }];
    let err: core::ffi::c_int;

    build_check_layer();
    lockdep_assert_held(&mut (*ruleset).lock);
    err = landlock_store_rule(
        &mut (*ruleset).rules,
        id,
        &mut layers as *mut [landlock_layer],
        ARRAY_SIZE(&layers),
    );

    #[cfg(CONFIG_TRACEPOINTS)]
    {
        if err == 0 {
            (*ruleset).version += 1;
        }
    } /* CONFIG_TRACEPOINTS */

    err
}

pub unsafe fn landlock_free_rules(rules: *mut landlock_rules) {
    let mut freeme: *mut landlock_rule = core::ptr::null_mut();
    let mut next: *mut landlock_rule = core::ptr::null_mut();

    might_sleep();
    rbtree_postorder_for_each_entry_safe(
        &mut freeme,
        &mut next,
        &mut (*rules).root_inode,
        node,
        |freeme| free_rule(freeme, LANDLOCK_KEY_INODE),
    );

    #[cfg(CONFIG_INET)]
    {
        rbtree_postorder_for_each_entry_safe(
            &mut freeme,
            &mut next,
            &mut (*rules).root_net_port,
            node,
            |freeme| free_rule(freeme, LANDLOCK_KEY_NET_PORT),
        );
    } /* IS_ENABLED(CONFIG_INET) */
}

unsafe fn free_ruleset(ruleset: *mut landlock_ruleset) {
    might_sleep();
    trace_landlock_free_ruleset(ruleset);
    landlock_free_rules(&mut (*ruleset).rules);
    kfree(ruleset as *mut core::ffi::c_void);
}

pub unsafe fn landlock_put_ruleset(ruleset: *mut landlock_ruleset) {
    might_sleep();
    if !ruleset.is_null() && refcount_dec_and_test(&mut (*ruleset).usage) {
        free_ruleset(ruleset);
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
