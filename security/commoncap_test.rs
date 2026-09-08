// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * KUnit tests for commoncap.c security functions
 *
 * Tests for security-critical functions in the capability subsystem,
 * particularly namespace-related capability checks.
 */

// Linux kernel headers (external dependencies):
// #include <kunit/test.h>
// #include <linux/user_namespace.h>
// #include <linux/uidgid.h>
// #include <linux/cred.h>
// #include <linux/mnt_idmapping.h>
// #include <linux/module.h>
// #include <linux/slab.h>
// #include <linux/refcount.h>

// CONFIG_SECURITY_COMMONCAP_KUNIT_TEST conditional compilation in Rust comments

extern "C" {
    // KUnit test context type from kunit/test.h
    pub struct kunit;

    // Types from linux/uidgid.h and linux/user_namespace.h
    pub type vfsuid_t;
    pub type kuid_t;
    pub type kgid_t;

    // struct ns_common from linux/ns_common.h
    #[repr(C)]
    pub struct ns_common {
        pub ns_ref: u32, // refcount field
        pub inum: u32,
    }

    // struct uid_gid_extent from linux/user_namespace.h
    #[repr(C)]
    pub struct uid_gid_extent {
        pub first: u32,
        pub lower_first: u32,
        pub count: u32,
    }

    // struct user_namespace from linux/user_namespace.h
    #[repr(C)]
    pub struct user_namespace {
        pub parent: *mut user_namespace,
        pub level: u32,
        pub owner: kuid_t,
        pub group: kgid_t,
        pub ns: ns_common,
        pub uid_map: uidgid_map,
        pub gid_map: uidgid_map,
    }

    #[repr(C)]
    pub struct uidgid_map {
        pub extent: [uid_gid_extent; 340],
        pub nr_extents: u32,
    }

    // External kernel globals
    pub static init_user_ns: user_namespace;

    // Functions from commoncap.c (static but accessible during KUnit)
    pub fn vfsuid_root_in_currentns(vfsuid: vfsuid_t) -> bool;
    pub fn kuid_root_in_ns(kuid: kuid_t, ns: *mut user_namespace) -> bool;

    // Macros converted to functions
    pub fn KUIDT_INIT(val: u32) -> kuid_t;
    pub fn KGIDT_INIT(val: u32) -> kgid_t;
    pub fn VFSUIDT_INIT(kuid: kuid_t) -> vfsuid_t;
    pub static INVALID_VFSUID: vfsuid_t;
    pub fn __kuid_val(kuid: kuid_t) -> u32;

    // KUnit allocation and refcount functions
    pub fn kunit_kzalloc(test: *mut kunit, size: usize, gfp_flags: u32) -> *mut core::ffi::c_void;
    pub fn refcount_set(r: *mut u32, n: u32);

    // KUnit expectation macros - exposed as functions
    pub fn __kunit_do_expect_bool(test: *mut kunit, condition: bool, expected: bool, msg: *const u8);
    pub fn __kunit_do_assertion(test: *mut kunit, condition: bool, expected: bool, msg: *const u8);

    // GFP_KERNEL constant
    pub static GFP_KERNEL: u32;
}

const GFP_KERNEL_VALUE: u32 = 0x00000000; // Kernel memory allocation flag

/* Functions are static in commoncap.c, but we can call them since we're
 * included in the same compilation unit when tests are enabled.
 */

/// test_vfsuid_root_in_currentns_init_ns - Test vfsuid_root_in_currentns with init ns
///
/// Verifies that UID 0 in the init namespace correctly owns the current
/// namespace when running in init_user_ns.
///
/// test: KUnit test context
unsafe fn test_vfsuid_root_in_currentns_init_ns(test: *mut kunit) {
    let vfsuid: vfsuid_t;
    let kuid: kuid_t;

    /* Create UID 0 in init namespace */
    kuid = KUIDT_INIT(0);
    vfsuid = VFSUIDT_INIT(kuid);

    /* In init namespace, UID 0 should own current namespace */
    let result = vfsuid_root_in_currentns(vfsuid);
    __kunit_do_expect_bool(test, result, true, "vfsuid_root_in_currentns(vfsuid)".as_ptr() as *const u8);
}

/// test_vfsuid_root_in_currentns_invalid - Test vfsuid_root_in_currentns with invalid vfsuid
///
/// Verifies that an invalid vfsuid correctly returns false.
///
/// test: KUnit test context
unsafe fn test_vfsuid_root_in_currentns_invalid(test: *mut kunit) {
    let invalid_vfsuid: vfsuid_t;

    /* Use the predefined invalid vfsuid */
    invalid_vfsuid = INVALID_VFSUID;

    /* Invalid vfsuid should return false */
    let result = vfsuid_root_in_currentns(invalid_vfsuid);
    __kunit_do_expect_bool(test, result, false, "vfsuid_root_in_currentns(invalid_vfsuid)".as_ptr() as *const u8);
}

