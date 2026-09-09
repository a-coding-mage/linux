// SPDX-License-Identifier: GPL-2.0
/*
 * KCSAN short boot-time selftests.
 *
 * Copyright (C) 2019, Google LLC.
 */

// Dependencies supplied by the surrounding kernel translation unit.
use core::ffi::c_ulong;

const ITERS_PER_TEST: i32 = 2000;

/*
 * Test watchpoint encode and decode: check that encoding some access's info,
 * and then subsequent decode preserves the access's info.
 */
unsafe fn test_encode_decode() -> bool {
    let mut i: i32 = 0;
    while i < ITERS_PER_TEST {
        let size: usize = get_random_u32_inclusive(1, MAX_ENCODABLE_SIZE) as usize;
        let is_write: bool = get_random_u32_below(2) != 0;
        let mut verif_masked_addr: c_ulong;
        let encoded_watchpoint: i64;
        let mut verif_is_write: bool;
        let mut addr: c_ulong = 0;
        let mut verif_size: usize;

        get_random_bytes((&mut addr as *mut c_ulong).cast(), core::mem::size_of::<c_ulong>());
        if addr < PAGE_SIZE as c_ulong {
            addr = PAGE_SIZE as c_ulong;
        }

        if WARN_ON(!check_encodable(addr, size)) {
            return false;
        }

        encoded_watchpoint = encode_watchpoint(addr, size, is_write);

        /* Check special watchpoints */
        if WARN_ON(!decode_watchpoint(INVALID_WATCHPOINT, &mut verif_masked_addr, &mut verif_size, &mut verif_is_write)) {
            return false;
        }
        if WARN_ON(!decode_watchpoint(CONSUMED_WATCHPOINT, &mut verif_masked_addr, &mut verif_size, &mut verif_is_write)) {
            return false;
        }

        /* Check decoding watchpoint returns same data */
        if WARN_ON(!decode_watchpoint(encoded_watchpoint, &mut verif_masked_addr, &mut verif_size, &mut verif_is_write)) {
            return false;
        }
        if WARN_ON(verif_masked_addr != (addr & WATCHPOINT_ADDR_MASK)) {
            pr_err("{} fail: {} {} bytes @ {:x} -> encoded: {:x} -> {} {} bytes @ {:x}\n", "test_encode_decode", if is_write { "write" } else { "read" }, size, addr, encoded_watchpoint, if verif_is_write { "write" } else { "read" }, verif_size, verif_masked_addr);
            return false;
        }
        if WARN_ON(verif_size != size) {
            pr_err("{} fail: {} {} bytes @ {:x} -> encoded: {:x} -> {} {} bytes @ {:x}\n", "test_encode_decode", if is_write { "write" } else { "read" }, size, addr, encoded_watchpoint, if verif_is_write { "write" } else { "read" }, verif_size, verif_masked_addr);
            return false;
        }
        if WARN_ON(is_write != verif_is_write) {
            pr_err("{} fail: {} {} bytes @ {:x} -> encoded: {:x} -> {} {} bytes @ {:x}\n", "test_encode_decode", if is_write { "write" } else { "read" }, size, addr, encoded_watchpoint, if verif_is_write { "write" } else { "read" }, verif_size, verif_masked_addr);
            return false;
        }
        i += 1;
    }
    true
}

/* Test access matching function. */
unsafe fn test_matching_access() -> bool {
    if WARN_ON(!matching_access(10, 1, 10, 1)) { return false; }
    if WARN_ON(!matching_access(10, 2, 11, 1)) { return false; }
    if WARN_ON(!matching_access(10, 1, 9, 2)) { return false; }
    if WARN_ON(matching_access(10, 1, 11, 1)) { return false; }
    if WARN_ON(matching_access(9, 1, 10, 1)) { return false; }
    /* An access of size 0 could match another access, as demonstrated here.
     * Rather than add more comparisons to 'matching_access()', which would
     * end up in the fast-path for *all* checks, check_access() simply
     * returns for all accesses of size 0. */
    if WARN_ON(!matching_access(8, 8, 12, 0)) { return false; }
    true
}

