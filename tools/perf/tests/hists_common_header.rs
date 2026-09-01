// SPDX-License-Identifier: GPL-2.0

pub enum machine {}
pub enum machines {}
pub enum hists {}

pub const FAKE_PID_PERF1: i32 = 100;
pub const FAKE_PID_PERF2: i32 = 200;
pub const FAKE_PID_BASH: i32 = 300;

pub const FAKE_MAP_PERF: i32 = 0x400000;
pub const FAKE_MAP_BASH: i32 = 0x400000;
pub const FAKE_MAP_LIBC: i32 = 0x500000;
pub const FAKE_MAP_KERNEL: i32 = 0xf00000;
pub const FAKE_MAP_LENGTH: i32 = 0x100000;

pub const FAKE_SYM_OFFSET1: i32 = 700;
pub const FAKE_SYM_OFFSET2: i32 = 800;
pub const FAKE_SYM_OFFSET3: i32 = 900;
pub const FAKE_SYM_LENGTH: i32 = 100;

pub const FAKE_IP_PERF_MAIN: i32 = FAKE_MAP_PERF + FAKE_SYM_OFFSET1;
pub const FAKE_IP_PERF_RUN_COMMAND: i32 = FAKE_MAP_PERF + FAKE_SYM_OFFSET2;
pub const FAKE_IP_PERF_CMD_RECORD: i32 = FAKE_MAP_PERF + FAKE_SYM_OFFSET3;
pub const FAKE_IP_BASH_MAIN: i32 = FAKE_MAP_BASH + FAKE_SYM_OFFSET1;
pub const FAKE_IP_BASH_XMALLOC: i32 = FAKE_MAP_BASH + FAKE_SYM_OFFSET2;
pub const FAKE_IP_BASH_XFREE: i32 = FAKE_MAP_BASH + FAKE_SYM_OFFSET3;
pub const FAKE_IP_LIBC_MALLOC: i32 = FAKE_MAP_LIBC + FAKE_SYM_OFFSET1;
pub const FAKE_IP_LIBC_FREE: i32 = FAKE_MAP_LIBC + FAKE_SYM_OFFSET2;
pub const FAKE_IP_LIBC_REALLOC: i32 = FAKE_MAP_LIBC + FAKE_SYM_OFFSET3;
pub const FAKE_IP_KERNEL_SCHEDULE: i32 = FAKE_MAP_KERNEL + FAKE_SYM_OFFSET1;
pub const FAKE_IP_KERNEL_PAGE_FAULT: i32 = FAKE_MAP_KERNEL + FAKE_SYM_OFFSET2;
pub const FAKE_IP_KERNEL_SYS_PERF_EVENT_OPEN: i32 = FAKE_MAP_KERNEL + FAKE_SYM_OFFSET3;

/*
 * The setup_fake_machine() provides a test environment which consists
 * of 3 processes that have 3 mappings and in turn, have 3 symbols
 * respectively.  See below table:
 *
 * Command:  Pid  Shared Object               Symbol
 * .............  .............  ...................
 *    perf:  100           perf  main
 *    perf:  100           perf  run_command
 *    perf:  100           perf  cmd_record
 *    perf:  100           libc  malloc
 *    perf:  100           libc  free
 *    perf:  100           libc  realloc
 *    perf:  100       [kernel]  schedule
 *    perf:  100       [kernel]  page_fault
 *    perf:  100       [kernel]  sys_perf_event_open
 *    perf:  200           perf  main
 *    perf:  200           perf  run_command
 *    perf:  200           perf  cmd_record
 *    perf:  200           libc  malloc
 *    perf:  200           libc  free
 *    perf:  200           libc  realloc
 *    perf:  200       [kernel]  schedule
 *    perf:  200       [kernel]  page_fault
 *    perf:  200       [kernel]  sys_perf_event_open
 *    bash:  300           bash  main
 *    bash:  300           bash  xmalloc
 *    bash:  300           bash  xfree
 *    bash:  300           libc  malloc
 *    bash:  300           libc  free
 *    bash:  300           libc  realloc
 *    bash:  300       [kernel]  schedule
 *    bash:  300       [kernel]  page_fault
 *    bash:  300       [kernel]  sys_perf_event_open
 */
unsafe extern "C" {
    pub fn setup_fake_machine(machines: *mut machines) -> *mut machine;

    pub fn print_hists_in(hists: *mut hists);
    pub fn print_hists_out(hists: *mut hists);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
