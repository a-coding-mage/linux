/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright (C) 2019 - 2021
 *
 * Richard van Schagen <vschagen@icloud.com>
 * Christian Marangi <ansuelsmth@gmail.com>
 */

extern "C" {
    pub static mut eip93_alg_ecb_aes: eip93_alg_template;
    pub static mut eip93_alg_cbc_aes: eip93_alg_template;
    pub static mut eip93_alg_ctr_aes: eip93_alg_template;
    pub static mut eip93_alg_rfc3686_aes: eip93_alg_template;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
