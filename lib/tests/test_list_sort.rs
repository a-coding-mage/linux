// SPDX-License-Identifier: GPL-2.0-only

// Kernel/KUnit dependencies supplied by the surrounding build.

pub const TEST_LIST_LEN: usize = 512 + 128 + 2;
pub const TEST_POISON1: u32 = 0xDEADBEEF;
pub const TEST_POISON2: u32 = 0xA324354C;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct kunit {
    pub priv_: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct debug_el {
    pub poison1: u32,
    pub list: list_head,
    pub poison2: u32,
    pub value: i32,
    pub serial: u32,
}

extern "C" {
    fn kunit_kcalloc(
        test: *mut kunit,
        n: usize,
        size: usize,
        flags: u32,
    ) -> *mut *mut debug_el;
    fn kunit_kmalloc(test: *mut kunit, size: usize, flags: u32) -> *mut debug_el;
    fn get_random_u32_below(n: u32) -> u32;
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_sort(
        priv_: *mut kunit,
        head: *mut list_head,
        cmp: unsafe extern "C" fn(*mut core::ffi::c_void, *const list_head, *const list_head) -> i32,
    );
}

unsafe fn check(test: *mut kunit, ela: *const debug_el, elb: *const debug_el) {
    let elts = (*test).priv_ as *mut *mut debug_el;

    assert!((*ela).serial < TEST_LIST_LEN as u32, "incorrect serial");
    assert!((*elb).serial < TEST_LIST_LEN as u32, "incorrect serial");
    assert_eq!(*elts.add((*ela).serial as usize), ela as *mut debug_el, "phantom element");
    assert_eq!(*elts.add((*elb).serial as usize), elb as *mut debug_el, "phantom element");
    assert_eq!((*ela).poison1, TEST_POISON1, "bad poison");
    assert_eq!((*ela).poison2, TEST_POISON2, "bad poison");
    assert_eq!((*elb).poison1, TEST_POISON1, "bad poison");
    assert_eq!((*elb).poison2, TEST_POISON2, "bad poison");
}

/* `priv` is the test pointer so check() can fail the test if the list is invalid. */
unsafe extern "C" fn cmp(
    priv_: *mut core::ffi::c_void,
    a: *const list_head,
    b: *const list_head,
) -> i32 {
    let ela = (a as *const u8).sub(core::mem::offset_of!(debug_el, list)) as *const debug_el;
    let elb = (b as *const u8).sub(core::mem::offset_of!(debug_el, list)) as *const debug_el;

    check(priv_ as *mut kunit, ela, elb);
    (*ela).value.wrapping_sub((*elb).value)
}

unsafe fn list_sort_test(test: *mut kunit) {
    let mut count = 1;
    let mut head = list_head {
        next: core::ptr::null_mut(),
        prev: core::ptr::null_mut(),
    };
    head.next = &mut head;
    head.prev = &mut head;

    let elts = kunit_kcalloc(test, TEST_LIST_LEN, core::mem::size_of::<*mut debug_el>(), 0);
    assert!(!elts.is_null());
    (*test).priv_ = elts as *mut core::ffi::c_void;

    for i in 0..TEST_LIST_LEN {
        let el = kunit_kmalloc(test, core::mem::size_of::<debug_el>(), 0);
        assert!(!el.is_null());
        (*el).value = get_random_u32_below((TEST_LIST_LEN / 3) as u32) as i32;
        (*el).serial = i as u32;
        (*el).poison1 = TEST_POISON1;
        (*el).poison2 = TEST_POISON2;
        *elts.add(i) = el;
        list_add_tail(&mut (*el).list, &mut head);
    }

    list_sort(test, &mut head, cmp);

    let mut cur = head.next;
    while (*cur).next != &mut head {
        let next = (*cur).next;
        assert_eq!((*next).prev, cur, "list is corrupted");
        let cmp_result = cmp(test as *mut core::ffi::c_void, cur, next);
        assert!(cmp_result <= 0, "list is not sorted");
        let el = (cur as *const u8).sub(core::mem::offset_of!(debug_el, list)) as *const debug_el;
        let el1 = (next as *const u8).sub(core::mem::offset_of!(debug_el, list)) as *const debug_el;
        if cmp_result == 0 {
            assert!((*el).serial <= (*el1).serial, "order of equivalent elements not preserved");
        }
        check(test, el, el1);
        count += 1;
        cur = next;
    }
    assert_eq!(head.prev, cur, "list is corrupted");
    assert_eq!(count, TEST_LIST_LEN, "list length changed after sorting!");
}

#[repr(C)]
pub struct kunit_case {
    pub run_case: unsafe fn(*mut kunit),
}

#[repr(C)]
pub struct kunit_suite {
    pub name: &'static [u8],
    pub test_cases: *const kunit_case,
}

pub static LIST_SORT_CASES: [kunit_case; 1] = [kunit_case {
    run_case: list_sort_test,
}];

pub static LIST_SORT_SUITE: kunit_suite = kunit_suite {
    name: b"list_sort\0",
    test_cases: LIST_SORT_CASES.as_ptr(),
};

pub const MODULE_DESCRIPTION: &str = "list_sort() KUnit test suite";
pub const MODULE_LICENSE: &str = "GPL";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
