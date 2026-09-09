// SPDX-License-Identifier: GPL-2.0
/*
 * Test cases for binder allocator code.
 *
 * Copyright 2025 Google LLC.
 * Author: Tiffany Yang <ynaffit@google.com>
 */

// Kernel includes and binder headers are supplied by the surrounding build.

const BINDER_MMAP_SIZE: usize = 128 * 1024;
const BUFFER_NUM: usize = 5;
const BUFFER_MIN_SIZE: usize = PAGE_SIZE / 8;
const FREESEQ_BUFLEN: usize = (3 * BUFFER_NUM) + 1;
const ALIGN_TYPE_STRLEN: usize = 12;
const ALIGNMENTS_BUFLEN: usize = ((ALIGN_TYPE_STRLEN + 6) * BUFFER_NUM) + 1;
const PRINT_ALL_CASES: bool = false;
const TOTAL_EXHAUSTIVE_CASES: usize = 3125 * 2 * 120;

#[repr(C)]
#[derive(Copy, Clone)]
enum BufEndAlignType {
    SamePageUnaligned = 0,
    SamePageAligned,
    NextPageUnaligned,
    NextPageAligned,
    NextNextUnaligned,
    LoopEnd,
}

static BUF_END_ALIGN_TYPE_STRS: [&[u8]; 5] = [
    b"SP_UNALIGNED\0", b" SP_ALIGNED \0", b"NP_UNALIGNED\0", b" NP_ALIGNED \0", b"NN_UNALIGNED\0",
];

#[repr(C)]
struct BinderAllocTestCaseInfo {
    alignments: [i8; ALIGNMENTS_BUFLEN],
    alignments_sb: SeqBuf,
    buffer_sizes: *mut usize,
    free_sequence: *mut i32,
    front_pages: bool,
}

unsafe fn stringify_free_seq(test: *mut Kunit, seq: *mut i32, sb: *mut SeqBuf) {
    for i in 0..BUFFER_NUM { seq_buf_printf(sb, b"[%d]\0", *seq.add(i)); }
    kunit_expect_false(test, seq_buf_has_overflowed(sb));
}

unsafe fn stringify_alignments(test: *mut Kunit, alignments: *mut i32, sb: *mut SeqBuf) {
    for i in 0..BUFFER_NUM {
        seq_buf_printf(sb, b"[ %d:%s ]\0", i, BUF_END_ALIGN_TYPE_STRS[*alignments.add(i) as usize].as_ptr());
    }
    kunit_expect_false(test, seq_buf_has_overflowed(sb));
}

unsafe fn check_buffer_pages_allocated(test: *mut Kunit, alloc: *mut BinderAlloc, buffer: *mut BinderBuffer, size: usize) -> bool {
    let end = page_align((*buffer).user_data + size);
    let mut page_addr = (*buffer).user_data;
    while page_addr < end {
        let page_index = (page_addr - (*alloc).vm_start) / PAGE_SIZE;
        let page = *(*alloc).pages.add(page_index);
        if page.is_null() || !list_empty(page_to_lru(page)) {
            kunit_err(test, b"expect alloc but is %s at page index %d\n\0", if page.is_null() { b"free\0".as_ptr() } else { b"lru\0".as_ptr() }, page_index);
            return false;
        }
        page_addr += PAGE_SIZE;
    }
    true
}

unsafe fn binder_alloc_test_alloc_buf(test: *mut Kunit, alloc: *mut BinderAlloc, buffers: *mut *mut BinderBuffer, sizes: *mut usize, _seq: *mut i32) -> usize {
    let mut failures = 0;
    for i in 0..BUFFER_NUM {
        let buffer = binder_alloc_new_buf(alloc, *sizes.add(i), 0, 0, 0);
        *buffers.add(i) = buffer;
        if is_err(buffer) || !check_buffer_pages_allocated(test, alloc, buffer, *sizes.add(i)) { failures += 1; }
    }
    failures
}

