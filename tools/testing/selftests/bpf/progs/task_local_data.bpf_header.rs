/* SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause) */

/*
 * Task local data is a library that facilitates sharing per-task data
 * between user space and bpf programs.
 *
 *
 * USAGE
 *
 * A TLD, an entry of data in task local data, first needs to be created by the
 * user space. This is done by calling user space API, TLD_DEFINE_KEY() or
 * tld_create_key(), with the name of the TLD and the size.
 *
 * TLD_DEFINE_KEY(prio, "priority", sizeof(int));
 *
 * or
 *
 * void func_call(...) {
 *     tld_key_t prio, in_cs;
 *
 *     prio = tld_create_key("priority", sizeof(int));
 *     in_cs = tld_create_key("in_critical_section", sizeof(bool));
 *     ...
 *
 * A key associated with the TLD, which has an opaque type tld_key_t, will be
 * initialized or returned. It can be used to get a pointer to the TLD in the
 * user space by calling tld_get_data().
 *
 * In a bpf program, tld_object_init() first needs to be called to initialized a
 * tld_object on the stack. Then, TLDs can be accessed by calling tld_get_data().
 * The API will try to fetch the key by the name and use it to locate the data.
 * A pointer to the TLD will be returned. It also caches the key in a task local
 * storage map, tld_key_map, whose value type, struct tld_keys, must be defined
 * by the developer.
 *
 * struct tld_keys {
 *     tld_key_t prio;
 *     tld_key_t in_cs;
 * };
 *
 * SEC("struct_ops")
 * void prog(struct task_struct task, ...)
 * {
 *     struct tld_object tld_obj;
 *     int err, *p;
 *
 *     err = tld_object_init(task, &tld_obj);
 *     if (err)
 *         return;
 *
 *     p = tld_get_data(&tld_obj, prio, "priority", sizeof(int));
 *     if (p)
 *         // do something depending on *p
 */

pub const TLD_NAME_LEN: usize = 62;
pub const TLD_KEY_MAP_CREATE_RETRY: i32 = 10;

pub const fn TLD_ROUND_MASK(x: usize, y: usize) -> usize {
    let _ = x;
    y - 1
}

pub const fn TLD_ROUND_UP(x: usize, y: usize) -> usize {
    ((x - 1) | TLD_ROUND_MASK(x, y)) + 1
}

pub const TLD_MAX_DATA_CNT: usize =
    (__PAGE_SIZE as usize / core::mem::size_of::<tld_metadata>()) - 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tld_key_t {
    pub off: __s16,
}

#[repr(C)]
pub struct tld_metadata {
    pub name: [core::ffi::c_char; TLD_NAME_LEN],
    pub size: __u16,
}

#[repr(C)]
pub struct tld_meta_u {
    pub cnt: __u16,
    pub size: __u16,
    pub metadata: [tld_metadata; TLD_MAX_DATA_CNT],
}

#[repr(C, align(8))]
pub struct tld_data_aligned_data {
    pub data: [core::ffi::c_char; __PAGE_SIZE as usize - core::mem::size_of::<__u64>()],
}

#[repr(C)]
pub struct tld_data_u {
    pub unused: __u64,
    pub data: tld_data_aligned_data,
}

#[repr(C)]
pub struct tld_map_value {
    /* __uptr user pointer annotation from C is preserved as raw pointers here. */
    pub data: *mut tld_data_u,
    pub meta: *mut tld_meta_u,
    pub start: __u16, /* offset of tld_data_u->data in a page */
}

#[repr(C)]
pub struct tld_uptr_dummy {
    pub data: [tld_data_u; 0],
    pub meta: [tld_meta_u; 0],
}

pub type tld_uptr_dummy_t = *mut tld_uptr_dummy;

#[repr(C)]
pub struct tld_object {
    pub data_map: *mut tld_map_value,
    pub key_map: *mut tld_keys,
    /*
     * Force the compiler to generate the actual definition of tld_meta_u
     * and tld_data_u in BTF. Without it, tld_meta_u and u_tld_data will
     * be BTF_KIND_FWD.
     */
    pub dummy: [tld_uptr_dummy_t; 0],
}

/*
 * Map value of tld_key_map for caching keys. Must be defined by the developer.
 * Members should be tld_key_t and passed to the 3rd argument of tld_fetch_key().
 */
pub enum tld_keys {}

/* Original C defines BPF task-storage maps with SEC(".maps"):
 *
 * struct {
 *     __uint(type, BPF_MAP_TYPE_TASK_STORAGE);
 *     __uint(map_flags, BPF_F_NO_PREALLOC);
 *     __type(key, int);
 *     __type(value, struct tld_map_value);
 * } tld_data_map SEC(".maps");
 *
 * struct {
 *     __uint(type, BPF_MAP_TYPE_TASK_STORAGE);
 *     __uint(map_flags, BPF_F_NO_PREALLOC);
 *     __type(key, int);
 *     __type(value, struct tld_keys);
 * } tld_key_map SEC(".maps");
 */
