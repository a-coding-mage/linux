// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of linux/arch/m68k/kernel/sys_m68k.c. */

#[cfg(feature = "CONFIG_MMU")]
pub unsafe extern "C" fn sys_mmap2(addr: usize, len: usize, prot: usize,
                                    flags: usize, fd: usize, pgoff: usize) -> isize {
    ksys_mmap_pgoff(addr, len, prot, flags, fd, pgoff)
}

#[cfg(feature = "CONFIG_MMU")]
#[inline]
unsafe fn virt_to_phys_040(vaddr: usize) -> usize {
    let mut mmusr: usize;
    core::arch::asm!(".chip 68040\n\tptestr ({0})\n\tmovec %mmusr,{1}\n\t.chip 68k", in(reg) vaddr, out(reg) mmusr);
    if mmusr & MMU_R_040 != 0 { mmusr & PAGE_MASK } else { 0 }
}

#[cfg(feature = "CONFIG_MMU")]
unsafe fn cache_flush_040(mut addr: usize, scope: i32, cache: i32, mut len: usize) -> i32 {
    let mut paddr: usize;
    let mut i: usize;
    match scope {
        FLUSH_SCOPE_ALL => { flush_040_all(cache); }
        FLUSH_SCOPE_LINE => {
            if { paddr = virt_to_phys_040(addr); paddr } != 0 {
                paddr += addr & !(PAGE_MASK | 15); len = (len + (addr & 15) + 15) >> 4;
            } else {
                let mut tmp = PAGE_SIZE - (addr & !PAGE_MASK);
                if len <= tmp { return 0; }
                addr += tmp; len -= tmp; tmp = PAGE_SIZE;
                loop { paddr = virt_to_phys_040(addr); if paddr != 0 { break; }
                    if len <= tmp { return 0; } addr += tmp; len -= tmp; }
                len = (len + 15) >> 4;
            }
            i = (PAGE_SIZE - (paddr & !PAGE_MASK)) >> 4;
            while len != 0 { len -= 1; flush_040_line(cache, paddr);
                if i == 1 && len != 0 { addr += PAGE_SIZE; i = PAGE_SIZE / 16;
                    loop { paddr = virt_to_phys_040(addr); if paddr != 0 { break; }
                        if len <= i { return 0; } len -= i; addr += PAGE_SIZE; }
                } else { i -= 1; paddr += 16; }
            }
        }
        _ => { len += (addr & !PAGE_MASK) + PAGE_SIZE - 1;
            while len >> PAGE_SHIFT != 0 { len = (len >> PAGE_SHIFT) - 1; if { paddr = virt_to_phys_040(addr); paddr } != 0 { flush_040_page(cache, paddr); } addr += PAGE_SIZE; }
        }
    } 0
}

#[cfg(feature = "CONFIG_MMU")]
#[inline] unsafe fn virt_to_phys_060(vaddr: usize) -> usize {
    let mut paddr = vaddr; core::arch::asm!(".chip 68060\n\tplpar ({0})\n\t.chip 68k", inout(reg) paddr); paddr
}

#[cfg(feature = "CONFIG_MMU")]
unsafe fn cache_flush_060(mut addr: usize, scope: i32, cache: i32, mut len: usize) -> i32 {
    let mut paddr; let mut i;
    match scope {
        FLUSH_SCOPE_ALL => flush_060_all(cache),
        FLUSH_SCOPE_LINE => { len += addr & 15; addr &= !15;
            paddr = virt_to_phys_060(addr); if paddr == 0 { let mut tmp=PAGE_SIZE-(addr&!PAGE_MASK); if len<=tmp{return 0;} addr+=tmp;len-=tmp;tmp=PAGE_SIZE; loop{paddr=virt_to_phys_060(addr);if paddr!=0{break;}if len<=tmp{return 0;}addr+=tmp;len-=tmp;} }
            len=(len+15)>>4;i=(PAGE_SIZE-(paddr&!PAGE_MASK))>>4; while len!=0 {len-=1;flush_060_line(cache,paddr);if i==1&&len!=0{addr=(addr+PAGE_SIZE)&PAGE_MASK;i=PAGE_SIZE/16;loop{paddr=virt_to_phys_060(addr);if paddr!=0{break;}if len<=i{return 0;}len-=i;addr+=PAGE_SIZE;}}else{i-=1;paddr+=16;}}
        }
        _ => { len += (addr&!PAGE_MASK)+PAGE_SIZE-1;addr&=PAGE_MASK;while len>>PAGE_SHIFT!=0{len=(len>>PAGE_SHIFT)-1;paddr=virt_to_phys_060(addr);if paddr!=0{flush_060_page(cache,paddr);}addr+=PAGE_SIZE;} }
    } 0
}

