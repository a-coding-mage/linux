/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Kernel interface for the RISCV arch_random_* functions
 *
 * Copyright (c) 2023 Rivos Inc.
 *
 */

// Dependencies supplied by the corresponding architecture headers:
// asm/csr.h and asm/processor.h

pub const SEED_RETRY_LOOPS: u32 = 100;

pub unsafe fn csr_seed_long(v: *mut ::core::ffi::c_ulong) -> bool {
    let mut retry: u32 = SEED_RETRY_LOOPS;
    let mut valid_seeds: u32 = 0;
    let needed_seeds: isize =
        ::core::mem::size_of::<::core::ffi::c_long>() as isize
            / ::core::mem::size_of::<u16>() as isize;
    let entropy = v as *mut u16;

    loop {
        /*
         * The SEED CSR must be accessed with a read-write instruction.
         */
        let csr_seed: ::core::ffi::c_ulong = csr_swap(CSR_SEED, 0);
        let opst = csr_seed & SEED_OPST_MASK;

        match opst {
            SEED_OPST_ES16 => {
                *entropy.add(valid_seeds as usize) =
                    (csr_seed & SEED_ENTROPY_MASK) as u16;
                valid_seeds += 1;
                if valid_seeds as isize == needed_seeds {
                    return true;
                }
            }

            SEED_OPST_DEAD => {
                pr_err_once("archrandom: Unrecoverable error\n");
                return false;
            }

            SEED_OPST_BIST | SEED_OPST_WAIT => {
                cpu_relax();
                continue;
            }

            _ => {
                cpu_relax();
                continue;
            }
        }

        retry -= 1;
        if retry == 0 {
            break;
        }
    }

    false
}

pub unsafe fn arch_get_random_longs(
    _v: *mut ::core::ffi::c_ulong,
    _max_longs: usize,
) -> usize {
    0
}

pub unsafe fn arch_get_random_seed_longs(
    v: *mut ::core::ffi::c_ulong,
    max_longs: usize,
) -> usize {
    if max_longs == 0 {
        return 0;
    }

    /*
     * If Zkr is supported and csr_seed_long succeeds, we return one long
     * worth of entropy.
     */
    if riscv_has_extension_likely(RISCV_ISA_EXT_ZKR) && csr_seed_long(v) {
        return 1;
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
