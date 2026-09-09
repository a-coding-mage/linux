// SPDX-License-Identifier: GPL-2.0-only
/*
 * Tag parsing.
 *
 * Copyright (C) 1995-2001 Russell King
 */

/*
 * This is the traditional way of passing data to the kernel at boot time.  Rather
 * than passing a fixed inflexible structure to the kernel, we pass a list
 * of variable-sized tags to the kernel.  The first tag must be a ATAG_CORE
 * tag for the list to be recognised (to distinguish the tagged list from
 * a param_struct).  The list is terminated with a zero-length tag (this tag
 * is not parsed in any way).
 */

// C headers and build-time configuration are supplied by other translation units.

static mut default_command_line: [core::ffi::c_char; COMMAND_LINE_SIZE] = CONFIG_CMDLINE;

#[cfg(not(MEM_SIZE))]
const MEM_SIZE: usize = 16 * 1024 * 1024;

#[repr(C)]
struct DefaultTags {
    hdr1: tag_header,
    core: tag_core,
    hdr2: tag_header,
    mem: tag_mem32,
    hdr3: tag_header,
}

static mut default_tags: DefaultTags = DefaultTags {
    hdr1: tag_header { size: tag_size::<tag_core>(), tag: ATAG_CORE },
    core: tag_core { flags: 1, pagesize: PAGE_SIZE, rootdev: 0xff },
    hdr2: tag_header { size: tag_size::<tag_mem32>(), tag: ATAG_MEM },
    mem: tag_mem32 { start: 0, size: MEM_SIZE },
    hdr3: tag_header { size: 0, tag: ATAG_NONE },
};

unsafe fn parse_tag_core(tag: *const tag) -> i32 {
    if (*tag).hdr.size > 2 {
        if ((*tag).u.core.flags & 1) == 0 {
            root_mountflags &= !MS_RDONLY;
        }
        ROOT_DEV = old_decode_dev((*tag).u.core.rootdev);
    }
    0
}

__tagtable!(ATAG_CORE, parse_tag_core);

unsafe fn parse_tag_mem32(tag: *const tag) -> i32 {
    arm_add_memory((*tag).u.mem.start, (*tag).u.mem.size)
}

__tagtable!(ATAG_MEM, parse_tag_mem32);

#[cfg(all(CONFIG_ARCH_FOOTBRIDGE, CONFIG_VGA_CONSOLE))]
unsafe fn parse_tag_videotext(tag: *const tag) -> i32 {
    vgacon_screen_info.orig_x = (*tag).u.videotext.x;
    vgacon_screen_info.orig_y = (*tag).u.videotext.y;
    vgacon_screen_info.orig_video_page = (*tag).u.videotext.video_page;
    vgacon_screen_info.orig_video_mode = (*tag).u.videotext.video_mode;
    vgacon_screen_info.orig_video_cols = (*tag).u.videotext.video_cols;
    vgacon_screen_info.orig_video_ega_bx = (*tag).u.videotext.video_ega_bx;
    vgacon_screen_info.orig_video_lines = (*tag).u.videotext.video_lines;
    vgacon_screen_info.orig_video_isVGA = (*tag).u.videotext.video_isvga;
    vgacon_screen_info.orig_video_points = (*tag).u.videotext.video_points;
    0
}

#[cfg(all(CONFIG_ARCH_FOOTBRIDGE, CONFIG_VGA_CONSOLE))]
__tagtable!(ATAG_VIDEOTEXT, parse_tag_videotext);

#[cfg(CONFIG_BLK_DEV_RAM)]
unsafe fn parse_tag_ramdisk(tag: *const tag) -> i32 {
    rd_image_start = (*tag).u.ramdisk.start;
    if (*tag).u.ramdisk.size != 0 {
        rd_size = (*tag).u.ramdisk.size;
    }
    0
}

#[cfg(CONFIG_BLK_DEV_RAM)]
__tagtable!(ATAG_RAMDISK, parse_tag_ramdisk);

unsafe fn parse_tag_serialnr(tag: *const tag) -> i32 {
    system_serial_low = (*tag).u.serialnr.low;
    system_serial_high = (*tag).u.serialnr.high;
    0
}

__tagtable!(ATAG_SERIAL, parse_tag_serialnr);

unsafe fn parse_tag_revision(tag: *const tag) -> i32 {
    system_rev = (*tag).u.revision.rev;
    0
}

