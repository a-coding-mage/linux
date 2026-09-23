# SPDX-License-Identifier: GPL-2.0-or-later
"""Source-built libfdt reference and safe Rust command harnesses."""

import atexit
import functools
import os
from pathlib import Path
import shlex
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]
LIB = ROOT / "scripts/dtc/libfdt"
SOURCES = ["fdt", "fdt_ro", "fdt_wip", "fdt_sw", "fdt_rw", "fdt_strerror",
           "fdt_empty_tree", "fdt_addresses", "fdt_overlay"]

# Each command has the same arguments/result on both sides. Names and property
# values are hex, so embedded/non-UTF-8 data never depends on text conversion.
OPS = {
    "empty": ("fdt_create_empty_tree(d, cap)", "f::create_empty_tree(&mut d)"),
    "create": ("fdt_create_with_flags(d, cap, n(1))", "f::create_with_flags(&mut d,n(1) as u32)"),
    "reserve": ("fdt_add_reservemap_entry(d,n(1),n(2))", "f::add_reservemap_entry(&mut d,n(1),n(2))"),
    "finishreserve": ("fdt_finish_reservemap(d)", "f::finish_reservemap(&mut d)"),
    "begin": ("fdt_begin_node(d,b(1))", "f::begin_node(&mut d,&b(1))"),
    "end": ("fdt_end_node(d)", "f::end_node(&mut d)"),
    "property": ("fdt_property(d,b(1),b(2),l(2))", "f::property_write(&mut d,&b(1),&b(2))"),
    "finish": ("fdt_finish(d)", "f::finish(&mut d)"),
    "open": ("fdt_open_into(d,d,cap)", "f::open_inplace(&mut d)"),
    "pack": ("fdt_pack(d)", "f::pack(&mut d)"),
    "addreserve": ("fdt_add_mem_rsv(d,n(1),n(2))", "f::add_mem_rsv(&mut d,n(1),n(2))"),
    "delreserve": ("fdt_del_mem_rsv(d,n(1))", "f::del_mem_rsv(&mut d,n(1) as i32)"),
    "rename": ("fdt_set_name(d,p(1),b(2))", "f::set_name(&mut d,node,&b(2))"),
    "set": ("fdt_setprop(d,p(1),b(2),b(3),l(3))", "f::setprop(&mut d,node,&b(2),&b(3))"),
    "append": ("fdt_appendprop(d,p(1),b(2),b(3),l(3))", "f::appendprop(&mut d,node,&b(2),&b(3))"),
    "del": ("fdt_delprop(d,p(1),b(2))", "f::delprop(&mut d,node,&b(2))"),
    "delnode": ("fdt_del_node(d,p(1))", "f::del_node(&mut d,node)"),
    "inplace": ("fdt_setprop_inplace(d,p(1),b(2),b(3),l(3))", "f::setprop_inplace(&mut d,node,&b(2),&b(3))"),
    "partial": ("fdt_setprop_inplace_namelen_partial(d,p(1),b(2),l(2),n(3),b(4),l(4))", "f::setprop_inplace_partial(&mut d,node,&b(2),n(3) as usize,&b(4))"),
    "nopprop": ("fdt_nop_property(d,p(1),b(2))", "f::nop_property(&mut d,node,&b(2))"),
    "nopnode": ("fdt_nop_node(d,p(1))", "f::nop_node(&mut d,node)"),
    "addrange": ("fdt_appendprop_addrrange(d,p(1),p(2),b(3),n(4),n(5))", "f::appendprop_addrrange(&mut d,node,node2,&b(3),n(4),n(5))"),
    "check": ("fdt_check_header(d)", "f::check_header(&d)"),
}
INTOPS = {
    "add": ("fdt_add_subnode(d,p(1),b(2))", "f::add_subnode(&mut d,node,&b(2))"),
    "path": ("fdt_path_offset(d,b(1))", "f::path_offset(&d,&b(1))"),
    "depth": ("fdt_node_depth(d,n(1))", "f::node_depth(&d,n(1) as i32)"),
    "parent": ("fdt_parent_offset(d,n(1))", "f::parent_offset(&d,n(1) as i32)"),
    "firstprop": ("fdt_first_property_offset(d,n(1))", "f::first_property_offset(&d,n(1) as i32)"),
    "nextprop": ("fdt_next_property_offset(d,n(1))", "f::next_property_offset(&d,n(1) as i32)"),
    "firstsub": ("fdt_first_subnode(d,n(1))", "f::first_subnode(&d,n(1) as i32)"),
    "nextsub": ("fdt_next_subnode(d,n(1))", "f::next_subnode(&d,n(1) as i32)"),
    "findphandle": ("fdt_node_offset_by_phandle(d,n(1))", "f::node_offset_by_phandle(&d,n(1) as u32)"),
    "findvalue": ("fdt_node_offset_by_prop_value(d,n(1),b(2),b(3),l(3))", "f::node_offset_by_prop_value(&d,n(1) as i32,&b(2),&b(3))"),
    "findcompat": ("fdt_node_offset_by_compatible(d,n(1),b(2))", "f::node_offset_by_compatible(&d,n(1) as i32,&b(2))"),
    "count": ("fdt_stringlist_count(d,p(1),b(2))", "f::stringlist_count(&d,node,&b(2))"),
    "search": ("fdt_stringlist_search(d,p(1),b(2),b(3))", "f::stringlist_search(&d,node,&b(2),&b(3))"),
    "ac": ("fdt_address_cells(d,p(1))", "f::address_cells(&d,node).map(|n|n as i32)"),
    "sc": ("fdt_size_cells(d,p(1))", "f::size_cells(&d,node).map(|n|n as i32)"),
}

