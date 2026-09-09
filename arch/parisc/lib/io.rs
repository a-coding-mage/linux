// SPDX-License-Identifier: GPL-2.0
/*
 * arch/parisc/lib/io.c
 *
 * IO accessing functions which shouldn't be inlined because they're too big
 */

extern "C" {
    fn inb(port: usize) -> u8;
    fn inw(port: usize) -> u16;
    fn inl(port: usize) -> u32;
    fn outb(value: u8, port: usize);
    fn outw(value: u16, port: usize);
    fn outl(value: u32, port: usize);
    fn cpu_to_le16(value: u16) -> u16;
    fn cpu_to_le32(value: u32) -> u32;
    fn le16_to_cpu(value: u16) -> u16;
    fn le32_to_cpu(value: u32) -> u32;
}

pub unsafe fn insb(port: usize, dst: *mut core::ffi::c_void, mut count: usize) {
    let mut p = dst as *mut u8;
    while (p as usize) & 0x3 != 0 {
        if count == 0 { return; }
        count -= 1;
        *p = inb(port);
        p = p.add(1);
    }
    while count >= 4 {
        count -= 4;
        let mut w = (inb(port) as u32) << 24;
        w |= (inb(port) as u32) << 16;
        w |= (inb(port) as u32) << 8;
        w |= inb(port) as u32;
        *(p as *mut u32) = w;
        p = p.add(4);
    }
    while count != 0 {
        count -= 1;
        *p = inb(port);
        p = p.add(1);
    }
}

pub unsafe fn insw(port: usize, dst: *mut core::ffi::c_void, mut count: usize) {
    let mut l: u32 = 0;
    let mut p = dst as *mut u8;
    if count == 0 { return; }
    match (p as usize) & 0x3 {
        0 => {
            while count >= 2 { count -= 2; l = (cpu_to_le16(inw(port)) as u32) << 16; l |= cpu_to_le16(inw(port)) as u32; *(p as *mut u32) = l; p = p.add(4); }
            if count != 0 { *(p as *mut u16) = cpu_to_le16(inw(port)); }
        },
        2 => {
            *(p as *mut u16) = cpu_to_le16(inw(port)); p = p.add(2); count -= 1;
            while count >= 2 { count -= 2; l = (cpu_to_le16(inw(port)) as u32) << 16; l |= cpu_to_le16(inw(port)) as u32; *(p as *mut u32) = l; p = p.add(4); }
            if count != 0 { *(p as *mut u16) = cpu_to_le16(inw(port)); }
        },
        1 | 3 => {
            count -= 1; l = cpu_to_le16(inw(port)) as u32; *p = (l >> 8) as u8; p = p.add(1);
            while count != 0 { count -= 1; let l2 = cpu_to_le16(inw(port)) as u32; *(p as *mut u16) = ((l & 0xff) << 8 | l2 >> 8) as u16; p = p.add(2); l = l2; }
            *p = (l & 0xff) as u8;
        }, _ => unreachable!()
    }
}

pub unsafe fn insl(port: usize, dst: *mut core::ffi::c_void, mut count: usize) {
    let mut l: u32 = 0; let mut p = dst as *mut u8;
    if count == 0 { return; }
    match (dst as usize) & 3 {
        0 => while count != 0 { count -= 1; *(p as *mut u32) = cpu_to_le32(inl(port)); p = p.add(4); },
        2 => { count -= 1; l = cpu_to_le32(inl(port)); *(p as *mut u16) = (l >> 16) as u16; p = p.add(2); while count != 0 { count -= 1; let l2 = cpu_to_le32(inl(port)); *(p as *mut u32) = (l & 0xffff) << 16 | l2 >> 16; p = p.add(4); l = l2; } *(p as *mut u16) = (l & 0xffff) as u16; },
        1 => { count -= 1; l = cpu_to_le32(inl(port)); *p = (l >> 24) as u8; p = p.add(1); *(p as *mut u16) = (l >> 8) as u16; p = p.add(2); while count != 0 { count -= 1; let l2 = cpu_to_le32(inl(port)); *(p as *mut u32) = (l & 0xff) << 24 | l2 >> 8; p = p.add(4); l = l2; } *p = l as u8; },
        3 => { count -= 1; l = cpu_to_le32(inl(port)); *p = (l >> 24) as u8; p = p.add(1); while count != 0 { count -= 1; let l2 = cpu_to_le32(inl(port)); *(p as *mut u32) = (l & 0xffffff) << 8 | l2 >> 24; p = p.add(4); l = l2; } *(p as *mut u16) = (l >> 8) as u16; p = p.add(2); *p = l as u8; }, _ => unreachable!()
    }
}

