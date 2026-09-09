/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: _ASM_GENERIC_PERCPU_TYPES_H_

// __percpu_qual is the qualifier for the percpu named address space.
//
// Most architectures use generic named address space for percpu variables but
// some architectures define percpu variables in different named address space.
// E.g. on x86, percpu variable may be declared as being relative to the %fs or
// %gs segments using __seg_fs or __seg_gs named address space qualifier.
//
// In the C source, __percpu_qual is defined as empty when not already defined.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
