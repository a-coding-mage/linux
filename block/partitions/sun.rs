// SPDX-License-Identifier: GPL-2.0
/*
 *  fs/partitions/sun.c
 *
 *  Code extracted from drivers/block/genhd.c
 *
 *  Copyright (C) 1991-1998  Linus Torvalds
 *  Re-organised Feb 1998 Russell King
 */

// Dependency supplied by the surrounding repository: "check.h".

const SUN_LABEL_MAGIC: u16 = 0xDABE;
const SUN_VTOC_SANITY: u32 = 0x600DDEEE;

const SUN_WHOLE_DISK: u16 = 5;
const LINUX_RAID_PARTITION: u16 = 0xfd; // autodetect RAID partition

#[repr(C)]
struct SunInfo {
    id: u16,
    flags: u16,
}

#[repr(C)]
struct SunVtoc {
    version: u32,
    volume: [i8; 8],
    nparts: u16,
    infos: [SunInfo; 8],
    padding: u16,
    bootinfo: [u32; 3],
    sanity: u32,
    reserved: [u32; 10],
    timestamp: [u32; 8],
}

#[repr(C)]
struct SunPartition {
    start_cylinder: u32,
    num_sectors: u32,
}

#[repr(C)]
struct SunDisklabel {
    info: [u8; 128],
    vtoc: SunVtoc,
    write_reinstruct: u32,
    read_reinstruct: u32,
    spare: [u8; 148],
    rspeed: u16,
    pcylcount: u16,
    sparecyl: u16,
    obs1: u16,
    obs2: u16,
    ilfact: u16,
    ncyl: u16,
    nacyl: u16,
    ntrks: u16,
    nsect: u16,
    obs3: u16,
    obs4: u16,
    partitions: [SunPartition; 8],
    magic: u16,
    csum: u16,
}

pub unsafe fn sun_partition(state: *mut parsed_partitions) -> i32 {
    let mut csum: u16;
    let mut slot: i32 = 1;
    let sect: Sector;
    let label: *mut SunDisklabel;
    let mut use_vtoc: bool;
    let nparts: usize;

    label = read_part_sector(state, 0, &mut sect);
    if label.is_null() {
        return -1;
    }

    if u16::from_be((*label).magic) != SUN_LABEL_MAGIC {
        put_dev_sector(sect);
        return 0;
    }

    // Look at the checksum. The C implementation walks all 16-bit words
    // in the label backwards from the final word.
    let mut ush = (label as *mut u16).add(core::mem::size_of::<SunDisklabel>() / 2);
    csum = 0;
    while ush > label as *mut u16 {
        ush = ush.sub(1);
        csum ^= *ush;
    }
    if csum != 0 {
        printk(
            "Dev %s Sun disklabel: Csum bad, label corrupted\n",
            (*(*state).disk).disk_name,
        );
        put_dev_sector(sect);
        return 0;
    }

    use_vtoc = u32::from_be((*label).vtoc.sanity) == SUN_VTOC_SANITY
        && u32::from_be((*label).vtoc.version) == 1
        && u16::from_be((*label).vtoc.nparts) <= 8;

    // Use 8 partition entries if not specified in validated VTOC
    nparts = if use_vtoc {
        u16::from_be((*label).vtoc.nparts) as usize
    } else {
        8
    };

    /*
     * So that old Linux-Sun partitions continue to work,
     * alow the VTOC to be used under the additional condition ...
     */
    use_vtoc = use_vtoc
        || ((*label).vtoc.sanity == 0
            && (*label).vtoc.version == 0
            && (*label).vtoc.nparts == 0);
    let spc = u16::from_be((*label).ntrks) as u64 * u16::from_be((*label).nsect) as u64;
    let mut p = (*label).partitions.as_mut_ptr();
    for i in 0..nparts {
        let st_sector = u32::from_be((*p).start_cylinder) as u64 * spc;
        let num_sectors = u32::from_be((*p).num_sectors);
        if num_sectors != 0 {
            put_partition(state, slot, st_sector, num_sectors);
            (*state).parts[slot as usize].flags = 0;
            if use_vtoc {
                if u16::from_be((*label).vtoc.infos[i].id) == LINUX_RAID_PARTITION {
                    (*state).parts[slot as usize].flags |= ADDPART_FLAG_RAID;
                } else if u16::from_be((*label).vtoc.infos[i].id) == SUN_WHOLE_DISK {
                    (*state).parts[slot as usize].flags |= ADDPART_FLAG_WHOLEDISK;
                }
            }
        }
        slot += 1;
        p = p.add(1);
    }
    seq_buf_puts(&mut (*state).pp_buf, "\n");
    put_dev_sector(sect);
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
