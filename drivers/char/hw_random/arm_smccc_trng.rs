// SPDX-License-Identifier: GPL-2.0
/*
 * Randomness driver for the ARM SMCCC TRNG Firmware Interface
 * https://developer.arm.com/documentation/den0098/latest/
 *
 *  Copyright (C) 2020 Arm Ltd.
 *
 * The ARM TRNG firmware interface specifies a protocol to read entropy
 * from a higher exception level, to abstract from any machine specific
 * implemenations and allow easier use in hypervisors.
 *
 * The firmware interface is realised using the SMCCC specification.
 */

// Dependencies supplied by the kernel translation environment.

#[cfg(target_pointer_width = "64")]
const ARM_SMCCC_TRNG_RND: u32 = ARM_SMCCC_TRNG_RND64;
#[cfg(target_pointer_width = "64")]
const MAX_BITS_PER_CALL: usize = 3 * 64;
#[cfg(not(target_pointer_width = "64"))]
const ARM_SMCCC_TRNG_RND: u32 = ARM_SMCCC_TRNG_RND32;
#[cfg(not(target_pointer_width = "64"))]
const MAX_BITS_PER_CALL: usize = 3 * 32;

/* We don't want to allow the firmware to stall us forever. */
const SMCCC_TRNG_MAX_TRIES: i32 = 20;

const SMCCC_RET_TRNG_INVALID_PARAMETER: i32 = -2;
const SMCCC_RET_TRNG_NO_ENTROPY: i32 = -3;

#[repr(C)]
pub struct arm_smccc_res {
    pub a0: u64,
    pub a1: u64,
    pub a2: u64,
    pub a3: u64,
}

extern "C" {
    fn arm_smccc_1_1_invoke(function_id: u32, arg0: usize, res: *mut arm_smccc_res);
    fn cond_resched();
}

unsafe fn copy_from_registers(buf: *mut i8, res: *const arm_smccc_res, bytes: usize) -> i32 {
    let mut chunk: usize;
    let mut copied: usize;

    if bytes == 0 {
        return 0;
    }

    chunk = core::cmp::min(bytes, core::mem::size_of::<isize>());
    core::ptr::copy_nonoverlapping(
        res.cast::<u8>().add(24),
        buf.cast::<u8>(),
        chunk,
    );
    copied = chunk;
    if copied >= bytes {
        return copied as i32;
    }

    chunk = core::cmp::min(bytes - copied, core::mem::size_of::<isize>());
    core::ptr::copy_nonoverlapping(
        res.cast::<u8>().add(16),
        buf.cast::<u8>().add(copied),
        chunk,
    );
    copied += chunk;
    if copied >= bytes {
        return copied as i32;
    }

    chunk = core::cmp::min(bytes - copied, core::mem::size_of::<isize>());
    core::ptr::copy_nonoverlapping(
        res.cast::<u8>().add(8),
        buf.cast::<u8>().add(copied),
        chunk,
    );

    (copied + chunk) as i32
}

unsafe fn smccc_trng_read(
    _rng: *mut hwrng,
    data: *mut core::ffi::c_void,
    max: usize,
    wait: bool,
) -> i32 {
    let mut res = core::mem::MaybeUninit::<arm_smccc_res>::uninit();
    let buf = data.cast::<u8>();
    let mut copied: usize = 0;
    let mut tries: i32 = 0;

    while copied < max {
        let bits = core::cmp::min((max - copied) * 8, MAX_BITS_PER_CALL);

        arm_smccc_1_1_invoke(ARM_SMCCC_TRNG_RND, bits, res.as_mut_ptr());
        let res = res.assume_init_ref();

        match res.a0 as i32 {
            SMCCC_RET_SUCCESS => {
                copied += copy_from_registers(
                    buf.add(copied).cast(),
                    res,
                    bits / 8,
                ) as usize;
                tries = 0;
            }
            SMCCC_RET_TRNG_NO_ENTROPY => {
                if !wait {
                    return copied as i32;
                }
                tries += 1;
                if tries >= SMCCC_TRNG_MAX_TRIES {
                    return copied as i32;
                }
                cond_resched();
            }
            _ => return -EIO,
        }
    }

    copied as i32
}

unsafe fn smccc_trng_probe(pdev: *mut platform_device) -> i32 {
    let mut trng = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<hwrng>(),
        GFP_KERNEL,
    ) as *mut hwrng;
    if trng.is_null() {
        return -ENOMEM;
    }

    (*trng).name = b"smccc_trng\0".as_ptr().cast();
    (*trng).read = Some(smccc_trng_read);

    devm_hwrng_register(&mut (*pdev).dev, trng)
}

// The platform driver registration and module metadata are provided by the
// kernel build environment: module_platform_driver(smccc_trng_driver).
// MODULE_ALIAS("platform:smccc_trng");
// MODULE_AUTHOR("Andre Przywara");
// MODULE_DESCRIPTION("Arm SMCCC TRNG firmware interface support");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
