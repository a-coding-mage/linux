/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of hash-test-template.h. Included symbols and macros are
 * supplied by the corresponding hash test suite. */

unsafe fn test_hash_test_vectors(test: *mut kunit) {
    let max_len: usize = 16384;
    let data: *mut u8 = alloc_buf(test, max_len);
    for i in 0..ARRAY_SIZE!(hash_testvecs) {
        let data_len = hash_testvecs[i].data_len;
        let mut actual_hash = [0u8; HASH_SIZE];
        KUNIT_ASSERT_LE!(test, data_len, max_len);
        rand_bytes_seeded_from_len(data, data_len);
        HASH!(data, data_len, actual_hash.as_mut_ptr());
        KUNIT_ASSERT_MEMEQ_MSG!(test, actual_hash.as_ptr(), hash_testvecs[i].digest,
            HASH_SIZE, "Wrong result with test vector %zu; data_len=%zu", i, data_len);
    }
}

unsafe fn test_hash_all_lens_up_to_4096(test: *mut kunit) {
    let max_len: usize = 4096;
    let data = alloc_buf(test, max_len);
    let mut ctx: HASH_CTX = core::mem::zeroed();
    let mut hash = [0u8; HASH_SIZE];
    rand_bytes_seeded_from_len(data, max_len);
    HASH_INIT!(&mut ctx);
    for len in 0..=max_len {
        HASH!(data, len, hash.as_mut_ptr());
        HASH_UPDATE!(&mut ctx, hash.as_ptr(), HASH_SIZE);
    }
    HASH_FINAL!(&mut ctx, hash.as_mut_ptr());
    KUNIT_ASSERT_MEMEQ!(test, hash.as_ptr(), hash_testvec_consolidated, HASH_SIZE);
}

unsafe fn test_hash_incremental_updates(test: *mut kunit) {
    let max_len: usize = 16384;
    let data = alloc_guarded_buf(test, max_len);
    for _ in 0..1000 {
        let total_len = rand_length(max_len);
        let offset = rand_offset(max_len - total_len);
        let mut ctx: HASH_CTX = core::mem::zeroed();
        let mut hash1 = [0u8; HASH_SIZE];
        let mut hash2 = [0u8; HASH_SIZE];
        let mut num_parts = 0usize;
        rand_bytes(data.add(offset), total_len);
        HASH!(data.add(offset), total_len, hash1.as_mut_ptr());
        HASH_INIT!(&mut ctx);
        let mut remaining_len = total_len;
        let mut cur_offset = offset;
        while rand_bool() {
            let part_len = rand_length(remaining_len);
            HASH_UPDATE!(&mut ctx, data.add(cur_offset), part_len);
            num_parts += 1; cur_offset += part_len; remaining_len -= part_len;
        }
        if remaining_len != 0 || rand_bool() {
            HASH_UPDATE!(&mut ctx, data.add(cur_offset), remaining_len);
            num_parts += 1;
        }
        HASH_FINAL!(&mut ctx, hash2.as_mut_ptr());
        KUNIT_ASSERT_MEMEQ_MSG!(test, hash1.as_ptr(), hash2.as_ptr(), HASH_SIZE,
            "Incremental test failed with total_len=%zu num_parts=%zu offset=%zu",
            total_len, num_parts, offset);
    }
}

unsafe fn test_hash_buffer_overruns(test: *mut kunit) {
    let buf_len: usize = 16384;
    let buf = alloc_guarded_buf(test, buf_len);
    let buf_end = buf.add(buf_len);
    let max_tested_len = buf_len - core::mem::size_of::<HASH_CTX>();
    let guarded_ctx = (buf_end as *mut u8).sub(core::mem::size_of::<HASH_CTX>()) as *mut HASH_CTX;
    rand_bytes(buf, buf_len);
    for _ in 0..100 {
        let len = rand_length(max_tested_len);
        let mut ctx: HASH_CTX = core::mem::zeroed();
        let mut hash = [0u8; HASH_SIZE];
        HASH!(buf_end.sub(len), len, hash.as_mut_ptr());
        HASH_INIT!(&mut ctx); HASH_UPDATE!(&mut ctx, buf_end.sub(len), len);
        HASH_FINAL!(&mut ctx, hash.as_mut_ptr());
        HASH!(buf, len, buf_end.sub(HASH_SIZE));
        HASH_INIT!(&mut ctx); HASH_UPDATE!(&mut ctx, buf, len);
        HASH_FINAL!(&mut ctx, buf_end.sub(HASH_SIZE));
        HASH_INIT!(guarded_ctx); HASH_UPDATE!(guarded_ctx, buf, len);
        HASH_FINAL!(guarded_ctx, hash.as_mut_ptr());
    }
}

