// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2020 Arm Ltd.

// Dependencies supplied by the surrounding kernel translation.

const ARM_SMCCC_TRNG_VERSION_1_0: libc::c_ulong = 0x10000;

/* Those values are deliberately separate from the generic SMCCC definitions. */
const TRNG_SUCCESS: libc::c_ulong = 0;
const TRNG_NOT_SUPPORTED: libc::c_ulong = (-1i64) as libc::c_ulong;
const TRNG_INVALID_PARAMETER: libc::c_ulong = (-2i64) as libc::c_ulong;
const TRNG_NO_ENTROPY: libc::c_ulong = (-3i64) as libc::c_ulong;

const TRNG_MAX_BITS64: usize = 192;

static arm_smc_trng_uuid: uuid_t = UUID_INIT!(
    0x0d21e000, 0x4384, 0x11eb, 0x80, 0x70, 0x52, 0x44, 0x55, 0x4e, 0x5a, 0x4c
);

unsafe fn kvm_trng_do_rnd(vcpu: *mut kvm_vcpu, size: libc::c_int) -> libc::c_int {
    let mut bits: [libc::c_ulong; 3] = [0; 3];
    let num_bits: u32 = smccc_get_arg1(vcpu);
    let mut i: libc::c_int = 0;

    if num_bits > (3 * size) as u32 {
        smccc_set_retval(vcpu, TRNG_INVALID_PARAMETER, 0, 0, 0);
        return 1;
    }

    /* get as many bits as we need to fulfil the request */
    while i < ((num_bits as usize + BITS_PER_LONG - 1) / BITS_PER_LONG) as libc::c_int {
        bits[i as usize] = get_random_long();
        i += 1;
    }

    bitmap_clear(bits.as_mut_ptr(), num_bits as usize, TRNG_MAX_BITS64 - num_bits as usize);

    if size == 32 {
        smccc_set_retval(
            vcpu,
            TRNG_SUCCESS,
            lower_32_bits(bits[1]),
            upper_32_bits(bits[0]),
            lower_32_bits(bits[0]),
        );
    } else {
        smccc_set_retval(vcpu, TRNG_SUCCESS, bits[2], bits[1], bits[0]);
    }

    memzero_explicit(bits.as_mut_ptr() as *mut libc::c_void, core::mem::size_of_val(&bits));
    1
}

unsafe fn kvm_trng_call(vcpu: *mut kvm_vcpu) -> libc::c_int {
    let u: *const __le32 = (*core::ptr::addr_of!(arm_smc_trng_uuid)).b.as_ptr() as *const __le32;
    let func_id: u32 = smccc_get_function(vcpu);
    let mut val: libc::c_ulong = TRNG_NOT_SUPPORTED;
    let mut size: libc::c_int = 64;

    match func_id {
        ARM_SMCCC_TRNG_VERSION => {
            val = ARM_SMCCC_TRNG_VERSION_1_0;
        }
        ARM_SMCCC_TRNG_FEATURES => {
            match smccc_get_arg1(vcpu) {
                ARM_SMCCC_TRNG_VERSION
                | ARM_SMCCC_TRNG_FEATURES
                | ARM_SMCCC_TRNG_GET_UUID
                | ARM_SMCCC_TRNG_RND32
                | ARM_SMCCC_TRNG_RND64 => val = TRNG_SUCCESS,
                _ => {}
            }
        }
        ARM_SMCCC_TRNG_GET_UUID => {
            smccc_set_retval(
                vcpu,
                le32_to_cpu(*u.add(0)),
                le32_to_cpu(*u.add(1)),
                le32_to_cpu(*u.add(2)),
                le32_to_cpu(*u.add(3)),
            );
            return 1;
        }
        ARM_SMCCC_TRNG_RND32 => {
            size = 32;
            return kvm_trng_do_rnd(vcpu, size);
        }
        ARM_SMCCC_TRNG_RND64 => return kvm_trng_do_rnd(vcpu, size),
        _ => {}
    }

    smccc_set_retval(vcpu, val, 0, 0, 0);
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
