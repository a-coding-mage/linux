// SPDX-License-Identifier: GPL-2.0
/*
 *  Copyright (c) 1996-2000 Russell King.
 *
 *  Scan ADFS partitions on hard disk drives.  Unfortunately, there
 *  isn't a standard for partitioning drives on Acorn machines, so
 *  every single manufacturer of SCSI and IDE cards created their own
 *  method.
 */

// Dependencies supplied by the surrounding kernel translation unit.

const PARTITION_RISCIX_MFM: u32 = 1;
const PARTITION_RISCIX_SCSI: u32 = 2;
const PARTITION_LINUX: u32 = 9;

#[cfg(any(CONFIG_ACORN_PARTITION_CUMANA, CONFIG_ACORN_PARTITION_ADFS))]
unsafe fn adfs_partition(
    state: *mut parsed_partitions, name: *mut std::ffi::c_char,
    data: *mut std::ffi::c_char, first_sector: u64, slot: i32,
) -> *mut adfs_discrecord {
    if adfs_checkbblk(data) != 0 { return core::ptr::null_mut(); }
    let dr = (data as *mut u8).add(0x1c0) as *mut adfs_discrecord;
    if (*dr).disc_size == 0 && (*dr).disc_size_high == 0 { return core::ptr::null_mut(); }
    let nr_sects = (u32::from(le32_to_cpu((*dr).disc_size_high)) << 23)
        | (u32::from(le32_to_cpu((*dr).disc_size)) >> 9);
    if !name.is_null() { seq_buf_printf(&mut (*state).pp_buf, " [%s]", name); }
    put_partition(state, slot, first_sector, nr_sects as u64);
    dr
}

#[cfg(CONFIG_ACORN_PARTITION_RISCIX)]
#[repr(C)]
struct riscix_part { start: u32, length: u32, one: u32, name: [u8; 16] }
#[cfg(CONFIG_ACORN_PARTITION_RISCIX)]
#[repr(C)]
struct riscix_record { magic: u32, date: u32, part: [riscix_part; 8] }

#[cfg(all(CONFIG_ACORN_PARTITION_RISCIX, any(CONFIG_ACORN_PARTITION_CUMANA, CONFIG_ACORN_PARTITION_ADFS)))]
unsafe fn riscix_partition(state: *mut parsed_partitions, first_sect: u64, mut slot: i32, nr_sects: u64) -> i32 {
    let mut sect = core::mem::MaybeUninit::<Sector>::uninit();
    let rr = read_part_sector(state, first_sect, sect.as_mut_ptr()) as *mut riscix_record;
    if rr.is_null() { return -1; }
    seq_buf_puts(&mut (*state).pp_buf, " [RISCiX]");
    if (*rr).magic == cpu_to_le32(0x4a657320) {
        let size = core::cmp::min(nr_sects, 2);
        seq_buf_puts(&mut (*state).pp_buf, " <");
        put_partition(state, slot, first_sect, size); slot += 1;
        for part in 0..8 {
            let p = &(*rr).part[part];
            if p.one != 0 && core::slice::from_raw_parts(p.name.as_ptr(), 4) != b"All\0" {
                put_partition(state, slot, le32_to_cpu(p.start) as u64, le32_to_cpu(p.length) as u64); slot += 1;
                seq_buf_printf(&mut (*state).pp_buf, "(%s)", p.name.as_ptr());
            }
        }
        seq_buf_puts(&mut (*state).pp_buf, " >\n");
    } else { put_partition(state, slot, first_sect, nr_sects); slot += 1; }
    put_dev_sector(sect.assume_init()); slot
}

const LINUX_NATIVE_MAGIC: u32 = 0xdeafa1de;
const LINUX_SWAP_MAGIC: u32 = 0xdeafab1e;
#[repr(C)] struct linux_part { magic: u32, start_sect: u32, nr_sects: u32 }

#[cfg(any(CONFIG_ACORN_PARTITION_CUMANA, CONFIG_ACORN_PARTITION_ADFS))]
unsafe fn linux_partition(state: *mut parsed_partitions, first_sect: u64, mut slot: i32, nr_sects: u64) -> i32 {
    let size = core::cmp::min(nr_sects, 2);
    seq_buf_puts(&mut (*state).pp_buf, " [Linux]");
    put_partition(state, slot, first_sect, size); slot += 1;
    let mut sect = core::mem::MaybeUninit::<Sector>::uninit();
    let mut linuxp = read_part_sector(state, first_sect, sect.as_mut_ptr()) as *mut linux_part;
    if linuxp.is_null() { return -1; }
    seq_buf_puts(&mut (*state).pp_buf, " <");
    while (*linuxp).magic == cpu_to_le32(LINUX_NATIVE_MAGIC) || (*linuxp).magic == cpu_to_le32(LINUX_SWAP_MAGIC) {
        if slot == (*state).limit { break; }
        put_partition(state, slot, first_sect + le32_to_cpu((*linuxp).start_sect) as u64, le32_to_cpu((*linuxp).nr_sects) as u64);
        slot += 1; linuxp = linuxp.add(1);
    }
    seq_buf_puts(&mut (*state).pp_buf, " >");
    put_dev_sector(sect.assume_init()); slot
}

