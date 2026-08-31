// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/tests/sample-parsing.c. C include dependencies are
// expected to be supplied by the surrounding translated perf tree.

const BS_EXPECTED_BE: u64 = 0xa000d00000000000;
const BS_EXPECTED_LE: u64 = 0x1aa00000000;

macro_rules! comp {
    ($s1:expr, $s2:expr, $field:tt) => {{
        if (*$s1).$field != (*$s2).$field {
            pr_debug(format_args!("Samples differ at '{}'\n", stringify!($field)));
            return false;
        }
    }};
}

macro_rules! comp_path {
    ($left:expr, $right:expr, $name:expr) => {{
        if $left != $right {
            pr_debug(format_args!("Samples differ at '{}'\n", $name));
            return false;
        }
    }};
}

macro_rules! mcomp_path {
    ($left:expr, $right:expr, $name:expr) => {{
        if memcmp(
            core::ptr::addr_of!($left) as *const core::ffi::c_void,
            core::ptr::addr_of!($right) as *const core::ffi::c_void,
            core::mem::size_of_val(&$left),
        ) != 0
        {
            pr_debug(format_args!("Samples differ at '{}'\n", $name));
            return false;
        }
    }};
}

unsafe fn samples_same(
    s1: *mut perf_sample,
    s2: *mut perf_sample,
    type_: u64,
    read_format: u64,
    needs_swap: bool,
) -> bool {
    let mut i: usize;

    if type_ & PERF_SAMPLE_IDENTIFIER != 0 {
        comp!(s1, s2, id);
    }

    if type_ & PERF_SAMPLE_IP != 0 {
        comp!(s1, s2, ip);
    }

    if type_ & PERF_SAMPLE_TID != 0 {
        comp!(s1, s2, pid);
        comp!(s1, s2, tid);
    }

    if type_ & PERF_SAMPLE_TIME != 0 {
        comp!(s1, s2, time);
    }

    if type_ & PERF_SAMPLE_ADDR != 0 {
        comp!(s1, s2, addr);
    }

    if type_ & PERF_SAMPLE_ID != 0 {
        comp!(s1, s2, id);
    }

    if type_ & PERF_SAMPLE_STREAM_ID != 0 {
        comp!(s1, s2, stream_id);
    }

    if type_ & PERF_SAMPLE_CPU != 0 {
        comp!(s1, s2, cpu);
    }

    if type_ & PERF_SAMPLE_PERIOD != 0 {
        comp!(s1, s2, period);
    }

    if type_ & PERF_SAMPLE_READ != 0 {
        if read_format & PERF_FORMAT_GROUP != 0 {
            comp_path!((*s1).read.group.nr, (*s2).read.group.nr, "read.group.nr");
        } else {
            comp_path!((*s1).read.one.value, (*s2).read.one.value, "read.one.value");
        }
        if read_format & PERF_FORMAT_TOTAL_TIME_ENABLED != 0 {
            comp_path!(
                (*s1).read.time_enabled,
                (*s2).read.time_enabled,
                "read.time_enabled"
            );
        }
        if read_format & PERF_FORMAT_TOTAL_TIME_RUNNING != 0 {
            comp_path!(
                (*s1).read.time_running,
                (*s2).read.time_running,
                "read.time_running"
            );
        }
        /* PERF_FORMAT_ID is forced for PERF_SAMPLE_READ */
        if read_format & PERF_FORMAT_GROUP != 0 {
            let mut v1: *mut sample_read_value = (*s1).read.group.values;
            let mut v2: *mut sample_read_value = (*s2).read.group.values;

            i = 0;
            while i < (*s1).read.group.nr as usize {
                if (*v1).value != (*v2).value {
                    pr_debug(format_args!(
                        "Samples differ at 'read.group.values[].value'\n"
                    ));
                    return false;
                }

                if (*v1).id != (*v2).id {
                    pr_debug(format_args!(
                        "Samples differ at 'read.group.values[].id'\n"
                    ));
                    return false;
                }

                if read_format & PERF_FORMAT_LOST != 0 && (*v1).lost != (*v2).lost {
                    pr_debug(format_args!(
                        "Samples differ at 'read.group.values[].lost'\n"
                    ));
                    return false;
                }
                v1 = next_sample_read_value(v1, read_format);
                v2 = next_sample_read_value(v2, read_format);
                i += 1;
            }
        } else {
            comp_path!((*s1).read.one.id, (*s2).read.one.id, "read.one.id");
            if read_format & PERF_FORMAT_LOST != 0 {
                comp_path!((*s1).read.one.lost, (*s2).read.one.lost, "read.one.lost");
            }
        }
    }

    if type_ & PERF_SAMPLE_CALLCHAIN != 0 {
        comp_path!((*(*s1).callchain).nr, (*(*s2).callchain).nr, "callchain->nr");
        i = 0;
        while i < (*(*s1).callchain).nr as usize {
            comp_path!(
                (*(*s1).callchain).ips[i],
                (*(*s2).callchain).ips[i],
                "callchain->ips[i]"
            );
            i += 1;
        }
    }

    if type_ & PERF_SAMPLE_RAW != 0 {
        comp!(s1, s2, raw_size);
        if memcmp((*s1).raw_data, (*s2).raw_data, (*s1).raw_size as usize) != 0 {
            pr_debug(format_args!("Samples differ at 'raw_data'\n"));
            return false;
        }
    }

    if type_ & PERF_SAMPLE_BRANCH_STACK != 0 {
        comp_path!(
            (*(*s1).branch_stack).nr,
            (*(*s2).branch_stack).nr,
            "branch_stack->nr"
        );
        comp_path!(
            (*(*s1).branch_stack).hw_idx,
            (*(*s2).branch_stack).hw_idx,
            "branch_stack->hw_idx"
        );
        i = 0;
        while i < (*(*s1).branch_stack).nr as usize {
            if needs_swap {
                return if host_is_bigendian() {
                    (*(*s2).branch_stack).entries[i].flags.value == BS_EXPECTED_BE
                } else {
                    (*(*s2).branch_stack).entries[i].flags.value == BS_EXPECTED_LE
                };
            } else {
                mcomp_path!(
                    (*(*s1).branch_stack).entries[i],
                    (*(*s2).branch_stack).entries[i],
                    "branch_stack->entries[i]"
                );
            }
            i += 1;
        }
    }

    if type_ & PERF_SAMPLE_REGS_USER != 0 {
        let s1_regs: *mut regs_dump = perf_sample__user_regs(s1);
        let s2_regs: *mut regs_dump = perf_sample__user_regs(s2);
        let sz: usize = hweight_long((*s1_regs).mask) as usize * core::mem::size_of::<u64>();

        comp_path!((*(*s1).user_regs).mask, (*(*s2).user_regs).mask, "user_regs->mask");
        comp_path!((*(*s1).user_regs).abi, (*(*s2).user_regs).abi, "user_regs->abi");
        if (*s1_regs).abi != 0
            && ((*s1_regs).regs.is_null()
                || (*s2_regs).regs.is_null()
                || memcmp(
                    (*s1_regs).regs as *const core::ffi::c_void,
                    (*s2_regs).regs as *const core::ffi::c_void,
                    sz,
                ) != 0)
        {
            pr_debug(format_args!("Samples differ at 'user_regs'\n"));
            return false;
        }
    }

    if type_ & PERF_SAMPLE_STACK_USER != 0 {
        comp_path!((*s1).user_stack.size, (*s2).user_stack.size, "user_stack.size");
        if memcmp(
            (*s1).user_stack.data,
            (*s2).user_stack.data,
            (*s1).user_stack.size as usize,
        ) != 0
        {
            pr_debug(format_args!("Samples differ at 'user_stack'\n"));
            return false;
        }
    }

    if type_ & PERF_SAMPLE_WEIGHT != 0 {
        comp!(s1, s2, weight);
    }

    if type_ & PERF_SAMPLE_WEIGHT_STRUCT != 0 {
        comp!(s1, s2, weight);
        comp!(s1, s2, ins_lat);
        comp!(s1, s2, weight3);
    }

    if type_ & PERF_SAMPLE_DATA_SRC != 0 {
        comp!(s1, s2, data_src);
    }

    if type_ & PERF_SAMPLE_TRANSACTION != 0 {
        comp!(s1, s2, transaction);
    }

    if type_ & PERF_SAMPLE_REGS_INTR != 0 {
        let s1_regs: *mut regs_dump = perf_sample__intr_regs(s1);
        let s2_regs: *mut regs_dump = perf_sample__intr_regs(s2);
        let sz: usize = hweight_long((*s1_regs).mask) as usize * core::mem::size_of::<u64>();

        comp_path!((*(*s1).intr_regs).mask, (*(*s2).intr_regs).mask, "intr_regs->mask");
        comp_path!((*(*s1).intr_regs).abi, (*(*s2).intr_regs).abi, "intr_regs->abi");
        if (*s1_regs).abi != 0
            && ((*s1_regs).regs.is_null()
                || (*s2_regs).regs.is_null()
                || memcmp(
                    (*s1_regs).regs as *const core::ffi::c_void,
                    (*s2_regs).regs as *const core::ffi::c_void,
                    sz,
                ) != 0)
        {
            pr_debug(format_args!("Samples differ at 'intr_regs'\n"));
            return false;
        }
    }

    if type_ & PERF_SAMPLE_PHYS_ADDR != 0 {
        comp!(s1, s2, phys_addr);
    }

    if type_ & PERF_SAMPLE_CGROUP != 0 {
        comp!(s1, s2, cgroup);
    }

    if type_ & PERF_SAMPLE_DATA_PAGE_SIZE != 0 {
        comp!(s1, s2, data_page_size);
    }

    if type_ & PERF_SAMPLE_CODE_PAGE_SIZE != 0 {
        comp!(s1, s2, code_page_size);
    }

    if type_ & PERF_SAMPLE_AUX != 0 {
        comp_path!((*s1).aux_sample.size, (*s2).aux_sample.size, "aux_sample.size");
        if memcmp(
            (*s1).aux_sample.data,
            (*s2).aux_sample.data,
            (*s1).aux_sample.size as usize,
        ) != 0
        {
            pr_debug(format_args!("Samples differ at 'aux_sample'\n"));
            return false;
        }
    }

    true
}