unsafe fn test_hash_overlaps(test: *mut kunit) {
    let buf_len: usize = 16384;
    let buf = alloc_guarded_buf(test, buf_len);
    let max_tested_len = buf_len - HASH_SIZE;
    let mut ctx: HASH_CTX = core::mem::zeroed();
    let mut hash = [0u8; HASH_SIZE];
    rand_bytes(buf, buf_len);
    for _ in 0..100 {
        let len = rand_length(max_tested_len);
        let offset = HASH_SIZE + rand_offset(max_tested_len - len);
        let left_end = rand_bool();
        let ovl_hash = if left_end { buf.add(offset) } else { buf.add(offset + len - HASH_SIZE) };
        HASH!(buf.add(offset), len, hash.as_mut_ptr()); HASH!(buf.add(offset), len, ovl_hash);
        KUNIT_ASSERT_MEMEQ_MSG!(test, hash.as_ptr(), ovl_hash, HASH_SIZE,
            "Overlap test 1 failed with len=%zu offset=%zu left_end=%d", len, offset, left_end);
        HASH!(buf.add(offset), len, hash.as_mut_ptr()); HASH_INIT!(&mut ctx);
        HASH_UPDATE!(&mut ctx, buf.add(offset), len); HASH_FINAL!(&mut ctx, ovl_hash);
        KUNIT_ASSERT_MEMEQ_MSG!(test, hash.as_ptr(), ovl_hash, HASH_SIZE,
            "Overlap test 2 failed with len=%zu offset=%zu left_end=%d", len, offset, left_end);
        HASH!(buf.add(offset), len, hash.as_mut_ptr()); HASH_INIT!(&mut ctx);
        HASH_UPDATE!(&mut ctx, buf.add(offset), len); rand_bytes(buf.add(offset), len);
        HASH_FINAL!(&mut ctx, ovl_hash);
        KUNIT_ASSERT_MEMEQ_MSG!(test, hash.as_ptr(), ovl_hash, HASH_SIZE,
            "Overlap test 3 failed with len=%zu offset=%zu left_end=%d", len, offset, left_end);
    }
}

unsafe fn test_hash_alignment_consistency(test: *mut kunit) {
    let max_len: usize = 16384;
    let data = alloc_guarded_buf(test, max_len);
    let mut hash1 = [0u8; 128 + HASH_SIZE]; let mut hash2 = [0u8; 128 + HASH_SIZE];
    for _ in 0..100 {
        let len = rand_length(max_len); let data_offs1 = rand_offset(max_len-len);
        let data_offs2 = rand_offset(max_len-len); let hash_offs1 = rand_offset(128); let hash_offs2 = rand_offset(128);
        rand_bytes(data.add(data_offs1), len); HASH!(data.add(data_offs1), len, hash1.as_mut_ptr().add(hash_offs1));
        core::ptr::copy(data.add(data_offs1), data.add(data_offs2), len);
        HASH!(data.add(data_offs2), len, hash2.as_mut_ptr().add(hash_offs2));
        KUNIT_ASSERT_MEMEQ_MSG!(test, hash1.as_ptr().add(hash_offs1), hash2.as_ptr().add(hash_offs2), HASH_SIZE,
            "Alignment consistency test failed with len=%zu data_offs=(%zu,%zu) hash_offs=(%zu,%zu)", len, data_offs1, data_offs2, hash_offs1, hash_offs2);
    }
}

unsafe fn test_hash_ctx_zeroization(test: *mut kunit) {
    let zeroes = [0u8; core::mem::size_of::<HASH_CTX>()]; let mut ctx: HASH_CTX = core::mem::zeroed();
    let data_len = 128usize; let data = alloc_buf(test, data_len); let mut hash = [0u8; HASH_SIZE];
    rand_bytes(data, data_len); HASH_INIT!(&mut ctx); HASH_UPDATE!(&mut ctx, data, data_len); HASH_FINAL!(&mut ctx, hash.as_mut_ptr());
    KUNIT_ASSERT_MEMEQ_MSG!(test, &ctx as *const _, zeroes.as_ptr(), core::mem::size_of::<HASH_CTX>(), "Hash context was not zeroized by finalization");
}

const IRQ_TEST_DATA_LEN: usize = 256;
const IRQ_TEST_NUM_BUFFERS: usize = 3;

