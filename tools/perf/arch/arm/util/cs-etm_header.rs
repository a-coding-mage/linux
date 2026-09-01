// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright(C) 2015 Linaro Limited. All rights reserved.
 * Author: Mathieu Poirier <mathieu.poirier@linaro.org>
 */

// Header guard from C source: INCLUDE__PERF_CS_ETM_H__

unsafe extern "C" {
    pub fn cs_etm_record_init(err: *mut libc::c_int) -> *mut auxtrace_record;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
