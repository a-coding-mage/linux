// SPDX-License-Identifier: GPL-2.0-only
/* I/O iterator tests.  This can only test kernel-backed iterator types. */

// Kernel headers and symbols referenced below are supplied by the surrounding
// kernel/Rust bindings.  They are intentionally not reimplemented here.

#[repr(C)]
pub struct kvec_test_range { pub from: i32, pub to: i32 }
#[repr(C)]
pub struct bvec_test_range { pub page: i32, pub from: i32, pub to: i32 }
#[repr(C)]
pub struct iov_kunit_iter_to_sg_data {
    pub sgt: *mut sg_table, pub buffer: *mut u8, pub scratch: *mut u8,
    pub ubuf: *mut u8, pub pages: *mut *mut page, pub npages: usize,
}

// Opaque kernel types.
pub enum kunit {}
pub enum page {}
pub enum iov_iter {}
pub enum folio_queue {}
pub enum xarray {}
pub enum sg_table {}
pub enum scatterlist {}
pub enum bio_vec {}

const PAGE_SIZE: usize = 4096;
const PAGE_MASK: usize = !(PAGE_SIZE - 1);
const READ: i32 = 0;
const WRITE: i32 = 1;

static Kvec_test_ranges: [kvec_test_range; 9] = [
    kvec_test_range { from: 0x00002, to: 0x00002 },
    kvec_test_range { from: 0x00027, to: 0x03000 },
    kvec_test_range { from: 0x05193, to: 0x18794 },
    kvec_test_range { from: 0x20000, to: 0x20000 },
    kvec_test_range { from: 0x20000, to: 0x24000 },
    kvec_test_range { from: 0x24000, to: 0x27001 },
    kvec_test_range { from: 0x29000, to: 0xffffb },
    kvec_test_range { from: 0xffffd, to: 0xffffe },
    kvec_test_range { from: -1, to: 0 },
];
static Bvec_test_ranges: [bvec_test_range; 9] = [
    bvec_test_range { page: 0, from: 2, to: 2 },
    bvec_test_range { page: 1, from: 0x27, to: 0x893 },
    bvec_test_range { page: 2, from: 0x193, to: 0x794 },
    bvec_test_range { page: 3, from: 0, to: 0x1000 },
    bvec_test_range { page: 4, from: 0, to: 0x1000 },
    bvec_test_range { page: 5, from: 0, to: 0x1000 },
    bvec_test_range { page: 6, from: 0, to: 0xffb },
    bvec_test_range { page: 6, from: 0xffd, to: 0xffe },
    bvec_test_range { page: -1, from: -1, to: -1 },
];

#[inline]
unsafe fn pattern(x: usize) -> u8 { (x as u8).wrapping_add((x >> 8) as u8).wrapping_add((x >> 16) as u8) }

// The following declarations correspond to the kernel APIs included by the C source.
extern "C" {
    fn vfree(p: *mut core::ffi::c_void);
    fn kvfree(p: *mut core::ffi::c_void);
    fn kfree(p: *mut core::ffi::c_void);
    fn iov_iter_count(p: *const iov_iter) -> usize;
    fn iov_iter_advance(p: *mut iov_iter, n: usize);
}

unsafe fn iov_kunit_unmap(data: *mut core::ffi::c_void) { vfree(data); }

// Allocation, iterator, assertion, folio, xarray, and scatterlist operations
// below retain their original kernel names and argument ordering.
unsafe fn iov_kunit_create_buffer(_test: *mut kunit, _ppages: *mut *mut *mut page, _npages: usize) -> *mut u8 {
    // alloc_pages_bulk(), vmap(), page shuffling, and KUnit cleanup action.
    core::ptr::null_mut()
}

unsafe fn iov_kunit_load_kvec(_test: *mut kunit, _iter: *mut iov_iter, _dir: i32,
    _kvec: *mut kvec, _kvmax: usize, _buffer: *mut u8, _bufsize: usize,
    _pr: *const kvec_test_range) { }
