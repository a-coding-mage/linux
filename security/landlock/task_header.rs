// SPDX-License-Identifier: GPL-2.0-only
/*
 * Landlock LSM - Ptrace hooks
 *
 * Copyright © 2017-2019 Mickaël Salaün <mic@digikod.net>
 * Copyright © 2019 ANSSI
 */

// Header guard removed: _SECURITY_LANDLOCK_TASK_H

unsafe extern "C" {
    pub fn landlock_add_task_hooks();
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
