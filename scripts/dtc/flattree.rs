// SPDX-License-Identifier: GPL-2.0-or-later
/* (C) Copyright David Gibson <dwg@au1.ibm.com>, IBM Corporation.  2005. */

// Dependencies supplied by the translated dtc/srcpos units.

const FTF_FULLPATH: i32 = 0x1;
const FTF_VARALIGN: i32 = 0x2;
const FTF_NAMEPROPS: i32 = 0x4;
const FTF_BOOTCPUID: i32 = 0x8;
const FTF_STRTABSIZE: i32 = 0x10;
const FTF_STRUCTSIZE: i32 = 0x20;
const FTF_NOPS: i32 = 0x40;

#[repr(C)]
struct VersionInfo { version: i32, last_comp_version: i32, hdr_size: i32, flags: i32 }

static mut VERSION_TABLE: [VersionInfo; 5] = [
    VersionInfo { version: 1, last_comp_version: 1, hdr_size: FDT_V1_SIZE, flags: FTF_FULLPATH|FTF_VARALIGN|FTF_NAMEPROPS },
    VersionInfo { version: 2, last_comp_version: 1, hdr_size: FDT_V2_SIZE, flags: FTF_FULLPATH|FTF_VARALIGN|FTF_NAMEPROPS|FTF_BOOTCPUID },
    VersionInfo { version: 3, last_comp_version: 1, hdr_size: FDT_V3_SIZE, flags: FTF_FULLPATH|FTF_VARALIGN|FTF_NAMEPROPS|FTF_BOOTCPUID|FTF_STRTABSIZE },
    VersionInfo { version: 16, last_comp_version: 16, hdr_size: FDT_V3_SIZE, flags: FTF_BOOTCPUID|FTF_STRTABSIZE|FTF_NOPS },
    VersionInfo { version: 17, last_comp_version: 16, hdr_size: FDT_V17_SIZE, flags: FTF_BOOTCPUID|FTF_STRTABSIZE|FTF_STRUCTSIZE|FTF_NOPS },
];

#[repr(C)]
struct Emitter {
    cell: unsafe fn(*mut core::ffi::c_void, cell_t),
    string: unsafe fn(*mut core::ffi::c_void, *const i8, i32),
    align: unsafe fn(*mut core::ffi::c_void, i32),
    data: unsafe fn(*mut core::ffi::c_void, data),
    beginnode: unsafe fn(*mut core::ffi::c_void, *mut label),
    endnode: unsafe fn(*mut core::ffi::c_void, *mut label),
    property: unsafe fn(*mut core::ffi::c_void, *mut label),
}

unsafe fn bin_emit_cell(e: *mut core::ffi::c_void, val: cell_t) { let d = e as *mut data; *d = data_append_cell(*d, val); }
unsafe fn bin_emit_string(e: *mut core::ffi::c_void, s: *const i8, mut len: i32) { let d=e as *mut data; if len==0 { len=strlen(s) as i32; } *d=data_append_data(*d,s as *const _,len); *d=data_append_byte(*d,0); }
unsafe fn bin_emit_align(e: *mut core::ffi::c_void, a: i32) { let d=e as *mut data; *d=data_append_align(*d,a); }
unsafe fn bin_emit_data(e: *mut core::ffi::c_void, v: data) { let d=e as *mut data; *d=data_append_data(*d,v.val,v.len); }
unsafe fn bin_emit_beginnode(e:*mut core::ffi::c_void,_:*mut label){bin_emit_cell(e,FDT_BEGIN_NODE)}
unsafe fn bin_emit_endnode(e:*mut core::ffi::c_void,_:*mut label){bin_emit_cell(e,FDT_END_NODE)}
unsafe fn bin_emit_property(e:*mut core::ffi::c_void,_:*mut label){bin_emit_cell(e,FDT_PROP)}
static mut BIN_EMITTER: Emitter=Emitter{cell:bin_emit_cell,string:bin_emit_string,align:bin_emit_align,data:bin_emit_data,beginnode:bin_emit_beginnode,endnode:bin_emit_endnode,property:bin_emit_property};

