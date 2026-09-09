// SPDX-License-Identifier: GPL-2.0-or-later
/* Parse a signed PE binary */

use core::{ffi::c_void, mem, ptr};

// Types, constants, logging, allocation, and crypto/key functions are supplied
// by the surrounding kernel translation.

unsafe fn pefile_parse_binary(pebuf: *const c_void, pelen: u32,
                              ctx: *mut pefile_context) -> i32 {
    let mz = pebuf as *const mz_hdr;
    let mut cursor: usize;
    let datalen = pelen as usize;

    macro_rules! chkaddr {
        ($base:expr, $x:expr, $s:expr) => {
            if ($x) < ($base) || ($s) >= datalen || ($x) > datalen - ($s) {
                return -ELIBBAD;
            }
        };
    }

    chkaddr!(0usize, 0usize, mem::size_of::<mz_hdr>());
    if (*mz).magic != IMAGE_DOS_SIGNATURE { return -ELIBBAD; }
    cursor = mem::size_of::<mz_hdr>();

    chkaddr!(cursor, (*mz).peaddr as usize, mem::size_of::<pe_hdr>());
    let pe = (pebuf as *const u8).add((*mz).peaddr as usize) as *const pe_hdr;
    if (*pe).magic != IMAGE_NT_SIGNATURE { return -ELIBBAD; }
    cursor = (*mz).peaddr as usize + mem::size_of::<pe_hdr>();

    chkaddr!(0usize, cursor, mem::size_of::<u16>());
    let pe32 = (pebuf as *const u8).add(cursor) as *const pe32_opt_hdr;
    let pe64 = pe32 as *const pe32plus_opt_hdr;
    match (*pe32).magic {
        IMAGE_NT_OPTIONAL_HDR32_MAGIC => {
            chkaddr!(0usize, cursor, mem::size_of::<pe32_opt_hdr>());
            (*ctx).image_checksum_offset = (&(*pe32).csum as *const _ as usize) - pebuf as usize;
            (*ctx).header_size = (*pe32).header_size;
            cursor += mem::size_of::<pe32_opt_hdr>();
            (*ctx).n_data_dirents = (*pe32).data_dirs;
        }
        IMAGE_NT_OPTIONAL_HDR64_MAGIC => {
            chkaddr!(0usize, cursor, mem::size_of::<pe32plus_opt_hdr>());
            (*ctx).image_checksum_offset = (&(*pe64).csum as *const _ as usize) - pebuf as usize;
            (*ctx).header_size = (*pe64).header_size;
            cursor += mem::size_of::<pe32plus_opt_hdr>();
            (*ctx).n_data_dirents = (*pe64).data_dirs;
        }
        _ => return -ELIBBAD,
    }
    if cursor >= (*ctx).header_size as usize || (*ctx).header_size as usize >= datalen { return -ELIBBAD; }
    if (*ctx).n_data_dirents as usize > ((*ctx).header_size as usize - cursor) / mem::size_of::<data_dirent>() { return -ELIBBAD; }
    let ddir = (pebuf as *const u8).add(cursor) as *const data_directory;
    cursor += mem::size_of::<data_dirent>() * (*ctx).n_data_dirents as usize;
    (*ctx).cert_dirent_offset = (&(*ddir).certs as *const _ as usize) - pebuf as usize;
    (*ctx).certs_size = (*ddir).certs.size;
    if (*ddir).certs.virtual_address == 0 || (*ddir).certs.size == 0 { return -ENODATA; }
    chkaddr!((*ctx).header_size as usize, (*ddir).certs.virtual_address as usize, (*ddir).certs.size as usize);
    (*ctx).sig_offset = (*ddir).certs.virtual_address;
    (*ctx).sig_len = (*ddir).certs.size;
    (*ctx).n_sections = (*pe).sections;
    if (*ctx).n_sections as usize > ((*ctx).header_size as usize - cursor) / mem::size_of::<section_header>() { return -ELIBBAD; }
    (*ctx).secs = (pebuf as *const u8).add(cursor) as *const section_header;
    0
}

unsafe fn pefile_strip_sig_wrapper(pebuf: *const c_void, ctx: *mut pefile_context) -> i32 {
    if (*ctx).sig_len < mem::size_of::<win_certificate>() as u32 { return -ELIBBAD; }
    let wrapper = ptr::read_unaligned((pebuf as *const u8).add((*ctx).sig_offset as usize) as *const win_certificate);
    if wrapper.length > (*ctx).sig_len { return -ELIBBAD; }
    if wrapper.revision != WIN_CERT_REVISION_2_0 || wrapper.cert_type != WIN_CERT_TYPE_PKCS_SIGNED_DATA { return -ENOTSUPP; }
    (*ctx).sig_len = wrapper.length - mem::size_of::<win_certificate>() as u32;
    (*ctx).sig_offset += mem::size_of::<win_certificate>() as u32;
    if (*ctx).sig_len < 4 { return -EKEYREJECTED; }
    let p = (pebuf as *const u8).add((*ctx).sig_offset as usize);
    if *p != (ASN1_CONS_BIT | ASN1_SEQ) { return -ELIBBAD; }
    let len = match *p.add(1) {
        n @ 0..=0x7f => n as u32 + 2,
        ASN1_INDEFINITE_LENGTH => return 0,
        0x81 => *p.add(2) as u32 + 3,
        0x82 => ((*p.add(2) as u32) << 8 | *p.add(3) as u32) + 4,
        0x83..=0xff => return -EMSGSIZE,
        _ => return -ELIBBAD,
    };
    if len <= (*ctx).sig_len { (*ctx).sig_len = len; return 0; }
    -ELIBBAD
}

