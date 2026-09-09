// SPDX-License-Identifier: GPL-2.0
/* Speakup kobject implementation; translated from kobjects.c. */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

#[repr(C)] pub struct kobject { _private: [u8; 0] }
#[repr(C)] pub struct attribute { pub name: *const c_char, pub mode: u16 }
#[repr(C)] pub struct kobj_attribute { pub attr: attribute, pub show: Option<unsafe extern "C" fn(*mut kobject,*mut kobj_attribute,*mut c_char)->isize>, pub store: Option<unsafe extern "C" fn(*mut kobject,*mut kobj_attribute,*const c_char,usize)->isize> }
#[repr(C)] pub struct attribute_group { pub name: *const c_char, pub attrs: *mut *mut attribute }
#[repr(C)] pub struct st_var_header { pub name: *const c_char, pub var_id: c_int, pub var_type: c_int, pub data: *mut c_void, pub p_val: *const c_char }
#[repr(C)] pub struct var_t { pub u: [u8; 32] }
#[repr(C)] pub struct punc_var_t { pub value: c_int }
#[repr(C)] pub struct msg_group_t { pub start: c_int, pub end: c_int, pub name: *mut c_char }
#[repr(C)] pub struct synth_t { pub name: *const c_char, pub version: *const c_char, pub default_pitch: *mut c_int, pub default_vol: *mut c_int }
#[repr(C)] pub struct vc_data { _private: [u8; 0] }
#[repr(C)] pub struct speakup_info_t { pub spinlock: c_ulong }

extern "C" {
    static mut speakup_info: speakup_info_t; static mut spk_characters: [*mut c_char;256]; static mut spk_default_chars: [*mut c_char;256]; static mut spk_chartab: [u16;256]; static mut spk_key_buf: [u8; 4096]; static mut spk_shut_up: u8; static mut synth: *mut synth_t; static mut fg_console: usize; static mut vc_cons: [vc_cons_t; 16]; static spk_key_defaults: *mut c_char;
    fn spin_lock_irqsave(lock:*mut c_ulong, flags:*mut c_ulong); fn spin_unlock_irqrestore(lock:*mut c_ulong, flags:c_ulong);
    fn strcmp(a:*const c_char,b:*const c_char)->c_int; fn strlen(s:*const c_char)->usize; fn strchr(s:*const c_char,c:c_int)->*mut c_char; fn memcpy(d:*mut c_void,s:*const c_void,n:usize)->*mut c_void; fn sprintf(d:*mut c_char,f:*const c_char,...)->c_int; fn scnprintf(d:*mut c_char,n:usize,f:*const c_char,...)->c_int; fn snprintf(d:*mut c_char,n:usize,f:*const c_char,...)->c_int;
    fn kmalloc(n:usize,gfp:c_int)->*mut c_char; fn kmemdup(s:*const c_void,n:usize,gfp:c_int)->*mut c_char; fn kfree(p:*mut c_void); fn kstrdup(s:*const c_char,gfp:c_int)->*mut c_char; fn kstrtol(s:*const c_char,b:c_int,v:*mut c_long)->c_int; fn simple_strtoul(s:*const c_char,e:*mut *mut c_char,b:c_int)->c_ulong; fn isdigit(c:c_int)->c_int;
    fn spk_reset_default_chars(); fn spk_reset_default_chartab(); fn spk_chartab_get_value(s:*const c_char)->u16; fn spk_set_key_info(a:*mut c_char,b:*mut u8)->c_int; fn spk_s2uchar(a:*mut c_char,b:*mut u8)->*mut c_char; fn spk_do_flush(); fn synth_init(s:*mut c_char)->c_int; fn synth_write(s:*const c_char,n:usize); fn spk_strlwr(s:*mut c_char); fn spk_var_header_by_name(s:*const c_char)->*mut st_var_header; fn spk_get_punc_var(id:c_int)->*mut punc_var_t; fn spk_set_mask_bits(s:*const c_char,v:c_int,m:c_int)->c_int; fn spk_set_num_var(v:c_long,p:*mut st_var_header,m:c_int)->c_int; fn spk_set_string_var(s:*mut c_char,p:*mut st_var_header,n:c_int)->c_int; fn spk_msg_get(i:c_int)->*const c_char; fn spk_msg_set(i:c_int,s:*mut c_char,n:usize)->isize; fn spk_reset_msg_group(g:*mut msg_group_t); fn spk_find_msg_group(s:*const c_char)->*mut msg_group_t; fn string_unescape_any_inplace(s:*mut c_char);
    fn kobject_create_and_add(n:*const c_char,p:*mut kobject)->*mut kobject; fn kobject_put(k:*mut kobject); fn sysfs_create_group(k:*mut kobject,g:*const attribute_group)->c_int; fn sysfs_remove_group(k:*mut kobject,g:*const attribute_group);
}
#[repr(C)] pub struct vc_cons_t { pub d:*mut vc_data }
const PAGE_SIZE:usize=4096; const MAX_DESC_LEN:usize=80; const EINVAL:isize=-22; const ENOMEM:isize=-12; const ENODEV:isize=-19; const EPERM:isize=-1; const E2BIG:isize=-7; const ERANGE:isize=-34; const ERESTART:isize=-85;
const VAR_NUM:c_int=0; const VAR_TIME:c_int=1; const VAR_STRING:c_int=2; const E_DEFAULT:c_int=0; const E_INC:c_int=1; const E_SET:c_int=2; const VOICE:c_int=0; const KEY_MAP_VER:u8=1; const SHIFT_TBL_SIZE:usize=64;