/*
 * Correct memory barrier instrumentation is critical to avoiding false
 * positives: simple test to check at boot certain barriers are always properly
 * instrumented. See kcsan_test for a more complete test.
 */
static mut TEST_SPINLOCK: core::mem::MaybeUninit<spinlock_t> = core::mem::MaybeUninit::uninit();

unsafe fn test_barrier() -> bool {
    // CONFIG_KCSAN_WEAK_MEMORY selects the kernel's current reorder access;
    // this declaration is supplied by the surrounding kernel translation.
    let reorder_access: *mut kcsan_scoped_access = core::ptr::null_mut();
    let mut ret = true;
    let mut arch_spinlock: arch_spinlock_t = ARCH_SPIN_LOCK_UNLOCKED;
    let mut dummy: atomic_t = core::mem::zeroed();
    let mut test_var: i64 = 0;

    if reorder_access.is_null() || !IS_ENABLED_CONFIG_SMP {
        return true;
    }

    macro_rules! kcsan_check_barrier {
        ($access_type:expr, $barrier:expr, $name:expr) => {{
            (*reorder_access).type_ = ($access_type) | KCSAN_ACCESS_SCOPED;
            (*reorder_access).size = 1;
            $barrier;
            if (*reorder_access).size != 0 {
                pr_err!("improperly instrumented type={}: {}\n", $access_type, $name);
                ret = false;
            }
        }};
    }
    macro_rules! kcsan_check_read_barrier { ($b:expr) => { kcsan_check_barrier!(0, $b, stringify!($b)); }; }
    macro_rules! kcsan_check_write_barrier { ($b:expr) => { kcsan_check_barrier!(KCSAN_ACCESS_WRITE, $b, stringify!($b)); }; }
    macro_rules! kcsan_check_rw_barrier { ($b:expr) => { kcsan_check_barrier!(KCSAN_ACCESS_WRITE | KCSAN_ACCESS_COMPOUND, $b, stringify!($b)); }; }

    kcsan_nestable_atomic_begin();
    // The barrier invocation list is intentionally preserved from the C source.
    kcsan_check_read_barrier!(mb()); kcsan_check_read_barrier!(rmb()); kcsan_check_read_barrier!(smp_mb());
    kcsan_check_read_barrier!(smp_rmb()); kcsan_check_read_barrier!(dma_rmb());
    kcsan_check_read_barrier!(smp_mb__before_atomic()); kcsan_check_read_barrier!(smp_mb__after_atomic());
    kcsan_check_read_barrier!(smp_mb__after_spinlock()); kcsan_check_read_barrier!(smp_store_mb(test_var, 0));
    kcsan_check_read_barrier!(smp_store_release(&mut test_var, 0)); kcsan_check_read_barrier!(xchg(&mut test_var, 0));
    kcsan_check_read_barrier!(xchg_release(&mut test_var, 0)); kcsan_check_read_barrier!(cmpxchg(&mut test_var, 0, 0));
    kcsan_check_read_barrier!(cmpxchg_release(&mut test_var, 0, 0)); kcsan_check_read_barrier!(atomic_set_release(&mut dummy, 0));
    kcsan_check_read_barrier!(atomic_add_return(1, &mut dummy)); kcsan_check_read_barrier!(atomic_add_return_release(1, &mut dummy));
    kcsan_check_read_barrier!(atomic_fetch_add(1, &mut dummy)); kcsan_check_read_barrier!(atomic_fetch_add_release(1, &mut dummy));
    arch_spin_lock(&mut arch_spinlock); kcsan_check_read_barrier!(arch_spin_unlock(&mut arch_spinlock));
    spin_lock(&mut TEST_SPINLOCK); kcsan_check_read_barrier!(spin_unlock(&mut TEST_SPINLOCK));
    kcsan_check_write_barrier!(mb()); kcsan_check_write_barrier!(wmb()); kcsan_check_write_barrier!(smp_mb());
    kcsan_check_write_barrier!(smp_wmb()); kcsan_check_write_barrier!(dma_wmb());
    kcsan_check_write_barrier!(smp_mb__before_atomic()); kcsan_check_write_barrier!(smp_mb__after_atomic());
    kcsan_check_write_barrier!(smp_mb__after_spinlock()); kcsan_check_write_barrier!(smp_store_mb(test_var, 0));
    kcsan_check_write_barrier!(smp_store_release(&mut test_var, 0)); kcsan_check_write_barrier!(xchg(&mut test_var, 0));
    kcsan_check_write_barrier!(xchg_release(&mut test_var, 0)); kcsan_check_write_barrier!(cmpxchg(&mut test_var, 0, 0));
    kcsan_check_write_barrier!(cmpxchg_release(&mut test_var, 0, 0)); kcsan_check_write_barrier!(atomic_set_release(&mut dummy, 0));
    kcsan_check_write_barrier!(atomic_add_return(1, &mut dummy)); kcsan_check_write_barrier!(atomic_add_return_release(1, &mut dummy));
    kcsan_check_write_barrier!(atomic_fetch_add(1, &mut dummy)); kcsan_check_write_barrier!(atomic_fetch_add_release(1, &mut dummy));
    arch_spin_lock(&mut arch_spinlock); kcsan_check_write_barrier!(arch_spin_unlock(&mut arch_spinlock));
    spin_lock(&mut TEST_SPINLOCK); kcsan_check_write_barrier!(spin_unlock(&mut TEST_SPINLOCK));
    kcsan_check_rw_barrier!(mb()); kcsan_check_rw_barrier!(wmb()); kcsan_check_rw_barrier!(rmb());
    kcsan_check_rw_barrier!(smp_mb()); kcsan_check_rw_barrier!(smp_wmb()); kcsan_check_rw_barrier!(smp_rmb());
    kcsan_check_rw_barrier!(dma_wmb()); kcsan_check_rw_barrier!(dma_rmb());
    kcsan_check_rw_barrier!(smp_mb__before_atomic()); kcsan_check_rw_barrier!(smp_mb__after_atomic());
    kcsan_check_rw_barrier!(smp_mb__after_spinlock()); kcsan_check_rw_barrier!(smp_store_mb(test_var, 0));
    kcsan_check_rw_barrier!(smp_store_release(&mut test_var, 0)); kcsan_check_rw_barrier!(xchg(&mut test_var, 0));
    kcsan_check_rw_barrier!(xchg_release(&mut test_var, 0)); kcsan_check_rw_barrier!(cmpxchg(&mut test_var, 0, 0));
    kcsan_check_rw_barrier!(cmpxchg_release(&mut test_var, 0, 0)); kcsan_check_rw_barrier!(atomic_set_release(&mut dummy, 0));
    kcsan_check_rw_barrier!(atomic_add_return(1, &mut dummy)); kcsan_check_rw_barrier!(atomic_add_return_release(1, &mut dummy));
    kcsan_check_rw_barrier!(atomic_fetch_add(1, &mut dummy)); kcsan_check_rw_barrier!(atomic_fetch_add_release(1, &mut dummy));
    kcsan_check_rw_barrier!(xor_unlock_is_negative_byte(1, &mut test_var));
    kcsan_check_read_barrier!(xor_unlock_is_negative_byte(1, &mut test_var));
    kcsan_check_write_barrier!(xor_unlock_is_negative_byte(1, &mut test_var));
    kcsan_nestable_atomic_end();
    ret
}

unsafe fn kcsan_selftest() -> i32 {
    let mut passed = 0;
    let mut total = 0;
    macro_rules! run_test { ($test:ident) => {{ total += 1; if $test() { passed += 1; } else { pr_err!("selftest: {} failed", stringify!($test)); } }}; }
    run_test!(test_encode_decode);
    run_test!(test_matching_access);
    run_test!(test_barrier);
    pr_info!("selftest: {}/{} tests passed\n", passed, total);
    if passed != total { panic!("selftests failed"); }
    0
}

postcore_initcall!(kcsan_selftest);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
