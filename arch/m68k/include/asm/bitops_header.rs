//! Rust translation of the m68k `bitops.h` header.
// The original includes and build-time configuration are supplied externally.

#[inline]
pub unsafe fn bset_reg_set_bit(nr: i32, vaddr: *mut core::ffi::c_ulong) {
    let p = (vaddr as *mut u8).offset(((nr ^ 31) / 8) as isize);
    core::arch::asm!("bset {1},({0})", in("a") p, in("di") (nr & 7), options(nostack));
}

#[inline]
pub unsafe fn bset_mem_set_bit(nr: i32, vaddr: *mut core::ffi::c_ulong) {
    let p = (vaddr as *mut u8).offset(((nr ^ 31) / 8) as isize);
    core::arch::asm!("bset {1},{0}", inout(reg_byte) *p, in("di") (nr & 7), options(nostack));
}

#[inline]
pub unsafe fn bfset_mem_set_bit(nr: i32, vaddr: *mut core::ffi::c_ulong) {
    core::arch::asm!("bfset {1}{{{0}:#1}}", in("d") (nr ^ 31), inout("o") *vaddr, options(nostack));
}

#[inline(always)]
pub unsafe fn arch___set_bit(nr: core::ffi::c_ulong, addr: *mut core::ffi::c_ulong) {
    #[cfg(CONFIG_COLDFIRE)] { bset_reg_set_bit(nr as i32, addr); }
    #[cfg(all(not(CONFIG_COLDFIRE), CONFIG_CPU_HAS_NO_BITFIELDS))] { bset_mem_set_bit(nr as i32, addr); }
    #[cfg(all(not(CONFIG_COLDFIRE), not(CONFIG_CPU_HAS_NO_BITFIELDS)))] { bset_mem_set_bit(nr as i32, addr); }
}

