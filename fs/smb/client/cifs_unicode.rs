// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct Rust translation of cifs_unicode.c. */

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static NLS_MAX_CHARSET_SIZE: c_int;
    fn nls_nullsize(cp: *const nls_table) -> c_int;
    fn utf16s_to_utf8s(from: *const u16, len: c_int, endian: c_int, to: *mut c_char, max: c_int) -> c_int;
    fn utf8s_to_utf16s(from: *const c_char, len: c_int, endian: c_int, to: *mut u16, max: c_int) -> c_int;
    fn utf8_to_utf32(from: *const c_char, len: c_int, out: *mut u32) -> c_int;
    fn kmalloc(len: usize, flags: usize) -> *mut c_void;
    fn kzalloc(len: usize, flags: usize) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn kstrndup(s: *const c_char, len: usize, flags: usize) -> *mut c_char;
    fn cifs_dbg(kind: c_int, fmt: *const c_char, ...);
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
}

#[repr(C)] pub struct nls_table {
    pub charset: *const c_char,
    pub uni2char: unsafe extern "C" fn(u16, *mut c_char, c_int) -> c_int,
    pub char2uni: unsafe extern "C" fn(*const c_char, c_int, *mut u16) -> c_int,
}

extern "C" {
    fn get_unaligned_le16(p: *const u16) -> u16;
    fn put_unaligned_le16(v: u16, p: *mut u16);
}

unsafe fn convert_sfu_char(src: u16, target: *mut c_char) -> bool {
    let v = match src { UNI_COLON => b':', UNI_ASTERISK => b'*', UNI_QUESTION => b'?', UNI_PIPE => b'|', UNI_GRTRTHAN => b'>', UNI_LESSTHAN => b'<', _ => return false };
    *target = v as c_char; true
}
unsafe fn convert_sfm_char(src: u16, target: *mut c_char) -> bool {
    if (0xf001..=0xf01f).contains(&src) { *target = (src - 0xf000) as c_char; return true; }
    let v = match src { SFM_COLON=>b':', SFM_DOUBLEQUOTE=>b'"', SFM_ASTERISK=>b'*', SFM_QUESTION=>b'?', SFM_PIPE=>b'|', SFM_GRTRTHAN=>b'>', SFM_LESSTHAN=>b'<', SFM_SPACE=>b' ', SFM_PERIOD=>b'.', _=>return false };
    *target=v as c_char; true
}

unsafe fn cifs_mapchar(target: *mut c_char, from: *const u16, cp: *const nls_table, maptype: c_int) -> c_int {
    if maptype == SFM_MAP_UNI_RSVD && convert_sfm_char(*from,target) || maptype == SFU_MAP_UNI_RSVD && convert_sfu_char(*from,target) { return 1; }
    let mut len = ((*cp).uni2char)(*from,target,NLS_MAX_CHARSET_SIZE);
    if len > 0 { return len; }
    if strcmp((*cp).charset, b"utf8\0".as_ptr() as *const c_char) == 0 { len=utf16s_to_utf8s(from,3,UTF16_LITTLE_ENDIAN,target,6); if len>0{return len;} }
    *target=b'?' as c_char; 1
}

#[no_mangle] pub unsafe extern "C" fn cifs_from_utf16(to:*mut c_char, from:*const u16, tolen:c_int, fromlen:c_int, cp:*const nls_table, map_type:c_int)->c_int {
    let nullsize=nls_nullsize(cp); let fromwords=fromlen/2; let safelen=tolen-(NLS_MAX_CHARSET_SIZE+nullsize); let mut outlen=0; let mut tmp=[0i8; 16]; let mut ftmp=[0u16;3];
    for i in 0..fromwords { ftmp[0]=get_unaligned_le16(from.add(i as usize)); if ftmp[0]==0{break;} ftmp[1]=if i+1<fromwords{get_unaligned_le16(from.add(i as usize+1))}else{0}; ftmp[2]=if i+2<fromwords{get_unaligned_le16(from.add(i as usize+2))}else{0}; if outlen>=safelen { let n=cifs_mapchar(tmp.as_mut_ptr(),ftmp.as_ptr(),cp,map_type); if outlen+n>tolen-nullsize{break;} } let n=cifs_mapchar(to.add(outlen as usize),ftmp.as_ptr(),cp,map_type); outlen+=n; }
    for _ in 0..nullsize {*to.add(outlen as usize)=0;outlen+=1;} outlen
}

