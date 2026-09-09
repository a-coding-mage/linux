/*
 * Copyright 2018 Advanced Micro Devices, Inc.
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
 * Authors: AMD
 *
 */

/**
 * This file defines external dependencies of Display Core.
 */

macro_rules! EVENT_LOG_AUX_REQ {
    ($ddc:expr, $type:expr, $action:expr, $address:expr, $len:expr, $data:expr) => {{
        let _ = (&$ddc, &$type, &$action, &$address, &$len, &$data);
    }};
}

macro_rules! EVENT_LOG_AUX_REP {
    ($ddc:expr, $type:expr, $replyStatus:expr, $len:expr, $data:expr) => {{
        let _ = (&$ddc, &$type, &$replyStatus, &$len, &$data);
    }};
}

macro_rules! EVENT_LOG_CUST_MSG {
    ($tag:expr, $a:expr $(, $rest:expr)*) => {{
        let _ = (&$tag, &$a);
        $(let _ = &$rest;)*
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
