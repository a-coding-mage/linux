// SPDX-License-Identifier: GPL-2.0-only
/*
 *  (C) 2010,2011      Thomas Renninger <trenn@suse.de>, Novell Inc.
 *
 *  PCI initialization based on example code from:
 *  Andreas Herrmann <andreas.herrmann3@amd.com>
 */

// C source is compiled only for: defined(__i386__) || defined(__x86_64__)

use core::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_ulonglong, c_void};

const PCI_NON_PC0_OFFSET: c_uint = 0xb0;
const PCI_PC1_OFFSET: c_uint = 0xb4;
const PCI_PC6_OFFSET: c_uint = 0xb8;

const PCI_MONITOR_ENABLE_REG: c_uint = 0xe0;

const PCI_NON_PC0_ENABLE_BIT: c_int = 0;
const PCI_PC1_ENABLE_BIT: c_int = 1;
const PCI_PC6_ENABLE_BIT: c_int = 2;

const PCI_NBP1_STAT_OFFSET: c_uint = 0x98;
const PCI_NBP1_ACTIVE_BIT: c_int = 2;
const PCI_NBP1_ENTERED_BIT: c_int = 1;

const PCI_NBP1_CAP_OFFSET: c_uint = 0x90;
const PCI_NBP1_CAPABLE_BIT: c_int = 31;

const OVERFLOW_MS: c_ulonglong = 343597; /* 32 bit register filled at 12500 HZ
                                          * (1 tick per 80ns)
                                          */

const NON_PC0: c_uint = 0;
const PC1: c_uint = 1;
const PC6: c_uint = 2;
const NBP1: c_uint = 3;
const AMD_FAM14H_STATE_NUM: usize = 4;

const RANGE_PACKAGE: c_uint = 0;
const X86_VENDOR_AMD: c_uint = 0;
const MONITOR_NAME_LEN: usize = 64;
const CLOCK_REALTIME: c_int = 0;

