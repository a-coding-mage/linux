// SPDX-License-Identifier: GPL-2.0-only
// Low level x86 E820 memory map handling functions.
// Translated from e820.c; kernel-provided dependencies remain external.

#[repr(C)]
pub struct change_member { pub entry: *mut e820_entry, pub addr: u64 }
#[repr(C)]
pub struct e820_entry { pub addr: u64, pub size: u64, pub type_: e820_type }
pub type e820_type = u32;
#[repr(C)]
pub struct e820_table { pub nr_entries: u32, pub entries: [e820_entry; E820_MAX_ENTRIES] }
#[repr(C)]
pub struct boot_e820_entry { pub addr: u64, pub size: u64, pub type_: u32 }
#[repr(C)]
pub struct setup_data { pub next: u64, pub type_: u32, pub len: u32, pub data: [u8; 0] }
#[repr(C)]
pub struct resource { pub start: u64, pub end: u64, pub name: *const i8, pub flags: u64, pub desc: u64, pub parent: *mut resource }

extern "C" {
    static mut e820_table: *mut e820_table;
    static mut e820_table_kexec: *mut e820_table;
    static mut e820_table_firmware: *mut e820_table;
    static mut pci_mem_start: usize;
}

unsafe fn _e820__mapped_any(table:*mut e820_table,start:u64,end:u64,type_:e820_type)->bool {
    let mut idx=0;
    while idx<(*table).nr_entries { let e=&(*table).entries[idx as usize];
        if type_!=0&&e.type_!=type_ {idx+=1;continue;}
        if e.addr>=end||e.addr.wrapping_add(e.size)<=start {idx+=1;continue;}
        return true; idx+=1;
    } false
}
pub unsafe fn e820__mapped_raw_any(s:u64,e:u64,t:e820_type)->bool{_e820__mapped_any(e820_table_firmware,s,e,t)}
pub unsafe fn e820__mapped_any(s:u64,e:u64,t:e820_type)->bool{_e820__mapped_any(e820_table,s,e,t)}

unsafe fn __e820__mapped_all(mut start:u64,end:u64,type_:e820_type)->*mut e820_entry {
    let mut idx=0; while idx<(*e820_table).nr_entries {let e=&mut(*e820_table).entries[idx as usize];
        if type_!=0&&e.type_!=type_ {idx+=1;continue;}
        if e.addr>=end||e.addr.wrapping_add(e.size)<=start {idx+=1;continue;}
        if e.addr<=start {start=e.addr+e.size;} if start>=end{return e as *mut _}; idx+=1;
    } core::ptr::null_mut()
}
pub unsafe fn e820__mapped_all(s:u64,e:u64,t:e820_type)->bool{!__e820__mapped_all(s,e,t).is_null()}
pub unsafe fn e820__get_entry_type(s:u64,e:u64)->i32{let p=__e820__mapped_all(s,e,0);if p.is_null(){-22}else{(*p).type_ as i32}}

unsafe fn __e820__range_add(t:*mut e820_table,s:u64,z:u64,ty:e820_type){let i=(*t).nr_entries as usize;if i<E820_MAX_ENTRIES{(*t).entries[i]=e820_entry{addr:s,size:z,type_:ty};(*t).nr_entries+=1;}}
pub unsafe fn e820__range_add(s:u64,z:u64,t:e820_type){__e820__range_add(e820_table,s,z,t)}

unsafe fn e820_type_mergeable(t:e820_type)->bool{t!=E820_TYPE_PRAM&&t!=E820_TYPE_SOFT_RESERVED}
static mut CHANGE_POINT_LIST:[change_member;2*E820_MAX_ENTRIES]=[change_member{entry:core::ptr::null_mut(),addr:0};2*E820_MAX_ENTRIES];
static mut NEW_ENTRIES:[e820_entry;E820_MAX_ENTRIES]=[e820_entry{addr:0,size:0,type_:0};E820_MAX_ENTRIES];

