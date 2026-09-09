// SPDX-License-Identifier: GPL-2.0
/*
 * KUnit test for the Kernel Hashtable structures.
 *
 * Copyright (C) 2022, Google LLC.
 * Author: Rae Moar <rmoar@google.com>
 */
// Dependencies supplied by the kernel and KUnit environment are intentionally
// left external to this translation.

#[repr(C)]
struct hashtable_test_entry {
    key: ::core::ffi::c_int,
    data: ::core::ffi::c_int,
    node: hlist_node,
    visited: ::core::ffi::c_int,
}

unsafe fn hashtable_test_hash_init(test: *mut kunit) {
    /* Test the different ways of initialising a hashtable. */
    let mut hash1 = DEFINE_HASHTABLE!(2);
    let mut hash2 = DECLARE_HASHTABLE!(3);

    /* When using DECLARE_HASHTABLE, must use hash_init to
     * initialize the hashtable.
     */
    hash_init!(hash2);

    KUNIT_EXPECT_TRUE!(test, hash_empty!(hash1));
    KUNIT_EXPECT_TRUE!(test, hash_empty!(hash2));
}

unsafe fn hashtable_test_hash_empty(test: *mut kunit) {
    let mut a: hashtable_test_entry = ::core::mem::zeroed();
    let mut hash = DEFINE_HASHTABLE!(1);

    KUNIT_EXPECT_TRUE!(test, hash_empty!(hash));

    a.key = 1;
    a.data = 13;
    hash_add!(hash, &mut a.node, a.key);

    /* Hashtable should no longer be empty. */
    KUNIT_EXPECT_FALSE!(test, hash_empty!(hash));
}

unsafe fn hashtable_test_hash_hashed(test: *mut kunit) {
    let mut a: hashtable_test_entry = ::core::mem::zeroed();
    let mut b: hashtable_test_entry = ::core::mem::zeroed();
    let mut hash = DEFINE_HASHTABLE!(4);

    a.key = 1;
    a.data = 13;
    hash_add!(hash, &mut a.node, a.key);
    b.key = 1;
    b.data = 2;
    hash_add!(hash, &mut b.node, b.key);

    KUNIT_EXPECT_TRUE!(test, hash_hashed!(&mut a.node));
    KUNIT_EXPECT_TRUE!(test, hash_hashed!(&mut b.node));
}

unsafe fn hashtable_test_hash_add(test: *mut kunit) {
    let mut a: hashtable_test_entry = ::core::mem::zeroed();
    let mut b: hashtable_test_entry = ::core::mem::zeroed();
    let mut hash = DEFINE_HASHTABLE!(3);

    a.key = 1;
    a.data = 13;
    a.visited = 0;
    hash_add!(hash, &mut a.node, a.key);
    b.key = 2;
    b.data = 10;
    b.visited = 0;
    hash_add!(hash, &mut b.node, b.key);

    hash_for_each!(hash, bkt, x, node, {
        x.visited += 1;
        if x.key == a.key {
            KUNIT_EXPECT_EQ!(test, x.data, 13);
        } else if x.key == b.key {
            KUNIT_EXPECT_EQ!(test, x.data, 10);
        } else {
            KUNIT_FAIL!(test, "Unexpected key in hashtable.");
        }
    });

    /* Both entries should have been visited exactly once. */
    KUNIT_EXPECT_EQ!(test, a.visited, 1);
    KUNIT_EXPECT_EQ!(test, b.visited, 1);
}

unsafe fn hashtable_test_hash_del(test: *mut kunit) {
    let mut a: hashtable_test_entry = ::core::mem::zeroed();
    let mut b: hashtable_test_entry = ::core::mem::zeroed();
    let mut hash = DEFINE_HASHTABLE!(6);

    a.key = 1;
    a.data = 13;
    hash_add!(hash, &mut a.node, a.key);
    b.key = 2;
    b.data = 10;
    b.visited = 0;
    hash_add!(hash, &mut b.node, b.key);

    hash_del!(&mut b.node);
    hash_for_each_possible!(hash, x, node, b.key, {
        x.visited += 1;
        KUNIT_EXPECT_NE!(test, x.key, b.key);
    });

    /* The deleted entry should not have been visited. */
    KUNIT_EXPECT_EQ!(test, b.visited, 0);

    hash_del!(&mut a.node);

    /* The hashtable should be empty. */
    KUNIT_EXPECT_TRUE!(test, hash_empty!(hash));
}

