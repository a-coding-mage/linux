/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_char;

/**
 * struct perf_cpu - wrapper around a CPU number.
 * @cpu: CPU number, -1 for the "any CPU"/dummy value.
 *
 * int16_t limits this to 32767 CPUs.  Widening to int requires a libperf
 * ABI bump -- see tools/lib/perf/TODO for the full scope.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_cpu {
    pub cpu: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_cache {
    pub cache_lvl: i32,
    pub cache: i32,
}

pub enum perf_cpu_map {}

unsafe extern "C" {
    /**
     * perf_cpu_map__new_any_cpu - a map with a singular "any CPU"/dummy -1 value.
     */
    pub fn perf_cpu_map__new_any_cpu() -> *mut perf_cpu_map;

    /**
     * perf_cpu_map__new_online_cpus - a map read from
     *                                 /sys/devices/system/cpu/online if
     *                                 available. If reading wasn't possible a map
     *                                 is created using the online processors
     *                                 assuming the first 'n' processors are all
     *                                 online.
     */
    pub fn perf_cpu_map__new_online_cpus() -> *mut perf_cpu_map;

    /**
     * perf_cpu_map__new - create a map from the given cpu_list such as "0-7". If no
     *                     cpu_list argument is provided then
     *                     perf_cpu_map__new_online_cpus is returned.
     */
    pub fn perf_cpu_map__new(cpu_list: *const c_char) -> *mut perf_cpu_map;

    /** perf_cpu_map__new_int - create a map with the one given cpu. */
    pub fn perf_cpu_map__new_int(cpu: i32) -> *mut perf_cpu_map;
    pub fn perf_cpu_map__get(map: *mut perf_cpu_map) -> *mut perf_cpu_map;
    pub fn perf_cpu_map__merge(orig: *mut *mut perf_cpu_map, other: *mut perf_cpu_map) -> i32;
    pub fn perf_cpu_map__intersect(
        orig: *mut perf_cpu_map,
        other: *mut perf_cpu_map,
    ) -> *mut perf_cpu_map;
    pub fn perf_cpu_map__put(map: *mut perf_cpu_map);

    /**
     * perf_cpu_map__cpu - get the CPU value at the given index. Returns -1 if index
     *                     is invalid.
     */
    pub fn perf_cpu_map__cpu(cpus: *const perf_cpu_map, idx: u32) -> perf_cpu;

    /**
     * perf_cpu_map__nr - for an empty map returns 1, as perf_cpu_map__cpu returns a
     *                    cpu of -1 for an invalid index, this makes an empty map
     *                    look like it contains the "any CPU"/dummy value. Otherwise
     *                    the result is the number CPUs in the map plus one if the
     *                    "any CPU"/dummy value is present.
     */
    pub fn perf_cpu_map__nr(cpus: *const perf_cpu_map) -> u32;

    /**
     * perf_cpu_map__has_any_cpu_or_is_empty - is map either empty or has the "any CPU"/dummy value.
     */
    pub fn perf_cpu_map__has_any_cpu_or_is_empty(map: *const perf_cpu_map) -> bool;

    /**
     * perf_cpu_map__is_any_cpu_or_is_empty - is map either empty or the "any CPU"/dummy value.
     */
    pub fn perf_cpu_map__is_any_cpu_or_is_empty(map: *const perf_cpu_map) -> bool;

    /**
     * perf_cpu_map__is_empty - does the map contain no values and it doesn't
     *                          contain the special "any CPU"/dummy value.
     */
    pub fn perf_cpu_map__is_empty(map: *const perf_cpu_map) -> bool;

    /**
     * perf_cpu_map__min - the minimum CPU value or -1 if empty or just the "any CPU"/dummy value.
     */
    pub fn perf_cpu_map__min(map: *const perf_cpu_map) -> perf_cpu;

    /**
     * perf_cpu_map__max - the maximum CPU value or -1 if empty or just the "any CPU"/dummy value.
     */
    pub fn perf_cpu_map__max(map: *const perf_cpu_map) -> perf_cpu;
    pub fn perf_cpu_map__has(map: *const perf_cpu_map, cpu: perf_cpu) -> bool;
    pub fn perf_cpu_map__equal(lhs: *const perf_cpu_map, rhs: *const perf_cpu_map) -> bool;

    /**
     * perf_cpu_map__any_cpu - Does the map contain the "any CPU"/dummy -1 value?
     */
    pub fn perf_cpu_map__has_any_cpu(map: *const perf_cpu_map) -> bool;
}

#[macro_export]
macro_rules! perf_cpu_map__for_each_cpu {
    ($cpu:ident, $idx:ident, $cpus:expr, $body:block) => {{
        $idx = 0;
        $cpu = unsafe { perf_cpu_map__cpu($cpus, $idx) };
        while $idx < unsafe { perf_cpu_map__nr($cpus) } {
            $body
            $idx += 1;
            $cpu = unsafe { perf_cpu_map__cpu($cpus, $idx) };
        }
    }};
}

#[macro_export]
macro_rules! perf_cpu_map__for_each_cpu_skip_any {
    ($_cpu:ident, $idx:ident, $cpus:expr, $body:block) => {{
        $idx = 0;
        $_cpu = unsafe { perf_cpu_map__cpu($cpus, $idx) };
        while $idx < unsafe { perf_cpu_map__nr($cpus) } {
            if $_cpu.cpu != -1 {
                $body
            }
            $idx += 1;
            $_cpu = unsafe { perf_cpu_map__cpu($cpus, $idx) };
        }
    }};
}

#[macro_export]
macro_rules! perf_cpu_map__for_each_idx {
    ($idx:ident, $cpus:expr, $body:block) => {{
        $idx = 0;
        while $idx < unsafe { perf_cpu_map__nr($cpus) } {
            $body
            $idx += 1;
        }
    }};
}
