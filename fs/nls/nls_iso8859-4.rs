/* linux/fs/nls/nls_iso8859-4.c -- Rust translation. */

static CHARSET2UNI: [u16; 256] = [
    0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,
    32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,
    64,65,66,67,68,69,70,71,72,73,74,75,76,77,78,79,80,81,82,83,84,85,86,87,88,89,90,91,92,93,94,95,
    96,97,98,99,100,101,102,103,104,105,106,107,108,109,110,111,112,113,114,115,116,117,118,119,120,121,122,123,124,125,126,127,
    128,129,130,131,132,133,134,135,136,137,138,139,140,141,142,143,144,145,146,147,148,149,150,151,152,153,154,155,156,157,158,159,
    0xa0,0x104,0x138,0x156,0xa4,0x128,0x13b,0xa7,0xa8,0x160,0x112,0x122,0x166,0xad,0x17d,0xaf,
    0xb0,0x105,0x2db,0x157,0xb4,0x129,0x13c,0x2c7,0xb8,0x161,0x113,0x123,0x167,0x14a,0x17e,0x14b,
    0x100,0xc1,0xc2,0xc3,0xc4,0xc5,0xc6,0x12e,0x10c,0xc9,0x118,0xcb,0x116,0xcd,0xce,0x12a,
    0x110,0x145,0x14c,0x136,0xd4,0xd5,0xd6,0xd7,0xd8,0x172,0xda,0xdb,0xdc,0x168,0x16a,0xdf,
    0x101,0xe1,0xe2,0xe3,0xe4,0xe5,0xe6,0x12f,0x10d,0xe9,0x119,0xeb,0x117,0xed,0xee,0x12b,
    0x111,0x146,0x14d,0x137,0xf4,0xf5,0xf6,0xf7,0xf8,0x173,0xfa,0xfb,0xfc,0x169,0x16b,0x2d9,
];

static CHARSET2LOWER: [u8; 256] = { let mut a=[0u8;256]; let mut i=0; while i<256 { a[i]=i as u8; i+=1; } a };
static CHARSET2UPPER: [u8; 256] = CHARSET2LOWER;

extern "C" { fn register_nls(table: *mut NlsTable) -> i32; fn unregister_nls(table: *mut NlsTable); }
#[repr(C)] pub struct NlsTable { pub charset: *const u8, pub uni2char: Option<unsafe extern "C" fn(u16,*mut u8,i32)->i32>, pub char2uni: Option<unsafe extern "C" fn(*const u8,i32,*mut u16)->i32>, pub charset2lower: *const u8, pub charset2upper: *const u8 }
unsafe extern "C" fn uni2char(uni:u16,out:*mut u8,boundlen:i32)->i32 { if boundlen<=0{return -36} ; let mut i=0; while i<256 { if CHARSET2UNI[i]==uni { *out=i as u8; return 1; } i+=1; } -22 }
unsafe extern "C" fn char2uni(raw:*const u8,_boundlen:i32,uni:*mut u16)->i32 { *uni=CHARSET2UNI[*raw as usize]; if *uni==0{-22}else{1} }
static mut TABLE: NlsTable = NlsTable { charset: b"iso8859-4\0".as_ptr(), uni2char:Some(uni2char), char2uni:Some(char2uni), charset2lower:CHARSET2LOWER.as_ptr(), charset2upper:CHARSET2UPPER.as_ptr() };
unsafe extern "C" fn init_nls_iso8859_4()->i32 { register_nls(&raw mut TABLE) }
unsafe extern "C" fn exit_nls_iso8859_4(){ unregister_nls(&raw mut TABLE); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
