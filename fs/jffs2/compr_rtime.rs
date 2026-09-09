/*
 * JFFS2 -- Journalling Flash File System, Version 2.
 *
 * Copyright © 2001-2007 Red Hat, Inc.
 * Copyright © 2004-2010 David Woodhouse <dwmw2@infradead.org>
 *
 * Created by Arjan van de Ven <arjanv@redhat.com>
 *
 * Very simple lz77-ish encoder.
 *
 * Theory of operation: Both encoder and decoder have a list of "last
 * occurrences" for every possible source-value; after sending the
 * first source-byte, the second byte indicated the "run" length of
 * matches
 */

// C dependencies supplied by the surrounding JFFS2 translation.

unsafe fn jffs2_rtime_compress(
    data_in: *mut u8,
    cpage_out: *mut u8,
    sourcelen: *mut u32,
    dstlen: *mut u32,
) -> i32 {
    let mut positions = [0u16; 256];
    let mut outpos: i32 = 0;
    let mut pos: i32 = 0;

    if *dstlen <= 3 {
        return -1;
    }

    while pos < *sourcelen as i32 && outpos <= (*dstlen as i32) - 2 {
        let mut backpos: i32;
        let mut runlen: i32 = 0;
        let value: u8;

        value = *data_in.add(pos as usize);
        *cpage_out.add(outpos as usize) = *data_in.add(pos as usize);
        outpos += 1;
        pos += 1;

        backpos = positions[value as usize] as i32;
        positions[value as usize] = pos as u16;

        while backpos < pos
            && pos < *sourcelen as i32
            && *data_in.add(pos as usize) == *data_in.add(backpos as usize)
            && runlen < 255
        {
            pos += 1;
            backpos += 1;
            runlen += 1;
        }
        *cpage_out.add(outpos as usize) = runlen as u8;
        outpos += 1;
    }

    if outpos >= pos {
        /* We failed */
        return -1;
    }

    /* Tell the caller how much we managed to compress, and how much space it took */
    *sourcelen = pos as u32;
    *dstlen = outpos as u32;
    0
}

unsafe fn jffs2_rtime_decompress(
    data_in: *mut u8,
    cpage_out: *mut u8,
    srclen: u32,
    destlen: u32,
) -> i32 {
    let mut positions = [0u16; 256];
    let mut outpos: i32 = 0;
    let mut pos: i32 = 0;

    let _ = srclen;

    while outpos < destlen as i32 {
        let value: u8;
        let mut backoffs: i32;
        let mut repeat: i32;

        value = *data_in.add(pos as usize);
        pos += 1;
        *cpage_out.add(outpos as usize) = value; /* first the verbatim copied byte */
        outpos += 1;
        repeat = *data_in.add(pos as usize) as i32;
        pos += 1;
        backoffs = positions[value as usize] as i32;

        positions[value as usize] = outpos as u16;
        if repeat != 0 {
            if outpos + repeat > destlen as i32 {
                return 1;
            }
            if backoffs + repeat >= outpos {
                while repeat != 0 {
                    let byte = *cpage_out.add(backoffs as usize);
                    *cpage_out.add(outpos as usize) = byte;
                    outpos += 1;
                    backoffs += 1;
                    repeat -= 1;
                }
            } else {
                std::ptr::copy(
                    cpage_out.add(backoffs as usize),
                    cpage_out.add(outpos as usize),
                    repeat as usize,
                );
                outpos += repeat;
            }
        }
    }
    0
}

// The compressor structure and registration functions are supplied by compr.h
// and the surrounding JFFS2 translation. The original C initializer is retained
// here in source-level form pending the shared Rust definition of that type.
// static mut jffs2_rtime_comp: struct jffs2_compressor = {
//     .priority = JFFS2_RTIME_PRIORITY,
//     .name = "rtime",
//     .compr = JFFS2_COMPR_RTIME,
//     .compress = jffs2_rtime_compress,
//     .decompress = jffs2_rtime_decompress,
//     .disabled = 0, // JFFS2_RTIME_DISABLED sets this to 1 at build time.
// };

unsafe extern "C" {
    static mut jffs2_rtime_comp: jffs2_compressor;
    fn jffs2_register_compressor(comp: *mut jffs2_compressor) -> i32;
    fn jffs2_unregister_compressor(comp: *mut jffs2_compressor);
}

// Opaque declaration of the externally defined compressor type.
#[repr(C)]
pub struct jffs2_compressor {
    _private: [u8; 0],
}

unsafe fn jffs2_rtime_init() -> i32 {
    jffs2_register_compressor(&raw mut jffs2_rtime_comp)
}

unsafe fn jffs2_rtime_exit() {
    jffs2_unregister_compressor(&raw mut jffs2_rtime_comp);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
