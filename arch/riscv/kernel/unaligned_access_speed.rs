// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2024 Rivos Inc.
 */

// Linux kernel and RISC-V dependencies supplied by other translation units.

const MISALIGNED_ACCESS_NS: u64 = 8_000_000;
const MISALIGNED_BUFFER_SIZE: usize = 0x4000;
const MISALIGNED_COPY_SIZE: usize = (MISALIGNED_BUFFER_SIZE / 2) - 0x80;

static mut MISALIGNED_ACCESS_SPEED: isize = RISCV_HWPROBE_MISALIGNED_SCALAR_UNKNOWN as isize;
static mut VECTOR_MISALIGNED_ACCESS: isize = RISCV_HWPROBE_MISALIGNED_VECTOR_UNSUPPORTED as isize;

static mut UNALIGNED_SCALAR_SPEED_PARAM: isize = RISCV_HWPROBE_MISALIGNED_SCALAR_UNKNOWN as isize;
static mut UNALIGNED_VECTOR_SPEED_PARAM: isize = RISCV_HWPROBE_MISALIGNED_VECTOR_UNKNOWN as isize;

unsafe fn measure_cycles(
    func: unsafe extern "C" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize),
    dst: *mut core::ffi::c_void,
    src: *mut core::ffi::c_void,
    len: usize,
) -> u64 {
    let mut start_cycles: u64;
    let mut end_cycles: u64;
    let mut cycles = u64::MAX;
    let start_ns: u64;

    // Do a warmup.
    func(dst, src, len);

    preempt_disable();

    /*
     * For a fixed amount of time, repeatedly try the function, and take
     * the best time in cycles as the measurement.
     */
    start_ns = ktime_get_mono_fast_ns();
    while ktime_get_mono_fast_ns() < start_ns + MISALIGNED_ACCESS_NS {
        start_cycles = get_cycles64();
        // Ensure the CSR read can't reorder WRT to the copy.
        mb();
        func(dst, src, len);
        // Ensure the copy ends before the end time is snapped.
        mb();
        end_cycles = get_cycles64();
        if end_cycles.wrapping_sub(start_cycles) < cycles {
            cycles = end_cycles.wrapping_sub(start_cycles);
        }
    }

    preempt_enable();
    cycles
}

/*
 * Return:
 *     1 if unaligned accesses are fast
 *     0 if unaligned accesses are slow
 *    -1 if check cannot be done
 */
unsafe fn compare_unaligned_access(
    word_copy: unsafe extern "C" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize),
    byte_copy: unsafe extern "C" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize),
    buf: *mut core::ffi::c_void,
    kind: *const core::ffi::c_char,
) -> i32 {
    let cpu = smp_processor_id();
    let dst = (buf as usize | 0x1) as *mut core::ffi::c_void;
    // Unalign src as well, but differently (off by 1 + 2 = 3).
    let src = (dst as usize + MISALIGNED_BUFFER_SIZE / 2 + 2) as *mut core::ffi::c_void;

    let word_cycles = measure_cycles(word_copy, dst, src, MISALIGNED_COPY_SIZE);
    let byte_cycles = measure_cycles(byte_copy, dst, src, MISALIGNED_COPY_SIZE);

    // Don't divide by zero.
    if word_cycles == 0 || byte_cycles == 0 {
        pr_warn("cpu%d: rdtime lacks granularity needed to measure %s unaligned access speed\n", cpu, kind);
        return -1;
    }

    let fast = word_cycles < byte_cycles;
    let ratio = div_u64(byte_cycles.wrapping_mul(100), word_cycles);
    pr_info("cpu%d: %s unaligned word access speed is %d.%02dx byte access speed (%s)\n",
            cpu, kind, ratio / 100, ratio % 100, if fast { "fast" } else { "slow" });
    fast as i32
}

