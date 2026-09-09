// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2026 Christoph Hellwig.
 */

#[repr(C)]
struct BlkErrorInject {
    entry: ListHead,
    start: SectorT,
    end: SectorT,
    op: ReqOp,
    status: BlkStatusT,
    // only inject every 1 / chance times
    chance: c_uint,
}

// DEFINE_STATIC_KEY_FALSE(blk_error_injection_enabled);
static mut BLK_ERROR_INJECTION_ENABLED: StaticKeyFalse = StaticKeyFalse::new();

unsafe fn __blk_error_inject(bio: *mut Bio) -> bool {
    let disk = (*(*bio).bi_bdev).bd_disk;
    let mut inj: *mut BlkErrorInject;

    rcu_read_lock();
    list_for_each_entry_rcu!(inj, (*disk).error_injection_list, entry);
    while !inj.is_null() {
        if bio_op(bio) != (*inj).op {
            inj = list_next_entry_rcu!(inj, entry);
            continue;
        }
        /*
         * This never matches 0-sized bios like empty WRITEs with
         * REQ_PREFLUSH or ZONE_RESET_ALL.  While adding a special case
         * for them would be trivial, that means any WRITE rule would
         * trigger for flushes.  So before we can make this work
         * properly, we'll need to start using REQ_OP_FLUSH for pure
         * flushes at the bio level like we already do in blk-mq.
         */
        if (*bio).bi_iter.bi_sector > (*inj).end
            || bio_end_sector(bio) <= (*inj).start
        {
            inj = list_next_entry_rcu!(inj, entry);
            continue;
        }
        if (*inj).chance > 1 && get_random_u32() % (*inj).chance != 0 {
            inj = list_next_entry_rcu!(inj, entry);
            continue;
        }

        pr_info_ratelimited!(
            "%pg: injecting %s error for %s at sector %llu:%u\n",
            (*disk).part0,
            blk_status_to_str((*inj).status),
            blk_op_str((*inj).op),
            (*bio).bi_iter.bi_sector,
            bio_sectors(bio),
        );
        (*bio).bi_status = (*inj).status;
        rcu_read_unlock();
        bio_endio(bio);
        return true;
    }
    rcu_read_unlock();
    false
}

unsafe fn error_inject_add(
    disk: *mut Gendisk,
    op: ReqOp,
    start: SectorT,
    nr_sectors: u64,
    status: BlkStatusT,
    chance: c_uint,
) -> c_int {
    let mut error = -EINVAL;

    if op == REQ_OP_LAST || status == BLK_STS_OK {
        return -EINVAL;
    }
    let inj = kzalloc_obj::<BlkErrorInject>();
    if inj.is_null() {
        return -ENOMEM;
    }

    if nr_sectors != 0 {
        if u64::MAX - nr_sectors < start {
            kfree(inj);
            return error;
        }
        (*inj).end = start + nr_sectors - 1;
    } else {
        (*inj).end = u64::MAX;
    }

    (*inj).op = op;
    (*inj).start = start;
    (*inj).status = status;
    (*inj).chance = chance;

    pr_debug_ratelimited!(
        "%pg: adding %s injection for %s at sector %llu:%llu\n",
        (*disk).part0,
        blk_status_to_str(status),
        blk_op_str(op),
        start,
        nr_sectors,
    );

    // Add to the front of the list so newer entries can partially override others.
    // Duplicate entries are intentionally allowed.
    mutex_lock(&mut (*disk).error_injection_lock);
    if !disk_live(disk) {
        mutex_unlock(&mut (*disk).error_injection_lock);
        error = -ENODEV;
        kfree(inj);
        return error;
    }
    if list_empty(&(*disk).error_injection_list) {
        static_branch_inc(unsafe { &mut BLK_ERROR_INJECTION_ENABLED });
    }
    list_add_rcu(&mut (*inj).entry, &mut (*disk).error_injection_list);
    set_bit(GD_ERROR_INJECT, &mut (*disk).state);
    mutex_unlock(&mut (*disk).error_injection_lock);
    0
}

unsafe fn error_inject_removeall(disk: *mut Gendisk) {
    mutex_lock(&mut (*disk).error_injection_lock);
    if test_and_clear_bit(GD_ERROR_INJECT, &mut (*disk).state) {
        static_branch_dec(unsafe { &mut BLK_ERROR_INJECTION_ENABLED });
    }
    while let Some(inj) = list_first_entry_or_null::<BlkErrorInject>(
        &(*disk).error_injection_list,
    ) {
        list_del_rcu(&mut (*inj).entry);
        kfree_rcu_mightsleep(inj);
    }
    mutex_unlock(&mut (*disk).error_injection_lock);
}

#[repr(u32)]
enum Options {
    OptAdd = 1u32 << 0,
    OptRemoveall = 1u32 << 1,
    OptOp = 1u32 << 16,
    OptStart = 1u32 << 17,
    OptNrSectors = 1u32 << 18,
    OptStatus = 1u32 << 19,
    OptChance = 1u32 << 20,
    OptInvalid,
}