unsafe fn emit_label(f:*mut FILE,p:*const i8,l:*const i8){fprintf(f,b"\t.globl\t%s_%s\n\0".as_ptr() as _,p,l);fprintf(f,b"%s_%s:\n\0".as_ptr() as _,p,l);fprintf(f,b"_%s_%s:\n\0".as_ptr() as _,p,l)}
unsafe fn emit_offset_label(f:*mut FILE,l:*const i8,o:i32){fprintf(f,b"\t.globl\t%s\n\0".as_ptr() as _,l);fprintf(f,b"%s\t= . + %d\n\0".as_ptr() as _,l,o)}
unsafe fn asm_emit_cell(e:*mut core::ffi::c_void,v:cell_t){let f=e as *mut FILE;fprintf(f,b"\t.byte\t0x%02x\n\t.byte\t0x%02x\n\t.byte\t0x%02x\n\t.byte\t0x%02x\n\0".as_ptr() as _,(v>>24)&255,(v>>16)&255,(v>>8)&255,v&255)}
unsafe fn asm_emit_string(e:*mut core::ffi::c_void,s:*const i8,len:i32){let f=e as *mut FILE;if len!=0{fprintf(f,b"\t.asciz\t\"%.*s\"\n\0".as_ptr() as _,len,s)}else{fprintf(f,b"\t.asciz\t\"%s\"\n\0".as_ptr() as _,s)}}
unsafe fn asm_emit_align(e:*mut core::ffi::c_void,a:i32){fprintf(e as *mut FILE,b"\t.balign\t%d, 0\n\0".as_ptr() as _,a)}
unsafe fn asm_emit_data(e:*mut core::ffi::c_void,d:data){let f=e as *mut FILE;let mut off=0;while d.len-off>=4{asm_emit_cell(e,dtb_ld32(d.val.add(off as usize)));off+=4}while d.len-off>=1{fprintf(f,b"\t.byte\t0x%hhx\n\0".as_ptr() as _,*d.val.add(off as usize));off+=1}}
unsafe fn asm_emit_beginnode(e:*mut core::ffi::c_void,l:*mut label){asm_emit_cell(e,FDT_BEGIN_NODE);}
unsafe fn asm_emit_endnode(e:*mut core::ffi::c_void,l:*mut label){asm_emit_cell(e,FDT_END_NODE);}
unsafe fn asm_emit_property(e:*mut core::ffi::c_void,l:*mut label){asm_emit_cell(e,FDT_PROP);}
static mut ASM_EMITTER:Emitter=Emitter{cell:asm_emit_cell,string:asm_emit_string,align:asm_emit_align,data:asm_emit_data,beginnode:asm_emit_beginnode,endnode:asm_emit_endnode,property:asm_emit_property};

unsafe fn stringtable_insert(d:*mut data,s:*const i8)->i32{let mut i=0;while i<(*d).len{if streq(s,(*d).val.add(i as usize)){return i}i+=1}*d=data_append_data(*d,s,strlen(s) as i32+1);i}
unsafe fn flatten_tree(t:*mut node,e:*mut Emitter,target:*mut core::ffi::c_void,strbuf:*mut data,vi:*mut VersionInfo){if (*t).deleted{return}((*e).beginnode)(target,(*t).labels);if (*vi).flags&FTF_FULLPATH!=0{((*e).string)(target,(*t).fullpath,0)}else{((*e).string)(target,(*t).name,0)}((*e).align)(target,core::mem::size_of::<cell_t>() as i32);/* for_each_property */for_each_property(t,|p|{let n=stringtable_insert(strbuf,(*p).name);((*e).property)(target,(*p).labels);((*e).cell)(target,(*p).val.len as cell_t);((*e).cell)(target,n as cell_t);if (*vi).flags&FTF_VARALIGN!=0&&(*p).val.len>=8{((*e).align)(target,8)}((*e).data)(target,(*p).val);((*e).align)(target,4)});if (*vi).flags&FTF_NAMEPROPS!=0{((*e).property)(target,core::ptr::null_mut());((*e).cell)(target,((*t).basenamelen+1) as cell_t);((*e).cell)(target,stringtable_insert(strbuf,b"name\0".as_ptr() as _ ) as cell_t);((*e).string)(target,(*t).name,(*t).basenamelen);((*e).align)(target,4)}for_each_child(t,|c|flatten_tree(c,e,target,strbuf,vi));((*e).endnode)(target,(*t).labels)}