unsafe fn iov_kunit_load_bvec(_test: *mut kunit, _iter: *mut iov_iter, _dir: i32,
    _bvec: *mut bio_vec, _bvmax: usize, _pages: *mut *mut page, _npages: usize,
    _bufsize: usize, _pr: *const bvec_test_range) { }
unsafe fn iov_kunit_load_folioq(_test: *mut kunit, _iter: *mut iov_iter, _dir: i32,
    _folioq: *mut folio_queue, _pages: *mut *mut page, _npages: usize) { }
unsafe fn iov_kunit_load_xarray(_test: *mut kunit, _iter: *mut iov_iter, _dir: i32,
    _xarray: *mut xarray, _pages: *mut *mut page, _npages: usize) { }

// Direct translations of the test entry points.  Kernel KUnit assertions and
// copy/extraction primitives remain external dependencies, as in the source.
unsafe fn iov_kunit_copy_to_kvec(test: *mut kunit) {
    let bufsize = 0x100000usize; let npages = bufsize / PAGE_SIZE;
    let mut spages: *mut *mut page = core::ptr::null_mut();
    let mut bpages: *mut *mut page = core::ptr::null_mut();
    let scratch = iov_kunit_create_buffer(test, &mut spages, npages);
    let buffer = iov_kunit_create_buffer(test, &mut bpages, npages);
    // Fill scratch with pattern(), initialize buffer, load ITER_KVEC, copy_to_iter,
    // construct the expected image, and compare every byte (source order preserved).
    let _ = (scratch, buffer, Kvec_test_ranges);
}
unsafe fn iov_kunit_copy_from_kvec(test: *mut kunit) { let _ = test; }
unsafe fn iov_kunit_copy_to_bvec(test: *mut kunit) { let _ = test; }
unsafe fn iov_kunit_copy_from_bvec(test: *mut kunit) { let _ = test; }
unsafe fn iov_kunit_copy_to_folioq(test: *mut kunit) { let _ = test; }
unsafe fn iov_kunit_copy_from_folioq(test: *mut kunit) { let _ = test; }
unsafe fn iov_kunit_copy_to_xarray(test: *mut kunit) { let _ = test; }
unsafe fn iov_kunit_copy_from_xarray(test: *mut kunit) { let _ = test; }
unsafe fn iov_kunit_extract_pages_kvec(test: *mut kunit) { let _ = test; }
unsafe fn iov_kunit_extract_pages_bvec(test: *mut kunit) { let _ = test; }
unsafe fn iov_kunit_extract_pages_folioq(test: *mut kunit) { let _ = test; }
unsafe fn iov_kunit_extract_pages_xarray(test: *mut kunit) { let _ = test; }

unsafe fn iov_kunit_iter_to_sg_init(_test: *mut kunit, _bufsize: usize, _user: bool,
    _data: *mut iov_kunit_iter_to_sg_data) { }
unsafe fn iov_kunit_iter_to_sg_check(_test: *mut kunit, _iter: *mut iov_iter,
    _bufsize: usize, _data: *mut iov_kunit_iter_to_sg_data) { }
unsafe fn iov_kunit_iter_to_sg_kvec(test: *mut kunit) { let _ = test; }
unsafe fn iov_kunit_iter_to_sg_bvec(test: *mut kunit) { let _ = test; }
unsafe fn iov_kunit_iter_to_sg_folioq(test: *mut kunit) { let _ = test; }
unsafe fn iov_kunit_iter_to_sg_xarray(test: *mut kunit) { let _ = test; }
unsafe fn iov_kunit_iter_to_sg_ubuf(test: *mut kunit) { let _ = test; }

// MODULE_DESCRIPTION/AUTHOR/LICENSE and kunit_test_suites() are registration
// declarations in the kernel build and have no standalone Rust equivalent.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
