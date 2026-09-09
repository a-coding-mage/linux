/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/blkdev.h, asm/local.h, NR_STAT_GROUPS, TYPEOF_UNQUAL,
// preempt_disable, preempt_enable, per_cpu_ptr, smp_processor_id,
// for_each_possible_cpu, bdev_is_partition, bdev_whole, __this_cpu_add,
// local_dec, local_inc, and local_read.

#[repr(C)]
pub struct disk_stats {
    pub nsecs: [u64; NR_STAT_GROUPS],
    pub sectors: [::core::ffi::c_ulong; NR_STAT_GROUPS],
    pub ios: [::core::ffi::c_ulong; NR_STAT_GROUPS],
    pub merges: [::core::ffi::c_ulong; NR_STAT_GROUPS],
    pub io_ticks: ::core::ffi::c_ulong,
    pub in_flight: [local_t; 2],
}

#[inline(always)]
pub unsafe fn part_stat_set_all(part: *mut block_device, value: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int;
    // for_each_possible_cpu(i)
    for_each_possible_cpu!(i) {
        memset(
            per_cpu_ptr((*part).bd_stats, i),
            value,
            ::core::mem::size_of::<disk_stats>(),
        );
    }
}

#[macro_export]
macro_rules! part_stat_lock {
    () => { preempt_disable() };
}

#[macro_export]
macro_rules! part_stat_unlock {
    () => { preempt_enable() };
}

#[macro_export]
macro_rules! part_stat_get_cpu {
    ($part:expr, $field:ident, $cpu:expr) => {
        (*per_cpu_ptr(($part)->bd_stats, $cpu)).$field
    };
}

#[macro_export]
macro_rules! part_stat_get {
    ($part:expr, $field:ident) => {
        part_stat_get_cpu!($part, $field, smp_processor_id())
    };
}

#[macro_export]
macro_rules! part_stat_read {
    ($part:expr, $field:ident) => {{
        let mut res = 0;
        let mut _cpu: ::core::ffi::c_uint;
        // for_each_possible_cpu(_cpu)
        for_each_possible_cpu!(_cpu) {
            res += (*per_cpu_ptr(($part)->bd_stats, _cpu)).$field;
        }
        res
    }};
}

#[macro_export]
macro_rules! part_stat_read_accum {
    ($part:expr, $field:ident) => {
        part_stat_read!($part, $field[STAT_READ])
            + part_stat_read!($part, $field[STAT_WRITE])
            + part_stat_read!($part, $field[STAT_DISCARD])
    };
}

#[macro_export]
macro_rules! __part_stat_add {
    ($part:expr, $field:ident, $addnd:expr) => {
        __this_cpu_add!(($part)->bd_stats-> $field, $addnd)
    };
}

#[macro_export]
macro_rules! part_stat_add {
    ($part:expr, $field:ident, $addnd:expr) => {{
        __part_stat_add!($part, $field, $addnd);
        if bdev_is_partition($part) {
            __part_stat_add!(bdev_whole($part), $field, $addnd);
        }
    }};
}

#[macro_export]
macro_rules! part_stat_dec { ($part:expr, $field:ident) => { part_stat_add!($part, $field, -1) }; }
#[macro_export]
macro_rules! part_stat_inc { ($part:expr, $field:ident) => { part_stat_add!($part, $field, 1) }; }
#[macro_export]
macro_rules! part_stat_sub { ($part:expr, $field:ident, $subnd:expr) => { part_stat_add!($part, $field, -$subnd) }; }

#[macro_export]
macro_rules! part_stat_local_dec { ($part:expr, $field:ident) => { local_dec!(&mut part_stat_get!($part, $field)) }; }
#[macro_export]
macro_rules! part_stat_local_inc { ($part:expr, $field:ident) => { local_inc!(&mut part_stat_get!($part, $field)) }; }
#[macro_export]
macro_rules! part_stat_local_read { ($part:expr, $field:ident) => { local_read!(&part_stat_get!($part, $field)) }; }
#[macro_export]
macro_rules! part_stat_local_read_cpu { ($part:expr, $field:ident, $cpu:expr) => { local_read!(&part_stat_get_cpu!($part, $field, $cpu)) }; }

extern "C" {
    pub fn bdev_count_inflight(part: *mut block_device) -> ::core::ffi::c_uint;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