#[cfg(CONFIG_ACORN_PARTITION_CUMANA)]
pub unsafe fn adfspart_check_CUMANA(state: *mut parsed_partitions) -> i32 {
    let mut first_sector = 0u64; let mut start_blk = 0u32; let mut sect = core::mem::MaybeUninit::<Sector>::uninit();
    let mut name: *mut std::ffi::c_char = b"CUMANA/ADFS\0".as_ptr() as *mut _; let mut first = 1; let mut slot = 1;
    loop {
        let data = read_part_sector(state, start_blk as u64 * 2 + 6, sect.as_mut_ptr()) as *mut u8;
        if data.is_null() { return -1; } if slot == (*state).limit { break; }
        let dr = adfs_partition(state, name, data as *mut _, first_sector, slot); slot += 1; if dr.is_null() { break; } name = core::ptr::null_mut();
        let mut nr = (data[0x1fd] as u32 | (data[0x1fe] as u32) << 8) * ((*dr).heads as u32 + if (*dr).lowsector & 0x40 != 0 { 1 } else { 0 }) * (*dr).secspertrack as u32;
        if nr == 0 { break; } first = 0; first_sector += nr as u64; start_blk += nr >> (BLOCK_SIZE_BITS - 9); nr = 0;
        match data[0x1fc] & 15 { 0 => {},
            #[cfg(CONFIG_ACORN_PARTITION_RISCIX)] PARTITION_RISCIX_SCSI => { slot = riscix_partition(state, first_sector, slot, nr as u64); },
            PARTITION_LINUX => { slot = linux_partition(state, first_sector, slot, nr as u64); }, _ => {}
        }
        put_dev_sector(sect.assume_init()); if slot == -1 { return -1; }
    }
    put_dev_sector(sect.assume_init()); if first != 0 { 0 } else { 1 }
}

#[repr(C)] struct adfs_discrecord { disc_size: u32, disc_size_high: u32, heads: u8, lowsector: u8, secspertrack: u8 }
#[repr(C)] struct parsed_partitions { pp_buf: seq_buf, limit: i32, disk: *mut disk }
#[repr(C)] struct seq_buf { _private: [u8; 0] }
#[repr(C)] struct Sector { _private: [u8; 0] }
#[repr(C)] struct disk { _private: [u8; 0] }
extern "C" { fn adfs_checkbblk(data: *mut std::ffi::c_char) -> i32; fn le32_to_cpu(x: u32) -> u32; fn cpu_to_le32(x: u32) -> u32; fn seq_buf_printf(_: *mut seq_buf, _: &str, ...); fn seq_buf_puts(_: *mut seq_buf, _: &str); fn put_partition(_: *mut parsed_partitions, _: i32, _: u64, _: u64); fn read_part_sector(_: *mut parsed_partitions, _: u64, _: *mut Sector) -> *mut u8; fn put_dev_sector(_: Sector); fn get_capacity(_: *mut disk) -> u64; }
const BLOCK_SIZE_BITS: u32 = 12;

// The remaining format-specific scanners retain the original layouts and algorithms.
#[cfg(CONFIG_ACORN_PARTITION_ADFS)]
pub unsafe fn adfspart_check_ADFS(state: *mut parsed_partitions) -> i32 {
    let mut sect = core::mem::MaybeUninit::<Sector>::uninit();
    let data = read_part_sector(state, 6, sect.as_mut_ptr()); if data.is_null() { return -1; }
    let dr = adfs_partition(state, b"ADFS\0".as_ptr() as *mut _, data as *mut _, 0, 1);
    if dr.is_null() { put_dev_sector(sect.assume_init()); return 0; }
    let heads = (*dr).heads as u64 + (((*dr).lowsector >> 6) & 1) as u64;
    let sectscyl = (*dr).secspertrack as u64 * heads;
    let start = (((*data.add(0x1fe) as u64) << 8) + *data.add(0x1fd) as u64) * sectscyl;
    let id = *data.add(0x1fc) & 15; put_dev_sector(sect.assume_init());
    let nr = get_capacity((*state).disk) - start;
    if start != 0 { match id as u32 {
        #[cfg(CONFIG_ACORN_PARTITION_RISCIX)] PARTITION_RISCIX_SCSI | PARTITION_RISCIX_MFM => { riscix_partition(state, start, 2, nr); },
        PARTITION_LINUX => { linux_partition(state, start, 2, nr); }, _ => {}
    }}
    seq_buf_puts(&mut (*state).pp_buf, "\n"); 1
}

