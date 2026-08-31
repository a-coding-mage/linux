use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_void};
use std::ptr;

// C dependencies: <stdio.h>, <assert.h>, and <linux/scatterlist.h>.
const MAX_PAGES: usize = 64;
const EINVAL: c_int = 22;
const PAGE_SIZE: c_uint = 4096;
const GFP_KERNEL: c_uint = 0;

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sg_table {
    pub sgl: *mut c_void,
    pub nents: c_uint,
    pub orig_nents: c_uint,
}

#[repr(C)]
pub struct sg_append_table {
    pub sgt: sg_table,
    pub prv: *mut c_void,
    pub total_nents: c_uint,
}

#[repr(C)]
struct test {
    alloc_ret: c_int,
    num_pages: c_uint,
    pfn: *const c_uint,
    pfn_app: *const c_uint,
    size: c_uint,
    max_seg: c_uint,
    expected_segments: c_uint,
}

unsafe extern "C" {
    fn sg_alloc_append_table_from_pages(
        append: *mut sg_append_table,
        pages: *mut *mut page,
        n_pages: c_uint,
        offset: c_uint,
        size: c_uint,
        max_segment: c_uint,
        left_pages: c_int,
        gfp_mask: c_uint,
    ) -> c_int;

    fn sg_alloc_table_from_pages_segment(
        sgt: *mut sg_table,
        pages: *mut *mut page,
        n_pages: c_uint,
        offset: c_uint,
        size: c_uint,
        max_segment: c_uint,
        gfp_mask: c_uint,
    ) -> c_int;

    fn sg_free_append_table(append: *mut sg_append_table);
    fn sg_free_table(sgt: *mut sg_table);
}

unsafe fn set_pages(pages: *mut *mut page, array: *const c_uint, num: c_uint) {
    assert!((num as usize) < MAX_PAGES);
    for i in 0..num {
        *pages.add(i as usize) =
            (((1u32.wrapping_add(*array.add(i as usize))).wrapping_mul(PAGE_SIZE)) as c_ulong)
                as *mut page;
    }
}

unsafe fn fail(test: *mut test, st: *mut sg_table, cond: *const c_char) -> ! {
    let cond = CStr::from_ptr(cond).to_string_lossy();

    eprintln!("Failed on '{}'!\n", cond);

    println!(
        "size = {}, max segment = {}, expected nents = {}\nst->nents = {}, st->orig_nents= {}",
        (*test).size,
        (*test).max_seg,
        (*test).expected_segments,
        (*st).nents,
        (*st).orig_nents
    );

    print!("{} input PFNs:", (*test).num_pages);
    for i in 0..(*test).num_pages {
        print!(" {:x}", *(*test).pfn.add(i as usize));
    }
    println!();

    std::process::exit(1);
}

macro_rules! validate {
    ($cond:expr, $st:expr, $test:expr) => {
        if !($cond) {
            unsafe {
                fail(
                    $test,
                    $st,
                    concat!(stringify!($cond), "\0").as_ptr() as *const c_char,
                )
            };
        }
    };
}