/// test_vfsuid_root_in_currentns_nonzero - Test vfsuid_root_in_currentns with non-zero UID
///
/// Verifies that a non-zero UID correctly returns false.
///
/// test: KUnit test context
unsafe fn test_vfsuid_root_in_currentns_nonzero(test: *mut kunit) {
    let vfsuid: vfsuid_t;
    let kuid: kuid_t;

    /* Create a non-zero UID */
    kuid = KUIDT_INIT(1000);
    vfsuid = VFSUIDT_INIT(kuid);

    /* Non-zero UID should return false */
    let result = vfsuid_root_in_currentns(vfsuid);
    __kunit_do_expect_bool(test, result, false, "vfsuid_root_in_currentns(vfsuid)".as_ptr() as *const u8);
}

/// test_kuid_root_in_ns_init_ns_uid0 - Test kuid_root_in_ns with init namespace and UID 0
///
/// Verifies that kuid_root_in_ns correctly identifies UID 0 in init namespace.
/// This tests the core namespace traversal logic. In init namespace, UID 0
/// maps to itself, so it should own the namespace.
///
/// test: KUnit test context
unsafe fn test_kuid_root_in_ns_init_ns_uid0(test: *mut kunit) {
    let kuid: kuid_t;
    let init_ns: *mut user_namespace;

    kuid = KUIDT_INIT(0);
    init_ns = &init_user_ns as *const _ as *mut _;

    /* UID 0 should own init namespace */
    let result = kuid_root_in_ns(kuid, init_ns);
    __kunit_do_expect_bool(test, result, true, "kuid_root_in_ns(kuid, init_ns)".as_ptr() as *const u8);
}

/// test_kuid_root_in_ns_init_ns_nonzero - Test kuid_root_in_ns with init namespace and non-zero UID
///
/// Verifies that kuid_root_in_ns correctly rejects non-zero UIDs in init namespace.
/// Only UID 0 should own a namespace.
///
/// test: KUnit test context
unsafe fn test_kuid_root_in_ns_init_ns_nonzero(test: *mut kunit) {
    let kuid: kuid_t;
    let init_ns: *mut user_namespace;

    kuid = KUIDT_INIT(1000);
    init_ns = &init_user_ns as *const _ as *mut _;

    /* Non-zero UID should not own namespace */
    let result = kuid_root_in_ns(kuid, init_ns);
    __kunit_do_expect_bool(test, result, false, "kuid_root_in_ns(kuid, init_ns)".as_ptr() as *const u8);
}

/// create_test_user_ns_with_mapping - Create a mock user namespace with UID mapping
///
/// Creates a minimal user namespace structure for testing where uid 0 in the
/// namespace maps to a specific kuid in the parent namespace.
///
/// test: KUnit test context
/// parent_ns: Parent namespace (typically init_user_ns)
/// mapped_kuid: The kuid that uid 0 in this namespace maps to in parent
///
/// Returns: Pointer to allocated namespace, or NULL on failure
unsafe fn create_test_user_ns_with_mapping(
    test: *mut kunit,
    parent_ns: *mut user_namespace,
    mapped_kuid: kuid_t,
) -> *mut user_namespace {
    let ns: *mut user_namespace;
    let mut extent: uid_gid_extent;

    /* Allocate a test namespace - use kzalloc to zero all fields */
    ns = kunit_kzalloc(test, core::mem::size_of::<user_namespace>(), GFP_KERNEL_VALUE) as *mut user_namespace;
    if ns.is_null() {
        return core::ptr::null_mut();
    }

    /* Initialize basic namespace structure fields */
    (*ns).parent = parent_ns;
    (*ns).level = if !parent_ns.is_null() {
        (*parent_ns).level + 1
    } else {
        0
    };
    (*ns).owner = mapped_kuid;
    (*ns).group = KGIDT_INIT(0);

    /* Initialize ns_common structure */
    refcount_set(&mut (*ns).ns.ns_ref as *mut u32, 1);
    (*ns).ns.inum = 0; /* Mock inum */

    /* Set up uid mapping: uid 0 in this namespace maps to mapped_kuid in parent
     * Format: first (uid in ns) : lower_first (kuid in parent) : count
     * So: uid 0 in ns -> kuid mapped_kuid in parent
     * This means from_kuid(ns, mapped_kuid) returns 0
     */
    extent.first = 0;                              /* uid 0 in this namespace */
    extent.lower_first = __kuid_val(mapped_kuid);  /* maps to this kuid in parent */
    extent.count = 1;

    (*ns).uid_map.extent[0] = extent;
    (*ns).uid_map.nr_extents = 1;

    /* Set up gid mapping: gid 0 maps to gid 0 in parent (simplified) */
    extent.first = 0;
    extent.lower_first = 0;
    extent.count = 1;

    (*ns).gid_map.extent[0] = extent;
    (*ns).gid_map.nr_extents = 1;

    ns
}