#[cfg(CONFIG_RISCV_PROBE_UNALIGNED_ACCESS)]
unsafe fn check_unaligned_access(page: *mut page) -> i32 {
    let buf = page_address(page);
    let cpu = smp_processor_id();
    if per_cpu(MISALIGNED_ACCESS_SPEED, cpu) != RISCV_HWPROBE_MISALIGNED_SCALAR_UNKNOWN {
        return 0;
    }
    let ret = compare_unaligned_access(__riscv_copy_words_unaligned, __riscv_copy_bytes_unaligned, buf, "scalar");
    if ret < 0 { return 0; }
    per_cpu(MISALIGNED_ACCESS_SPEED, cpu) = if ret != 0 {
        RISCV_HWPROBE_MISALIGNED_SCALAR_FAST
    } else { RISCV_HWPROBE_MISALIGNED_SCALAR_SLOW };
    0
}

#[cfg(CONFIG_RISCV_PROBE_UNALIGNED_ACCESS)]
unsafe fn check_unaligned_access_speed_all_cpus() {
    let cpu_count = num_possible_cpus();
    let bufs = kzalloc_objs::<*mut page>(cpu_count);
    if bufs.is_null() { pr_warn("Allocation failure, not measuring misaligned performance\n"); return; }
    for_each_cpu!(cpu, cpu_online_mask) {
        *bufs.add(cpu) = alloc_pages(GFP_KERNEL, get_order(MISALIGNED_BUFFER_SIZE));
        if (*bufs.add(cpu)).is_null() { pr_warn("Allocation failure, not measuring misaligned performance\n"); break; }
    }
    on_each_cpu(_check_unaligned_access, bufs, 1);
    for_each_cpu!(cpu, cpu_online_mask) {
        if !(*bufs.add(cpu)).is_null() { __free_pages(*bufs.add(cpu), get_order(MISALIGNED_BUFFER_SIZE)); }
    }
    kfree(bufs);
}

#[cfg(not(CONFIG_RISCV_PROBE_UNALIGNED_ACCESS))]
unsafe fn check_unaligned_access_speed_all_cpus() {}

static mut FAST_UNALIGNED_ACCESS_SPEED_KEY: bool = false;

unsafe fn modify_unaligned_access_branches(mask: *const cpumask_t) {
    let mut fast = true;
    for_each_cpu!(cpu, mask) {
        if per_cpu(MISALIGNED_ACCESS_SPEED, cpu) != RISCV_HWPROBE_MISALIGNED_SCALAR_FAST { fast = false; break; }
    }
    if fast { static_branch_enable_cpuslocked(&mut FAST_UNALIGNED_ACCESS_SPEED_KEY); }
    else { static_branch_disable_cpuslocked(&mut FAST_UNALIGNED_ACCESS_SPEED_KEY); }
}

unsafe fn riscv_online_cpu(cpu: u32) -> i32 {
    let ret = cpu_online_unaligned_access_init(cpu);
    if ret != 0 { return ret; }
    if per_cpu(MISALIGNED_ACCESS_SPEED, cpu) == RISCV_HWPROBE_MISALIGNED_SCALAR_UNKNOWN {
        if UNALIGNED_SCALAR_SPEED_PARAM != RISCV_HWPROBE_MISALIGNED_SCALAR_UNKNOWN {
            per_cpu(MISALIGNED_ACCESS_SPEED, cpu) = UNALIGNED_SCALAR_SPEED_PARAM;
        } else {
            let buf = alloc_pages(GFP_KERNEL, get_order(MISALIGNED_BUFFER_SIZE));
            if buf.is_null() { pr_warn("Allocation failure, not measuring misaligned performance\n"); return -ENOMEM; }
            check_unaligned_access(buf);
            __free_pages(buf, get_order(MISALIGNED_BUFFER_SIZE));
        }
    }
    modify_unaligned_access_branches(cpu_online_mask);
    0
}