unsafe fn flatten_reserve_list(mut r:*mut reserve_info,_:*mut VersionInfo)->data{let mut d=empty_data;while !r.is_null(){d=data_append_re(d,(*r).address,(*r).size);r=(*r).next}for _ in 0..reserven um{d=data_append_re(d,0,0)}d}

unsafe fn make_fdt_header(f:*mut fdt_header,vi:*mut VersionInfo,res:i32,dt:i32,st:i32,boot:i32){memset(f as _,255,core::mem::size_of::<fdt_header>());let ro=ALIGN((*vi).hdr_size+core::mem::size_of::<fdt_reserve_entry>() as i32,8);(*f).magic=cpu_to_fdt32(FDT_MAGIC);(*f).version=cpu_to_fdt32((*vi).version as _);(*f).last_comp_version=cpu_to_fdt32((*vi).last_comp_version as _);(*f).off_mem_rsvmap=cpu_to_fdt32(ro as _);(*f).off_dt_struct=cpu_to_fdt32((ro+res) as _);(*f).off_dt_strings=cpu_to_fdt32((ro+res+dt) as _);(*f).totalsize=cpu_to_fdt32((ro+res+dt+st) as _);if (*vi).flags&FTF_BOOTCPUID!=0{(*f).boot_cpuid_phys=cpu_to_fdt32(boot)}if (*vi).flags&FTF_STRTABSIZE!=0{(*f).size_dt_strings=cpu_to_fdt32(st as _)}if (*vi).flags&FTF_STRUCTSIZE!=0{(*f).size_dt_struct=cpu_to_fdt32(dt as _)}}

pub unsafe fn dt_to_blob(f:*mut FILE,dti:*mut dt_info,version:i32){let mut vi=core::ptr::null_mut();for i in 0..VERSION_TABLE.len(){if VERSION_TABLE[i].version==version{vi=&mut VERSION_TABLE[i]}}if vi.is_null(){die(b"Unknown device tree blob version %d\n\0".as_ptr() as _,version)}let mut blob=empty_data;let mut rb=empty_data;let mut db=empty_data;let mut sb=empty_data;flatten_tree((*dti).dt,&mut BIN_EMITTER,&mut db,&mut sb,vi);bin_emit_cell(&mut db,FDT_END);rb=flatten_reserve_list((*dti).reservelist,vi);let mut h: fdt_header=core::mem::zeroed();make_fdt_header(&mut h,vi,rb.len,db.len,sb.len,(*dti).boot_cpuid_phys);let mut pad=0;if minsize>0{pad=minsize-fdt32_to_cpu(h.totalsize) as i32;if pad<0{pad=0}}if padsize>0{pad=padsize}if alignsize>0{pad=ALIGN(fdt32_to_cpu(h.totalsize) as i32+pad,alignsize)-fdt32_to_cpu(h.totalsize) as i32}h.totalsize=cpu_to_fdt32((fdt32_to_cpu(h.totalsize) as i32+pad) as _);blob=data_append_data(blob,&h as *const _ as _,vi as isize as i32);blob=data_append_align(blob,8);blob=data_merge(blob,rb);blob=data_append_zeroes(blob,core::mem::size_of::<fdt_reserve_entry>() as i32);blob=data_merge(blob,db);blob=data_merge(blob,sb);if pad>0{blob=data_append_zeroes(blob,pad)}if fwrite(blob.val,blob.len as usize,1,f)!=1{die(b"Error writing device tree blob\0".as_ptr() as _)}data_free(blob)}