C_PREFIX = r'''
#include <libfdt.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
static char *args[10]; static unsigned char *decoded[10]; static int lengths[10];
static unsigned char *d; static int cap;
static unsigned long long n(int i) {return strtoull(args[i] ? args[i] : "0",NULL,0);}
static char *b(int i) {
 if (!decoded[i]) {
  char *s=args[i] ? args[i] : "-"; int len=strcmp(s,"-") ? strlen(s)/2:0;
  decoded[i]=calloc(len+1,1); lengths[i]=len;
  for(int j=0;j<len;j++){unsigned v=0;sscanf(s+j*2,"%2x",&v);decoded[i][j]=v;}
 } return (char *)decoded[i];
}
static int l(int i) {b(i);return lengths[i];}
static int p(int i) {return fdt_path_offset(d,b(i));}
static void hex(const void *v,int len) {const unsigned char *p=v;for(int i=0;i<len;i++)printf("%02x",p[i]);}
static void value(const void *v,int len) {printf("%d ",len);if(v)hex(v,len);putchar('\n');}
static void walk(void) {
 int at=-1; while((at=fdt_next_node(d,at,NULL))>=0) {
  int len; const char *name=fdt_get_name(d,at,&len);printf("N %d %d %u ",at,fdt_node_depth(d,at),fdt_get_phandle(d,at));value(name,len);
  int prop=fdt_first_property_offset(d,at);
  while(prop>=0) {const char *name;const void *val=fdt_getprop_by_offset(d,prop,&name,&len);printf("P %d ",prop);if(name)hex(name,strlen(name));putchar(' ');value(val,len);prop=fdt_next_property_offset(d,prop);}
  printf("E %d\n",prop);
 } printf("END %d\n",at);
}
int main(int argc,char **argv) {
 if(argc!=4)return 2;cap=atoi(argv[3]);d=calloc(cap,1);
 FILE *in=fopen(argv[1],"rb");if(in){fread(d,1,cap,in);fclose(in);}
 char *line=NULL;size_t linesize=0;
 while(getline(&line,&linesize,stdin)>=0) {
  memset(args,0,sizeof(args));memset(decoded,0,sizeof(decoded));
  int count=0;char *save,*s=strtok_r(line," \n",&save);
  while(s&&count<10){args[count++]=s;s=strtok_r(NULL," \n",&save);}if(!count)continue;
  const char *op=args[0];int ret=0;
  if(!strcmp(op,"walk")){walk();goto done;}
  if(!strcmp(op,"get")){int len;const void *v=fdt_getprop(d,p(1),b(2),&len);value(v,len);goto done;}
  if(!strcmp(op,"string")){int len;const void *v=fdt_get_string(d,n(1),&len);value(v,len);goto done;}
  if(!strcmp(op,"listget")){int len;const void *v=fdt_stringlist_get(d,p(1),b(2),n(3),&len);value(v,len);goto done;}
  if(!strcmp(op,"name")){int len;const void *v=fdt_get_name(d,n(1),&len);value(v,len);goto done;}
  if(!strcmp(op,"getpath")){int len=n(2);char *buf=calloc(len+1,1);ret=fdt_get_path(d,n(1),buf,len);printf("%d ",ret);if(!ret)hex(buf,strlen(buf));putchar('\n');free(buf);goto done;}
  if(!strcmp(op,"tag")){int next;unsigned tag=fdt_next_tag(d,n(1),&next);printf("%u %d\n",tag,next);goto done;}
  if(!strcmp(op,"next")){int dep=n(2);ret=fdt_next_node(d,n(1),&dep);printf("%d %d\n",ret,dep);goto done;}
  if(!strcmp(op,"rsv")){uint64_t a=0,z=0;ret=fdt_get_mem_rsv(d,n(1),&a,&z);printf("%d %llu %llu\n",ret,(unsigned long long)a,(unsigned long long)z);goto done;}
  if(!strcmp(op,"max")){uint32_t v=0;ret=fdt_find_max_phandle(d,&v);printf("%d %u\n",ret,v);goto done;}
  if(!strcmp(op,"gen")){uint32_t v=0;ret=fdt_generate_phandle(d,&v);printf("%d %u\n",ret,v);goto done;}
  if(!strcmp(op,"overlay")){FILE *f=fopen(b(1),"rb");fseek(f,0,SEEK_END);size_t len=ftell(f);rewind(f);void *ov=malloc(len);fread(ov,1,len,f);fclose(f);ret=fdt_overlay_apply(d,ov);char *path=malloc(strlen(argv[2])+9);sprintf(path,"%s.overlay",argv[2]);f=fopen(path,"wb");fwrite(ov,1,len,f);fclose(f);free(path);free(ov);}
  else if(!strcmp(op,"copyopen")||!strcmp(op,"resize")){int size=n(1);unsigned char *copy=calloc(cap,1);ret=!strcmp(op,"resize")?fdt_resize(d,copy,size):fdt_open_into(d,copy,size);if(!ret)memcpy(d,copy,cap);free(copy);}
'''
C_SUFFIX = r'''
 else {fprintf(stderr,"unknown %s\n",op);return 3;}
 printf("%d\n",ret);
done:for(int i=0;i<10;i++)free(decoded[i]);
 }
 FILE *out=fopen(argv[2],"wb");fwrite(d,1,cap,out);fclose(out);free(d);free(line);return 0;
}
'''

