// SPDX-License-Identifier: GPL-2.0-only
/* -*- linux-c -*- ------------------------------------------------------- *
 *
 *   Copyright (C) 1991, 1992 Linus Torvalds
 *   Copyright 2007 rPath, Inc. - All Rights Reserved
 *   Copyright 2009 Intel Corporation; author H. Peter Anvin
 *
 * ----------------------------------------------------------------------- */

/* Get EDD BIOS disk information */

// Dependencies supplied by boot.h, <linux/edd.h>, and string.h remain external.
// The CONFIG_EDD conditional is preserved from the C translation unit.

// Translated body of CONFIG_EDD; retain this block when CONFIG_EDD is enabled.
    unsafe fn read_mbr(devno: u8, buf: *mut core::ffi::c_void) -> i32 {
        let mut ireg: biosregs = core::mem::zeroed();
        let mut oreg: biosregs = core::mem::zeroed();

        initregs(&mut ireg);
        ireg.ax = 0x0201; // Legacy Read, one sector
        ireg.cx = 0x0001; // Sector 0-0-1
        ireg.dl = devno;
        ireg.bx = buf as usize;

        intcall(0x13, &ireg, &mut oreg);

        -((oreg.eflags & X86_EFLAGS_CF) as i32) // 0 or -1
    }

    unsafe fn read_mbr_sig(devno: u8, ei: *mut edd_info, mbrsig: *mut u32) -> u32 {
        let mut sector_size: i32 = (*ei).params.bytes_per_sector as i32;
        let mbrbuf_ptr: *mut u8;
        let mbrbuf_end: *mut u8;
        let buf_base: u32;
        let mbr_base: u32;
        unsafe extern "C" {
            static mut _end: u8;
        }
        let mbr_magic: u16;

        if sector_size == 0 {
            sector_size = 512; // Best available guess
        }

        // Produce a naturally aligned buffer on the heap
        buf_base = (ds() << 4).wrapping_add((&raw mut _end) as u32);
        mbr_base = buf_base
            .wrapping_add(sector_size as u32 - 1)
            & !(sector_size as u32 - 1);
        mbrbuf_ptr = (&raw mut _end).add(mbr_base.wrapping_sub(buf_base) as usize);
        mbrbuf_end = mbrbuf_ptr.add(sector_size as usize);

        // Make sure we actually have space on the heap...
        if (boot_params.hdr.loadflags & CAN_USE_HEAP) == 0 {
            return u32::MAX;
        }
        if (mbrbuf_end as usize) > boot_params.hdr.heap_end_ptr as usize {
            return u32::MAX;
        }

        core::ptr::write_bytes(mbrbuf_ptr, 0, sector_size as usize);
        if read_mbr(devno, mbrbuf_ptr.cast()) != 0 {
            return u32::MAX;
        }

        *mbrsig = core::ptr::read_unaligned(mbrbuf_ptr.add(EDD_MBR_SIG_OFFSET as usize).cast());
        mbr_magic = core::ptr::read_unaligned(mbrbuf_ptr.add(510).cast());

        // check for valid MBR magic
        if mbr_magic == 0xAA55 { 0 } else { u32::MAX }
    }

    unsafe fn get_edd_info(devno: u8, ei: *mut edd_info) -> i32 {
        let mut ireg: biosregs = core::mem::zeroed();
        let mut oreg: biosregs = core::mem::zeroed();

        core::ptr::write_bytes(ei.cast::<u8>(), 0, core::mem::size_of::<edd_info>());

        // Check Extensions Present
        initregs(&mut ireg);
        ireg.ah = 0x41;
        ireg.bx = EDDMAGIC1;
        ireg.dl = devno;
        intcall(0x13, &ireg, &mut oreg);

        if (oreg.eflags & X86_EFLAGS_CF) != 0 {
            return -1; // No extended information
        }
        if oreg.bx != EDDMAGIC2 {
            return -1;
        }

        (*ei).device = devno;
        (*ei).version = oreg.ah; // EDD version number
        (*ei).interface_support = oreg.cx; // EDD functionality subsets

        // Extended Get Device Parameters
        (*ei).params.length = core::mem::size_of_val(&(*ei).params) as _;
        ireg.ah = 0x48;
        ireg.si = (&raw mut (*ei).params) as usize;
        intcall(0x13, &ireg, &mut oreg);

        // Get legacy CHS parameters
        // Ralf Brown recommends setting ES:DI to 0:0
        ireg.ah = 0x08;
        ireg.es = 0;
        intcall(0x13, &ireg, &mut oreg);

        if (oreg.eflags & X86_EFLAGS_CF) == 0 {
            (*ei).legacy_max_cylinder = oreg.ch as _ + (((oreg.cl & 0xc0) as _) << 2);
            (*ei).legacy_max_head = oreg.dh;
            (*ei).legacy_sectors_per_track = oreg.cl & 0x3f;
        }

        0
    }

    pub unsafe fn query_edd() {
        let mut eddarg = [0u8; 8];
        let mut do_mbr = true;
        let mut do_edd = true; // !IS_ENABLED(CONFIG_EDD_OFF)
        let be_quiet: bool;
        let mut ei: edd_info = core::mem::zeroed();
        let mut edp = boot_params.eddbuf;
        let mut mbrptr = boot_params.edd_mbr_sig_buffer;

        if cmdline_find_option(b"edd\0".as_ptr().cast(), eddarg.as_mut_ptr().cast(), eddarg.len()) > 0 {
            if strcmp(eddarg.as_ptr(), b"skipmbr\0".as_ptr()) == 0 || strcmp(eddarg.as_ptr(), b"skip\0".as_ptr()) == 0 {
                do_edd = true;
                do_mbr = false;
            } else if strcmp(eddarg.as_ptr(), b"off\0".as_ptr()) == 0 {
                do_edd = false;
            } else if strcmp(eddarg.as_ptr(), b"on\0".as_ptr()) == 0 {
                do_edd = true;
            }
        }
        be_quiet = cmdline_find_option_bool(b"quiet\0".as_ptr().cast());

        if !do_edd { return; }
        if !be_quiet { printf(b"Probing EDD (edd=off to disable)... \0".as_ptr().cast()); }

        for devno in 0x80u8..0x80u8 + EDD_MBR_SIG_MAX as u8 {
            if get_edd_info(devno, &mut ei) == 0 && boot_params.eddbuf_entries < EDDMAXNR {
                core::ptr::copy_nonoverlapping(&ei, edp, 1);
                edp = edp.add(1);
                boot_params.eddbuf_entries += 1;
            }
            if do_mbr && read_mbr_sig(devno, &mut ei, mbrptr) == 0 {
                boot_params.edd_mbr_sig_buf_entries = devno as _ - 0x80 + 1;
            }
            mbrptr = mbrptr.add(1);
        }
        if !be_quiet { printf(b"ok\n\0".as_ptr().cast()); }
    }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
