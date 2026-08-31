// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2022, Athira Rajeev, IBM Corp.
 * Copyright 2022, Madhavan Srinivasan, IBM Corp.
 * Copyright 2022, Kajol Jain, IBM Corp.
 */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u64 = u64;
type size_t = usize;

const NULL: *mut c_void = ptr::null_mut();
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

/* C header dependencies preserved as external items. */
unsafe extern "C" {
    static POWER9: c_int;
    static POWER10: c_int;
    static POWER11: c_int;
    static PPC_FEATURE2_ARCH_3_00: u64;
    static PPC_FEATURE2_ARCH_3_1: u64;
    static PPC_FEATURE2_EBB: u64;
    static PERF_POWER9_MASK: u64;
    static PERF_POWER10_MASK: u64;
    static PERF_SAMPLE_REGS_INTR: u64;
    static PERF_SAMPLE_BRANCH_STACK: u64;
    static PERF_RECORD_SAMPLE: u32;
    static SPRN_PVR: c_int;
    static _SC_PAGESIZE: c_int;
    static PROT_READ: c_int;
    static PROT_WRITE: c_int;
    static MAP_SHARED: c_int;

    fn sysconf(name: c_int) -> c_long;
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn perror(s: *const c_char);
    fn printf(format: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;

    fn have_hwcap2(feature: u64) -> bool;
    fn event_init(event: *mut event, config: u64);
    fn event_open(event: *mut event) -> c_int;
    fn mfspr(spr: c_int) -> c_ulong;
    fn PVR_VER(pvr: c_ulong) -> c_int;
    fn mb();
    fn auxv_base_platform() -> *mut c_char;
    fn auxv_platform() -> *mut c_char;
    fn read_sysfs_file(path: *const c_char, buf: *mut c_char, size: size_t) -> c_int;
    fn FAIL_IF_EXIT(cond: c_int);
    fn SKIP_IF(cond: c_int);
}

#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32,
    pub sample_period: u64,
    pub disabled: u64,
    pub sample_type: u64,
    pub sample_regs_intr: u64,
    pub config: u64,
    pub config1: u64,
}

#[repr(C)]
pub struct event {
    pub attr: perf_event_attr,
}

#[repr(C)]
pub struct perf_event_header {
    pub type_: u32,
    pub misc: u16,
    pub size: u16,
}

#[repr(C)]
pub struct perf_event_mmap_page {
    pub data_head: c_ulong,
    pub data_tail: c_ulong,
}

/* Storage for platform version */
#[no_mangle]
pub static mut pvr: c_int = 0;
#[no_mangle]
pub static mut platform_extended_mask: u64 = 0;

/* Mask and Shift for Event code fields */
#[no_mangle]
pub static mut ev_mask_pmcxsel: c_int = 0;
#[no_mangle]
pub static mut ev_shift_pmcxsel: c_int = 0; //pmcxsel field
#[no_mangle]
pub static mut ev_mask_marked: c_int = 0;
#[no_mangle]
pub static mut ev_shift_marked: c_int = 0; //marked filed
#[no_mangle]
pub static mut ev_mask_comb: c_int = 0;
#[no_mangle]
pub static mut ev_shift_comb: c_int = 0; //combine field
#[no_mangle]
pub static mut ev_mask_unit: c_int = 0;
#[no_mangle]
pub static mut ev_shift_unit: c_int = 0; //unit field
#[no_mangle]
pub static mut ev_mask_pmc: c_int = 0;
#[no_mangle]
pub static mut ev_shift_pmc: c_int = 0; //pmc field
#[no_mangle]
pub static mut ev_mask_cache: c_int = 0;
#[no_mangle]
pub static mut ev_shift_cache: c_int = 0; //Cache sel field
#[no_mangle]
pub static mut ev_mask_sample: c_int = 0;
#[no_mangle]
pub static mut ev_shift_sample: c_int = 0; //Random sampling field
#[no_mangle]
pub static mut ev_mask_thd_sel: c_int = 0;
#[no_mangle]
pub static mut ev_shift_thd_sel: c_int = 0; //thresh_sel field
#[no_mangle]
pub static mut ev_mask_thd_start: c_int = 0;
#[no_mangle]
pub static mut ev_shift_thd_start: c_int = 0; //thresh_start field
#[no_mangle]
pub static mut ev_mask_thd_stop: c_int = 0;
#[no_mangle]
pub static mut ev_shift_thd_stop: c_int = 0; //thresh_stop field
#[no_mangle]
pub static mut ev_mask_thd_cmp: c_int = 0;
#[no_mangle]
pub static mut ev_shift_thd_cmp: c_int = 0; //thresh cmp field
#[no_mangle]
pub static mut ev_mask_sm: c_int = 0;
#[no_mangle]
pub static mut ev_shift_sm: c_int = 0; //SDAR mode field
#[no_mangle]
pub static mut ev_mask_rsq: c_int = 0;
#[no_mangle]
pub static mut ev_shift_rsq: c_int = 0; //radix scope qual field
#[no_mangle]
pub static mut ev_mask_l2l3: c_int = 0;
#[no_mangle]
pub static mut ev_shift_l2l3: c_int = 0; //l2l3 sel field
#[no_mangle]
pub static mut ev_mask_mmcr3_src: c_int = 0;
#[no_mangle]
pub static mut ev_shift_mmcr3_src: c_int = 0; //mmcr3 field

