// SPDX-License-Identifier: GPL-2.0
/* Multipath TCP cryptographic functions
 * Copyright (c) 2017 - 2019, Intel Corporation.
 *
 * Note: This code is based on mptcp_ctrl.c, mptcp_ipv4.c, and
 *       mptcp_ipv6 from multipath-tcp.org, authored by:
 *
 *       Sébastien Barré <sebastien.barre@uclouvain.be>
 *       Christoph Paasch <christoph.paasch@uclouvain.be>
 *       Jaakko Korkeaniemi <jaakko.korkeaniemi@aalto.fi>
 *       Gregory Detal <gregory.detal@uclouvain.be>
 *       Fabien Duchêne <fabien.duchene@uclouvain.be>
 *       Andreas Seelinger <Andreas.Seelinger@rwth-aachen.de>
 *       Lavkesh Lahngir <lavkesh51@gmail.com>
 *       Andreas Ripke <ripke@neclab.eu>
 *       Vlad Dogaru <vlad.dogaru@intel.com>
 *       Octavian Purdila <octavian.purdila@intel.com>
 *       John Ronan <jronan@tssg.org>
 *       Catalin Nicutar <catalin.nicutar@gmail.com>
 *       Brandon Heller <brandonh@stanford.edu>
 */

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    fn sha256(data: *const u8, len: usize, digest: *mut u8);
    fn hmac_sha256_usingrawkey(
        key: *const u8,
        key_len: usize,
        msg: *const u8,
        msg_len: i32,
        hmac: *mut core::ffi::c_void,
    );
}

const SHA256_DIGEST_WORDS: usize = 32 / 4;

pub unsafe extern "C" fn mptcp_crypto_key_sha(key: u64, token: *mut u32, idsn: *mut u64) {
    let mut mptcp_hashed_key: [u32; SHA256_DIGEST_WORDS] = [0; SHA256_DIGEST_WORDS];
    let input: u64 = key.to_be();

    sha256(
        (&input as *const u64).cast::<u8>(),
        core::mem::size_of::<u64>(),
        mptcp_hashed_key.as_mut_ptr().cast::<u8>(),
    );

    if !token.is_null() {
        *token = u32::from_be(mptcp_hashed_key[0]);
    }
    if !idsn.is_null() {
        *idsn = u64::from_be(
            core::ptr::read_unaligned(mptcp_hashed_key.as_ptr().add(6).cast::<u64>()),
        );
    }
}

pub unsafe extern "C" fn mptcp_crypto_hmac_sha(
    key1: u64,
    key2: u64,
    msg: *mut u8,
    len: i32,
    hmac: *mut core::ffi::c_void,
) {
    let key: [u64; 2] = [key1.to_be(), key2.to_be()];

    hmac_sha256_usingrawkey(
        key.as_ptr().cast::<u8>(),
        core::mem::size_of_val(&key),
        msg.cast_const(),
        len,
        hmac,
    );
}

// The C source exports mptcp_crypto_hmac_sha when IS_MODULE(CONFIG_MPTCP_KUNIT_TEST).

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
