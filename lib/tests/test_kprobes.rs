// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * test_kprobes.c - simple sanity test for k*probes
 *
 * Copyright IBM Corp. 2008
 */

// Dependencies supplied by the kernel and KUnit bindings.

const DIV_FACTOR: u32 = 3;

static mut rand1: u32 = 0;
static mut preh_val: u32 = 0;
static mut posth_val: u32 = 0;
static mut target: Option<unsafe extern "C" fn(u32) -> u32> = None;
static mut recursed_target: Option<unsafe extern "C" fn(u32) -> u32> = None;
static mut target2: Option<unsafe extern "C" fn(u32) -> u32> = None;
static mut current_test: *mut kunit = core::ptr::null_mut();

static mut internal_target: Option<unsafe extern "C" fn() -> usize> = None;
static mut stacktrace_target: Option<unsafe extern "C" fn() -> usize> = None;
static mut stacktrace_driver: Option<unsafe extern "C" fn() -> usize> = None;
static mut target_return_address: [usize; 2] = [0; 2];

unsafe extern "C" fn kprobe_target(value: u32) -> u32 { value / DIV_FACTOR }

unsafe extern "C" fn kprobe_recursed_target(value: u32) -> u32 { value / DIV_FACTOR }

unsafe extern "C" fn kp_pre_handler(_p: *mut kprobe, _regs: *mut pt_regs) -> i32 {
    // KUNIT_EXPECT_FALSE(current_test, preemptible());
    preh_val = recursed_target.expect("recursed_target")(rand1);
    0
}

unsafe extern "C" fn kp_post_handler(_p: *mut kprobe, _regs: *mut pt_regs, _flags: usize) {
    let expval = recursed_target.expect("recursed_target")(rand1);
    // KUNIT_EXPECT_FALSE(current_test, preemptible());
    // KUNIT_EXPECT_EQ(current_test, preh_val, expval);
    posth_val = preh_val + DIV_FACTOR;
}

static mut kp: kprobe = kprobe {
    symbol_name: "kprobe_target",
    pre_handler: Some(kp_pre_handler),
    post_handler: Some(kp_post_handler),
};

unsafe extern "C" fn test_kprobe(test: *mut kunit) {
    current_test = test;
    // KUNIT_EXPECT_EQ(test, 0, register_kprobe(&kp));
    target.expect("target")(rand1);
    // unregister_kprobe(&kp);
    // KUNIT_EXPECT_NE(test, 0, preh_val);
    // KUNIT_EXPECT_NE(test, 0, posth_val);
}

unsafe extern "C" fn kprobe_target2(value: u32) -> u32 { (value / DIV_FACTOR) + 1 }

unsafe extern "C" fn kprobe_stacktrace_internal_target() -> usize {
    if target_return_address[0] == 0 { target_return_address[0] = core::ptr::addr_of!(kprobe_stacktrace_internal_target) as usize; }
    target_return_address[0]
}

unsafe extern "C" fn kprobe_stacktrace_target() -> usize {
    if target_return_address[1] == 0 { target_return_address[1] = core::ptr::addr_of!(kprobe_stacktrace_target) as usize; }
    if let Some(f) = internal_target { f(); }
    target_return_address[1]
}

unsafe extern "C" fn kprobe_stacktrace_driver() -> usize {
    if let Some(f) = stacktrace_target { f(); }
    core::ptr::addr_of!(kprobe_stacktrace_driver) as usize
}

unsafe extern "C" fn kp_pre_handler2(_p: *mut kprobe, _regs: *mut pt_regs) -> i32 {
    preh_val = (rand1 / DIV_FACTOR) + 1; 0
}

unsafe extern "C" fn kp_post_handler2(_p: *mut kprobe, _regs: *mut pt_regs, _flags: usize) {
    // KUNIT_EXPECT_EQ(current_test, preh_val, (rand1 / div_factor) + 1);
    posth_val = preh_val + DIV_FACTOR;
}

