// SPDX-License-Identifier: GPL-2.0-only
/*
 * Landlock - Domain management
 *
 * Copyright © 2016-2020 Mickaël Salaün <mic@digikod.net>
 * Copyright © 2018-2020 ANSSI
 * Copyright © 2024-2025 Microsoft Corporation
 * Copyright © 2026 Cloudflare, Inc.
 */

// Dependencies from the original C includes:
// <kunit/test.h>, <linux/bitops.h>, <linux/bits.h>, <linux/cleanup.h>,
// <linux/cred.h>, <linux/err.h>, <linux/file.h>, <linux/lockdep.h>,
// <linux/mm.h>, <linux/mutex.h>, <linux/overflow.h>, <linux/path.h>,
// <linux/pid.h>, <linux/rbtree.h>, <linux/refcount.h>, <linux/sched.h>,
// <linux/signal.h>, <linux/slab.h>, <linux/uidgid.h>, <linux/workqueue.h>,
// "access.h", "common.h", "domain.h", "id.h", "limits.h", "ruleset.h".

use core::ffi::{c_char, c_void};
use core::mem::size_of;
use core::ptr;

type size_t = usize;
type ssize_t = isize;

unsafe fn build_check_domain() {
    let mut domain: landlock_domain = core::mem::zeroed();

    domain.num_layers = !0;

    BUILD_BUG_ON(domain.num_layers < LANDLOCK_MAX_NUM_LAYERS);
}

unsafe fn create_domain(num_layers: u32) -> *mut landlock_domain {
    let new_domain: *mut landlock_domain;

    build_check_domain();
    new_domain = kzalloc_flex_landlock_domain_handled_masks(num_layers, GFP_KERNEL_ACCOUNT);
    if new_domain.is_null() {
        return ERR_PTR(-ENOMEM) as *mut landlock_domain;
    }

    refcount_set(&mut (*new_domain).usage, 1);
    (*new_domain).rules.root_inode = RB_ROOT;

    // #if IS_ENABLED(CONFIG_INET)
    if IS_ENABLED_CONFIG_INET {
        (*new_domain).rules.root_net_port = RB_ROOT;
    }
    // #endif /* IS_ENABLED(CONFIG_INET) */

    (*new_domain).num_layers = num_layers;
    new_domain
}

unsafe fn free_domain(domain: *mut landlock_domain) {
    might_sleep();
    landlock_free_rules(&mut (*domain).rules);
    landlock_put_hierarchy((*domain).hierarchy);
    kfree(domain as *const c_void);
}

pub unsafe fn landlock_put_domain(domain: *mut landlock_domain) {
    might_sleep();
    if !domain.is_null() && refcount_dec_and_test(&mut (*domain).usage) {
        free_domain(domain);
    }
}

unsafe fn free_domain_work(work: *mut work_struct) {
    let domain: *mut landlock_domain;

    domain = container_of_landlock_domain_work_free(work);
    free_domain(domain);
}

pub unsafe fn landlock_put_domain_deferred(domain: *mut landlock_domain) {
    if !domain.is_null() && refcount_dec_and_test(&mut (*domain).usage) {
        INIT_WORK(&mut (*domain).work_free, Some(free_domain_work));
        schedule_work(&mut (*domain).work_free);
    }
}

/* The returned access has the same lifetime as the domain. */
unsafe fn find_rule(
    domain: *const landlock_domain,
    id: landlock_id,
) -> *const landlock_rule {
    let root: *const rb_root;
    let mut node: *const rb_node;

    root = landlock_get_rule_root(&(*domain).rules as *const _ as *mut landlock_rules, id.type_);
    if IS_ERR(root as *const c_void) {
        return ptr::null();
    }
    node = (*root).rb_node;

    while !node.is_null() {
        let this: *mut landlock_rule = rb_entry_landlock_rule_node(node as *mut rb_node);

        if (*this).key.data == id.key.data {
            return this;
        }
        if (*this).key.data < id.key.data {
            node = (*node).rb_right;
        } else {
            node = (*node).rb_left;
        }
    }
    ptr::null()
}

