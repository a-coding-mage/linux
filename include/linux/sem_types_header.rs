/* SPDX-License-Identifier: GPL-2.0 */
// Translated from linux/sem_types.h.

pub struct sem_undo_list;

#[repr(C)]
pub struct sysv_sem {
    #[cfg(CONFIG_SYSVIPC)]
    pub undo_list: *mut sem_undo_list,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
