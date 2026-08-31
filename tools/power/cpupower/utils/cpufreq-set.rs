// SPDX-License-Identifier: GPL-2.0-only
/*
 *  (C) 2004-2009  Dominik Brodowski <linux@dominikbrodowski.de>
 */

/* C dependencies: unistd.h, stdio.h, errno.h, stdlib.h, limits.h, string.h,
 * ctype.h, getopt.h, cpufreq.h, cpuidle.h, helpers/helpers.h
 */

use std::os::raw::{c_char, c_int, c_ulong, c_uint, c_void};

const NORM_FREQ_LEN: usize = 32;
const REQUIRED_ARGUMENT: c_int = 1;
const NO_ARGUMENT: c_int = 0;
const EINVAL: c_int = 22;
const ULONG_MAX: c_ulong = c_ulong::MAX;

#[repr(C)]
pub struct option {
    name: *const c_char,
    has_arg: c_int,
    flag: *mut c_int,
    val: c_int,
}

#[repr(C)]
pub struct cpufreq_policy {
    pub min: c_ulong,
    pub max: c_ulong,
    pub governor: *mut c_char,
}

#[repr(C)]
pub struct cpufreq_affected_cpus {
    pub cpu: c_uint,
    pub next: *mut cpufreq_affected_cpus,
}

#[repr(C)]
pub struct freq_units {
    str_unit: *const c_char,
    power_of_ten: c_int,
}

