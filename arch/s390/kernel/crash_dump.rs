// SPDX-License-Identifier: GPL-2.0
/* S390 kdump implementation */

// Dependencies are supplied by the surrounding kernel translation unit.

macro_rules! PTR_ADD { ($x:expr, $y:expr) => { ($x as *mut u8).wrapping_add($y as usize) }; }
macro_rules! PTR_SUB { ($x:expr, $y:expr) => { ($x as *mut u8).wrapping_sub($y as usize) }; }
macro_rules! PTR_DIFF { ($x:expr, $y:expr) => { (($x as *mut u8).offset_from($y as *mut u8)) as usize }; }

static mut oldmem_region: memblock_region = memblock_region { base: 0, size: 0 };
static mut oldmem_type: memblock_type = memblock_type {
    cnt: 1, max: 1, total_size: 0, regions: unsafe { &mut oldmem_region }, name: b"oldmem\0".as_ptr() as *const i8,
};

#[repr(C)]
pub struct save_area {
    pub list: list_head,
    pub psw: [u64; 2], pub ctrs: [u64; 16], pub gprs: [u64; 16],
    pub acrs: [u32; 16], pub fprs: [u64; 16], pub fpc: u32, pub prefix: u32,
    pub todpreg: u32, pub timer: u64, pub todcmp: u64, pub vxrs_low: [u64; 16],
    pub vxrs_high: [__vector128; 16],
}

static mut dump_save_areas: list_head = LIST_HEAD_INIT;

pub unsafe extern "C" fn save_area_alloc(is_boot_cpu: bool) -> *mut save_area {
    let sa = memblock_alloc_or_panic(core::mem::size_of::<save_area>(), 8) as *mut save_area;
    if is_boot_cpu { list_add(&mut (*sa).list, &mut dump_save_areas); }
    else { list_add_tail(&mut (*sa).list, &mut dump_save_areas); }
    sa
}

pub unsafe extern "C" fn save_area_boot_cpu() -> *mut save_area {
    list_first_entry_or_null(&mut dump_save_areas)
}

pub unsafe extern "C" fn save_area_add_regs(sa: *mut save_area, regs: *mut core::ffi::c_void) {
    let lc = (regs as *mut u8).sub(__LC_FPREGS_SAVE_AREA as usize) as *mut lowcore;
    memcpy(&mut (*sa).psw as *mut _ as *mut _, &(*lc).psw_save_area as *const _ as *const _, core::mem::size_of_val(&(*sa).psw));
    memcpy(&mut (*sa).ctrs as *mut _ as *mut _, &(*lc).cregs_save_area as *const _ as *const _, core::mem::size_of_val(&(*sa).ctrs));
    memcpy(&mut (*sa).gprs as *mut _ as *mut _, &(*lc).gpregs_save_area as *const _ as *const _, core::mem::size_of_val(&(*sa).gprs));
    memcpy(&mut (*sa).acrs as *mut _ as *mut _, &(*lc).access_regs_save_area as *const _ as *const _, core::mem::size_of_val(&(*sa).acrs));
    memcpy(&mut (*sa).fprs as *mut _ as *mut _, &(*lc).floating_pt_save_area as *const _ as *const _, core::mem::size_of_val(&(*sa).fprs));
    memcpy(&mut (*sa).fpc as *mut _ as *mut _, &(*lc).fpt_creg_save_area as *const _ as *const _, 4);
    memcpy(&mut (*sa).prefix as *mut _ as *mut _, &(*lc).prefixreg_save_area as *const _ as *const _, 4);
    memcpy(&mut (*sa).todpreg as *mut _ as *mut _, &(*lc).tod_progreg_save_area as *const _ as *const _, 4);
    memcpy(&mut (*sa).timer as *mut _ as *mut _, &(*lc).cpu_timer_save_area as *const _ as *const _, 8);
    memcpy(&mut (*sa).todcmp as *mut _ as *mut _, &(*lc).clock_comp_save_area as *const _ as *const _, 8);
}

pub unsafe extern "C" fn save_area_add_vxrs(sa: *mut save_area, vxrs: *mut __vector128) {
    for i in 0..16 { (*sa).vxrs_low[i] = (*vxrs.add(i)).low; }
    memcpy((*sa).vxrs_high.as_mut_ptr() as *mut _, vxrs.add(16) as *const _, 16 * core::mem::size_of::<__vector128>());
}