#[repr(C)] struct hash_irq_test1_state { data: *mut u8, expected_hashes: [[u8; HASH_SIZE]; IRQ_TEST_NUM_BUFFERS], seqno: atomic_t }
unsafe fn hash_irq_test1_func(state_: *mut core::ffi::c_void) -> bool {
    let state = &mut *(state_ as *mut hash_irq_test1_state); let i = (atomic_inc_return(&mut state.seqno) as usize) % IRQ_TEST_NUM_BUFFERS; let mut actual = [0u8; HASH_SIZE];
    HASH!(state.data.add(i * IRQ_TEST_DATA_LEN), IRQ_TEST_DATA_LEN, actual.as_mut_ptr());
    core::slice::from_raw_parts(actual.as_ptr(), HASH_SIZE) == &state.expected_hashes[i]
}

unsafe fn test_hash_interrupt_context_1(test: *mut kunit) {
    let total_data_len = IRQ_TEST_NUM_BUFFERS * IRQ_TEST_DATA_LEN;
    let mut state: hash_irq_test1_state = core::mem::zeroed();
    state.data = alloc_buf(test, total_data_len); rand_bytes(state.data, total_data_len);
    for i in 0..IRQ_TEST_NUM_BUFFERS { HASH!(state.data.add(i*IRQ_TEST_DATA_LEN), IRQ_TEST_DATA_LEN, state.expected_hashes[i].as_mut_ptr()); }
    kunit_run_irq_test!(test, hash_irq_test1_func, 100000, &mut state);
}

#[repr(C)] struct hash_irq_test2_hash_ctx { hash_ctx: HASH_CTX, in_use: atomic_t, offset: i32, step: i32 }
#[repr(C)] struct hash_irq_test2_state { data: *mut u8, data_len: usize, ctxs: [hash_irq_test2_hash_ctx; IRQ_TEST_NUM_BUFFERS], expected_hash: [u8; HASH_SIZE], update_lens: [u16; 32], num_steps: i32 }

unsafe fn hash_irq_test2_func(state_: *mut core::ffi::c_void) -> bool {
    let state = &mut *(state_ as *mut hash_irq_test2_state); let mut selected: *mut hash_irq_test2_hash_ctx = core::ptr::null_mut();
    for i in 0..IRQ_TEST_NUM_BUFFERS { let ctx = &mut state.ctxs[i]; if atomic_cmpxchg(&mut ctx.in_use, 0, 1) == 0 { selected = ctx; break; } }
    if selected.is_null() { WARN_ON_ONCE!(true); return false; }
    let ctx = &mut *selected; let mut ret = true;
    if ctx.step == 0 { HASH_INIT!(&mut ctx.hash_ctx); ctx.offset = 0; ctx.step += 1; }
    else if ctx.step < state.num_steps - 1 { HASH_UPDATE!(&mut ctx.hash_ctx, state.data.add(ctx.offset as usize), state.update_lens[(ctx.step-1) as usize] as usize); ctx.offset += state.update_lens[(ctx.step-1) as usize] as i32; ctx.step += 1; }
    else { let mut actual = [0u8; HASH_SIZE]; if ctx.offset as usize != state.data_len { WARN_ON_ONCE!(true); ret = false; } HASH_FINAL!(&mut ctx.hash_ctx, actual.as_mut_ptr()); if actual != state.expected_hash { ret = false; } ctx.step = 0; }
    atomic_set_release(&mut ctx.in_use, 0); ret
}

unsafe fn test_hash_interrupt_context_2(test: *mut kunit) {
    let data_len = 16384usize; let state = kunit_kzalloc!(test, core::mem::size_of::<hash_irq_test2_state>(), GFP_KERNEL) as *mut hash_irq_test2_state;
    KUNIT_ASSERT_NOT_NULL!(test, state); (*state).data_len = data_len; (*state).data = alloc_buf(test, data_len); rand_bytes((*state).data, data_len); HASH!((*state).data, data_len, (*state).expected_hash.as_mut_ptr());
    let mut remaining = data_len; KUNIT_ASSERT_GT!(test, data_len / 4096, 1);
    (*state).num_steps = 0;
    while (*state).num_steps < 31 && remaining != 0 { let n = rand_length(core::cmp::min(remaining, 4096)); (*state).update_lens[(*state).num_steps as usize] = n as u16; remaining -= n; (*state).num_steps += 1; }
    if remaining != 0 { (*state).update_lens[(*state).num_steps as usize] = remaining as u16; (*state).num_steps += 1; } (*state).num_steps += 2;
    kunit_run_irq_test!(test, hash_irq_test2_func, 250000, state);
}