/**
 * landlock_unmask_layers - Remove the access rights in @masks which are
 *                          granted by a matching rule
 *
 * Looks up the rule matching @id in @domain, then updates the set of
 * (per-layer) unfulfilled access rights @masks so that all the access rights
 * granted by that rule are removed (because they are now fulfilled).
 *
 * @domain: The Landlock domain to search for a matching rule.
 * @id: Identifier for the rule target (e.g. inode, port).
 * @masks: A matrix of unfulfilled access rights for each layer.
 * @matched_rule: Optional output for the matched rule (for tracing); set to
 *                the matching rule when non-NULL, unchanged otherwise.
 *
 * Return: True if the request is allowed (i.e. the access rights granted all
 * remaining unfulfilled access rights and masks has no leftover set bits).
 */
pub unsafe fn landlock_unmask_layers(
    domain: *const landlock_domain,
    id: landlock_id,
    masks: *mut layer_masks,
    matched_rule: *mut *const landlock_rule,
) -> bool {
    let rule: *const landlock_rule;

    if masks.is_null() {
        return true;
    }

    rule = find_rule(domain, id);
    if rule.is_null() {
        return false;
    }

    if !matched_rule.is_null() {
        *matched_rule = rule;
    }

    /*
     * An access is granted if, for each policy layer, at least one rule
     * encountered on the pathwalk grants the requested access, regardless
     * of its position in the layer stack.  We must then check the remaining
     * layers for each inode, from the first added layer to the last one.
     * When there are multiple requested accesses, for each policy layer,
     * the full set of requested accesses may not be granted by only one
     * rule, but by the union (binary OR) of multiple rules.  For example,
     * /a/b <execute> + /a <read> grants /a/b <execute + read>.
     *
     * This function is called once per matching rule during the pathwalk,
     * progressively clearing bits in @masks.  The overall access decision
     * is per-layer: access is granted iff masks->layers[l].access == 0 for
     * all layers l.  When two independent mechanisms can each grant access
     * within a layer (e.g. a path rule OR a scope exception), the
     * composition must evaluate per-layer: FOR-ALL l (A(l) OR B(l)), not
     * (FOR-ALL l A(l)) OR (FOR-ALL l B(l)), to prevent bypass when
     * different layers grant via different mechanisms.
     */
    for i in 0..(*rule).num_layers as usize {
        let layer: *const landlock_layer = &*(*rule).layers.as_ptr().add(i);

        /* Clear the bits where the layer in the rule grants access. */
        (*masks).layers[((*layer).level - 1) as usize].access &= !(*layer).access;

        // #ifdef CONFIG_SECURITY_LANDLOCK_LOG
        if CONFIG_SECURITY_LANDLOCK_LOG {
            /* Collect rule flags for each layer. */
            if (*layer).flags.quiet {
                (*masks).layers[((*layer).level - 1) as usize].quiet = true;
            }
        }
        // #endif /* CONFIG_SECURITY_LANDLOCK_LOG */
    }

    for i in 0..ARRAY_SIZE_layer_masks_layers() {
        if (*masks).layers[i].access != 0 {
            return false;
        }
    }
    true
}

type get_access_mask_t = unsafe fn(domain: *const landlock_domain, layer_level: u16) -> access_mask_t;

/**
 * landlock_init_layer_masks - Initialize layer masks from an access request
 *
 * Populates @masks such that for each access right in @access_request, the bits
 * for all the layers are set where this access right is handled.  Rule flags
 * are also zeroed.
 *
 * @domain: The domain that defines the current restrictions.
 * @access_request: The requested access rights to check.
 * @masks: Layer access masks to populate.
 * @key_type: The key type to switch between access masks of different types.
 *
 * Return: An access mask where each access right bit is set which is handled in
 * any of the active layers in @domain.
 */
