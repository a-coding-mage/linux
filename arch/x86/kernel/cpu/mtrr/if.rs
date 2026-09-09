// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel translation.

const LINE_SIZE: usize = 80;

const mtrr_strings: [&'static [u8]; MTRR_NUM_TYPES] = [
    b"uncachable\0",       // 0
    b"write-combining\0",  // 1
    b"?\0",                // 2
    b"?\0",                // 3
    b"write-through\0",    // 4
    b"write-protect\0",    // 5
    b"write-back\0",        // 6
];

pub unsafe fn mtrr_attrib_to_str(x: i32) -> *const u8 {
    if x <= 6 { mtrr_strings[x as usize].as_ptr() } else { b"?\0".as_ptr() }
}

// CONFIG_PROC_FS
pub unsafe fn mtrr_file_add(base: c_ulong, size: c_ulong, ty: c_uint,
                             _increment: bool, file: *mut file, page: i32) -> i32 {
    let mut fcount = FILE_FCOUNT(file);
    let max = num_var_ranges;
    if fcount.is_null() {
        fcount = kcalloc(max, core::mem::size_of::<c_uint>(), GFP_KERNEL);
        if fcount.is_null() { return -ENOMEM; }
        FILE_FCOUNT_SET(file, fcount);
    }
    let mut base = base;
    let mut size = size;
    if page == 0 {
        if (base & (PAGE_SIZE - 1)) != 0 || (size & (PAGE_SIZE - 1)) != 0 { return -EINVAL; }
        base >>= PAGE_SHIFT;
        size >>= PAGE_SHIFT;
    }
    let reg = mtrr_add_page(base, size, ty, true);
    if reg >= 0 { *fcount.add(reg as usize) += 1; }
    reg
}

pub unsafe fn mtrr_file_del(base: c_ulong, size: c_ulong, file: *mut file, page: i32) -> i32 {
    let fcount = FILE_FCOUNT(file);
    let mut base = base;
    let mut size = size;
    if page == 0 {
        if (base & (PAGE_SIZE - 1)) != 0 || (size & (PAGE_SIZE - 1)) != 0 { return -EINVAL; }
        base >>= PAGE_SHIFT;
        size >>= PAGE_SHIFT;
    }
    let reg = mtrr_del_page(-1, base, size);
    if reg < 0 { return reg; }
    if fcount.is_null() { return reg; }
    if *fcount.add(reg as usize) < 1 { return -EINVAL; }
    *fcount.add(reg as usize) -= 1;
    reg
}

// seq_file can seek but we ignore it.
// Format of control line: "base=%Lx size=%Lx type=%s" or "disable=%d"
pub unsafe fn mtrr_write(file: *mut file, buf: *const u8, mut len: usize, _ppos: *mut loff_t) -> isize {
    let mut line = [0u8; LINE_SIZE];
    len = core::cmp::min(len, LINE_SIZE - 1);
    let length = strncpy_from_user(line.as_mut_ptr(), buf, len);
    if length < 0 { return length as isize; }
    let mut ptr = line.as_mut_ptr().add(length as usize - 1);
    if length != 0 && *ptr == b'\n' { *ptr = 0; }
    if !strncmp(line.as_ptr(), b"disable=", 8) {
        let reg = simple_strtoul(line.as_ptr().add(8), &mut ptr, 0);
        let err = mtrr_del_page(reg, 0, 0);
        if err < 0 { return err as isize; }
        return len as isize;
    }
    if strncmp(line.as_ptr(), b"base=", 5) != 0 { return -EINVAL as isize; }
    let base = simple_strtoull(line.as_ptr().add(5), &mut ptr, 0);
    ptr = skip_spaces(ptr);
    if strncmp(ptr, b"size=", 5) != 0 { return -EINVAL as isize; }
    let size = simple_strtoull(ptr.add(5), &mut ptr, 0);
    if (base & 0xfff) != 0 || (size & 0xfff) != 0 { return -EINVAL as isize; }
    ptr = skip_spaces(ptr);
    if strncmp(ptr, b"type=", 5) != 0 { return -EINVAL as isize; }
    ptr = skip_spaces(ptr.add(5));
    let i = match_string(mtrr_strings.as_ptr(), MTRR_NUM_TYPES, ptr);
    if i < 0 { return i as isize; }
    let err = mtrr_add_page((base >> PAGE_SHIFT) as c_ulong, (size >> PAGE_SHIFT) as c_ulong, i as c_uint, true);
    if err < 0 { return err as isize; }
    len as isize
}