pub unsafe fn dt_to_asm(f:*mut FILE,dti:*mut dt_info,version:i32){let mut vi=core::ptr::null_mut();for i in 0..VERSION_TABLE.len(){if VERSION_TABLE[i].version==version{vi=&mut VERSION_TABLE[i]}}if vi.is_null(){die(b"Unknown device tree blob version %d\n\0".as_ptr() as _)}fprintf(f,b"/* autogenerated by dtc, do not edit */\n\n\0".as_ptr() as _);let p=b"dt\0".as_ptr() as _;emit_label(f,p,b"blob_start\0".as_ptr() as _);emit_label(f,p,b"header\0".as_ptr() as _);asm_emit_cell(f,FDT_MAGIC);if (*vi).flags&FTF_BOOTCPUID!=0{asm_emit_cell(f,(*dti).boot_cpuid_phys)}asm_emit_align(f,8);emit_label(f,p,b"reserve_map\0".as_ptr() as _);fprintf(f,b"\t.long\t0, 0\n\t.long\t0, 0\n\0".as_ptr() as _);emit_label(f,p,b"struct_start\0".as_ptr() as _);let mut sb=empty_data;flatten_tree((*dti).dt,&mut ASM_EMITTER,f as _,&mut sb,vi);asm_emit_cell(f,FDT_END);emit_label(f,p,b"struct_end\0".as_ptr() as _);emit_label(f,p,b"strings_start\0".as_ptr() as _);let mut q=sb.val;while q<sb.val.add(sb.len as usize){fprintf(f,b"\t.asciz \"%s\"\n\0".as_ptr() as _,q);q=q.add(strlen(q)+1)}emit_label(f,p,b"strings_end\0".as_ptr() as _);emit_label(f,p,b"blob_end\0".as_ptr() as _);if minsize>0{fprintf(f,b"\t.space\t%d - (_dt_blob_end - _dt_blob_start), 0\n\0".as_ptr() as _,minsize)}if padsize>0{fprintf(f,b"\t.space\t%d, 0\n\0".as_ptr() as _,padsize)}if alignsize>0{asm_emit_align(f,alignsize)}emit_label(f,p,b"blob_abs_end\0".as_ptr() as _);data_free(sb)}

#[repr(C)] struct inbuf{base:*mut i8,limit:*mut i8,ptr:*mut i8}
unsafe fn inbuf_init(i:*mut inbuf,b:*mut i8,l:*mut i8){(*i).base=b;(*i).limit=l;(*i).ptr=b}
unsafe fn flat_read_chunk(i:*mut inbuf,p:*mut core::ffi::c_void,len:i32){if (*i).ptr.add(len as usize)>(*i).limit{die(b"Premature end of data parsing flat device tree\n\0".as_ptr() as _)}memcpy(p,(*i).ptr,len as usize);(*i).ptr=(*i).ptr.add(len as usize)}
unsafe fn flat_read_word(i:*mut inbuf)->u32{let mut v=0;flat_read_chunk(i,&mut v as *mut _ as _,4);fdt32_to_cpu(v)}
unsafe fn flat_realign(i:*mut inbuf,a:i32){let o=(*i).ptr.offset_from((*i).base) as i32;(*i).ptr=(*i).base.add(ALIGN(o,a) as usize);if (*i).ptr>(*i).limit{die(b"Premature end of data parsing flat device tree\n\0".as_ptr() as _)}}
unsafe fn flat_read_string(i:*mut inbuf)->*const i8{let s=(*i).ptr;while *(*i).ptr!=0{if (*i).ptr>=(*i).limit{die(b"Premature end\0".as_ptr() as _)}(*i).ptr=(*i).ptr.add(1)}(*i).ptr=(*i).ptr.add(1);flat_realign(i,4);s}
unsafe fn flat_read_data(i:*mut inbuf,len:i32)->data{if len==0{return empty_data}let mut d=data_grow_for(empty_data,len);d.len=len;flat_read_chunk(i,d.val as _,len);flat_realign(i,4);d}
unsafe fn flat_read_stringtable(i:*mut inbuf,o:i32)->*mut i8{let p=(*i).base.add(o as usize);let mut q=p;while *q!=0{if q>=(*i).limit||q<(*i).base{die(b"String offset overruns string table\n\0".as_ptr() as _)}q=q.add(1)}xstrdup(p)}
unsafe fn flat_read_property(d:*mut inbuf,s:*mut inbuf,flags:i32)->*mut property{let n=flat_read_word(d);let o=flat_read_word(d);let name=flat_read_stringtable(s,o as i32);if flags&FTF_VARALIGN!=0&&n>=8{flat_realign(d,8)}build_property(name,flat_read_data(d,n as i32),core::ptr::null_mut())}
unsafe fn flat_read_mem_reserve(i:*mut inbuf)->*mut reserve_info{let mut out=core::ptr::null_mut();loop{let mut r:fdt_reserve_entry=core::mem::zeroed();flat_read_chunk(i,&mut r as *mut _ as _,16);let a=fdt64_to_cpu(r.address);let z=fdt64_to_cpu(r.size);if z==0{break}out=add_reserve_entry(out,build_reserve_entry(a,z))}out}