unsafe fn ev_code_extract_thd_cmp(value: u64) -> u64 {
    (value >> ev_shift_thd_cmp) & ev_mask_thd_cmp as u64
}

unsafe fn init_ev_encodes() {
    ev_mask_pmcxsel = 0xff;
    ev_shift_pmcxsel = 0;
    ev_mask_marked = 1;
    ev_shift_marked = 8;
    ev_mask_unit = 0xf;
    ev_shift_unit = 12;
    ev_mask_pmc = 0xf;
    ev_shift_pmc = 16;
    ev_mask_sample = 0x1f;
    ev_shift_sample = 24;
    ev_mask_thd_sel = 0x7;
    ev_shift_thd_sel = 29;
    ev_mask_thd_start = 0xf;
    ev_shift_thd_start = 36;
    ev_mask_thd_stop = 0xf;
    ev_shift_thd_stop = 32;

    if pvr == POWER11 || pvr == POWER10 {
        ev_mask_thd_cmp = 0x3ffff;
        ev_shift_thd_cmp = 0;
        ev_mask_rsq = 1;
        ev_shift_rsq = 9;
        ev_mask_comb = 3;
        ev_shift_comb = 10;
        ev_mask_cache = 3;
        ev_shift_cache = 20;
        ev_mask_sm = 0x3;
        ev_shift_sm = 22;
        ev_mask_l2l3 = 0x1f;
        ev_shift_l2l3 = 40;
        ev_mask_mmcr3_src = 0x7fff;
        ev_shift_mmcr3_src = 45;
    } else if pvr == POWER9 {
        ev_mask_comb = 3;
        ev_shift_comb = 10;
        ev_mask_cache = 0xf;
        ev_shift_cache = 20;
        ev_mask_thd_cmp = 0x3ff;
        ev_shift_thd_cmp = 40;
        ev_mask_sm = 0x3;
        ev_shift_sm = 50;
    } else {
        FAIL_IF_EXIT(1);
    }
}

/* Return the extended regs mask value */
#[no_mangle]
pub unsafe extern "C" fn perf_get_platform_reg_mask() -> u64 {
    if have_hwcap2(PPC_FEATURE2_ARCH_3_1) {
        return PERF_POWER10_MASK;
    }
    if have_hwcap2(PPC_FEATURE2_ARCH_3_00) {
        return PERF_POWER9_MASK;
    }

    -1i64 as u64
}

#[no_mangle]
pub unsafe extern "C" fn check_extended_regs_support() -> c_int {
    let mut fd: c_int;
    let mut event: event = core::mem::zeroed();

    event_init(&mut event, 0x1001e);

    event.attr.type_ = 4;
    event.attr.sample_period = 1;
    event.attr.disabled = 1;
    event.attr.sample_type = PERF_SAMPLE_REGS_INTR;
    event.attr.sample_regs_intr = platform_extended_mask;

    fd = event_open(&mut event);
    if fd != -1 {
        return 0;
    }

    -1
}

