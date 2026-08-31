/* SPDX-License-Identifier: GPL-2.0-or-later */

// C dependencies: <linux/kernel.h>, <endian.h>
// The original header expects GElf_Ehdr, EI_DATA, and ELFDATA2LSB from ELF
// headers supplied elsewhere.

/*
 * Does a byte swap if target file endianness doesn't match the host, i.e. cross
 * compilation for little endian on big endian and vice versa.
 * To be used for multi-byte values conversion, which are read from / about
 * to be written to a target native endianness ELF file.
 */
#[inline]
pub unsafe fn need_bswap(ehdr: *mut GElf_Ehdr) -> bool {
    cfg!(target_endian = "little") ^ ((*ehdr).e_ident[EI_DATA as usize] == ELFDATA2LSB)
}

pub trait __BswapIfNeeded: Sized {
    fn __bswap_if_needed(self, __need_bswap: bool) -> Self;
}

impl __BswapIfNeeded for u64 {
    #[inline]
    fn __bswap_if_needed(self, __need_bswap: bool) -> Self {
        if __need_bswap {
            self.swap_bytes()
        } else {
            self
        }
    }
}

impl __BswapIfNeeded for i64 {
    #[inline]
    fn __bswap_if_needed(self, __need_bswap: bool) -> Self {
        if __need_bswap {
            (self as u64).swap_bytes() as i64
        } else {
            self
        }
    }
}

impl __BswapIfNeeded for u32 {
    #[inline]
    fn __bswap_if_needed(self, __need_bswap: bool) -> Self {
        if __need_bswap {
            self.swap_bytes()
        } else {
            self
        }
    }
}

impl __BswapIfNeeded for i32 {
    #[inline]
    fn __bswap_if_needed(self, __need_bswap: bool) -> Self {
        if __need_bswap {
            (self as u32).swap_bytes() as i32
        } else {
            self
        }
    }
}

impl __BswapIfNeeded for u16 {
    #[inline]
    fn __bswap_if_needed(self, __need_bswap: bool) -> Self {
        if __need_bswap {
            self.swap_bytes()
        } else {
            self
        }
    }
}

impl __BswapIfNeeded for i16 {
    #[inline]
    fn __bswap_if_needed(self, __need_bswap: bool) -> Self {
        if __need_bswap {
            (self as u16).swap_bytes() as i16
        } else {
            self
        }
    }
}

#[inline]
pub unsafe fn __bswap_if_needed<T: __BswapIfNeeded>(ehdr: *mut GElf_Ehdr, val: T) -> T {
    let __need_bswap: bool = need_bswap(ehdr);
    val.__bswap_if_needed(__need_bswap)
}
