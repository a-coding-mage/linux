// SPDX-License-Identifier: GPL-2.0-only
// Dependencies supplied by the Linux KUnit and page-table headers are kept as
// external Rust symbols/macros.

#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kunit_case {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kunit_suite {
    pub name: *const u8,
    pub test_cases: *mut kunit_case,
}

extern "C" {
    fn is_lazy_mmu_mode_active() -> bool;
    fn lazy_mmu_mode_enable();
    fn lazy_mmu_mode_disable();
    fn lazy_mmu_mode_pause();
    fn lazy_mmu_mode_resume();
}

unsafe fn expect_not_active(test: *mut kunit) {
    KUNIT_EXPECT_FALSE!(test, is_lazy_mmu_mode_active());
}

unsafe fn expect_active(test: *mut kunit) {
    KUNIT_EXPECT_TRUE!(test, is_lazy_mmu_mode_active());
}

unsafe fn lazy_mmu_mode_active(test: *mut kunit) {
    expect_not_active(test);

    lazy_mmu_mode_enable();
    expect_active(test);

    {
        /* Nested section */
        lazy_mmu_mode_enable();
        expect_active(test);

        lazy_mmu_mode_disable();
        expect_active(test);
    }

    {
        /* Paused section */
        lazy_mmu_mode_pause();
        expect_not_active(test);

        {
            /* No effect (paused) */
            lazy_mmu_mode_enable();
            expect_not_active(test);

            lazy_mmu_mode_disable();
            expect_not_active(test);

            lazy_mmu_mode_pause();
            expect_not_active(test);

            lazy_mmu_mode_resume();
            expect_not_active(test);
        }

        lazy_mmu_mode_resume();
        expect_active(test);
    }

    lazy_mmu_mode_disable();
    expect_not_active(test);
}

static mut lazy_mmu_mode_test_cases: [kunit_case; 2] = [
    KUNIT_CASE!(lazy_mmu_mode_active),
    kunit_case { _private: [] },
];

static mut lazy_mmu_mode_test_suite: kunit_suite = kunit_suite {
    name: b"lazy_mmu_mode\0".as_ptr(),
    test_cases: unsafe { lazy_mmu_mode_test_cases.as_mut_ptr() },
};

kunit_test_suite!(lazy_mmu_mode_test_suite);

// MODULE_IMPORT_NS("EXPORTED_FOR_KUNIT_TESTING");
// MODULE_DESCRIPTION("Tests for the lazy MMU mode");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
