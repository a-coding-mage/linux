/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2023 ARM Ltd.
 */

/* Dependencies supplied by the surrounding kernel translation. */

pub const RSI_GRANULE_SHIFT: u32 = 12;
pub const RSI_GRANULE_SIZE: usize = 1usize << RSI_GRANULE_SHIFT;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ripas {
    RSI_RIPAS_EMPTY = 0,
    RSI_RIPAS_RAM = 1,
    RSI_RIPAS_DESTROYED = 2,
    RSI_RIPAS_DEV = 3,
}

pub unsafe fn rsi_request_version(
    req: usize,
    out_lower: *mut usize,
    out_higher: *mut usize,
) -> usize {
    let mut res: arm_smccc_res = core::mem::zeroed();

    arm_smccc_smc(SMC_RSI_ABI_VERSION, req, 0, 0, 0, 0, 0, 0, &mut res);

    if !out_lower.is_null() {
        *out_lower = res.a1;
    }
    if !out_higher.is_null() {
        *out_higher = res.a2;
    }

    res.a0
}

pub unsafe fn rsi_get_realm_config(cfg: *mut realm_config) -> usize {
    let mut res: arm_smccc_res = core::mem::zeroed();

    arm_smccc_smc(SMC_RSI_REALM_CONFIG, virt_to_phys(cfg), 0, 0, 0, 0, 0, 0, &mut res);
    res.a0
}

pub unsafe fn rsi_ipa_state_get(
    start: phys_addr_t,
    end: phys_addr_t,
    state: *mut ripas,
    top: *mut phys_addr_t,
) -> usize {
    let mut res: arm_smccc_res = core::mem::zeroed();

    arm_smccc_smc(SMC_RSI_IPA_STATE_GET, start, end, 0, 0, 0, 0, 0, &mut res);

    if res.a0 == RSI_SUCCESS {
        if !top.is_null() {
            *top = res.a1;
        }
        if !state.is_null() {
            *state = res.a2 as ripas;
        }
    }

    res.a0
}

pub unsafe fn rsi_set_addr_range_state(
    start: phys_addr_t,
    end: phys_addr_t,
    state: ripas,
    flags: usize,
    top: *mut phys_addr_t,
) -> isize {
    let mut res: arm_smccc_res = core::mem::zeroed();

    arm_smccc_smc(SMC_RSI_IPA_STATE_SET, start, end, state as usize, flags, 0, 0, 0, &mut res);

    if !top.is_null() {
        *top = res.a1;
    }

    if res.a2 != RSI_ACCEPT {
        return -EPERM;
    }

    res.a0 as isize
}

pub const RSI_ATTEST_CHALLENGE_MIN_SIZE: usize = 32;
pub const RSI_ATTEST_CHALLENGE_MAX_SIZE: usize = 64;

#[repr(C)]
pub struct rsi_attestation_token_init_args {
    pub fid: usize,
    pub challenge: [u8; RSI_ATTEST_CHALLENGE_MAX_SIZE],
}

/**
 * rsi_attestation_token_init - Initialise the operation to retrieve an
 * attestation token.
 *
 * @challenge: The challenge data to be used in the attestation token
 *             generation.
 * @size:      Size of the challenge data in bytes.
 *
 * Initialises the attestation token generation and returns an upper bound
 * on the attestation token size that can be used to allocate an adequate
 * buffer. The caller is expected to subsequently call
 * rsi_attestation_token_continue() to retrieve the attestation token data
 * on the same CPU.
 *
 * Returns:
 *  On success, returns the upper limit of the attestation report size.
 *  Otherwise, -EINVAL
 */
pub unsafe fn rsi_attestation_token_init(challenge: *const u8, size: usize) -> isize {
    let mut args: rsi_attestation_token_init_union = core::mem::zeroed();

    if challenge.is_null()
        || size < RSI_ATTEST_CHALLENGE_MIN_SIZE
        || size > RSI_ATTEST_CHALLENGE_MAX_SIZE
    {
        return -EINVAL;
    }

    (*args.init()).fid = SMC_RSI_ATTESTATION_TOKEN_INIT;
    core::ptr::copy_nonoverlapping(challenge, (*args.init()).challenge.as_mut_ptr(), size);
    arm_smccc_1_2_smc(args.regs_mut(), args.regs_mut());

    if (*args.regs()).a0 == RSI_SUCCESS {
        return (*args.regs()).a1 as isize;
    }

    -EINVAL
}

#[repr(C)]
pub union rsi_attestation_token_init_union {
    pub regs: arm_smccc_1_2_regs,
    pub init: rsi_attestation_token_init_args,
}

impl rsi_attestation_token_init_union {
    unsafe fn regs(&self) -> *const arm_smccc_1_2_regs { &self.regs }
    unsafe fn regs_mut(&mut self) -> *mut arm_smccc_1_2_regs { &mut self.regs }
    unsafe fn init(&mut self) -> *mut rsi_attestation_token_init_args { &mut self.init }
}

pub unsafe fn rsi_attestation_token_continue(
    granule: phys_addr_t,
    offset: usize,
    size: usize,
    len: *mut usize,
) -> usize {
    let mut res: arm_smccc_res = core::mem::zeroed();

    arm_smccc_1_1_invoke(SMC_RSI_ATTESTATION_TOKEN_CONTINUE, granule, offset, size, 0, &mut res);

    if !len.is_null() {
        *len = res.a1;
    }
    res.a0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
