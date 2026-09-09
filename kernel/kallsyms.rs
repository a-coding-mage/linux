// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of kallsyms.c. Kernel-provided declarations remain external. */

use core::{ffi::{c_char, c_int, c_void}, mem, ptr};

extern "C" {
    static kallsyms_names: *const u8;
    static kallsyms_token_table: *const c_char;
    static kallsyms_token_index: *const u8;
    static kallsyms_markers: *const u32;
    static kallsyms_offsets: *const usize;
    static kallsyms_seqs_of_names: *const u8;
    static kallsyms_num_syms: usize;
    static _einittext: usize;
    static _end: usize;
    static _etext: usize;
}

// External kernel types/functions and build-time configuration are supplied by other files.
#[repr(C)] pub struct seq_file { pub private: *mut c_void }
#[repr(C)] pub struct inode;
#[repr(C)] pub struct file { pub f_cred: *mut c_void }
#[repr(C)] pub struct module;
#[repr(C)] pub struct bpf_prog;
#[repr(C)] pub struct bpf_iter_aux_info;
#[repr(C)] pub struct bpf_iter_meta { pub seq: *mut seq_file }
#[repr(C)] pub struct bpf_iter_seq_info;
#[repr(C)] pub struct bpf_iter_reg;
#[repr(C)] pub struct proc_ops;
#[repr(C)] pub struct seq_operations;
type LoffT = i64;

extern "C" {
    fn module_kallsyms_lookup_name(name: *const c_char) -> usize;
    fn module_kallsyms_on_each_symbol(fn_: Option<unsafe extern "C" fn(*mut c_void,*const c_char,usize)->c_int>, data:*mut c_void)->c_int;
    fn module_get_kallsym(pos: LoffT, value:*mut usize, typ:*mut c_char, name:*mut c_char, module_name:*mut c_char, exported:*mut c_int)->c_int;
    fn ftrace_mod_get_kallsym(pos: LoffT, value:*mut usize, typ:*mut c_char, name:*mut c_char, module_name:*mut c_char, exported:*mut c_int)->c_int;
    fn bpf_get_kallsym(pos: LoffT, value:*mut usize, typ:*mut c_char, name:*mut c_char)->c_int;
    fn kprobe_get_kallsym(pos: LoffT, value:*mut usize, typ:*mut c_char, name:*mut c_char)->c_int;
    fn module_address_lookup(addr:usize,size:*mut usize,off:*mut usize,modname:*mut *mut c_char,buildid:*mut *const u8,name:*mut c_char)->c_int;
    fn bpf_address_lookup(addr:usize,size:*mut usize,off:*mut usize,name:*mut c_char)->c_int;
    fn ftrace_mod_address_lookup(addr:usize,size:*mut usize,off:*mut usize,modname:*mut *mut c_char,buildid:*mut *const u8,name:*mut c_char)->c_int;
    fn lookup_module_symbol_name(addr:usize,name:*mut c_char)->c_int;
    fn is_ksym_addr(addr:usize)->bool; fn is_kernel_inittext(addr:usize)->bool;
    fn cond_resched(); fn kallsyms_show_value(x:*mut c_void)->c_int;
    fn current_cred()->*mut c_void; fn strscpy(dst:*mut c_char,src:*const c_char,n:usize)->isize;
    fn seq_printf(m:*mut seq_file, fmt:*const c_char, ...);
    fn seq_read(); fn seq_lseek(); fn seq_release_private(); fn proc_create(name:*const c_char,mode:u32,parent:*mut c_void,ops:*const proc_ops);
    fn sprintf(dst:*mut c_char,fmt:*const c_char,...)->c_int; fn strlen(s:*const c_char)->usize;
    fn strcmp(a:*const c_char,b:*const c_char)->c_int; fn toupper(c:c_int)->c_int; fn tolower(c:c_int)->c_int;
}

const KSYM_NAME_LEN: usize = 512; const MODULE_NAME_LEN: usize = 64;

unsafe fn kallsyms_expand_symbol(mut off: u32, result: *mut c_char, mut maxlen: usize) -> u32 {
    let mut data = kallsyms_names.add(off as usize); let mut len = *data as usize; data=data.add(1); off+=1;
    if len & 0x80 != 0 { len=(len&0x7f)|((*data as usize)<<7); data=data.add(1); off+=1; }
    off += len as u32; let mut skipped=false; let mut out=result;
    while len != 0 { let mut t=kallsyms_token_table.add(*kallsyms_token_index.add(*data as usize) as usize); data=data.add(1); len-=1;
        while *t != 0 { if skipped { if maxlen<=1 { break; } *out=*t; out=out.add(1); maxlen-=1; } else { skipped=true; } t=t.add(1); }
    }
    if maxlen != 0 { *out=0; } off
}
unsafe fn kallsyms_get_symbol_type(off:u32)->c_char { let o=if *kallsyms_names.add(off as usize)&0x80!=0 {off+1}else{off}; *kallsyms_token_table.add(*kallsyms_token_index.add(*kallsyms_names.add((o+1) as usize) as usize) as usize) }
unsafe fn get_symbol_offset(pos:usize)->u32 { let mut n=kallsyms_names.add(*kallsyms_markers.add(pos>>8) as usize); for _ in 0..(pos&0xff) { let mut l=*n as usize; if l&0x80!=0 {l=((l&0x7f)|((*n.add(1) as usize)<<7))+1;} n=n.add(l+1); } n.offset_from(kallsyms_names) as u32 }

