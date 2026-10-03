// SPDX-License-Identifier: GPL-2.0
//! Legacy RAM disk image recognition, copying, and streaming decompression.

#[allow(
    clippy::all,
    dead_code,
    missing_docs,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    improper_ctypes,
    unreachable_pub,
    unsafe_op_in_unsafe_fn
)]
mod bindings {
    use kernel::bindings::{file, inode};
    use kernel::ffi;

    include!(concat!(
        env!("OBJTREE"),
        "/rust/bindings/init_ramdisk_generated.rs"
    ));
}

mod main_printk;
mod main_setup;

use core::ptr;
use kernel::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use main_printk::main_printk;

// The boot loader and its decompressor callbacks run synchronously, as in C.
static mut IN_FILE: *mut kernel::bindings::file = ptr::null_mut();
static mut OUT_FILE: *mut kernel::bindings::file = ptr::null_mut();
static mut IN_POS: bindings::loff_t = 0;
static mut OUT_POS: bindings::loff_t = 0;
static mut EXIT_CODE: c_int = 0;
static mut DECOMPRESS_ERROR: c_int = 0;

/// Starting 1 KiB block, also set by architecture boot setup.
#[no_mangle]
#[link_section = ".init.data"]
pub static mut rd_image_start: c_int = 0;

#[link_section = ".init.text"]
unsafe extern "C" fn ramdisk_start_setup(argument: *mut c_char) -> c_int {
    // SAFETY: setup parsing serializes access and supplies a terminated string.
    unsafe {
        main_printk!(
            "ramdisk_start_setup",
            b"\x014ramdisk_start= option is deprecated and will be removed soon\n\0"
        );
        (bindings::kstrtoint(argument, 0, ptr::addr_of_mut!(rd_image_start)) == 0) as c_int
    }
}

main_setup::setup_param!(
    "ramdisk_start=",
    RAMDISK_START_SETUP,
    Some(ramdisk_start_setup),
    0
);

// C's multiplication happens in int before assignment to loff_t.
#[inline]
fn block_position(block: c_int) -> bindings::loff_t {
    block.wrapping_mul(bindings::BLOCK_SIZE as c_int) as bindings::loff_t
}

#[inline]
fn is_err<T>(pointer: *const T) -> bool {
    pointer as usize >= (-(bindings::MAX_ERRNO as isize)) as usize
}

// This is the exact inline ext2_image_size() calculation from ext2_fs.h, using
// its canonical offsets. Its u64 result is subsequently converted to C ulong.
#[link_section = ".init.text"]
unsafe fn ext2_image_size(buffer: *const u8) -> u64 {
    // SAFETY: identify_ramdisk_image owns a 512-byte allocation; every header
    // offset read below is within that allocation. Unaligned reads also avoid
    // imposing alignment requirements beyond those of the byte buffer.
    unsafe {
        let magic = ptr::read_unaligned(buffer.add(bindings::EXT2_SB_MAGIC_OFFSET as usize).cast());
        if u16::from_le(magic) != bindings::EXT2_SUPER_MAGIC as u16 {
            return 0;
        }
        let blocks =
            ptr::read_unaligned(buffer.add(bindings::EXT2_SB_BLOCKS_OFFSET as usize).cast());
        let shift = ptr::read_unaligned(buffer.add(bindings::EXT2_SB_BSIZE_OFFSET as usize).cast());
        (u32::from_le(blocks) as u64).wrapping_shl(u32::from_le(shift))
    }
}

