// SPDX-License-Identifier: GPL-2.0
//
// Original C header guard: _EVSEL_H
// Depends on external definition of `struct evsel`.

unsafe extern "C" {
    pub fn evsel__sys_has_perf_metrics(evsel: *const evsel) -> bool;
}
