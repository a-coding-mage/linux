/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (c) 2023, Linaro Limited
 */

// Header guard: __DT_BINDINGS_INTERCONNECT_QCOM_RPM_ICC_H

pub const RPM_ACTIVE_TAG: u32 = 1 << 0;
pub const RPM_SLEEP_TAG: u32 = 1 << 1;
pub const RPM_ALWAYS_TAG: u32 = RPM_ACTIVE_TAG | RPM_SLEEP_TAG;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
