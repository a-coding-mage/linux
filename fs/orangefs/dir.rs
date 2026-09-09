// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2017 Omnibond Systems, L.L.C.
 */

#[repr(C)]
struct orangefs_dir_part {
    next: *mut orangefs_dir_part,
    len: usize,
}

#[repr(C)]
struct orangefs_dir {
    token: u64,
    part: *mut orangefs_dir_part,
    end: i64,
    error: i32,
}

const PART_SHIFT: u32 = 24;
const PART_SIZE: usize = 1 << 24;
const PART_MASK: i64 = !(PART_SIZE as i64 - 1);

/*
 * Directory data is received in linked-list parts. The position encodes the
 * part number in bits above PART_SHIFT and the offset below it. Part zero is
 * synthesized for `.' and `..'.
 */

unsafe fn do_readdir(
    od: *mut orangefs_dir,
    inode: *mut inode,
    op: *mut orangefs_kernel_op_s,
) -> i32 {
    let oi = ORANGEFS_I(inode);
    (*op).uses_shared_memory = 1;
    (*op).upcall.req.readdir.refn = (*oi).refn;
    (*op).upcall.req.readdir.token = (*od).token;
    (*op).upcall.req.readdir.max_dirent_count = ORANGEFS_MAX_DIRENT_COUNT_READDIR;

    loop {
        let bufi = orangefs_readdir_index_get();
        if bufi < 0 { (*od).error = bufi; return bufi; }
        (*op).upcall.req.readdir.buf_index = bufi;
        let r = service_operation(op, "orangefs_readdir", get_interruptible_flag(inode));
        orangefs_readdir_index_put(bufi);
        if op_state_purged(op) {
            if r == -EAGAIN { vfree((*op).downcall.trailer_buf); continue; }
            if r == -EIO { vfree((*op).downcall.trailer_buf); (*od).error = r; return r; }
        }
        if r < 0 { vfree((*op).downcall.trailer_buf); (*od).error = r; return r; }
        if (*op).downcall.status != 0 {
            vfree((*op).downcall.trailer_buf);
            (*od).error = (*op).downcall.status;
            return (*op).downcall.status;
        }
        if (*op).downcall.trailer_size > PART_SIZE {
            vfree((*op).downcall.trailer_buf); (*od).error = -EIO; return -EIO;
        }
        let resp = (*op).downcall.trailer_buf as *mut orangefs_readdir_response_s;
        (*od).token = (*resp).token;
        return 0;
    }
}

unsafe fn parse_readdir(od: *mut orangefs_dir, op: *mut orangefs_kernel_op_s) -> i32 {
    let mut part = (*od).part;
    let mut count: usize = 1;
    while !part.is_null() { count += 1; if (*part).next.is_null() { break; } part = (*part).next; }
    let new = (*op).downcall.trailer_buf as *mut orangefs_dir_part;
    (*new).next = core::ptr::null_mut();
    (*new).len = (*op).downcall.trailer_size - core::mem::size_of::<orangefs_readdir_response_s>();
    if (*od).part.is_null() { (*od).part = new; } else { (*part).next = new; }
    count += 1;
    (*od).end = (count as i64) << PART_SHIFT;
    0
}

unsafe fn orangefs_dir_more(od: *mut orangefs_dir, inode: *mut inode) -> i32 {
    let op = op_alloc(ORANGEFS_VFS_OP_READDIR);
    if op.is_null() { (*od).error = -ENOMEM; return -ENOMEM; }
    let mut r = do_readdir(od, inode, op);
    if r == 0 { r = parse_readdir(od, op); }
    if r != 0 { (*od).error = r; }
    op_release(op);
    (*od).error
}

