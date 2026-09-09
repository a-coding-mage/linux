/*
 * ladder.c - the residency ladder algorithm
 *
 *  Copyright (C) 2001, 2002 Andy Grover <andrew.grover@intel.com>
 *  Copyright (C) 2001, 2002 Paul Diefenbaugh <paul.s.diefenbaugh@intel.com>
 *  Copyright (C) 2004, 2005 Dominik Brodowski <linux@brodo.de>
 *
 * (C) 2006-2007 Venkatesh Pallipadi <venkatesh.pallipadi@intel.com>
 *               Shaohua Li <shaohua.li@intel.com>
 *               Adam Belay <abelay@novell.com>
 *
 * This code is licenced under the GPL.
 */

const PROMOTION_COUNT: u32 = 4;
const DEMOTION_COUNT: u32 = 1;

#[repr(C)]
struct LadderDeviceState {
    threshold: LadderThreshold,
    stats: LadderStats,
}

#[repr(C)]
struct LadderThreshold {
    promotion_count: u32,
    demotion_count: u32,
    promotion_time_ns: u64,
    demotion_time_ns: u64,
}

#[repr(C)]
struct LadderStats {
    promotion_count: i32,
    demotion_count: i32,
}

#[repr(C)]
struct LadderDevice {
    states: [LadderDeviceState; CPUIDLE_STATE_MAX],
}

static mut LADDER_DEVICES: [LadderDevice; NR_CPUS] = unsafe { core::mem::zeroed() };

#[inline]
unsafe fn ladder_do_selection(
    dev: *mut cpuidle_device,
    ldev: *mut LadderDevice,
    old_idx: i32,
    new_idx: i32,
) {
    (*ldev).states[old_idx as usize].stats.promotion_count = 0;
    (*ldev).states[old_idx as usize].stats.demotion_count = 0;
    (*dev).last_state_idx = new_idx;
}

unsafe fn ladder_select_state(
    drv: *mut cpuidle_driver,
    dev: *mut cpuidle_device,
    _dummy: *mut bool,
) -> i32 {
    let ldev = &mut LADDER_DEVICES[(*dev).cpu as usize] as *mut LadderDevice;
    let last_idx = (*dev).last_state_idx;
    let first_idx = if (*drv).states[0].flags & CPUIDLE_FLAG_POLLING != 0 {
        1
    } else {
        0
    };
    let latency_req = cpuidle_governor_latency_req((*dev).cpu);
    let last_residency: i64;

    if latency_req == 0 {
        ladder_do_selection(dev, ldev, last_idx, 0);
        return 0;
    }

    let last_state = &mut (*ldev).states[last_idx as usize];
    last_residency = (*dev).last_residency_ns
        - (*drv).states[last_idx as usize].exit_latency_ns;

    if last_idx < (*drv).state_count - 1
        && !(*dev).states_usage[(last_idx + 1) as usize].disable
        && last_residency > last_state.threshold.promotion_time_ns as i64
        && (*drv).states[(last_idx + 1) as usize].exit_latency_ns <= latency_req
    {
        last_state.stats.promotion_count += 1;
        last_state.stats.demotion_count = 0;
        if last_state.stats.promotion_count >= last_state.threshold.promotion_count as i32 {
            ladder_do_selection(dev, ldev, last_idx, last_idx + 1);
            return last_idx + 1;
        }
    }

    if last_idx > first_idx
        && ((*dev).states_usage[last_idx as usize].disable
            || (*drv).states[last_idx as usize].exit_latency_ns > latency_req)
    {
        let mut i = last_idx - 1;
        while i > first_idx {
            if (*drv).states[i as usize].exit_latency_ns <= latency_req {
                break;
            }
            i -= 1;
        }
        ladder_do_selection(dev, ldev, last_idx, i);
        return i;
    }

    if last_idx > first_idx && last_residency < last_state.threshold.demotion_time_ns as i64 {
        last_state.stats.demotion_count += 1;
        last_state.stats.promotion_count = 0;
        if last_state.stats.demotion_count >= last_state.threshold.demotion_count as i32 {
            ladder_do_selection(dev, ldev, last_idx, last_idx - 1);
            return last_idx - 1;
        }
    }

    last_idx
}

unsafe fn ladder_enable_device(
    drv: *mut cpuidle_driver,
    dev: *mut cpuidle_device,
) -> i32 {
    let first_idx = if (*drv).states[0].flags & CPUIDLE_FLAG_POLLING != 0 {
        1
    } else {
        0
    };
    let ldev = &mut LADDER_DEVICES[(*dev).cpu as usize] as *mut LadderDevice;

    (*dev).last_state_idx = first_idx;

    let mut i = first_idx;
    while i < (*drv).state_count {
        let state = &(*drv).states[i as usize];
        let lstate = &mut (*ldev).states[i as usize];

        lstate.stats.promotion_count = 0;
        lstate.stats.demotion_count = 0;
        lstate.threshold.promotion_count = PROMOTION_COUNT;
        lstate.threshold.demotion_count = DEMOTION_COUNT;

        if i < (*drv).state_count - 1 {
            lstate.threshold.promotion_time_ns = state.exit_latency_ns as u64;
        }
        if i > first_idx {
            lstate.threshold.demotion_time_ns = state.exit_latency_ns as u64;
        }
        i += 1;
    }

    0
}

unsafe fn ladder_reflect(dev: *mut cpuidle_device, index: i32) {
    if index > 0 {
        (*dev).last_state_idx = index;
    }
}

static mut LADDER_GOVERNOR: cpuidle_governor = cpuidle_governor {
    name: "ladder",
    rating: 10,
    enable: ladder_enable_device,
    select: ladder_select_state,
    reflect: ladder_reflect,
};

unsafe fn init_ladder() -> i32 {
    if !tick_nohz_enabled {
        LADDER_GOVERNOR.rating = 25;
    }

    cpuidle_register_governor(&mut LADDER_GOVERNOR)
}

// Equivalent of: postcore_initcall(init_ladder);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
