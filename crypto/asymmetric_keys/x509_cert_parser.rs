// SPDX-License-Identifier: GPL-2.0-or-later
/* X.509 certificate parser */

use core::{ffi::c_void, ptr};

#[repr(C)]
pub struct X509ParseContext {
    pub cert: *mut x509_certificate, pub data: usize, pub key: *const c_void,
    pub key_size: usize, pub params: *const c_void, pub params_size: usize,
    pub key_algo: OID, pub last_oid: OID, pub sig_algo: OID,
    pub o_size: u8, pub cn_size: u8, pub email_size: u8,
    pub o_offset: u16, pub cn_offset: u16, pub email_offset: u16,
    pub raw_akid_size: u32, pub raw_akid: *const c_void,
    pub akid_raw_issuer: *const c_void, pub akid_raw_issuer_size: u32,
}

extern "C" {
    pub fn public_key_free(p: *mut public_key); pub fn public_key_signature_free(p: *mut public_key_signature);
    pub fn kfree(p: *mut c_void); pub fn asn1_ber_decoder(decoder: *const c_void, ctx: *mut c_void, data: *const c_void, len: usize) -> isize;
    pub fn kmemdup(p: *const c_void, n: usize, gfp: u32) -> *mut c_void; pub fn kzalloc(n: usize, gfp: u32) -> *mut c_void;
    pub fn kmalloc(n: usize, gfp: u32) -> *mut c_void; pub fn asymmetric_key_generate_id(a:*const c_void, an:usize,b:*const c_void,bn:usize)->*mut asymmetric_key_id;
    pub fn x509_get_sig_params(c:*mut x509_certificate)->isize; pub fn x509_check_for_self_signed(c:*mut x509_certificate)->isize;
    pub fn look_up_OID(v:*const c_void,n:usize)->OID; pub fn sprint_oid(v:*const c_void,n:usize,b:*mut u8,s:usize);
    pub fn parse_OID(v:*const c_void,n:usize,o:*mut OID)->isize; pub fn mktime64(y:u32,m:u32,d:u32,h:u32,mi:u32,s:u32)->i64;
}

#[repr(C)] pub struct x509_certificate { pub pub_: *mut public_key, pub sig:*mut public_key_signature, pub issuer:*mut i8, pub subject:*mut i8, pub id:*mut asymmetric_key_id, pub skid:*mut asymmetric_key_id, pub raw_serial:*const c_void, pub raw_serial_size:usize, pub raw_issuer:*const c_void, pub raw_issuer_size:usize, pub raw_subject:*const c_void, pub raw_subject_size:usize, pub raw_sig:*const c_void, pub raw_sig_size:usize, pub raw_skid:*const c_void, pub raw_skid_size:usize, pub tbs:*const c_void, pub tbs_size:usize, pub valid_from:i64, pub valid_to:i64 }
#[repr(C)] pub struct public_key { pub key:*mut c_void, pub keylen:usize, pub params:*mut c_void, pub paramlen:usize, pub algo:*const i8, pub pkey_algo:*const i8, pub key_eflags:u32 }
#[repr(C)] pub struct public_key_signature { pub hash_algo:*const i8, pub pkey_algo:*const i8, pub encoding:*const i8, pub algo_takes_data:bool, pub auth_ids:[*mut asymmetric_key_id;3] }
#[repr(C)] pub struct asymmetric_key_id { pub len:usize, pub data:*const u8 }
pub type OID = u32; pub type time64_t=i64;

pub unsafe fn x509_free_certificate(cert:*mut x509_certificate) { if !cert.is_null() { public_key_free((*cert).pub_); public_key_signature_free((*cert).sig); kfree((*cert).issuer.cast()); kfree((*cert).subject.cast()); kfree((*cert).id.cast()); kfree((*cert).skid.cast()); kfree(cert.cast()); } }

pub unsafe fn x509_cert_parse(data:*const c_void, datalen:usize)->*mut x509_certificate {
    let cert=kzalloc(core::mem::size_of::<x509_certificate>(),0) as *mut x509_certificate; if cert.is_null(){return ptr::null_mut()}
    (*cert).pub_=kzalloc(core::mem::size_of::<public_key>(),0) as *mut public_key; if (*cert).pub_.is_null(){return ptr::null_mut()}
    (*cert).sig=kzalloc(core::mem::size_of::<public_key_signature>(),0) as *mut public_key_signature; if (*cert).sig.is_null(){return ptr::null_mut()}
    let ctx=kzalloc(core::mem::size_of::<X509ParseContext>(),0) as *mut X509ParseContext; if ctx.is_null(){return ptr::null_mut()}
    (*ctx).cert=cert; (*ctx).data=data as usize;
    let mut ret=asn1_ber_decoder(ptr::null(),ctx.cast(),data,datalen); if ret<0{return ptr::null_mut()}
    if !(*ctx).raw_akid.is_null(){ ret=asn1_ber_decoder(ptr::null(),ctx.cast(),(*ctx).raw_akid,(*ctx).raw_akid_size as usize); if ret<0{return ptr::null_mut()} }
    (*cert).pub_.as_mut().unwrap().key=kmemdup((*ctx).key,(*ctx).key_size,0); (*cert).pub_.as_mut().unwrap().keylen=(*ctx).key_size;
    (*cert).pub_.as_mut().unwrap().params=kmemdup((*ctx).params,(*ctx).params_size,0); (*cert).pub_.as_mut().unwrap().paramlen=(*ctx).params_size; (*cert).pub_.as_mut().unwrap().algo=(*ctx).key_algo as *const i8;
    if x509_get_sig_params(cert)<0{return ptr::null_mut()}; let kid=asymmetric_key_generate_id((*cert).raw_serial,(*cert).raw_serial_size,(*cert).raw_issuer,(*cert).raw_issuer_size); (*cert).id=kid; if x509_check_for_self_signed(cert)<0{return ptr::null_mut()}; cert
}

