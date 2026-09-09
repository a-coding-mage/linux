/* SPDX-License-Identifier: GPL-2.0 */

/*
 * C header guard: ___ASM_SPARC_CPUDATA_H
 *
 * The declarations below are intended for non-assembler consumers.
 * Dependencies supplied by the surrounding translation unit:
 *   linux/threads.h
 *   linux/percpu.h
 */

extern "C" {
    pub static cpuinfo_op: seq_operations;
}

/*
 * On SPARC64 (__sparc__ && __arch64__), this header includes
 * <asm/cpudata_64.h>; otherwise it includes <asm/cpudata_32.h>.
 * The corresponding declarations are supplied by those dependencies.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