fn main() {
    unsafe {
        let sgmax: c_uint = c_uint::MAX;

        let pfn_0 = [0u32];
        let pfn_0_1 = [0u32, 1];
        let pfn_1_0 = [1u32, 0];
        let pfn_0_1_2 = [0u32, 1, 2];
        let pfn_3_4_5 = [3u32, 4, 5];
        let pfn_4_5_6 = [4u32, 5, 6];
        let pfn_0_2_1 = [0u32, 2, 1];
        let pfn_0_1_3 = [0u32, 1, 3];
        let pfn_1_2_4 = [1u32, 2, 4];
        let pfn_1_3_4 = [1u32, 3, 4];
        let pfn_0_1_3_4 = [0u32, 1, 3, 4];
        let pfn_0_1_3_4_5 = [0u32, 1, 3, 4, 5];
        let pfn_0_1_3_4_6 = [0u32, 1, 3, 4, 6];
        let pfn_0_1_2_3_4 = [0u32, 1, 2, 3, 4];
        let pfn_0_1_2_3_4_5 = [0u32, 1, 2, 3, 4, 5];
        let pfn_0_2_3_4_5_6 = [0u32, 2, 3, 4, 5, 6];
        let pfn_0_1_3_4_5_6 = [0u32, 1, 3, 4, 5, 6];
        let pfn_7_8_9_10_11_12 = [7u32, 8, 9, 10, 11, 12];

        let mut tests = [
            test {
                alloc_ret: -EINVAL,
                num_pages: 1,
                pfn: pfn_0.as_ptr(),
                pfn_app: ptr::null(),
                size: PAGE_SIZE,
                max_seg: 0,
                expected_segments: 1,
            },
            test {
                alloc_ret: 0,
                num_pages: 1,
                pfn: pfn_0.as_ptr(),
                pfn_app: ptr::null(),
                size: PAGE_SIZE,
                max_seg: PAGE_SIZE + 1,
                expected_segments: 1,
            },
            test {
                alloc_ret: 0,
                num_pages: 1,
                pfn: pfn_0.as_ptr(),
                pfn_app: ptr::null(),
                size: PAGE_SIZE,
                max_seg: sgmax,
                expected_segments: 1,
            },
            test {
                alloc_ret: 0,
                num_pages: 1,
                pfn: pfn_0.as_ptr(),
                pfn_app: ptr::null(),
                size: 1,
                max_seg: sgmax,
                expected_segments: 1,
            },
            test {
                alloc_ret: 0,
                num_pages: 2,
                pfn: pfn_0_1.as_ptr(),
                pfn_app: ptr::null(),
                size: 2 * PAGE_SIZE,
                max_seg: sgmax,
                expected_segments: 1,
            },
            test {
                alloc_ret: 0,
                num_pages: 2,
                pfn: pfn_1_0.as_ptr(),
                pfn_app: ptr::null(),
                size: 2 * PAGE_SIZE,
                max_seg: sgmax,
                expected_segments: 2,
            },
            test {
                alloc_ret: 0,
                num_pages: 3,
                pfn: pfn_0_1_2.as_ptr(),
                pfn_app: ptr::null(),
                size: 3 * PAGE_SIZE,
                max_seg: sgmax,
                expected_segments: 1,
            },
            test {
                alloc_ret: 0,
                num_pages: 3,
                pfn: pfn_0_1_2.as_ptr(),
                pfn_app: ptr::null(),
                size: 3 * PAGE_SIZE,
                max_seg: sgmax,
                expected_segments: 1,
            },
            test {
                alloc_ret: 0,
                num_pages: 3,
                pfn: pfn_0_1_2.as_ptr(),
                pfn_app: pfn_3_4_5.as_ptr(),
                size: 3 * PAGE_SIZE,
                max_seg: sgmax,
                expected_segments: 1,
            },
            test {
                alloc_ret: 0,
                num_pages: 3,
                pfn: pfn_0_1_2.as_ptr(),
                pfn_app: pfn_4_5_6.as_ptr(),
                size: 3 * PAGE_SIZE,
                max_seg: sgmax,
                expected_segments: 2,
            },
            test {
                alloc_ret: 0,
                num_pages: 3,
                pfn: pfn_0_2_1.as_ptr(),
                pfn_app: ptr::null(),
                size: 3 * PAGE_SIZE,
                max_seg: sgmax,
                expected_segments: 3,
            },
            test {
                alloc_ret: 0,
                num_pages: 3,
                pfn: pfn_0_1_3.as_ptr(),
                pfn_app: ptr::null(),
                size: 3 * PAGE_SIZE,
                max_seg: sgmax,
                expected_segments: 2,
            },
            test {
                alloc_ret: 0,
                num_pages: 3,
                pfn: pfn_1_2_4.as_ptr(),
                pfn_app: ptr::null(),
                size: 3 * PAGE_SIZE,
                max_seg: sgmax,
                expected_segments: 2,
            },
            test {
                alloc_ret: 0,
                num_pages: 3,
                pfn: pfn_1_3_4.as_ptr(),
                pfn_app: ptr::null(),
                size: 3 * PAGE_SIZE,
                max_seg: sgmax,
                expected_segments: 2,
            },
            test {
                alloc_ret: 0,
                num_pages: 4,
                pfn: pfn_0_1_3_4.as_ptr(),
                pfn_app: ptr::null(),
                size: 4 * PAGE_SIZE,
                max_seg: sgmax,
                expected_segments: 2,
            },
            test {
                alloc_ret: 0,
                num_pages: 5,
                pfn: pfn_0_1_3_4_5.as_ptr(),
                pfn_app: ptr::null(),
                size: 5 * PAGE_SIZE,
                max_seg: sgmax,
                expected_segments: 2,
            },
            test {
                alloc_ret: 0,
                num_pages: 5,
                pfn: pfn_0_1_3_4_6.as_ptr(),
                pfn_app: ptr::null(),
                size: 5 * PAGE_SIZE,
                max_seg: sgmax,
                expected_segments: 3,
            },
            test {
                alloc_ret: 0,
                num_pages: 5,
                pfn: pfn_0_1_2_3_4.as_ptr(),
                pfn_app: ptr::null(),
                size: 5 * PAGE_SIZE,
                max_seg: sgmax,
                expected_segments: 1,
            },
            test {
                alloc_ret: 0,
                num_pages: 5,
                pfn: pfn_0_1_2_3_4.as_ptr(),
                pfn_app: ptr::null(),
                size: 5 * PAGE_SIZE,
                max_seg: 2 * PAGE_SIZE,
                expected_segments: 3,
            },
            test {
                alloc_ret: 0,
                num_pages: 6,
                pfn: pfn_0_1_2_3_4_5.as_ptr(),
                pfn_app: ptr::null(),
                size: 6 * PAGE_SIZE,
                max_seg: 2 * PAGE_SIZE,
                expected_segments: 3,
            },
            test {
                alloc_ret: 0,
                num_pages: 6,
                pfn: pfn_0_2_3_4_5_6.as_ptr(),
                pfn_app: ptr::null(),
                size: 6 * PAGE_SIZE,
                max_seg: 2 * PAGE_SIZE,
                expected_segments: 4,
            },
            test {
                alloc_ret: 0,
                num_pages: 6,
                pfn: pfn_0_1_3_4_5_6.as_ptr(),
                pfn_app: pfn_7_8_9_10_11_12.as_ptr(),
                size: 6 * PAGE_SIZE,
                max_seg: 12 * PAGE_SIZE,
                expected_segments: 2,
            },
            test {
                alloc_ret: 0,
                num_pages: 0,
                pfn: ptr::null(),
                pfn_app: ptr::null(),
                size: 0,
                max_seg: 0,
                expected_segments: 0,
            },
        ];

        let mut i: c_uint = 0;
        let mut test = tests.as_mut_ptr();
        while (*test).expected_segments != 0 {
            let left_pages: c_int = if !(*test).pfn_app.is_null() {
                (*test).num_pages as c_int
            } else {
                0
            };
            let mut append: sg_append_table = std::mem::zeroed();
            let mut pages: [*mut page; MAX_PAGES] = [ptr::null_mut(); MAX_PAGES];
            let ret: c_int;

            set_pages(pages.as_mut_ptr(), (*test).pfn, (*test).num_pages);

            if !(*test).pfn_app.is_null() {
                ret = sg_alloc_append_table_from_pages(
                    &mut append,
                    pages.as_mut_ptr(),
                    (*test).num_pages,
                    0,
                    (*test).size,
                    (*test).max_seg,
                    left_pages,
                    GFP_KERNEL,
                );
            } else {
                ret = sg_alloc_table_from_pages_segment(
                    &mut append.sgt,
                    pages.as_mut_ptr(),
                    (*test).num_pages,
                    0,
                    (*test).size,
                    (*test).max_seg,
                    GFP_KERNEL,
                );
            }

            assert!(ret == (*test).alloc_ret);

            if (*test).alloc_ret != 0 {
                test = test.add(1);
                i = i.wrapping_add(1);
                continue;
            }

            if !(*test).pfn_app.is_null() {
                set_pages(pages.as_mut_ptr(), (*test).pfn_app, (*test).num_pages);
                ret = sg_alloc_append_table_from_pages(
                    &mut append,
                    pages.as_mut_ptr(),
                    (*test).num_pages,
                    0,
                    (*test).size,
                    (*test).max_seg,
                    0,
                    GFP_KERNEL,
                );

                assert!(ret == (*test).alloc_ret);
            }

            validate!(
                append.sgt.nents == (*test).expected_segments,
                &mut append.sgt,
                test
            );
            if (*test).pfn_app.is_null() {
                validate!(
                    append.sgt.orig_nents == (*test).expected_segments,
                    &mut append.sgt,
                    test
                );
            }

            if !(*test).pfn_app.is_null() {
                sg_free_append_table(&mut append);
            } else {
                sg_free_table(&mut append.sgt);
            }

            test = test.add(1);
            i = i.wrapping_add(1);
        }

        assert!((i as usize) == (std::mem::size_of_val(&tests) / std::mem::size_of::<test>()) - 1);
    }
}
