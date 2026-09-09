/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2019-2021 Linaro Ltd.
 *
 * Author:
 * Sumit Garg <sumit.garg@linaro.org>
 */

// Dependency corresponding to: #include <keys/trusted-type.h>

extern "C" {
    pub static mut trusted_key_tee_ops: trusted_key_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
