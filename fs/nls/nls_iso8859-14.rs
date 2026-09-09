/*
 * linux/fs/nls/nls_iso8859-14.c
 * Charset iso8859-14 translation tables.
 * Generated automatically from the Unicode and charset table provided by the Unicode Organisation.
 */

// The Linux kernel headers and NLS registration symbols are supplied by the surrounding translation.

const fn make_charset2uni() -> [u16; 256] {
    let mut a = [0u16; 256];
    let mut i = 0;
    while i < 128 { a[i] = i as u16; i += 1; }
    let v: [u16; 128] = [
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0x00a0,0x1e02,0x1e03,0x00a3,0x010a,0x010b,0x1e0a,0x00a7,0x1e80,0x00a9,0x1e82,0x1e0b,0x1ef2,0x00ad,0x00ae,0x0178,
        0x1e1e,0x1e1f,0x0120,0x0121,0x1e40,0x1e41,0x00b6,0x1e56,0x1e81,0x1e57,0x1e83,0x1e60,0x1ef3,0x1e84,0x1e85,0x1e61,
        0x00c0,0x00c1,0x00c2,0x00c3,0x00c4,0x00c5,0x00c6,0x00c7,0x00c8,0x00c9,0x00ca,0x00cb,0x00cc,0x00cd,0x00ce,0x00cf,
        0x0174,0x00d1,0x00d2,0x00d3,0x00d4,0x00d5,0x00d6,0x1e6a,0x00d8,0x00d9,0x00da,0x00db,0x00dc,0x00dd,0x0176,0x00df,
        0x00e0,0x00e1,0x00e2,0x00e3,0x00e4,0x00e5,0x00e6,0x00e7,0x00e8,0x00e9,0x00ea,0x00eb,0x00ec,0x00ed,0x00ee,0x00ef,
        0x0175,0x00f1,0x00f2,0x00f3,0x00f4,0x00f5,0x00f6,0x1e6b,0x00f8,0x00f9,0x00fa,0x00fb,0x00fc,0x00fd,0x0177,0x00ff,
    ];
    let mut j=0; while j<128 { a[j+128]=v[j]; j+=1; } a
}
const charset2uni: [u16; 256] = make_charset2uni();

const fn make_page(which: u8) -> [u8; 256] {
    let mut a=[0u8;256]; let mut i=0;
    if which==0 { while i<160 { a[i]=i as u8; i+=1; } while i<256 { a[i]=i as u8; i+=1; } }
    else {
        let pairs: &[(usize,u8)] = &[(2,0xa1),(3,0xa2),(10,0xa6),(11,0xab),(30,0xb0),(31,0xb1),(32,0xb2),(33,0xb3),(64,0xb4),(65,0xb5),(86,0xb7),(87,0xb9),(96,0xbb),(97,0xbf),(106,0xd7),(107,0xf7),(116,0xd0),(117,0xf0),(118,0xde),(119,0xfe),(120,0xaf),(128,0xa8),(129,0xb8),(130,0xaa),(131,0xba),(132,0xbd),(133,0xbe),(241,0xac),(242,0xbc)];
        let mut j=0; while j<pairs.len() { a[pairs[j].0]=pairs[j].1; j+=1; }
    } a
}
const page00: [u8;256] = make_page(0);
const page01: [u8;256] = make_page(1);
const page1e: [u8;256] = make_page(1);
const page_uni2charset: [Option<&'static [u8;256]>;256] = {
    let mut a=[None;256]; a[0]=Some(&page00); a[1]=Some(&page01); a[0x1e]=Some(&page1e); a
};

const fn make_lower() -> [u8;256] {
    let mut a=[0u8;256]; let mut i=0; while i<256 { a[i]=i as u8; i+=1; }
    let mut j=0; while j<26 { a[0x41+j]=0x61+j as u8; a[0xc0+j]=0xe0+j as u8; a[0xd0+j]=0xf0+j as u8; j+=1; }
    a[0xa1]=0xa2;a[0xa2]=0xa2;a[0xa4]=0xab;a[0xa5]=0xab;a[0xa6]=0xab;a[0xa8]=0xb8;a[0xaa]=0xba;a[0xab]=0xab;a[0xac]=0xbc;a[0xaf]=0xff;
    a[0xb0]=0xb1;a[0xb2]=0xb3;a[0xb4]=0xb5;a[0xb7]=0xb9;a[0xb8]=0xb8;a[0xb9]=0xb9;a[0xba]=0xba;a[0xbb]=0xbf;a[0xbc]=0xbc;a[0xbd]=0xbe;a[0xbe]=0xbe;a[0xbf]=0xbf;a[0xdf]=0xdf;a[0xff]=0xff;a
}
const fn make_upper() -> [u8;256] {
    let mut a=[0u8;256]; let mut i=0; while i<256 { a[i]=i as u8; i+=1; }
    let mut j=0; while j<26 { a[0x61+j]=0x41+j as u8; a[0xe0+j]=0xc0+j as u8; a[0xf0+j]=0xd0+j as u8; j+=1; }
    a[0xa1]=0xa1;a[0xa2]=0xa1;a[0xa4]=0xa6;a[0xa5]=0xa6;a[0xa6]=0xa6;a[0xab]=0xa6;a[0xb7]=0xb7;a[0xb8]=0xa8;a[0xba]=0xaa;a[0xbb]=0xbb;a[0xbc]=0xac;a[0xbe]=0xbd;a[0xbf]=0xbb;a[0xff]=0xaf;a
}
const charset2lower: [u8;256] = make_lower();
const charset2upper: [u8;256] = make_upper();

unsafe fn uni2char(uni: u16, out: *mut u8, boundlen: i32) -> i32 {
    if boundlen <= 0 { return -36; }
    let p = page_uni2charset[(uni >> 8) as usize];
    match p { Some(table) if table[(uni & 0xff) as usize] != 0 => { *out = table[(uni & 0xff) as usize]; 1 }, _ => -22 }
}
unsafe fn char2uni(rawstring: *const u8, _boundlen: i32, uni: *mut u16) -> i32 {
    *uni = charset2uni[*rawstring as usize]; if *uni == 0 { -22 } else { 1 }
}

#[repr(C)]
struct nls_table {
    charset: *const u8,
    uni2char: unsafe fn(u16,*mut u8,i32)->i32,
    char2uni: unsafe fn(*const u8,i32,*mut u16)->i32,
    charset2lower: *const u8,
    charset2upper: *const u8,
}
static mut table: nls_table = nls_table { charset: b"iso8859-14\0".as_ptr(), uni2char, char2uni, charset2lower: charset2lower.as_ptr(), charset2upper: charset2upper.as_ptr() };

unsafe fn init_nls_iso8859_14() -> i32 { register_nls(&mut table) }
unsafe fn exit_nls_iso8859_14() { unregister_nls(&mut table); }
extern "C" { fn register_nls(table: *mut nls_table) -> i32; fn unregister_nls(table: *mut nls_table); }

// module_init(init_nls_iso8859_14)
// module_exit(exit_nls_iso8859_14)
// MODULE_DESCRIPTION("NLS ISO 8859-14 (Latin 8; Celtic)")
// MODULE_LICENSE("Dual BSD/GPL")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