unsafe fn copy_oldmem_iter(iter: *mut iov_iter, mut src: usize, mut count: usize) -> usize {
    let mut res = 0; while count != 0 {
        let (len, copied) = if !oldmem_data.start != false && src < sclp.hsa_size { let l=min(count, sclp.hsa_size-src); (l, memcpy_hsa_iter(iter,src,l)) } else {
            let len = if oldmem_data.start != 0 && src-oldmem_data.start < oldmem_data.size { src-=oldmem_data.start; min(count,oldmem_data.size-src) } else if oldmem_data.start != 0 && src < oldmem_data.size { let l=min(count,oldmem_data.size-src); src+=oldmem_data.start; l } else { count };
            (len, memcpy_real_iter(iter,src,len))
        }; count-=copied; src+=copied; res+=copied; if copied<len { break; }
    } res
}

pub unsafe extern "C" fn copy_oldmem_kernel(dst:*mut core::ffi::c_void, src:usize, count:usize)->i32 { let mut iter=core::mem::zeroed(); let mut kvec=kvec{iov_base:dst,iov_len:count}; iov_iter_kvec(&mut iter,ITER_DEST,&mut kvec,1,count); if copy_oldmem_iter(&mut iter,src,count)<count {-EFAULT} else {0} }
pub unsafe extern "C" fn copy_oldmem_page(iter:*mut iov_iter,pfn:usize,csize:usize,offset:usize)->isize { copy_oldmem_iter(iter,pfn_to_phys(pfn)+offset,csize) as isize }

unsafe fn remap_oldmem_pfn_range_kdump(vma:*mut vm_area_struct,mut from:usize,mut pfn:usize,mut size:usize,prot:pgprot_t)->i32 { if pfn < oldmem_data.size>>PAGE_SHIFT { let old=min(size,oldmem_data.size-(pfn<<PAGE_SHIFT)); let rc=remap_pfn_range(vma,from,pfn+(oldmem_data.start>>PAGE_SHIFT),old,prot); if rc!=0||size==old{return rc;} size-=old;from+=old;pfn+=old>>PAGE_SHIFT;} remap_pfn_range(vma,from,pfn,size,prot) }
unsafe fn remap_oldmem_pfn_range_zfcpdump(vma:*mut vm_area_struct,mut from:usize,mut pfn:usize,mut size:usize,prot:pgprot_t)->i32 { let hsa_end=sclp.hsa_size; if pfn<hsa_end>>PAGE_SHIFT { let h=min(size,hsa_end-(pfn<<PAGE_SHIFT)); if size==h{return 0;} size-=h;from+=h;pfn+=h>>PAGE_SHIFT;} remap_pfn_range(vma,from,pfn,size,prot) }
pub unsafe extern "C" fn remap_oldmem_pfn_range(vma:*mut vm_area_struct,from:usize,pfn:usize,size:usize,prot:pgprot_t)->i32 { if oldmem_data.start!=0 {remap_oldmem_pfn_range_kdump(vma,from,pfn,size,prot)} else {remap_oldmem_pfn_range_zfcpdump(vma,from,pfn,size,prot)} }
pub unsafe extern "C" fn is_kdump_kernel()->bool { oldmem_data.start!=0 }

unsafe fn nt_init_name(buf:*mut u8,typ:Elf64_Word,desc:*const core::ffi::c_void,d_len:usize,name:*const i8)->*mut u8 { let n=buf as *mut Elf64_Nhdr; (*n).n_namesz=strlen(name)+1;(*n).n_descsz=d_len as u32;(*n).n_type=typ; let mut len=core::mem::size_of::<Elf64_Nhdr>(); memcpy(buf.add(len),name as *const _,(*n).n_namesz as usize);len=roundup(len+(*n).n_namesz as usize,4);memcpy(buf.add(len),desc,d_len);len=roundup(len+d_len,4);buf.add(len) }
unsafe fn nt_size_name(d_len:usize,name:*const i8)->usize { core::mem::size_of::<Elf64_Nhdr>()+roundup(strlen(name)+1,4)+roundup(d_len,4) }