unsafe fn hashtable_test_hash_for_each(test: *mut kunit) {
    let mut entries: [hashtable_test_entry; 3] = ::core::mem::zeroed();
    let mut hash = DEFINE_HASHTABLE!(3);

    /* Add three entries to the hashtable. */
    for i in 0..3 {
        entries[i].key = i;
        entries[i].data = i + 10;
        entries[i].visited = 0;
        hash_add!(hash, &mut entries[i].node, entries[i].key);
    }

    let mut count = 0;
    hash_for_each!(hash, bkt, x, node, {
        x.visited += 1;
        KUNIT_ASSERT_GE_MSG!(test, x.key, 0, "Unexpected key in hashtable.");
        KUNIT_ASSERT_LT_MSG!(test, x.key, 3, "Unexpected key in hashtable.");
        count += 1;
    });

    /* Should have visited each entry exactly once. */
    KUNIT_EXPECT_EQ!(test, count, 3);
    for j in 0..3 {
        KUNIT_EXPECT_EQ!(test, entries[j].visited, 1);
    }
}

unsafe fn hashtable_test_hash_for_each_safe(test: *mut kunit) {
    let mut entries: [hashtable_test_entry; 3] = ::core::mem::zeroed();
    let mut hash = DEFINE_HASHTABLE!(3);

    /* Add three entries to the hashtable. */
    for i in 0..3 {
        entries[i].key = i;
        entries[i].data = i + 10;
        entries[i].visited = 0;
        hash_add!(hash, &mut entries[i].node, entries[i].key);
    }

    let mut count = 0;
    hash_for_each_safe!(hash, bkt, tmp, x, node, {
        x.visited += 1;
        KUNIT_ASSERT_GE_MSG!(test, x.key, 0, "Unexpected key in hashtable.");
        KUNIT_ASSERT_LT_MSG!(test, x.key, 3, "Unexpected key in hashtable.");
        count += 1;

        /* Delete entry during loop. */
        hash_del!(&mut x.node);
    });

    /* Should have visited each entry exactly once. */
    KUNIT_EXPECT_EQ!(test, count, 3);
    for j in 0..3 {
        KUNIT_EXPECT_EQ!(test, entries[j].visited, 1);
    }
}

unsafe fn hashtable_test_hash_for_each_possible(test: *mut kunit) {
    let mut entries: [hashtable_test_entry; 4] = ::core::mem::zeroed();
    let mut buckets: [::core::ffi::c_int; 2] = ::core::mem::zeroed();
    let mut hash = DEFINE_HASHTABLE!(5);

    /* Add three entries with key = 0 to the hashtable. */
    for i in 0..3 {
        entries[i].key = 0;
        entries[i].data = i;
        entries[i].visited = 0;
        hash_add!(hash, &mut entries[i].node, entries[i].key);
    }

    /* Add an entry with key = 1. */
    entries[3].key = 1;
    entries[3].data = 3;
    entries[3].visited = 0;
    hash_add!(hash, &mut entries[3].node, entries[3].key);

    let mut count = 0;
    hash_for_each_possible!(hash, x, node, 0, {
        x.visited += 1;
        KUNIT_ASSERT_GE_MSG!(test, x.data, 0, "Unexpected data in hashtable.");
        KUNIT_ASSERT_LT_MSG!(test, x.data, 4, "Unexpected data in hashtable.");
        count += 1;
    });

    /* Should have visited each entry with key = 0 exactly once. */
    for j in 0..3 {
        KUNIT_EXPECT_EQ!(test, entries[j].visited, 1);
    }

    /* Save the buckets for the different keys. */
    hash_for_each!(hash, bkt, y, node, {
        KUNIT_ASSERT_GE_MSG!(test, y.key, 0, "Unexpected key in hashtable.");
        KUNIT_ASSERT_LE_MSG!(test, y.key, 1, "Unexpected key in hashtable.");
        buckets[y.key as usize] = bkt;
    });

    /* If entry with key = 1 is in the same bucket as the entries with
     * key = 0, check it was visited. Otherwise ensure that only three
     * entries were visited.
     */
    if buckets[0] == buckets[1] {
        KUNIT_EXPECT_EQ!(test, count, 4);
        KUNIT_EXPECT_EQ!(test, entries[3].visited, 1);
    } else {
        KUNIT_EXPECT_EQ!(test, count, 3);
        KUNIT_EXPECT_EQ!(test, entries[3].visited, 0);
    }
}