pub unsafe fn landlock_init_layer_masks(
    domain: *const landlock_domain,
    access_request: access_mask_t,
    masks: *mut layer_masks,
    key_type: landlock_key_type,
) -> access_mask_t {
    let mut handled_accesses: access_mask_t = 0;
    let get_access_mask: get_access_mask_t;

    match key_type {
        LANDLOCK_KEY_INODE => {
            get_access_mask = landlock_get_fs_access_mask;
        }

        // #if IS_ENABLED(CONFIG_INET)
        LANDLOCK_KEY_NET_PORT if IS_ENABLED_CONFIG_INET => {
            get_access_mask = landlock_get_net_access_mask;
        }
        // #endif /* IS_ENABLED(CONFIG_INET) */

        _ => {
            WARN_ON_ONCE(1 != 0);
            return 0;
        }
    }

    /* An empty access request can happen because of O_WRONLY | O_RDWR. */
    if access_request == 0 {
        return 0;
    }

    for i in 0..(*domain).num_layers as usize {
        let handled: access_mask_t = get_access_mask(domain, i as u16);

        (*masks).layers[i].access = access_request & handled;
        handled_accesses |= (*masks).layers[i].access;
        // #ifdef CONFIG_SECURITY_LANDLOCK_LOG
        if CONFIG_SECURITY_LANDLOCK_LOG {
            (*masks).layers[i].quiet = false;
        }
        // #endif /* CONFIG_SECURITY_LANDLOCK_LOG */
    }
    for i in (*domain).num_layers as usize..ARRAY_SIZE_layer_masks_layers() {
        (*masks).layers[i].access = 0;
        // #ifdef CONFIG_SECURITY_LANDLOCK_LOG
        if CONFIG_SECURITY_LANDLOCK_LOG {
            (*masks).layers[i].quiet = false;
        }
        // #endif /* CONFIG_SECURITY_LANDLOCK_LOG */
    }

    handled_accesses
}

unsafe fn merge_tree(
    dst: *mut landlock_domain,
    src: *mut landlock_ruleset,
    key_type: landlock_key_type,
) -> i32 {
    let mut walker_rule: *mut landlock_rule;
    let mut next_rule: *mut landlock_rule;
    let src_root: *mut rb_root;
    let mut err: i32 = 0;

    might_sleep();
    lockdep_assert_held(&mut (*src).lock);

    src_root = landlock_get_rule_root(&mut (*src).rules, key_type);
    if IS_ERR(src_root as *const c_void) {
        return PTR_ERR(src_root as *const c_void) as i32;
    }

    /* Merges the @src tree. */
    walker_rule = rbtree_postorder_first_landlock_rule(src_root);
    while !walker_rule.is_null() {
        next_rule = rbtree_postorder_next_landlock_rule(walker_rule);
        let mut layers: [landlock_layer; 1] = [core::mem::zeroed()];
        let id = landlock_id {
            key: (*walker_rule).key,
            type_: key_type,
        };

        layers[0].level = (*dst).num_layers as u16;

        if WARN_ON_ONCE((*walker_rule).num_layers != 1) {
            return -EINVAL;
        }

        if WARN_ON_ONCE((*walker_rule).layers[0].level != 0) {
            return -EINVAL;
        }

        layers[0].access = (*walker_rule).layers[0].access;
        layers[0].flags = (*walker_rule).layers[0].flags;

        err = landlock_store_rule(&mut (*dst).rules, id, layers.as_mut_ptr(), layers.len());
        if err != 0 {
            return err;
        }
        walker_rule = next_rule;
    }
    err
}

