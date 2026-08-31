// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Joe Lawrence <joe.lawrence@redhat.com>

// C dependency intent:
// #define pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// #include <linux/module.h>
// #include <linux/kernel.h>
// #include <linux/list.h>
// #include <linux/livepatch.h>
// #include <linux/slab.h>

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::MaybeUninit;
use core::ptr;

type size_t = usize;
type gfp_t = u32;
type klp_shadow_ctor_t = Option<
    unsafe extern "C" fn(obj: *mut c_void, shadow_data: *mut c_void, ctor_data: *mut c_void) -> c_int,
>;
type klp_shadow_dtor_t =
    Option<unsafe extern "C" fn(obj: *mut c_void, shadow_data: *mut c_void)>;

const GFP_ATOMIC: gfp_t = 0;
const GFP_KERNEL: gfp_t = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;

#[repr(C)]
struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

extern "C" {
    fn kmalloc(size: size_t, flags: gfp_t) -> *mut c_void;
    fn kfree(ptr: *mut c_void);

    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);

    fn klp_shadow_get(obj: *mut c_void, id: c_ulong) -> *mut c_void;
    fn klp_shadow_alloc(
        obj: *mut c_void,
        id: c_ulong,
        size: size_t,
        gfp_flags: gfp_t,
        ctor: klp_shadow_ctor_t,
        ctor_data: *mut c_void,
    ) -> *mut c_void;
    fn klp_shadow_get_or_alloc(
        obj: *mut c_void,
        id: c_ulong,
        size: size_t,
        gfp_flags: gfp_t,
        ctor: klp_shadow_ctor_t,
        ctor_data: *mut c_void,
    ) -> *mut c_void;
    fn klp_shadow_free(obj: *mut c_void, id: c_ulong, dtor: klp_shadow_dtor_t);
    fn klp_shadow_free_all(id: c_ulong, dtor: klp_shadow_dtor_t);

    fn printk(fmt: *const c_char, ...);
}

macro_rules! pr_info {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {{
        unsafe {
            printk(concat!($fmt, "\0").as_ptr() as *const c_char $(, $arg)*);
        }
    }};
}

/*
 * Keep a small list of pointers so that we can print address-agnostic
 * pointer values.  Use a rolling integer count to differentiate the values.
 * Ironically we could have used the shadow variable API to do this, but
 * let's not lean too heavily on the very code we're testing.
 */
static mut PTR_LIST: list_head = list_head {
    next: ptr::null_mut(),
    prev: ptr::null_mut(),
};

#[repr(C)]
struct shadow_ptr {
    ptr: *mut c_void,
    id: c_int,
    list: list_head,
}

unsafe fn init_ptr_list_once() {
    if PTR_LIST.next.is_null() {
        let head = ptr::addr_of_mut!(PTR_LIST);
        PTR_LIST.next = head;
        PTR_LIST.prev = head;
    }
}

unsafe fn shadow_ptr_from_list(list: *mut list_head) -> *mut shadow_ptr {
    (list as *mut u8).sub(core::mem::offset_of!(shadow_ptr, list)) as *mut shadow_ptr
}

unsafe fn free_ptr_list() {
    let mut pos: *mut list_head;
    let mut next: *mut list_head;

    init_ptr_list_once();
    pos = PTR_LIST.next;
    while pos != ptr::addr_of_mut!(PTR_LIST) {
        let sp = shadow_ptr_from_list(pos);
        next = (*pos).next;
        list_del(ptr::addr_of_mut!((*sp).list));
        kfree(sp as *mut c_void);
        pos = next;
    }
}

unsafe fn ptr_id(ptr: *mut c_void) -> c_int {
    static mut COUNT: c_int = 0;
    let mut pos: *mut list_head;
    let sp: *mut shadow_ptr;

    init_ptr_list_once();
    pos = PTR_LIST.next;
    while pos != ptr::addr_of_mut!(PTR_LIST) {
        let cur = shadow_ptr_from_list(pos);
        if (*cur).ptr == ptr {
            return (*cur).id;
        }
        pos = (*pos).next;
    }

    sp = kmalloc(core::mem::size_of::<shadow_ptr>(), GFP_ATOMIC) as *mut shadow_ptr;
    if sp.is_null() {
        return -ENOMEM;
    }
    (*sp).ptr = ptr;
    (*sp).id = COUNT;
    COUNT = COUNT.wrapping_add(1);

    list_add(ptr::addr_of_mut!((*sp).list), ptr::addr_of_mut!(PTR_LIST));

    (*sp).id
}