unsafe fn hashtable_test_hash_for_each_possible_safe(test: *mut kunit) {
    let mut entries: [hashtable_test_entry; 4] = ::core::mem::zeroed();
    let mut buckets: [::core::ffi::c_int; 2] = ::core::mem::zeroed();
    let mut hash = DEFINE_HASHTABLE!(5);

    /* Add three entries with key = 0 to the hashtable. */
    for i in 0..3 {
        entries[i].key = 0;
        entries[i].data = i;
        entries[i].visited = 0;
        hash_add!(hash, &mut entries[i].node, entries[i].key);
    }

    /* Add an entry with key = 1. */
    entries[3].key = 1;
    entries[3].data = 3;
    entries[3].visited = 0;
    hash_add!(hash, &mut entries[3].node, entries[3].key);

    let mut count = 0;
    hash_for_each_possible_safe!(hash, x, tmp, node, 0, {
        x.visited += 1;
        KUNIT_ASSERT_GE_MSG!(test, x.data, 0, "Unexpected data in hashtable.");
        KUNIT_ASSERT_LT_MSG!(test, x.data, 4, "Unexpected data in hashtable.");
        count += 1;

        /* Delete entry during loop. */
        hash_del!(&mut x.node);
    });

    /* Should have visited each entry with key = 0 exactly once. */
    for j in 0..3 {
        KUNIT_EXPECT_EQ!(test, entries[j].visited, 1);
    }

    /* Save the buckets for the different keys. */
    hash_for_each!(hash, bkt, y, node, {
        KUNIT_ASSERT_GE_MSG!(test, y.key, 0, "Unexpected key in hashtable.");
        KUNIT_ASSERT_LE_MSG!(test, y.key, 1, "Unexpected key in hashtable.");
        buckets[y.key as usize] = bkt;
    });

    /* If entry with key = 1 is in the same bucket as the entries with
     * key = 0, check it was visited. Otherwise ensure that only three
     * entries were visited.
     */
    if buckets[0] == buckets[1] {
        KUNIT_EXPECT_EQ!(test, count, 4);
        KUNIT_EXPECT_EQ!(test, entries[3].visited, 1);
    } else {
        KUNIT_EXPECT_EQ!(test, count, 3);
        KUNIT_EXPECT_EQ!(test, entries[3].visited, 0);
    }
}

static mut hashtable_test_cases: [kunit_case; 10] = [
    KUNIT_CASE!(hashtable_test_hash_init),
    KUNIT_CASE!(hashtable_test_hash_empty),
    KUNIT_CASE!(hashtable_test_hash_hashed),
    KUNIT_CASE!(hashtable_test_hash_add),
    KUNIT_CASE!(hashtable_test_hash_del),
    KUNIT_CASE!(hashtable_test_hash_for_each),
    KUNIT_CASE!(hashtable_test_hash_for_each_safe),
    KUNIT_CASE!(hashtable_test_hash_for_each_possible),
    KUNIT_CASE!(hashtable_test_hash_for_each_possible_safe),
    KUNIT_CASE_END!,
];

static mut hashtable_test_module: kunit_suite = kunit_suite {
    name: "hashtable",
    test_cases: hashtable_test_cases.as_ptr(),
};

kunit_test_suites!(&mut hashtable_test_module);

MODULE_DESCRIPTION!("KUnit test for the Kernel Hashtable structures");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