unsafe fn merge_ruleset(dst: *mut landlock_domain, src: *mut landlock_ruleset) -> i32 {
    let mut err: i32 = 0;

    might_sleep();
    /* Should already be checked by landlock_merge_ruleset() */
    if WARN_ON_ONCE(src.is_null()) {
        return 0;
    }
    /* Only merge into a domain. */
    if WARN_ON_ONCE(dst.is_null() || (*dst).hierarchy.is_null()) {
        return -EINVAL;
    }

    lockdep_assert_held(&mut (*src).lock);

    /* Stacks the new layer. */
    if WARN_ON_ONCE((*dst).num_layers < 1) {
        return -EINVAL;
    }

    (*dst).handled_masks[((*dst).num_layers - 1) as usize] =
        landlock_upgrade_handled_access_masks((*src).handled_masks);

    /* Merges the @src inode tree. */
    err = merge_tree(dst, src, LANDLOCK_KEY_INODE);
    if err != 0 {
        return err;
    }

    // #if IS_ENABLED(CONFIG_INET)
    if IS_ENABLED_CONFIG_INET {
        /* Merges the @src network port tree. */
        err = merge_tree(dst, src, LANDLOCK_KEY_NET_PORT);
        if err != 0 {
            return err;
        }
    }
    // #endif /* IS_ENABLED(CONFIG_INET) */

    0
}

unsafe fn inherit_tree(
    parent: *mut landlock_domain,
    child: *mut landlock_domain,
    key_type: landlock_key_type,
) -> i32 {
    let mut walker_rule: *mut landlock_rule;
    let mut next_rule: *mut landlock_rule;
    let parent_root: *mut rb_root;
    let mut err: i32 = 0;

    might_sleep();

    parent_root = landlock_get_rule_root(&mut (*parent).rules, key_type);
    if IS_ERR(parent_root as *const c_void) {
        return PTR_ERR(parent_root as *const c_void) as i32;
    }

    /* Copies the @parent inode or network tree. */
    walker_rule = rbtree_postorder_first_landlock_rule(parent_root);
    while !walker_rule.is_null() {
        next_rule = rbtree_postorder_next_landlock_rule(walker_rule);
        let id = landlock_id {
            key: (*walker_rule).key,
            type_: key_type,
        };

        err = landlock_store_rule(
            &mut (*child).rules,
            id,
            (*walker_rule).layers.as_ptr() as *mut landlock_layer,
            (*walker_rule).num_layers as usize,
        );
        if err != 0 {
            return err;
        }
        walker_rule = next_rule;
    }
    err
}

unsafe fn inherit_ruleset(parent: *mut landlock_domain, child: *mut landlock_domain) -> i32 {
    let mut err: i32 = 0;

    might_sleep();
    if parent.is_null() {
        return 0;
    }

    /* Copies the @parent inode tree. */
    err = inherit_tree(parent, child, LANDLOCK_KEY_INODE);
    if err != 0 {
        return err;
    }

    // #if IS_ENABLED(CONFIG_INET)
    if IS_ENABLED_CONFIG_INET {
        /* Copies the @parent network port tree. */
        err = inherit_tree(parent, child, LANDLOCK_KEY_NET_PORT);
        if err != 0 {
            return err;
        }
    }
    // #endif /* IS_ENABLED(CONFIG_INET) */

    if WARN_ON_ONCE((*child).num_layers <= (*parent).num_layers) {
        return -EINVAL;
    }

    /*
     * Copies the parent layer stack and leaves a space for the new layer.
     */
    memcpy(
        (*child).handled_masks.as_mut_ptr() as *mut c_void,
        (*parent).handled_masks.as_ptr() as *const c_void,
        flex_array_size_landlock_domain_handled_masks(parent, (*parent).num_layers),
    );

    if WARN_ON_ONCE((*parent).hierarchy.is_null()) {
        return -EINVAL;
    }

    landlock_get_hierarchy((*parent).hierarchy);
    (*(*child).hierarchy).parent = (*parent).hierarchy;

    0
}

