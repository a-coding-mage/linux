/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation. */

#[repr(C)]
pub struct nvram_ops {
    pub get_size: Option<unsafe extern "C" fn() -> isize>,
    pub read_byte: Option<unsafe extern "C" fn(i32) -> u8>,
    pub write_byte: Option<unsafe extern "C" fn(u8, i32)>,
    pub read: Option<unsafe extern "C" fn(*mut i8, usize, *mut i64) -> isize>,
    pub write: Option<unsafe extern "C" fn(*mut i8, usize, *mut i64) -> isize>,
    /* Present when CONFIG_X86 or CONFIG_M68K is enabled. */
    #[cfg(any(CONFIG_X86, CONFIG_M68K))]
    pub initialize: Option<unsafe extern "C" fn() -> isize>,
    #[cfg(any(CONFIG_X86, CONFIG_M68K))]
    pub set_checksum: Option<unsafe extern "C" fn() -> isize>,
}

extern "C" {
    pub static arch_nvram_ops: nvram_ops;
}

/* CONFIG_PPC uses ppc_md.nvram_* hooks instead of arch_nvram_ops hooks. */

#[inline]
pub unsafe fn nvram_get_size() -> isize {
    #[cfg(CONFIG_PPC)]
    {
        if let Some(nvram_size) = ppc_md.nvram_size {
            return nvram_size();
        }
    }
    #[cfg(not(CONFIG_PPC))]
    {
        if let Some(get_size) = arch_nvram_ops.get_size {
            return get_size();
        }
    }
    -19 /* -ENODEV */
}

#[inline]
pub unsafe fn nvram_read_byte(addr: i32) -> u8 {
    #[cfg(CONFIG_PPC)]
    {
        if let Some(read_val) = ppc_md.nvram_read_val {
            return read_val(addr);
        }
    }
    #[cfg(not(CONFIG_PPC))]
    {
        if let Some(read_byte) = arch_nvram_ops.read_byte {
            return read_byte(addr);
        }
    }
    0xFF
}

#[inline]
pub unsafe fn nvram_write_byte(val: u8, addr: i32) {
    #[cfg(CONFIG_PPC)]
    {
        if let Some(write_val) = ppc_md.nvram_write_val {
            write_val(addr, val);
        }
    }
    #[cfg(not(CONFIG_PPC))]
    {
        if let Some(write_byte) = arch_nvram_ops.write_byte {
            write_byte(val, addr);
        }
    }
}

#[inline]
pub unsafe fn nvram_read_bytes(buf: *mut i8, count: usize, ppos: *mut i64) -> isize {
    let nvram_size = nvram_get_size();
    let mut i = *ppos;
    let mut p = buf;
    let mut remaining = count;

    if nvram_size < 0 {
        return nvram_size;
    }
    while remaining > 0 && i < nvram_size {
        *p = nvram_read_byte(i as i32) as i8;
        i += 1;
        p = p.add(1);
        remaining -= 1;
    }
    *ppos = i;
    p.offset_from(buf)
}

#[inline]
pub unsafe fn nvram_write_bytes(buf: *mut i8, count: usize, ppos: *mut i64) -> isize {
    let nvram_size = nvram_get_size();
    let mut i = *ppos;
    let mut p = buf;
    let mut remaining = count;

    if nvram_size < 0 {
        return nvram_size;
    }
    while remaining > 0 && i < nvram_size {
        nvram_write_byte(*p as u8, i as i32);
        i += 1;
        p = p.add(1);
        remaining -= 1;
    }
    *ppos = i;
    p.offset_from(buf)
}

#[inline]
pub unsafe fn nvram_read(buf: *mut i8, count: usize, ppos: *mut i64) -> isize {
    #[cfg(CONFIG_PPC)]
    {
        if let Some(read) = ppc_md.nvram_read {
            return read(buf, count, ppos);
        }
    }
    #[cfg(not(CONFIG_PPC))]
    {
        if let Some(read) = arch_nvram_ops.read {
            return read(buf, count, ppos);
        }
    }
    nvram_read_bytes(buf, count, ppos)
}

#[inline]
pub unsafe fn nvram_write(buf: *mut i8, count: usize, ppos: *mut i64) -> isize {
    #[cfg(CONFIG_PPC)]
    {
        if let Some(write) = ppc_md.nvram_write {
            return write(buf, count, ppos);
        }
    }
    #[cfg(not(CONFIG_PPC))]
    {
        if let Some(write) = arch_nvram_ops.write {
            return write(buf, count, ppos);
        }
    }
    nvram_write_bytes(buf, count, ppos)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