#[inline]
pub unsafe fn bclr_reg_clear_bit(nr: i32, vaddr: *mut core::ffi::c_ulong) { let p=(vaddr as *mut u8).offset(((nr^31)/8) as isize); core::arch::asm!("bclr {1},({0})", in("a") p, in("di") (nr&7), options(nostack)); }
#[inline]
pub unsafe fn bclr_mem_clear_bit(nr: i32, vaddr: *mut core::ffi::c_ulong) { let p=(vaddr as *mut u8).offset(((nr^31)/8) as isize); core::arch::asm!("bclr {1},{0}", inout(reg_byte) *p, in("di") (nr&7), options(nostack)); }
#[inline]
pub unsafe fn bfclr_mem_clear_bit(nr: i32, vaddr: *mut core::ffi::c_ulong) { core::arch::asm!("bfclr {1}{{{0}:#1}}", in("d") (nr^31), inout("o") *vaddr, options(nostack)); }
#[inline(always)]
pub unsafe fn arch___clear_bit(nr: core::ffi::c_ulong, addr: *mut core::ffi::c_ulong) { #[cfg(CONFIG_COLDFIRE)] { bclr_reg_clear_bit(nr as i32,addr); } #[cfg(not(CONFIG_COLDFIRE))] { bclr_mem_clear_bit(nr as i32,addr); } }

#[inline]
pub unsafe fn bchg_reg_change_bit(nr:i32,vaddr:*mut core::ffi::c_ulong){let p=(vaddr as *mut u8).offset(((nr^31)/8)as isize);core::arch::asm!("bchg {1},({0})",in("a")p,in("di")(nr&7),options(nostack));}
#[inline]
pub unsafe fn bchg_mem_change_bit(nr:i32,vaddr:*mut core::ffi::c_ulong){let p=(vaddr as *mut u8).offset(((nr^31)/8)as isize);core::arch::asm!("bchg {1},{0}",inout(reg_byte)*p,in("di")(nr&7),options(nostack));}
#[inline]
pub unsafe fn bfchg_mem_change_bit(nr:i32,vaddr:*mut core::ffi::c_ulong){core::arch::asm!("bfchg {1}{{{0}:#1}}",in("d")(nr^31),inout("o")*vaddr,options(nostack));}
#[inline(always)]
pub unsafe fn arch___change_bit(nr:core::ffi::c_ulong,addr:*mut core::ffi::c_ulong){#[cfg(CONFIG_COLDFIRE)]{bchg_reg_change_bit(nr as i32,addr);}#[cfg(not(CONFIG_COLDFIRE))]{bchg_mem_change_bit(nr as i32,addr);}}

pub unsafe fn bset_reg_test_and_set_bit(nr:i32,vaddr:*mut core::ffi::c_ulong)->i32{let p=(vaddr as *mut u8).offset(((nr^31)/8)as isize);let mut r:i8;core::arch::asm!("bset {2},({1}); sne {0}",out(reg_byte)r,in("a")p,in("di")(nr&7),options(nostack));r as i32}
pub unsafe fn bset_mem_test_and_set_bit(nr:i32,vaddr:*mut core::ffi::c_ulong)->i32{let p=(vaddr as *mut u8).offset(((nr^31)/8)as isize);let mut r:i8;core::arch::asm!("bset {2},{1}; sne {0}",out(reg_byte)r,inout(reg_byte)*p,in("di")(nr&7),options(nostack));r as i32}
pub unsafe fn bfset_mem_test_and_set_bit(nr:i32,vaddr:*mut core::ffi::c_ulong)->i32{let mut r:i8;core::arch::asm!("bfset {2}{{{1}:#1}}; sne {0}",out(reg_byte)r,in("d")(nr^31),inout("o")*vaddr,options(nostack));r as i32}
pub unsafe fn bclr_reg_test_and_clear_bit(nr:i32,vaddr:*mut core::ffi::c_ulong)->i32{bset_reg_test_and_set_bit(nr,vaddr)}
pub unsafe fn bclr_mem_test_and_clear_bit(nr:i32,vaddr:*mut core::ffi::c_ulong)->i32{bset_mem_test_and_set_bit(nr,vaddr)}
pub unsafe fn bfclr_mem_test_and_clear_bit(nr:i32,vaddr:*mut core::ffi::c_ulong)->i32{bfset_mem_test_and_set_bit(nr,vaddr)}
pub unsafe fn bchg_reg_test_and_change_bit(nr:i32,vaddr:*mut core::ffi::c_ulong)->i32{bset_reg_test_and_set_bit(nr,vaddr)}
pub unsafe fn bchg_mem_test_and_change_bit(nr:i32,vaddr:*mut core::ffi::c_ulong)->i32{bset_mem_test_and_set_bit(nr,vaddr)}
pub unsafe fn bfchg_mem_test_and_change_bit(nr:i32,vaddr:*mut core::ffi::c_ulong)->i32{bfset_mem_test_and_set_bit(nr,vaddr)}
#[inline(always)] pub unsafe fn arch___test_and_set_bit(nr:core::ffi::c_ulong,a:*mut core::ffi::c_ulong)->bool{bset_mem_test_and_set_bit(nr as i32,a)!=0}
#[inline(always)] pub unsafe fn arch___test_and_clear_bit(nr:core::ffi::c_ulong,a:*mut core::ffi::c_ulong)->bool{bclr_mem_test_and_clear_bit(nr as i32,a)!=0}
#[inline(always)] pub unsafe fn arch___test_and_change_bit(nr:core::ffi::c_ulong,a:*mut core::ffi::c_ulong)->bool{bchg_mem_test_and_change_bit(nr as i32,a)!=0}

pub unsafe fn xor_unlock_is_negative_byte(mask:core::ffi::c_ulong,p:*mut core::ffi::c_ulong)->bool { let q=(p as *mut u8).offset(3); let mut r:i8; core::arch::asm!("eor.b {1},{2}; smi {0}",out(reg_byte)r,in("di")mask,inout(reg_byte)*q,options(nostack)); r<0 }

// The generic bit-search declarations are provided by asm-generic/bitops when CONFIG_CPU_HAS_NO_BITFIELDS is set.
#[inline] pub fn find_first_zero_bit(vaddr:*const core::ffi::c_ulong,size:core::ffi::c_ulong)->core::ffi::c_ulong { find_next_zero_bit(vaddr,size,0) }
#[inline] pub fn find_next_zero_bit(vaddr:*const core::ffi::c_ulong,size:core::ffi::c_ulong,offset:core::ffi::c_ulong)->core::ffi::c_ulong { let mut i=offset; while i<size { if unsafe{(*vaddr.add((i/32)as usize))&(1<<(31-(i&31)))==0}{return i;} i+=1;} size }
#[inline] pub fn find_first_bit(vaddr:*const core::ffi::c_ulong,size:core::ffi::c_ulong)->core::ffi::c_ulong { find_next_bit(vaddr,size,0) }
#[inline] pub fn find_next_bit(vaddr:*const core::ffi::c_ulong,size:core::ffi::c_ulong,offset:core::ffi::c_ulong)->core::ffi::c_ulong { let mut i=offset; while i<size { if unsafe{(*vaddr.add((i/32)as usize))&(1<<(31-(i&31)))!=0}{return i;} i+=1;} size }
#[inline] pub const fn ffz(word:core::ffi::c_ulong)->core::ffi::c_ulong { (!word).trailing_zeros() as core::ffi::c_ulong }

pub const fn ffs(x:i32)->i32 { if x==0 {0} else {32-x.trailing_zeros() as i32} }
pub const fn __ffs(x:core::ffi::c_ulong)->core::ffi::c_ulong { x.trailing_zeros() as core::ffi::c_ulong }
pub const fn fls(x:u32)->i32 { if x==0 {0} else {32-x.leading_zeros() as i32} }
pub const fn __fls(x:core::ffi::c_ulong)->core::ffi::c_ulong { (core::mem::size_of::<core::ffi::c_ulong>()*8-1-x.leading_zeros() as usize) as core::ffi::c_ulong }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
