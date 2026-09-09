// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * SPU core dump code
 *
 * (C) Copyright 2006 IBM Corp.
 *
 * Author: Dwayne Grant McConnell <decimal@us.ibm.com>
 */

// Dependencies supplied by the surrounding kernel and spufs sources are
// intentionally referenced here rather than reimplemented.

unsafe fn spufs_ctx_note_size(ctx: *mut spu_context, dfd: i32) -> i32 {
    let mut i: usize = 0;
    let mut total: i32 = 0;
    let mut fullname = [0i8; 80];

    while !spufs_coredump_read[i].name.is_null() {
        let name = spufs_coredump_read[i].name;
        let sz = spufs_coredump_read[i].size;

        sprintf(fullname.as_mut_ptr(), b"SPU/%d/%s\0".as_ptr() as *const i8, dfd, name);

        total += core::mem::size_of::<elf_note>() as i32;
        total += roundup(strlen(fullname.as_ptr()) + 1, 4) as i32;
        total += roundup(sz, 4) as i32;
        i += 1;
    }

    total
}

unsafe extern "C" fn match_context(
    _v: *const core::ffi::c_void,
    file: *mut file,
    fd: u32,
) -> i32 {
    if (*file).f_op != &spufs_context_fops {
        return 0;
    }
    let ctx = (*SPUFS_I(file_inode(file))).i_ctx;
    if (*ctx).flags & SPU_CREATE_NOSCHED != 0 {
        return 0;
    }
    (fd + 1) as i32
}

/*
 * The additional architecture-specific notes for Cell are various
 * context files in the spu context.
 *
 * This function iterates over all open file descriptors and sees
 * if they are a directory in spufs.  In that case we use spufs
 * internal functionality to dump them without needing to actually
 * open the files.
 */
/*
 * descriptor table is not shared, so files can't change or go away.
 */
unsafe fn coredump_next_context(fd: *mut i32) -> *mut spu_context {
    let mut ctx: *mut spu_context = core::ptr::null_mut();
    let mut file: *mut file;
    let n = iterate_fd((*current).files, *fd, Some(match_context), core::ptr::null_mut());
    if n == 0 {
        return core::ptr::null_mut();
    }
    *fd = n - 1;

    file = fget_raw(*fd);
    if !file.is_null() {
        ctx = (*SPUFS_I(file_inode(file))).i_ctx;
        get_spu_context(ctx);
        fput(file);
    }

    ctx
}

pub unsafe fn spufs_coredump_extra_notes_size() -> i32 {
    let mut size: i32 = 0;
    let mut fd: i32 = 0;
    loop {
        let ctx = coredump_next_context(&mut fd);
        if ctx.is_null() {
            break;
        }
        let rc = spu_acquire_saved(ctx);
        if rc != 0 {
            put_spu_context(ctx);
            break;
        }

        let rc = spufs_ctx_note_size(ctx, fd);
        spu_release_saved(ctx);
        if rc < 0 {
            put_spu_context(ctx);
            break;
        }

        size += rc;

        /* start searching the next fd next time */
        fd += 1;
        put_spu_context(ctx);
    }

    size
}

unsafe fn spufs_arch_write_note(
    ctx: *mut spu_context,
    i: usize,
    cprm: *mut coredump_params,
    dfd: i32,
) -> i32 {
    let sz = spufs_coredump_read[i].size;
    let mut fullname = [0i8; 80];
    let mut en: elf_note = core::mem::zeroed();

    sprintf(
        fullname.as_mut_ptr(),
        b"SPU/%d/%s\0".as_ptr() as *const i8,
        dfd,
        spufs_coredump_read[i].name,
    );
    en.n_namesz = strlen(fullname.as_ptr()) + 1;
    en.n_descsz = sz;
    en.n_type = NT_SPU;

    if !dump_emit(cprm, &en as *const _ as *const core::ffi::c_void, core::mem::size_of::<elf_note>()) {
        return -EIO;
    }
    if !dump_emit(cprm, fullname.as_ptr() as *const core::ffi::c_void, en.n_namesz as usize) {
        return -EIO;
    }
    if !dump_align(cprm, 4) {
        return -EIO;
    }

    let mut ret: i32;
    if let Some(dump) = spufs_coredump_read[i].dump {
        ret = dump(ctx, cprm);
        if ret < 0 {
            return ret;
        }
    } else {
        let mut buf = [0i8; 32];
        ret = snprintf(
            buf.as_mut_ptr(),
            buf.len(),
            b"0x%.16llx\0".as_ptr() as *const i8,
            (spufs_coredump_read[i].get)(ctx),
        );
        if ret >= buf.len() as i32 {
            return buf.len() as i32;
        }

        /* count trailing the NULL: */
        if !dump_emit(cprm, buf.as_ptr() as *const core::ffi::c_void, (ret + 1) as usize) {
            return -EIO;
        }
    }

    dump_skip_to(cprm, roundup((*cprm).pos - ret as usize + sz, 4));
    0
}

pub unsafe fn spufs_coredump_extra_notes_write(cprm: *mut coredump_params) -> i32 {
    let mut fd: i32 = 0;
    loop {
        let ctx = coredump_next_context(&mut fd);
        if ctx.is_null() {
            break;
        }
        let rc = spu_acquire_saved(ctx);
        if rc != 0 {
            return rc;
        }

        let mut j: usize = 0;
        while !spufs_coredump_read[j].name.is_null() {
            let rc = spufs_arch_write_note(ctx, j, cprm, fd);
            if rc != 0 {
                spu_release_saved(ctx);
                return rc;
            }
            j += 1;
        }

        spu_release_saved(ctx);

        /* start searching the next fd next time */
        fd += 1;
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
