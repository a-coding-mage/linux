// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright 2018, Christophe Leroy CS S.I.
 * <christophe.leroy@c-s.fr>
 *
 * This dumps the content of BATS
 */

// Dependencies supplied by the surrounding kernel translation unit.

unsafe fn bat_show_603(
    m: *mut seq_file,
    idx: i32,
    lower: u32,
    upper: u32,
    is_d: bool,
) {
    let bepi: u32 = upper & 0xfffe0000;
    let bl: u32 = (upper >> 2) & 0x7ff;
    let k: u32 = upper & 3;
    let brpn: phys_addr_t = PHYS_BAT_ADDR(lower);
    let size: u32 = (bl + 1) << 17;

    seq_printf(m, "%d: ", idx);
    if k == 0 {
        seq_puts(m, "        -\n");
        return;
    }

    seq_printf(m, "0x%08x-0x%08x ", bepi, bepi + size - 1);
    // CONFIG_PHYS_64BIT selects the kernel's physical address formatting.
    seq_printf(m, "0x%016llx ", brpn);
    pt_dump_size(m, size);

    if k == 1 {
        seq_puts(m, "User ");
    } else if k == 2 {
        seq_puts(m, "Kernel ");
    } else {
        seq_puts(m, "Kernel/User ");
    }

    if lower & BPP_RX != 0 {
        seq_puts(m, if is_d { "r   " } else { "  x " });
    } else if lower & BPP_RW != 0 {
        seq_puts(m, if is_d { "rw  " } else { "  x " });
    } else {
        seq_puts(m, "    ");
    }

    seq_puts(m, if lower & _PAGE_WRITETHRU != 0 { "w " } else { "  " });
    seq_puts(m, if lower & _PAGE_NO_CACHE != 0 { "i " } else { "  " });
    seq_puts(m, if lower & _PAGE_COHERENT != 0 { "m " } else { "  " });
    seq_puts(m, if lower & _PAGE_GUARDED != 0 { "g " } else { "  " });
    seq_puts(m, "\n");
}

macro_rules! bat_show_603 {
    ($m:expr, $n:expr, $l:expr, $u:expr, $d:expr) => {
        bat_show_603($m, $n, mfspr($l), mfspr($u), $d)
    };
}

unsafe fn bats_show(m: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    seq_puts(m, "---[ Instruction Block Address Translation ]---\n");

    bat_show_603!(m, 0, SPRN_IBAT0L, SPRN_IBAT0U, false);
    bat_show_603!(m, 1, SPRN_IBAT1L, SPRN_IBAT1U, false);
    bat_show_603!(m, 2, SPRN_IBAT2L, SPRN_IBAT2U, false);
    bat_show_603!(m, 3, SPRN_IBAT3L, SPRN_IBAT3U, false);
    if mmu_has_feature(MMU_FTR_USE_HIGH_BATS) {
        bat_show_603!(m, 4, SPRN_IBAT4L, SPRN_IBAT4U, false);
        bat_show_603!(m, 5, SPRN_IBAT5L, SPRN_IBAT5U, false);
        bat_show_603!(m, 6, SPRN_IBAT6L, SPRN_IBAT6U, false);
        bat_show_603!(m, 7, SPRN_IBAT7L, SPRN_IBAT7U, false);
    }

    seq_puts(m, "\n---[ Data Block Address Translation ]---\n");

    bat_show_603!(m, 0, SPRN_DBAT0L, SPRN_DBAT0U, true);
    bat_show_603!(m, 1, SPRN_DBAT1L, SPRN_DBAT1U, true);
    bat_show_603!(m, 2, SPRN_DBAT2L, SPRN_DBAT2U, true);
    bat_show_603!(m, 3, SPRN_DBAT3L, SPRN_DBAT3U, true);
    if mmu_has_feature(MMU_FTR_USE_HIGH_BATS) {
        bat_show_603!(m, 4, SPRN_DBAT4L, SPRN_DBAT4U, true);
        bat_show_603!(m, 5, SPRN_DBAT5L, SPRN_DBAT5U, true);
        bat_show_603!(m, 6, SPRN_DBAT6L, SPRN_DBAT6U, true);
        bat_show_603!(m, 7, SPRN_DBAT7L, SPRN_DBAT7U, true);
    }

    0
}

// Equivalent of DEFINE_SHOW_ATTRIBUTE(bats).
DEFINE_SHOW_ATTRIBUTE!(bats);

unsafe fn bats_init() -> i32 {
    debugfs_create_file(
        "block_address_translation",
        0o400,
        arch_debugfs_dir,
        core::ptr::null_mut(),
        &bats_fops,
    );
    0
}

// Equivalent of device_initcall(bats_init).
device_initcall!(bats_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