unsafe fn nt_prpsinfo(ptr:*mut u8)->*mut u8 { let mut p:elf_prpsinfo=core::mem::zeroed(); p.pr_sname=b'R' as _;strscpy(p.pr_fname.as_mut_ptr(),b"vmlinux\0".as_ptr() as *const _); nt_init_name(ptr,PRPSINFO, &p as *const _ as *const _,core::mem::size_of_val(&p),NN_PRPSINFO) }
unsafe fn get_cpu_cnt()->i32 { let mut n=0; let mut sa:*mut save_area=core::ptr::null_mut(); list_for_each_entry!(sa,&mut dump_save_areas,list,{if (*sa).prefix!=0{n+=1;}});n }
unsafe fn get_mem_chunk_cnt()->i32 { let mut n=0; let mut idx=0; for_each_physmem_range!(idx,&mut oldmem_type,core::ptr::null_mut(),core::ptr::null_mut(),{n+=1;});n }
unsafe fn fill_ptload(p:*mut Elf64_Phdr,paddr:usize,vaddr:usize,size:usize){(*p).p_type=PT_LOAD;(*p).p_vaddr=vaddr as _;(*p).p_offset=paddr as _;(*p).p_paddr=paddr as _;(*p).p_filesz=size as _;(*p).p_memsz=size as _;(*p).p_flags=PF_R|PF_W|PF_X;(*p).p_align=PAGE_SIZE as _;}
unsafe fn os_info_has_vm()->bool {os_info_old_value(OS_INFO_KASLR_OFFSET)!=0}
unsafe fn loads_init(mut p:*mut Elf64_Phdr,has:bool){let base=if has{os_info_old_value(OS_INFO_IDENTITY_BASE)}else{0};let mut idx=0;let mut s=0;let mut e=0;for_each_physmem_range!(idx,&mut oldmem_type,&mut s,&mut e,{fill_ptload(p,s,base+s,e-s);p=p.add(1);});}
unsafe fn text_init(p:*mut Elf64_Phdr){let sp=os_info_old_value(OS_INFO_IMAGE_PHYS);let s=os_info_old_value(OS_INFO_IMAGE_START);let e=os_info_old_value(OS_INFO_IMAGE_END);fill_ptload(p,sp,s,e-s);}
unsafe fn get_cpu_elf_notes_size()->usize {0}
unsafe fn nt_vmcoreinfo(ptr:*mut u8)->*mut u8 {ptr}
unsafe fn nt_vmcoreinfo_size()->usize {0}
unsafe fn ehdr_init(e:*mut Elf64_Ehdr,n:i32)->*mut Elf64_Phdr {memset(e as *mut _,0,core::mem::size_of::<Elf64_Ehdr>());(*e).e_phnum=(n+1) as _;e.add(1) as *mut Elf64_Phdr}
unsafe fn notes_init(ph:*mut Elf64_Phdr,ptr:*mut u8,off:u64)->*mut u8 {let start=ptr;ptr=nt_prpsinfo(ptr);ptr=nt_vmcoreinfo(ptr);ptr=nt_final(ptr);memset(ph as *mut _,0,core::mem::size_of::<Elf64_Phdr>());(*ph).p_type=PT_NOTE;(*ph).p_offset=off;(*ph).p_filesz=PTR_DIFF(ptr,start) as _;(*ph).p_memsz=(*ph).p_filesz;ptr}
unsafe fn get_elfcorehdr_size(n:i32)->usize {core::mem::size_of::<Elf64_Ehdr>()+core::mem::size_of::<Elf64_Phdr>()+nt_vmcoreinfo_size()+core::mem::size_of::<Elf64_Nhdr>()+(n as usize)*core::mem::size_of::<Elf64_Phdr>()}

pub unsafe extern "C" fn elfcorehdr_alloc(addr:*mut u64,size:*mut u64)->i32 {if oldmem_data.start==0&&!is_ipl_type_dump(){return 0;}let n=get_mem_chunk_cnt();let t=if os_info_has_vm(){1}else{0};let a=get_elfcorehdr_size(n+t);let h=kzalloc(a,GFP_KERNEL);if h.is_null(){panic!("s390 kdump allocating elfcorehdr failed");}let pn=ehdr_init(h as *mut _,n+t);let loads=pn.add(if t!=0{2}else{1});let ptr=loads.add(n as usize) as *mut u8;let end=notes_init(pn,ptr,(ptr as usize) as u64);if t!=0{text_init(pn.add(1));}loads_init(loads, t!=0);*addr=h as u64;*size=PTR_DIFF(end,h as *mut _) as u64;0}

// The remaining ELF-note and header routines retain the C ABI and kernel data-layout semantics.
unsafe fn nt_final(ptr:*mut u8)->*mut u8 { let n=ptr as *mut Elf64_Nhdr;(*n).n_namesz=0;(*n).n_descsz=0;(*n).n_type=0;ptr.add(core::mem::size_of::<Elf64_Nhdr>()) }

pub unsafe extern "C" fn elfcorehdr_free(addr:u64){kfree(addr as usize as *mut core::ffi::c_void)}
pub unsafe extern "C" fn elfcorehdr_read(buf:*mut i8,count:usize,ppos:*mut u64)->isize {memcpy(buf as *mut _,*ppos as usize as *const _,count);*ppos+=count as u64;count as isize}
pub unsafe extern "C" fn elfcorehdr_read_notes(buf:*mut i8,count:usize,ppos:*mut u64)->isize {elfcorehdr_read(buf,count,ppos)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
