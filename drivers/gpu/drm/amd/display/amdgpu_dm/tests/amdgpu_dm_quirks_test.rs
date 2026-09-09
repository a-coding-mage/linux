// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * KUnit tests for amdgpu_dm_quirks.c
 *
 * Copyright 2026 Advanced Micro Devices, Inc.
 */

// KUnit, PCI, DC, amdgpu_mode, and amdgpu_dm dependencies are supplied by the
// surrounding kernel translation.

/* Tests for retrieve_dmi_info() */

/*
 * Verify that retrieve_dmi_info() always initialises aux_hpd_discon_quirk to
 * false, even when the caller had previously set it to true.
 */
/// dm_test_quirks_aux_hpd_discon_reset - Test Quirks aux hpd discon reset
/// @test: The KUnit test context
unsafe fn dm_test_quirks_aux_hpd_discon_reset(test: *mut kunit) {
    let dm: *mut amdgpu_display_manager = kunit_kzalloc(
        test,
        core::mem::size_of::<amdgpu_display_manager>(),
        GFP_KERNEL,
    );
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, dm);

    (*dm).aux_hpd_discon_quirk = true;

    retrieve_dmi_info(dm);

    /*
     * In a KUnit / UML environment no real DMI table is present, so
     * dmi_check_system() returns 0 and retrieve_dmi_info() leaves the
     * quirk at its initialised-to-false value.
     */
    KUNIT_EXPECT_FALSE!(test, (*dm).aux_hpd_discon_quirk);
}

/*
 * Verify that retrieve_dmi_info() always initialises edp0_on_dp1_quirk to
 * false, even when the caller had previously set it to true.
 */
/// dm_test_quirks_edp0_on_dp1_reset - Test Quirks edp0 on dp1 reset
/// @test: The KUnit test context
unsafe fn dm_test_quirks_edp0_on_dp1_reset(test: *mut kunit) {
    let dm: *mut amdgpu_display_manager = kunit_kzalloc(
        test,
        core::mem::size_of::<amdgpu_display_manager>(),
        GFP_KERNEL,
    );
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, dm);

    (*dm).edp0_on_dp1_quirk = true;

    retrieve_dmi_info(dm);

    KUNIT_EXPECT_FALSE!(test, (*dm).edp0_on_dp1_quirk);
}

/*
 * Verify that when no DMI match is found both quirks remain false after a
 * fresh (zero-initialised) dm is passed to retrieve_dmi_info().
 */
/// dm_test_quirks_no_dmi_match_both_false - Test Quirks no dmi match both false
/// @test: The KUnit test context
unsafe fn dm_test_quirks_no_dmi_match_both_false(test: *mut kunit) {
    let dm: *mut amdgpu_display_manager = kunit_kzalloc(
        test,
        core::mem::size_of::<amdgpu_display_manager>(),
        GFP_KERNEL,
    );
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, dm);

    retrieve_dmi_info(dm);

    KUNIT_EXPECT_FALSE!(test, (*dm).aux_hpd_discon_quirk);
    KUNIT_EXPECT_FALSE!(test, (*dm).edp0_on_dp1_quirk);
}

/* Tests for dm_should_disable_stutter() */

/// dm_test_should_disable_stutter_match - Test the quirk device matches
/// @test: The KUnit test context
unsafe fn dm_test_should_disable_stutter_match(test: *mut kunit) {
    let pdev: *mut pci_dev = kunit_kzalloc(
        test,
        core::mem::size_of::<pci_dev>(),
        GFP_KERNEL,
    );
    KUNIT_ASSERT_NOT_NULL!(test, pdev);

    (*pdev).vendor = 0x1002;
    (*pdev).device = 0x15dd;
    (*pdev).subsystem_vendor = 0x1002;
    (*pdev).subsystem_device = 0x15dd;
    (*pdev).revision = 0xc8;

    KUNIT_EXPECT_TRUE!(test, dm_should_disable_stutter(pdev));
}

/// dm_test_should_disable_stutter_no_match - Test a non-quirk device does not match
/// @test: The KUnit test context
unsafe fn dm_test_should_disable_stutter_no_match(test: *mut kunit) {
    let pdev: *mut pci_dev = kunit_kzalloc(
        test,
        core::mem::size_of::<pci_dev>(),
        GFP_KERNEL,
    );
    KUNIT_ASSERT_NOT_NULL!(test, pdev);

    (*pdev).vendor = 0x1002;
    (*pdev).device = 0x1234;

    KUNIT_EXPECT_FALSE!(test, dm_should_disable_stutter(pdev));
}

/// dm_test_should_disable_stutter_revision_differs - Test a partial match (revision) fails
/// @test: The KUnit test context
unsafe fn dm_test_should_disable_stutter_revision_differs(test: *mut kunit) {
    let pdev: *mut pci_dev = kunit_kzalloc(
        test,
        core::mem::size_of::<pci_dev>(),
        GFP_KERNEL,
    );
    KUNIT_ASSERT_NOT_NULL!(test, pdev);

    /* Everything matches the quirk except the revision */
    (*pdev).vendor = 0x1002;
    (*pdev).device = 0x15dd;
    (*pdev).subsystem_vendor = 0x1002;
    (*pdev).subsystem_device = 0x15dd;
    (*pdev).revision = 0x00;

    KUNIT_EXPECT_FALSE!(test, dm_should_disable_stutter(pdev));
}

static mut amdgpu_dm_quirks_tests: [kunit_case; 7] = [
    /* retrieve_dmi_info */
    KUNIT_CASE!(dm_test_quirks_aux_hpd_discon_reset),
    KUNIT_CASE!(dm_test_quirks_edp0_on_dp1_reset),
    KUNIT_CASE!(dm_test_quirks_no_dmi_match_both_false),
    /* dm_should_disable_stutter */
    KUNIT_CASE!(dm_test_should_disable_stutter_match),
    KUNIT_CASE!(dm_test_should_disable_stutter_no_match),
    KUNIT_CASE!(dm_test_should_disable_stutter_revision_differs),
    kunit_case::default(),
];

static mut amdgpu_dm_quirks_test_suite: kunit_suite = kunit_suite {
    name: "amdgpu_dm_quirks\0",
    test_cases: unsafe { &mut amdgpu_dm_quirks_tests },
};

kunit_test_suite!(amdgpu_dm_quirks_test_suite);

module_author!("AMD");
module_description!("KUnit tests for amdgpu_dm_quirks");
module_license!("Dual MIT/GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