#[repr(C)]
pub struct pci_access {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timespec {
    pub tv_sec: c_long,
    pub tv_nsec: c_long,
}

#[repr(C)]
pub struct cstate {
    pub name: *const c_char,
    pub desc: *const c_char,
    pub id: c_uint,
    pub range: c_uint,
    pub get_count: Option<unsafe extern "C" fn(c_uint, *mut c_ulonglong, c_uint) -> c_int>,
    pub get_count_percent: Option<unsafe extern "C" fn(c_uint, *mut c_double, c_uint) -> c_int>,
}

pub type cstate_t = cstate;

#[repr(C)]
pub struct cpuidle_monitor_flags {
    pub needs_root: c_uint,
}

#[repr(C)]
pub struct cpuidle_monitor {
    pub name: [c_char; MONITOR_NAME_LEN],
    pub name_len: c_uint,
    pub hw_states: *mut cstate_t,
    pub hw_states_num: c_uint,
    pub start: Option<unsafe extern "C" fn() -> c_int>,
    pub stop: Option<unsafe extern "C" fn() -> c_int>,
    pub do_register: Option<unsafe extern "C" fn() -> *mut cpuidle_monitor>,
    pub unregister: Option<unsafe extern "C" fn()>,
    pub flags: cpuidle_monitor_flags,
    pub overflow_s: c_ulonglong,
}

#[repr(C)]
pub struct cpupower_cpu_info_t {
    pub vendor: c_uint,
    pub family: c_uint,
}

unsafe extern "C" {
    static mut cpu_count: c_uint;
    static mut cpupower_cpu_info: cpupower_cpu_info_t;

    fn pci_read_long(dev: *mut pci_dev, pos: c_uint) -> u32;
    fn pci_write_long(dev: *mut pci_dev, pos: c_uint, data: u32) -> u32;
    fn pci_slot_func_init(acc: *mut *mut pci_access, slot: c_uint, func: c_uint) -> *mut pci_dev;
    fn pci_cleanup(acc: *mut pci_access);

    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strlen(s: *const c_char) -> usize;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    fn timespec_diff_us(start: timespec, end: timespec) -> c_ulonglong;
    fn print_overflow_err(overflow_sec: c_uint, overflow_ms: c_ulonglong);
    fn dprint(fmt: *const c_char, ...);
}

static mut AMD_FAM14H_CSTATES: [cstate_t; AMD_FAM14H_STATE_NUM] = [
    cstate_t {
        name: c"!PC0".as_ptr(),
        desc: c"Package in sleep state (PC1 or deeper)".as_ptr(),
        id: NON_PC0,
        range: RANGE_PACKAGE,
        get_count: None,
        get_count_percent: Some(fam14h_get_count_percent),
    },
    cstate_t {
        name: c"PC1".as_ptr(),
        desc: c"Processor Package C1".as_ptr(),
        id: PC1,
        range: RANGE_PACKAGE,
        get_count: None,
        get_count_percent: Some(fam14h_get_count_percent),
    },
    cstate_t {
        name: c"PC6".as_ptr(),
        desc: c"Processor Package C6".as_ptr(),
        id: PC6,
        range: RANGE_PACKAGE,
        get_count: None,
        get_count_percent: Some(fam14h_get_count_percent),
    },
    cstate_t {
        name: c"NBP1".as_ptr(),
        desc: c"North Bridge P1 boolean counter (returns 0 or 1)".as_ptr(),
        id: NBP1,
        range: RANGE_PACKAGE,
        get_count: Some(fam14h_nbp1_count),
        get_count_percent: None,
    },
];

static mut PCI_ACC: *mut pci_access = core::ptr::null_mut();
static mut AMD_FAM14H_PCI_DEV: *mut pci_dev = core::ptr::null_mut();
static mut NBP1_ENTERED: c_int = 0;

static mut START_TIME: timespec = timespec {
    tv_sec: 0,
    tv_nsec: 0,
};
static mut TIMEDIFF: c_ulonglong = 0;

// DEBUG-only C globals:
// static struct timespec dbg_time;
// static long dbg_timediff;

static mut PREVIOUS_COUNT: [*mut c_ulonglong; AMD_FAM14H_STATE_NUM] =
    [core::ptr::null_mut(); AMD_FAM14H_STATE_NUM];
static mut CURRENT_COUNT: [*mut c_ulonglong; AMD_FAM14H_STATE_NUM] =
    [core::ptr::null_mut(); AMD_FAM14H_STATE_NUM];

unsafe extern "C" fn amd_fam14h_get_pci_info(
    state: *mut cstate,
    pci_offset: *mut c_uint,
    enable_bit: *mut c_uint,
    _cpu: c_uint,
) -> c_int {
    unsafe {
        match (*state).id {
            NON_PC0 => {
                *enable_bit = PCI_NON_PC0_ENABLE_BIT as c_uint;
                *pci_offset = PCI_NON_PC0_OFFSET;
            }
            PC1 => {
                *enable_bit = PCI_PC1_ENABLE_BIT as c_uint;
                *pci_offset = PCI_PC1_OFFSET;
            }
            PC6 => {
                *enable_bit = PCI_PC6_ENABLE_BIT as c_uint;
                *pci_offset = PCI_PC6_OFFSET;
            }
            NBP1 => {
                *enable_bit = PCI_NBP1_ENTERED_BIT as c_uint;
                *pci_offset = PCI_NBP1_STAT_OFFSET;
            }
            _ => return -1,
        }
        0
    }
}

unsafe extern "C" fn amd_fam14h_init(state: *mut cstate_t, cpu: c_uint) -> c_int {
    unsafe {
        let mut enable_bit: c_uint = 0;
        let mut pci_offset: c_uint = 0;
        let ret: c_int;
        let mut val: u32;

        ret = amd_fam14h_get_pci_info(state, &mut pci_offset, &mut enable_bit, cpu);
        if ret != 0 {
            return ret;
        }

        /* NBP1 needs extra treating -> write 1 to D18F6x98 bit 1 for init */
        if (*state).id == NBP1 {
            val = pci_read_long(AMD_FAM14H_PCI_DEV, pci_offset);
            val |= 1u32 << enable_bit;
            val = pci_write_long(AMD_FAM14H_PCI_DEV, pci_offset, val);
            let _ = val;
            return ret;
        }

        /* Enable monitor */
        val = pci_read_long(AMD_FAM14H_PCI_DEV, PCI_MONITOR_ENABLE_REG);
        dprint(
            c"Init %s: read at offset: 0x%x val: %u\n".as_ptr(),
            (*state).name,
            PCI_MONITOR_ENABLE_REG,
            val as c_uint,
        );
        val |= 1u32 << enable_bit;
        pci_write_long(AMD_FAM14H_PCI_DEV, PCI_MONITOR_ENABLE_REG, val);

        dprint(
            c"Init %s: offset: 0x%x enable_bit: %d - val: %u (%u)\n".as_ptr(),
            (*state).name,
            PCI_MONITOR_ENABLE_REG,
            enable_bit as c_int,
            val as c_uint,
            cpu,
        );

        /* Set counter to zero */
        pci_write_long(AMD_FAM14H_PCI_DEV, pci_offset, 0);
        *PREVIOUS_COUNT[(*state).id as usize].add(cpu as usize) = 0;

        0
    }
}

unsafe extern "C" fn amd_fam14h_disable(state: *mut cstate_t, cpu: c_uint) -> c_int {
    unsafe {
        let mut enable_bit: c_uint = 0;
        let mut pci_offset: c_uint = 0;
        let ret: c_int;
        let mut val: u32;

        ret = amd_fam14h_get_pci_info(state, &mut pci_offset, &mut enable_bit, cpu);
        if ret != 0 {
            return ret;
        }

        val = pci_read_long(AMD_FAM14H_PCI_DEV, pci_offset);
        dprint(c"%s: offset: 0x%x %u\n".as_ptr(), (*state).name, pci_offset, val);
        if (*state).id == NBP1 {
            /* was the bit whether NBP1 got entered set? */
            NBP1_ENTERED = ((val & (1u32 << PCI_NBP1_ACTIVE_BIT))
                | (val & (1u32 << PCI_NBP1_ENTERED_BIT))) as c_int;

            dprint(
                c"NBP1 was %sentered - 0x%x - enable_bit: %d - pci_offset: 0x%x\n".as_ptr(),
                if NBP1_ENTERED != 0 {
                    c"".as_ptr()
                } else {
                    c"not ".as_ptr()
                },
                val,
                enable_bit as c_int,
                pci_offset,
            );
            return ret;
        }
        *CURRENT_COUNT[(*state).id as usize].add(cpu as usize) = val as c_ulonglong;

        dprint(
            c"%s: Current -  %llu (%u)\n".as_ptr(),
            (*state).name,
            *CURRENT_COUNT[(*state).id as usize].add(cpu as usize),
            cpu,
        );
        dprint(
            c"%s: Previous - %llu (%u)\n".as_ptr(),
            (*state).name,
            *PREVIOUS_COUNT[(*state).id as usize].add(cpu as usize),
            cpu,
        );

        val = pci_read_long(AMD_FAM14H_PCI_DEV, PCI_MONITOR_ENABLE_REG);
        val &= !(1u32 << enable_bit);
        pci_write_long(AMD_FAM14H_PCI_DEV, PCI_MONITOR_ENABLE_REG, val);

        0
    }
}

unsafe extern "C" fn fam14h_nbp1_count(
    id: c_uint,
    count: *mut c_ulonglong,
    _cpu: c_uint,
) -> c_int {
    unsafe {
        if id == NBP1 {
            if NBP1_ENTERED != 0 {
                *count = 1;
            } else {
                *count = 0;
            }
            return 0;
        }
        -1
    }
}

unsafe extern "C" fn fam14h_get_count_percent(
    id: c_uint,
    percent: *mut c_double,
    cpu: c_uint,
) -> c_int {
    unsafe {
        let diff: c_ulong;

        if id >= AMD_FAM14H_STATE_NUM as c_uint {
            return -1;
        }
        /* residency count in 80ns -> divide through 12.5 to get us residency */
        diff = (*CURRENT_COUNT[id as usize].add(cpu as usize))
            .wrapping_sub(*PREVIOUS_COUNT[id as usize].add(cpu as usize)) as c_ulong;

        if TIMEDIFF == 0 {
            *percent = 0.0;
        } else {
            *percent = 100.0 * diff as c_double / TIMEDIFF as c_double / 12.5;
        }

        dprint(
            c"Timediff: %llu - res~: %lu us - percent: %.2f %%\n".as_ptr(),
            TIMEDIFF,
            diff.wrapping_mul(10).wrapping_div(125),
            *percent,
        );

        0
    }
}

unsafe extern "C" fn amd_fam14h_start() -> c_int {
    unsafe {
        let mut num: c_int;
        let mut cpu: c_int;
        clock_gettime(CLOCK_REALTIME, &raw mut START_TIME);
        num = 0;
        while num < AMD_FAM14H_STATE_NUM as c_int {
            cpu = 0;
            while cpu < cpu_count as c_int {
                amd_fam14h_init(&mut AMD_FAM14H_CSTATES[num as usize], cpu as c_uint);
                cpu += 1;
            }
            num += 1;
        }
        // DEBUG block omitted from executable Rust; it measured counter enable time.
        0
    }
}

unsafe extern "C" fn amd_fam14h_stop() -> c_int {
    unsafe {
        let mut num: c_int;
        let mut cpu: c_int;
        let mut end_time = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };

        clock_gettime(CLOCK_REALTIME, &mut end_time);

        num = 0;
        while num < AMD_FAM14H_STATE_NUM as c_int {
            cpu = 0;
            while cpu < cpu_count as c_int {
                amd_fam14h_disable(&mut AMD_FAM14H_CSTATES[num as usize], cpu as c_uint);
                cpu += 1;
            }
            num += 1;
        }
        // DEBUG block omitted from executable Rust; it measured counter disable time.
        TIMEDIFF = timespec_diff_us(START_TIME, end_time);
        if TIMEDIFF / 1000 > OVERFLOW_MS {
            print_overflow_err((TIMEDIFF as c_uint) / 1000000, OVERFLOW_MS / 1000);
        }

        0
    }
}

