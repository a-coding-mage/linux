// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Data structures shared between BPF and tools.

// These constants impact code size of bperf_cgroup.bpf.c that may result in BPF
// verifier issues. They are exposed to control the size and also to disable BPF
// counters when the number of user events is too large.

// max cgroup hierarchy level: arbitrary
pub const BPERF_CGROUP__MAX_LEVELS: u32 = 10;
// max events per cgroup: arbitrary
pub const BPERF_CGROUP__MAX_EVENTS: u32 = 128;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
