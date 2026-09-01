// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)

// Translated from perf/util/bpf-utils.c.
// C dependencies from <errno.h>, <stdlib.h>, <linux/err.h>,
// <linux/kernel.h>, <bpf/bpf.h>, "bpf-utils.h", and "debug.h" are expected
// to be supplied by surrounding bindings.

#[repr(C)]
struct bpil_array_desc {
    array_offset: i32, /* e.g. offset of jited_prog_insns */
    count_offset: i32, /* e.g. offset of jited_prog_len */
    size_offset: i32, /* > 0: offset of rec size,
                       * < 0: fix size of -size_offset
                       */
}

const BPIL_ARRAY_DESC_LEN: usize = PERF_BPIL_LAST_ARRAY as usize;

static BPIL_ARRAY_DESC: [bpil_array_desc; BPIL_ARRAY_DESC_LEN] = {
    let mut desc = [bpil_array_desc {
        array_offset: 0,
        count_offset: 0,
        size_offset: 0,
    }; BPIL_ARRAY_DESC_LEN];

    desc[PERF_BPIL_JITED_INSNS as usize] = bpil_array_desc {
        array_offset: offset_of!(bpf_prog_info, jited_prog_insns) as i32,
        count_offset: offset_of!(bpf_prog_info, jited_prog_len) as i32,
        size_offset: -1,
    };
    desc[PERF_BPIL_XLATED_INSNS as usize] = bpil_array_desc {
        array_offset: offset_of!(bpf_prog_info, xlated_prog_insns) as i32,
        count_offset: offset_of!(bpf_prog_info, xlated_prog_len) as i32,
        size_offset: -1,
    };
    desc[PERF_BPIL_MAP_IDS as usize] = bpil_array_desc {
        array_offset: offset_of!(bpf_prog_info, map_ids) as i32,
        count_offset: offset_of!(bpf_prog_info, nr_map_ids) as i32,
        size_offset: -(core::mem::size_of::<__u32>() as i32),
    };
    desc[PERF_BPIL_JITED_KSYMS as usize] = bpil_array_desc {
        array_offset: offset_of!(bpf_prog_info, jited_ksyms) as i32,
        count_offset: offset_of!(bpf_prog_info, nr_jited_ksyms) as i32,
        size_offset: -(core::mem::size_of::<__u64>() as i32),
    };
    desc[PERF_BPIL_JITED_FUNC_LENS as usize] = bpil_array_desc {
        array_offset: offset_of!(bpf_prog_info, jited_func_lens) as i32,
        count_offset: offset_of!(bpf_prog_info, nr_jited_func_lens) as i32,
        size_offset: -(core::mem::size_of::<__u32>() as i32),
    };
    desc[PERF_BPIL_FUNC_INFO as usize] = bpil_array_desc {
        array_offset: offset_of!(bpf_prog_info, func_info) as i32,
        count_offset: offset_of!(bpf_prog_info, nr_func_info) as i32,
        size_offset: offset_of!(bpf_prog_info, func_info_rec_size) as i32,
    };
    desc[PERF_BPIL_LINE_INFO as usize] = bpil_array_desc {
        array_offset: offset_of!(bpf_prog_info, line_info) as i32,
        count_offset: offset_of!(bpf_prog_info, nr_line_info) as i32,
        size_offset: offset_of!(bpf_prog_info, line_info_rec_size) as i32,
    };
    desc[PERF_BPIL_JITED_LINE_INFO as usize] = bpil_array_desc {
        array_offset: offset_of!(bpf_prog_info, jited_line_info) as i32,
        count_offset: offset_of!(bpf_prog_info, nr_jited_line_info) as i32,
        size_offset: offset_of!(bpf_prog_info, jited_line_info_rec_size) as i32,
    };
    desc[PERF_BPIL_PROG_TAGS as usize] = bpil_array_desc {
        array_offset: offset_of!(bpf_prog_info, prog_tags) as i32,
        count_offset: offset_of!(bpf_prog_info, nr_prog_tags) as i32,
        size_offset: -((core::mem::size_of::<__u8>() * BPF_TAG_SIZE as usize) as i32),
    };

    desc
};

