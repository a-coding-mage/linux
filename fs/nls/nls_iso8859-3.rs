/* Charset iso8859-3 translation tables. */

const fn charset2uni_init() -> [u32; 256] {
    let mut a = [0u32; 256]; let mut i = 0;
    while i < 128 { a[i] = i as u32; i += 1; }
    let v = [0xa0,0x0126,0x02d8,0xa3,0xa4,0,0x0124,0xa7,0xa8,0x0130,0x015e,0x011e,0x0134,0xad,0,0x017b,0xb0,0x0127,0xb2,0xb3,0xb4,0xb5,0x0125,0xb7,0xb8,0x0131,0x015f,0x011f,0x0135,0xbd,0,0x017c,0xc0,0xc1,0xc2,0,0xc4,0x010a,0x0108,0xc7,0xc8,0xc9,0xca,0xcb,0xcc,0xcd,0xce,0xcf,0,0xd1,0xd2,0xd3,0xd4,0x0120,0xd6,0xd7,0x011c,0xd9,0xda,0xdb,0xdc,0x016c,0x015c,0xdf,0xe0,0xe1,0xe2,0,0xe4,0x010b,0x0109,0xe7,0xe8,0xe9,0xea,0xeb,0xec,0xed,0xee,0xef,0,0xf1,0xf2,0xf3,0xf4,0x0121,0xf6,0xf7,0x011d,0xf9,0xfa,0xfb,0xfc,0x016d,0x015d,0x02d9];
    i = 0; while i < 96 { a[160+i] = v[i]; i += 1; } a
}
static charset2uni: [u32; 256] = charset2uni_init();

const fn lower_init() -> [u8; 256] { let mut a=[0u8;256]; let mut i=0; while i<256 {a[i]=i as u8;i+=1;} i=0x41; while i<=0x5a {a[i]=(i+0x20) as u8;i+=1;} a[0xa1]=0xb1;a[0xa9]=0xb9;a[0xaa]=0xba;a[0xac]=0xbc;a[0xc5]=0xe5;a[0xc6]=0xe6;a[0xc8]=0xe8;a[0xd5]=0xf5;a[0xd8]=0xf8;a[0xde]=0xfe;a[0xab]=0xbb;a[0xae]=0; a }
const fn upper_init() -> [u8; 256] { let mut a=[0u8;256]; let mut i=0; while i<256 {a[i]=i as u8;i+=1;} i=0x61; while i<=0x7a {a[i]=(i-0x20) as u8;i+=1;} a[0xb1]=0xa1;a[0xb9]=0xa9;a[0xba]=0xaa;a[0xbc]=0xac;a[0xe5]=0xc5;a[0xe6]=0xc6;a[0xe8]=0xc8;a[0xf5]=0xd5;a[0xf8]=0xd8;a[0xfe]=0xde;a[0xbb]=0xab;a[0xbe]=0; a }
static charset2lower: [u8; 256] = lower_init();
static charset2upper: [u8; 256] = upper_init();

static page00: [u8; 256] = charset_page00();
const fn charset_page00() -> [u8;256] { let mut a=[0u8;256]; let mut i=0; while i<256 {a[i]=i as u8;i+=1;} a }
static page01: [u8; 256] = [0;256];
static page02: [u8; 256] = [0;256];
static page_uni2charset: [*const u8; 256] = [core::ptr::null(); 256];

extern "C" { fn register_nls(table: *mut NlsTable) -> i32; fn unregister_nls(table: *mut NlsTable); }
#[repr(C)] struct NlsTable { charset: *const u8, uni2char: unsafe extern "C" fn(u32,*mut u8,i32)->i32, char2uni: unsafe extern "C" fn(*const u8,i32,*mut u32)->i32, charset2lower: *const u8, charset2upper: *const u8 }
extern "C" { static ENAMETOOLONG: i32; static EINVAL: i32; }

unsafe extern "C" fn uni2char(uni:u32,out:*mut u8,boundlen:i32)->i32 { if boundlen<=0{return -ENAMETOOLONG;} let cl=(uni&0xff) as usize; let ch=((uni&0xff00)>>8) as usize; if ch==0 { *out=page00[cl]; return 1; } -EINVAL }
unsafe extern "C" fn char2uni(raw:*const u8,_boundlen:i32,uni:*mut u32)->i32 { *uni=charset2uni[*raw as usize]; if *uni==0{return -EINVAL;} 1 }
static mut table: NlsTable=NlsTable { charset:b"iso8859-3\0".as_ptr(),uni2char,char2uni,charset2lower:charset2lower.as_ptr(),charset2upper:charset2upper.as_ptr() };
unsafe extern "C" fn init_nls_iso8859_3()->i32 { register_nls(&mut table) }
unsafe extern "C" fn exit_nls_iso8859_3(){unregister_nls(&mut table)}
/* module_init(init_nls_iso8859_3); module_exit(exit_nls_iso8859_3); */
/* MODULE_DESCRIPTION("NLS ISO 8859-3 (Latin 3; Esperanto, Galician, Maltese, Turkish)"); */
/* MODULE_LICENSE("Dual BSD/GPL"); */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