// The ioctl implementation and procfs registration retain the kernel ABI and
// are translated below using the surrounding kernel's declarations.
pub unsafe fn mtrr_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    let mut err: c_int = 0;
    let mut base: c_ulong = 0;
    let mut size: c_ulong = 0;
    let mut ty: mtrr_type = 0;
    let mut sentry = core::mem::MaybeUninit::<mtrr_sentry>::uninit();
    let mut gentry = core::mem::MaybeUninit::<mtrr_gentry>::zeroed();
    let user_arg = arg as *mut core::ffi::c_void;
    let _ = (file, cmd, user_arg, &mut err, &mut base, &mut size, &mut ty, &mut sentry, &mut gentry);
    match cmd {
        MTRRIOC_ADD_ENTRY => err = mtrr_file_add((*sentry.as_ptr()).base, (*sentry.as_ptr()).size, (*sentry.as_ptr()).type_, true, file, 0),
        MTRRIOC_SET_ENTRY => err = mtrr_add((*sentry.as_ptr()).base, (*sentry.as_ptr()).size, (*sentry.as_ptr()).type_, false),
        MTRRIOC_DEL_ENTRY => err = mtrr_file_del((*sentry.as_ptr()).base, (*sentry.as_ptr()).size, file, 0),
        MTRRIOC_KILL_ENTRY => err = mtrr_del(-1, (*sentry.as_ptr()).base, (*sentry.as_ptr()).size),
        MTRRIOC_ADD_PAGE_ENTRY => err = mtrr_file_add((*sentry.as_ptr()).base, (*sentry.as_ptr()).size, (*sentry.as_ptr()).type_, true, file, 1),
        MTRRIOC_SET_PAGE_ENTRY => err = mtrr_add_page((*sentry.as_ptr()).base, (*sentry.as_ptr()).size, (*sentry.as_ptr()).type_, false),
        MTRRIOC_DEL_PAGE_ENTRY => err = mtrr_file_del((*sentry.as_ptr()).base, (*sentry.as_ptr()).size, file, 1),
        MTRRIOC_KILL_PAGE_ENTRY => err = mtrr_del_page(-1, (*sentry.as_ptr()).base, (*sentry.as_ptr()).size),
        MTRRIOC_GET_ENTRY => {
            if (*gentry.as_ptr()).regnum >= num_var_ranges { return -EINVAL as c_long; }
            mtrr_if.get((*gentry.as_ptr()).regnum, &mut base, &mut size, &mut ty);
            if base + size - 1 >= (1UL << (8 * core::mem::size_of::<u64>() - PAGE_SHIFT)) || size >= (1UL << (8 * core::mem::size_of::<u64>() - PAGE_SHIFT)) { (*gentry.as_mut_ptr()).base = 0; (*gentry.as_mut_ptr()).size = 0; (*gentry.as_mut_ptr()).type_ = 0; }
            else { (*gentry.as_mut_ptr()).base = base << PAGE_SHIFT; (*gentry.as_mut_ptr()).size = size << PAGE_SHIFT; (*gentry.as_mut_ptr()).type_ = ty; }
        }
        MTRRIOC_GET_PAGE_ENTRY => {
            if (*gentry.as_ptr()).regnum >= num_var_ranges { return -EINVAL as c_long; }
            mtrr_if.get((*gentry.as_ptr()).regnum, &mut base, &mut size, &mut ty);
            (*gentry.as_mut_ptr()).base = base; (*gentry.as_mut_ptr()).size = size; (*gentry.as_mut_ptr()).type_ = ty;
        }
        _ => return -ENOTTY as c_long,
    }
    if err != 0 { return err as c_long; }
    if cmd == MTRRIOC_GET_ENTRY || cmd == MTRRIOC_GET_PAGE_ENTRY { copy_to_user(user_arg, gentry.as_ptr() as *const _, core::mem::size_of::<mtrr_gentry>()); }
    err as c_long
}

// CONFIG_PROC_FS implementation hooks.
pub unsafe fn mtrr_close(ino: *mut inode, file: *mut file) -> c_int {
    let fcount = FILE_FCOUNT(file);
    if !fcount.is_null() {
        for i in 0..num_var_ranges {
            while *fcount.add(i) > 0 { mtrr_del(i as c_int, 0, 0); *fcount.add(i) -= 1; }
        }
        kfree(fcount as *mut core::ffi::c_void);
        FILE_FCOUNT_SET(file, core::ptr::null_mut());
    }
    single_release(ino, file)
}

pub unsafe fn mtrr_seq_show(seq: *mut seq_file, _offset: *mut core::ffi::c_void) -> c_int {
    let mut base = 0; let mut size = 0; let mut ty = 0;
    for i in 0..num_var_ranges {
        mtrr_if.get(i, &mut base, &mut size, &mut ty);
        if size == 0 { mtrr_usage_table[i] = 0; continue; }
        let (factor, out_size) = if size < (0x100000 >> PAGE_SHIFT) { ('K' as c_char, size << (PAGE_SHIFT - 10)) } else { ('M' as c_char, size >> (20 - PAGE_SHIFT)) };
        seq_printf(seq, b"reg%02i: base=0x%06lx000 (%5luMB), size=%5lu%cB, count=%d: %s\n\0".as_ptr(), i, base, base >> (20 - PAGE_SHIFT), out_size, factor, mtrr_usage_table[i], mtrr_attrib_to_str(ty));
    }
    0
}

pub unsafe fn mtrr_open(inode: *mut inode, file: *mut file) -> c_int {
    if mtrr_if.is_null() { return -EIO; }
    if mtrr_if.get.is_none() { return -ENXIO; }
    if !capable(CAP_SYS_ADMIN) { return -EPERM; }
    single_open(file, Some(mtrr_seq_show), core::ptr::null_mut())
}

// proc_ops mtrr_proc_ops = { open: mtrr_open, read: seq_read, lseek: seq_lseek,
// write: mtrr_write, ioctl: mtrr_ioctl, release: mtrr_close };
pub unsafe fn mtrr_if_init() -> c_int {
    let c = &boot_cpu_data;
    if !cpu_has(c, X86_FEATURE_MTRR) && !cpu_has(c, X86_FEATURE_K6_MTRR) && !cpu_has(c, X86_FEATURE_CYRIX_ARR) && !cpu_has(c, X86_FEATURE_CENTAUR_MCR) { return -ENODEV; }
    proc_create(b"mtrr\0".as_ptr(), S_IWUSR | S_IRUGO, core::ptr::null_mut(), &mtrr_proc_ops);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
