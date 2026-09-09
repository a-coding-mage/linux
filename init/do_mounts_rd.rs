// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the corresponding kernel headers and source files.

static mut IN_FILE: *mut file = core::ptr::null_mut();
static mut OUT_FILE: *mut file = core::ptr::null_mut();
static mut IN_POS: loff_t = 0;
static mut OUT_POS: loff_t = 0;

static mut rd_image_start: i32 = 0;

unsafe extern "C" fn ramdisk_start_setup(str_: *mut i8) -> i32 {
    pr_warn!("ramdisk_start= option is deprecated and will be removed soon\n");
    (kstrtoint(str_, 0, &mut rd_image_start) == 0) as i32
}

unsafe extern "C" fn identify_ramdisk_image(
    file_: *mut file,
    mut pos: loff_t,
    decompressor: *mut decompress_fn,
) -> i32 {
    let size: i32 = 512;
    let mut nblocks: i32 = -1;
    let mut buf = kmalloc(size as usize, GFP_KERNEL) as *mut u8;
    if buf.is_null() {
        return -ENOMEM;
    }

    let minixsb = buf as *mut minix_super_block;
    let romfsb = buf as *mut romfs_super_block;
    let cramfsb = buf as *mut cramfs_super;
    let squashfsb = buf as *mut squashfs_super_block;
    let mut compress_name: *const i8 = core::ptr::null();
    let mut n: c_ulong = 0;
    let start_block = rd_image_start;
    core::ptr::write_bytes(buf, 0xe5, size as usize);

    pos = (start_block as loff_t) * BLOCK_SIZE;
    kernel_read(file_, buf as *mut core::ffi::c_void, size as usize, &mut pos);

    *decompressor = decompress_method(buf, size as usize, &mut compress_name);
    if !compress_name.is_null() {
        printk!(KERN_NOTICE "RAMDISK: %s image found at block %d\n", compress_name, start_block);
        if (*decompressor).is_none() {
            printk!(KERN_EMERG "RAMDISK: %s decompressor not configured!\n", compress_name);
        }
        nblocks = 0;
        goto_done!(buf, nblocks);
    }

    if (*romfsb).word0 == ROMSB_WORD0 && (*romfsb).word1 == ROMSB_WORD1 {
        printk!(KERN_NOTICE "RAMDISK: romfs filesystem found at block %d\n", start_block);
        nblocks = ((ntohl((*romfsb).size) + BLOCK_SIZE - 1) >> BLOCK_SIZE_BITS) as i32;
        goto_done!(buf, nblocks);
    }
    if (*cramfsb).magic == CRAMFS_MAGIC {
        printk!(KERN_NOTICE "RAMDISK: cramfs filesystem found at block %d\n", start_block);
        nblocks = (((*cramfsb).size + BLOCK_SIZE - 1) >> BLOCK_SIZE_BITS) as i32;
        goto_done!(buf, nblocks);
    }
    if le32_to_cpu((*squashfsb).s_magic) == SQUASHFS_MAGIC {
        printk!(KERN_NOTICE "RAMDISK: squashfs filesystem found at block %d\n", start_block);
        nblocks = ((le64_to_cpu((*squashfsb).bytes_used) + BLOCK_SIZE - 1) >> BLOCK_SIZE_BITS) as i32;
        goto_done!(buf, nblocks);
    }

    pos = (start_block as loff_t) * BLOCK_SIZE + 0x200;
    kernel_read(file_, buf as *mut core::ffi::c_void, size as usize, &mut pos);
    if (*cramfsb).magic == CRAMFS_MAGIC {
        printk!(KERN_NOTICE "RAMDISK: cramfs filesystem found at block %d\n", start_block);
        nblocks = (((*cramfsb).size + BLOCK_SIZE - 1) >> BLOCK_SIZE_BITS) as i32;
        goto_done!(buf, nblocks);
    }

    pos = ((start_block + 1) as loff_t) * BLOCK_SIZE;
    kernel_read(file_, buf as *mut core::ffi::c_void, size as usize, &mut pos);
    if (*minixsb).s_magic == MINIX_SUPER_MAGIC || (*minixsb).s_magic == MINIX_SUPER_MAGIC2 {
        printk!(KERN_NOTICE "RAMDISK: Minix filesystem found at block %d\n", start_block);
        nblocks = ((*minixsb).s_nzones << (*minixsb).s_log_zone_size) as i32;
        goto_done!(buf, nblocks);
    }
    n = ext2_image_size(buf);
    if n != 0 {
        printk!(KERN_NOTICE "RAMDISK: ext2 filesystem found at block %d\n", start_block);
        nblocks = n as i32;
        goto_done!(buf, nblocks);
    }
    printk!(KERN_NOTICE "RAMDISK: Couldn't find valid RAM disk image starting at %d.\n", start_block);
    kfree(buf as *mut core::ffi::c_void);
    nblocks
}