#[no_mangle] pub unsafe extern "C" fn cifs_strtoUTF16(to:*mut u16, mut from:*const c_char, mut len:c_int, cp:*const nls_table)->c_int { let mut i=0; let mut w=0u16; let mut charlen; if strcmp((*cp).charset,b"utf8\0".as_ptr() as *const c_char)==0 { let n=utf8s_to_utf16s(from,len,UTF16_LITTLE_ENDIAN,to,len); if n>=0 {put_unaligned_le16(0,to.add(n as usize));return n;} } while len>0 && *from!=0 { charlen=((*cp).char2uni)(from,len,&mut w); if charlen<1 {w=0x3f;charlen=1;} put_unaligned_le16(w,to.add(i as usize));i+=1;from=from.add(charlen as usize);len-=charlen;} put_unaligned_le16(0,to.add(i as usize));i }

#[no_mangle] pub unsafe extern "C" fn cifs_utf16_bytes(from:*const u16,maxbytes:c_int,cp:*const nls_table)->c_int { let mut out=0; let mut t=[0i8;16]; let mut f=[0u16;3]; for i in 0..maxbytes/2 {f[0]=get_unaligned_le16(from.add(i as usize));if f[0]==0{break;}f[1]=if i+1<maxbytes/2{get_unaligned_le16(from.add(i as usize+1))}else{0};f[2]=if i+2<maxbytes/2{get_unaligned_le16(from.add(i as usize+2))}else{0};out+=cifs_mapchar(t.as_mut_ptr(),f.as_ptr(),cp,NO_MAP_UNI_RSVD);}out }

#[no_mangle] pub unsafe extern "C" fn cifs_strndup_from_utf16(src:*const c_char,maxlen:c_int,is_unicode:bool,cp:*const nls_table)->*mut c_char { if is_unicode {let len=cifs_utf16_bytes(src as *const u16,maxlen,cp)+nls_nullsize(cp);let dst=kmalloc(len as usize,GFP_KERNEL) as *mut c_char;if dst.is_null(){return core::ptr::null_mut();}cifs_from_utf16(dst,src as *const u16,len,maxlen,cp,NO_MAP_UNI_RSVD);dst}else{kstrndup(src,maxlen as usize,GFP_KERNEL)} }

unsafe fn convert_to_sfu_char(c:c_char)->u16 { match c as u8 {b':'=>UNI_COLON,b'*'=>UNI_ASTERISK,b'?'=>UNI_QUESTION,b'<'=>UNI_LESSTHAN,b'>'=>UNI_GRTRTHAN,b'|'=>UNI_PIPE,_=>0} }
unsafe fn convert_to_sfm_char(c:c_char,end:bool)->u16 { let c=c as u8;if (1..=0x1f).contains(&c){return c as u16+0xf000;} match c {b':'=>SFM_COLON,b'"'=>SFM_DOUBLEQUOTE,b'*'=>SFM_ASTERISK,b'?'=>SFM_QUESTION,b'<'=>SFM_LESSTHAN,b'>'=>SFM_GRTRTHAN,b'|'=>SFM_PIPE,b'.' if end=>SFM_PERIOD,b' ' if end=>SFM_SPACE,_=>0} }

#[no_mangle] pub unsafe extern "C" fn cifsConvertToUTF16(target:*mut u16,source:*const c_char,srclen:c_int,cp:*const nls_table,map_chars:c_int)->c_int { if map_chars==NO_MAP_UNI_RSVD{return cifs_strtoUTF16(target,source,PATH_MAX,cp);} let mut i=0;let mut j=0;let mut tmp=0u16;while i<srclen&&*source.add(i as usize)!=0 {let c=*source.add(i as usize);let mut n=1;let mut d=if map_chars==SFU_MAP_UNI_RSVD{convert_to_sfu_char(c)}else{convert_to_sfm_char(c,i==srclen-1||*source.add((i+1) as usize)==b'\\' as i8)};if d==0{n=((*cp).char2uni)(source.add(i as usize),srclen-i,&mut tmp);if n<1{d=0x3f;n=1;}else{d=tmp;}}i+=n;put_unaligned_le16(d,target.add(j as usize));j+=1;}put_unaligned_le16(0,target.add(j as usize));j}

#[no_mangle] pub unsafe extern "C" fn cifs_strndup_to_utf16(src:*const c_char,maxlen:c_int,outlen:*mut c_int,cp:*const nls_table,remap:c_int)->*mut u16 { if src.is_null(){return core::ptr::null_mut();}let len=2+maxlen*2;let d=kmalloc(len as usize,GFP_KERNEL) as *mut u16;if d.is_null(){*outlen=0;return d;}cifsConvertToUTF16(d,src,libc_strlen(src) as c_int,cp,remap);*outlen=len;d }

extern "C" { fn libc_strlen(s:*const c_char)->usize; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