// static const match_table_t opt_tokens = { ... };
// The kernel parser table is supplied by the external dependency set.

unsafe fn match_op(args: *mut SubstringT, op: *mut ReqOp) -> c_int {
    let tag = match_strdup(args);
    if tag.is_null() {
        return -ENOMEM;
    }
    *op = str_to_blk_op(tag);
    if *op == REQ_OP_LAST {
        pr_warn!("invalid op '%s'\n", tag);
    }
    kfree(tag);
    0
}

unsafe fn match_status(args: *mut SubstringT, status: *mut BlkStatusT) -> c_int {
    let tag = match_strdup(args);
    if tag.is_null() {
        return -ENOMEM;
    }
    *status = tag_to_blk_status(tag);
    if *status == 0 {
        pr_warn!("invalid status '%s'\n", tag);
    }
    kfree(tag);
    0
}

unsafe fn blk_error_injection_parse_options(disk: *mut Gendisk, mut options: *mut c_char) -> isize {
    let mut action = 0; // Unset, Add, Removeall
    let mut option_mask: u32 = 0;
    let mut chance: c_uint = 1;
    let mut op = REQ_OP_LAST;
    let mut start: u64 = 0;
    let mut nr_sectors: u64 = 0;
    let mut status = BLK_STS_OK;
    let mut args: [SubstringT; MAX_OPT_ARGS] = core::mem::zeroed();

    while let Some(p) = strsep(&mut options, b",\n\0".as_ptr() as *const c_char) {
        let mut error = 0;
        let token = match_token(p, &OPT_TOKENS, args.as_mut_ptr());
        option_mask |= token as u32;
        match token {
            x if x == OptAdd as isize => { if action != 0 { return -EINVAL as isize; } action = 1; }
            x if x == OptRemoveall as isize => { if action != 0 { return -EINVAL as isize; } action = 2; }
            x if x == OptOp as isize => error = match_op(args.as_mut_ptr(), &mut op),
            x if x == OptStart as isize => error = match_u64(args.as_mut_ptr(), &mut start),
            x if x == OptNrSectors as isize => error = match_u64(args.as_mut_ptr(), &mut nr_sectors),
            x if x == OptStatus as isize => error = match_status(args.as_mut_ptr(), &mut status),
            x if x == OptChance as isize => { error = match_uint(args.as_mut_ptr(), &mut chance); if error == 0 && chance == 0 { error = -EINVAL; } }
            _ => { pr_warn!("unknown parameter or missing value '%s'\n", p); error = -EINVAL; }
        }
        if error != 0 { return error as isize; }
    }
    match action {
        1 => error_inject_add(disk, op, start, nr_sectors, status, chance) as isize,
        2 => { if option_mask & !(OptRemoveall as u32) != 0 { return -EINVAL as isize; } error_inject_removeall(disk); 0 }
        _ => -EINVAL as isize,
    }
}

unsafe fn blk_error_injection_write(file: *mut File, ubuf: *const c_char, count: usize, _pos: *mut LoFFt) -> isize {
    let disk = (*file_inode(file)).i_private as *mut Gendisk;
    let options = memdup_user_nul(ubuf, count);
    if is_err(options) { return ptr_err(options); }
    let error = blk_error_injection_parse_options(disk, options);
    kfree(options);
    if error != 0 { error } else { count as isize }
}

unsafe fn blk_error_injection_show(s: *mut SeqFile, _private: *mut c_void) -> c_int {
    let disk = (*s).private as *mut Gendisk;
    let mut inj: *mut BlkErrorInject;
    rcu_read_lock();
    list_for_each_entry_rcu!(inj, (*disk).error_injection_list, entry);
    while !inj.is_null() {
        seq_printf!(s, "%llu:%llu op=%s,status=%s,chance=%u", (*inj).start, (*inj).end, blk_op_str((*inj).op), blk_status_to_tag((*inj).status), (*inj).chance);
        seq_putc(s, b'\n' as c_int);
        inj = list_next_entry_rcu!(inj, entry);
    }
    rcu_read_unlock();
    0
}

unsafe fn blk_error_injection_open(inode: *mut Inode, file: *mut File) -> c_int {
    single_open(file, blk_error_injection_show, (*inode).i_private)
}

unsafe fn blk_error_injection_release(inode: *mut Inode, file: *mut File) -> c_int {
    single_release(inode, file)
}

// static const struct file_operations blk_error_injection_fops = { ... };

unsafe fn blk_error_injection_init(disk: *mut Gendisk) {
    debugfs_create_file!("error_injection", 0o600, (*(*disk).queue).debugfs_dir, disk, &BLK_ERROR_INJECTION_FOPS);
}

unsafe fn blk_error_injection_exit(disk: *mut Gendisk) {
    error_inject_removeall(disk);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