/**
 * landlock_merge_ruleset - Merge a ruleset with a domain
 *
 * @parent: Parent domain.
 * @ruleset: New ruleset to be merged.
 *
 * The current task is requesting to be restricted.  The subjective credentials
 * must not be in an overridden state. cf. landlock_init_hierarchy_log().
 *
 * The caller must hold @ruleset->lock.
 *
 * Return: A new domain merging @parent and @ruleset on success, or ERR_PTR() on
 * failure.  If @parent is NULL, the new domain duplicates @ruleset.
 */
pub unsafe fn landlock_merge_ruleset(
    parent: *mut landlock_domain,
    ruleset: *mut landlock_ruleset,
) -> *mut landlock_domain {
    let mut new_dom: *mut landlock_domain = ptr::null_mut();
    let num_layers: u32;
    let mut err: i32;

    might_sleep();
    lockdep_assert_held(&mut (*ruleset).lock);
    if WARN_ON_ONCE(ruleset.is_null()) {
        return ERR_PTR(-EINVAL) as *mut landlock_domain;
    }

    if !parent.is_null() {
        if (*parent).num_layers >= LANDLOCK_MAX_NUM_LAYERS {
            return ERR_PTR(-E2BIG) as *mut landlock_domain;
        }
        num_layers = (*parent).num_layers + 1;
    } else {
        num_layers = 1;
    }

    /* Creates a new domain... */
    new_dom = create_domain(num_layers);
    if IS_ERR(new_dom as *const c_void) {
        return new_dom;
    }

    (*new_dom).hierarchy = kzalloc_obj_landlock_hierarchy(GFP_KERNEL_ACCOUNT);
    if (*new_dom).hierarchy.is_null() {
        return ERR_PTR(-ENOMEM) as *mut landlock_domain;
    }

    refcount_set(&mut (*(*new_dom).hierarchy).usage, 1);

    /* ...as a child of @parent... */
    err = inherit_ruleset(parent, new_dom);
    if err != 0 {
        landlock_put_domain(new_dom);
        return ERR_PTR(err) as *mut landlock_domain;
    }

    /* ...and including @ruleset. */
    err = merge_ruleset(new_dom, ruleset);
    if err != 0 {
        landlock_put_domain(new_dom);
        return ERR_PTR(err) as *mut landlock_domain;
    }

    err = landlock_init_hierarchy_log((*new_dom).hierarchy);
    if err != 0 {
        landlock_put_domain(new_dom);
        return ERR_PTR(err) as *mut landlock_domain;
    }

    // #ifdef CONFIG_SECURITY_LANDLOCK_LOG
    if CONFIG_SECURITY_LANDLOCK_LOG {
        (*(*new_dom).hierarchy).quiet_masks = (*ruleset).quiet_masks;
    }
    // #endif /* CONFIG_SECURITY_LANDLOCK_LOG */

    new_dom
}

// #ifdef CONFIG_SECURITY_LANDLOCK_LOG

/**
 * get_current_exe - Get the current's executable path, if any
 *
 * @exe_str: Returned pointer to a path string with a lifetime tied to the
 *           returned buffer, if any.
 * @exe_size: Returned size of @exe_str (including the trailing null
 *            character), if any.
 *
 * Return: A pointer to an allocated buffer where @exe_str point to, %NULL if
 * there is no executable path, or an error otherwise.
 */
