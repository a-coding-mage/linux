// SPDX-License-Identifier: GPL-2.0
/*
 * Testsuite for NMI: IPIs
 *
 * Started by Don Zickus:
 * (using lib/locking-selftest.c as a guide)
 *
 *   Copyright (C) 2011 Red Hat, Inc., Don Zickus <dzickus@redhat.com>
 */

// Dependencies supplied by the surrounding kernel translation unit.

const SUCCESS: i32 = 0;
const FAILURE: i32 = 1;
const TIMEOUT: i32 = 2;

static mut nmi_fail: i32 = 0;

/* check to see if NMI IPIs work on this machine */
static mut nmi_ipi_mask: Bitmap = Bitmap::uninitialized();

static mut testcase_total: i32 = 0;
static mut testcase_successes: i32 = 0;
static mut unexpected_testcase_failures: i32 = 0;
static mut unexpected_testcase_unknowns: i32 = 0;

unsafe extern "C" {
    static mut cpu_online_mask: Cpumask;

    fn register_nmi_handler(
        ty: i32,
        callback: unsafe extern "C" fn(u32, *mut PtRegs) -> i32,
        flags: i32,
        name: *const u8,
        initdata: *const u8,
    ) -> i32;
    fn unregister_nmi_handler(ty: i32, name: *const u8);
    fn raw_smp_processor_id() -> i32;
    fn smp_processor_id() -> i32;
    fn cpumask_test_and_clear_cpu(cpu: i32, mask: *mut Cpumask) -> bool;
    fn cpumask_empty(mask: *const Cpumask) -> bool;
    fn cpumask_copy(dst: *mut Cpumask, src: *const Cpumask);
    fn cpumask_clear_cpu(cpu: i32, mask: *mut Cpumask);
    fn cpumask_clear(mask: *mut Cpumask);
    fn cpumask_set_cpu(cpu: i32, mask: *mut Cpumask);
    fn __apic_send_IPI_mask(mask: *const Cpumask, vector: u8);
    fn wmb();
    fn udelay(usecs: u32);
    fn dump_stack();
    fn pr_cont(fmt: *const u8, ...);
    fn pr_info(fmt: *const u8, ...);
}

#[repr(C)]
struct Bitmap {
    bits: [usize; 1],
}

impl Bitmap {
    const fn uninitialized() -> Self {
        Self { bits: [0] }
    }
}

#[repr(C)]
struct Cpumask {
    bits: [usize; 1],
}

#[repr(C)]
struct PtRegs {
    _private: [u8; 0],
}

const NMI_UNKNOWN: i32 = 0;
const NMI_LOCAL: i32 = 1;
const NMI_HANDLED: i32 = 1;
const NMI_DONE: i32 = 0;
const NMI_FLAG_FIRST: i32 = 1;
const NMI_VECTOR: u8 = 2;
const USEC_PER_SEC: usize = 1_000_000;

unsafe extern "C" fn nmi_unk_cb(_val: u32, _regs: *mut PtRegs) -> i32 {
    unexpected_testcase_unknowns += 1;
    NMI_HANDLED
}

unsafe fn init_nmi_testsuite() {
    /* trap all the unknown NMIs we may generate */
    register_nmi_handler(
        NMI_UNKNOWN,
        nmi_unk_cb,
        0,
        b"nmi_selftest_unk\0".as_ptr(),
        b"__initdata\0".as_ptr(),
    );
}

unsafe fn cleanup_nmi_testsuite() {
    unregister_nmi_handler(NMI_UNKNOWN, b"nmi_selftest_unk\0".as_ptr());
}

unsafe extern "C" fn test_nmi_ipi_callback(_val: u32, _regs: *mut PtRegs) -> i32 {
    let cpu = raw_smp_processor_id();

    if cpumask_test_and_clear_cpu(cpu, &mut nmi_ipi_mask as *mut _ as *mut Cpumask) {
        return NMI_HANDLED;
    }

    NMI_DONE
}

unsafe fn test_nmi_ipi(mask: *mut Cpumask) {
    let mut timeout: usize;

    if register_nmi_handler(
        NMI_LOCAL,
        test_nmi_ipi_callback,
        NMI_FLAG_FIRST,
        b"nmi_selftest\0".as_ptr(),
        b"__initdata\0".as_ptr(),
    ) != 0 {
        nmi_fail = FAILURE;
        return;
    }

    /* sync above data before sending NMI */
    wmb();

    __apic_send_IPI_mask(mask, NMI_VECTOR);

    /* Don't wait longer than a second */
    timeout = USEC_PER_SEC;
    while !cpumask_empty(mask) && {
        timeout -= 1;
        timeout != 0
    } {
        udelay(1);
    }

    /* What happens if we timeout, do we still unregister?? */
    unregister_nmi_handler(NMI_LOCAL, b"nmi_selftest\0".as_ptr());

    if timeout == 0 {
        nmi_fail = TIMEOUT;
    }
}

unsafe fn remote_ipi() {
    cpumask_copy(&mut nmi_ipi_mask as *mut _ as *mut Cpumask, &cpu_online_mask);
    cpumask_clear_cpu(smp_processor_id(), &mut nmi_ipi_mask as *mut _ as *mut Cpumask);
    let mask = &mut nmi_ipi_mask as *mut _ as *mut Cpumask;
    if !cpumask_empty(mask) {
        test_nmi_ipi(mask);
    }
}

unsafe fn local_ipi() {
    let mask = &mut nmi_ipi_mask as *mut _ as *mut Cpumask;
    cpumask_clear(mask);
    cpumask_set_cpu(smp_processor_id(), mask);
    test_nmi_ipi(mask);
}

unsafe fn reset_nmi() {
    nmi_fail = 0;
}

unsafe fn dotest(testcase_fn: unsafe fn(), expected: i32) {
    testcase_fn();
    /*
     * Filter out expected failures:
     */
    if nmi_fail != expected {
        unexpected_testcase_failures += 1;

        if nmi_fail == FAILURE {
            pr_cont(b"FAILED |\0".as_ptr());
        } else if nmi_fail == TIMEOUT {
            pr_cont(b"TIMEOUT|\0".as_ptr());
        } else {
            pr_cont(b"ERROR  |\0".as_ptr());
        }
        dump_stack();
    } else {
        testcase_successes += 1;
        pr_cont(b"  ok  |\0".as_ptr());
    }
    pr_cont(b"\n\0".as_ptr());

    testcase_total += 1;
    reset_nmi();
}

pub unsafe fn nmi_selftest() {
    init_nmi_testsuite();

    /*
     * Run the testsuite:
     */
    pr_info(b"----------------\n\0".as_ptr());
    pr_info(b"| NMI testsuite:\n\0".as_ptr());
    pr_info(b"--------------------\n\0".as_ptr());

    pr_info(b"%12s:\0".as_ptr(), b"remote IPI\0".as_ptr());
    dotest(remote_ipi, SUCCESS);

    pr_info(b"%12s:\0".as_ptr(), b"local IPI\0".as_ptr());
    dotest(local_ipi, SUCCESS);

    cleanup_nmi_testsuite();

    pr_info(b"--------------------\n\0".as_ptr());
    if unexpected_testcase_failures != 0 {
        pr_info(
            b"BUG: %3d unexpected failures (out of %3d) - debugging disabled! |\n\0".as_ptr(),
            unexpected_testcase_failures,
            testcase_total,
        );
    } else {
        pr_info(
            b"Good, all %3d testcases passed! |\n\0".as_ptr(),
            testcase_successes,
        );
    }
    pr_info(b"-----------------------------------------------------------------\n\0".as_ptr());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
