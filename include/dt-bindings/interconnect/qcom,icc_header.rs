/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2020, The Linux Foundation. All rights reserved.
 */

/*
 * The AMC bucket denotes constraints that are applied to hardware when
 * icc_set_bw() completes, whereas the WAKE and SLEEP constraints are applied
 * when the execution environment transitions between active and low power mode.
 */
pub const QCOM_ICC_BUCKET_AMC: u32 = 0;
pub const QCOM_ICC_BUCKET_WAKE: u32 = 1;
pub const QCOM_ICC_BUCKET_SLEEP: u32 = 2;
pub const QCOM_ICC_NUM_BUCKETS: u32 = 3;

pub const QCOM_ICC_TAG_AMC: u32 = 1u32 << QCOM_ICC_BUCKET_AMC;
pub const QCOM_ICC_TAG_WAKE: u32 = 1u32 << QCOM_ICC_BUCKET_WAKE;
pub const QCOM_ICC_TAG_SLEEP: u32 = 1u32 << QCOM_ICC_BUCKET_SLEEP;
pub const QCOM_ICC_TAG_ACTIVE_ONLY: u32 = QCOM_ICC_TAG_AMC | QCOM_ICC_TAG_WAKE;
pub const QCOM_ICC_TAG_ALWAYS: u32 =
    QCOM_ICC_TAG_AMC | QCOM_ICC_TAG_WAKE | QCOM_ICC_TAG_SLEEP;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