unsafe extern "C" fn is_nbp1_capable() -> c_int {
    unsafe {
        let val: u32;
        val = pci_read_long(AMD_FAM14H_PCI_DEV, PCI_NBP1_CAP_OFFSET);
        (val & (1u32 << PCI_NBP1_CAPABLE_BIT)) as c_int
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn amd_fam14h_register() -> *mut cpuidle_monitor {
    unsafe {
        let mut num: c_int;

        if cpupower_cpu_info.vendor != X86_VENDOR_AMD {
            return core::ptr::null_mut();
        }

        if cpupower_cpu_info.family == 0x14 {
            strncpy(
                AMD_FAM14H_MONITOR.name.as_mut_ptr(),
                c"Fam_14h".as_ptr(),
                MONITOR_NAME_LEN - 1,
            );
        } else if cpupower_cpu_info.family == 0x12 {
            strncpy(
                AMD_FAM14H_MONITOR.name.as_mut_ptr(),
                c"Fam_12h".as_ptr(),
                MONITOR_NAME_LEN - 1,
            );
        } else {
            return core::ptr::null_mut();
        }

        /* We do not alloc for nbp1 machine wide counter */
        num = 0;
        while num < (AMD_FAM14H_STATE_NUM - 1) as c_int {
            PREVIOUS_COUNT[num as usize] =
                calloc(cpu_count as usize, core::mem::size_of::<c_ulonglong>())
                    as *mut c_ulonglong;
            CURRENT_COUNT[num as usize] =
                calloc(cpu_count as usize, core::mem::size_of::<c_ulonglong>())
                    as *mut c_ulonglong;
            num += 1;
        }

        /* We need PCI device: Slot 18, Func 6, compare with BKDG
         * for fam 12h/14h
         */
        AMD_FAM14H_PCI_DEV = pci_slot_func_init(&raw mut PCI_ACC, 0x18, 6);
        if AMD_FAM14H_PCI_DEV.is_null() || PCI_ACC.is_null() {
            return core::ptr::null_mut();
        }

        if is_nbp1_capable() == 0 {
            AMD_FAM14H_MONITOR.hw_states_num = (AMD_FAM14H_STATE_NUM - 1) as c_uint;
        }

        AMD_FAM14H_MONITOR.name_len = strlen(AMD_FAM14H_MONITOR.name.as_ptr()) as c_uint;
        &raw mut AMD_FAM14H_MONITOR
    }
}

unsafe extern "C" fn amd_fam14h_unregister() {
    unsafe {
        let mut num: c_int;
        num = 0;
        while num < (AMD_FAM14H_STATE_NUM - 1) as c_int {
            free(PREVIOUS_COUNT[num as usize] as *mut c_void);
            free(CURRENT_COUNT[num as usize] as *mut c_void);
            num += 1;
        }
        pci_cleanup(PCI_ACC);
    }
}

#[unsafe(no_mangle)]
pub static mut AMD_FAM14H_MONITOR: cpuidle_monitor = cpuidle_monitor {
    name: [0; MONITOR_NAME_LEN],
    name_len: 0,
    hw_states: &raw mut AMD_FAM14H_CSTATES as *mut cstate_t,
    hw_states_num: AMD_FAM14H_STATE_NUM as c_uint,
    start: Some(amd_fam14h_start),
    stop: Some(amd_fam14h_stop),
    do_register: Some(amd_fam14h_register),
    unregister: Some(amd_fam14h_unregister),
    flags: cpuidle_monitor_flags { needs_root: 1 },
    overflow_s: OVERFLOW_MS / 1000,
};