#[no_mangle]
pub unsafe extern "C" fn platform_check_for_tests() -> c_int {
    pvr = PVR_VER(mfspr(SPRN_PVR));

    /*
     * Check for supported platforms
     * for sampling test
     */
    if !(pvr == POWER11 || pvr == POWER10 || pvr == POWER9) {
        printf(
            b"%s: Tests unsupported for this platform\n\0".as_ptr() as *const c_char,
            b"platform_check_for_tests\0".as_ptr() as *const c_char,
        );
        return -1;
    }

    /*
     * Check PMU driver registered by looking for
     * PPC_FEATURE2_EBB bit in AT_HWCAP2
     */
    if !have_hwcap2(PPC_FEATURE2_EBB) || !have_hwcap2(PPC_FEATURE2_ARCH_3_00) {
        printf(
            b"%s: Tests unsupported for this platform\n\0".as_ptr() as *const c_char,
            b"platform_check_for_tests\0".as_ptr() as *const c_char,
        );
        return -1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn check_pvr_for_sampling_tests() -> c_int {
    SKIP_IF(platform_check_for_tests());

    platform_extended_mask = perf_get_platform_reg_mask();
    /* check if platform supports extended regs */
    if check_extended_regs_support() != 0 {
        printf(
            b"%s: Sampling tests un-supported\n\0".as_ptr() as *const c_char,
            b"check_pvr_for_sampling_tests\0".as_ptr() as *const c_char,
        );
        return -1;
    }

    init_ev_encodes();
    0
}

/*
 * Allocate mmap buffer of "mmap_pages" number of
 * pages.
 */
#[no_mangle]
pub unsafe extern "C" fn event_sample_buf_mmap(fd: c_int, mmap_pages: c_int) -> *mut c_void {
    let page_size: size_t = sysconf(_SC_PAGESIZE) as size_t;
    let mmap_size: size_t;
    let buff: *mut c_void;

    if mmap_pages <= 0 {
        return NULL;
    }

    if fd <= 0 {
        return NULL;
    }

    mmap_size = page_size * (1 + mmap_pages as size_t);
    buff = mmap(
        NULL,
        mmap_size,
        PROT_READ | PROT_WRITE,
        MAP_SHARED,
        fd,
        0,
    );

    if buff == MAP_FAILED {
        perror(b"mmap() failed.\0".as_ptr() as *const c_char);
        return NULL;
    }
    buff
}

/*
 * Post process the mmap buffer.
 * - If sample_count != NULL then return count of total
 *   number of samples present in the mmap buffer.
 * - If sample_count == NULL then return the address
 *   of first sample from the mmap buffer
 */
#[no_mangle]
pub unsafe extern "C" fn __event_read_samples(
    sample_buff: *mut c_void,
    size: *mut size_t,
    sample_count: *mut u64,
) -> *mut c_void {
    let page_size: size_t = sysconf(_SC_PAGESIZE) as size_t;
    let mut header: *mut perf_event_header =
        (sample_buff as *mut u8).add(page_size) as *mut perf_event_header;
    let metadata_page: *mut perf_event_mmap_page = sample_buff as *mut perf_event_mmap_page;
    let data_head: c_ulong;
    let mut data_tail: c_ulong;

    /*
     * PERF_RECORD_SAMPLE:
     * struct {
     *     struct perf_event_header hdr;
     *     u64 data[];
     * };
     */

    data_head = (*metadata_page).data_head;
    /* sync memory before reading sample */
    mb();
    data_tail = (*metadata_page).data_tail;

    /* Check for sample_count */
    if !sample_count.is_null() {
        *sample_count = 0;
    }

    loop {
        /*
         * Reads the mmap data buffer by moving
         * the data_tail to know the last read data.
         * data_head points to head in data buffer.
         * refer "struct perf_event_mmap_page" in
         * "include/uapi/linux/perf_event.h".
         */
        if data_head.wrapping_sub(data_tail) < size_of::<*mut perf_event_header>() as c_ulong {
            return NULL;
        }

        data_tail = data_tail.wrapping_add(size_of::<*mut perf_event_header>() as c_ulong);
        if (*header).type_ == PERF_RECORD_SAMPLE {
            *size = ((*header).size as size_t).wrapping_sub(size_of::<*mut perf_event_header>());
            if sample_count.is_null() {
                return (sample_buff as *mut u8)
                    .add(page_size)
                    .add(data_tail as size_t) as *mut c_void;
            }
            data_tail = data_tail.wrapping_add(*size as c_ulong);
            *sample_count = (*sample_count).wrapping_add(1);
        } else {
            *size = ((*header).size as size_t).wrapping_sub(size_of::<*mut perf_event_header>());
            if ((*metadata_page).data_tail).wrapping_add(*size as c_ulong) > (*metadata_page).data_head {
                data_tail = (*metadata_page).data_head;
            } else {
                data_tail = data_tail.wrapping_add(*size as c_ulong);
            }
        }
        header = (header as *mut u8).add((*header).size as size_t) as *mut perf_event_header;
    }
}

#[no_mangle]
pub unsafe extern "C" fn collect_samples(sample_buff: *mut c_void) -> c_int {
    let mut sample_count: u64 = 0;
    let mut size: size_t = 0;

    __event_read_samples(sample_buff, &mut size, &mut sample_count);
    sample_count as c_int
}

unsafe fn perf_read_first_sample(sample_buff: *mut c_void, size: *mut size_t) -> *mut c_void {
    __event_read_samples(sample_buff, size, ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn get_intr_regs(event: *mut event, sample_buff: *mut c_void) -> *mut u64 {
    let type_: u64 = (*event).attr.sample_type;
    let mut intr_regs: *mut u64;
    let mut size: size_t = 0;

    if (type_ ^ (PERF_SAMPLE_REGS_INTR | PERF_SAMPLE_BRANCH_STACK)) != 0
        && (type_ ^ PERF_SAMPLE_REGS_INTR) != 0
    {
        return ptr::null_mut();
    }

    intr_regs = perf_read_first_sample(sample_buff, &mut size) as *mut u64;
    if intr_regs.is_null() {
        return ptr::null_mut();
    }

    if (type_ & PERF_SAMPLE_BRANCH_STACK) != 0 {
        /*
         * PERF_RECORD_SAMPLE and PERF_SAMPLE_BRANCH_STACK:
         * struct {
         *     struct perf_event_header hdr;
         *     u64 number_of_branches;
         *     struct perf_branch_entry[number_of_branches];
         *     u64 data[];
         * };
         * struct perf_branch_entry {
         *     u64	from;
         *     u64	to;
         *     u64	misc;
         * };
         */
        intr_regs = intr_regs.add(((*intr_regs).wrapping_mul(3).wrapping_add(1)) as usize);
    }

    /*
     * First entry in the sample buffer used to specify
     * PERF_SAMPLE_REGS_ABI_64, skip perf regs abi to access
     * interrupt registers.
     */
    intr_regs = intr_regs.add(1);

    intr_regs
}

unsafe fn __perf_reg_mask(register_name: *const c_char) -> c_int {
    if strcmp(register_name, b"R0\0".as_ptr() as *const c_char) == 0 {
        0
    } else if strcmp(register_name, b"R1\0".as_ptr() as *const c_char) == 0 {
        1
    } else if strcmp(register_name, b"R2\0".as_ptr() as *const c_char) == 0 {
        2
    } else if strcmp(register_name, b"R3\0".as_ptr() as *const c_char) == 0 {
        3
    } else if strcmp(register_name, b"R4\0".as_ptr() as *const c_char) == 0 {
        4
    } else if strcmp(register_name, b"R5\0".as_ptr() as *const c_char) == 0 {
        5
    } else if strcmp(register_name, b"R6\0".as_ptr() as *const c_char) == 0 {
        6
    } else if strcmp(register_name, b"R7\0".as_ptr() as *const c_char) == 0 {
        7
    } else if strcmp(register_name, b"R8\0".as_ptr() as *const c_char) == 0 {
        8
    } else if strcmp(register_name, b"R9\0".as_ptr() as *const c_char) == 0 {
        9
    } else if strcmp(register_name, b"R10\0".as_ptr() as *const c_char) == 0 {
        10
    } else if strcmp(register_name, b"R11\0".as_ptr() as *const c_char) == 0 {
        11
    } else if strcmp(register_name, b"R12\0".as_ptr() as *const c_char) == 0 {
        12
    } else if strcmp(register_name, b"R13\0".as_ptr() as *const c_char) == 0 {
        13
    } else if strcmp(register_name, b"R14\0".as_ptr() as *const c_char) == 0 {
        14
    } else if strcmp(register_name, b"R15\0".as_ptr() as *const c_char) == 0 {
        15
    } else if strcmp(register_name, b"R16\0".as_ptr() as *const c_char) == 0 {
        16
    } else if strcmp(register_name, b"R17\0".as_ptr() as *const c_char) == 0 {
        17
    } else if strcmp(register_name, b"R18\0".as_ptr() as *const c_char) == 0 {
        18
    } else if strcmp(register_name, b"R19\0".as_ptr() as *const c_char) == 0 {
        19
    } else if strcmp(register_name, b"R20\0".as_ptr() as *const c_char) == 0 {
        20
    } else if strcmp(register_name, b"R21\0".as_ptr() as *const c_char) == 0 {
        21
    } else if strcmp(register_name, b"R22\0".as_ptr() as *const c_char) == 0 {
        22
    } else if strcmp(register_name, b"R23\0".as_ptr() as *const c_char) == 0 {
        23
    } else if strcmp(register_name, b"R24\0".as_ptr() as *const c_char) == 0 {
        24
    } else if strcmp(register_name, b"R25\0".as_ptr() as *const c_char) == 0 {
        25
    } else if strcmp(register_name, b"R26\0".as_ptr() as *const c_char) == 0 {
        26
    } else if strcmp(register_name, b"R27\0".as_ptr() as *const c_char) == 0 {
        27
    } else if strcmp(register_name, b"R28\0".as_ptr() as *const c_char) == 0 {
        28
    } else if strcmp(register_name, b"R29\0".as_ptr() as *const c_char) == 0 {
        29
    } else if strcmp(register_name, b"R30\0".as_ptr() as *const c_char) == 0 {
        30
    } else if strcmp(register_name, b"R31\0".as_ptr() as *const c_char) == 0 {
        31
    } else if strcmp(register_name, b"NIP\0".as_ptr() as *const c_char) == 0 {
        32
    } else if strcmp(register_name, b"MSR\0".as_ptr() as *const c_char) == 0 {
        33
    } else if strcmp(register_name, b"ORIG_R3\0".as_ptr() as *const c_char) == 0 {
        34
    } else if strcmp(register_name, b"CTR\0".as_ptr() as *const c_char) == 0 {
        35
    } else if strcmp(register_name, b"LINK\0".as_ptr() as *const c_char) == 0 {
        36
    } else if strcmp(register_name, b"XER\0".as_ptr() as *const c_char) == 0 {
        37
    } else if strcmp(register_name, b"CCR\0".as_ptr() as *const c_char) == 0 {
        38
    } else if strcmp(register_name, b"SOFTE\0".as_ptr() as *const c_char) == 0 {
        39
    } else if strcmp(register_name, b"TRAP\0".as_ptr() as *const c_char) == 0 {
        40
    } else if strcmp(register_name, b"DAR\0".as_ptr() as *const c_char) == 0 {
        41
    } else if strcmp(register_name, b"DSISR\0".as_ptr() as *const c_char) == 0 {
        42
    } else if strcmp(register_name, b"SIER\0".as_ptr() as *const c_char) == 0 {
        43
    } else if strcmp(register_name, b"MMCRA\0".as_ptr() as *const c_char) == 0 {
        44
    } else if strcmp(register_name, b"MMCR0\0".as_ptr() as *const c_char) == 0 {
        45
    } else if strcmp(register_name, b"MMCR1\0".as_ptr() as *const c_char) == 0 {
        46
    } else if strcmp(register_name, b"MMCR2\0".as_ptr() as *const c_char) == 0 {
        47
    } else if strcmp(register_name, b"MMCR3\0".as_ptr() as *const c_char) == 0 {
        48
    } else if strcmp(register_name, b"SIER2\0".as_ptr() as *const c_char) == 0 {
        49
    } else if strcmp(register_name, b"SIER3\0".as_ptr() as *const c_char) == 0 {
        50
    } else if strcmp(register_name, b"PMC1\0".as_ptr() as *const c_char) == 0 {
        51
    } else if strcmp(register_name, b"PMC2\0".as_ptr() as *const c_char) == 0 {
        52
    } else if strcmp(register_name, b"PMC3\0".as_ptr() as *const c_char) == 0 {
        53
    } else if strcmp(register_name, b"PMC4\0".as_ptr() as *const c_char) == 0 {
        54
    } else if strcmp(register_name, b"PMC5\0".as_ptr() as *const c_char) == 0 {
        55
    } else if strcmp(register_name, b"PMC6\0".as_ptr() as *const c_char) == 0 {
        56
    } else if strcmp(register_name, b"SDAR\0".as_ptr() as *const c_char) == 0 {
        57
    } else if strcmp(register_name, b"SIAR\0".as_ptr() as *const c_char) == 0 {
        58
    } else {
        -1
    }
}

#[no_mangle]
pub unsafe extern "C" fn get_reg_value(intr_regs: *mut u64, register_name: *mut c_char) -> u64 {
    let register_bit_position: c_int;

    register_bit_position = __perf_reg_mask(register_name);

    if register_bit_position < 0
        || (((platform_extended_mask >> (register_bit_position - 1)) & 1) == 0)
    {
        return -1i64 as u64;
    }

    *intr_regs.add(register_bit_position as usize)
}

#[no_mangle]
pub unsafe extern "C" fn get_thresh_cmp_val(event: event) -> c_int {
    let mut exp: c_int = 0;
    let mut result: u64 = 0;
    let mut value: u64;

    if !have_hwcap2(PPC_FEATURE2_ARCH_3_1) {
        return ev_code_extract_thd_cmp(event.attr.config) as c_int;
    }

    value = ev_code_extract_thd_cmp(event.attr.config1);

    if value == 0 {
        return value as c_int;
    }

    /*
     * Incase of P10, thresh_cmp value is not part of raw event code
     * and provided via attr.config1 parameter. To program threshold in MMCRA,
     * take a 18 bit number N and shift right 2 places and increment
     * the exponent E by 1 until the upper 10 bits of N are zero.
     * Write E to the threshold exponent and write the lower 8 bits of N
     * to the threshold mantissa.
     * The max threshold that can be written is 261120.
     */
    if value > 261120 {
        value = 261120;
    }
    while (64 - value.leading_zeros()) > 8 {
        exp += 1;
        value >>= 2;
    }

    /*
     * Note that it is invalid to write a mantissa with the
     * upper 2 bits of mantissa being zero, unless the
     * exponent is also zero.
     */
    if (value & 0xC0) == 0 && exp != 0 {
        result = -1i64 as u64;
    } else {
        result = ((exp as u64) << 8) | value;
    }
    result as c_int
}

/*
 * Utility function to check for generic compat PMU
 * by comparing base_platform value from auxv and real
 * PVR value.
 * auxv_base_platform() func gives information of "base platform"
 * corresponding to PVR value. Incase, if the distro doesn't
 * support platform PVR (missing cputable support), base platform
 * in auxv will have a default value other than the real PVR's.
 * In this case, ISAv3 PMU (generic compat PMU) will be registered
 * in the system. auxv_generic_compat_pmu() makes use of the base
 * platform value from auxv to do this check.
 */
unsafe fn auxv_generic_compat_pmu() -> bool {
    let mut base_pvr: c_int = 0;

    if strcmp(auxv_base_platform(), b"power9\0".as_ptr() as *const c_char) == 0 {
        base_pvr = POWER9;
    } else if strcmp(auxv_base_platform(), b"power10\0".as_ptr() as *const c_char) == 0 {
        base_pvr = POWER10;
    } else if strcmp(auxv_base_platform(), b"power11\0".as_ptr() as *const c_char) == 0 {
        base_pvr = POWER11;
    }

    base_pvr == 0
}

/*
 * Check for generic compat PMU.
 * First check for presence of pmu_name from
 * "/sys/bus/event_source/devices/cpu/caps".
 * If doesn't exist, fallback to using value
 * auxv.
 */
#[no_mangle]
pub unsafe extern "C" fn check_for_generic_compat_pmu() -> bool {
    let mut pmu_name: [c_char; 256] = [0; 256];

    memset(
        pmu_name.as_mut_ptr() as *mut c_void,
        0,
        size_of::<[c_char; 256]>(),
    );
    if read_sysfs_file(
        b"bus/event_source/devices/cpu/caps/pmu_name\0".as_ptr() as *const c_char,
        pmu_name.as_mut_ptr(),
        size_of::<[c_char; 256]>(),
    ) < 0
    {
        return auxv_generic_compat_pmu();
    }

    if strcmp(pmu_name.as_ptr(), b"ISAv3\0".as_ptr() as *const c_char) == 0 {
        true
    } else {
        false
    }
}

/*
 * Check if system is booted in compat mode.
 */
#[no_mangle]
pub unsafe extern "C" fn check_for_compat_mode() -> bool {
    let platform: *mut c_char = auxv_platform();
    let base_platform: *mut c_char = auxv_base_platform();

    strcmp(platform, base_platform) != 0
}
