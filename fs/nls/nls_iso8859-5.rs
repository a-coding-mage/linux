/*
 * linux/fs/nls/nls_iso8859-5.c
 *
 * Charset iso8859-5 translation tables.
 * Generated automatically from the Unicode and charset tables
 * from the Unicode Organization (www.unicode.org).
 * The Unicode to charset table has only exact mappings.
 */

// Kernel headers and symbols are supplied by the surrounding translation.

const fn charset_to_uni() -> [u16; 256] {
    let mut a = [0u16; 256];
    let mut i = 0;
    while i < 256 { a[i] = i as u16; i += 1; }
    a[0xa0] = 0x00a0; a[0xa1] = 0x0401; a[0xa2] = 0x0402;
    a[0xa3] = 0x0403; a[0xa4] = 0x0404; a[0xa5] = 0x0405;
    a[0xa6] = 0x0406; a[0xa7] = 0x0407; a[0xa8] = 0x0408;
    a[0xa9] = 0x0409; a[0xaa] = 0x040a; a[0xab] = 0x040b;
    a[0xac] = 0x040c; a[0xad] = 0x00ad; a[0xae] = 0x040e; a[0xaf] = 0x040f;
    i = 0xb0; while i < 0xf0 { a[i] = (0x0410 + i - 0xb0) as u16; i += 1; }
    a[0xf0] = 0x2116; i = 0xf1; while i < 0xfe { a[i] = (0x0451 + i - 0xf1) as u16; i += 1; }
    a[0xfd] = 0x00a7; a[0xfe] = 0x045e; a[0xff] = 0x045f;
    a
}

const fn lower_table() -> [u8; 256] {
    let mut a = [0u8; 256]; let mut i = 0; while i < 256 { a[i] = i as u8; i += 1; }
    i = 0x41; while i <= 0x5a { a[i] = (i + 0x20) as u8; i += 1; }
    i = 0xc0; while i <= 0xcf { a[i] = (i + 0x10) as u8; i += 1; }
    i = 0xd0; while i <= 0xef { a[i] = i as u8; i += 1; }
    i = 0xa1; while i <= 0xac { a[i] = (i + 0x50) as u8; i += 1; }
    a[0xad] = 0xad; a[0xfd] = 0xfd; a
}

const fn upper_table() -> [u8; 256] {
    let mut a = [0u8; 256]; let mut i = 0; while i < 256 { a[i] = i as u8; i += 1; }
    i = 0x61; while i <= 0x7a { a[i] = (i - 0x20) as u8; i += 1; }
    i = 0xd0; while i <= 0xef { a[i] = (i - 0x20) as u8; i += 1; }
    i = 0xf1; while i <= 0xfc { a[i] = (i - 0x50) as u8; i += 1; }
    a[0xfd] = 0xfd; a
}

static CHARSET2UNI: [u16; 256] = charset_to_uni();
static CHARSET2LOWER: [u8; 256] = lower_table();
static CHARSET2UPPER: [u8; 256] = upper_table();

static PAGE00: [u8; 256] = {
    let mut a = [0u8; 256]; let mut i = 0; while i < 160 { a[i] = i as u8; i += 1; }
    a[0xa0] = 0xa0; a[0xa7] = 0xfd; a[0xad] = 0xad; a
};
static PAGE04: [u8; 256] = {
    let mut a = [0u8; 256]; let mut i = 1; while i <= 0x4f { a[i] = (i + 0xa0) as u8; i += 1; }
    i = 0x51; while i <= 0x5f { a[i] = (i + 0xa0) as u8; i += 1; } a
};
static PAGE21: [u8; 256] = { let mut a = [0u8; 256]; a[0x16] = 0xf0; a };

static PAGE_UNI2CHARSET: [*const u8; 256] = {
    let mut a = [core::ptr::null(); 256]; a[0] = PAGE00.as_ptr(); a[4] = PAGE04.as_ptr(); a[0x21] = PAGE21.as_ptr(); a
};

unsafe fn uni2char(uni: u32, out: *mut u8, boundlen: i32) -> i32 {
    if boundlen <= 0 { return -ENAMETOOLONG; }
    let cl = (uni & 0xff) as usize; let ch = ((uni & 0xff00) >> 8) as usize;
    let page = PAGE_UNI2CHARSET[ch];
    if !page.is_null() && *page.add(cl) != 0 { *out = *page.add(cl); 1 } else { -EINVAL }
}

unsafe fn char2uni(rawstring: *const u8, _boundlen: i32, uni: *mut u32) -> i32 {
    *uni = CHARSET2UNI[*rawstring as usize] as u32;
    if *uni == 0 { -EINVAL } else { 1 }
}

// The following table/module declarations correspond to the kernel nls_table
// and module registration macros supplied by the surrounding translation.
static mut TABLE: nls_table = nls_table {
    charset: "iso8859-5", uni2char, char2uni,
    charset2lower: CHARSET2LOWER.as_ptr(), charset2upper: CHARSET2UPPER.as_ptr(),
};

unsafe fn init_nls_iso8859_5() -> i32 { register_nls(&mut TABLE) }
unsafe fn exit_nls_iso8859_5() { unregister_nls(&mut TABLE); }

// MODULE_DESCRIPTION("NLS ISO 8859-5 (Cyrillic)");
// MODULE_LICENSE("Dual BSD/GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