unsafe fn bpf_prog_info_read_offset_u32(info: *mut bpf_prog_info, offset: i32) -> __u32 {
    let array = info as *mut __u32;

    if offset >= 0 {
        *array.add((offset as usize) / core::mem::size_of::<__u32>())
    } else {
        (-(offset as i32)) as __u32
    }
}

unsafe fn bpf_prog_info_read_offset_u64(info: *mut bpf_prog_info, offset: i32) -> __u64 {
    let array = info as *mut __u64;

    if offset >= 0 {
        *array.add((offset as usize) / core::mem::size_of::<__u64>())
    } else {
        (-(offset as i32)) as __u64
    }
}

unsafe fn bpf_prog_info_set_offset_u32(info: *mut bpf_prog_info, offset: i32, val: __u32) {
    let array = info as *mut __u32;

    if offset >= 0 {
        *array.add((offset as usize) / core::mem::size_of::<__u32>()) = val;
    }
}

unsafe fn bpf_prog_info_set_offset_u64(info: *mut bpf_prog_info, offset: i32, val: __u64) {
    let array = info as *mut __u64;

    if offset >= 0 {
        *array.add((offset as usize) / core::mem::size_of::<__u64>()) = val;
    }
}

pub unsafe extern "C" fn get_bpf_prog_info_linear(fd: i32, mut arrays: __u64) -> *mut perf_bpil {
    let mut info: bpf_prog_info = core::mem::zeroed();
    let mut info_linear: *mut perf_bpil;
    let mut info_len: __u32 = core::mem::size_of_val(&info) as __u32;
    let mut data_len: __u32 = 0;
    let mut err: i32;
    let mut ptr: *mut __u8;

    if (arrays >> PERF_BPIL_LAST_ARRAY) != 0 {
        return ERR_PTR(-EINVAL) as *mut perf_bpil;
    }

    /* step 1: get array dimensions */
    err = bpf_obj_get_info_by_fd(fd, &mut info as *mut _ as *mut _, &mut info_len);
    if err != 0 {
        pr_debug!("can't get prog info: %m\n");
        return ERR_PTR(-EFAULT) as *mut perf_bpil;
    }
    if info.type_ >= __MAX_BPF_PROG_TYPE {
        pr_debug!(
            "%s:%d: unexpected program type %u\n",
            cstr!("get_bpf_prog_info_linear"),
            line!(),
            info.type_
        );
    }

    /* step 2: calculate total size of all arrays */
    let mut i = PERF_BPIL_FIRST_ARRAY;
    while i < PERF_BPIL_LAST_ARRAY {
        let desc = &BPIL_ARRAY_DESC[i as usize] as *const bpil_array_desc;
        let mut include_array = (arrays & (1u64 << i)) > 0;
        let count: __u32;
        let size: __u32;

        /* kernel is too old to support this field */
        if info_len < ((*desc).array_offset as __u32).wrapping_add(core::mem::size_of::<__u32>() as __u32)
            || info_len < ((*desc).count_offset as __u32).wrapping_add(core::mem::size_of::<__u32>() as __u32)
            || ((*desc).size_offset > 0 && info_len < (*desc).size_offset as __u32)
        {
            include_array = false;
        }

        if !include_array {
            arrays &= !(1u64 << i); /* clear the bit */
            i += 1;
            continue;
        }

        count = bpf_prog_info_read_offset_u32(&mut info, (*desc).count_offset);
        size = bpf_prog_info_read_offset_u32(&mut info, (*desc).size_offset);

        data_len = data_len.wrapping_add(roundup(
            count.wrapping_mul(size),
            core::mem::size_of::<__u64>() as __u32,
        ));
        i += 1;
    }

    /* step 3: allocate continuous memory */
    info_linear = malloc(
        core::mem::size_of::<perf_bpil>().wrapping_add(data_len as usize),
    ) as *mut perf_bpil;
    if info_linear.is_null() {
        return ERR_PTR(-ENOMEM) as *mut perf_bpil;
    }

    /* step 4: fill data to info_linear->info */
    (*info_linear).arrays = arrays;
    memset(
        &mut (*info_linear).info as *mut _ as *mut _,
        0,
        core::mem::size_of_val(&info),
    );
    ptr = core::ptr::addr_of_mut!((*info_linear).data) as *mut __u8;

    i = PERF_BPIL_FIRST_ARRAY;
    while i < PERF_BPIL_LAST_ARRAY {
        let desc = &BPIL_ARRAY_DESC[i as usize] as *const bpil_array_desc;
        let count: __u32;
        let size: __u32;

        if (arrays & (1u64 << i)) == 0 {
            i += 1;
            continue;
        }

        count = bpf_prog_info_read_offset_u32(&mut info, (*desc).count_offset);
        size = bpf_prog_info_read_offset_u32(&mut info, (*desc).size_offset);
        bpf_prog_info_set_offset_u32(&mut (*info_linear).info, (*desc).count_offset, count);
        bpf_prog_info_set_offset_u32(&mut (*info_linear).info, (*desc).size_offset, size);
        assert!(ptr >= core::ptr::addr_of_mut!((*info_linear).data) as *mut __u8);
        assert!(ptr < (core::ptr::addr_of_mut!((*info_linear).data) as *mut __u8).add(data_len as usize));
        bpf_prog_info_set_offset_u64(
            &mut (*info_linear).info,
            (*desc).array_offset,
            ptr_to_u64(ptr as *const _) as __u64,
        );
        ptr = ptr.add(roundup(
            count.wrapping_mul(size),
            core::mem::size_of::<__u64>() as __u32,
        ) as usize);
        i += 1;
    }

    /* step 5: call syscall again to get required arrays */
    err = bpf_obj_get_info_by_fd(fd, &mut (*info_linear).info as *mut _ as *mut _, &mut info_len);
    if err != 0 {
        pr_debug!("can't get prog info: %m\n");
        free(info_linear as *mut _);
        return ERR_PTR(-EFAULT) as *mut perf_bpil;
    }
    if (*info_linear).info.type_ >= __MAX_BPF_PROG_TYPE {
        pr_debug!(
            "%s:%d: unexpected program type %u\n",
            cstr!("get_bpf_prog_info_linear"),
            line!(),
            (*info_linear).info.type_
        );
    }

    /* step 6: verify the data */
    ptr = core::ptr::addr_of_mut!((*info_linear).data) as *mut __u8;
    i = PERF_BPIL_FIRST_ARRAY;
    while i < PERF_BPIL_LAST_ARRAY {
        let desc = &BPIL_ARRAY_DESC[i as usize] as *const bpil_array_desc;
        let count1: __u32;
        let count2: __u32;
        let size1: __u32;
        let size2: __u32;
        let ptr2: __u64;

        if (arrays & (1u64 << i)) == 0 {
            i += 1;
            continue;
        }

        count1 = bpf_prog_info_read_offset_u32(&mut info, (*desc).count_offset);
        count2 = bpf_prog_info_read_offset_u32(&mut (*info_linear).info, (*desc).count_offset);
        if count1 != count2 {
            pr_warning!(
                "%s: mismatch in element count %u vs %u\n",
                cstr!("get_bpf_prog_info_linear"),
                count1,
                count2
            );
            free(info_linear as *mut _);
            return ERR_PTR(-ERANGE) as *mut perf_bpil;
        }

        size1 = bpf_prog_info_read_offset_u32(&mut info, (*desc).size_offset);
        size2 = bpf_prog_info_read_offset_u32(&mut (*info_linear).info, (*desc).size_offset);
        if size1 != size2 {
            pr_warning!(
                "%s: mismatch in rec size %u vs %u\n",
                cstr!("get_bpf_prog_info_linear"),
                size1,
                size2
            );
            free(info_linear as *mut _);
            return ERR_PTR(-ERANGE) as *mut perf_bpil;
        }
        ptr2 = bpf_prog_info_read_offset_u64(&mut (*info_linear).info, (*desc).array_offset);
        if ptr_to_u64(ptr as *const _) as __u64 != ptr2 {
            pr_warning!(
                "%s: mismatch in array %p vs %llx\n",
                cstr!("get_bpf_prog_info_linear"),
                ptr,
                ptr2
            );
            free(info_linear as *mut _);
            return ERR_PTR(-ERANGE) as *mut perf_bpil;
        }
        ptr = ptr.add(roundup(
            count1.wrapping_mul(size1),
            core::mem::size_of::<__u64>() as __u32,
        ) as usize);
        i += 1;
    }

    /* step 7: update info_len and data_len */
    (*info_linear).info_len = core::mem::size_of::<bpf_prog_info>() as __u32;
    (*info_linear).data_len = data_len;

    info_linear
}

