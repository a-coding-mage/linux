/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies:
// #include <linux/types.h>
// #include <crypto/aes.h>

extern "C" {
    pub static mut p8_aes_cbc_alg: skcipher_alg;
    pub static mut p8_aes_ctr_alg: skcipher_alg;
    pub static mut p8_aes_xts_alg: skcipher_alg;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
