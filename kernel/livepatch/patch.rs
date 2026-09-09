// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * patch.c - livepatch patching functions
 *
 * Copyright (C) 2014 Seth Jennings <sjenning@redhat.com>
 * Copyright (C) 2014 SUSE
 * Copyright (C) 2015 Josh Poimboeuf <jpoimboe@redhat.com>
 */

// #define pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// Dependencies supplied by the Linux livepatch, list, ftrace, RCU, slab,
// bug, printk, core, patch, and transition interfaces are external.

static mut klp_ops: ListHead = ListHead::new();

pub unsafe fn klp_find_ops(old_func: *mut core::ffi::c_void) -> *mut klp_ops {
    let mut ops: *mut klp_ops;
    let mut func: *mut klp_func;

    list_for_each_entry!(ops, &raw mut klp_ops, node);
    {
        func = list_first_entry!(
            &raw mut (*ops).func_stack,
            klp_func,
            stack_node,
        );
        if (*func).old_func == old_func {
            return ops;
        }
    }

    core::ptr::null_mut()
}

unsafe extern "C" fn klp_ftrace_handler(
    ip: usize,
    parent_ip: usize,
    fops: *mut ftrace_ops,
    fregs: *mut ftrace_regs,
) {
    let mut ops: *mut klp_ops;
    let mut func: *mut klp_func;
    let mut patch_state: i32;
    let mut bit: i32;

    ops = container_of!(fops, klp_ops, fops);

    /*
     * The ftrace_test_recursion_trylock() will disable preemption,
     * which is required for the variant of synchronize_rcu() that
     * is used to allow patching functions where RCU is not watching.
     * See klp_synchronize_transition() for more details.
     */
    bit = ftrace_test_recursion_trylock(ip, parent_ip);
    if (WARN_ON_ONCE!(bit < 0)) {
        return;
    }

    func = list_first_or_null_rcu!(&raw mut (*ops).func_stack, klp_func, stack_node);

    /*
     * func should never be NULL because preemption should be disabled here
     * and unregister_ftrace_function() does the equivalent of a
     * synchronize_rcu() before the func_stack removal.
     */
    if (WARN_ON_ONCE!(func.is_null())) {
        goto!(unlock);
    }

    /*
     * In the enable path, enforce the order of the ops->func_stack and
     * func->transition reads.  The corresponding write barrier is in
     * __klp_enable_patch().
     *
     * (Note that this barrier technically isn't needed in the disable
     * path.  In the rare case where klp_update_patch_state() runs before
     * this handler, its TIF_PATCH_PENDING read and this func->transition
     * read need to be ordered.  But klp_update_patch_state() already
     * enforces that.)
     */
    smp_rmb!();

    if (unlikely!((*func).transition)) {
        /*
         * Enforce the order of the func->transition and
         * current->patch_state reads.  Otherwise we could read an
         * out-of-date task state and pick the wrong function.  The
         * corresponding write barrier is in klp_init_transition().
         */
        smp_rmb!();

        patch_state = (*current).patch_state;

        WARN_ON_ONCE!(patch_state == KLP_TRANSITION_IDLE);

        if patch_state == KLP_TRANSITION_UNPATCHED {
            /*
             * Use the previously patched version of the function.
             * If no previous patches exist, continue with the
             * original function.
             */
            func = list_entry_rcu!((*func).stack_node.next, klp_func, stack_node);

            if core::ptr::eq(
                &raw mut (*func).stack_node,
                &raw mut (*ops).func_stack,
            ) {
                goto!(unlock);
            }
        }
    }

    /*
     * NOPs are used to replace existing patches with original code.
     * Do nothing! Setting pc would cause an infinite loop.
     */
    if (*func).nop {
        goto!(unlock);
    }

    ftrace_regs_set_instruction_pointer(fregs, (*func).new_func as usize);

unlock:
    ftrace_test_recursion_unlock(bit);
}

unsafe fn klp_unpatch_func(func: *mut klp_func) {
    let mut ops: *mut klp_ops;

    if WARN_ON!(!(*func).patched) {
        return;
    }
    if WARN_ON!((*func).old_func.is_null()) {
        return;
    }

    ops = klp_find_ops((*func).old_func);
    if WARN_ON!(ops.is_null()) {
        return;
    }

    if list_is_singular!(&raw mut (*ops).func_stack) {
        let ftrace_loc: usize;

        ftrace_loc = ftrace_location((*func).old_func as usize);
        if WARN_ON!(ftrace_loc == 0) {
            return;
        }

        WARN_ON!(unregister_ftrace_function(&raw mut (*ops).fops));
        WARN_ON!(ftrace_set_filter_ip(&raw mut (*ops).fops, ftrace_loc, 1, 0));

        list_del_rcu!(&raw mut (*func).stack_node);
        list_del!(&raw mut (*ops).node);
        kfree(ops);
    } else {
        list_del_rcu!(&raw mut (*func).stack_node);
    }

    (*func).patched = false;
}