#[link_section = ".init.text"]
unsafe fn identify_ramdisk_image(
    file: *mut kernel::bindings::file,
    _position: bindings::loff_t,
    decompressor: *mut bindings::decompress_fn,
) -> c_int {
    // SAFETY: the file is open, the result pointer is writable, and boot loading
    // is serialized. The original code deliberately ignores probe-read return
    // values and retains bytes left by prior short reads. Preserve that behavior.
    unsafe {
        let buffer = bindings::rust_init_rd_alloc_probe().cast::<u8>();
        if buffer.is_null() {
            return -(bindings::ENOMEM as c_int);
        }
        let minix = buffer.cast::<bindings::minix_super_block>();
        let romfs = buffer.cast::<bindings::romfs_super_block>();
        let cramfs = buffer.cast::<bindings::cramfs_super>();
        let squashfs = buffer.cast::<bindings::squashfs_super_block>();
        let start = rd_image_start;
        ptr::write_bytes(buffer, 0xe5, 512);

        let blocks = 'identified: {
            let mut position = block_position(start);
            bindings::kernel_read(file, buffer.cast(), 512, &mut position);
            let mut compression_name: *const c_char = ptr::null();
            decompressor.write(bindings::decompress_method(
                buffer,
                512,
                &mut compression_name,
            ));
            if !compression_name.is_null() {
                main_printk!(
                    "identify_ramdisk_image",
                    b"\x015RAMDISK: %s image found at block %d\n\0",
                    compression_name,
                    start
                );
                if (*decompressor).is_none() {
                    main_printk!(
                        "identify_ramdisk_image",
                        b"\x010RAMDISK: %s decompressor not configured!\n\0",
                        compression_name
                    );
                }
                break 'identified 0;
            }
            if (*romfs).word0 == bindings::RUST_INIT_RD_ROMSB_WORD0
                && (*romfs).word1 == bindings::RUST_INIT_RD_ROMSB_WORD1
            {
                main_printk!(
                    "identify_ramdisk_image",
                    b"\x015RAMDISK: romfs filesystem found at block %d\n\0",
                    start
                );
                break 'identified (u32::from_be((*romfs).size)
                    .wrapping_add(bindings::BLOCK_SIZE - 1)
                    >> bindings::BLOCK_SIZE_BITS) as c_int;
            }
            if (*cramfs).magic == bindings::CRAMFS_MAGIC {
                main_printk!(
                    "identify_ramdisk_image",
                    b"\x015RAMDISK: cramfs filesystem found at block %d\n\0",
                    start
                );
                break 'identified ((*cramfs).size.wrapping_add(bindings::BLOCK_SIZE - 1)
                    >> bindings::BLOCK_SIZE_BITS) as c_int;
            }
            if u32::from_le((*squashfs).s_magic) == bindings::SQUASHFS_MAGIC {
                main_printk!(
                    "identify_ramdisk_image",
                    b"\x015RAMDISK: squashfs filesystem found at block %d\n\0",
                    start
                );
                break 'identified (u64::from_le((*squashfs).bytes_used)
                    .wrapping_add((bindings::BLOCK_SIZE - 1) as u64)
                    >> bindings::BLOCK_SIZE_BITS) as c_int;
            }

            position = start
                .wrapping_mul(bindings::BLOCK_SIZE as c_int)
                .wrapping_add(0x200) as bindings::loff_t;
            bindings::kernel_read(file, buffer.cast(), 512, &mut position);
            if (*cramfs).magic == bindings::CRAMFS_MAGIC {
                main_printk!(
                    "identify_ramdisk_image",
                    b"\x015RAMDISK: cramfs filesystem found at block %d\n\0",
                    start
                );
                break 'identified ((*cramfs).size.wrapping_add(bindings::BLOCK_SIZE - 1)
                    >> bindings::BLOCK_SIZE_BITS) as c_int;
            }

            position = block_position(start.wrapping_add(1));
            bindings::kernel_read(file, buffer.cast(), 512, &mut position);
            if (*minix).s_magic == bindings::MINIX_SUPER_MAGIC as u16
                || (*minix).s_magic == bindings::MINIX_SUPER_MAGIC2 as u16
            {
                main_printk!(
                    "identify_ramdisk_image",
                    b"\x015RAMDISK: Minix filesystem found at block %d\n\0",
                    start
                );
                break 'identified ((*minix).s_nzones as c_int)
                    .wrapping_shl((*minix).s_log_zone_size as u32);
            }
            let blocks = ext2_image_size(buffer) as c_ulong;
            if blocks != 0 {
                main_printk!(
                    "identify_ramdisk_image",
                    b"\x015RAMDISK: ext2 filesystem found at block %d\n\0",
                    start
                );
                break 'identified blocks as c_int;
            }
            main_printk!(
                "identify_ramdisk_image",
                b"\x015RAMDISK: Couldn't find valid RAM disk image starting at %d.\n\0",
                start
            );
            -1
        };
        bindings::kfree(buffer.cast());
        blocks
    }
}

