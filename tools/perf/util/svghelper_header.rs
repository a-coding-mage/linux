/* SPDX-License-Identifier: GPL-2.0 */

// C dependency intent: <linux/types.h> provides u64.

pub type u64 = crate::linux::types::u64;

#[repr(C)]
pub struct perf_env {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn open_svg(filename: *const ::std::os::raw::c_char, cpus: ::std::os::raw::c_int, rows: ::std::os::raw::c_int, start: u64, end: u64);
    pub fn svg_ubox(Yslot: ::std::os::raw::c_int, start: u64, end: u64, height: ::std::os::raw::c_double, type_: *const ::std::os::raw::c_char, fd: ::std::os::raw::c_int, err: ::std::os::raw::c_int, merges: ::std::os::raw::c_int);
    pub fn svg_lbox(Yslot: ::std::os::raw::c_int, start: u64, end: u64, height: ::std::os::raw::c_double, type_: *const ::std::os::raw::c_char, fd: ::std::os::raw::c_int, err: ::std::os::raw::c_int, merges: ::std::os::raw::c_int);
    pub fn svg_fbox(Yslot: ::std::os::raw::c_int, start: u64, end: u64, height: ::std::os::raw::c_double, type_: *const ::std::os::raw::c_char, fd: ::std::os::raw::c_int, err: ::std::os::raw::c_int, merges: ::std::os::raw::c_int);
    pub fn svg_box(Yslot: ::std::os::raw::c_int, start: u64, end: u64, type_: *const ::std::os::raw::c_char);
    pub fn svg_blocked(Yslot: ::std::os::raw::c_int, cpu: ::std::os::raw::c_int, start: u64, end: u64, backtrace: *const ::std::os::raw::c_char);
    pub fn svg_running(Yslot: ::std::os::raw::c_int, cpu: ::std::os::raw::c_int, start: u64, end: u64, backtrace: *const ::std::os::raw::c_char);
    pub fn svg_waiting(Yslot: ::std::os::raw::c_int, cpu: ::std::os::raw::c_int, start: u64, end: u64, backtrace: *const ::std::os::raw::c_char);
    pub fn svg_cpu_box(cpu: ::std::os::raw::c_int, max_frequency: u64, turbo_frequency: u64);

    pub fn svg_process(cpu: ::std::os::raw::c_int, start: u64, end: u64, pid: ::std::os::raw::c_int, name: *const ::std::os::raw::c_char, backtrace: *const ::std::os::raw::c_char);
    pub fn svg_cstate(cpu: ::std::os::raw::c_int, start: u64, end: u64, type_: ::std::os::raw::c_int);
    pub fn svg_pstate(cpu: ::std::os::raw::c_int, start: u64, end: u64, freq: u64);

    pub fn svg_time_grid(min_thickness: ::std::os::raw::c_double);
    pub fn svg_io_legenda();
    pub fn svg_legenda();
    pub fn svg_wakeline(start: u64, row1: ::std::os::raw::c_int, row2: ::std::os::raw::c_int, backtrace: *const ::std::os::raw::c_char);
    pub fn svg_partial_wakeline(start: u64, row1: ::std::os::raw::c_int, desc1: *mut ::std::os::raw::c_char, row2: ::std::os::raw::c_int, desc2: *mut ::std::os::raw::c_char, backtrace: *const ::std::os::raw::c_char);
    pub fn svg_interrupt(start: u64, row: ::std::os::raw::c_int, backtrace: *const ::std::os::raw::c_char);
    pub fn svg_text(Yslot: ::std::os::raw::c_int, start: u64, text: *const ::std::os::raw::c_char);
    pub fn svg_close();
    pub fn svg_build_topology_map(env: *mut perf_env) -> ::std::os::raw::c_int;

    pub static mut svg_page_width: ::std::os::raw::c_int;
    pub static mut svg_highlight: u64;
    pub static mut svg_highlight_name: *const ::std::os::raw::c_char;
}
