/*
 * linux/fs/nls/nls_cp1251.c
 *
 * Charset cp1251 translation tables.
 * Generated automatically from the Unicode and charset tables from the
 * Unicode Organization (www.unicode.org).
 */

// Linux kernel headers and registration APIs are supplied by the surrounding
// translation unit.

const fn charset2uni_table() -> [u16; 256] {
    let mut a = [0u16; 256];
    let mut i = 0usize;
    while i < 128 { a[i] = i as u16; i += 1; }
    let v: [u16; 128] = [
        0x0402,0x0403,0x201a,0x0453,0x201e,0x2026,0x2020,0x2021,0x20ac,0x2030,0x0409,0x2039,0x040a,0x040c,0x040b,0x040f,
        0x0452,0x2018,0x2019,0x201c,0x201d,0x2022,0x2013,0x2014,0x0000,0x2122,0x0459,0x203a,0x045a,0x045c,0x045b,0x045f,
        0x00a0,0x040e,0x045e,0x0408,0x00a4,0x0490,0x00a6,0x00a7,0x0401,0x00a9,0x0404,0x00ab,0x00ac,0x00ad,0x00ae,0x0407,
        0x00b0,0x00b1,0x0406,0x0456,0x0491,0x00b5,0x00b6,0x00b7,0x0451,0x2116,0x0454,0x00bb,0x0458,0x0405,0x0455,0x0457,
        0x0410,0x0411,0x0412,0x0413,0x0414,0x0415,0x0416,0x0417,0x0418,0x0419,0x041a,0x041b,0x041c,0x041d,0x041e,0x041f,
        0x0420,0x0421,0x0422,0x0423,0x0424,0x0425,0x0426,0x0427,0x0428,0x0429,0x042a,0x042b,0x042c,0x042d,0x042e,0x042f,
        0x0430,0x0431,0x0432,0x0433,0x0434,0x0435,0x0436,0x0437,0x0438,0x0439,0x043a,0x043b,0x043c,0x043d,0x043e,0x043f,
        0x0440,0x0441,0x0442,0x0443,0x0444,0x0445,0x0446,0x0447,0x0448,0x0449,0x044a,0x044b,0x044c,0x044d,0x044e,0x044f,
    ];
    i = 0; while i < 128 { a[i + 128] = v[i]; i += 1; } a
}

static CHARSET2UNI: [u16; 256] = charset2uni_table();

const fn case_table(upper: bool) -> [u8; 256] {
    let mut a = [0u8; 256]; let mut i = 0usize;
    while i < 256 { a[i] = i as u8; i += 1; }
    i = 0x61; while i <= 0x7a { a[i] = if upper { (i - 0x20) as u8 } else { i as u8 }; i += 1; }
    i = 0xc0; while i <= 0xff { a[i] = if upper { i as u8 } else { (i + 0x20) as u8 }; i += 1; }
    a
}
static CHARSET2LOWER: [u8; 256] = case_table(false);
static CHARSET2UPPER: [u8; 256] = case_table(true);

// Unicode-to-charset pages. Unlisted code points are represented by zero.
static PAGE00: [u8; 256] = {
    let mut a = [0u8; 256]; let mut i=0usize; while i<128 { a[i]=i as u8; i+=1; }
    a[0xa0]=0xa0; a[0xa4]=0xa4; a[0xa6]=0xa6; a[0xa7]=0xa7; a[0xa9]=0xa9; a[0xab]=0xab; a[0xac]=0xac; a[0xad]=0xad; a[0xae]=0xae; a[0xb0]=0xb0; a[0xb1]=0xb1; a[0xb5]=0xb5; a[0xb6]=0xb6; a[0xb7]=0xb7; a[0xbb]=0xbb; a
};

fn uni2char(uni: u32, out: *mut u8, boundlen: i32) -> i32 {
    if boundlen <= 0 { return -36; }
    let b = ((uni >> 8) & 0xff) as u8; let c = (uni & 0xff) as usize;
    let value = if b == 0 { PAGE00[c] } else { 0 };
    if value == 0 { return -22; }
    unsafe { *out = value; } 1
}

fn char2uni(rawstring: *const u8, _boundlen: i32, uni: *mut u32) -> i32 {
    unsafe { *uni = CHARSET2UNI[*rawstring as usize] as u32; if *uni == 0 { return -22; } } 1
}

// The surrounding kernel/Rust integration supplies nls_table and module
// registration declarations corresponding to these C interfaces.
extern "C" {
    fn register_nls(table: *mut NlsTable) -> i32;
    fn unregister_nls(table: *mut NlsTable);
}

#[repr(C)]
struct NlsTable {
    charset: *const u8,
    uni2char: unsafe extern "C" fn(u32, *mut u8, i32) -> i32,
    char2uni: unsafe extern "C" fn(*const u8, i32, *mut u32) -> i32,
    charset2lower: *const u8,
    charset2upper: *const u8,
}

static mut TABLE: NlsTable = NlsTable { charset: b"cp1251\0".as_ptr(), uni2char: uni2char, char2uni: char2uni, charset2lower: CHARSET2LOWER.as_ptr(), charset2upper: CHARSET2UPPER.as_ptr() };

unsafe fn init_nls_cp1251() -> i32 { register_nls(&raw mut TABLE) }
unsafe fn exit_nls_cp1251() { unregister_nls(&raw mut TABLE); }

// module_init(init_nls_cp1251)
// module_exit(exit_nls_cp1251)
// MODULE_DESCRIPTION("NLS Windows CP1251 (Bulgarian, Belarusian)")
// MODULE_LICENSE("Dual BSD/GPL")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
