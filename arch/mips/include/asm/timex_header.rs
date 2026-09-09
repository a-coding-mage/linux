/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1998, 1999, 2003 by Ralf Baechle
 * Copyright (C) 2014 by Maciej W. Rozycki
 */

// The original header is active only when __KERNEL__ is defined.  Its include
// dependencies and configuration-provided symbols are supplied externally.

/// Standard cycle-counter type.
pub type cycles_t = core::ffi::c_uint;

/*
 * On R4000/R4400 an erratum exists such that if the cycle counter is read in
 * the exact moment that it is matching the compare register, no interrupt
 * will be generated.
 */
#[inline]
pub unsafe fn can_use_mips_counter(prid: core::ffi::c_uint) -> core::ffi::c_int {
    let comp = (prid & PRID_COMP_MASK) != PRID_COMP_LEGACY;

    // __builtin_constant_p is a compiler/build-time property.  Preserve the
    // source decision points while leaving evaluation to the Rust build's
    // configuration-provided constants.
    if !cpu_has_counter {
        return 0;
    } else if cpu_has_mips_r {
        return 1;
    } else if likely(comp) {
        return 1;
    }

    // Make sure we don't peek at cpu_data[0].options in the fast path.
    // The original statement is an empty volatile asm memory constraint;
    // the corresponding dependency-side compiler barrier is required here.
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);

    if likely(cpu_has_counter &&
              prid > (PRID_IMP_R4000 | PRID_REV_ENCODE_44(15, 15))) {
        1
    } else {
        0
    }
}

#[inline]
pub unsafe fn get_cycles() -> cycles_t {
    if can_use_mips_counter(read_c0_prid()) != 0 {
        read_c0_count()
    } else {
        0 // no usable counter
    }
}

// #define get_cycles get_cycles

/*
 * Like get_cycles - but where c0_count is not available we desperately use
 * c0_random in an attempt to get at least a little bit of entropy.
 */
#[inline]
pub unsafe fn random_get_entropy() -> core::ffi::c_ulong {
    let c0_random: core::ffi::c_uint;

    if can_use_mips_counter(read_c0_prid()) != 0 {
        return read_c0_count() as core::ffi::c_ulong;
    }

    if cpu_has_3kex {
        c0_random = (read_c0_random() >> 8) & 0x3f;
    } else {
        c0_random = read_c0_random() & 0x3f;
    }
    (random_get_entropy_fallback() << 6) | (0x3f - c0_random as core::ffi::c_ulong)
}

// #define random_get_entropy random_get_entropy

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
