// SPDX-License-Identifier: MIT
/* Copyright Red Hat Inc 2010.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Author : Dave Airlie <airlied@redhat.com>
 */

// Dependencies supplied by the surrounding repository:
// <drm/amdgpu_drm.h>
// "amdgpu_cs.h"
// "amdgpu.h"

// C translation intent: define CREATE_TRACE_POINTS before including
// "amdgpu_trace.h" so that the trace points are defined in this translation
// unit. The included trace declarations are supplied externally.
#[allow(dead_code)]
const CREATE_TRACE_POINTS: bool = true;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
