/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * rseq-bits-template.h
 *
 * (C) Copyright 2016-2022 - Mathieu Desnoyers <mathieu.desnoyers@efficios.com>
 */

/*
 * This header is a C preprocessor template selected by one of:
 * RSEQ_TEMPLATE_CPU_ID, RSEQ_TEMPLATE_MM_CID, or RSEQ_TEMPLATE_CPU_ID_NONE,
 * combined with RSEQ_TEMPLATE_MO_RELEASE or RSEQ_TEMPLATE_MO_RELAXED.
 *
 * The C template defines RSEQ_TEMPLATE_CPU_ID_OFFSET,
 * RSEQ_TEMPLATE_CPU_ID_FIELD, RSEQ_TEMPLATE_SUFFIX, and
 * RSEQ_TEMPLATE_IDENTIFIER(x). Rust has no direct token-pasting equivalent
 * for this file-local preprocessor machinery without the including context, so
 * the conditional mapping is preserved below.
 */

/*
 * If RSEQ_TEMPLATE_CPU_ID is defined:
 *   RSEQ_TEMPLATE_CPU_ID_OFFSET = RSEQ_CPU_ID_OFFSET
 *   RSEQ_TEMPLATE_CPU_ID_FIELD = cpu_id
 *   if RSEQ_TEMPLATE_MO_RELEASE is defined:
 *     RSEQ_TEMPLATE_SUFFIX = _release_cpu_id
 *   else if RSEQ_TEMPLATE_MO_RELAXED is defined:
 *     RSEQ_TEMPLATE_SUFFIX = _relaxed_cpu_id
 *   else:
 *     error: "Never use <rseq-bits-template.h> directly; include <rseq.h> instead."
 *
 * Else if RSEQ_TEMPLATE_MM_CID is defined:
 *   RSEQ_TEMPLATE_CPU_ID_OFFSET = RSEQ_MM_CID_OFFSET
 *   RSEQ_TEMPLATE_CPU_ID_FIELD = mm_cid
 *   if RSEQ_TEMPLATE_MO_RELEASE is defined:
 *     RSEQ_TEMPLATE_SUFFIX = _release_mm_cid
 *   else if RSEQ_TEMPLATE_MO_RELAXED is defined:
 *     RSEQ_TEMPLATE_SUFFIX = _relaxed_mm_cid
 *   else:
 *     error: "Never use <rseq-bits-template.h> directly; include <rseq.h> instead."
 *
 * Else if RSEQ_TEMPLATE_CPU_ID_NONE is defined:
 *   if RSEQ_TEMPLATE_MO_RELEASE is defined:
 *     RSEQ_TEMPLATE_SUFFIX = _release
 *   else if RSEQ_TEMPLATE_MO_RELAXED is defined:
 *     RSEQ_TEMPLATE_SUFFIX = _relaxed
 *   else:
 *     error: "Never use <rseq-bits-template.h> directly; include <rseq.h> instead."
 *
 * Else:
 *   error: "Never use <rseq-bits-template.h> directly; include <rseq.h> instead."
 *
 * RSEQ_TEMPLATE_IDENTIFIER(x) = RSEQ_COMBINE_TOKENS(x, RSEQ_TEMPLATE_SUFFIX)
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