/*
 * Shadow variable wrapper functions that echo the function and arguments
 * to the kernel log for testing verification.  Don't display raw pointers,
 * but use the ptr_id() value instead.
 */
unsafe fn shadow_get(obj: *mut c_void, id: c_ulong) -> *mut c_void {
    let sv: *mut *mut c_int;

    sv = klp_shadow_get(obj, id) as *mut *mut c_int;
    pr_info!(
        "klp_%s(obj=PTR%d, id=0x%lx) = PTR%d\n",
        b"shadow_get\0".as_ptr() as *const c_char,
        ptr_id(obj),
        id,
        ptr_id(sv as *mut c_void)
    );

    sv as *mut c_void
}

unsafe fn shadow_alloc(
    obj: *mut c_void,
    id: c_ulong,
    size: size_t,
    gfp_flags: gfp_t,
    ctor: klp_shadow_ctor_t,
    ctor_data: *mut c_void,
) -> *mut c_void {
    let var = ctor_data as *mut *mut c_int;
    let sv: *mut *mut c_int;

    sv = klp_shadow_alloc(obj, id, size, gfp_flags, ctor, var as *mut c_void) as *mut *mut c_int;
    pr_info!(
        "klp_%s(obj=PTR%d, id=0x%lx, size=%zx, gfp_flags=%pGg), ctor=PTR%d, ctor_data=PTR%d = PTR%d\n",
        b"shadow_alloc\0".as_ptr() as *const c_char,
        ptr_id(obj),
        id,
        size,
        ptr::addr_of!(gfp_flags),
        ptr_id(ctor.map_or(ptr::null_mut(), |f| f as *mut c_void)),
        ptr_id(*var as *mut c_void),
        ptr_id(sv as *mut c_void)
    );

    sv as *mut c_void
}

unsafe fn shadow_get_or_alloc(
    obj: *mut c_void,
    id: c_ulong,
    size: size_t,
    gfp_flags: gfp_t,
    ctor: klp_shadow_ctor_t,
    ctor_data: *mut c_void,
) -> *mut c_void {
    let var = ctor_data as *mut *mut c_int;
    let sv: *mut *mut c_int;

    sv = klp_shadow_get_or_alloc(obj, id, size, gfp_flags, ctor, var as *mut c_void)
        as *mut *mut c_int;
    pr_info!(
        "klp_%s(obj=PTR%d, id=0x%lx, size=%zx, gfp_flags=%pGg), ctor=PTR%d, ctor_data=PTR%d = PTR%d\n",
        b"shadow_get_or_alloc\0".as_ptr() as *const c_char,
        ptr_id(obj),
        id,
        size,
        ptr::addr_of!(gfp_flags),
        ptr_id(ctor.map_or(ptr::null_mut(), |f| f as *mut c_void)),
        ptr_id(*var as *mut c_void),
        ptr_id(sv as *mut c_void)
    );

    sv as *mut c_void
}

unsafe fn shadow_free(obj: *mut c_void, id: c_ulong, dtor: klp_shadow_dtor_t) {
    klp_shadow_free(obj, id, dtor);
    pr_info!(
        "klp_%s(obj=PTR%d, id=0x%lx, dtor=PTR%d)\n",
        b"shadow_free\0".as_ptr() as *const c_char,
        ptr_id(obj),
        id,
        ptr_id(dtor.map_or(ptr::null_mut(), |f| f as *mut c_void))
    );
}

unsafe fn shadow_free_all(id: c_ulong, dtor: klp_shadow_dtor_t) {
    klp_shadow_free_all(id, dtor);
    pr_info!(
        "klp_%s(id=0x%lx, dtor=PTR%d)\n",
        b"shadow_free_all\0".as_ptr() as *const c_char,
        id,
        ptr_id(dtor.map_or(ptr::null_mut(), |f| f as *mut c_void))
    );
}

/* Shadow variable constructor - remember simple pointer data */
unsafe extern "C" fn shadow_ctor(
    _obj: *mut c_void,
    shadow_data: *mut c_void,
    ctor_data: *mut c_void,
) -> c_int {
    let sv = shadow_data as *mut *mut c_int;
    let var = ctor_data as *mut *mut c_int;

    if var.is_null() {
        return -EINVAL;
    }

    *sv = *var;
    pr_info!(
        "%s: PTR%d -> PTR%d\n",
        b"shadow_ctor\0".as_ptr() as *const c_char,
        ptr_id(sv as *mut c_void),
        ptr_id(*var as *mut c_void)
    );

    0
}

/*
 * With more than one item to free in the list, order is not determined and
 * shadow_dtor will not be passed to shadow_free_all() which would make the
 * test fail. (see pass 6)
 */
