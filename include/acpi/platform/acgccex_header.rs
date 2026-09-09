/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/******************************************************************************
 *
 * Name: acgccex.h - Extra GCC specific defines, etc.
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

/*
 * Some versions of gcc implement strchr() with a buggy macro. So,
 * undef it here. Prevents error messages of this form (usually from the
 * file getopt.c):
 *
 * error: logical '&&' with non-zero constant will always evaluate as true
 *
 * C preprocessor intent: if strchr is defined as a macro, undefine it.
 * Rust has no equivalent local preprocessor macro to remove.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