unsafe fn binder_alloc_test_free_buf(test: *mut Kunit, alloc: *mut BinderAlloc, buffers: *mut *mut BinderBuffer, _sizes: *mut usize, seq: *mut i32, end: usize) -> usize {
    let mut failures = 0;
    for i in 0..BUFFER_NUM { binder_alloc_free_buf(alloc, *buffers.add(*seq.add(i) as usize)); }
    for i in 0..=((end - 1) / PAGE_SIZE) {
        if list_empty(page_to_lru(*(*alloc).pages.add(i))) {
            kunit_err(test, b"expect lru but is %s at page index %d\n\0", if (*(*alloc).pages.add(i)).is_null() { b"free\0".as_ptr() } else { b"alloc\0".as_ptr() }, i); failures += 1;
        }
    }
    failures
}

unsafe fn binder_alloc_test_free_page(test: *mut Kunit, alloc: *mut BinderAlloc) -> usize {
    let mut failures = 0;
    while list_lru_count((*alloc).freelist) != 0 { list_lru_walk((*alloc).freelist, binder_alloc_free_page, core::ptr::null_mut(), list_lru_count((*alloc).freelist)); }
    for i in 0..((*alloc).buffer_size / PAGE_SIZE) {
        let page = *(*alloc).pages.add(i);
        if !page.is_null() { kunit_err(test, b"expect free but is %s at page index %d\n\0", if list_empty(page_to_lru(page)) { b"alloc\0".as_ptr() } else { b"lru\0".as_ptr() }, i); failures += 1; }
    }
    failures
}

unsafe fn binder_alloc_test_alloc_free(test: *mut Kunit, alloc: *mut BinderAlloc, tc: *mut BinderAllocTestCaseInfo, end: usize) -> bool {
    let pages = page_align(end) / PAGE_SIZE;
    let mut buffers = [core::ptr::null_mut(); BUFFER_NUM];
    let mut failed = false;
    let mut failures = binder_alloc_test_alloc_buf(test, alloc, buffers.as_mut_ptr(), (*tc).buffer_sizes, (*tc).free_sequence); failed |= failures != 0;
    kunit_expect_eq_msg(test, failures, 0, b"Initial allocation failed: %lu/%u buffers with errors\0", failures, BUFFER_NUM);
    failures = binder_alloc_test_free_buf(test, alloc, buffers.as_mut_ptr(), (*tc).buffer_sizes, (*tc).free_sequence, end); failed |= failures != 0;
    kunit_expect_eq_msg(test, failures, 0, b"Initial buffers not freed correctly: %lu/%lu pages not on lru list\0", failures, pages);
    failures = binder_alloc_test_alloc_buf(test, alloc, buffers.as_mut_ptr(), (*tc).buffer_sizes, (*tc).free_sequence); failed |= failures != 0;
    kunit_expect_eq_msg(test, failures, 0, b"Reallocation failed: %lu/%u buffers with errors\0", failures, BUFFER_NUM);
    failures = list_lru_count((*alloc).freelist); failed |= failures != 0;
    kunit_expect_eq_msg(test, failures, 0, b"lru list should be empty after reallocation but still has %lu pages\0", failures);
    failures = binder_alloc_test_free_buf(test, alloc, buffers.as_mut_ptr(), (*tc).buffer_sizes, (*tc).free_sequence, end); failed |= failures != 0;
    kunit_expect_eq_msg(test, failures, 0, b"Reallocated buffers not freed correctly: %lu/%lu pages not on lru list\0", failures, pages);
    failures = binder_alloc_test_free_page(test, alloc); failed |= failures != 0;
    kunit_expect_eq_msg(test, failures, 0, b"Failed to clean up allocated pages: %lu/%lu pages still installed\0", failures, (*alloc).buffer_size / PAGE_SIZE);
    failed
}

unsafe fn is_dup(seq: *mut i32, index: usize, val: i32) -> bool {
    for i in 0..index { if *seq.add(i) == val { return true; } }
    false
}

