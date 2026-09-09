/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * SHA-3 optimized using the CP Assist for Cryptographic Functions (CPACF)
 *
 * Copyright 2025 Google LLC
 */

// C dependencies supplied by other translation units are intentionally not
// implemented here.

static mut have_sha3: StaticKey = StaticKey { _private: [] };
static mut have_sha3_init_optim: StaticKey = StaticKey { _private: [] };

#[repr(C)]
struct StaticKey {
    _private: [u8; 0],
}

unsafe fn sha3_absorb_blocks(
    state: *mut sha3_state,
    data: *const u8,
    nblocks: usize,
    block_size: usize,
) {
    if static_branch_likely(&raw mut have_sha3) {
        /*
         * Note that KIMD assumes little-endian order of the state
         * words.  sha3_state already uses that order, though, so there's
         * no need for a byteswap.
         */
        match block_size {
            SHA3_224_BLOCK_SIZE => {
                cpacf_kimd(CPACF_KIMD_SHA3_224, state, data, nblocks * block_size);
                return;
            }
            SHA3_256_BLOCK_SIZE => {
                /*
                 * This case handles both SHA3-256 and SHAKE256, since
                 * they have the same block size.
                 */
                cpacf_kimd(CPACF_KIMD_SHA3_256, state, data, nblocks * block_size);
                return;
            }
            SHA3_384_BLOCK_SIZE => {
                cpacf_kimd(CPACF_KIMD_SHA3_384, state, data, nblocks * block_size);
                return;
            }
            SHA3_512_BLOCK_SIZE => {
                cpacf_kimd(CPACF_KIMD_SHA3_512, state, data, nblocks * block_size);
                return;
            }
            _ => {}
        }
    }
    sha3_absorb_blocks_generic(state, data, nblocks, block_size);
}

unsafe fn sha3_keccakf(state: *mut sha3_state) {
    if static_branch_likely(&raw mut have_sha3) {
        /* Passing zeroes to CPACF performs the plain Keccak-f permutation. */
        static zeroes: [u8; SHA3_512_BLOCK_SIZE] = [0; SHA3_512_BLOCK_SIZE];
        cpacf_kimd(CPACF_KIMD_SHA3_512, state, zeroes.as_ptr(), zeroes.len());
    } else {
        sha3_keccakf_generic(state);
    }
}

unsafe fn s390_sha3(
    func: i32,
    input: *const u8,
    in_len: usize,
    output: *mut u8,
    out_len: usize,
) -> bool {
    let mut state = core::mem::MaybeUninit::<sha3_state>::uninit();

    if !static_branch_likely(&raw mut have_sha3) {
        return false;
    }

    let mut actual_func = func;
    if static_branch_likely(&raw mut have_sha3_init_optim) {
        actual_func |= CPACF_KLMD_NIP | CPACF_KLMD_DUFOP;
    } else {
        core::ptr::write_bytes(state.as_mut_ptr() as *mut u8, 0, core::mem::size_of::<sha3_state>());
    }

    cpacf_klmd(actual_func, state.as_mut_ptr(), input, in_len);

    if static_branch_likely(&raw mut have_sha3_init_optim) {
        kmsan_unpoison_memory(state.as_mut_ptr() as *mut u8, out_len);
    }

    core::ptr::copy_nonoverlapping(state.as_ptr() as *const u8, output, out_len);
    memzero_explicit(state.as_mut_ptr() as *mut u8, core::mem::size_of::<sha3_state>());
    true
}

unsafe fn sha3_224_arch(input: *const u8, in_len: usize, output: *mut u8) -> bool {
    s390_sha3(CPACF_KLMD_SHA3_224, input, in_len, output, SHA3_224_DIGEST_SIZE)
}

unsafe fn sha3_256_arch(input: *const u8, in_len: usize, output: *mut u8) -> bool {
    s390_sha3(CPACF_KLMD_SHA3_256, input, in_len, output, SHA3_256_DIGEST_SIZE)
}

unsafe fn sha3_384_arch(input: *const u8, in_len: usize, output: *mut u8) -> bool {
    s390_sha3(CPACF_KLMD_SHA3_384, input, in_len, output, SHA3_384_DIGEST_SIZE)
}

unsafe fn sha3_512_arch(input: *const u8, in_len: usize, output: *mut u8) -> bool {
    s390_sha3(CPACF_KLMD_SHA3_512, input, in_len, output, SHA3_512_DIGEST_SIZE)
}

unsafe fn sha3_mod_init_arch() {
    let mut num_present = 0;
    let mut num_possible = 0;

    if !cpu_have_feature(S390_CPU_FEATURE_MSA) {
        return;
    }

    // All SHA-3 functions are in Message-Security-Assist Extension 6.
    macro_rules! query {
        ($opcode:expr, $func:expr) => {{
            num_present += (cpacf_query_func($opcode, $func) != 0) as i32;
            num_possible += 1;
        }};
    }
    query!(CPACF_KIMD, CPACF_KIMD_SHA3_224);
    query!(CPACF_KIMD, CPACF_KIMD_SHA3_256);
    query!(CPACF_KIMD, CPACF_KIMD_SHA3_384);
    query!(CPACF_KIMD, CPACF_KIMD_SHA3_512);
    query!(CPACF_KLMD, CPACF_KLMD_SHA3_224);
    query!(CPACF_KLMD, CPACF_KLMD_SHA3_256);
    query!(CPACF_KLMD, CPACF_KLMD_SHA3_384);
    query!(CPACF_KLMD, CPACF_KLMD_SHA3_512);

    if num_present == num_possible {
        static_branch_enable(&raw mut have_sha3);
        if test_facility(86) {
            static_branch_enable(&raw mut have_sha3_init_optim);
        }
    } else if num_present != 0 {
        pr_warn("Unsupported combination of SHA-3 facilities\n");
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
