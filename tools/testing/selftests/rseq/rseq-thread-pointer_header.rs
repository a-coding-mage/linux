/* SPDX-License-Identifier: LGPL-2.1-only OR MIT */
/*
 * rseq-thread-pointer.h
 *
 * (C) Copyright 2021 - Mathieu Desnoyers <mathieu.desnoyers@efficios.com>
 */

/*
 * C header guard omitted in Rust.
 *
 * Original dependency selection:
 * - if defined(__x86_64__) || defined(__i386__): include "rseq-x86-thread-pointer.h"
 * - elif defined(__PPC__): include "rseq-ppc-thread-pointer.h"
 * - elif defined(__or1k__): include "rseq-or1k-thread-pointer.h"
 * - else: include "rseq-generic-thread-pointer.h"
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