#[cfg(CONFIG_ACORN_PARTITION_ICS)]
#[repr(C)] struct ics_part { start: u32, size: u32 }
#[cfg(CONFIG_ACORN_PARTITION_ICS)]
unsafe fn adfspart_check_icslinux(state: *mut parsed_partitions, block: u64) -> i32 {
    let mut s = core::mem::MaybeUninit::<Sector>::uninit(); let d = read_part_sector(state, block, s.as_mut_ptr()); let mut r = 0;
    if !d.is_null() { if core::slice::from_raw_parts(d, 9) == b"LinuxPart" { r = 1; } put_dev_sector(s.assume_init()); } r
}
#[cfg(CONFIG_ACORN_PARTITION_ICS)]
unsafe fn valid_ics_sector(data: *const u8) -> bool {
    let mut sum = 0x50617274u32; for i in 0..508 { sum = sum.wrapping_add(*data.add(i) as u32); }
    sum.wrapping_sub(le32_to_cpu(core::ptr::read_unaligned(data.add(508) as *const u32))) == 0
}
#[cfg(CONFIG_ACORN_PARTITION_ICS)]
pub unsafe fn adfspart_check_ICS(state: *mut parsed_partitions) -> i32 {
    let mut s = core::mem::MaybeUninit::<Sector>::uninit(); let data = read_part_sector(state, 0, s.as_mut_ptr()); if data.is_null() { return -1; }
    if !valid_ics_sector(data) { put_dev_sector(s.assume_init()); return 0; } seq_buf_puts(&mut (*state).pp_buf, " [ICS]");
    let mut slot = 1; let mut p = data as *const ics_part;
    while (*p).size != 0 { let mut start = le32_to_cpu((*p).start) as u64; let mut size = le32_to_cpu((*p).size) as i32; if slot == (*state).limit { break; }
        if size < 0 { size = -size; if size > 1 && adfspart_check_icslinux(state, start) != 0 { start += 1; size -= 1; } }
        if size != 0 { put_partition(state, slot, start, size as u64); slot += 1; } p = p.add(1);
    } put_dev_sector(s.assume_init()); seq_buf_puts(&mut (*state).pp_buf, "\n"); 1
}

#[cfg(CONFIG_ACORN_PARTITION_POWERTEC)]
#[repr(C)] struct ptec_part { unused1:u32, unused2:u32, start:u32, size:u32, unused5:u32, type_: [u8;8] }
#[cfg(CONFIG_ACORN_PARTITION_POWERTEC)]
unsafe fn valid_ptec_sector(d:*const u8)->bool { if *d.add(510)==0x55 && *d.add(511)==0xaa { return false; } let mut c=0x2au8; for i in 0..511 { c=c.wrapping_add(*d.add(i)); } c==*d.add(511) }
#[cfg(CONFIG_ACORN_PARTITION_POWERTEC)]
pub unsafe fn adfspart_check_POWERTEC(state:*mut parsed_partitions)->i32 { let mut s=core::mem::MaybeUninit::<Sector>::uninit(); let d=read_part_sector(state,0,s.as_mut_ptr()); if d.is_null(){return -1} if !valid_ptec_sector(d){put_dev_sector(s.assume_init());return 0} seq_buf_puts(&mut (*state).pp_buf," [POWERTEC]"); let p=d as *const ptec_part; for i in 0..12 {let q=p.add(i);if (*q).size!=0{put_partition(state,i+1,le32_to_cpu((*q).start) as u64,le32_to_cpu((*q).size) as u64)}} put_dev_sector(s.assume_init());seq_buf_puts(&mut (*state).pp_buf,"\n");1 }

#[cfg(CONFIG_ACORN_PARTITION_EESOX)]
#[repr(C)] struct eesox_part { magic:[u8;6], name:[u8;10], start:u32, unused6:u32, unused7:u32, unused8:u32 }
#[cfg(CONFIG_ACORN_PARTITION_EESOX)]
pub unsafe fn adfspart_check_EESOX(state:*mut parsed_partitions)->i32 {
    let key=b"Neil Critchell  "; let mut s=core::mem::MaybeUninit::<Sector>::uninit(); let d=read_part_sector(state,7,s.as_mut_ptr()); if d.is_null(){return -1}
    let mut b=[0u8;256]; for i in 0..256 {b[i]=*d.add(i)^key[i&15]} put_dev_sector(s.assume_init()); let p=b.as_ptr() as *const eesox_part; let mut start=0u64; let mut slot=1; let mut i=0;
    while i<8 {let q=p.add(i);if core::slice::from_raw_parts((*q).magic.as_ptr(),6)!=b"Eesox\0" {break} let next=le32_to_cpu((*q).start) as u64;if i!=0{put_partition(state,slot,start,next-start);slot+=1}start=next;i+=1}
    if i!=0 {put_partition(state,slot,start,get_capacity((*state).disk)-start);seq_buf_puts(&mut (*state).pp_buf,"\n");1}else{0}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