unsafe fn nr_blocks(file: *mut kernel::bindings::file) -> c_ulong {
    // SAFETY: the open file pins its mapping and host inode. The C companion
    // invokes the native i_size_read concurrency primitive for this config.
    unsafe {
        let inode = (*(*file).f_mapping).host;
        if ((*inode).i_mode as u32 & bindings::S_IFMT) != bindings::S_IFBLK {
            return 0;
        }
        (bindings::rust_init_rd_i_size_read(inode) >> 10) as c_ulong
    }
}

/// Load /initrd.image into /dev/ram, returning one only for a successful load.
///
/// # Safety
/// Called during serialized boot with the original initrd filesystem state.
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn rd_load_image() -> c_int {
    // SAFETY: initrd loading owns these file references and callback globals.
    // The flags track successful opens, reproducing the original goto cleanup
    // labels without ever passing a failed-open error pointer to fput.
    unsafe {
        let mut output_open = false;
        let mut input_open = false;
        let mut buffer: *mut c_void = ptr::null_mut();
        let result = 'load: {
            OUT_FILE =
                bindings::filp_open(c"/dev/ram".as_ptr().cast(), bindings::O_RDWR as c_int, 0);
            if is_err(OUT_FILE) {
                break 'load 0;
            }
            output_open = true;
            IN_FILE = bindings::filp_open(
                c"/initrd.image".as_ptr().cast(),
                bindings::O_RDONLY as c_int,
                0,
            );
            if is_err(IN_FILE) {
                break 'load 0;
            }
            input_open = true;
            IN_POS = block_position(rd_image_start);
            let mut decompressor: bindings::decompress_fn = None;
            let blocks = identify_ramdisk_image(IN_FILE, IN_POS, &mut decompressor);
            if blocks < 0 {
                break 'load 0;
            }
            if blocks == 0 {
                break 'load (crd_load(decompressor) == 0) as c_int;
            }
            // In spite of its name, blocks counts KiB, not device sectors.
            let ram_blocks = nr_blocks(OUT_FILE);
            if blocks as c_ulong > ram_blocks {
                main_printk!(
                    "rd_load_image",
                    b"RAMDISK: image too big! (%dKiB/%ldKiB)\n\0",
                    blocks,
                    ram_blocks
                );
                break 'load 0;
            }
            let device_blocks = blocks as c_ulong;
            if device_blocks == 0 {
                main_printk!(
                    "rd_load_image",
                    b"\x013RAMDISK: could not determine device size\n\0"
                );
                break 'load 0;
            }
            buffer = bindings::rust_init_rd_alloc_copy().cast();
            if buffer.is_null() {
                main_printk!(
                    "rd_load_image",
                    b"\x013RAMDISK: could not allocate buffer\n\0"
                );
                break 'load 0;
            }
            let disks = (blocks as c_ulong - 1) / device_blocks + 1;
            let plural = if disks == 1 { c"" } else { c"s" };
            main_printk!(
                "rd_load_image",
                b"\x015RAMDISK: Loading %dKiB [%ld disk%s] into ram disk... \0",
                blocks,
                disks,
                plural.as_ptr()
            );
            let rotator = [b'|', b'/', b'-', b'\\'];
            let mut rotate: u16 = 0;
            for index in 0..blocks {
                if index != 0 && index as c_ulong % device_blocks == 0 {
                    main_printk!("rd_load_image", b"\x01cdone disk #1.\n\0");
                    // This original multi-disk branch is unreachable because
                    // device_blocks == blocks. Retain its original fput/order.
                    bindings::fput(IN_FILE);
                    break;
                }
                // C intentionally ignores raw-copy read/write return values.
                bindings::kernel_read(
                    IN_FILE,
                    buffer,
                    bindings::BLOCK_SIZE as usize,
                    ptr::addr_of_mut!(IN_POS),
                );
                bindings::kernel_write(
                    OUT_FILE,
                    buffer,
                    bindings::BLOCK_SIZE as usize,
                    ptr::addr_of_mut!(OUT_POS),
                );
                if !cfg!(CONFIG_S390) && index % 16 == 0 {
                    main_printk!(
                        @index !cfg!(CONFIG_S390),
                        "rd_load_image",
                        b"\x01c%c\x08\0",
                        rotator[(rotate & 3) as usize] as c_int
                    );
                    rotate = rotate.wrapping_add(1);
                }
            }
            main_printk!("rd_load_image", b"\x01cdone.\n\0");
            1
        };
        if input_open {
            bindings::fput(IN_FILE);
        }
        if output_open {
            bindings::fput(OUT_FILE);
        }
        bindings::kfree(buffer);
        bindings::init_unlink(c"/dev/ram".as_ptr().cast());
        result
    }
}