unsafe fn get_current_exe(exe_str: *mut *const c_char, exe_size: *mut size_t) -> *const c_void {
    let buffer_size: size_t = LANDLOCK_PATH_MAX_SIZE;
    let mm: *mut mm_struct = (*current).mm;
    let mut file: *mut file = ptr::null_mut();
    let mut buffer: *mut c_char = ptr::null_mut();
    let exe: *const c_char;
    let size: ssize_t;

    if mm.is_null() {
        return ptr::null();
    }

    file = get_mm_exe_file(mm);
    if file.is_null() {
        return ptr::null();
    }

    buffer = kmalloc(buffer_size, GFP_KERNEL) as *mut c_char;
    if buffer.is_null() {
        fput(file);
        return ERR_PTR(-ENOMEM) as *const c_void;
    }

    exe = d_path(&mut (*file).f_path, buffer, buffer_size);
    if WARN_ON_ONCE(IS_ERR(exe as *const c_void)) {
        /* Should never happen according to LANDLOCK_PATH_MAX_SIZE. */
        fput(file);
        kfree(buffer as *const c_void);
        return ERR_CAST(exe as *const c_void);
    }

    size = buffer.add(buffer_size).offset_from(exe) as ssize_t;
    if WARN_ON_ONCE(size <= 0) {
        fput(file);
        kfree(buffer as *const c_void);
        return ERR_PTR(-ENAMETOOLONG) as *const c_void;
    }

    *exe_size = size as size_t;
    *exe_str = exe;
    fput(file);
    buffer as *const c_void
}

/*
 * Return: A newly allocated object describing a domain, or an error
 * otherwise.
 */
unsafe fn get_current_details() -> *mut landlock_details {
    /* Cf. audit_log_d_path_exe() */
    static NULL_PATH: [c_char; 7] = [
        b'(' as c_char,
        b'n' as c_char,
        b'u' as c_char,
        b'l' as c_char,
        b'l' as c_char,
        b')' as c_char,
        0,
    ];
    let mut path_str: *const c_char = NULL_PATH.as_ptr();
    let mut path_size: size_t = size_of::<[c_char; 7]>();
    let mut buffer: *const c_void = ptr::null();
    let details: *mut landlock_details;

    buffer = get_current_exe(&mut path_str, &mut path_size);
    if IS_ERR(buffer) {
        return ERR_CAST(buffer) as *mut landlock_details;
    }

    /*
     * Create the new details according to the path's length.  Account to
     * the calling task's memcg, like the other Landlock per-domain
     * allocations, even if it may not control the related size.
     */
    details = kzalloc_flex_landlock_details_exe_path(path_size, GFP_KERNEL_ACCOUNT);
    if details.is_null() {
        if !buffer.is_null() {
            kfree(buffer);
        }
        return ERR_PTR(-ENOMEM) as *mut landlock_details;
    }

    memcpy((*details).exe_path.as_mut_ptr() as *mut c_void, path_str as *const c_void, path_size);
    (*details).pid = get_pid(task_tgid(current));
    (*details).uid = from_kuid(&init_user_ns, current_uid());
    get_task_comm((*details).comm.as_mut_ptr(), current);
    if !buffer.is_null() {
        kfree(buffer);
    }
    details
}

/**
 * landlock_init_hierarchy_log - Partially initialize landlock_hierarchy
 *
 * @hierarchy: The hierarchy to initialize.
 *
 * The current task is referenced as the domain that is enforcing the
 * restriction.  The subjective credentials must not be in an overridden state.
 *
 * @hierarchy->parent and @hierarchy->usage should already be set.
 *
 * Return: 0 on success, -errno on failure.
 */
pub unsafe fn landlock_init_hierarchy_log(hierarchy: *mut landlock_hierarchy) -> i32 {
    let details: *mut landlock_details;

    details = get_current_details();
    if IS_ERR(details as *const c_void) {
        return PTR_ERR(details as *const c_void) as i32;
    }

    (*hierarchy).details = details;
    (*hierarchy).id = landlock_get_id_range(1);
    /*
     * The hierarchy is born unobservable: landlock_restrict_self() moves it
     * out of LANDLOCK_LOG_UNCOMMITTED once it has emitted the creation
     * event, so the matching free_domain event fires for it and not for a
     * hierarchy whose creation was never observed.
     */
    (*hierarchy).log_status = LANDLOCK_LOG_UNCOMMITTED;
    (*hierarchy).log_same_exec = true;
    (*hierarchy).log_new_exec = false;
    atomic64_set(&mut (*hierarchy).num_denials, 0);
    0
}