pub unsafe fn outsb(port: usize, src: *const core::ffi::c_void, mut count: usize) { let mut p = src as *const u8; while count != 0 { count -= 1; outb(*p, port); p = p.add(1); } }

pub unsafe fn outsw(port: usize, src: *const core::ffi::c_void, mut count: usize) {
    let mut p = src as *const u8; if count == 0 { return; }
    match (p as usize) & 3 {
        0 => { while count >= 2 { count -= 2; let l = *(p as *const u32); p = p.add(4); outw(le16_to_cpu((l >> 16) as u16), port); outw(le16_to_cpu(l as u16), port); } if count != 0 { outw(le16_to_cpu(*(p as *const u16)), port); } },
        2 => { outw(le16_to_cpu(*(p as *const u16)), port); p = p.add(2); count -= 1; while count >= 2 { count -= 2; let l = *(p as *const u32); p = p.add(4); outw(le16_to_cpu((l >> 16) as u16), port); outw(le16_to_cpu(l as u16), port); } if count != 0 { outw(le16_to_cpu(*(p as *const u16)), port); } },
        1 => { let mut l = (*p as u32) << 8; p = p.add(1); count -= 1; while count != 0 { count -= 1; let l2 = *(p as *const u16) as u32; p = p.add(2); outw(le16_to_cpu((l | l2 >> 8) as u16), port); l = l2 << 8; } outw(le16_to_cpu((l | *p as u32 >> 8) as u16), port); },
        _ => {}
    }
}

pub unsafe fn outsl(port: usize, src: *const core::ffi::c_void, mut count: usize) {
    let mut p = src as *const u8; if count == 0 { return; }
    match (p as usize) & 3 {
        0 => while count != 0 { count -= 1; outl(le32_to_cpu(*(p as *const u32)), port); p = p.add(4); },
        2 => { count -= 1; let mut l = *(p as *const u16) as u32; p = p.add(2); while count != 0 { count -= 1; let l2 = *(p as *const u32); p = p.add(4); outl(le32_to_cpu(l << 16 | l2 >> 16), port); l = l2; } let l2 = *(p as *const u16) as u32; outl(le32_to_cpu(l << 16 | l2), port); },
        1 => { count -= 1; let mut l = (*p as u32) << 24; p = p.add(1); l |= *(p as *const u16) as u32 << 8; p = p.add(2); while count != 0 { count -= 1; let l2 = *(p as *const u32); p = p.add(4); outl(le32_to_cpu(l | l2 >> 24), port); l = l2 << 8; } outl(le32_to_cpu(l | *p as u32), port); },
        3 => { count -= 1; let mut l = (*p as u32) << 24; p = p.add(1); while count != 0 { count -= 1; let l2 = *(p as *const u32); p = p.add(4); outl(le32_to_cpu(l | l2 >> 8), port); l = l2 << 24; } let mut l2 = *(p as *const u16) as u32 << 16; p = p.add(2); l2 |= *p as u32; outl(le32_to_cpu(l | l2), port); }, _ => {}
    }
}

// EXPORT_SYMBOL(insb), EXPORT_SYMBOL(insw), EXPORT_SYMBOL(insl),
// EXPORT_SYMBOL(outsb), EXPORT_SYMBOL(outsw), EXPORT_SYMBOL(outsl)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
