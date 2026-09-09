// SPDX-License-Identifier: GPL-2.0
/* Machine specific setup for xen. */

// C headers and build-time configuration are supplied by the surrounding kernel.

type PhysAddr = u64;
type ULong = usize;
type DomidT = u16;

const GB: fn(u64) -> u64 = |x| x * 1024 * 1024 * 1024;
const REMAP_SIZE: usize = P2M_PER_PAGE - 3;

#[repr(C)] struct E820Entry { addr: PhysAddr, size: PhysAddr, r#type: u32 }
#[repr(C)] struct E820Table { entries: *mut E820Entry, nr_entries: u32 }
#[repr(C)] struct RemapBuf { next_area_mfn: usize, target_pfn: usize, size: usize, mfns: [usize; REMAP_SIZE] }
#[repr(C)] struct MemoryReservation { address_bits: u8, extent_order: u8, domid: DomidT, nr_extents: usize, extent_start: *mut usize }
#[repr(C)] struct MmuUpdate { ptr: u64, val: usize }
#[repr(C)] struct MemoryMap { nr_entries: u32, buffer: *mut E820Entry }
#[repr(C)] struct CallbackRegister { r#type: u32, address: usize, flags: u32 }

extern "C" {
    static mut xen_pv_pci_possible: bool;
    static mut xen_e820_table: E820Table;
    static mut ini_nr_pages: usize;
    static mut xen_remap_buf: RemapBuf;
    static mut xen_remap_mfn: usize;
    static mut xen_512gb_limit: bool;
    static mut xen_extra_mem: [ExtraMem; XEN_EXTRA_MEM_MAX_REGIONS];
    static mut xen_released_pages: usize;
    static mut xen_max_p2m_pfn: usize;
    static mut max_mem_size: PhysAddr;
    static mut e820_table: E820Table;
    static mut boot_params: BootParams;
    static mut numa_off: i32;
    static xen_start_info: *mut XenStartInfo;
    static max_pfn_mapped: usize;
    static _text: u8; static _end: u8;
    static mut boot_command_line: [u8; COMMAND_LINE_SIZE];
    fn strstr(a: *const u8, b: *const u8) -> *mut u8;
    fn strlen(a: *const u8) -> usize;
    fn kstrtobool(a: *const u8, b: *mut bool) -> i32;
    fn memblock_phys_free(a: PhysAddr, b: PhysAddr); fn memblock_reserve(a: PhysAddr,b:PhysAddr);
    fn memblock_is_reserved(a: PhysAddr) -> bool;
    fn set_phys_to_machine(pfn: usize,mfn:usize)->bool; fn __set_phys_to_machine(pfn:usize,mfn:usize)->bool;
    fn pfn_to_mfn(pfn:usize)->usize; fn mfn_to_pfn(mfn:usize)->usize;
    fn set_phys_range_identity(a:usize,b:usize); fn xen_add_extra_mem(a:usize,b:usize);
    fn HYPERVISOR_memory_op(op:i32,arg:*mut core::ffi::c_void)->isize;
    fn HYPERVISOR_mmu_update(a:*mut MmuUpdate,b:u32,c:*mut u32,d:DomidT)->i32;
    fn HYPERVISOR_update_va_mapping(a:usize,b:usize,c:u32)->i32;
    fn HYPERVISOR_callback_op(a:i32,b:*mut CallbackRegister)->i32;
    fn HYPERVISOR_vm_assist(a:i32,b:i32)->i32; fn virt_to_mfn(a:*mut core::ffi::c_void)->usize;
    fn set_pte_mfn(a:usize,b:usize,c:usize); fn xen_do_remap_nonram(); fn xen_relocate_p2m();
    fn e820__range_add(a:PhysAddr,b:PhysAddr,c:u32); fn e820__update_table(a:*mut E820Table);
    fn early_memremap(a:PhysAddr,b:PhysAddr)->*mut u8; fn early_memunmap(a:*mut u8,b:PhysAddr);
    fn xen_pt_check_e820(); fn xen_panic_handler_init(); fn xen_set_default_idle()->i32;
    fn disable_acpi(); fn disable_cpuidle(); fn disable_cpufreq();
    fn xen_initial_domain()->bool; fn xen_raw_console_write(a:*const u8); fn setup_clear_cpu_cap(a:i32);
    fn cpu_feature_enabled(a:i32)->bool; fn printk(a:*const u8,...); fn pr_info(a:*const u8,...); fn pr_warn(a:*const u8,...);
}
#[repr(C)] struct ExtraMem { start_pfn:usize, n_pfns:usize }
#[repr(C)] struct XenStartInfo { cmd_line:*mut u8, nr_pages:usize, mfn_list:PhysAddr, first_p2m_pfn:usize, nr_p2m_frames:usize, flags:u32 }
#[repr(C)] struct BootHeader { ramdisk_image:PhysAddr, ramdisk_size:PhysAddr }
#[repr(C)] struct BootParams { hdr:BootHeader, ext_ramdisk_image:u32 }