pub unsafe fn e820__update_table(t:*mut e820_table)->i32 {
    let n=(*t).nr_entries as usize;if n<2{return -1;}
    for i in 0..n{let e=&(*t).entries[i];if e.addr.wrapping_add(e.size)<e.addr{return -1;}}
    let mut cp:Vec<*mut change_member>=Vec::new();
    for i in 0..n{let e=&mut(*t).entries[i] as *mut _;if (*e).size!=0{
        let a=&mut CHANGE_POINT_LIST[cp.len()] as *mut _;(*a).addr=(*e).addr;(*a).entry=e;cp.push(a);
        let a=&mut CHANGE_POINT_LIST[cp.len()] as *mut _;(*a).addr=(*e).addr+(*e).size;(*a).entry=e;cp.push(a);
    }}
    cp.sort_by(|a,b|{let x=**a;let y=**b;x.addr.cmp(&y.addr).then((x.addr!=(*x.entry).addr).cmp(&(y.addr!=(*y.entry).addr)))});
    let mut overlap:Vec<*mut e820_entry>=Vec::new();let mut out=0usize;let mut last_type=0;let mut last_addr=0;
    for p in cp{let e=(*p).entry;if (*p).addr==(*e).addr{overlap.push(e)}else if let Some(i)=overlap.iter().position(|x|*x==e){overlap[i]=*overlap.last().unwrap();overlap.pop();}
        let mut cur=0;for e in &overlap{if(**e).type_>cur{cur=(**e).type_;}}
        if cur!=last_type||!e820_type_mergeable(cur){if last_type{NEW_ENTRIES[out].size=(*p).addr-last_addr;if NEW_ENTRIES[out].size!=0{out+=1;if out>=E820_MAX_ENTRIES{break;}}}if cur!=0{NEW_ENTRIES[out].addr=(*p).addr;NEW_ENTRIES[out].type_=cur;last_addr=(*p).addr;}last_type=cur;}
    }
    core::ptr::copy_nonoverlapping(NEW_ENTRIES.as_ptr(),(*t).entries.as_mut_ptr(),out);(*t).nr_entries=out as u32;0
}

unsafe fn __e820__range_update(t:*mut e820_table,start:u64,mut size:u64,old:e820_type,new_:e820_type)->u64{
    if size>u64::MAX-start{size=u64::MAX-start}let end=start+size;let mut total=0;let mut i=0;
    while i<(*t).nr_entries{let e=&mut(*t).entries[i as usize];if e.type_!=old{i+=1;continue;}let ee=e.addr+e.size;
        if e.addr>=start&&ee<=end{e.type_=new_;total+=e.size}
        else if e.addr<start&&ee>end{__e820__range_add(t,start,size,new_);__e820__range_add(t,end,ee-end,e.type_);e.size=start-e.addr;total+=size}
        else{let a=core::cmp::max(start,e.addr);let b=core::cmp::min(end,ee);if a<b{__e820__range_add(t,a,b-a,new_);total+=b-a;e.size-=b-a;if e.addr>=a{e.addr=b}}}i+=1;
    }total
}
pub unsafe fn e820__range_update(s:u64,z:u64,o:e820_type,n:e820_type)->u64{__e820__range_update(e820_table,s,z,o,n)}
pub unsafe fn e820__range_update_table(t:*mut e820_table,s:u64,z:u64,o:e820_type,n:e820_type)->u64{__e820__range_update(t,s,z,o,n)}
pub unsafe fn e820__range_remove(s:u64,mut z:u64,filter:e820_type){if z>u64::MAX-s{z=u64::MAX-s}let end=s+z;let mut i=0;while i<(*e820_table).nr_entries{let e=&mut(*e820_table).entries[i as usize];if filter!=0&&e.type_!=filter{i+=1;continue;}let ee=e.addr+e.size;if e.addr>=s&&ee<=end{*e=e820_entry{addr:0,size:0,type_:0}}else if e.addr<s&&ee>end{__e820__range_add(e820_table,end,ee-end,e.type_);e.size=s-e.addr}else{let a=core::cmp::max(s,e.addr);let b=core::cmp::min(end,ee);if a<b{e.size-=b-a;if e.addr>=a{e.addr=b}}}i+=1}}
pub unsafe fn e820__update_table_print(){let _=e820__update_table(e820_table);}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