RUST_PREFIX = r'''
//! libfdt differential harness.
#[path="@LIB@/mod.rs"] mod f;
use std::{io::{self,BufRead},fs};
use std::os::unix::ffi::OsStrExt;
fn hex(v:&[u8])->String {v.iter().map(|b|format!("{b:02x}")).collect()}
fn value(v:f::Result<&[u8]>) {match v {Ok(v)=>println!("{} {}",v.len(),hex(v)),Err(e)=>println!("{} ",e.code())}}
fn walk(d:&[u8]) {
 let mut at=-1;loop {match f::next_node(d,at,None) {
  Ok(node)=>{at=node;print!("N {} {} {} ",at,f::node_depth(d,at).unwrap_or_else(|e|e.code()),f::get_phandle(d,at));value(f::get_name(d,at));
   let mut prop=f::first_property_offset(d,at);while let Ok(off)=prop {match f::getprop_by_offset(d,off){Ok((name,v))=>{print!("P {} {} ",off,hex(name));value(Ok(v));},Err(e)=>{print!("P {}  ",off);value(Err(e));}}prop=f::next_property_offset(d,off);}println!("E {}",prop.unwrap_err().code());
  },Err(e)=>{println!("END {}",e.code());break;}
 }}
}
fn main(){
 let argv:Vec<_>=std::env::args_os().collect();let cap:usize=argv[3].to_str().unwrap().parse().unwrap();let mut d=vec![0;cap];
 if let Ok(v)=fs::read(&argv[1]){let n=v.len().min(cap);d[..n].copy_from_slice(&v[..n]);}
 for line in io::stdin().lock().lines(){let line=line.unwrap();let a:Vec<_>=line.split_whitespace().collect();if a.is_empty(){continue;}
  let n=|i:usize|->u64{let s=a.get(i).copied().unwrap_or("0");if let Some(s)=s.strip_prefix('-'){0u64.wrapping_sub(s.parse().unwrap())}else if let Some(s)=s.strip_prefix("0x"){u64::from_str_radix(s,16).unwrap()}else{s.parse().unwrap()}};
  let b=|i:usize|->Vec<u8>{let s=a.get(i).copied().unwrap_or("-");if s=="-"{Vec::new()}else{(0..s.len()).step_by(2).map(|j|u8::from_str_radix(&s[j..j+2],16).unwrap()).collect()}};
  let node=if ["rename","set","append","del","delnode","inplace","partial","nopprop","nopnode","addrange","add","count","search","ac","sc","get","listget"].contains(&a[0]){f::path_offset(&d,&b(1)).unwrap_or_else(|e|e.code())}else{0};
  let node2=if a[0]=="addrange"{f::path_offset(&d,&b(2)).unwrap_or_else(|e|e.code())}else{0};
  let ret:i32=match a[0]{
   "walk"=>{walk(&d);continue;},
   "get"=>{value(f::getprop(&d,node,&b(2)));continue;},
   "string"=>{value(f::get_string(&d,n(1) as i32));continue;},
   "listget"=>{value(f::stringlist_get(&d,node,&b(2),n(3) as i32));continue;},
   "name"=>{value(f::get_name(&d,n(1) as i32));continue;},
   "getpath"=>{let mut buf=vec![0;n(2) as usize];let ret=f::get_path_into(&d,n(1) as i32,&mut buf);println!("{} {}",ret.map(|_|0).unwrap_or_else(|e|e.code()),if ret.is_ok(){hex(&buf[..buf.iter().position(|&b|b==0).unwrap()])}else{String::new()});continue;},
   "tag"=>{let (tag,next)=f::next_tag(&d,n(1) as i32);println!("{} {}",tag,next.unwrap_or_else(|e|e.code()));continue;},
   "next"=>{let mut depth=n(2) as i32;let ret=f::next_node(&d,n(1) as i32,Some(&mut depth));println!("{} {}",ret.unwrap_or_else(|e|e.code()),depth);continue;},
   "rsv"=>{match f::get_mem_rsv(&d,n(1) as i32){Ok((a,z))=>println!("0 {a} {z}"),Err(e)=>println!("{} 0 0",e.code())}continue;},
   "max"|"gen"=>{match if a[0]=="max"{f::find_max_phandle(&d)}else{f::generate_phandle(&d)}{Ok(v)=>println!("0 {v}"),Err(e)=>println!("{} 0",e.code())}continue;},
   "overlay"=>{let mut ov=fs::read(std::ffi::OsStr::from_bytes(&b(1))).unwrap();let ret=f::overlay_apply(&mut d,&mut ov);let mut path=argv[2].clone();path.push(".overlay");fs::write(path,ov).unwrap();ret.map(|_|0).unwrap_or_else(|e|e.code())},
   "copyopen"|"resize"=>{let mut copy=vec![0;cap];let ret=if a[0]=="resize"{f::resize(&d,&mut copy[..n(1) as usize])}else{f::open_into(&d,&mut copy[..n(1) as usize])};if ret.is_ok(){d=copy;}ret.map(|_|0).unwrap_or_else(|e|e.code())},
'''
RUST_SUFFIX = r'''
 _=>panic!("unknown command"),
 };println!("{ret}");}
 fs::write(&argv[2],d).unwrap();
}
'''