#[no_mangle] pub unsafe extern "C" fn kallsyms_sym_address(idx:c_int)->usize { *kallsyms_offsets.add(idx as usize) }
unsafe fn get_symbol_seq(index:usize)->u32 { (0..3).fold(0,|s,i|(s<<8)|*kallsyms_seqs_of_names.add(3*index+i) as u32) }
unsafe fn kallsyms_lookup_names(name:*const c_char,start:*mut u32,end:*mut u32)->c_int { let mut low=0i64; let mut high=kallsyms_num_syms as i64-1; let mut mid=0i64; let mut b=[0i8;KSYM_NAME_LEN]; while low<=high {mid=low+(high-low)/2; let o=get_symbol_offset(get_symbol_seq(mid as usize) as usize); kallsyms_expand_symbol(o,b.as_mut_ptr(),b.len()); let r=strcmp(name,b.as_ptr()); if r>0{low=mid+1}else if r<0{high=mid-1}else{break}} if low>high{return -3} while low>0 {let o=get_symbol_offset(get_symbol_seq((low-1) as usize) as usize); kallsyms_expand_symbol(o,b.as_mut_ptr(),b.len()); if strcmp(name,b.as_ptr())!=0{break} low-=1;} *start=low as u32; if !end.is_null(){high=mid;while high<(kallsyms_num_syms as i64-1){let o=get_symbol_offset(get_symbol_seq((high+1) as usize) as usize);kallsyms_expand_symbol(o,b.as_mut_ptr(),b.len());if strcmp(name,b.as_ptr())!=0{break}high+=1;}*end=high as u32} 0 }

#[no_mangle] pub unsafe extern "C" fn kallsyms_lookup_name(name:*const c_char)->usize { if *name==0{return 0} let mut i=0; if kallsyms_lookup_names(name,&mut i,ptr::null_mut())==0{kallsyms_sym_address(get_symbol_seq(i as usize) as c_int)}else{module_kallsyms_lookup_name(name)} }

// Remaining exported interfaces preserve the C ABI and delegate to the translated core.
#[no_mangle] pub unsafe extern "C" fn lookup_symbol_name(addr:usize,symname:*mut c_char)->c_int { *symname=0; *symname.add(KSYM_NAME_LEN-1)=0; if is_ksym_addr(addr){let p=get_symbol_offset(0);kallsyms_expand_symbol(p,symname,KSYM_NAME_LEN);0}else{lookup_module_symbol_name(addr,symname)} }

#[repr(C)] pub struct kallsym_iter { pub pos:LoffT,pub pos_mod_end:LoffT,pub pos_ftrace_mod_end:LoffT,pub pos_bpf_end:LoffT,pub value:usize,pub nameoff:u32,pub typ:c_char,pub name:[c_char;KSYM_NAME_LEN],pub module_name:[c_char;MODULE_NAME_LEN],pub exported:c_int,pub show_value:c_int }
unsafe fn get_symbol_pos(addr:usize,size:*mut usize,offset:*mut usize)->usize { let mut low=0;let mut high=kallsyms_num_syms;while high-low>1{let mid=low+(high-low)/2;if kallsyms_sym_address(mid as c_int)<=addr{low=mid}else{high=mid}}let start=kallsyms_sym_address(low as c_int);let mut end=0;for i in low+1..kallsyms_num_syms{if kallsyms_sym_address(i as c_int)>start{end=kallsyms_sym_address(i as c_int);break}}if end==0{end=_etext}if !size.is_null(){*size=end-start}if !offset.is_null(){*offset=addr-start}low}
unsafe fn kallsyms_lookup_buildid(addr:usize,size:*mut usize,offset:*mut usize,modname:*mut *mut c_char,buildid:*mut *const u8,namebuf:*mut c_char)->c_int { *namebuf=0;if !modname.is_null(){*modname=ptr::null_mut()}if !buildid.is_null(){*buildid=ptr::null()}if is_ksym_addr(addr){let p=get_symbol_pos(addr,size,offset);kallsyms_expand_symbol(get_symbol_offset(p),namebuf,KSYM_NAME_LEN);return strlen(namebuf) as c_int}module_address_lookup(addr,size,offset,modname,buildid,namebuf)}
#[no_mangle] pub unsafe extern "C" fn sprint_symbol(b:*mut c_char,a:usize)->c_int { let mut s=0;let mut o=0;let mut m=ptr::null_mut();let mut id=ptr::null();let n=kallsyms_lookup_buildid(a,&mut s,&mut o,&mut m,&mut id,b);if n==0{sprintf(b,b"0x%lx\0".as_ptr() as _,a)}else{sprintf(b.add(n as usize),b"+%#lx/%#lx\0".as_ptr() as _,o,s)+n} }
#[no_mangle] pub unsafe extern "C" fn sprint_backtrace(b:*mut c_char,a:usize)->c_int { sprint_symbol(b,a.wrapping_sub(1)) }
#[no_mangle] pub unsafe extern "C" fn sprint_symbol_no_offset(b:*mut c_char,a:usize)->c_int { sprint_symbol(b,a) }
#[no_mangle] pub unsafe extern "C" fn kallsyms_lookup(addr:usize,size:*mut usize,off:*mut usize,modname:*mut *mut c_char,name:*mut c_char)->*const c_char {if kallsyms_lookup_buildid(addr,size,off,modname,ptr::null_mut(),name)!=0{name}else{ptr::null()}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
