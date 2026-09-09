/* SPDX-License-Identifier: GPL-2.0 */
// Translated from linux/find.h. Dependencies supplied by linux/bitops.h are external.

unsafe extern "C" {
    pub fn _find_next_bit(addr1: *const usize, nbits: usize, start: usize) -> usize;
    pub fn _find_next_and_bit(addr1: *const usize, addr2: *const usize, nbits: usize, start: usize) -> usize;
    pub fn _find_next_andnot_bit(addr1: *const usize, addr2: *const usize, nbits: usize, start: usize) -> usize;
    pub fn _find_next_or_bit(addr1: *const usize, addr2: *const usize, nbits: usize, start: usize) -> usize;
    pub fn _find_next_zero_bit(addr: *const usize, nbits: usize, start: usize) -> usize;
    pub fn _find_first_bit(addr: *const usize, size: usize) -> usize;
    pub fn __find_nth_bit(addr: *const usize, size: usize, n: usize) -> usize;
    pub fn __find_nth_and_bit(addr1: *const usize, addr2: *const usize, size: usize, n: usize) -> usize;
    pub fn __find_nth_and_andnot_bit(addr1: *const usize, addr2: *const usize, addr3: *const usize, size: usize, n: usize) -> usize;
    pub fn _find_first_and_bit(addr1: *const usize, addr2: *const usize, size: usize) -> usize;
    pub fn _find_first_andnot_bit(addr1: *const usize, addr2: *const usize, size: usize) -> usize;
    pub fn _find_first_and_and_bit(addr1: *const usize, addr2: *const usize, addr3: *const usize, size: usize) -> usize;
    pub fn _find_first_zero_bit(addr: *const usize, size: usize) -> usize;
    pub fn _find_last_bit(addr: *const usize, size: usize) -> usize;
    pub fn find_random_bit(addr: *const usize, size: usize) -> usize;
    pub fn find_next_clump8(clump: *mut usize, addr: *const usize, size: usize, offset: usize) -> usize;
}

#[inline(always)] pub unsafe fn find_next_bit(a: *const usize, s: usize, o: usize) -> usize { _find_next_bit(a,s,o) }
#[inline(always)] pub unsafe fn find_next_and_bit(a: *const usize,b:*const usize,s:usize,o:usize)->usize{_find_next_and_bit(a,b,s,o)}
#[inline(always)] pub unsafe fn find_next_andnot_bit(a:*const usize,b:*const usize,s:usize,o:usize)->usize{_find_next_andnot_bit(a,b,s,o)}
#[inline(always)] pub unsafe fn find_next_or_bit(a:*const usize,b:*const usize,s:usize,o:usize)->usize{_find_next_or_bit(a,b,s,o)}
#[inline(always)] pub unsafe fn find_next_zero_bit(a:*const usize,s:usize,o:usize)->usize{_find_next_zero_bit(a,s,o)}
#[inline(always)] pub unsafe fn find_first_bit(a:*const usize,s:usize)->usize{_find_first_bit(a,s)}
#[inline(always)] pub unsafe fn find_nth_bit(a:*const usize,s:usize,n:usize)->usize{if n>=s{s}else{__find_nth_bit(a,s,n)}}
#[inline(always)] pub unsafe fn find_nth_and_bit(a:*const usize,b:*const usize,s:usize,n:usize)->usize{if n>=s{s}else{__find_nth_and_bit(a,b,s,n)}}
#[inline(always)] pub unsafe fn find_nth_and_andnot_bit(a:*const usize,b:*const usize,c:*const usize,s:usize,n:usize)->usize{if n>=s{s}else{__find_nth_and_andnot_bit(a,b,c,s,n)}}
#[inline(always)] pub unsafe fn find_first_and_bit(a:*const usize,b:*const usize,s:usize)->usize{_find_first_and_bit(a,b,s)}
#[inline(always)] pub unsafe fn find_first_andnot_bit(a:*const usize,b:*const usize,s:usize)->usize{_find_first_andnot_bit(a,b,s)}
#[inline(always)] pub unsafe fn find_first_and_and_bit(a:*const usize,b:*const usize,c:*const usize,s:usize)->usize{_find_first_and_and_bit(a,b,c,s)}
#[inline(always)] pub unsafe fn find_first_zero_bit(a:*const usize,s:usize)->usize{_find_first_zero_bit(a,s)}
#[inline(always)] pub unsafe fn find_last_bit(a:*const usize,s:usize)->usize{_find_last_bit(a,s)}