unsafe fn nodename_from_path(p:*const i8,c:*const i8)->*const i8{if !strstarts(c,p){die(b"Invalid child path\n\0".as_ptr() as _)}if streq(p,b"/\0".as_ptr() as _){c}else{c.add(strlen(p)+1)}}
unsafe fn unflatten_tree(d:*mut inbuf,s:*mut inbuf,parent:*const i8,flags:i32)->*mut node{let n=build_node(core::ptr::null_mut(),core::ptr::null_mut(),core::ptr::null_mut());let flat=flat_read_string(d);(*n).name=xstrdup(if flags&FTF_FULLPATH!=0{nodename_from_path(parent,flat)}else{flat});loop{match flat_read_word(d){FDT_PROP=>add_property(n,flat_read_property(d,s,flags)),FDT_BEGIN_NODE=>add_child(n,unflatten_tree(d,s,flat,flags)),FDT_END_NODE=>break,FDT_END=>die(b"Premature FDT_END\n\0".as_ptr() as _),FDT_NOP=>(),v=>die(b"Invalid opcode word %08x\n\0".as_ptr() as _,v)}}n}

pub unsafe fn dt_from_blob(fname:*const i8)->*mut dt_info{let f=srcfile_relative_open(fname,core::ptr::null_mut());let mut h: fdt_header=core::mem::zeroed();if fread(&mut h as *mut _ as _,core::mem::size_of::<fdt_header>(),1,f)!=1{die(b"Error reading DT blob\n\0".as_ptr() as _)}let total=fdt32_to_cpu(h.totalsize);if fdt32_to_cpu(h.magic)!=FDT_MAGIC||total<FDT_V1_SIZE as u32{die(b"Invalid DT blob\n\0".as_ptr() as _)}let blob=xmalloc(total as usize) as *mut i8;memcpy(blob,&h as *const _ as _,core::mem::size_of::<fdt_header>());fread(blob.add(core::mem::size_of::<fdt_header>()),1,total as usize-core::mem::size_of::<fdt_header>(),f);let v=fdt32_to_cpu(h.version);let flags=if v<16{FTF_FULLPATH|FTF_NAMEPROPS|FTF_VARALIGN}else{FTF_NOPS};let mut m=inbuf{base:blob.add(fdt32_to_cpu(h.off_mem_rsvmap) as usize),limit:blob.add(total as usize),ptr:core::ptr::null_mut()};inbuf_init(&mut m,m.base,m.limit);let mut d=inbuf{base:blob.add(fdt32_to_cpu(h.off_dt_struct) as usize),limit:blob.add(total as usize),ptr:core::ptr::null_mut()};inbuf_init(&mut d,d.base,d.limit);let mut st=inbuf{base:blob.add(fdt32_to_cpu(h.off_dt_strings) as usize),limit:blob.add(total as usize),ptr:core::ptr::null_mut()};inbuf_init(&mut st,st.base,st.limit);let r=flat_read_mem_reserve(&mut m);if flat_read_word(&mut d)!=FDT_BEGIN_NODE{die(b"Tree does not begin\0".as_ptr() as _)}let tree=unflatten_tree(&mut d,&mut st,b"\0".as_ptr() as _,flags);if flat_read_word(&mut d)!=FDT_END{die(b"Tree does not end\0".as_ptr() as _)}free(blob as _);fclose(f);build_dt_info(DTSF_V1,r,tree,fdt32_to_cpu(h.boot_cpuid_phys))}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