unsafe fn nr_blocks(file_: *mut file) -> c_ulong {
    let inode = (*(*file_).f_mapping).host;
    if !S_ISBLK((*inode).i_mode) { return 0; }
    (i_size_read(inode) >> 10) as c_ulong
}

pub unsafe extern "C" fn rd_load_image() -> i32 {
    let mut res = 0;
    let mut buf: *mut i8 = core::ptr::null_mut();
    let mut rotate: u16 = 0;
    let mut decompressor: decompress_fn = None;
    let rotator = [b'|' as i8, b'/' as i8, b'-' as i8, b'\\' as i8];
    OUT_FILE = filp_open(c"/dev/ram".as_ptr(), O_RDWR, 0);
    if IS_ERR(OUT_FILE) { goto_out!(res, buf); }
    IN_FILE = filp_open(c"/initrd.image".as_ptr(), O_RDONLY, 0);
    if IS_ERR(IN_FILE) { fput(OUT_FILE); goto_out!(res, buf); }
    IN_POS = (rd_image_start as loff_t) * BLOCK_SIZE;
    let nblocks = identify_ramdisk_image(IN_FILE, IN_POS, &mut decompressor);
    if nblocks < 0 { fput(IN_FILE); fput(OUT_FILE); goto_out!(res, buf); }
    if nblocks == 0 { if crd_load(decompressor) == 0 { res = 1; } fput(IN_FILE); fput(OUT_FILE); goto_out!(res, buf); }
    let rd_blocks = nr_blocks(OUT_FILE);
    if nblocks as c_ulong > rd_blocks { printk!("RAMDISK: image too big! (%dKiB/%ldKiB)\n", nblocks, rd_blocks); fput(IN_FILE); fput(OUT_FILE); goto_out!(res, buf); }
    let devblocks = nblocks as c_ulong;
    if devblocks == 0 { printk!(KERN_ERR "RAMDISK: could not determine device size\n"); fput(IN_FILE); fput(OUT_FILE); goto_out!(res, buf); }
    buf = kmalloc(BLOCK_SIZE as usize, GFP_KERNEL) as *mut i8;
    if buf.is_null() { printk!(KERN_ERR "RAMDISK: could not allocate buffer\n"); fput(IN_FILE); fput(OUT_FILE); goto_out!(res, buf); }
    let nr_disks = ((nblocks as c_ulong - 1) / devblocks) + 1;
    pr_notice!("RAMDISK: Loading %dKiB [%ld disk%s] into ram disk... ", nblocks, nr_disks, str_plural(nr_disks));
    for i in 0..nblocks { if i != 0 && (i as c_ulong) % devblocks == 0 { pr_cont!("done disk #1.\n"); rotate = 0; fput(IN_FILE); break; } kernel_read(IN_FILE, buf as *mut _, BLOCK_SIZE as usize, &mut IN_POS); kernel_write(OUT_FILE, buf as *mut _, BLOCK_SIZE as usize, &mut OUT_POS); if !IS_ENABLED!(CONFIG_S390) && i % 16 == 0 { pr_cont!("%c\b", rotator[(rotate & 3) as usize]); rotate += 1; } }
    pr_cont!("done.\n"); res = 1;
    fput(IN_FILE); fput(OUT_FILE);
    goto_out!(res, buf)
}

static mut exit_code: i32 = 0;
static mut decompress_error: i32 = 0;

unsafe extern "C" fn compr_fill(buf: *mut core::ffi::c_void, len: c_ulong) -> c_long { let r = kernel_read(IN_FILE, buf, len as usize, &mut IN_POS); if r < 0 { printk!(KERN_ERR "RAMDISK: error while reading compressed data"); } else if r == 0 { printk!(KERN_ERR "RAMDISK: EOF while reading compressed data"); } r as c_long }
unsafe extern "C" fn compr_flush(window: *mut core::ffi::c_void, outcnt: c_ulong) -> c_long { let written = kernel_write(OUT_FILE, window, outcnt as usize, &mut OUT_POS); if written != outcnt as isize { if decompress_error == 0 { printk!(KERN_ERR "RAMDISK: incomplete write (%ld != %ld)\n", written, outcnt); } decompress_error = 1; return -1; } outcnt as c_long }
unsafe extern "C" fn error(x: *mut i8) { printk!(KERN_ERR "%s\n", x); exit_code = 1; decompress_error = 1; }
unsafe fn crd_load(deco: decompress_fn) -> i32 { if deco.is_none() { pr_emerg!("Invalid ramdisk decompression routine.  Select appropriate config option.\n"); panic!("Could not decompress initial ramdisk image."); } let mut result = deco.unwrap()(core::ptr::null_mut(), 0, compr_fill, compr_flush, None, None, error); if decompress_error != 0 { result = 1; } result }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