unsafe extern "C" fn shadow_dtor(obj: *mut c_void, shadow_data: *mut c_void) {
    let sv = shadow_data as *mut *mut c_int;

    pr_info!(
        "%s(obj=PTR%d, shadow_data=PTR%d)\n",
        b"shadow_dtor\0".as_ptr() as *const c_char,
        ptr_id(obj),
        ptr_id(sv as *mut c_void)
    );
}

/* number of objects we simulate that need shadow vars */
const NUM_OBJS: usize = 3;

/* dynamically created obj fields have the following shadow var id values */
const SV_ID1: c_ulong = 0x1234;
const SV_ID2: c_ulong = 0x1235;

/*
 * The main test case adds/removes new fields (shadow var) to each of these
 * test structure instances. The last group of fields in the struct represent
 * the idea that shadow variables may be added and removed to and from the
 * struct during execution.
 */
#[repr(C)]
struct test_object {
    /* add anything here below and avoid to define an empty struct */
    sp: shadow_ptr,

    /* these represent shadow vars added and removed with SV_ID{1,2} */
    /* char nfield1; */
    /* int  nfield2; */
}

unsafe extern "C" fn test_klp_shadow_vars_init() -> c_int {
    let mut objs: [MaybeUninit<test_object>; NUM_OBJS] = MaybeUninit::uninit().assume_init();
    let mut nfields1: [c_char; NUM_OBJS] = [0; NUM_OBJS];
    let mut pnfields1: [*mut c_char; NUM_OBJS] = [ptr::null_mut(); NUM_OBJS];
    let mut sv1: [*mut *mut c_char; NUM_OBJS] = [ptr::null_mut(); NUM_OBJS];
    let mut pndup: [*mut c_char; NUM_OBJS] = [ptr::null_mut(); NUM_OBJS];
    let mut nfields2: [c_int; NUM_OBJS] = [0; NUM_OBJS];
    let mut pnfields2: [*mut c_int; NUM_OBJS] = [ptr::null_mut(); NUM_OBJS];
    let mut sv2: [*mut *mut c_int; NUM_OBJS] = [ptr::null_mut(); NUM_OBJS];
    let mut sv: *mut *mut c_void;
    let ret: c_int;
    let mut i: usize;

    ptr_id(ptr::null_mut());

    /*
     * With an empty shadow variable hash table, expect not to find
     * any matches.
     */
    sv = shadow_get(objs[0].as_mut_ptr() as *mut c_void, SV_ID1) as *mut *mut c_void;
    if sv.is_null() {
        pr_info!("  got expected NULL result\n");
    }

    /* pass 1: init & alloc a char+int pair of svars for each objs */
    i = 0;
    while i < NUM_OBJS {
        pnfields1[i] = ptr::addr_of_mut!(nfields1[i]);
        ptr_id(pnfields1[i] as *mut c_void);

        if i % 2 != 0 {
            sv1[i] = shadow_alloc(
                objs[i].as_mut_ptr() as *mut c_void,
                SV_ID1,
                core::mem::size_of_val(&pnfields1[i]),
                GFP_KERNEL,
                Some(shadow_ctor),
                ptr::addr_of_mut!(pnfields1[i]) as *mut c_void,
            ) as *mut *mut c_char;
        } else {
            sv1[i] = shadow_get_or_alloc(
                objs[i].as_mut_ptr() as *mut c_void,
                SV_ID1,
                core::mem::size_of_val(&pnfields1[i]),
                GFP_KERNEL,
                Some(shadow_ctor),
                ptr::addr_of_mut!(pnfields1[i]) as *mut c_void,
            ) as *mut *mut c_char;
        }
        if sv1[i].is_null() {
            ret = -ENOMEM;
            goto_out(ret);
            return ret;
        }

        pnfields2[i] = ptr::addr_of_mut!(nfields2[i]);
        ptr_id(pnfields2[i] as *mut c_void);
        sv2[i] = shadow_alloc(
            objs[i].as_mut_ptr() as *mut c_void,
            SV_ID2,
            core::mem::size_of_val(&pnfields2[i]),
            GFP_KERNEL,
            Some(shadow_ctor),
            ptr::addr_of_mut!(pnfields2[i]) as *mut c_void,
        ) as *mut *mut c_int;
        if sv2[i].is_null() {
            ret = -ENOMEM;
            goto_out(ret);
            return ret;
        }

        i += 1;
    }

    /* pass 2: verify we find allocated svars and where they point to */
    i = 0;
    while i < NUM_OBJS {
        /* check the "char" svar for all objects */
        sv = shadow_get(objs[i].as_mut_ptr() as *mut c_void, SV_ID1) as *mut *mut c_void;
        if sv.is_null() {
            ret = -EINVAL;
            goto_out(ret);
            return ret;
        }
        if (sv as *mut *mut c_char) == sv1[i] && *sv1[i] == pnfields1[i] {
            pr_info!(
                "  got expected PTR%d -> PTR%d result\n",
                ptr_id(sv1[i] as *mut c_void),
                ptr_id(*sv1[i] as *mut c_void)
            );
        }

        /* check the "int" svar for all objects */
        sv = shadow_get(objs[i].as_mut_ptr() as *mut c_void, SV_ID2) as *mut *mut c_void;
        if sv.is_null() {
            ret = -EINVAL;
            goto_out(ret);
            return ret;
        }
        if (sv as *mut *mut c_int) == sv2[i] && *sv2[i] == pnfields2[i] {
            pr_info!(
                "  got expected PTR%d -> PTR%d result\n",
                ptr_id(sv2[i] as *mut c_void),
                ptr_id(*sv2[i] as *mut c_void)
            );
        }

        i += 1;
    }

    /* pass 3: verify that 'get_or_alloc' returns already allocated svars */
    i = 0;
    while i < NUM_OBJS {
        pndup[i] = ptr::addr_of_mut!(nfields1[i]);
        ptr_id(pndup[i] as *mut c_void);

        sv = shadow_get_or_alloc(
            objs[i].as_mut_ptr() as *mut c_void,
            SV_ID1,
            core::mem::size_of_val(&pndup[i]),
            GFP_KERNEL,
            Some(shadow_ctor),
            ptr::addr_of_mut!(pndup[i]) as *mut c_void,
        ) as *mut *mut c_void;
        if sv.is_null() {
            ret = -EINVAL;
            goto_out(ret);
            return ret;
        }
        if (sv as *mut *mut c_char) == sv1[i] && *sv1[i] == pnfields1[i] {
            pr_info!(
                "  got expected PTR%d -> PTR%d result\n",
                ptr_id(sv1[i] as *mut c_void),
                ptr_id(*sv1[i] as *mut c_void)
            );
        }

        i += 1;
    }

    /* pass 4: free <objs[*], SV_ID1> pairs of svars, verify removal */
    i = 0;
    while i < NUM_OBJS {
        shadow_free(
            objs[i].as_mut_ptr() as *mut c_void,
            SV_ID1,
            Some(shadow_dtor),
        ); /* 'char' pairs */
        sv = shadow_get(objs[i].as_mut_ptr() as *mut c_void, SV_ID1) as *mut *mut c_void;
        if sv.is_null() {
            pr_info!("  got expected NULL result\n");
        }

        i += 1;
    }

    /* pass 5: check we still find <objs[*], SV_ID2> svar pairs */
    i = 0;
    while i < NUM_OBJS {
        sv = shadow_get(objs[i].as_mut_ptr() as *mut c_void, SV_ID2) as *mut *mut c_void; /* 'int' pairs */
        if sv.is_null() {
            ret = -EINVAL;
            goto_out(ret);
            return ret;
        }
        if (sv as *mut *mut c_int) == sv2[i] && *sv2[i] == pnfields2[i] {
            pr_info!(
                "  got expected PTR%d -> PTR%d result\n",
                ptr_id(sv2[i] as *mut c_void),
                ptr_id(*sv2[i] as *mut c_void)
            );
        }

        i += 1;
    }

    /* pass 6: free all the <objs[*], SV_ID2> svar pairs too. */
    shadow_free_all(SV_ID2, None); /* 'int' pairs */
    i = 0;
    while i < NUM_OBJS {
        sv = shadow_get(objs[i].as_mut_ptr() as *mut c_void, SV_ID2) as *mut *mut c_void;
        if sv.is_null() {
            pr_info!("  got expected NULL result\n");
        }

        i += 1;
    }

    free_ptr_list();

    return 0;

    unsafe fn goto_out(ret: c_int) {
        let _ = ret;
        shadow_free_all(SV_ID1, None); /* 'char' pairs */
        shadow_free_all(SV_ID2, None); /* 'int' pairs */
        free_ptr_list();
    }
}

unsafe extern "C" fn test_klp_shadow_vars_exit() {}

// module_init(test_klp_shadow_vars_init);
// module_exit(test_klp_shadow_vars_exit);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Joe Lawrence <joe.lawrence@redhat.com>");
// MODULE_DESCRIPTION("Livepatch test: shadow variables");