unsafe fn pefile_compare_shdrs(a: *const c_void, b: *const c_void) -> i32 {
    let x = &*(a as *const section_header); let y = &*(b as *const section_header);
    if x.data_addr != y.data_addr { return if x.data_addr > y.data_addr { 1 } else { -1 }; }
    if x.virtual_address != y.virtual_address { return if x.virtual_address > y.virtual_address { 1 } else { -1 }; }
    let rc = strcmp(x.name.as_ptr(), y.name.as_ptr()); if rc != 0 { return rc; }
    if x.virtual_size != y.virtual_size { return if x.virtual_size > y.virtual_size { 1 } else { -1 }; }
    if x.raw_data_size != y.raw_data_size { return if x.raw_data_size > y.raw_data_size { 1 } else { -1 }; }
    0
}

unsafe extern "C" { fn strcmp(a: *const u8, b: *const u8) -> i32; }

unsafe fn pefile_digest_pe_contents(pebuf: *const c_void, pelen: u32, ctx: *mut pefile_context, desc: *mut shash_desc) -> i32 {
    let mut canon = vec![0usize; (*ctx).n_sections as usize];
    let mut tmp = (*ctx).image_checksum_offset + 4;
    let mut ret = crypto_shash_update(desc, pebuf, (*ctx).image_checksum_offset);
    if ret < 0 { return ret; }
    ret = crypto_shash_update(desc, (pebuf as *const u8).add(tmp), (*ctx).cert_dirent_offset - tmp); if ret < 0 { return ret; }
    tmp = (*ctx).cert_dirent_offset + mem::size_of::<data_dirent>();
    ret = crypto_shash_update(desc, (pebuf as *const u8).add(tmp), (*ctx).header_size as usize - tmp); if ret < 0 { return ret; }
    if !canon.is_empty() { canon[0] = 0; }
    for loop_ in 1..(*ctx).n_sections as usize {
        let mut i = 0; while i < loop_ {
            if pefile_compare_shdrs(&*(*ctx).secs.add(canon[i]) as *const _ as *const c_void, &*(*ctx).secs.add(loop_) as *const _ as *const c_void) > 0 {
                canon[i + 1..=loop_].rotate_right(1); break;
            } i += 1;
        } canon[i] = loop_;
    }
    let mut hashed = (*ctx).header_size as usize;
    for &i in &canon { let sec = &*(*ctx).secs.add(i); if sec.raw_data_size == 0 { continue; } ret = crypto_shash_update(desc, (pebuf as *const u8).add(sec.data_addr as usize), sec.raw_data_size as usize); if ret < 0 { return ret; } hashed += sec.raw_data_size as usize; }
    if pelen as usize > hashed { tmp = hashed + (*ctx).certs_size as usize; if tmp <= hashed || pelen as usize < tmp { return -ELIBBAD; } ret = crypto_shash_update(desc, (pebuf as *const u8).add(hashed), pelen as usize - tmp); }
    ret
}

unsafe fn pefile_digest_pe(pebuf: *const c_void, pelen: u32, ctx: *mut pefile_context) -> i32 {
    let tfm = crypto_alloc_shash((*ctx).digest_algo, 0, 0); if IS_ERR(tfm) { return PTR_ERR(tfm); }
    let dsize = crypto_shash_descsize(tfm) + mem::size_of::<shash_desc>(); let hsize = crypto_shash_digestsize(tfm);
    if hsize != (*ctx).digest_len as usize { crypto_free_shash(tfm); return -EBADMSG; }
    let desc = kzalloc(dsize + hsize); if desc.is_null() { crypto_free_shash(tfm); return -ENOMEM; }
    (*desc).tfm = tfm; let mut ret = crypto_shash_init(desc); if ret >= 0 { ret = pefile_digest_pe_contents(pebuf, pelen, ctx, desc); }
    if ret >= 0 { ret = crypto_shash_final(desc, (desc as *mut u8).add(dsize) as *mut c_void); }
    kfree_sensitive(desc); crypto_free_shash(tfm); ret
}

pub unsafe fn verify_pefile_signature(pebuf: *const c_void, pelen: u32, trusted_keys: *mut key, usage: key_being_used_for) -> i32 {
    let mut ctx: pefile_context = mem::zeroed();
    let mut ret = pefile_parse_binary(pebuf, pelen, &mut ctx); if ret < 0 { return ret; }
    ret = pefile_strip_sig_wrapper(pebuf, &mut ctx); if ret < 0 { return ret; }
    ret = verify_pkcs7_signature(ptr::null(), 0, (pebuf as *const u8).add(ctx.sig_offset as usize), ctx.sig_len as usize, trusted_keys, usage, mscode_parse, &mut ctx as *mut _);
    if ret >= 0 { ret = pefile_digest_pe(pebuf, pelen, &mut ctx); }
    kfree_sensitive(ctx.digest); ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