/// test_kuid_root_in_ns_with_mapping - Test kuid_root_in_ns with namespace where uid 0
///                                   maps to different kuid
///
/// Creates a user namespace where uid 0 maps to kuid 1000 in the parent namespace.
/// Verifies that kuid_root_in_ns correctly identifies kuid 1000 as owning the namespace.
///
/// Note: kuid_root_in_ns walks up the namespace hierarchy, so it checks the current
/// namespace first, then parent, then parent's parent, etc. So:
/// - kuid 1000 owns test_ns because from_kuid(test_ns, 1000) == 0
/// - kuid 0 also owns test_ns because from_kuid(init_user_ns, 0) == 0
///   (checked in parent)
///
/// This tests the actual functionality as requested: creating namespaces with
/// different values for the namespace's uid 0.
///
/// test: KUnit test context
unsafe fn test_kuid_root_in_ns_with_mapping(test: *mut kunit) {
    let test_ns: *mut user_namespace;
    let parent_ns: *mut user_namespace;
    let mapped_kuid: kuid_t;
    let other_kuid: kuid_t;

    parent_ns = &init_user_ns as *const _ as *mut _;
    mapped_kuid = KUIDT_INIT(1000);
    other_kuid = KUIDT_INIT(2000);

    test_ns = create_test_user_ns_with_mapping(test, parent_ns, mapped_kuid);
    if test_ns.is_null() {
        return;
    }

    /* kuid 1000 should own test_ns because it maps to uid 0 in test_ns */
    let result = kuid_root_in_ns(mapped_kuid, test_ns);
    __kunit_do_expect_bool(test, result, true, "kuid_root_in_ns(mapped_kuid, test_ns)".as_ptr() as *const u8);

    /* kuid 0 should also own test_ns (checked via parent init_user_ns) */
    let result = kuid_root_in_ns(KUIDT_INIT(0), test_ns);
    __kunit_do_expect_bool(test, result, true, "kuid_root_in_ns(KUIDT_INIT(0), test_ns)".as_ptr() as *const u8);

    /* Other kuids should not own test_ns */
    let result = kuid_root_in_ns(other_kuid, test_ns);
    __kunit_do_expect_bool(test, result, false, "kuid_root_in_ns(other_kuid, test_ns)".as_ptr() as *const u8);

    let result = kuid_root_in_ns(KUIDT_INIT(500), test_ns);
    __kunit_do_expect_bool(test, result, false, "kuid_root_in_ns(KUIDT_INIT(500), test_ns)".as_ptr() as *const u8);
}