unsafe fn get_layer_deny_mask(
    all_existing_optional_access: access_mask_t,
    access_bit: core::ffi::c_ulong,
    layer: size_t,
) -> deny_masks_t {
    let access_weight: core::ffi::c_ulong;

    /* This may require change with new object types. */
    WARN_ON_ONCE(all_existing_optional_access != _LANDLOCK_ACCESS_FS_OPTIONAL);

    if WARN_ON_ONCE(layer >= LANDLOCK_MAX_NUM_LAYERS as usize) {
        return 0;
    }

    access_weight = hweight_long(all_existing_optional_access & GENMASK(access_bit, 0));
    if WARN_ON_ONCE(access_weight < 1) {
        return 0;
    }

    (layer as deny_masks_t)
        << ((access_weight - 1) * HWEIGHT(LANDLOCK_MAX_NUM_LAYERS - 1)) as usize
}

/**
 * landlock_get_quiet_optional_accesses - Get optional accesses which are
 *                                        covered by quiet rule flags.
 *
 * @all_existing_optional_access: Bitmask of valid optional accesses.
 * @deny_masks: Domain layer levels that denied each optional access (the
 *              deny_masks field on struct landlock_file_security).
 * @masks: The struct layer_masks collected during the path walk.
 *
 * Return: a bitmask of which optional accesses are denied by layers for which
 * the quiet flag was collected during the path walk.
 */
pub unsafe fn landlock_get_quiet_optional_accesses(
    all_existing_optional_access: access_mask_t,
    deny_masks: deny_masks_t,
    masks: *const layer_masks,
) -> optional_access_t {
    let access_opt: core::ffi::c_ulong = all_existing_optional_access;
    let mut access_index: size_t = 0;
    let mut quiet_optional_accesses: optional_access_t = 0;

    /* This will require change with new object types. */
    WARN_ON_ONCE(access_opt != _LANDLOCK_ACCESS_FS_OPTIONAL);

    let mut access_bit = find_first_bit(&access_opt, BITS_PER_TYPE_access_mask_t());
    while access_bit < BITS_PER_TYPE_access_mask_t() {
        let layer: u8 = ((deny_masks
            >> (access_index * HWEIGHT(LANDLOCK_MAX_NUM_LAYERS - 1) as usize))
            & (LANDLOCK_MAX_NUM_LAYERS - 1) as deny_masks_t) as u8;

        if (*masks).layers[layer as usize].quiet {
            quiet_optional_accesses |= BIT(access_index as core::ffi::c_ulong) as optional_access_t;
        }
        access_index += 1;
        access_bit = find_next_bit(&access_opt, BITS_PER_TYPE_access_mask_t(), access_bit + 1);
    }
    quiet_optional_accesses
}

// #ifdef CONFIG_SECURITY_LANDLOCK_KUNIT_TEST

unsafe fn test_get_layer_deny_mask(test: *mut kunit) {
    let truncate: core::ffi::c_ulong = BIT_INDEX(LANDLOCK_ACCESS_FS_TRUNCATE);
    let ioctl_dev: core::ffi::c_ulong = BIT_INDEX(LANDLOCK_ACCESS_FS_IOCTL_DEV);

    KUNIT_EXPECT_EQ(
        test,
        0,
        get_layer_deny_mask(_LANDLOCK_ACCESS_FS_OPTIONAL, truncate, 0),
    );
    KUNIT_EXPECT_EQ(
        test,
        0x3,
        get_layer_deny_mask(_LANDLOCK_ACCESS_FS_OPTIONAL, truncate, 3),
    );

    KUNIT_EXPECT_EQ(
        test,
        0,
        get_layer_deny_mask(_LANDLOCK_ACCESS_FS_OPTIONAL, ioctl_dev, 0),
    );
    KUNIT_EXPECT_EQ(
        test,
        0xf0,
        get_layer_deny_mask(_LANDLOCK_ACCESS_FS_OPTIONAL, ioctl_dev, 15),
    );
}

