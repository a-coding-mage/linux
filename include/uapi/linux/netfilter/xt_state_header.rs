/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// C header guard: _XT_STATE_H

// IP_CT_IS_REPLY and IP_CT_NUMBER are supplied by the corresponding
// connection-tracking headers.
#[macro_export]
macro_rules! XT_STATE_BIT {
    ($ctinfo:expr) => {
        1 << (($ctinfo) % IP_CT_IS_REPLY + 1)
    };
}

pub const XT_STATE_INVALID: i32 = 1 << 0;

pub const XT_STATE_UNTRACKED: i32 = 1 << (IP_CT_NUMBER + 1);

#[repr(C)]
pub struct xt_state_info {
    pub statemask: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