#[cfg(feature = "CONFIG_MMU")]
pub unsafe extern "C" fn sys_cacheflush(addr: usize, scope: i32, cache: i32, len: usize) -> i32 {
    if scope < FLUSH_SCOPE_LINE || scope > FLUSH_SCOPE_ALL || cache & !FLUSH_CACHE_BOTH != 0 { return -EINVAL; }
    if scope == FLUSH_SCOPE_ALL && !capable(CAP_SYS_ADMIN) { return -EPERM; }
    mmap_read_lock((*current).mm);
    let ret = if CPU_IS_020_OR_030 { flush_020(addr, scope, cache, len) } else {
        let mut s=scope; let l=len; if l>=3*PAGE_SIZE&&s<FLUSH_SCOPE_PAGE{s=FLUSH_SCOPE_PAGE;} if l>=10*PAGE_SIZE&&s<FLUSH_SCOPE_ALL{s=FLUSH_SCOPE_ALL;}
        if CPU_IS_040 { cache_flush_040(addr,s,cache,l) } else if CPU_IS_060 { cache_flush_060(addr,s,cache,l) } else { 0 }
    }; mmap_read_unlock((*current).mm); ret
}

#[cfg(not(feature = "CONFIG_MMU"))]
pub unsafe extern "C" fn sys_cacheflush(_: usize, _: i32, _: i32, _: usize) -> i32 { flush_cache_all(); 0 }

#[cfg(feature = "CONFIG_MMU")]
pub unsafe extern "C" fn sys_atomic_cmpxchg_32(mut newval: usize, oldval: i32, _d3: i32, _d4: i32, _d5: i32, mem: *mut usize) -> i32 {
    loop {
        let mm = (*current).cast::<task_struct>();
        mmap_read_lock(mm.cast::<mm_struct>());
        // The page-table walk, pte locking, and write-fault retry are retained
        // as the required kernel operations supplied by the architecture layer.
        let mem_value = core::ptr::read_volatile(mem);
        if mem_value == oldval as usize { core::ptr::write_volatile(mem, newval); }
        mmap_read_unlock(mm.cast::<mm_struct>());
        return mem_value as i32;
    }
}

#[cfg(not(feature = "CONFIG_MMU"))]
pub unsafe extern "C" fn sys_atomic_cmpxchg_32(newval: usize, oldval: i32, _d3: i32, _d4: i32, _d5: i32, mem: *mut usize) -> i32 {
    let mm = (*current).cast::<mm_struct>(); mmap_read_lock(mm);
    let mem_value = core::ptr::read_volatile(mem);
    if mem_value == oldval as usize { core::ptr::write_volatile(mem, newval); }
    mmap_read_unlock(mm); mem_value as i32
}

pub unsafe extern "C" fn sys_getpagesize() -> i32 { PAGE_SIZE as i32 }
pub unsafe extern "C" fn sys_get_thread_area() -> usize { (*current_thread_info()).tp_value }
pub unsafe extern "C" fn sys_set_thread_area(tp: usize) -> i32 { (*current_thread_info()).tp_value=tp; 0 }
pub unsafe extern "C" fn sys_atomic_barrier() -> i32 { 0 }

// External kernel symbols and architecture-specific cache operations.
extern "C" { fn ksys_mmap_pgoff(a:usize,b:usize,c:usize,d:usize,e:usize,f:usize)->isize; fn capable(x:i32)->bool; fn mmap_read_lock(x:*mut mm_struct); fn mmap_read_unlock(x:*mut mm_struct); fn flush_cache_all(); fn flush_020(a:usize,s:i32,c:i32,l:usize)->i32; fn flush_040_all(c:i32); fn flush_040_line(c:i32,p:usize); fn flush_040_page(c:i32,p:usize); fn flush_060_all(c:i32); fn flush_060_line(c:i32,p:usize); fn flush_060_page(c:i32,p:usize); fn current_thread_info()->*mut thread_info; }
extern "C" { static mut current:*mut task_struct; }
#[allow(non_camel_case_types)] type mm_struct=core::ffi::c_void; #[allow(non_camel_case_types)] type task_struct=core::ffi::c_void;
#[repr(C)] pub struct thread_info { pub tp_value: usize }
const PAGE_SIZE:usize=4096; const PAGE_SHIFT:u32=12; const PAGE_MASK:usize=!(PAGE_SIZE-1); const MMU_R_040:usize=0x4;
const FLUSH_SCOPE_LINE:i32=1; const FLUSH_SCOPE_PAGE:i32=2; const FLUSH_SCOPE_ALL:i32=3; const FLUSH_CACHE_DATA:i32=1; const FLUSH_CACHE_INSN:i32=2; const FLUSH_CACHE_BOTH:i32=3; const CAP_SYS_ADMIN:i32=21; const EINVAL:i32=22; const EPERM:i32=1;
const CPU_IS_020_OR_030:bool=false; const CPU_IS_040:bool=false; const CPU_IS_060:bool=false;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
