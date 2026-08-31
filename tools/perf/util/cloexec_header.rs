/* SPDX-License-Identifier: GPL-2.0 */

use std::os::raw::c_ulong;

unsafe extern "C" {
    pub fn perf_event_open_cloexec_flag() -> c_ulong;
}
