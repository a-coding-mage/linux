// SPDX-License-Identifier: GPL-2.0-only

// C dependency: <stdbool.h>

pub const UNPRIV_SYSCTL: &str = "kernel/unprivileged_bpf_disabled";

unsafe extern "C" {
    pub fn get_unpriv_disabled() -> bool;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