unsafe fn do_test(sample_type: u64, sample_regs: u64, read_format: u64) -> i32 {
    let mut attr: perf_event_attr = core::mem::zeroed();
    attr.sample_type = sample_type;
    attr.read_format = read_format;

    let mut callchain: ip_callchain_data = core::mem::zeroed();
    callchain.data[0] = 3; /* 3 ips */
    callchain.data[1] = 201;
    callchain.data[2] = 202;
    callchain.data[3] = 203;

    let mut branch_stack: branch_stack_data = core::mem::zeroed();
    branch_stack.data[0] = 1; /* 1 branch_entry */
    branch_stack.data[1] = !0u64;
    branch_stack.data[2] = 211;
    branch_stack.data[3] = 212;
    branch_stack.data[4] = 213;

    let mut regs: [u64; 64] = [0; 64];
    let raw_data: [u32; 5] = [0x12345678, 0x0a0b0c0d, 0x11020304, 0x05060708, 0];
    let data: [u64; 3] = [0x2211443366558877, 0, 0xaabbccddeeff4321];
    let aux_data: [u64; 4] = [0xa55a, 0, 0xeeddee, 0x0282028202820282];
    let mut user_regs = regs_dump {
        abi: PERF_SAMPLE_REGS_ABI_64,
        mask: sample_regs,
        regs: regs.as_mut_ptr(),
    };
    let mut intr_regs = regs_dump {
        abi: PERF_SAMPLE_REGS_ABI_64,
        mask: sample_regs,
        regs: regs.as_mut_ptr(),
    };
    let mut sample: perf_sample = core::mem::zeroed();
    sample.ip = 101;
    sample.pid = 102;
    sample.tid = 103;
    sample.time = 104;
    sample.addr = 105;
    sample.id = 106;
    sample.stream_id = 107;
    sample.period = 108;
    sample.weight = 109;
    sample.cpu = 110;
    sample.raw_size = core::mem::size_of_val(&raw_data) as u32;
    sample.data_src = 111;
    sample.transaction = 112;
    sample.raw_data = raw_data.as_ptr() as *mut core::ffi::c_void;
    sample.callchain = core::ptr::addr_of_mut!(callchain.callchain);
    sample.no_hw_idx = false;
    sample.branch_stack = core::ptr::addr_of_mut!(branch_stack.branch_stack);
    sample.user_regs = &mut user_regs;
    sample.user_stack.size = core::mem::size_of_val(&data) as u64;
    sample.user_stack.data = data.as_ptr() as *mut core::ffi::c_void;
    sample.read.time_enabled = 0x030a59d664fca7de;
    sample.read.time_running = 0x011b6ae553eb98ed;
    sample.intr_regs = &mut intr_regs;
    sample.phys_addr = 113;
    sample.cgroup = 114;
    sample.data_page_size = 115;
    sample.code_page_size = 116;
    sample.ins_lat = 117;
    sample.weight3 = 118;
    sample.aux_sample.size = core::mem::size_of_val(&aux_data) as u64;
    sample.aux_sample.data = aux_data.as_ptr() as *mut core::ffi::c_void;

    let values: [sample_read_value; 4] = [
        sample_read_value { value: 1, id: 5, lost: 0 },
        sample_read_value { value: 9, id: 3, lost: 0 },
        sample_read_value { value: 2, id: 7, lost: 0 },
        sample_read_value { value: 6, id: 4, lost: 1 },
    ];
    let mut packed_values: [sample_read_value; 4] = core::mem::zeroed();
    let mut sample_out: perf_sample = core::mem::zeroed();
    let mut sample_out_endian: perf_sample = core::mem::zeroed();
    let mut i: usize;
    let mut ret: i32 = -1;

    let evsel: *mut evsel = evsel__new(&mut attr);
    if evsel.is_null() {
        pr_debug(format_args!("evsel__new failed\n"));
        return -1;
    }
    perf_sample__init(&mut sample_out, false);
    perf_sample__init(&mut sample_out_endian, false);
    if sample_type & PERF_SAMPLE_REGS_USER != 0 {
        (*evsel).core.attr.sample_regs_user = sample_regs;
    }

    if sample_type & PERF_SAMPLE_REGS_INTR != 0 {
        (*evsel).core.attr.sample_regs_intr = sample_regs;
    }

    if sample_type & PERF_SAMPLE_BRANCH_STACK != 0 {
        (*evsel).core.attr.branch_sample_type |= PERF_SAMPLE_BRANCH_HW_INDEX;
    }

    i = 0;
    while i < core::mem::size_of_val(&regs) {
        *(regs.as_mut_ptr() as *mut u8).add(i) = (i & 0xfe) as u8;
        i += 1;
    }

    if read_format & PERF_FORMAT_GROUP != 0 {
        let vsz: usize = sample_read_value_size(read_format);

        /*
         * evsel__parse_sample() points read.group.values at the event
         * data, where the entries are packed according to read_format,
         * so build the input the same way.  Otherwise the fields
         * compared afterwards are just overlapping bytes.
         */
        i = 0;
        while i < values.len() {
            memcpy(
                (packed_values.as_mut_ptr() as *mut u8).add(i * vsz) as *mut core::ffi::c_void,
                core::ptr::addr_of!(values[i]) as *const core::ffi::c_void,
                vsz,
            );
            i += 1;
        }

        sample.read.group.nr = values.len() as u64;
        sample.read.group.values = packed_values.as_mut_ptr();
    } else {
        sample.read.one.value = 0x08789faeb786aa87;
        sample.read.one.id = 99;
        sample.read.one.lost = 1;
    }

    let sz: usize = perf_event__sample_event_size(
        &mut sample,
        sample_type,
        read_format,
        (*evsel).core.attr.branch_sample_type,
    );
    let bufsz: usize = sz + 4096; /* Add a bit for overrun checking */
    let event: *mut perf_event = malloc(bufsz) as *mut perf_event;
    if event.is_null() {
        pr_debug(format_args!("malloc failed\n"));
        goto_out_free(event, evsel, &mut sample_out, &mut sample_out_endian);
        return ret;
    }

    memset(event as *mut core::ffi::c_void, 0xff, bufsz);
    (*event).header.type_ = PERF_RECORD_SAMPLE;
    (*event).header.misc = 0;
    (*event).header.size = sz as u16;

    let mut err: i32 = perf_event__synthesize_sample(
        event,
        sample_type,
        read_format,
        (*evsel).core.attr.branch_sample_type,
        &mut sample,
    );
    if err != 0 {
        pr_debug(format_args!(
            "{} failed for sample_type {:#x}, error {}\n",
            "perf_event__synthesize_sample",
            sample_type,
            err
        ));
        goto_out_free(event, evsel, &mut sample_out, &mut sample_out_endian);
        return ret;
    }

    /* The data does not contain 0xff so we use that to check the size */
    i = bufsz;
    while i > 0 {
        if *((event as *mut u8).add(i - 1)) != 0xff {
            break;
        }
        i -= 1;
    }
    if i != sz {
        pr_debug(format_args!(
            "Event size mismatch: actual {} vs expected {}\n",
            i, sz
        ));
        goto_out_free(event, evsel, &mut sample_out, &mut sample_out_endian);
        return ret;
    }

    (*evsel).sample_size = __evsel__sample_size(sample_type);

    err = evsel__parse_sample(evsel, event, &mut sample_out);
    if err != 0 {
        pr_debug(format_args!(
            "{} failed for sample_type {:#x}, error {}\n",
            "evsel__parse_sample",
            sample_type,
            err
        ));
        goto_out_free(event, evsel, &mut sample_out, &mut sample_out_endian);
        return ret;
    }

    if !samples_same(
        &mut sample,
        &mut sample_out,
        sample_type,
        read_format,
        (*evsel).needs_swap,
    ) {
        pr_debug(format_args!("parsing failed for sample_type {:#x}\n", sample_type));
        goto_out_free(event, evsel, &mut sample_out, &mut sample_out_endian);
        return ret;
    }

    if sample_type == PERF_SAMPLE_BRANCH_STACK {
        (*evsel).needs_swap = true;
        (*evsel).sample_size = __evsel__sample_size(sample_type);
        err = evsel__parse_sample(evsel, event, &mut sample_out_endian);
        if err != 0 {
            pr_debug(format_args!(
                "{} failed for sample_type {:#x}, error {}\n",
                "evsel__parse_sample",
                sample_type,
                err
            ));
            goto_out_free(event, evsel, &mut sample_out, &mut sample_out_endian);
            return ret;
        }

        if !samples_same(
            &mut sample,
            &mut sample_out_endian,
            sample_type,
            read_format,
            (*evsel).needs_swap,
        ) {
            pr_debug(format_args!("parsing failed for sample_type {:#x}\n", sample_type));
            goto_out_free(event, evsel, &mut sample_out, &mut sample_out_endian);
            return ret;
        }
    }

    ret = 0;
    goto_out_free(event, evsel, &mut sample_out, &mut sample_out_endian);
    if ret != 0 && read_format != 0 {
        pr_debug(format_args!("read_format {:#x}\n", read_format));
    }
    ret
}