unsafe fn permute_frees(test: *mut Kunit, alloc: *mut BinderAlloc, tc: *mut BinderAllocTestCaseInfo, runs: *mut usize, failures: *mut usize, index: usize, end: usize) {
    if index == BUFFER_NUM {
        let case_failed = binder_alloc_test_alloc_free(test, alloc, tc, end);
        *runs += 1; *failures += case_failed as usize;
        if case_failed || PRINT_ALL_CASES { stringify_free_seq(test, (*tc).free_sequence, core::ptr::null_mut()); }
        return;
    }
    for i in 0..BUFFER_NUM { if !is_dup((*tc).free_sequence, index, i as i32) { *(*tc).free_sequence.add(index) = i as i32; permute_frees(test, alloc, tc, runs, failures, index + 1, end); } }
}

unsafe fn gen_buf_sizes(test: *mut Kunit, alloc: *mut BinderAlloc, tc: *mut BinderAllocTestCaseInfo, end_offset: *mut usize, runs: *mut usize, failures: *mut usize) {
    let mut last_offset; let mut offset = 0; let mut front_sizes = [0usize; BUFFER_NUM]; let mut back_sizes = [0usize; BUFFER_NUM]; let mut seq = [0i32; BUFFER_NUM];
    (*tc).free_sequence = seq.as_mut_ptr();
    for i in 0..BUFFER_NUM { last_offset = offset; offset = *end_offset.add(i); front_sizes[i] = offset - last_offset; back_sizes[BUFFER_NUM - i - 1] = front_sizes[i]; }
    back_sizes[0] += (*alloc).buffer_size - *end_offset.add(BUFFER_NUM - 1);
    (*tc).front_pages = true; (*tc).buffer_sizes = front_sizes.as_mut_ptr(); permute_frees(test, alloc, tc, runs, failures, 0, *end_offset.add(BUFFER_NUM - 1));
    (*tc).front_pages = false; (*tc).buffer_sizes = back_sizes.as_mut_ptr(); permute_frees(test, alloc, tc, runs, failures, 0, (*alloc).buffer_size);
}

unsafe fn gen_buf_offsets(test: *mut Kunit, alloc: *mut BinderAlloc, end_offset: *mut usize, alignments: *mut i32, runs: *mut usize, failures: *mut usize, index: usize) {
    if index == BUFFER_NUM {
        let mut tc: BinderAllocTestCaseInfo = core::mem::zeroed(); seq_buf_init(&mut tc.alignments_sb, tc.alignments.as_mut_ptr(), ALIGNMENTS_BUFLEN); stringify_alignments(test, alignments, &mut tc.alignments_sb); gen_buf_sizes(test, alloc, &mut tc, end_offset, runs, failures); return;
    }
    let prev = if index == 0 { 0 } else { *end_offset.add(index - 1) }; let mut end = prev;
    for align in 0..(BufEndAlignType::LoopEnd as i32) { if align % 2 != 0 { end = align_up(end, PAGE_SIZE); } else { end += BUFFER_MIN_SIZE; } *end_offset.add(index) = end; *alignments.add(index) = align; gen_buf_offsets(test, alloc, end_offset, alignments, runs, failures, index + 1); }
}

#[repr(C)] struct BinderAllocTest { alloc: BinderAlloc, binder_test_freelist: ListLru, filp: *mut File, mmap_uaddr: usize }

unsafe fn binder_alloc_exhaustive_test(test: *mut Kunit) { let priv_ = (*test).priv_ as *mut BinderAllocTest; let mut end_offset = [0usize; BUFFER_NUM]; let mut alignments = [0i32; BUFFER_NUM]; let mut failures = 0usize; let mut runs = 0usize; gen_buf_offsets(test, &mut (*priv_).alloc, end_offset.as_mut_ptr(), alignments.as_mut_ptr(), &mut runs, &mut failures, 0); kunit_expect_eq(test, runs, TOTAL_EXHAUSTIVE_CASES); kunit_expect_eq(test, failures, 0); }

