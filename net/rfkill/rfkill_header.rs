/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2007 Ivo van Doorn
 * Copyright 2009 Johannes Berg <johannes@sipsolutions.net>
 */

/*
 * The C header guard has no executable Rust equivalent.
 * The `rfkill_type` enum is supplied by another dependency.
 */

/* core code */
extern "C" {
    pub fn rfkill_switch_all(type_: rfkill_type, blocked: bool);
    pub fn rfkill_epo();
    pub fn rfkill_restore_states();
    pub fn rfkill_remove_epo_lock();
    pub fn rfkill_is_epo_lock_active() -> bool;
    pub fn rfkill_get_global_sw_state(type_: rfkill_type) -> bool;

    /* input handler */
    pub fn rfkill_handler_init() -> ::core::ffi::c_int;
    pub fn rfkill_handler_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
