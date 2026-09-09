/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Interface for managing mitigations for riscv vulnerabilities.
 *
 * Copyright (C) 2024 Rivos Inc.
 */

/* Watch out, ordering is important here. */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum mitigation_state {
    UNAFFECTED = 0,
    MITIGATED = 1,
    VULNERABLE = 2,
}

extern "C" {
    pub fn ghostwrite_set_vulnerable();
    pub fn ghostwrite_enable_mitigation() -> bool;
    pub fn ghostwrite_get_state() -> mitigation_state;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