pub unsafe extern "C" fn bpil_addr_to_offs(info_linear: *mut perf_bpil) {
    let mut i: i32;

    i = PERF_BPIL_FIRST_ARRAY;
    while i < PERF_BPIL_LAST_ARRAY {
        let desc = &BPIL_ARRAY_DESC[i as usize] as *const bpil_array_desc;
        let addr: __u64;
        let offs: __u64;

        if ((*info_linear).arrays & (1u64 << i)) == 0 {
            i += 1;
            continue;
        }

        addr = bpf_prog_info_read_offset_u64(&mut (*info_linear).info, (*desc).array_offset);
        offs = addr.wrapping_sub(ptr_to_u64(core::ptr::addr_of_mut!((*info_linear).data) as *const _) as __u64);
        bpf_prog_info_set_offset_u64(&mut (*info_linear).info, (*desc).array_offset, offs);
        i += 1;
    }
}

pub unsafe extern "C" fn bpil_offs_to_addr(info_linear: *mut perf_bpil) {
    let mut i: i32;

    i = PERF_BPIL_FIRST_ARRAY;
    while i < PERF_BPIL_LAST_ARRAY {
        let desc = &BPIL_ARRAY_DESC[i as usize] as *const bpil_array_desc;
        let addr: __u64;
        let offs: __u64;
        let count: __u32;
        let size: __u32;

        if ((*info_linear).arrays & (1u64 << i)) == 0 {
            i += 1;
            continue;
        }

        offs = bpf_prog_info_read_offset_u64(&mut (*info_linear).info, (*desc).array_offset);
        count = bpf_prog_info_read_offset_u32(&mut (*info_linear).info, (*desc).count_offset);
        size = bpf_prog_info_read_offset_u32(&mut (*info_linear).info, (*desc).size_offset);
        /* offset and extent from perf.data are untrusted - keep within data[] */
        if offs >= (*info_linear).data_len as __u64
            || (count as u64).wrapping_mul(size as u64)
                > ((*info_linear).data_len as u64).wrapping_sub(offs as u64)
        {
            bpf_prog_info_set_offset_u64(&mut (*info_linear).info, (*desc).array_offset, 0);
            bpf_prog_info_set_offset_u32(&mut (*info_linear).info, (*desc).count_offset, 0);
            /* clear the bit so bpil_addr_to_offs() won't reverse a zeroed address */
            (*info_linear).arrays &= !(1u64 << i);
            i += 1;
            continue;
        }
        addr = offs.wrapping_add(ptr_to_u64(core::ptr::addr_of_mut!((*info_linear).data) as *const _) as __u64);
        bpf_prog_info_set_offset_u64(&mut (*info_linear).info, (*desc).array_offset, addr);
        i += 1;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