#[inline(always)] pub unsafe fn find_next_and_bit_wrap(a:*const usize,b:*const usize,s:usize,o:usize)->usize{let bit=find_next_and_bit(a,b,s,o);if bit<s||o==0{bit}else{let x=find_first_and_bit(a,b,o);if x<o{x}else{s}}}
#[inline(always)] pub unsafe fn find_next_bit_wrap(a:*const usize,s:usize,o:usize)->usize{let bit=find_next_bit(a,s,o);if bit<s||o==0{bit}else{let x=find_first_bit(a,o);if x<o{x}else{s}}}
#[inline(always)] pub unsafe fn __for_each_wrap(a:*const usize,s:usize,start:usize,mut n:usize)->usize{let mut bit;if n>start{bit=find_next_bit(a,s,n);if bit<s{return bit;}n=0;}bit=find_next_bit(a,start,n);if bit<start{bit}else{s}}

#[inline(always)] pub unsafe fn find_first_clump8(c:*mut usize,b:*const usize,s:usize)->usize{find_next_clump8(c,b,s,0)}

/* C iteration macros, expressed as Rust macros. */
#[macro_export] macro_rules! for_each_set_bit { ($bit:ident,$addr:expr,$size:expr,$body:block) => {{ $bit=0; loop{$bit=unsafe{$crate::find_next_bit($addr,$size,$bit)};if $bit >= $size{break;}$body;$bit+=1;}}}; }
#[macro_export] macro_rules! for_each_set_bit_wrap { ($bit:ident,$addr:expr,$size:expr,$start:expr,$body:block) => {{ $bit=unsafe{$crate::find_next_bit_wrap($addr,$size,$start)};while $bit<$size{$body;$bit=unsafe{$crate::__for_each_wrap($addr,$size,$start,$bit+1)};}}}; }

#[cfg(target_endian = "little")]
#[inline(always)] pub unsafe fn find_next_zero_bit_le(a:*const usize,s:usize,o:usize)->usize{find_next_zero_bit(a,s,o)}
#[cfg(target_endian = "little")]
#[inline(always)] pub unsafe fn find_next_bit_le(a:*const usize,s:usize,o:usize)->usize{find_next_bit(a,s,o)}
#[cfg(target_endian = "little")]
#[inline(always)] pub unsafe fn find_first_zero_bit_le(a:*const usize,s:usize)->usize{find_first_zero_bit(a,s)}
#[cfg(target_endian = "big")]
unsafe extern "C" { pub fn _find_next_zero_bit_le(a:*const core::ffi::c_void,s:usize,o:usize)->usize; pub fn _find_first_zero_bit_le(a:*const core::ffi::c_void,s:usize)->usize; pub fn _find_next_bit_le(a:*const core::ffi::c_void,s:usize,o:usize)->usize; }

#[macro_export] macro_rules! find_first_clump8 { ($clump:expr,$bits:expr,$size:expr) => { unsafe{$crate::find_next_clump8($clump,$bits,$size,0)} }; }
#[macro_export] macro_rules! for_each_and_bit { ($bit:ident,$a:expr,$b:expr,$size:expr,$body:block) => {{ $bit=0;loop{$bit=unsafe{$crate::find_next_and_bit($a,$b,$size,$bit)};if $bit >= $size{break;}$body;$bit+=1;}}}; }
#[macro_export] macro_rules! for_each_andnot_bit { ($bit:ident,$a:expr,$b:expr,$size:expr,$body:block) => {{ $bit=0;loop{$bit=unsafe{$crate::find_next_andnot_bit($a,$b,$size,$bit)};if $bit >= $size{break;}$body;$bit+=1;}}}; }
#[macro_export] macro_rules! for_each_or_bit { ($bit:ident,$a:expr,$b:expr,$size:expr,$body:block) => {{ $bit=0;loop{$bit=unsafe{$crate::find_next_or_bit($a,$b,$size,$bit)};if $bit >= $size{break;}$body;$bit+=1;}}}; }
#[macro_export] macro_rules! for_each_clear_bit { ($bit:ident,$a:expr,$size:expr,$body:block) => {{ $bit=0;loop{$bit=unsafe{$crate::find_next_zero_bit($a,$size,$bit)};if $bit >= $size{break;}$body;$bit+=1;}}}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