unsafe fn riscv_offline_cpu(cpu: u32) -> i32 {
    let mut mask = core::mem::MaybeUninit::<cpumask_t>::uninit();
    cpumask_copy(mask.as_mut_ptr(), cpu_online_mask);
    cpumask_clear_cpu(cpu, mask.as_mut_ptr());
    modify_unaligned_access_branches(mask.as_ptr());
    0
}

// CONFIG_RISCV_PROBE_VECTOR_UNALIGNED_ACCESS conditionally supplies the vector probe body.
unsafe fn check_vector_unaligned_access(_work: *mut work_struct) {}

unsafe fn riscv_online_cpu_vec(cpu: u32) -> i32 {
    if UNALIGNED_VECTOR_SPEED_PARAM != RISCV_HWPROBE_MISALIGNED_VECTOR_UNKNOWN {
        per_cpu(VECTOR_MISALIGNED_ACCESS, cpu) = UNALIGNED_VECTOR_SPEED_PARAM;
    }
    0
}

static SPEED_STR: [Option<&str>; 5] = [None, None, Some("slow"), Some("fast"), Some("unsupported")];

unsafe fn set_unaligned_scalar_speed_param(str_: *const core::ffi::c_char) -> i32 {
    if strcmp(str_, SPEED_STR[RISCV_HWPROBE_MISALIGNED_SCALAR_SLOW as usize]) == 0 { UNALIGNED_SCALAR_SPEED_PARAM = RISCV_HWPROBE_MISALIGNED_SCALAR_SLOW; }
    else if strcmp(str_, SPEED_STR[RISCV_HWPROBE_MISALIGNED_SCALAR_FAST as usize]) == 0 { UNALIGNED_SCALAR_SPEED_PARAM = RISCV_HWPROBE_MISALIGNED_SCALAR_FAST; }
    else if strcmp(str_, SPEED_STR[RISCV_HWPROBE_MISALIGNED_SCALAR_UNSUPPORTED as usize]) == 0 { UNALIGNED_SCALAR_SPEED_PARAM = RISCV_HWPROBE_MISALIGNED_SCALAR_UNSUPPORTED; }
    else { return -EINVAL; }
    1
}

unsafe fn set_unaligned_vector_speed_param(str_: *const core::ffi::c_char) -> i32 {
    if strcmp(str_, SPEED_STR[RISCV_HWPROBE_MISALIGNED_VECTOR_SLOW as usize]) == 0 { UNALIGNED_VECTOR_SPEED_PARAM = RISCV_HWPROBE_MISALIGNED_VECTOR_SLOW; }
    else if strcmp(str_, SPEED_STR[RISCV_HWPROBE_MISALIGNED_VECTOR_FAST as usize]) == 0 { UNALIGNED_VECTOR_SPEED_PARAM = RISCV_HWPROBE_MISALIGNED_VECTOR_FAST; }
    else if strcmp(str_, SPEED_STR[RISCV_HWPROBE_MISALIGNED_VECTOR_UNSUPPORTED as usize]) == 0 { UNALIGNED_VECTOR_SPEED_PARAM = RISCV_HWPROBE_MISALIGNED_VECTOR_UNSUPPORTED; }
    else { return -EINVAL; }
    1
}

unsafe fn check_unaligned_access_all_cpus() -> i32 {
    unaligned_access_init();
    // Command-line parameter handling and emulation probes are supplied by kernel dependencies.
    check_unaligned_access_speed_all_cpus();
    cpuhp_setup_state_nocalls(CPUHP_AP_ONLINE_DYN, "riscv:online", riscv_online_cpu, Some(riscv_offline_cpu));
    cpuhp_setup_state_nocalls(CPUHP_AP_ONLINE_DYN, "riscv:online", riscv_online_cpu_vec, None);
    cpus_read_lock();
    modify_unaligned_access_branches(cpu_online_mask);
    cpus_read_unlock();
    0
}

// Run after clocksource_done_booting() so measure_cycles() uses a stable clocksource,
// but before rootfs_initcall() enables usermode helpers.
fs_initcall_sync!(check_unaligned_access_all_cpus);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