#[link_section = ".init.text"]
unsafe extern "C" fn compr_fill(buffer: *mut c_void, length: c_ulong) -> c_long {
    // SAFETY: the synchronous decompressor supplies its writable input buffer
    // and the loader keeps the file open throughout the decompressor call.
    unsafe {
        let read =
            bindings::kernel_read(IN_FILE, buffer, length as usize, ptr::addr_of_mut!(IN_POS));
        if read < 0 {
            main_printk!(
                "compr_fill",
                b"\x013RAMDISK: error while reading compressed data\0"
            );
        } else if read == 0 {
            main_printk!(
                "compr_fill",
                b"\x013RAMDISK: EOF while reading compressed data\0"
            );
        }
        read as c_long
    }
}

#[link_section = ".init.text"]
unsafe extern "C" fn compr_flush(window: *mut c_void, count: c_ulong) -> c_long {
    // SAFETY: the synchronous decompressor supplies count readable bytes, and
    // the loader retains the output file reference until decompression returns.
    unsafe {
        let written =
            bindings::kernel_write(OUT_FILE, window, count as usize, ptr::addr_of_mut!(OUT_POS));
        // C compares signed long with unsigned long after unsigned conversion.
        if written as c_ulong != count {
            if DECOMPRESS_ERROR == 0 {
                main_printk!(
                    "compr_flush",
                    b"\x013RAMDISK: incomplete write (%ld != %ld)\n\0",
                    written as c_long,
                    count
                );
            }
            DECOMPRESS_ERROR = 1;
            return -1;
        }
        count as c_long
    }
}

#[link_section = ".init.text"]
unsafe extern "C" fn error(text: *mut c_char) {
    // SAFETY: the decompressor supplies a NUL-terminated error string.
    unsafe {
        main_printk!("error", b"\x013%s\n\0", text);
        EXIT_CODE = 1;
        DECOMPRESS_ERROR = 1;
    }
}

#[link_section = ".init.text"]
unsafe fn crd_load(decompressor: bindings::decompress_fn) -> c_int {
    // SAFETY: native decompress_method supplies the canonical function-pointer
    // ABI; files and positions remain live for all synchronous callbacks.
    unsafe {
        let Some(decompress) = decompressor else {
            main_printk!(
                "crd_load",
                b"\x010Invalid ramdisk decompression routine.  Select appropriate config option.\n\0"
            );
            bindings::panic(
                c"Could not decompress initial ramdisk image."
                    .as_ptr()
                    .cast(),
            );
        };
        let result = decompress(
            ptr::null_mut(),
            0,
            Some(compr_fill),
            Some(compr_flush),
            ptr::null_mut(),
            ptr::null_mut(),
            Some(error),
        );
        if DECOMPRESS_ERROR != 0 {
            1
        } else {
            result
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