extern "C" {
    pub static mut tld_data_map: core::ffi::c_void;
    pub static mut tld_key_map: core::ffi::c_void;

    pub fn bpf_task_storage_get(
        map: *mut core::ffi::c_void,
        task: *mut task_struct,
        value: *mut core::ffi::c_void,
        flags: __u64,
    ) -> *mut core::ffi::c_void;

    pub fn bpf_strncmp(
        s1: *const core::ffi::c_char,
        s1_sz: __u32,
        s2: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
}

/**
 * tld_object_init() - Initialize a tld_object.
 *
 * @task: The task_struct of the target task
 * @tld_obj: A pointer to a tld_object to be initialized
 *
 * Return 0 on success; -ENODATA if the user space did not initialize task local data
 * for the current task through tld_get_data(); -ENOMEM if the creation of tld_key_map
 * fails
 */
pub unsafe fn tld_object_init(task: *mut task_struct, tld_obj: *mut tld_object) -> core::ffi::c_int {
    let mut i: core::ffi::c_int;

    (*tld_obj).data_map =
        bpf_task_storage_get(&mut tld_data_map, task, core::ptr::null_mut(), 0) as *mut tld_map_value;
    if (*tld_obj).data_map.is_null() {
        return -ENODATA;
    }

    i = 0;
    while i < TLD_KEY_MAP_CREATE_RETRY {
        (*tld_obj).key_map = bpf_task_storage_get(
            &mut tld_key_map,
            task,
            core::ptr::null_mut(),
            BPF_LOCAL_STORAGE_GET_F_CREATE as __u64,
        ) as *mut tld_keys;
        if likely(!(*tld_obj).key_map.is_null()) {
            break;
        }
        i += 1;
    }
    if (*tld_obj).key_map.is_null() {
        return -ENOMEM;
    }

    0
}

/*
 * Return the offset of TLD if @name is found. Otherwise, return the current TLD count
 * using the nonpositive range so that the next tld_get_data() can skip fetching key if
 * no new TLD is added or start comparing name from the first newly added TLD.
 */
pub unsafe fn __tld_fetch_key(
    tld_obj: *mut tld_object,
    name: *const core::ffi::c_char,
    i_start: core::ffi::c_int,
) -> core::ffi::c_int {
    let metadata: *mut tld_metadata;
    let mut i: core::ffi::c_int;
    let cnt: core::ffi::c_int;
    let start: core::ffi::c_int;
    let mut off: core::ffi::c_int = 0;

    if (*tld_obj).data_map.is_null()
        || (*(*tld_obj).data_map).data.is_null()
        || (*(*tld_obj).data_map).meta.is_null()
    {
        return 0;
    }

    start = (*(*tld_obj).data_map).start as core::ffi::c_int;
    cnt = (*(*(*tld_obj).data_map).meta).cnt as core::ffi::c_int;
    metadata = (*(*(*tld_obj).data_map).meta).metadata.as_mut_ptr();

    i = 0;
    while i < cnt {
        if i >= TLD_MAX_DATA_CNT as core::ffi::c_int {
            break;
        }

        if i >= i_start
            && bpf_strncmp(
                (*metadata.add(i as usize)).name.as_ptr(),
                TLD_NAME_LEN as __u32,
                name,
            ) == 0
        {
            return start + off;
        }

        off += TLD_ROUND_UP((*metadata.add(i as usize)).size as usize, 8) as core::ffi::c_int;
        i += 1;
    }

    -cnt
}

/**
 * tld_get_data() - Retrieve a pointer to the TLD associated with the name.
 *
 * @tld_obj: A pointer to a valid tld_object initialized by tld_object_init()
 * @key: The cached key of the TLD in tld_key_map
 * @name: The name of the key associated with a TLD
 * @size: The size of the TLD. Must be a known constant value
 *
 * Return a pointer to the TLD associated with @name; NULL if not found or @size is too
 * big. @key is used to cache the key if the TLD is found to speed up subsequent calls.
 * It should be defined as an member of tld_keys of tld_key_t type by the developer.
 */
#[macro_export]
macro_rules! tld_get_data {
    ($tld_obj:expr, $key:ident, $name:expr, $size:expr) => {{
        let mut data: *mut core::ffi::c_void = core::ptr::null_mut();
        let _data: *mut core::ffi::c_void = (*(*$tld_obj).data_map).data as *mut core::ffi::c_void;
        let mut off: core::ffi::c_long = (*(*$tld_obj).key_map).$key.off as core::ffi::c_long;
        let cnt: core::ffi::c_int;

        if likely(!_data.is_null()) {
            if likely(off > 0) {
                barrier_var!(off);
                if likely(off < (__PAGE_SIZE as core::ffi::c_long - $size as core::ffi::c_long)) {
                    data = (_data as *mut u8).add(off as usize) as *mut core::ffi::c_void;
                }
            } else {
                cnt = -off as core::ffi::c_int;
                if likely(!(*(*$tld_obj).data_map).meta.is_null())
                    && cnt < (*(*(*$tld_obj).data_map).meta).cnt as core::ffi::c_int
                {
                    off = __tld_fetch_key($tld_obj, $name, cnt) as core::ffi::c_long;
                    (*(*$tld_obj).key_map).$key.off = off as __s16;

                    if likely(off < (__PAGE_SIZE as core::ffi::c_long - $size as core::ffi::c_long)) {
                        barrier_var!(off);
                        if off > 0 {
                            data = (_data as *mut u8).add(off as usize) as *mut core::ffi::c_void;
                        }
                    }
                }
            }
        }
        data
    }};
}