unsafe fn klp_patch_func(func: *mut klp_func) -> i32 {
    let mut ops: *mut klp_ops;
    let ret: i32;

    if WARN_ON!((*func).old_func.is_null()) {
        return -EINVAL;
    }

    if WARN_ON!((*func).patched) {
        return -EINVAL;
    }

    ops = klp_find_ops((*func).old_func);
    if ops.is_null() {
        let ftrace_loc: usize;

        ftrace_loc = ftrace_location((*func).old_func as usize);
        if ftrace_loc == 0 {
            pr_err!("failed to find location for function '{}'\n", (*func).old_name);
            return -EINVAL;
        }

        ops = kzalloc_obj!(*ops);
        if ops.is_null() {
            return -ENOMEM;
        }

        (*ops).fops.func = Some(klp_ftrace_handler);
        (*ops).fops.flags = FTRACE_OPS_FL_DYNAMIC |
            // #ifndef CONFIG_HAVE_DYNAMIC_FTRACE_WITH_ARGS
            FTRACE_OPS_FL_SAVE_REGS |
            // #endif
            FTRACE_OPS_FL_IPMODIFY |
            FTRACE_OPS_FL_PERMANENT;

        list_add!(&raw mut (*ops).node, &raw mut klp_ops);

        INIT_LIST_HEAD!(&raw mut (*ops).func_stack);
        list_add_rcu!(&raw mut (*func).stack_node, &raw mut (*ops).func_stack);

        ret = ftrace_set_filter_ip(&raw mut (*ops).fops, ftrace_loc, 0, 0);
        if ret != 0 {
            pr_err!(
                "failed to set ftrace filter for function '{}' ({})\n",
                (*func).old_name,
                ret,
            );
            goto!(err);
        }

        ret = register_ftrace_function(&raw mut (*ops).fops);
        if ret != 0 {
            pr_err!(
                "failed to register ftrace handler for function '{}' ({})\n",
                (*func).old_name,
                ret,
            );
            ftrace_set_filter_ip(&raw mut (*ops).fops, ftrace_loc, 1, 0);
            goto!(err);
        }
    } else {
        list_add_rcu!(&raw mut (*func).stack_node, &raw mut (*ops).func_stack);
    }

    (*func).patched = true;

    return 0;

err:
    list_del_rcu!(&raw mut (*func).stack_node);
    list_del!(&raw mut (*ops).node);
    kfree(ops);
    ret
}

unsafe fn __klp_unpatch_object(obj: *mut klp_object, nops_only: bool) {
    let mut func: *mut klp_func;

    klp_for_each_func!(obj, func);
    {
        if nops_only && !(*func).nop {
            continue;
        }

        if (*func).patched {
            klp_unpatch_func(func);
        }
    }

    if (*obj).dynamic || !nops_only {
        (*obj).patched = false;
    }
}

pub unsafe fn klp_unpatch_object(obj: *mut klp_object) {
    __klp_unpatch_object(obj, false);
}

pub unsafe fn klp_patch_object(obj: *mut klp_object) -> i32 {
    let mut func: *mut klp_func;
    let ret: i32;

    if WARN_ON!((*obj).patched) {
        return -EINVAL;
    }

    klp_for_each_func!(obj, func);
    {
        ret = klp_patch_func(func);
        if ret != 0 {
            klp_unpatch_object(obj);
            return ret;
        }
    }
    (*obj).patched = true;

    0
}

unsafe fn __klp_unpatch_objects(patch: *mut klp_patch, nops_only: bool) {
    let mut obj: *mut klp_object;

    klp_for_each_object!(patch, obj);
    if (*obj).patched {
        __klp_unpatch_object(obj, nops_only);
    }
}

pub unsafe fn klp_unpatch_objects(patch: *mut klp_patch) {
    __klp_unpatch_objects(patch, false);
}

pub unsafe fn klp_unpatch_objects_dynamic(patch: *mut klp_patch) {
    __klp_unpatch_objects(patch, true);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