unsafe fn fill_from_part(part: *mut orangefs_dir_part, ctx: *mut dir_context) -> i32 {
    let offset = core::mem::size_of::<orangefs_readdir_response_s>();
    let mut i = (*ctx).pos & !PART_MASK;
    if i > (*part).len as i64 { return 1; }
    if i % 8 != 0 { i += (8 - i % 8) % 8; }
    while i < (*part).len as i64 {
        let base = part.cast::<u8>().add(offset + i as usize);
        if (*part).len < i as usize + core::mem::size_of::<u32>() { break; }
        let len = *(base as *const u32) as usize;
        let mut padlen = core::mem::size_of::<u32>() as u64 + len as u64 + 1;
        padlen += (8 - padlen % 8) % 8;
        if (*part).len < i as usize + padlen as usize + core::mem::size_of::<orangefs_khandle>() { i += 8; continue; }
        let s = base.add(core::mem::size_of::<u32>());
        if *s.add(len) != 0 { i += 8; continue; }
        let khandle = s.add(len + 1 + (padlen as usize - core::mem::size_of::<u32>() - len - 1)) as *mut orangefs_khandle;
        if !dir_emit(ctx, s as *const i8, len, orangefs_khandle_to_ino(khandle), DT_UNKNOWN) { return 0; }
        i += padlen as i64 + core::mem::size_of::<orangefs_khandle>() as i64;
        i += (8 - i % 8) % 8;
        BUG_ON(i as usize > (*part).len);
        (*ctx).pos = ((*ctx).pos & PART_MASK) | i;
    }
    1
}

unsafe fn orangefs_dir_fill(od: *mut orangefs_dir, ctx: *mut dir_context) -> i32 {
    let mut count = (((*ctx).pos & PART_MASK) >> PART_SHIFT) - 1;
    let mut part = (*od).part;
    while !part.is_null() && count != 0 { count -= 1; part = (*part).next; }
    if count != 0 { (*od).error = -EIO; return -EIO; }
    while !part.is_null() && (*part).len != 0 {
        let r = fill_from_part(part, ctx);
        if r == 0 { break; }
        (*ctx).pos = ((*ctx).pos & PART_MASK) + (1 << PART_SHIFT);
        part = (*part).next;
    }
    0
}

unsafe fn orangefs_dir_llseek(file: *mut file, offset: i64, whence: i32) -> i64 {
    let od = (*file).private_data as *mut orangefs_dir;
    if whence == 0 && offset < (*od).end {
        let mut part = (*od).part;
        while !part.is_null() { let next = (*part).next; vfree(part as *mut _); part = next; }
        (*od).token = ORANGEFS_ITERATE_START; (*od).part = core::ptr::null_mut(); (*od).end = 1 << PART_SHIFT;
    }
    default_llseek(file, offset, whence)
}

unsafe fn orangefs_dir_iterate(file: *mut file, ctx: *mut dir_context) -> i32 {
    let od = (*file).private_data as *mut orangefs_dir;
    let inode = file_inode(file);
    if (*od).error != 0 { return (*od).error; }
    if (*ctx).pos == 0 { if !dir_emit_dot(file, ctx) { return 0; } (*ctx).pos += 1; }
    if (*ctx).pos == 1 { if !dir_emit_dotdot(file, ctx) { return 0; } (*ctx).pos = 1 << PART_SHIFT; }
    if ((*ctx).pos & PART_MASK) == 0 { return -EIO; }
    while (*od).token != ORANGEFS_ITERATE_END && (*ctx).pos > (*od).end { let r = orangefs_dir_more(od, inode); if r != 0 { return r; } }
    if (*od).token == ORANGEFS_ITERATE_END && (*ctx).pos > (*od).end { return -EIO; }
    if (*ctx).pos < (*od).end { let r = orangefs_dir_fill(od, ctx); if r != 0 { return r; } }
    if (*od).token != ORANGEFS_ITERATE_END { let r = orangefs_dir_more(od, inode); if r != 0 { return r; } return orangefs_dir_fill(od, ctx); }
    0
}

unsafe fn orangefs_dir_open(_inode: *mut inode, file: *mut file) -> i32 {
    let od = kmalloc_obj::<orangefs_dir>();
    if od.is_null() { return -ENOMEM; }
    (*file).private_data = od as *mut _;
    (*od).token = ORANGEFS_ITERATE_START; (*od).part = core::ptr::null_mut(); (*od).end = 1 << PART_SHIFT; (*od).error = 0; 0
}

unsafe fn orangefs_dir_release(_inode: *mut inode, file: *mut file) -> i32 {
    let od = (*file).private_data as *mut orangefs_dir;
    let mut part = (*od).part;
    while !part.is_null() { let next = (*part).next; vfree(part as *mut _); part = next; }
    kfree(od as *mut _); 0
}

const orangefs_dir_operations: file_operations = file_operations {
    llseek: Some(orangefs_dir_llseek), read: Some(generic_read_dir),
    iterate_shared: Some(orangefs_dir_iterate), open: Some(orangefs_dir_open),
    release: Some(orangefs_dir_release), setlease: Some(generic_setlease),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