@functools.lru_cache(maxsize=1)
def cached_libfdt_tools():
    temp = tempfile.TemporaryDirectory(prefix="libfdt-tools-")
    atexit.register(temp.cleanup)
    work = Path(temp.name)
    c = C_PREFIX + "\n".join(
        f'else if(!strcmp(op,"{op}"))ret={body[0]};'
        for op, body in (OPS | INTOPS).items()) + C_SUFFIX
    rust = RUST_PREFIX.replace("@LIB@", str(LIB)) + "\n".join(
        f'"{op}"=>{body[1]}.map(|_|0).unwrap_or_else(|e|e.code()),'
        for op, body in OPS.items()) + "\n" + "\n".join(
        f'"{op}"=>{body[1]}.unwrap_or_else(|e|e.code()),'
        for op, body in INTOPS.items()) + RUST_SUFFIX
    (work / "harness.c").write_text(c)
    (work / "harness.rs").write_text(rust)
    cc = shlex.split(os.environ.get("HOSTCC", "cc"))
    rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
    subprocess.run(cc + ["-O2", "-I", str(LIB), str(work / "harness.c")]
                   + [str(LIB / (s + ".c")) for s in SOURCES]
                   + ["-o", str(work / "c")], check=True, capture_output=True)
    subprocess.run(rustc + ["--edition=2021", "-O", "-Dwarnings", str(work / "harness.rs"),
                            "-o", str(work / "rust")], check=True, capture_output=True)
    return work / "c", work / "rust"