__tagtable!(ATAG_REVISION, parse_tag_revision);

unsafe fn parse_tag_cmdline(tag: *const tag) -> i32 {
    // CONFIG_CMDLINE_EXTEND / CONFIG_CMDLINE_FORCE select the corresponding C branch.
    #[cfg(CONFIG_CMDLINE_EXTEND)]
    {
        strlcat(default_command_line.as_mut_ptr(), c" ".as_ptr(), COMMAND_LINE_SIZE);
        strlcat(default_command_line.as_mut_ptr(), (*tag).u.cmdline.cmdline.as_ptr(), COMMAND_LINE_SIZE);
    }
    #[cfg(all(not(CONFIG_CMDLINE_EXTEND), CONFIG_CMDLINE_FORCE))]
    pr_warn!("Ignoring tag cmdline (using the default kernel command line)\n");
    #[cfg(all(not(CONFIG_CMDLINE_EXTEND), not(CONFIG_CMDLINE_FORCE)))]
    strscpy(default_command_line.as_mut_ptr(), (*tag).u.cmdline.cmdline.as_ptr(), COMMAND_LINE_SIZE);
    0
}

__tagtable!(ATAG_CMDLINE, parse_tag_cmdline);

/*
 * Scan the tag table for this tag, and call its parse function.
 * The tag table is built by the linker from all the __tagtable
 * declarations.
 */
unsafe fn parse_tag(tag: *const tag) -> bool {
    extern "C" {
        static mut __tagtable_begin: tagtable;
        static mut __tagtable_end: tagtable;
    }
    let mut t: *mut tagtable = &raw mut __tagtable_begin;
    while t < &raw mut __tagtable_end {
        if (*tag).hdr.tag == (*t).tag {
            ((*t).parse)(tag);
            break;
        }
        t = t.add(1);
    }
    t < &raw mut __tagtable_end
}

/*
 * Parse all tags in the list, checking both the global and architecture
 * specific tag tables.
 */
unsafe fn parse_tags(mut t: *const tag) {
    while (*t).hdr.size != 0 {
        if !parse_tag(t) {
            pr_warn!("Ignoring unrecognised tag 0x{:08x}\n", (*t).hdr.tag);
        }
        t = tag_next(t);
    }
}

unsafe fn squash_mem_tags(mut tag: *mut tag) {
    while (*tag).hdr.size != 0 {
        if (*tag).hdr.tag == ATAG_MEM {
            (*tag).hdr.tag = ATAG_NONE;
        }
        tag = tag_next(tag);
    }
}

unsafe fn setup_machine_tags(atags_vaddr: *mut core::ffi::c_void, machine_nr: u32) -> *const machine_desc {
    let mut tags: *mut tag = &raw mut default_tags as *mut DefaultTags as *mut tag;
    let mut mdesc: *const machine_desc = core::ptr::null();
    let mut from = default_command_line.as_mut_ptr();

    default_tags.mem.start = PHYS_OFFSET;

    /* locate machine in the list of supported machines. */
    for_each_machine_desc!(p, {
        if machine_nr == (*p).nr {
            pr_info!("Machine: %s\n", (*p).name);
            mdesc = p;
            break;
        }
    });

    if mdesc.is_null() {
        return core::ptr::null();
    }

    if !atags_vaddr.is_null() {
        tags = atags_vaddr as *mut tag;
    } else if (*mdesc).atag_offset != 0 {
        tags = (PAGE_OFFSET + (*mdesc).atag_offset) as *mut tag;
    }

    #[cfg(CONFIG_DEPRECATED_PARAM_STRUCT)]
    if (*tags).hdr.tag != ATAG_CORE {
        convert_to_tag_list(tags);
    }
    if (*tags).hdr.tag != ATAG_CORE {
        early_print!("Warning: Neither atags nor dtb found\n");
        tags = &raw mut default_tags as *mut DefaultTags as *mut tag;
    }

    if let Some(fixup) = (*mdesc).fixup {
        fixup(tags, &mut from);
    }
    if (*tags).hdr.tag == ATAG_CORE {
        if memblock_phys_mem_size() != 0 {
            squash_mem_tags(tags);
        }
        save_atags(tags);
        parse_tags(tags);
    }
    strscpy(boot_command_line.as_mut_ptr(), from, COMMAND_LINE_SIZE);
    mdesc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