unsafe extern "C" {
    static mut optarg: *mut c_char;
    static mut errno: c_int;
    static mut cpus_chosen: *mut c_void;

    fn printf(format: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn getopt_long(
        argc: c_int,
        argv: *mut *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;

    fn cpufreq_get_policy(cpu: c_uint) -> *mut cpufreq_policy;
    fn cpufreq_put_policy(policy: *mut cpufreq_policy);
    fn cpufreq_set_policy(cpu: c_uint, policy: *mut cpufreq_policy) -> c_int;
    fn cpufreq_set_frequency(cpu: c_uint, target_frequency: c_ulong) -> c_int;
    fn cpufreq_modify_policy_min(cpu: c_uint, min_freq: c_ulong) -> c_int;
    fn cpufreq_modify_policy_max(cpu: c_uint, max_freq: c_ulong) -> c_int;
    fn cpufreq_modify_policy_governor(cpu: c_uint, governor: *mut c_char) -> c_int;
    fn cpufreq_get_related_cpus(cpu: c_uint) -> *mut cpufreq_affected_cpus;
    fn cpufreq_put_related_cpus(cpus: *mut cpufreq_affected_cpus);

    fn bitmask_isallclear(mask: *mut c_void) -> c_int;
    fn bitmask_setall(mask: *mut c_void);
    fn bitmask_first(mask: *mut c_void) -> c_uint;
    fn bitmask_last(mask: *mut c_void) -> c_uint;
    fn bitmask_isbitset(mask: *mut c_void, bit: c_uint) -> c_int;
    fn bitmask_setbit(mask: *mut c_void, bit: c_uint) -> c_int;

    fn cpupower_is_cpu_online(cpu: c_uint) -> c_int;
    fn get_cpustate();
    fn print_offline_cpus();
}

static mut SET_OPTS: [option; 6] = [
    option {
        name: c"min".as_ptr(),
        has_arg: REQUIRED_ARGUMENT,
        flag: std::ptr::null_mut(),
        val: b'd' as c_int,
    },
    option {
        name: c"max".as_ptr(),
        has_arg: REQUIRED_ARGUMENT,
        flag: std::ptr::null_mut(),
        val: b'u' as c_int,
    },
    option {
        name: c"governor".as_ptr(),
        has_arg: REQUIRED_ARGUMENT,
        flag: std::ptr::null_mut(),
        val: b'g' as c_int,
    },
    option {
        name: c"freq".as_ptr(),
        has_arg: REQUIRED_ARGUMENT,
        flag: std::ptr::null_mut(),
        val: b'f' as c_int,
    },
    option {
        name: c"related".as_ptr(),
        has_arg: NO_ARGUMENT,
        flag: std::ptr::null_mut(),
        val: b'r' as c_int,
    },
    option {
        name: std::ptr::null(),
        has_arg: 0,
        flag: std::ptr::null_mut(),
        val: 0,
    },
];

#[inline]
unsafe fn _(s: *const c_char) -> *const c_char {
    s
}

unsafe fn isdigit_c(ch: c_char) -> bool {
    ch >= b'0' as c_char && ch <= b'9' as c_char
}

unsafe fn tolower_c(ch: c_char) -> c_char {
    if ch >= b'A' as c_char && ch <= b'Z' as c_char {
        ch + (b'a' - b'A') as c_char
    } else {
        ch
    }
}

unsafe fn print_error() {
    printf(_(c"Error setting new values. Common errors:\n- Do you have proper administration rights? (super-user?)\n- Is the governor you requested available and modprobed?\n- Trying to set an invalid policy?\n- Trying to set a specific frequency, but userspace governor is not available,\n   for example because of hardware which cannot be set to a specific frequency\n   or because the userspace governor isn't loaded?\n".as_ptr()));
}

static DEF_UNITS: [freq_units; 6] = [
    freq_units {
        str_unit: c"hz".as_ptr(),
        power_of_ten: -3,
    },
    freq_units {
        str_unit: c"khz".as_ptr(),
        power_of_ten: 0,
    }, /* default */
    freq_units {
        str_unit: c"mhz".as_ptr(),
        power_of_ten: 3,
    },
    freq_units {
        str_unit: c"ghz".as_ptr(),
        power_of_ten: 6,
    },
    freq_units {
        str_unit: c"thz".as_ptr(),
        power_of_ten: 9,
    },
    freq_units {
        str_unit: std::ptr::null(),
        power_of_ten: 0,
    },
];

unsafe fn print_unknown_arg() {
    printf(_(c"invalid or unknown argument\n".as_ptr()));
}

unsafe fn string_to_frequency(mut str_: *const c_char) -> c_ulong {
    let mut normalized = [0 as c_char; NORM_FREQ_LEN];
    let mut unit: *const freq_units;
    let mut scan: *const c_char;
    let mut end: *mut c_char = std::ptr::null_mut();
    let freq: c_ulong;
    let mut power: c_int = 0;
    let mut match_count: c_int = 0;
    let mut i: c_int;
    let mut cp: c_int;
    let pad: c_int;

    while *str_ == b'0' as c_char {
        str_ = str_.add(1);
    }

    scan = str_;
    while isdigit_c(*scan) || *scan == b'.' as c_char {
        if *scan == b'.' as c_char && match_count == 0 {
            match_count = 1;
        } else if *scan == b'.' as c_char && match_count == 1 {
            return 0;
        }
        scan = scan.add(1);
    }

    if *scan != 0 {
        match_count = 0;
        unit = DEF_UNITS.as_ptr();
        while !(*unit).str_unit.is_null() {
            i = 0;
            while *scan.add(i as usize) != 0
                && tolower_c(*scan.add(i as usize)) == *(*unit).str_unit.add(i as usize)
            {
                i += 1;
            }
            if *scan.add(i as usize) != 0 {
                unit = unit.add(1);
                continue;
            }
            match_count += 1;
            power = (*unit).power_of_ten;
            unit = unit.add(1);
        }
        if match_count != 1 {
            return 0;
        }
    }

    /* count the number of digits to be copied */
    cp = 0;
    while isdigit_c(*str_.add(cp as usize)) {
        cp += 1;
    }

    if *str_.add(cp as usize) == b'.' as c_char {
        while power > -1 && isdigit_c(*str_.add((cp + 1) as usize)) {
            cp += 1;
            power -= 1;
        }
    }
    if power >= -1 {
        /* not enough => pad */
        pad = power + 1;
    } else {
        /* too much => strip */
        pad = 0;
        cp += power + 1;
    }
    /* check bounds */
    if cp <= 0 || cp + pad > NORM_FREQ_LEN as c_int - 1 {
        return 0;
    }

    /* copy digits */
    i = 0;
    while i < cp {
        if *str_ == b'.' as c_char {
            str_ = str_.add(1);
        }
        normalized[i as usize] = *str_;
        i += 1;
        str_ = str_.add(1);
    }
    /* and pad */
    while i < cp + pad {
        normalized[i as usize] = b'0' as c_char;
        i += 1;
    }

    /* round up, down ? */
    match_count = (normalized[(i - 1) as usize] >= b'5' as c_char) as c_int;
    /* and drop the decimal part */
    normalized[(i - 1) as usize] = 0; /* cp > 0 && pad >= 0 ==> i > 0 */

    /* final conversion (and applying rounding) */
    errno = 0;
    freq = strtoul(normalized.as_ptr(), &mut end, 10);
    if errno != 0 {
        0
    } else {
        if match_count != 0 && freq != ULONG_MAX {
            freq + 1
        } else {
            freq
        }
    }
}

unsafe fn do_new_policy(cpu: c_uint, new_pol: *mut cpufreq_policy) -> c_int {
    let cur_pol = cpufreq_get_policy(cpu);
    let ret: c_int;

    if cur_pol.is_null() {
        printf(_(c"wrong, unknown or unhandled CPU?\n".as_ptr()));
        return -EINVAL;
    }

    if (*new_pol).min == 0 {
        (*new_pol).min = (*cur_pol).min;
    }

    if (*new_pol).max == 0 {
        (*new_pol).max = (*cur_pol).max;
    }

    if (*new_pol).governor.is_null() {
        (*new_pol).governor = (*cur_pol).governor;
    }

    ret = cpufreq_set_policy(cpu, new_pol);

    cpufreq_put_policy(cur_pol);

    ret
}

unsafe fn do_one_cpu(
    cpu: c_uint,
    new_pol: *mut cpufreq_policy,
    freq: c_ulong,
    pc: c_uint,
) -> c_int {
    match pc {
        0 => cpufreq_set_frequency(cpu, freq),

        1 => {
            /* if only one value of a policy is to be changed, we can
             * use a "fast path".
             */
            if (*new_pol).min != 0 {
                cpufreq_modify_policy_min(cpu, (*new_pol).min)
            } else if (*new_pol).max != 0 {
                cpufreq_modify_policy_max(cpu, (*new_pol).max)
            } else if !(*new_pol).governor.is_null() {
                cpufreq_modify_policy_governor(cpu, (*new_pol).governor)
            } else {
                do_new_policy(cpu, new_pol)
            }
        }

        _ => {
            /* slow path */
            do_new_policy(cpu, new_pol)
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cmd_freq_set(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut ret: c_int = 0;
    let mut cont: c_int = 1;
    let mut double_parm: c_int = 0;
    let mut related: c_int = 0;
    let mut policychange: c_int = 0;
    let mut freq: c_ulong = 0;
    let mut gov = [0 as c_char; 20];
    let mut cpu: c_uint;

    let mut new_pol = cpufreq_policy {
        min: 0,
        max: 0,
        governor: std::ptr::null_mut(),
    };

    /* parameter parsing */
    loop {
        ret = getopt_long(
            argc,
            argv,
            c"d:u:g:f:r".as_ptr(),
            SET_OPTS.as_ptr(),
            std::ptr::null_mut(),
        );
        match ret {
            x if x == b'?' as c_int => {
                print_unknown_arg();
                return -EINVAL;
            }
            -1 => {
                cont = 0;
            }
            x if x == b'r' as c_int => {
                if related != 0 {
                    double_parm += 1;
                }
                related += 1;
            }
            x if x == b'd' as c_int => {
                if new_pol.min != 0 {
                    double_parm += 1;
                }
                policychange += 1;
                new_pol.min = string_to_frequency(optarg);
                if new_pol.min == 0 {
                    print_unknown_arg();
                    return -EINVAL;
                }
            }
            x if x == b'u' as c_int => {
                if new_pol.max != 0 {
                    double_parm += 1;
                }
                policychange += 1;
                new_pol.max = string_to_frequency(optarg);
                if new_pol.max == 0 {
                    print_unknown_arg();
                    return -EINVAL;
                }
            }
            x if x == b'f' as c_int => {
                if freq != 0 {
                    double_parm += 1;
                }
                freq = string_to_frequency(optarg);
                if freq == 0 {
                    print_unknown_arg();
                    return -EINVAL;
                }
            }
            x if x == b'g' as c_int => {
                if !new_pol.governor.is_null() {
                    double_parm += 1;
                }
                policychange += 1;
                if strlen(optarg) < 3 || strlen(optarg) > 18 {
                    print_unknown_arg();
                    return -EINVAL;
                }
                if sscanf(optarg, c"%19s".as_ptr(), gov.as_mut_ptr()) != 1 {
                    print_unknown_arg();
                    return -EINVAL;
                }
                new_pol.governor = gov.as_mut_ptr();
            }
            _ => {}
        }
        if cont == 0 {
            break;
        }
    }

    /* parameter checking */
    if double_parm != 0 {
        printf(c"the same parameter was passed more than once\n".as_ptr());
        return -EINVAL;
    }

    if freq != 0 && policychange != 0 {
        printf(_(c"the -f/--freq parameter cannot be combined with -d/--min, -u/--max or\n-g/--governor parameters\n".as_ptr()));
        return -EINVAL;
    }

    if freq == 0 && policychange == 0 {
        printf(_(c"At least one parameter out of -f/--freq, -d/--min, -u/--max, and\n-g/--governor must be passed\n".as_ptr()));
        return -EINVAL;
    }

    /* Default is: set all CPUs */
    if bitmask_isallclear(cpus_chosen) != 0 {
        bitmask_setall(cpus_chosen);
    }

    /* Also set frequency settings for related CPUs if -r is passed */
    if related != 0 {
        cpu = bitmask_first(cpus_chosen);
        while cpu <= bitmask_last(cpus_chosen) {
            let mut cpus: *mut cpufreq_affected_cpus;

            if bitmask_isbitset(cpus_chosen, cpu) == 0 || cpupower_is_cpu_online(cpu) != 1 {
                cpu = cpu.wrapping_add(1);
                continue;
            }

            cpus = cpufreq_get_related_cpus(cpu);
            if cpus.is_null() {
                break;
            }
            while !(*cpus).next.is_null() {
                bitmask_setbit(cpus_chosen, (*cpus).cpu);
                cpus = (*cpus).next;
            }
            /* Set the last cpu in related cpus list */
            bitmask_setbit(cpus_chosen, (*cpus).cpu);
            cpufreq_put_related_cpus(cpus);
            cpu = cpu.wrapping_add(1);
        }
    }

    get_cpustate();

    /* loop over CPUs */
    cpu = bitmask_first(cpus_chosen);
    while cpu <= bitmask_last(cpus_chosen) {
        if bitmask_isbitset(cpus_chosen, cpu) == 0 || cpupower_is_cpu_online(cpu) != 1 {
            cpu = cpu.wrapping_add(1);
            continue;
        }

        printf(_(c"Setting cpu: %d\n".as_ptr()), cpu);
        ret = do_one_cpu(cpu, &mut new_pol, freq, policychange as c_uint);
        if ret != 0 {
            print_error();
            return ret;
        }
        cpu = cpu.wrapping_add(1);
    }

    print_offline_cpus();

    0
}