def cmd(name, *args):
    def token(arg):
        if isinstance(arg, str):
            arg = arg.encode()
        if isinstance(arg, bytes):
            return arg.hex() or "-"
        return str(arg)
    return " ".join([name] + [token(arg) for arg in args])


@functools.lru_cache(maxsize=1)
def cached_fdt_utilities():
    temp = tempfile.TemporaryDirectory(prefix="fdt-utilities-")
    atexit.register(temp.cleanup)
    work = Path(temp.name)
    cc = shlex.split(os.environ.get("HOSTCC", "cc"))
    rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
    result = {}
    for name in ("fdtget", "fdtput", "fdtoverlay"):
        # Linux retained old, unbuilt get/put applets when util.h changed. The
        # temporary reference adapts only the reader signature and usage macro;
        # all option handling, algorithms, formatting and mutations remain C.
        source = (LIB.parent / (name + ".c")).read_text()
        if name != "fdtoverlay":
            source = source.replace('#include "util.h"', '#include "util.h"\n#undef usage')
            source = source.replace("utilfdt_read(filename)", "utilfdt_read(filename, NULL)")
        src = work / (name + ".c")
        src.write_text(source)
        c, rust = work / (name + "-c"), work / (name + "-rust")
        subprocess.run(cc + ["-O2", "-I", str(LIB), "-I", str(LIB.parent), str(src),
                             str(LIB.parent / "util.c")]
                       + [str(LIB / (s + ".c")) for s in SOURCES]
                       + ["-o", str(c)], check=True, capture_output=True)
        subprocess.run(rustc + ["--edition=2021", "-O", "-Dwarnings", "-Wmissing_docs",
                                str(LIB.parent / (name + ".rs")), "-o", str(rust)],
                       check=True, capture_output=True)
        result[name] = c, rust
    return result