// External kernel types, constants, helpers, and KUnit registration are supplied by the surrounding build.

unsafe fn binder_alloc_test_init_freelist(test: *mut Kunit) { let p = (*test).priv_ as *mut BinderAllocTest; kunit_expect_ptr_eq(test, (*p).alloc.freelist, &mut (*p).binder_test_freelist); }
unsafe fn binder_alloc_test_mmap(test: *mut Kunit) { let p = (*test).priv_ as *mut BinderAllocTest; kunit_expect_eq(test, (*p).alloc.mapped, true); kunit_expect_eq(test, (*p).alloc.buffer_size, BINDER_MMAP_SIZE); let n = rb_first(&(*p).alloc.allocated_buffers); kunit_expect_ptr_eq(test, n, core::ptr::null_mut()); let n = rb_first(&(*p).alloc.free_buffers); let b = rb_entry(n, core::mem::offset_of!(BinderBuffer, rb_node)); kunit_expect_eq(test, binder_alloc_buffer_size(&mut (*p).alloc, b), BINDER_MMAP_SIZE); kunit_expect_true(test, list_is_last(&(*b).entry, &(*p).alloc.buffers)); }
unsafe fn binder_alloc_test_vma_close(vma: *mut VmAreaStruct) { binder_alloc_vma_close((*vma).vm_private_data); }
unsafe fn binder_alloc_test_mmap_handler(filp: *mut File, vma: *mut VmAreaStruct) -> i32 { let alloc = (*filp).private_data; vm_flags_mod(vma, VM_DONTCOPY | VM_MIXEDMAP, VM_MAYWRITE); (*vma).vm_ops = &BINDER_ALLOC_TEST_VM_OPS; (*vma).vm_private_data = alloc; binder_alloc_mmap_handler(alloc, vma) }
static BINDER_ALLOC_TEST_VM_OPS: VmOperationsStruct = VmOperationsStruct { close: Some(binder_alloc_test_vma_close), fault: Some(binder_vm_fault) };
static BINDER_ALLOC_TEST_FOPS: FileOperations = FileOperations { mmap: Some(binder_alloc_test_mmap_handler) };

unsafe fn binder_alloc_test_init(test: *mut Kunit) -> i32 { let p = kunit_kzalloc(test, core::mem::size_of::<BinderAllocTest>(), GFP_KERNEL) as *mut BinderAllocTest; if p.is_null() { return -12; } (*test).priv_ = p as *mut _; let mut ret = list_lru_init(&mut (*p).binder_test_freelist); if ret != 0 { return ret; } ret = kunit_attach_mm(); if ret != 0 { return ret; } binder_alloc_init(&mut (*p).alloc, &mut (*p).binder_test_freelist); (*p).filp = anon_inode_getfile(b"binder_alloc_kunit\0".as_ptr(), &BINDER_ALLOC_TEST_FOPS, &mut (*p).alloc, O_RDWR | O_CLOEXEC); if is_err_or_null((*p).filp) { return if !(*p).filp.is_null() { ptr_err((*p).filp) } else { -12 }; } (*p).mmap_uaddr = kunit_vm_mmap(test, (*p).filp, 0, BINDER_MMAP_SIZE, PROT_READ, MAP_PRIVATE | MAP_NORESERVE, 0); if (*p).mmap_uaddr == 0 { return -12; } 0 }
unsafe fn binder_alloc_test_exit(test: *mut Kunit) { let p = (*test).priv_ as *mut BinderAllocTest; if !is_err_or_null((*p).filp) { fput((*p).filp); } if !(*p).alloc.mm.is_null() { binder_alloc_deferred_release(&mut (*p).alloc); } kunit_expect_eq(test, list_lru_count(&mut (*p).binder_test_freelist), 0); list_lru_destroy(&mut (*p).binder_test_freelist); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
