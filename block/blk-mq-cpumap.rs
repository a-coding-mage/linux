// SPDX-License-Identifier: GPL-2.0
/*
 * CPU <-> hardware queue mapping helpers
 *
 * Copyright (C) 2013-2014 Jens Axboe
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

use core::ffi::c_int;

#[repr(C)]
pub struct cpumask {
    _private: [u64; 0],
}

#[repr(C)]
pub struct blk_mq_queue_map {
    pub nr_queues: u32,
    pub queue_offset: u32,
    pub mq_map: *mut u32,
}

#[repr(C)]
pub struct device_bus {
    pub irq_get_affinity: Option<unsafe extern "C" fn(*mut device, u32) -> *const cpumask>,
}

#[repr(C)]
pub struct device {
    pub bus: *mut device_bus,
}

extern "C" {
    static cpu_possible_mask: *const cpumask;
    static cpu_online_mask: *const cpumask;

    fn cpumask_weight(mask: *const cpumask) -> u32;
    fn min_not_zero(a: u32, b: u32) -> u32;
    fn group_cpus_evenly(nr_cpu_ids: u32, num_masks: *mut u32) -> *const cpumask;
    fn cpumask_next(cpu: c_int, mask: *const cpumask) -> c_int;
    fn num_possible_cpus() -> c_int;
    fn cpu_to_node(cpu: c_int) -> c_int;
    fn kfree(ptr: *const cpumask);
}

const NUMA_NO_NODE: c_int = -1;

unsafe fn blk_mq_num_queues(mask: *const cpumask, max_queues: u32) -> u32 {
    let num = cpumask_weight(mask);
    min_not_zero(num, max_queues)
}

/**
 * blk_mq_num_possible_queues - Calc nr of queues for multiqueue devices
 * @max_queues: The maximum number of queues the hardware/driver supports.
 *              If max_queues is 0, the argument is ignored.
 *
 * Calculates the number of queues to be used for a multiqueue device based on
 * the number of possible CPUs.
 */
#[no_mangle]
pub unsafe extern "C" fn blk_mq_num_possible_queues(max_queues: u32) -> u32 {
    blk_mq_num_queues(cpu_possible_mask, max_queues)
}

/**
 * blk_mq_num_online_queues - Calc nr of queues for multiqueue devices
 * @max_queues: The maximum number of queues the hardware/driver supports.
 *              If max_queues is 0, the argument is ignored.
 *
 * Calculates the number of queues to be used for a multiqueue device based on
 * the number of online CPUs.
 */
#[no_mangle]
pub unsafe extern "C" fn blk_mq_num_online_queues(max_queues: u32) -> u32 {
    blk_mq_num_queues(cpu_online_mask, max_queues)
}

#[no_mangle]
pub unsafe extern "C" fn blk_mq_map_queues(qmap: *mut blk_mq_queue_map) {
    let mut nr_masks = 0u32;
    let masks = group_cpus_evenly((*qmap).nr_queues, &mut nr_masks);
    if masks.is_null() {
        let mut cpu = cpumask_next(-1, cpu_possible_mask);
        while cpu < num_possible_cpus() {
            *(*qmap).mq_map.add(cpu as usize) = (*qmap).queue_offset;
            cpu = cpumask_next(cpu, cpu_possible_mask);
        }
        return;
    }

    let mut queue = 0u32;
    while queue < (*qmap).nr_queues {
        let mask = masks.add((queue % nr_masks) as usize);
        let mut cpu = cpumask_next(-1, mask);
        while cpu < num_possible_cpus() {
            *(*qmap).mq_map.add(cpu as usize) = (*qmap).queue_offset + queue;
            cpu = cpumask_next(cpu, mask);
        }
        queue += 1;
    }
    kfree(masks);
}

/**
 * blk_mq_hw_queue_to_node - Look up the memory node for a hardware queue index
 * @qmap: CPU to hardware queue map.
 * @index: hardware queue index.
 *
 * We have no quick way of doing reverse lookups. This is only used at queue
 * init time, so runtime isn't important.
 */
#[no_mangle]
pub unsafe extern "C" fn blk_mq_hw_queue_to_node(qmap: *mut blk_mq_queue_map, index: u32) -> c_int {
    let mut i = cpumask_next(-1, cpu_possible_mask);
    while i < num_possible_cpus() {
        if index == *(*qmap).mq_map.add(i as usize) {
            return cpu_to_node(i);
        }
        i = cpumask_next(i, cpu_possible_mask);
    }
    NUMA_NO_NODE
}

/**
 * blk_mq_map_hw_queues - Create CPU to hardware queue mapping
 * @qmap: CPU to hardware queue map
 * @dev: The device to map queues
 * @offset: Queue offset to use for the device
 *
 * Create a CPU to hardware queue mapping in @qmap. The struct bus_type
 * irq_get_affinity callback will be used to retrieve the affinity.
 */
#[no_mangle]
pub unsafe extern "C" fn blk_mq_map_hw_queues(
    qmap: *mut blk_mq_queue_map,
    dev: *mut device,
    offset: u32,
) {
    if (*(*dev).bus).irq_get_affinity.is_none() {
        blk_mq_map_queues(qmap);
        return;
    }

    let get_affinity = (*(*dev).bus).irq_get_affinity.unwrap();
    let mut queue = 0u32;
    while queue < (*qmap).nr_queues {
        let mask = get_affinity(dev, queue + offset);
        if mask.is_null() {
            blk_mq_map_queues(qmap);
            return;
        }
        let mut cpu = cpumask_next(-1, mask);
        while cpu < num_possible_cpus() {
            *(*qmap).mq_map.add(cpu as usize) = (*qmap).queue_offset + queue;
            cpu = cpumask_next(cpu, mask);
        }
        queue += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
