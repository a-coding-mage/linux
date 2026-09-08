/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Landlock - Credential hooks
 *
 * Copyright © 2019-2020 Mickaël Salaün <mic@digikod.net>
 * Copyright © 2019-2020 ANSSI
 * Copyright © 2021-2025 Microsoft Corporation
 */

/* Dependencies from the original header:
 * <linux/container_of.h>
 * <linux/cred.h>
 * <linux/init.h>
 * <linux/rcupdate.h>
 * "access.h"
 * "domain.h"
 * "limits.h"
 * "ruleset.h"
 * "setup.h"
 */

/**
 * struct landlock_cred_security - Credential security blob
 *
 * This structure is packed to minimize the size of struct
 * landlock_file_security.  However, it is always aligned in the LSM cred blob,
 * see lsm_set_blob_size().
 *
 * When updating this, also update landlock_cred_copy() if needed.
 */
#[repr(C, packed)]
pub struct landlock_cred_security {
    /**
     * @domain: Immutable domain enforced on a task.
     */
    pub domain: *mut landlock_domain,

    /* CONFIG_SECURITY_LANDLOCK_LOG */
    #[cfg(CONFIG_SECURITY_LANDLOCK_LOG)]
    /**
     * @domain_exec: Bitmask identifying the domain layers that were enforced by
     * the current task's executed file (i.e. no new execve(2) since
     * landlock_restrict_self(2)).
     */
    pub domain_exec: u16,
    #[cfg(CONFIG_SECURITY_LANDLOCK_LOG)]
    /**
     * @log_subdomains_off: Set if the domain descendants's log_status should be
     * set to %LANDLOCK_LOG_DISABLED.  This is not a landlock_hierarchy
     * configuration because it applies to future descendant domains and it does
     * not require a current domain.
     *
     * Original C field is a one-bit u8 bitfield.
     */
    pub log_subdomains_off: u8,
}

/* CONFIG_SECURITY_LANDLOCK_LOG:
 * Makes sure all layer executions can be stored.
 *
 * Original C:
 * static_assert(BITS_PER_TYPE(typeof_member(struct landlock_cred_security,
 *                                           domain_exec)) >=
 *               LANDLOCK_MAX_NUM_LAYERS);
 */
#[cfg(CONFIG_SECURITY_LANDLOCK_LOG)]
const _: [(); 1] = [(); (u16::BITS as usize >= LANDLOCK_MAX_NUM_LAYERS) as usize];

#[inline]
pub unsafe fn landlock_cred(cred: *const cred) -> *mut landlock_cred_security {
    unsafe { ((*cred).security as *mut u8).add(landlock_blob_sizes.lbs_cred) as *mut landlock_cred_security }
}

#[inline]
pub unsafe fn landlock_cred_copy(
    dst: *mut landlock_cred_security,
    src: *const landlock_cred_security,
) {
    unsafe {
        landlock_put_domain((*dst).domain);

        core::ptr::copy_nonoverlapping(src, dst, 1);

        landlock_get_domain((*src).domain);
    }
}

#[inline]
pub unsafe fn landlock_get_current_domain() -> *mut landlock_domain {
    unsafe { (*landlock_cred(current_cred())).domain }
}

/* The call needs to come from an RCU read-side critical section. */
#[inline]
pub unsafe fn landlock_get_task_domain(
    task: *const task_struct,
) -> *const landlock_domain {
    unsafe { (*landlock_cred(__task_cred(task))).domain }
}

#[inline]
pub unsafe fn landlocked(task: *const task_struct) -> bool {
    let has_dom: bool;

    unsafe {
        if task == current {
            return !landlock_get_current_domain().is_null();
        }

        rcu_read_lock();
        has_dom = !landlock_get_task_domain(task).is_null();
        rcu_read_unlock();
    }
    has_dom
}

/**
 * landlock_get_applicable_subject - Return the subject's Landlock credential
 *                                   if its enforced domain applies to (i.e.
 *                                   handles) at least one of the access rights
 *                                   specified in @masks
 *
 * @cred: credential
 * @masks: access masks
 * @handle_layer: returned youngest layer handling a subset of @masks.  Not set
 *                if the function returns NULL.
 *
 * Return: landlock_cred(@cred) if any access rights specified in @masks is
 * handled, or NULL otherwise.
 */
#[inline]
pub unsafe fn landlock_get_applicable_subject(
    cred: *const cred,
    masks: access_masks,
    handle_layer: *mut usize,
) -> *const landlock_cred_security {
    let masks_all = access_masks_all { masks };
    let domain: *const landlock_domain;
    let mut layer_level: isize;

    unsafe {
        if cred.is_null() {
            return core::ptr::null();
        }

        domain = (*landlock_cred(cred)).domain;
        if domain.is_null() {
            return core::ptr::null();
        }

        layer_level = (*domain).num_layers as isize - 1;
        while layer_level >= 0 {
            let layer = access_masks_all {
                masks: (*domain).handled_masks[layer_level as usize],
            };

            if layer.all & masks_all.all != 0 {
                if !handle_layer.is_null() {
                    *handle_layer = layer_level as usize;
                }

                return landlock_cred(cred);
            }

            layer_level -= 1;
        }
    }

    core::ptr::null()
}

extern "C" {
    pub fn landlock_add_cred_hooks();
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
