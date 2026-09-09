/* SPDX-License-Identifier: GPL-2.0 */
// Header guard: __TDO24M_H__

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum tdo24m_model {
    TDO24M = 0,
    TDO35S = 1,
}

#[repr(C)]
pub struct tdo24m_platform_data {
    pub model: tdo24m_model,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