unsafe fn goto_out_free(
    event: *mut perf_event,
    evsel: *mut evsel,
    sample_out: *mut perf_sample,
    sample_out_endian: *mut perf_sample,
) {
    free(event as *mut core::ffi::c_void);
    perf_sample__exit(sample_out_endian);
    perf_sample__exit(sample_out);
    evsel__put(evsel);
}

/**
 * test__sample_parsing - test sample parsing.
 *
 * This function implements a test that synthesizes a sample event, parses it
 * and then checks that the parsed sample matches the original sample.  The test
 * checks sample format bits separately and together.  If the test passes %0 is
 * returned, otherwise %-1 is returned.
 */
unsafe fn test__sample_parsing(
    _test: *mut test_suite,
    _subtest: i32,
) -> i32 {
    let rf: [u64; 15] = [4, 5, 6, 7, 12, 13, 14, 15, 20, 21, 22, 28, 29, 30, 31];
    let mut sample_type: u64;
    let mut sample_regs: u64;
    let mut i: usize;
    let mut err: i32;

    /*
     * Fail the test if it has not been updated when new sample format bits
     * were added.  Please actually update the test rather than just change
     * the condition below.
     */
    if PERF_SAMPLE_MAX > PERF_SAMPLE_WEIGHT_STRUCT << 1 {
        pr_debug(format_args!(
            "sample format has changed, some new PERF_SAMPLE_ bit was introduced - test needs updating\n"
        ));
        return -1;
    }

    /* Test each sample format bit separately */
    sample_type = 1;
    while sample_type != PERF_SAMPLE_MAX {
        /* Test read_format variations */
        if sample_type == PERF_SAMPLE_READ {
            i = 0;
            while i < rf.len() {
                err = do_test(sample_type, 0, rf[i]);
                if err != 0 {
                    return err;
                }
                i += 1;
            }
            sample_type <<= 1;
            continue;
        }
        sample_regs = 0;

        if sample_type == PERF_SAMPLE_REGS_USER {
            sample_regs = 0x3fff;
        }

        if sample_type == PERF_SAMPLE_REGS_INTR {
            sample_regs = 0xff0fff;
        }

        err = do_test(sample_type, sample_regs, 0);
        if err != 0 {
            return err;
        }
        sample_type <<= 1;
    }

    /*
     * Test all sample format bits together
     * Note: PERF_SAMPLE_WEIGHT and PERF_SAMPLE_WEIGHT_STRUCT cannot
     *       be set simultaneously.
     */
    sample_type = (PERF_SAMPLE_MAX - 1) & !PERF_SAMPLE_WEIGHT;
    sample_regs = 0x3fff; /* shared yb intr and user regs */
    i = 0;
    while i < rf.len() {
        err = do_test(sample_type, sample_regs, rf[i]);
        if err != 0 {
            return err;
        }
        i += 1;
    }
    sample_type = (PERF_SAMPLE_MAX - 1) & !PERF_SAMPLE_WEIGHT_STRUCT;
    i = 0;
    while i < rf.len() {
        err = do_test(sample_type, sample_regs, rf[i]);
        if err != 0 {
            return err;
        }
        i += 1;
    }

    0
}

DEFINE_SUITE!("Sample parsing", sample_parsing);
