/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright (C) 2019 - 2021
 *
 * Richard van Schagen <vschagen@icloud.com>
 * Christian Marangi <ansuelsmth@gmail.com>
 */

// C header guard: _EIP93_DES_H_

extern "C" {
    pub static mut eip93_alg_ecb_des: eip93_alg_template;
    pub static mut eip93_alg_cbc_des: eip93_alg_template;
    pub static mut eip93_alg_ecb_des3_ede: eip93_alg_template;
    pub static mut eip93_alg_cbc_des3_ede: eip93_alg_template;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