// #endif /* CONFIG_SECURITY_LANDLOCK_KUNIT_TEST */

pub unsafe fn landlock_get_deny_masks(
    all_existing_optional_access: access_mask_t,
    optional_access: access_mask_t,
    masks: *const layer_masks,
) -> deny_masks_t {
    let access_opt: core::ffi::c_ulong = optional_access;
    let mut deny_masks: deny_masks_t = 0;
    let mut all_denied: access_mask_t = 0;

    /* This may require change with new object types. */
    WARN_ON_ONCE(!access_mask_subset(optional_access, all_existing_optional_access));

    if WARN_ON_ONCE(masks.is_null()) {
        return 0;
    }

    if WARN_ON_ONCE(access_opt == 0) {
        return 0;
    }

    let mut i: ssize_t = ARRAY_SIZE_layer_masks_layers() as ssize_t - 1;
    while i >= 0 {
        let denied: access_mask_t = (*masks).layers[i as usize].access & optional_access;
        let newly_denied: core::ffi::c_ulong = denied & !all_denied;

        if newly_denied != 0 {
            let mut access_bit =
                find_first_bit(&newly_denied, 8 * size_of::<access_mask_t>());
            while access_bit < 8 * size_of::<access_mask_t>() {
                deny_masks |= get_layer_deny_mask(
                    all_existing_optional_access,
                    access_bit as core::ffi::c_ulong,
                    i as size_t,
                );
                access_bit =
                    find_next_bit(&newly_denied, 8 * size_of::<access_mask_t>(), access_bit + 1);
            }
        }
        all_denied |= denied;
        i -= 1;
    }
    deny_masks
}

// #ifdef CONFIG_SECURITY_LANDLOCK_KUNIT_TEST

unsafe fn test_landlock_get_deny_masks(test: *mut kunit) {
    let mut layers1: layer_masks = core::mem::zeroed();

    layers1.layers[0].access = LANDLOCK_ACCESS_FS_EXECUTE | LANDLOCK_ACCESS_FS_IOCTL_DEV;
    layers1.layers[1].access = LANDLOCK_ACCESS_FS_TRUNCATE;
    layers1.layers[2].access = LANDLOCK_ACCESS_FS_IOCTL_DEV;
    layers1.layers[9].access = LANDLOCK_ACCESS_FS_EXECUTE;

    KUNIT_EXPECT_EQ(
        test,
        0x1,
        landlock_get_deny_masks(
            _LANDLOCK_ACCESS_FS_OPTIONAL,
            LANDLOCK_ACCESS_FS_TRUNCATE,
            &layers1,
        ),
    );
    KUNIT_EXPECT_EQ(
        test,
        0x20,
        landlock_get_deny_masks(
            _LANDLOCK_ACCESS_FS_OPTIONAL,
            LANDLOCK_ACCESS_FS_IOCTL_DEV,
            &layers1,
        ),
    );
    KUNIT_EXPECT_EQ(
        test,
        0x21,
        landlock_get_deny_masks(
            _LANDLOCK_ACCESS_FS_OPTIONAL,
            LANDLOCK_ACCESS_FS_TRUNCATE | LANDLOCK_ACCESS_FS_IOCTL_DEV,
            &layers1,
        ),
    );
}

// Static KUnit test registration from the C source:
// static struct kunit_case test_cases[] = {
//     KUNIT_CASE(test_get_layer_deny_mask),
//     KUNIT_CASE(test_landlock_get_deny_masks),
//     {}
// };
//
// static struct kunit_suite test_suite = {
//     .name = "landlock_domain",
//     .test_cases = test_cases,
// };
//
// kunit_test_suite(test_suite);

// #endif /* CONFIG_SECURITY_LANDLOCK_KUNIT_TEST */

// #endif /* CONFIG_SECURITY_LANDLOCK_LOG */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