/* The remaining KUnit registration and interrupt-context declarations retain
 * the original template's external macro interfaces. */
macro_rules! UNKEYED_HASH_KUNIT_CASES { () => { KUNIT_CASE!(test_hash_test_vectors), KUNIT_CASE!(test_hash_all_lens_up_to_4096), KUNIT_CASE!(test_hash_incremental_updates), KUNIT_CASE!(test_hash_buffer_overruns), KUNIT_CASE!(test_hash_overlaps), KUNIT_CASE!(test_hash_alignment_consistency), KUNIT_CASE!(test_hash_ctx_zeroization), KUNIT_CASE!(test_hash_interrupt_context_1), KUNIT_CASE!(test_hash_interrupt_context_2) } }

#[cfg(feature = "HMAC")]
unsafe fn test_hmac(test: *mut kunit) {
    let max_data_len = 4096usize; let max_key_len = 293usize; let outer_key_len = 32usize;
    let data = alloc_guarded_buf(test, max_data_len); let raw_key = alloc_guarded_buf(test, max_key_len);
    let zeroes = [0u8; core::mem::size_of::<HMAC_CTX>()]; let mut key: HMAC_KEY = core::mem::zeroed(); let mut ctx: HMAC_CTX = core::mem::zeroed(); let mut mac=[0u8;HASH_SIZE]; let mut mac2=[0u8;HASH_SIZE];
    rand_bytes_seeded_from_len(data, max_data_len); rand_bytes_seeded_from_len(raw_key, outer_key_len); HMAC_PREPAREKEY!(&mut key, raw_key, outer_key_len); HMAC_INIT!(&mut ctx, &key);
    for data_len in 0..=max_data_len { let key_len = data_len % max_key_len; HMAC_UPDATE!(&mut ctx, data, data_len); rand_bytes_seeded_from_len(raw_key, key_len); HMAC_USINGRAWKEY!(raw_key, key_len, data, data_len, mac.as_mut_ptr()); HMAC_UPDATE!(&mut ctx, mac.as_ptr(), HASH_SIZE); HMAC_PREPAREKEY!(&mut key, raw_key, key_len); HMAC!(&key, data, data_len, mac2.as_mut_ptr()); KUNIT_ASSERT_MEMEQ_MSG!(test, mac.as_ptr(), mac2.as_ptr(), HASH_SIZE, "HMAC gave different results with raw and prepared keys"); }
    HMAC_FINAL!(&mut ctx, mac.as_mut_ptr()); KUNIT_EXPECT_MEMEQ_MSG!(test, mac.as_ptr(), hmac_testvec_consolidated, HASH_SIZE, "HMAC gave wrong result"); KUNIT_EXPECT_MEMEQ_MSG!(test, &ctx as *const _, zeroes.as_ptr(), core::mem::size_of::<HMAC_CTX>(), "HMAC context was not zeroized by finalization");
}

#[cfg(feature = "HMAC")]
macro_rules! HASH_KUNIT_CASES { () => { UNKEYED_HASH_KUNIT_CASES!(), KUNIT_CASE!(test_hmac) } }
#[cfg(not(feature = "HMAC"))]
macro_rules! HASH_KUNIT_CASES { () => { UNKEYED_HASH_KUNIT_CASES!() } }

/* benchmark_hash is omitted so that suites can put it last. */
unsafe fn benchmark_hash(test: *mut kunit) {
    const LENS_TO_TEST: [usize; 13] = [1,16,64,127,128,200,256,511,512,1024,3173,4096,16384];
    let max_len = 16384usize; let data = alloc_buf(test, max_len); let mut hash = [0u8; HASH_SIZE];
    if !IS_ENABLED!(CONFIG_CRYPTO_LIB_BENCHMARK) { kunit_skip!(test, "not enabled"); }
    core::ptr::write_bytes(data, 0, max_len);
    let mut i = 0; while i < 10000000 { HASH!(data, max_len, hash.as_mut_ptr()); i += max_len; }
    for len in LENS_TO_TEST { let num_iters = 10000000 / (len + 128); KUNIT_ASSERT_LE!(test, len, max_len); preempt_disable!(); let mut t = ktime_get_ns(); for _ in 0..num_iters { HASH!(data, len, hash.as_mut_ptr()); } t = ktime_get_ns() - t; preempt_enable!(); kunit_info!(test, "len=%zu: %llu MB/s", len, div64_u64((len as u64)*num_iters as u64*1000, if t != 0 {t} else {1})); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
