// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright(C) 2015 Linaro Limited. All rights reserved.
 * Author: Mathieu Poirier <mathieu.poirier@linaro.org>
 */

// C includes translated as external dependencies:
// <string.h>
// <linux/coresight-pmu.h>
// <linux/perf_event.h>
// <linux/string.h>
// "arm-spe.h"
// "hisi-ptt.h"
// "../../../util/cpumap.h"
// "../../../util/pmu.h"
// "../../../util/cs-etm.h"
// "../../arm64/util/mem-events.h"

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_event_attr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_pmu {
    pub name: *const c_char,
    pub auxtrace: bool,
    pub selectable: bool,
    pub is_uncore: bool,
    pub perf_event_attr_init_default:
        Option<unsafe extern "C" fn(*mut perf_pmu, *mut perf_event_attr) -> c_int>,
    pub mem_events: *mut c_void,
    pub cpus: *mut perf_cpu_map,
}

unsafe extern "C" {
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strstarts(str_: *const c_char, prefix: *const c_char) -> bool;

    fn cpu_map__online() -> *mut perf_cpu_map;
    fn perf_cpu_map__intersect(
        orig: *mut perf_cpu_map,
        other: *mut perf_cpu_map,
    ) -> *mut perf_cpu_map;
    fn perf_cpu_map__put(map: *mut perf_cpu_map);

    fn cs_etm_get_default_config(pmu: *mut perf_pmu, attr: *mut perf_event_attr) -> c_int;

    #[cfg(target_arch = "aarch64")]
    fn arm_spe_pmu_default_config(pmu: *mut perf_pmu, attr: *mut perf_event_attr) -> c_int;

    #[cfg(target_arch = "aarch64")]
    static mut perf_mem_events_arm: *mut c_void;
}

const CORESIGHT_ETM_PMU_NAME: *const c_char = c"cs_etm".as_ptr();

#[cfg(target_arch = "aarch64")]
const ARM_SPE_PMU_NAME: *const c_char = c"arm_spe".as_ptr();

#[cfg(target_arch = "aarch64")]
const HISI_PTT_PMU_NAME: *const c_char = c"hisi_ptt".as_ptr();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_pmu__arch_init(pmu: *mut perf_pmu) {
    let mut intersect: *mut perf_cpu_map;
    let online: *mut perf_cpu_map = unsafe { cpu_map__online() };

    if unsafe { strcmp((*pmu).name, CORESIGHT_ETM_PMU_NAME) } == 0 {
        /* add ETM default config here */
        unsafe {
            (*pmu).auxtrace = true;
            (*pmu).selectable = true;
            (*pmu).perf_event_attr_init_default = Some(cs_etm_get_default_config);
        }
    } else {
        #[cfg(target_arch = "aarch64")]
        {
            if unsafe { strstarts((*pmu).name, ARM_SPE_PMU_NAME) } {
                unsafe {
                    (*pmu).auxtrace = true;
                    (*pmu).selectable = true;
                    (*pmu).is_uncore = false;
                    (*pmu).perf_event_attr_init_default = Some(arm_spe_pmu_default_config);
                    if strstarts((*pmu).name, c"arm_spe_".as_ptr()) {
                        (*pmu).mem_events = perf_mem_events_arm;
                    }
                }
            } else if unsafe { strstarts((*pmu).name, HISI_PTT_PMU_NAME) } {
                unsafe {
                    (*pmu).auxtrace = true;
                    (*pmu).selectable = true;
                }
            }
        }
    }
    /* Workaround some ARM PMU's failing to correctly set CPU maps for online processors. */
    intersect = unsafe { perf_cpu_map__intersect(online, (*pmu).cpus) };
    unsafe {
        perf_cpu_map__put(online);
        perf_cpu_map__put((*pmu).cpus);
        (*pmu).cpus = intersect;
    }
}