const P2M_PER_PAGE:usize=1; const PAGE_SIZE:PhysAddr=4096; const PAGE_SHIFT:u32=12; const PAGE_MASK:PhysAddr=!(PAGE_SIZE-1);
const INVALID_P2M_ENTRY:usize=usize::MAX; const XEN_EXTRA_MEM_MAX_REGIONS:usize=16; const E820_TYPE_RAM:u32=1; const E820_TYPE_RESERVED:u32=2; const E820_TYPE_NVS:u32=4; const E820_TYPE_UNUSABLE:u32=5;
const DOMID_SELF:DomidT=0x7ff0; const COMMAND_LINE_SIZE:usize=2048; const MAX_GUEST_CMDLINE:usize=2048; const MAXMEM:PhysAddr=0xffff_ffff_ffff; const EXTRA_MEM_RATIO:usize=3; const NR_FIX_BTMAPS:usize=64; const ISA_START_ADDRESS:PhysAddr=0; const ISA_END_ADDRESS:PhysAddr=0x100000;

unsafe fn xen_parse_512gb(){ let mut val=false; let arg=xen_start_info; let p=(*arg).cmd_line; if strstr(p,b"xen_512gb_limit\0".as_ptr()).is_null(){return;} let q=strstr(p,b"xen_512gb_limit=\0".as_ptr()); if q.is_null(){val=true;}else if kstrtobool(q.add(strlen(b"xen_512gb_limit=\0".as_ptr())),&mut val)!=0{return;} xen_512gb_limit=val; }
unsafe fn xen_del_extra_mem(start_pfn:usize,n_pfns:usize){for i in 0..XEN_EXTRA_MEM_MAX_REGIONS{let s=xen_extra_mem[i].start_pfn;let z=xen_extra_mem[i].n_pfns;if s==start_pfn{assert!(n_pfns<=z);xen_extra_mem[i].start_pfn+=n_pfns;xen_extra_mem[i].n_pfns-=n_pfns;break;}if s+z==start_pfn+n_pfns{assert!(n_pfns<=z);xen_extra_mem[i].n_pfns-=n_pfns;break;}if start_pfn>s&&start_pfn<s+z{assert!(start_pfn+n_pfns<=s+z);xen_extra_mem[i].n_pfns=start_pfn-s;xen_add_extra_mem(start_pfn+n_pfns,s+z-start_pfn-n_pfns);break;}}memblock_phys_free((start_pfn as u64)<<PAGE_SHIFT,(n_pfns as u64)<<PAGE_SHIFT);}
pub unsafe fn xen_chk_extra_mem(pfn:usize)->usize{for i in 0..XEN_EXTRA_MEM_MAX_REGIONS{if pfn>=xen_extra_mem[i].start_pfn&&pfn<xen_extra_mem[i].start_pfn+xen_extra_mem[i].n_pfns{return INVALID_P2M_ENTRY;}} pfn}
pub unsafe fn xen_inv_extra_mem(){for i in 0..XEN_EXTRA_MEM_MAX_REGIONS{let s=xen_extra_mem[i].start_pfn;let e=s+xen_extra_mem[i].n_pfns;for p in s..e{set_phys_to_machine(p,INVALID_P2M_ENTRY);}}}
unsafe fn xen_find_pfn_range(min_pfn:&mut usize)->usize{let t=&xen_e820_table;let mut done=0;for i in 0..t.nr_entries{let e=&*t.entries.add(i as usize);if e.r#type!=E820_TYPE_RAM{continue;}let ep=((e.addr+e.size)>>PAGE_SHIFT) as usize;if ep<=*min_pfn{continue;}let sp=((e.addr+PAGE_SIZE-1)>>PAGE_SHIFT) as usize;if sp<=*min_pfn{done=ep-*min_pfn;}else{done=ep-sp;*min_pfn=sp;}break;}done}
unsafe fn xen_free_mfn(mfn:usize)->i32{let mut r=MemoryReservation{address_bits:0,extent_order:0,domid:DOMID_SELF,nr_extents:1,extent_start:&mut {let mut x=mfn;x}};HYPERVISOR_memory_op(0,&mut r as *mut _ as *mut _ ) as i32}
unsafe fn xen_set_identity_and_release_chunk(s:usize,e:usize){let end=core::cmp::min(e,ini_nr_pages);for p in s..end{let m=pfn_to_mfn(p);if m==INVALID_P2M_ENTRY||mfn_to_pfn(m)!=p{continue;}let r=xen_free_mfn(m);if r==1{xen_released_pages+=1;if !__set_phys_to_machine(p,INVALID_P2M_ENTRY){break;}}else{break;}}set_phys_range_identity(s,e);}
unsafe fn xen_update_mem_tables(pfn:usize,mfn:usize){let mut u=MmuUpdate{ptr:((mfn as u64)<<PAGE_SHIFT)|1,val:pfn};assert!(set_phys_to_machine(pfn,mfn));assert!(HYPERVISOR_mmu_update(&mut u,1,core::ptr::null_mut(),DOMID_SELF)>=0);assert!(HYPERVISOR_update_va_mapping(pfn<<PAGE_SHIFT,0,0)==0);}
unsafe fn xen_do_set_identity_and_remap_chunk(s:usize,size:usize,remap:usize){let buf=&mut xen_remap_buf as *mut _ as usize;let save=virt_to_mfn(buf as *mut _);let mut left=size;let mut ip=s;let mut rp=remap;while ip<s+size{let ch=core::cmp::min(left,REMAP_SIZE);let m=pfn_to_mfn(ip);set_pte_mfn(buf,m,0);xen_remap_buf.next_area_mfn=xen_remap_mfn;xen_remap_buf.target_pfn=rp;xen_remap_buf.size=ch;for i in 0..ch{xen_remap_buf.mfns[i]=pfn_to_mfn(ip+i);}xen_remap_mfn=m;set_phys_range_identity(ip,ip+ch);left-=ch;ip+=REMAP_SIZE;rp+=REMAP_SIZE;}set_pte_mfn(buf,save,0);}
unsafe fn xen_set_identity_and_remap_chunk(s:usize,e:usize,mut remap:usize)->usize{let mut i=0;let n=e-s;if remap==0{remap=ini_nr_pages;}while i<n{let cur=s+i;let left=n-i;let mut size=left;if cur>=ini_nr_pages{set_phys_range_identity(cur,cur+size);break;}if cur+size>ini_nr_pages{size=ini_nr_pages-cur;}let avail=xen_find_pfn_range(&mut remap);if avail==0{xen_set_identity_and_release_chunk(cur,cur+left);break;}size=core::cmp::min(size,avail);xen_do_set_identity_and_remap_chunk(cur,size,remap);i+=size;remap+=size;}remap}
unsafe fn xen_count_remap_pages(s:usize,e:usize,r:usize)->usize{if s>=ini_nr_pages{r}else{r+core::cmp::min(e,ini_nr_pages)-s}}
unsafe fn xen_foreach_remap_area(f:unsafe fn(usize,usize,usize)->usize)->usize{let mut start=0;let mut ret=0;let t=&xen_e820_table;for i in 0..t.nr_entries as usize{let e=&*t.entries.add(i);let end=e.addr+e.size;if e.r#type==E820_TYPE_RAM||i==t.nr_entries as usize-1{let s=(start>>PAGE_SHIFT) as usize;let ep=((end+PAGE_SIZE-1)>>PAGE_SHIFT) as usize;let ep=if e.r#type==E820_TYPE_RAM{((e.addr+PAGE_SIZE-1)>>PAGE_SHIFT) as usize}else{ep};if s<ep{ret=f(s,ep,ret);}start=end;}}ret}
pub unsafe fn xen_remap_memory(){let buf=&mut xen_remap_buf as *mut _ as usize;let save=virt_to_mfn(buf as *mut _);while xen_remap_mfn!=INVALID_P2M_ENTRY{set_pte_mfn(buf,xen_remap_mfn,0);let p=xen_remap_buf.target_pfn;for i in 0..xen_remap_buf.size{xen_update_mem_tables(p+i,xen_remap_buf.mfns[i]);}xen_remap_mfn=xen_remap_buf.next_area_mfn;}set_pte_mfn(buf,save,0);xen_do_remap_nonram();}
unsafe fn xen_get_pages_limit()->usize{let mut l=(MAXMEM/PAGE_SIZE) as usize;if !xen_initial_domain()&&xen_512gb_limit{l=(GB(512)/PAGE_SIZE) as usize;}l}
unsafe fn xen_get_max_pages()->usize{let l=xen_get_pages_limit();let mut m=l;if xen_initial_domain(){let mut d=DOMID_SELF;let r=HYPERVISOR_memory_op(1,&mut d as *mut _ as *mut _);if r>0{m=r as usize;}}core::cmp::min(m,l)}
unsafe fn xen_align_and_add_e820_region(mut s:PhysAddr,sz:PhysAddr,t:u32){let mut e=s+sz;if t==E820_TYPE_RAM{s=(s+PAGE_SIZE-1)&PAGE_MASK;e&=PAGE_MASK;max_mem_size=e;}e820__range_add(s,e-s,t);}
unsafe fn xen_ignore_unusable(){let t=&mut xen_e820_table;for i in 0..t.nr_entries as usize{let e=&mut *t.entries.add(i);if e.r#type==E820_TYPE_UNUSABLE{e.r#type=E820_TYPE_RAM;}}}
unsafe fn xen_is_e820_reserved(s:PhysAddr,sz:PhysAddr)->bool{if sz==0{return false;}let e=s+sz;let t=&xen_e820_table;for i in 0..t.nr_entries as usize{let x=&*t.entries.add(i);if x.r#type==E820_TYPE_RAM&&x.addr<=s&&x.addr+x.size>=e{return false;}}true}
pub unsafe fn xen_find_free_area(sz:PhysAddr)->PhysAddr{let t=&xen_e820_table;for i in 0..t.nr_entries as usize{let e=&*t.entries.add(i);if e.r#type!=E820_TYPE_RAM||e.size<sz{continue;}let mut s=e.addr;let mut a=s;while a<s+sz{if memblock_is_reserved(a){s=a+PAGE_SIZE;if s+sz>e.addr+e.size{break;}}a+=PAGE_SIZE;}if a>=s+sz{memblock_reserve(s,sz);return s;}}0}
unsafe fn xen_phys_memcpy(mut d:PhysAddr,mut s:PhysAddr,mut n:PhysAddr){while n!=0{let doff=d&!PAGE_MASK;let soff=s&!PAGE_MASK;let dl=core::cmp::min(n,(NR_FIX_BTMAPS as u64<<PAGE_SHIFT)-doff);let sl=core::cmp::min(n,(NR_FIX_BTMAPS as u64<<PAGE_SHIFT)-soff);let l=core::cmp::min(dl,sl);let to=early_memremap(d-doff,dl+doff);let from=early_memremap(s-soff,sl+soff);core::ptr::copy_nonoverlapping(from,to,l as usize);early_memunmap(to,dl+doff);early_memunmap(from,sl+soff);n-=l;d+=l;s+=l;}}
unsafe fn xen_reserve_xen_mfnlist(){let (s,z)=if xen_start_info as usize>=0x100000{((*xen_start_info).mfn_list,((*xen_start_info).nr_pages*8+PAGE_SIZE-1)&PAGE_MASK)}else{(((*xen_start_info).first_p2m_pfn as u64)<<PAGE_SHIFT,((*xen_start_info).nr_p2m_frames as u64)<<PAGE_SHIFT)};memblock_reserve(s,z);if xen_is_e820_reserved(s,z){xen_relocate_p2m();memblock_phys_free(s,z);}}
unsafe fn xen_e820_swap_entry_with_ram(_swap_entry:*mut E820Entry){assert!(false);}
unsafe fn xen_e820_resolve_conflicts(s:PhysAddr,z:PhysAddr){if z==0{return;}let end=s+z;let t=&xen_e820_table;for i in 0..t.nr_entries as usize{let e=&mut *t.entries.add(i);if e.addr>=end{return;}if e.addr+e.size>s&&e.r#type==E820_TYPE_NVS{xen_e820_swap_entry_with_ram(e);return;}}}
pub unsafe fn xen_chk_is_e820_usable(s:PhysAddr,z:PhysAddr,_c:*const u8){if !xen_is_e820_reserved(s,z){return;}xen_raw_console_write(b"Xen hypervisor allocated memory conflicts with E820 map\0".as_ptr());assert!(false);}
pub unsafe fn xen_memory_setup()->*mut u8{xen_parse_512gb();ini_nr_pages=core::cmp::min(xen_get_pages_limit(),(*xen_start_info).nr_pages);xen_reserve_xen_mfnlist();xen_foreach_remap_area(xen_set_identity_and_remap_chunk);b"Xen\0".as_ptr() as *mut u8}
unsafe fn register_callback(t:u32,f:usize)->i32{let mut c=CallbackRegister{r#type:t,address:f,flags:1};HYPERVISOR_callback_op(0,&mut c)}
pub unsafe fn xen_enable_syscall(){let _=register_callback(0,0);if !cpu_feature_enabled(0){return;}let _=register_callback(1,0);}
unsafe fn xen_pvmmu_arch_setup(){HYPERVISOR_vm_assist(0,0);assert!(register_callback(2,0)==0&&register_callback(3,0)==0);xen_enable_syscall();}
pub unsafe fn xen_arch_setup(){xen_panic_handler_init();xen_pvmmu_arch_setup();if (*xen_start_info).flags&1==0{disable_acpi();}core::ptr::copy_nonoverlapping((*xen_start_info).cmd_line,boot_command_line.as_mut_ptr(),core::cmp::min(MAX_GUEST_CMDLINE,COMMAND_LINE_SIZE));disable_cpuidle();disable_cpufreq();let _=xen_set_default_idle();numa_off=1;}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