pub unsafe fn x509_note_OID(context:*mut c_void,_:usize,_:u8,value:*const c_void,vlen:usize)->i32 { let c=&mut *(context as *mut X509ParseContext); c.last_oid=look_up_OID(value,vlen); 0 }
pub unsafe fn x509_note_tbs_certificate(context:*mut c_void,hdrlen:usize,_:u8,value:*const c_void,vlen:usize)->i32 { let c=&mut *(context as *mut X509ParseContext); (*c.cert).tbs=value.sub(hdrlen); (*c.cert).tbs_size=vlen+hdrlen; 0 }
pub unsafe fn x509_note_sig_algo(context:*mut c_void,_:usize,_:u8,_:*const c_void,_:usize)->i32 { let c=&mut *(context as *mut X509ParseContext); (*c.cert).sig.as_mut().unwrap().pkey_algo=c.last_oid as *const i8; c.sig_algo=c.last_oid; 0 }
pub unsafe fn x509_note_signature(context:*mut c_void,_:usize,_:u8,value:*const c_void,vlen:usize)->i32 { let c=&mut *(context as *mut X509ParseContext); if vlen<1{return -74}; (*c.cert).raw_sig=value.add(1); (*c.cert).raw_sig_size=vlen-1; 0 }
pub unsafe fn x509_note_serial(context:*mut c_void,_:usize,_:u8,value:*const c_void,vlen:usize)->i32 { let c=&mut *(context as *mut X509ParseContext); (*c.cert).raw_serial=value; (*c.cert).raw_serial_size=vlen; 0 }
pub unsafe fn x509_extract_name_segment(context:*mut c_void,_:usize,_:u8,value:*const c_void,vlen:usize)->i32 { let c=&mut *(context as *mut X509ParseContext); c.cn_size=vlen as u8; c.cn_offset=(value as usize-c.data) as u16; 0 }
pub unsafe fn x509_note_issuer(context:*mut c_void,_:usize,_:u8,value:*const c_void,vlen:usize)->i32 { let c=&mut *(context as *mut X509ParseContext); (*c.cert).raw_issuer=value; (*c.cert).raw_issuer_size=vlen; 0 }
pub unsafe fn x509_note_subject(context:*mut c_void,_:usize,_:u8,value:*const c_void,vlen:usize)->i32 { let c=&mut *(context as *mut X509ParseContext); (*c.cert).raw_subject=value; (*c.cert).raw_subject_size=vlen; 0 }
pub unsafe fn x509_note_params(context:*mut c_void,hdrlen:usize,_:u8,value:*const c_void,vlen:usize)->i32 { let c=&mut *(context as *mut X509ParseContext); if c.key.is_null(){c.params=value.sub(hdrlen);c.params_size=vlen+hdrlen;} 0 }
pub unsafe fn x509_extract_key_data(context:*mut c_void,_:usize,_:u8,value:*const c_void,vlen:usize)->i32 { let c=&mut *(context as *mut X509ParseContext); if vlen<1{return -74}; c.key=value.add(1);c.key_size=vlen-1;0 }
pub unsafe fn x509_process_extension(_: *mut c_void,_:usize,_:u8,_:*const c_void,_:usize)->i32 { 0 }

pub unsafe fn x509_decode_time(t:*mut time64_t,_:usize,tag:u8,value:*const u8,vlen:usize)->i32 { if (tag==0x17&&vlen!=13)||(tag==0x18&&vlen!=15){return -74}; if tag!=0x17&&tag!=0x18{return -74}; let mut p=value; let d=|p:&mut *const u8|{let a=**p;bump(p,2);(a.wrapping_sub(b'0')) as u32}; let year=if tag==0x17{let y=d(&mut p);if y>=50{y+1900}else{y+2000}}else{d(&mut p)*100+d(&mut p)}; let m=d(&mut p);let day=d(&mut p);let h=d(&mut p);let mi=d(&mut p);let s=d(&mut p);if *p!=b'Z'||year<1970||m<1||m>12||day<1||day>31||h>24||mi>59||s>60{return -74};*t=mktime64(year,m,day,h,mi,s);0 }
unsafe fn bump(p:&mut *const u8,n:usize){*p=p.add(n)}
pub unsafe fn x509_note_not_before(c:*mut c_void,h:usize,t:u8,v:*const c_void,n:usize)->i32{x509_decode_time(&mut (*(c as *mut X509ParseContext)).cert.as_mut().unwrap().valid_from,h,t,v as *const u8,n)}
pub unsafe fn x509_note_not_after(c:*mut c_void,h:usize,t:u8,v:*const c_void,n:usize)->i32{x509_decode_time(&mut (*(c as *mut X509ParseContext)).cert.as_mut().unwrap().valid_to,h,t,v as *const u8,n)}
pub unsafe fn x509_akid_note_kid(_: *mut c_void,_:usize,_:u8,_:*const c_void,_:usize)->i32{0}
pub unsafe fn x509_akid_note_name(_: *mut c_void,_:usize,_:u8,_:*const c_void,_:usize)->i32{0}
pub unsafe fn x509_akid_note_serial(_: *mut c_void,_:usize,_:u8,_:*const c_void,_:usize)->i32{0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
