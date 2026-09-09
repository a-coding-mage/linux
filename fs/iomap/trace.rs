// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2019 Christoph Hellwig
 */

// The C source includes <linux/iomap.h> before the trace definitions.
// The declarations supplied by that header are external dependencies.

/*
 * This is defined before including trace.h so that the trace-point
 * implementations are emitted rather than merely declared.
 *
 * The trace.h contents are external to this isolated translation unit and
 * are intentionally not reproduced here.
 */
// CREATE_TRACE_POINTS


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