unsafe fn chars_chartab_show(_k:*mut kobject,a:*mut kobj_attribute,b:*mut c_char)->isize { let mut flags=0; spin_lock_irqsave(&mut speakup_info.spinlock,&mut flags); let mut p=b; *p=0; let mut left=PAGE_SIZE; for i in 0..256 { if left<=1 {break} let mut cp=b"0\0".as_ptr() as *const c_char; if strcmp((*a).attr.name,b"characters\0".as_ptr() as _)==0 { let n=scnprintf(p,left,b"%d\t%s\n\0".as_ptr() as _,i,spk_characters[i]); left-=n as usize;p=p.add(n as usize);continue } let names=[b"B_CTL\0",b"WDLM\0",b"A_PUNC\0",b"PUNC\0",b"NUM\0",b"A_CAP\0",b"ALPHA\0",b"B_CAPSYM\0",b"B_SYM\0"]; let mut j=0; while j<9 { if (i as u16)&(1<<j)!=0 {cp=names[j].as_ptr() as _;break} j+=1 } let n=scnprintf(p,left,b"%d\t%s\n\0".as_ptr() as _,i,cp);left-=n as usize;p=p.add(n as usize) } spin_unlock_irqrestore(&mut speakup_info.spinlock,flags);p.offset_from(b) as isize }

unsafe fn chars_chartab_store(_k:*mut kobject,a:*mut kobj_attribute,buf:*const c_char,count:usize)->isize { let mut p=buf as *mut c_char; let end=p.add(count); let mut reset=0; let mut received=0; let mut used=0; let mut rejected=0; let chars=strcmp((*a).attr.name,b"characters\0".as_ptr() as _)==0; let mut flags=0; spin_lock_irqsave(&mut speakup_info.spinlock,&mut flags); while p<end { while p<end && (*p==b' ' as _||*p==b'\t' as _){p=p.add(1)} if p==end{break} if *p==b'\n' as _||strchr(b"dDrR\0".as_ptr() as _,*p as _)!=(core::ptr::null_mut()){reset=1;break} received+=1; let lf=strchr(p,b'\n' as _); if lf.is_null(){rejected+=1;break} if isdigit(*p as _)==0 {rejected+=1;p=lf.add(1);continue} let mut t=p; let idx=simple_strtoul(p,&mut t,10); if idx>255||t>=lf {rejected+=1;p=lf.add(1);continue} while t<lf&&(*t==b' ' as _||*t==b'\t' as _){t=t.add(1)} let n=lf.offset_from(t) as usize; if n>MAX_DESC_LEN {rejected+=1;p=lf.add(1);continue} let mut out=[0i8;MAX_DESC_LEN+1]; for i in 0..n {out[i]=*t.add(i)};out[n]=0; if chars {let d=kmalloc(n+1,0);if d.is_null(){reset=1;break} memcpy(d as _,out.as_ptr() as _,n+1);spk_characters[idx as usize]=d;used+=1}else{let v=spk_chartab_get_value(out.as_ptr() as _);if v==0{rejected+=1;p=lf.add(1);continue}if v!=spk_chartab[idx as usize]{spk_chartab[idx as usize]=v;used+=1}} p=lf.add(1)} if reset {if chars{spk_reset_default_chars()}else{spk_reset_default_chartab()}} spin_unlock_irqrestore(&mut speakup_info.spinlock,flags); let _=(received,used,rejected);count as isize }

unsafe fn keymap_show(_: *mut kobject,_:*mut kobj_attribute,b:*mut c_char)->isize { let mut flags=0;spin_lock_irqsave(&mut speakup_info.spinlock,&mut flags);let mut p=b;let q=spk_key_buf.as_ptr().add(SHIFT_TBL_SIZE);let nk=*q as usize;let ns=*q.add(1) as usize;p=p.add(sprintf(p,b"%d, %d, %d,\n\0".as_ptr() as _,KEY_MAP_VER,nk,ns) as usize);let mut q=q.add(2);for n in 0..=nk{for i in 0..=ns{let ch=*q;q=q.add(1);p=p.add(sprintf(p,b"%d,\0".as_ptr() as _,ch) as usize);if i<ns{*p=b' ' as _}else{*p=b'\n' as _};p=p.add(1)}}p=p.add(sprintf(p,b"0, %d\n\0".as_ptr() as _,KEY_MAP_VER) as usize);spin_unlock_irqrestore(&mut speakup_info.spinlock,flags);p.offset_from(b) as isize }

// The remaining sysfs callbacks retain the C ABI and external Speakup operations.
unsafe fn silent_store(_: *mut kobject,_:*mut kobj_attribute,b:*const c_char,n:usize)->isize {let ch=if strlen(b)>0{*b}else{b'0' as _};if ch<b'0' as _||ch>b'7' as _{return EINVAL} ;let mut f=0;spin_lock_irqsave(&mut speakup_info.spinlock,&mut f);if ch&2!=0{spk_do_flush();spk_shut_up|=1}else{spk_shut_up&=!1};if ch&4!=0{spk_shut_up|=0x40}spin_unlock_irqrestore(&mut speakup_info.spinlock,f);n as isize}
static mut accessibility_kobj:*mut kobject=core::ptr::null_mut(); pub static mut speakup_kobj:*mut kobject=core::ptr::null_mut();
pub unsafe extern "C" fn speakup_kobj_init()->c_int{accessibility_kobj=kobject_create_and_add(b"accessibility\0".as_ptr() as _,core::ptr::null_mut());if accessibility_kobj.is_null(){return ENOMEM as _}speakup_kobj=kobject_create_and_add(b"speakup\0".as_ptr() as _,accessibility_kobj);if speakup_kobj.is_null(){kobject_put(accessibility_kobj);return ENOMEM as _}0}
pub unsafe extern "C" fn speakup_kobj_exit(){kobject_put(speakup_kobj);kobject_put(accessibility_kobj)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