/// test_kuid_root_in_ns_with_different_mappings - Test with multiple namespaces
///
/// Creates multiple user namespaces with different UID mappings to verify
/// that kuid_root_in_ns correctly distinguishes between namespaces.
///
/// Each namespace maps uid 0 to a different kuid, and we verify that each
/// kuid only owns its corresponding namespace (plus kuid 0 owns all via
/// init_user_ns parent).
///
/// test: KUnit test context
unsafe fn test_kuid_root_in_ns_with_different_mappings(test: *mut kunit) {
    let ns1: *mut user_namespace;
    let ns2: *mut user_namespace;
    let ns3: *mut user_namespace;

    /* Create three independent namespaces, each mapping uid 0 to different kuids */
    ns1 = create_test_user_ns_with_mapping(test, &init_user_ns as *const _ as *mut _, KUIDT_INIT(1000));
    if ns1.is_null() {
        return;
    }

    ns2 = create_test_user_ns_with_mapping(test, &init_user_ns as *const _ as *mut _, KUIDT_INIT(2000));
    if ns2.is_null() {
        return;
    }

    ns3 = create_test_user_ns_with_mapping(test, &init_user_ns as *const _ as *mut _, KUIDT_INIT(3000));
    if ns3.is_null() {
        return;
    }

    /* Test ns1: kuid 1000 owns it, kuid 0 owns it (via parent), others do not */
    let result = kuid_root_in_ns(KUIDT_INIT(1000), ns1);
    __kunit_do_expect_bool(test, result, true, "kuid_root_in_ns(KUIDT_INIT(1000), ns1)".as_ptr() as *const u8);

    let result = kuid_root_in_ns(KUIDT_INIT(0), ns1);
    __kunit_do_expect_bool(test, result, true, "kuid_root_in_ns(KUIDT_INIT(0), ns1)".as_ptr() as *const u8);

    let result = kuid_root_in_ns(KUIDT_INIT(2000), ns1);
    __kunit_do_expect_bool(test, result, false, "kuid_root_in_ns(KUIDT_INIT(2000), ns1)".as_ptr() as *const u8);

    let result = kuid_root_in_ns(KUIDT_INIT(3000), ns1);
    __kunit_do_expect_bool(test, result, false, "kuid_root_in_ns(KUIDT_INIT(3000), ns1)".as_ptr() as *const u8);

    /* Test ns2: kuid 2000 owns it, kuid 0 owns it (via parent), others do not */
    let result = kuid_root_in_ns(KUIDT_INIT(2000), ns2);
    __kunit_do_expect_bool(test, result, true, "kuid_root_in_ns(KUIDT_INIT(2000), ns2)".as_ptr() as *const u8);

    let result = kuid_root_in_ns(KUIDT_INIT(0), ns2);
    __kunit_do_expect_bool(test, result, true, "kuid_root_in_ns(KUIDT_INIT(0), ns2)".as_ptr() as *const u8);

    let result = kuid_root_in_ns(KUIDT_INIT(1000), ns2);
    __kunit_do_expect_bool(test, result, false, "kuid_root_in_ns(KUIDT_INIT(1000), ns2)".as_ptr() as *const u8);

    let result = kuid_root_in_ns(KUIDT_INIT(3000), ns2);
    __kunit_do_expect_bool(test, result, false, "kuid_root_in_ns(KUIDT_INIT(3000), ns2)".as_ptr() as *const u8);

    /* Test ns3: kuid 3000 owns it, kuid 0 owns it (via parent), others do not */
    let result = kuid_root_in_ns(KUIDT_INIT(3000), ns3);
    __kunit_do_expect_bool(test, result, true, "kuid_root_in_ns(KUIDT_INIT(3000), ns3)".as_ptr() as *const u8);

    let result = kuid_root_in_ns(KUIDT_INIT(0), ns3);
    __kunit_do_expect_bool(test, result, true, "kuid_root_in_ns(KUIDT_INIT(0), ns3)".as_ptr() as *const u8);

    let result = kuid_root_in_ns(KUIDT_INIT(1000), ns3);
    __kunit_do_expect_bool(test, result, false, "kuid_root_in_ns(KUIDT_INIT(1000), ns3)".as_ptr() as *const u8);

    let result = kuid_root_in_ns(KUIDT_INIT(2000), ns3);
    __kunit_do_expect_bool(test, result, false, "kuid_root_in_ns(KUIDT_INIT(2000), ns3)".as_ptr() as *const u8);
}

#[repr(C)]
pub struct kunit_case {
    pub name: *const u8,
    pub run: unsafe extern "C" fn(*mut kunit),
}

#[repr(C)]
pub struct kunit_suite {
    pub name: *const u8,
    pub test_cases: *const kunit_case,
}

extern "C" {
    pub fn kunit_test_suite(suite: *const kunit_suite);
}

const COMMONCAP_TEST_CASES: &[kunit_case] = &[
    kunit_case {
        name: "test_vfsuid_root_in_currentns_init_ns".as_ptr(),
        run: test_vfsuid_root_in_currentns_init_ns,
    },
    kunit_case {
        name: "test_vfsuid_root_in_currentns_invalid".as_ptr(),
        run: test_vfsuid_root_in_currentns_invalid,
    },
    kunit_case {
        name: "test_vfsuid_root_in_currentns_nonzero".as_ptr(),
        run: test_vfsuid_root_in_currentns_nonzero,
    },
    kunit_case {
        name: "test_kuid_root_in_ns_init_ns_uid0".as_ptr(),
        run: test_kuid_root_in_ns_init_ns_uid0,
    },
    kunit_case {
        name: "test_kuid_root_in_ns_init_ns_nonzero".as_ptr(),
        run: test_kuid_root_in_ns_init_ns_nonzero,
    },
    kunit_case {
        name: "test_kuid_root_in_ns_with_mapping".as_ptr(),
        run: test_kuid_root_in_ns_with_mapping,
    },
    kunit_case {
        name: "test_kuid_root_in_ns_with_different_mappings".as_ptr(),
        run: test_kuid_root_in_ns_with_different_mappings,
    },
];

static COMMONCAP_TEST_SUITE: kunit_suite = kunit_suite {
    name: "commoncap".as_ptr(),
    test_cases: COMMONCAP_TEST_CASES.as_ptr(),
};

// Module setup
#[no_mangle]
pub extern "C" fn init_commoncap_test_module() {
    unsafe {
        kunit_test_suite(&COMMONCAP_TEST_SUITE);
    }
}

// MODULE_LICENSE("GPL") - represented as a module-level attribute in Rust
// This would be: #![crate_type = "cdylib"] with proper kernel module setup


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