static mut kp2: kprobe = kprobe { symbol_name: "kprobe_target2", pre_handler: Some(kp_pre_handler2), post_handler: Some(kp_post_handler2) };

unsafe extern "C" fn test_kprobes(test: *mut kunit) {
    let kps: [*mut kprobe; 2] = [&raw mut kp, &raw mut kp2];
    current_test = test;
    // KUNIT_EXPECT_EQ(test, 0, register_kprobes(kps, 2));
    preh_val = 0; posth_val = 0; target.expect("target")(rand1);
    // KUNIT_EXPECT_NE(test, 0, preh_val); KUNIT_EXPECT_NE(test, 0, posth_val);
    preh_val = 0; posth_val = 0; target2.expect("target2")(rand1);
    // KUNIT_EXPECT_NE(test, 0, preh_val); KUNIT_EXPECT_NE(test, 0, posth_val);
    // unregister_kprobes(kps, 2);
}

static mut kp_missed: kprobe = kprobe { symbol_name: "kprobe_recursed_target", pre_handler: Some(kp_pre_handler), post_handler: Some(kp_post_handler) };

unsafe extern "C" fn test_kprobe_missed(test: *mut kunit) {
    current_test = test; preh_val = 0; posth_val = 0;
    // KUNIT_EXPECT_EQ(test, 0, register_kprobe(&kp_missed));
    recursed_target.expect("recursed_target")(rand1);
    // KUNIT_EXPECT_EQ(test, 2, kp_missed.nmissed);
    // KUNIT_EXPECT_NE(test, 0, preh_val); KUNIT_EXPECT_NE(test, 0, posth_val);
    // unregister_kprobe(&kp_missed);
}

// CONFIG_KRETPROBES
static mut krph_val: u32 = 0;

unsafe extern "C" fn entry_handler(_ri: *mut kretprobe_instance, _regs: *mut pt_regs) -> i32 {
    // KUNIT_EXPECT_FALSE(current_test, preemptible());
    krph_val = rand1 / DIV_FACTOR; 0
}
unsafe extern "C" fn return_handler(_ri: *mut kretprobe_instance, regs: *mut pt_regs) -> i32 {
    let ret = regs_return_value(regs);
    // KUNIT_EXPECT_FALSE(current_test, preemptible());
    // KUNIT_EXPECT_EQ(current_test, ret, rand1 / div_factor);
    // KUNIT_EXPECT_NE(current_test, krph_val, 0);
    krph_val = rand1; 0
}
unsafe extern "C" fn return_handler2(_ri: *mut kretprobe_instance, regs: *mut pt_regs) -> i32 {
    let ret = regs_return_value(regs);
    // KUNIT_EXPECT_EQ(current_test, ret, (rand1 / div_factor) + 1);
    // KUNIT_EXPECT_NE(current_test, krph_val, 0);
    krph_val = rand1; 0
}
static mut rp: kretprobe = kretprobe::default();
static mut rp2: kretprobe = kretprobe::default();
unsafe extern "C" fn test_kretprobe(test: *mut kunit) {
    current_test = test; // KUNIT_EXPECT_EQ(test, 0, register_kretprobe(&rp));
    target.expect("target")(rand1); // unregister_kretprobe(&rp);
    // KUNIT_EXPECT_EQ(test, krph_val, rand1);
}
unsafe extern "C" fn test_kretprobes(test: *mut kunit) {
    current_test = test; // KUNIT_EXPECT_EQ(test, 0, register_kretprobes([&rp, &rp2], 2));
    krph_val = 0; target.expect("target")(rand1); // KUNIT_EXPECT_EQ(test, krph_val, rand1);
    krph_val = 0; target2.expect("target2")(rand1); // KUNIT_EXPECT_EQ(test, krph_val, rand1);
    // unregister_kretprobes([&rp, &rp2], 2);
}

