/* SPDX-License-Identifier: GPL-2.0 */
/*
 * ECDH helper functions - KPP wrappings
 *
 * Copyright (C) 2017 Intel Corporation
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT OF THIRD PARTY RIGHTS.
 * IN NO EVENT SHALL THE COPYRIGHT HOLDER(S) AND AUTHOR(S) BE LIABLE FOR ANY
 * CLAIM, ANY SPECIAL INDIRECT OR CONSEQUENTIAL DAMAGES, OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR
 * IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ALL LIABILITY, INCLUDING LIABILITY FOR INFRINGEMENT OF ANY PATENTS,
 * COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS, RELATING TO USE OF THIS
 * SOFTWARE IS DISCLAIMED.
 */

// Dependencies supplied by the corresponding C crypto and Linux type headers.

#[allow(non_camel_case_types)]
pub enum crypto_kpp {}

extern "C" {
    pub fn compute_ecdh_secret(
        tfm: *mut crypto_kpp,
        pair_public_key: *const u8,
        secret: *mut u8,
    ) -> i32;

    pub fn set_ecdh_privkey(tfm: *mut crypto_kpp, private_key: *const u8) -> i32;

    pub fn generate_ecdh_public_key(tfm: *mut crypto_kpp, public_key: *mut u8) -> i32;

    pub fn generate_ecdh_keys(tfm: *mut crypto_kpp, public_key: *mut u8) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
