/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Explicitly undef ARCH_DEFINE_ENCODE_FPROBE_HEADER, because loongarch does not
 * have enough number of fixed MSBs of the address of kernel objects for
 * encoding the size of data in fprobe_header. Use 2-entries encoding instead.
 *
 * The C preprocessor symbol is intentionally undefined here; there is no
 * corresponding Rust item to emit.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