// CONFIG_ARCH_CORRECT_STACKTRACE_ON_KRETPROBE
const STACK_BUF_SIZE: usize = 16;
static mut stack_buf: [usize; STACK_BUF_SIZE] = [0; STACK_BUF_SIZE];
unsafe extern "C" fn stacktrace_return_handler(_ri: *mut kretprobe_instance, regs: *mut pt_regs) -> i32 {
    let retval = regs_return_value(regs);
    // KUNIT_EXPECT_FALSE(current_test, preemptible());
    // KUNIT_EXPECT_EQ(current_test, retval, target_return_address[1]);
    let ret = stack_trace_save(stack_buf.as_mut_ptr(), STACK_BUF_SIZE, 0);
    // KUNIT_EXPECT_NE(current_test, ret, 0);
    let mut i = 0; while i < ret { if stack_buf[i] == target_return_address[1] { break; } i += 1; }
    // KUNIT_EXPECT_NE(current_test, i, ret);
    0
}
static mut rp3: kretprobe = kretprobe::default();
static mut rp4: kretprobe = kretprobe::default();
unsafe extern "C" fn test_stacktrace_on_kretprobe(test: *mut kunit) {
    let myretaddr = kprobe_stacktrace_driver(); current_test = test;
    // KUNIT_ASSERT_NE(test, myretaddr, stacktrace_driver());
    // KUNIT_ASSERT_EQ(test, 0, register_kretprobe(&rp3));
    // KUNIT_ASSERT_NE(test, myretaddr, stacktrace_driver()); unregister_kretprobe(&rp3);
}
unsafe extern "C" fn stacktrace_internal_return_handler(_ri: *mut kretprobe_instance, regs: *mut pt_regs) -> i32 {
    let retval = regs_return_value(regs);
    // KUNIT_EXPECT_FALSE(current_test, preemptible());
    // KUNIT_EXPECT_EQ(current_test, retval, target_return_address[0]);
    let ret = stack_trace_save(stack_buf.as_mut_ptr(), STACK_BUF_SIZE, 0);
    // KUNIT_EXPECT_NE(current_test, ret, 0);
    let mut i = 0; while i < ret - 1 { if stack_buf[i] == target_return_address[0] { /* KUNIT_EXPECT_EQ(stack_buf[i + 1], target_return_address[1]); */ break; } i += 1; }
    // KUNIT_EXPECT_NE(current_test, i, ret);
    0
}
unsafe extern "C" fn test_stacktrace_on_nested_kretprobe(test: *mut kunit) {
    let myretaddr = kprobe_stacktrace_driver(); current_test = test;
    // KUNIT_ASSERT_EQ(test, 0, register_kretprobes([&rp3, &rp4], 2));
    // KUNIT_ASSERT_NE(test, myretaddr, stacktrace_driver()); unregister_kretprobes([&rp3, &rp4], 2);
}

unsafe extern "C" fn kprobes_test_init(_test: *mut kunit) -> i32 {
    kp = kprobe::default(); kp2 = kprobe::default(); kp_missed = kprobe::default();
    target = Some(kprobe_target); target2 = Some(kprobe_target2); recursed_target = Some(kprobe_recursed_target);
    stacktrace_target = Some(kprobe_stacktrace_target); internal_target = Some(kprobe_stacktrace_internal_target);
    stacktrace_driver = Some(kprobe_stacktrace_driver);
    // rand1 = get_random_u32_above(div_factor);
    0
}

// KUnit case and suite registration:
// KUNIT_CASE(test_kprobe), KUNIT_CASE(test_kprobes), KUNIT_CASE(test_kprobe_missed),
// plus CONFIG_KRETPROBES and CONFIG_ARCH_CORRECT_STACKTRACE_ON_KRETPROBE cases.
// kunit_test_suites(&kprobes_test_suite);
// MODULE_DESCRIPTION("simple sanity test for k*probes");
// MODULE_LICENSE("GPL");

// Types and kernel functions below are supplied by the translated kernel headers.
extern "C" {
    type kunit;
    type pt_regs;
    type kprobe;
    type kretprobe;
    type kretprobe_instance;
    fn regs_return_value(regs: *mut pt_regs) -> usize;
    fn stack_trace_save(entries: *mut usize, size: usize, skipnr: usize) -> usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
