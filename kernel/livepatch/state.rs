// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * system_state.c - State of the system modified by livepatches
 *
 * Copyright (C) 2019 SUSE
 */

// The declarations below are provided by the corresponding livepatch headers
// and other translation units in the kernel build.

use core::ffi::c_ulong;

#[repr(C)]
pub struct klp_state {
    pub id: c_ulong,
    pub version: c_ulong,
}

#[repr(C)]
pub struct klp_patch {
    pub states: *mut klp_state,
    pub replace: bool,
    pub next: *mut klp_patch,
}

extern "C" {
    pub static mut klp_transition_patch: *mut klp_patch;
    pub static mut klp_patches: *mut klp_patch;
    pub fn WARN_ON_ONCE(condition: bool) -> bool;
}

unsafe fn klp_for_each_state<F>(patch: *mut klp_patch, mut body: F)
where
    F: FnMut(*mut klp_state),
{
    let mut state = (*patch).states;
    while !state.is_null() && (*state).id != 0 {
        body(state);
        state = state.add(1);
    }
}

/// Get information about system state modified by the given patch.
#[no_mangle]
pub unsafe extern "C" fn klp_get_state(
    patch: *mut klp_patch,
    id: c_ulong,
) -> *mut klp_state {
    let mut result = core::ptr::null_mut();

    klp_for_each_state(patch, |state| {
        if (*state).id == id {
            result = state;
        }
    });

    result
}

/// Get information about system state modified by the already installed livepatches.
#[no_mangle]
pub unsafe extern "C" fn klp_get_prev_state(id: c_ulong) -> *mut klp_state {
    let mut patch: *mut klp_patch;
    let mut state: *mut klp_state;
    let mut last_state: *mut klp_state = core::ptr::null_mut();

    if WARN_ON_ONCE(klp_transition_patch.is_null()) {
        return core::ptr::null_mut();
    }

    patch = klp_patches;
    while !patch.is_null() {
        if patch == klp_transition_patch {
            break;
        }

        state = klp_get_state(patch, id);
        if !state.is_null() {
            last_state = state;
        }

        patch = (*patch).next;
    }

    last_state
}

/* Check if the patch is able to deal with the existing system state. */
unsafe fn klp_is_state_compatible(
    patch: *mut klp_patch,
    old_state: *mut klp_state,
) -> bool {
    let state = klp_get_state(patch, (*old_state).id);

    /* A cumulative livepatch must handle all already modified states. */
    if state.is_null() {
        return !(*patch).replace;
    }

    (*state).version >= (*old_state).version
}

/*
 * Check that the new livepatch will not break the existing system states.
 * Cumulative patches must handle all already modified states.
 * Non-cumulative patches can touch already modified states.
 */
#[no_mangle]
pub unsafe extern "C" fn klp_is_patch_compatible(patch: *mut klp_patch) -> bool {
    let mut old_patch = klp_patches;
    while !old_patch.is_null() {
        let mut compatible = true;
        klp_for_each_state(old_patch, |old_state| {
            if compatible && !klp_is_state_compatible(patch, old_state) {
                compatible = false;
            }
        });

        if !compatible {
            return false;
        }
        old_patch = (*old_patch).next;
    }

    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
