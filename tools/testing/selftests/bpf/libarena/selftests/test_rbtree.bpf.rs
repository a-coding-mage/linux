// SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause

// Translated from C source using libarena/common.h, libarena/asan.h, and
// libarena/rbtree.h dependencies supplied externally.

type u64 = u64;
type size_t = usize;

const EINVAL: i32 = 22;
const EBUSY: i32 = 16;
const ENOMEM: i32 = 12;
const EALREADY: i32 = 114;

#[repr(C)]
pub struct rbnode {
    pub key: u64,
    pub value: u64,
}

#[repr(C)]
pub struct rbtree {
    pub root: *mut rbnode,
    pub alloc: rbtree_alloc,
}

pub type node_ctx = *mut node_ctx_struct;

#[repr(C)]
pub struct node_ctx_struct {
    pub rbnode: rbnode,
    pub next: node_ctx,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rbtree_alloc {
    RB_ALLOC,
    RB_NOALLOC,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rbtree_insert_mode {
    RB_DEFAULT,
    RB_UPDATE,
    RB_DUPLICATE,
}

extern "C" {
    static can_loop: bool;

    fn rb_create(alloc: rbtree_alloc, insert: rbtree_insert_mode) -> *mut rbtree;
    fn rb_destroy(rbtree: *mut rbtree) -> i32;
    fn rb_find(rbtree: *mut rbtree, key: u64, value: *mut u64) -> i32;
    fn rb_insert(rbtree: *mut rbtree, key: u64, value: u64) -> i32;
    fn rb_insert_node(rbtree: *mut rbtree, node: *mut rbnode) -> i32;
    fn rb_remove(rbtree: *mut rbtree, key: u64) -> i32;
    fn rb_remove_node(rbtree: *mut rbtree, node: *mut rbnode) -> i32;
    fn rb_print(rbtree: *mut rbtree);
    fn rb_integrity_check(rbtree: *mut rbtree) -> i32;
    fn rb_least(rbtree: *mut rbtree, key: *mut u64, value: *mut u64) -> i32;
    fn rb_pop(rbtree: *mut rbtree, key: *mut u64, value: *mut u64) -> i32;
    fn rb_node_alloc(key: u64, value: u64) -> *mut rbnode;
    fn rb_node_free(node: *mut rbnode);

    fn arena_malloc(size: size_t) -> *mut core::ffi::c_void;
    fn arena_free(ptr: *mut core::ffi::c_void);
    fn arena_stderr(fmt: *const i8, ...);
}

unsafe fn arena_container_of_rbnode(ptr: *mut rbnode) -> node_ctx {
    ptr as node_ctx
}

static keys: [u64; 61] = [
    51, 43, 37, 3, 301, 46, 383, 990, 776, 729, 871, 96, 189, 213, 376, 167, 131, 939, 626, 119,
    374, 700, 772, 154, 883, 620, 641, 5, 428, 516, 105, 622, 988, 811, 931, 973, 246, 690, 934,
    744, 210, 311, 32, 255, 960, 830, 523, 429, 541, 738, 705, 774, 715, 446, 98, 578, 777, 191,
    279, 91, 767,
];

static morekeys: [u64; 356] = [
    173, 636, 1201, 8642, 5957, 3617, 4586, 8053, 6551, 7592, 1748, 1589, 8644, 9918, 6977, 4448,
    5852, 4640, 9717, 2303, 7424, 7695, 2334, 8876, 8618, 5745, 7134, 2178, 5280, 2140, 1138,
    5083, 8922, 1516, 2437, 2488, 4307, 4329, 5088, 8456, 5938, 1441, 1684, 5750, 721, 1107,
    2089, 9737, 4687, 5016, 4849, 8193, 9603, 9147, 5992, 166, 6721, 812, 4144, 6237, 6509, 3466,
    9255, 7767, 3960, 6759, 2968, 6046, 9784, 8395, 2619, 1711, 528, 6424, 9084, 3179, 1342,
    5676, 9445, 5691, 6678, 8487, 1627, 998, 6178, 2229, 1987, 3319, 572, 169, 2161, 3018, 5439,
    7287, 7265, 5995, 5003, 5857, 2836, 5634, 4735, 9261, 8287, 5359, 533, 1406, 9573, 4026, 714,
    3956, 1722, 6395, 9648, 3887, 7185, 470, 4482, 4997, 841, 8913, 9946, 3999, 9357, 9847, 277,
    8184, 8704, 6766, 3323, 5468, 8638, 7905, 8858, 6142, 3685, 3452, 4689, 8878, 8836, 158, 831,
    7914, 3031, 8374, 4921, 4207, 3460, 5547, 3358, 1083, 4619, 7818, 2962, 4879, 4583, 2172,
    8819, 9830, 1194, 2666, 9812, 5704, 8432, 5916, 6007, 6609, 4791, 1985, 3226, 2478, 9605,
    5236, 8079, 3042, 1965, 3539, 9704, 4267, 6416, 760, 9968, 2983, 1190, 1964, 3211, 2870,
    3106, 2794, 1542, 6916, 5986, 9096, 441, 5894, 8353, 7765, 3757, 5732, 88, 3091, 5637, 6042,
    8447, 4073, 6923, 5491, 7010, 3663, 5029, 6162, 822, 4874, 7491, 5100, 3461, 6983, 2170,
    1458, 1856, 648, 6272, 4887, 976, 2369, 5909, 4274, 3324, 6968, 2312, 2271, 8891, 6268, 6581,
    1610, 8880, 6194, 6144, 9764, 6915, 829, 3774, 2265, 1752, 1314, 6377, 8760, 8004, 501, 4912,
    9278, 1425, 9578, 7337, 307, 1885, 3151, 9617, 1647, 2458, 3702, 6091, 8902, 5663, 9378, 7640,
    3336, 557, 1644, 6848, 1559, 8821, 266, 4330, 9790, 5920, 4222, 1143, 6248, 5792, 4847, 9726,
    6303, 821, 6839, 6062, 7133, 3649, 9888, 2528, 1966, 5456, 4914, 3615, 1543, 3206, 3353, 6097,
    2800, 1424, 9094, 7920, 7243, 1394, 5464, 1707, 576, 6524, 4261, 4187, 7889, 5336, 3377, 2921,
    7244, 2766, 6584, 5514, 1387, 2957, 2258, 1077, 9979, 1128, 876, 4056, 4668, 4532, 1982, 7093,
    4184, 5460, 7588, 4704, 6717, 61, 3959, 1826, 2294, 18, 8170, 9394, 8796, 7288, 7285, 7143,
    148, 6676, 6603, 1051, 8225, 4169, 3230, 7697, 6971, 3454, 7501, 9514, 394, 2339, 4993, 5606,
    6060, 1297, 8273, 3012, 157, 8181, 6765, 7207, 1005, 8833, 1914, 7456, 1846, 8375, 2741, 2074,
    1712, 5286,
];

#[no_mangle]
pub unsafe extern "C" fn test_rbtree_find_nonexistent() -> i32 {
    let key: u64 = 0xdeadbeef;
    let mut value: u64 = 0;
    let mut ret: i32;
    let rbtree: *mut rbtree;

    rbtree = rb_create(rbtree_alloc::RB_ALLOC, rbtree_insert_mode::RB_DEFAULT);
    if rbtree.is_null() {
        return 1;
    }

    /* Should return -EINVAL */
    ret = rb_find(rbtree, key, &mut value);
    if ret == 0 {
        return 2;
    }

    rb_destroy(rbtree)
}

#[no_mangle]
pub unsafe extern "C" fn test_rbtree_insert_existing() -> i32 {
    let key: u64 = 525252;
    let value: u64 = 24;
    let mut ret: i32;
    let rbtree: *mut rbtree;

    rbtree = rb_create(rbtree_alloc::RB_ALLOC, rbtree_insert_mode::RB_DEFAULT);
    if rbtree.is_null() {
        return 1;
    }

    ret = rb_insert(rbtree, key, value);
    if ret != 0 {
        return 2;
    }

    /* Should return -EALREADY. */
    ret = rb_insert(rbtree, key, value);
    if ret != -EALREADY {
        return 3;
    }

    rb_destroy(rbtree)
}

#[no_mangle]
pub unsafe extern "C" fn test_rbtree_update_existing() -> i32 {
    let key: u64 = 33333;
    let mut value: u64;
    let mut ret: i32;
    let rbtree: *mut rbtree;

    rbtree = rb_create(rbtree_alloc::RB_ALLOC, rbtree_insert_mode::RB_UPDATE);
    if rbtree.is_null() {
        return 1;
    }

    value = 52;
    ret = rb_insert(rbtree, key, value);
    if ret != 0 {
        return 2;
    }

    ret = rb_find(rbtree, key, &mut value);
    if ret != 0 {
        return 3;
    }

    if value != 52 {
        return 4;
    }

    value = 65;

    /* Should succeed. */
    ret = rb_insert(rbtree, key, value);
    if ret != 0 {
        return 5;
    }

    /* Should be updated. */
    ret = rb_find(rbtree, key, &mut value);
    if ret != 0 {
        return 6;
    }

    if value != 65 {
        return 7;
    }

    rb_destroy(rbtree)
}

#[no_mangle]
pub unsafe extern "C" fn test_rbtree_insert_one() -> i32 {
    let key: u64 = 202020;
    let mut value: u64 = 0xbadcafe;
    let mut ret: i32;
    let rbtree: *mut rbtree;

    rbtree = rb_create(rbtree_alloc::RB_ALLOC, rbtree_insert_mode::RB_UPDATE);
    if rbtree.is_null() {
        return 1;
    }

    ret = rb_insert(rbtree, key, value);
    if ret != 0 {
        return 2;
    }

    ret = rb_find(rbtree, key, &mut value);
    if ret != 0 {
        return 3;
    }

    if value != 0xbadcafe {
        return 4;
    }

    rb_destroy(rbtree)
}

#[no_mangle]
pub unsafe extern "C" fn test_rbtree_insert_ten() -> i32 {
    let mut key: u64;
    let mut value: u64 = 0;
    let mut ret: i32;
    let mut i: i32;
    let rbtree: *mut rbtree;

    rbtree = rb_create(rbtree_alloc::RB_ALLOC, rbtree_insert_mode::RB_UPDATE);
    if rbtree.is_null() {
        return 1;
    }

    i = 0;
    while i < 10 && can_loop {
        key = keys[i as usize];
        ret = rb_insert(rbtree, key, 2 * key);
        if ret != 0 {
            return 2 + 3 * i;
        }

        /* Read it back. */
        ret = rb_find(rbtree, key, &mut value);
        if ret != 0 {
            return 2 + 3 * i + 1;
        }

        if value != 2 * key {
            return 2 + 3 * i + 2;
        }
        i += 1;
    }

    /* Go find all inserted pairs. */
    i = 0;
    while i < 10 && can_loop {
        key = keys[i as usize];

        ret = rb_find(rbtree, key, &mut value);
        if ret != 0 {
            return 35 + 2 * i;
        }

        if value != 2 * key {
            return 35 + 2 * i + 1;
        }
        i += 1;
    }

    rb_destroy(rbtree)
}

#[no_mangle]
pub unsafe extern "C" fn test_rbtree_duplicate() -> i32 {
    let key: u64 = 0x121212;
    let mut value: u64 = 0;
    let mut ret: i32;
    let mut i: i32;
    let rbtree: *mut rbtree;

    rbtree = rb_create(rbtree_alloc::RB_ALLOC, rbtree_insert_mode::RB_DUPLICATE);
    if rbtree.is_null() {
        return 1;
    }

    i = 0;
    while i < 10 && can_loop {
        ret = rb_insert(rbtree, key, 2 * key);
        if ret != 0 {
            return 2 + 3 * i;
        }

        /* Read it back. */
        ret = rb_find(rbtree, key, &mut value);
        if ret != 0 {
            return 2 + 3 * i + 1;
        }

        if value != 2 * key {
            return 2 + 3 * i + 2;
        }
        i += 1;
    }

    /* Go find all inserted copies and remove them. */
    i = 0;
    while i < 10 && can_loop {
        ret = rb_find(rbtree, key, &mut value);
        if ret != 0 {
            rb_print(rbtree);
            return 35 + 3 * i;
        }

        if value != 2 * key {
            return 35 + 3 * i + 1;
        }

        ret = rb_remove(rbtree, key);
        if ret != 0 {
            return 35 + 3 * i + 2;
        }
        i += 1;
    }

    rb_destroy(rbtree)
}

#[inline]
unsafe fn clean_up_noalloc_tree(rbtree: *mut rbtree) -> i32 {
    let mut nodec: node_ctx;
    let mut ret: i32;

    if (*rbtree).alloc != rbtree_alloc::RB_NOALLOC {
        return -EINVAL;
    }

    /* Can't destroy an RB_NOALLOC tree that still has nodes. */
    if rb_destroy(rbtree) != -EBUSY {
        return -EINVAL;
    }

    while !(*rbtree).root.is_null() && can_loop {
        nodec = arena_container_of_rbnode((*rbtree).root);
        ret = rb_remove_node(rbtree, &mut (*nodec).rbnode);
        if ret != 0 {
            return ret;
        }

        arena_free(nodec as *mut core::ffi::c_void);
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn insert_many(alloc: rbtree_alloc, insert: rbtree_insert_mode) -> i32 {
    let numkeys: size_t = keys.len();
    let mut nodec: node_ctx;
    let mut key: u64;
    let mut value: u64 = 0;
    let mut ret: i32;
    let mut i: i32;
    let rbtree: *mut rbtree;

    rbtree = rb_create(alloc, insert);
    if rbtree.is_null() {
        return 1;
    }

    i = 0;
    while (i as size_t) < numkeys && can_loop {
        key = keys[i as usize];
        if (*rbtree).alloc != rbtree_alloc::RB_ALLOC {
            nodec = arena_malloc(core::mem::size_of::<node_ctx_struct>()) as node_ctx;
            if nodec.is_null() {
                arena_stderr(c"out of memory\n".as_ptr());
                return -ENOMEM;
            }
            (*nodec).rbnode.key = key;
            (*nodec).rbnode.value = 2 * key;
            ret = rb_insert_node(rbtree, &mut (*nodec).rbnode);
        } else {
            ret = rb_insert(rbtree, key, 2 * key);
        }
        if ret != 0 {
            return 2 + 3 * i;
        }

        /* Read it back. */
        ret = rb_find(rbtree, key, &mut value);
        if ret != 0 {
            return 2 + 3 * i + 1;
        }

        if value != 2 * key {
            return 2 + 3 * i + 2;
        }
        i += 1;
    }

    /* Go find all inserted pairs. */
    i = 0;
    while (i as size_t) < numkeys && can_loop {
        key = keys[i as usize];

        ret = rb_find(rbtree, key, &mut value);
        if ret != 0 {
            return 302 + 2 * i;
        }

        if value != 2 * key {
            return 302 + 2 * i + 1;
        }
        i += 1;
    }

    /* RB_ALLOC trees are destroyed while still having elements. */
    if (*rbtree).alloc == rbtree_alloc::RB_ALLOC {
        return rb_destroy(rbtree);
    }

    /* Otherwise manually clean up the tree. */
    if clean_up_noalloc_tree(rbtree) != 0 {
        return 5;
    }

    rb_destroy(rbtree)
}

#[no_mangle]
pub unsafe extern "C" fn test_rbtree_remove_one() -> i32 {
    let key: u64 = 20;
    let value: u64 = 5;
    let mut newvalue: u64 = 0;
    let mut ret: i32;
    let rbtree: *mut rbtree;

    rbtree = rb_create(rbtree_alloc::RB_ALLOC, rbtree_insert_mode::RB_DEFAULT);
    if rbtree.is_null() {
        return 1;
    }

    ret = rb_find(rbtree, key, &mut newvalue);
    if ret == 0 {
        return 2;
    }

    ret = rb_insert(rbtree, key, value);
    if ret != 0 {
        return 3;
    }

    ret = rb_find(rbtree, key, &mut newvalue);
    if ret != 0 || value != newvalue {
        return 4;
    }

    ret = rb_remove(rbtree, key);
    if ret != 0 {
        return 5;
    }

    ret = rb_find(rbtree, key, &mut newvalue);
    if ret == 0 {
        return 6;
    }

    rb_destroy(rbtree)
}

#[inline(always)]
unsafe fn remove_many_verify_all_present(rbtree: *mut rbtree) -> i32 {
    let numkeys: size_t = morekeys.len();
    let mut value: u64 = 0;
    let mut ret: i32;
    let mut i: i32;

    i = 0;
    while (i as size_t) < numkeys && can_loop {
        let key: u64 = morekeys[i as usize];

        ret = rb_find(rbtree, key, &mut value);
        if ret != 0 {
            return -1;
        }

        if value != 2 * key {
            return -1;
        }
        i += 1;
    }

    0
}

#[inline(always)]
unsafe fn remove_many_verify_remaining(rbtree: *mut rbtree) -> i32 {
    let numkeys: size_t = morekeys.len();
    let mut value: u64 = 0;
    let mut ret: i32;
    let mut i: i32;

    i = 0;
    while (i as size_t) < numkeys && can_loop {
        let mut key: u64 = morekeys[i as usize];

        ret = rb_find(rbtree, key, &mut value);
        if ret == 0 {
            return -1;
        }

        if (i as size_t) + 1 >= numkeys {
            break;
        }

        key = morekeys[(i + 1) as usize];
        ret = rb_find(rbtree, key, &mut value);
        if ret != 0 {
            return -1;
        }

        if value != 2 * key {
            return -1;
        }
        i += 2;
    }

    i = 1;
    while (i as size_t) < numkeys && can_loop {
        let key: u64 = morekeys[i as usize];

        ret = rb_find(rbtree, key, &mut value);
        if ret != 0 {
            return -1;
        }

        if value != 2 * key {
            return -1;
        }
        i += 2;
    }

    0
}

#[inline(never)]
unsafe fn remove_many_alloc(rbtree: *mut rbtree) -> i32 {
    let numkeys: size_t = morekeys.len();
    let mut value: u64 = 0;
    let mut ret: i32;
    let mut i: i32;

    i = 0;
    while (i as size_t) < numkeys && can_loop {
        let key: u64 = morekeys[i as usize];

        ret = rb_insert(rbtree, key, 2 * key);
        if ret != 0 {
            return -1;
        }

        if rb_integrity_check(rbtree) != 0 {
            arena_stderr(c"iteration %d\n".as_ptr(), i);
            return -EINVAL;
        }

        ret = rb_find(rbtree, key, &mut value);
        if ret != 0 {
            return -1;
        }

        if value != 2 * key {
            return -1;
        }
        i += 1;
    }

    ret = remove_many_verify_all_present(rbtree);
    if ret != 0 {
        return ret;
    }

    i = 0;
    while (i as size_t) < numkeys && can_loop {
        let key: u64 = morekeys[i as usize];

        ret = rb_remove(rbtree, key);
        if ret != 0 {
            arena_stderr(c"Failed to remove %ld\n".as_ptr(), key);
            return -1;
        }

        ret = rb_find(rbtree, key, &mut value);
        if ret == 0 {
            return -1;
        }
        i += 2;
    }

    remove_many_verify_remaining(rbtree)
}

#[inline(never)]
unsafe fn remove_many_noalloc(rbtree: *mut rbtree) -> i32 {
    let numkeys: size_t = morekeys.len();
    let mut first: node_ctx = core::ptr::null_mut();
    let mut last: node_ctx = core::ptr::null_mut();
    let mut value: u64 = 0;
    let mut ret: i32;
    let mut i: i32;

    i = 0;
    while (i as size_t) < numkeys && can_loop {
        let key: u64 = morekeys[i as usize];
        let nodec: node_ctx = arena_malloc(core::mem::size_of::<node_ctx_struct>()) as node_ctx;

        if nodec.is_null() {
            arena_stderr(c"out of memory\n".as_ptr());
            return -ENOMEM;
        }
        (*nodec).rbnode.key = key;
        (*nodec).rbnode.value = 2 * key;
        (*nodec).next = core::ptr::null_mut();

        if first.is_null() {
            first = nodec;
        }

        if !last.is_null() {
            (*last).next = nodec;
        }
        last = nodec;

        ret = rb_insert_node(rbtree, &mut (*nodec).rbnode);
        if ret != 0 {
            return -1;
        }

        if rb_integrity_check(rbtree) != 0 {
            arena_stderr(c"iteration %d\n".as_ptr(), i);
            return -EINVAL;
        }

        ret = rb_find(rbtree, key, &mut value);
        if ret != 0 {
            return -1;
        }

        if value != 2 * key {
            return -1;
        }
        i += 1;
    }

    ret = remove_many_verify_all_present(rbtree);
    if ret != 0 {
        return ret;
    }

    i = 0;
    while (i as size_t) < numkeys && can_loop {
        let key: u64 = morekeys[i as usize];
        let nodec: node_ctx = first;

        if nodec.is_null() || key != (*nodec).rbnode.key {
            return -1;
        }

        first = if !(*nodec).next.is_null() {
            (*(*nodec).next).next
        } else {
            core::ptr::null_mut()
        };
        ret = rb_remove_node(rbtree, &mut (*nodec).rbnode);
        if ret != 0 {
            arena_stderr(c"Failed to remove %ld\n".as_ptr(), key);
            return -1;
        }

        ret = rb_find(rbtree, key, &mut value);
        if ret == 0 {
            return -1;
        }
        i += 2;
    }

    remove_many_verify_remaining(rbtree)
}

#[inline]
unsafe fn remove_many(alloc: rbtree_alloc, insert: rbtree_insert_mode) -> i32 {
    let mut ret: i32;
    let rbtree: *mut rbtree;

    rbtree = rb_create(alloc, insert);
    if rbtree.is_null() {
        return -ENOMEM;
    }

    ret = if alloc == rbtree_alloc::RB_ALLOC {
        remove_many_alloc(rbtree)
    } else {
        remove_many_noalloc(rbtree)
    };
    if ret != 0 {
        return ret;
    }

    if alloc == rbtree_alloc::RB_ALLOC {
        return rb_destroy(rbtree);
    }

    ret = clean_up_noalloc_tree(rbtree);
    if ret != 0 {
        return ret;
    }

    rb_destroy(rbtree)
}

#[no_mangle]
pub unsafe extern "C" fn test_rbtree_insert_many_update() -> i32 {
    insert_many(rbtree_alloc::RB_ALLOC, rbtree_insert_mode::RB_UPDATE)
}

#[no_mangle]
pub unsafe extern "C" fn test_rbtree_insert_many_noalloc() -> i32 {
    insert_many(rbtree_alloc::RB_NOALLOC, rbtree_insert_mode::RB_DUPLICATE)
}

#[no_mangle]
pub unsafe extern "C" fn test_rbtree_remove_many_update() -> i32 {
    remove_many(rbtree_alloc::RB_ALLOC, rbtree_insert_mode::RB_UPDATE)
}

#[no_mangle]
pub unsafe extern "C" fn test_rbtree_remove_many_noalloc() -> i32 {
    remove_many(rbtree_alloc::RB_NOALLOC, rbtree_insert_mode::RB_DUPLICATE)
}

#[no_mangle]
pub unsafe extern "C" fn test_rbtree_add_remove_circular() -> i32 {
    let iters: size_t = 60;
    let prefill: size_t = 10;
    let numkeys: size_t = 50;
    let prefix: size_t = 400000;
    let mut value: u64 = 0;
    let mut rmval: u64;
    let mut errval: i32 = 1;
    let mut key: u64;
    let mut ret: i32;
    let mut i: i32;
    let rbtree: *mut rbtree;

    rbtree = rb_create(rbtree_alloc::RB_ALLOC, rbtree_insert_mode::RB_UPDATE);
    if rbtree.is_null() {
        return 1;
    }

    i = 0;
    while (i as size_t) < prefill && can_loop {
        ret = rb_insert(rbtree, prefix as u64 + ((i as size_t) % numkeys) as u64, i as u64);
        if ret != 0 {
            return errval;
        }

        errval += 1;
        i += 1;
    }

    errval = 2 * 1000 * 1000;

    i = 0;
    while (i as size_t) < prefill && can_loop {
        /* Read it back. */
        ret = rb_find(rbtree, prefix as u64 + ((i as size_t) % numkeys) as u64, &mut value);
        if ret != 0 {
            return errval;
        }

        if value != i as u64 {
            return errval;
        }
        i += 1;
    }

    errval = 3 * 1000 * 1000;

    i = prefill as i32;
    while (i as size_t) < iters && can_loop {
        key = prefix as u64 + ((i as size_t) % numkeys) as u64;

        ret = rb_find(rbtree, key, &mut value);
        if ret == 0 {
            arena_stderr(c"Key %d already present\n".as_ptr(), key);
            return errval;
        }

        errval += 1;

        ret = rb_insert(rbtree, key, i as u64);
        if ret != 0 {
            arena_stderr(c"ITERATION %d\n".as_ptr(), i);
            rb_print(rbtree);
            return errval;
        }

        rmval = (i as size_t - prefill) as u64;

        errval += 1;

        ret = rb_find(rbtree, prefix as u64 + (rmval % numkeys as u64), &mut value);
        if ret != 0 {
            return errval;
        }

        errval += 1;

        if value != rmval {
            return errval;
        }

        errval += 1;

        ret = rb_remove(rbtree, prefix as u64 + (rmval % numkeys as u64));
        if ret != 0 {
            arena_stderr(c"ITERATION %d\n".as_ptr(), i);
            return errval;
        }

        errval += 1;
        i += 1;
    }

    i = 0;
    while (i as size_t) < numkeys && can_loop {
        rb_remove(rbtree, prefix as u64 + i as u64);
        i += 1;
    }

    rb_destroy(rbtree)
}

#[no_mangle]
pub unsafe extern "C" fn test_rbtree_add_remove_circular_reverse() -> i32 {
    let iters: size_t = 110;
    let prefill: size_t = 10;
    let numkeys: size_t = 50;
    let prefix: size_t = 500000;
    let mut value: u64 = 0;
    let mut rmval: u64;
    let mut errval: i32 = 1;
    let mut key: u64;
    let mut ret: i32;
    let mut i: i32;
    let rbtree: *mut rbtree;

    rbtree = rb_create(rbtree_alloc::RB_ALLOC, rbtree_insert_mode::RB_UPDATE);
    if rbtree.is_null() {
        return 1;
    }

    i = 0;
    while (i as size_t) < prefill && can_loop {
        ret = rb_insert(rbtree, prefix as u64 - ((i as size_t) % numkeys) as u64, i as u64);
        if ret != 0 {
            return errval;
        }

        errval += 1;
        i += 1;
    }

    errval = 2 * 1000 * 1000;

    i = 0;
    while (i as size_t) < prefill && can_loop {
        /* Read it back. */
        ret = rb_find(rbtree, prefix as u64 - ((i as size_t) % numkeys) as u64, &mut value);
        if ret != 0 {
            return errval;
        }

        if value != i as u64 {
            return errval;
        }
        i += 1;
    }

    errval = 3 * 1000 * 1000;

    i = prefill as i32;
    while (i as size_t) < iters && can_loop {
        key = prefix as u64 - ((i as size_t) % numkeys) as u64;

        ret = rb_find(rbtree, key, &mut value);
        if ret == 0 {
            arena_stderr(c"Key %d already present\n".as_ptr(), key);
            return errval;
        }

        errval += 1;

        ret = rb_insert(rbtree, key, i as u64);
        if ret != 0 {
            arena_stderr(c"error %d on insert\n".as_ptr(), ret);
            rb_print(rbtree);
            return errval;
        }

        rmval = (i as size_t - prefill) as u64;

        errval += 1;

        ret = rb_find(rbtree, prefix as u64 - (rmval % numkeys as u64), &mut value);
        if ret != 0 {
            return errval;
        }

        errval += 1;

        if value != rmval {
            return errval;
        }

        errval += 1;

        ret = rb_remove(rbtree, prefix as u64 - (rmval % numkeys as u64));
        if ret != 0 {
            return errval;
        }

        errval += 1;
        i += 1;
    }

    errval = 4 * 1000 * 1000;
    i = 0;
    while (i as size_t) < prefill && can_loop {
        ret = rb_remove(rbtree, prefix as u64 - i as u64);
        if ret != 0 {
            arena_stderr(c"Did not remove %d, error %d\n".as_ptr(), prefix as u64 - i as u64, ret);
            return errval + i;
        }
        i += 1;
    }

    rb_destroy(rbtree)
}

#[no_mangle]
pub unsafe extern "C" fn test_rbtree_least_pop() -> i32 {
    let keys_count: size_t = 10;
    let mut key: u64 = 0;
    let mut value: u64 = 0;
    let mut errval: i32 = 1;
    let mut ret: i32;
    let mut i: i32;
    let rbtree: *mut rbtree;

    rbtree = rb_create(rbtree_alloc::RB_ALLOC, rbtree_insert_mode::RB_DEFAULT);
    if rbtree.is_null() {
        return errval;
    }

    errval += 1;

    i = 0;
    while (i as size_t) < keys_count / 2 && can_loop {
        ret = rb_insert(rbtree, i as u64, i as u64);
        if ret != 0 {
            return errval;
        }

        errval += 1;

        ret = rb_insert(
            rbtree,
            (keys_count - 1 - i as size_t) as u64,
            (keys_count - 1 - i as size_t) as u64,
        );
        if ret != 0 {
            return errval;
        }

        errval += 1;

        ret = rb_least(rbtree, &mut key, &mut value);
        if ret != 0 {
            return errval;
        }

        errval += 1;

        if key != 0 || value != 0 {
            return errval;
        }

        errval += 1;
        i += 1;
    }

    errval = 1000;

    i = 0;
    while (i as size_t) < keys_count && can_loop {
        ret = rb_least(rbtree, &mut key, &mut value);
        if ret != 0 {
            arena_stderr(c"rb_least failed with %d\n".as_ptr(), ret);
            return errval;
        }

        errval += 1;

        if key != i as u64 || value != i as u64 {
            arena_stderr(c"Got KV %ld/%ld expected %d\n".as_ptr(), key, value, i);
            return errval;
        }

        errval += 1;

        ret = rb_pop(rbtree, &mut key, &mut value);
        if ret != 0 {
            arena_stderr(c"Error %d during pop on iter %d\n".as_ptr(), ret, i);
            return errval;
        }

        errval += 1;

        if key != i as u64 || value != i as u64 {
            return errval;
        }
        i += 1;
    }

    rb_destroy(rbtree)
}

/* Reject rb_pop() for RB_NOALLOC trees. */
#[no_mangle]
pub unsafe extern "C" fn test_rbtree_noalloc_pop() -> i32 {
    let expect_value: u64 = 1;
    let expect_key: u64 = 0;
    let rbtree: *mut rbtree;
    let node: *mut rbnode;
    let mut value: u64 = 0;
    let mut ret: i32;

    rbtree = rb_create(rbtree_alloc::RB_NOALLOC, rbtree_insert_mode::RB_DEFAULT);
    if rbtree.is_null() {
        return 1;
    }

    node = rb_node_alloc(expect_key, expect_value);
    if node.is_null() {
        rb_destroy(rbtree);
        return 2;
    }

    ret = rb_insert_node(rbtree, node);
    if ret != 0 {
        rb_node_free(node);
        rb_destroy(rbtree);
        return 3;
    }

    ret = rb_pop(rbtree, core::ptr::null_mut(), &mut value);
    if ret != -EINVAL {
        return 4;
    }

    ret = rb_find(rbtree, expect_key, &mut value);
    if ret != 0 {
        return 5;
    }

    if value != expect_value {
        return 6;
    }

    ret = rb_remove_node(rbtree, node);
    if ret != 0 {
        return 7;
    }

    rb_node_free(node);

    rb_destroy(rbtree)
}

#[no_mangle]
pub unsafe extern "C" fn test_rbtree_alloc_check() -> i32 {
    let alloc: *mut rbtree;
    let noalloc: *mut rbtree;
    let node: *mut rbnode;
    let mut ret: i32;

    alloc = rb_create(rbtree_alloc::RB_ALLOC, rbtree_insert_mode::RB_DEFAULT);
    if alloc.is_null() {
        return 1;
    }

    noalloc = rb_create(rbtree_alloc::RB_NOALLOC, rbtree_insert_mode::RB_DEFAULT);
    if noalloc.is_null() {
        return 2;
    }

    node = rb_node_alloc(0, 0);
    if node.is_null() {
        return 3;
    }

    /*
     * RB_ALLOC trees can use rb_insert, RB_NOALLOC trees can
     * use rb_insert_node. RB_ALLOC and RB_NOALLOC trees cannot
     * use each other's APIs.
     *
     * NOTE: This begs the question, why not different types? We
     * want to partially share the API and that would require us
     * to duplicate it.
     */
    if rb_insert(alloc, 0, 0) != 0 {
        return 4;
    }

    if rb_insert_node(alloc, node) == 0 {
        return 5;
    }

    if rb_remove_node(alloc, node) == 0 {
        return 6;
    }

    if rb_remove(alloc, 0) != 0 {
        return 7;
    }

    if rb_insert_node(noalloc, node) != 0 {
        return 8;
    }

    if rb_insert(noalloc, 0, 0) == 0 {
        return 9;
    }

    if rb_remove(noalloc, 0) == 0 {
        return 10;
    }

    if rb_remove_node(noalloc, node) != 0 {
        return 11;
    }

    rb_node_free(node);

    ret = rb_destroy(alloc);
    if ret != 0 {
        return ret;
    }

    rb_destroy(noalloc)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
