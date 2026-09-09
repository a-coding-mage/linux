// SPDX-License-Identifier: GPL-2.0
/*
 *  fs/partitions/sgi.c
 *
 *  Code extracted from drivers/block/genhd.c
 */

// Dependency declarations and build-time configuration are supplied by check.h.

const SGI_LABEL_MAGIC: u32 = 0x0be5a941;

enum {
    LINUX_RAID_PARTITION = 0xfd, /* autodetect RAID partition */
}

#[repr(C)]
pub struct sgi_volume {
    pub name: [i8; 8],             /* Name of volume */
    pub block_num: __be32,         /* Logical block number */
    pub num_bytes: __be32,         /* How big, in bytes */
}

#[repr(C)]
pub struct sgi_partition {
    pub num_blocks: __be32,        /* Size in logical blocks */
    pub first_block: __be32,       /* First logical block */
    pub r#type: __be32,            /* Type of this partition */
}

#[repr(C)]
pub struct sgi_disklabel {
    pub magic_mushroom: __be32,    /* Big fat spliff... */
    pub root_part_num: __be16,     /* Root partition number */
    pub swap_part_num: __be16,     /* Swap partition number */
    pub boot_file: [i8; 16],       /* Name of boot file for ARCS */
    pub _unused0: [u8; 48],        /* Device parameter useless crapola.. */
    pub volume: [sgi_volume; 15],
    pub partitions: [sgi_partition; 16],
    pub csum: __be32,              /* Disk label checksum */
    pub _unused1: __be32,          /* Padding */
}

pub unsafe fn sgi_partition(state: *mut parsed_partitions) -> i32 {
    let mut csum: i32;
    let magic: __be32;
    let mut slot: i32 = 1;
    let mut start: u32;
    let mut blocks: u32;
    let mut ui: *mut __be32;
    let mut cs: __be32;
    let mut sect: Sector = core::mem::zeroed();
    let label: *mut sgi_disklabel;
    let mut p: *mut sgi_partition;

    label = read_part_sector(state, 0, &mut sect);
    if label.is_null() {
        return -1;
    }
    p = (*label).partitions.as_mut_ptr();
    magic = (*label).magic_mushroom;
    if be32_to_cpu(magic) != SGI_LABEL_MAGIC {
        put_dev_sector(sect);
        return 0;
    }
    ui = (label.add(1) as *mut __be32).sub(1);
    csum = 0;
    while ui >= (label as *mut __be32) {
        cs = *ui;
        ui = ui.sub(1);
        csum = csum.wrapping_add(be32_to_cpu(cs) as i32);
    }
    if csum != 0 {
        printk(
            KERN_WARNING,
            "Dev %s SGI disklabel: csum bad, label corrupted\\n",
            (*(*state).disk).disk_name,
        );
        put_dev_sector(sect);
        return 0;
    }
    /* All SGI disk labels have 16 partitions, disks under Linux only
     * have 15 minor's.  Luckily there are always a few zero length
     * partitions which we don't care about so we never overflow the
     * current_minor.
     */
    let mut i = 0;
    while i < 16 {
        blocks = be32_to_cpu((*p).num_blocks);
        start = be32_to_cpu((*p).first_block);
        if blocks != 0 {
            put_partition(state, slot, start, blocks);
            if be32_to_cpu((*p).r#type) == LINUX_RAID_PARTITION {
                (*state).parts[slot as usize].flags = ADDPART_FLAG_RAID;
            }
        }
        slot += 1;
        p = p.add(1);
        i += 1;
    }
    seq_buf_puts(&mut (*state).pp_buf, "\n");
    put_dev_sector(sect);
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
