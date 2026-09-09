/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2005-2008 Cavium Networks, Inc
 */

// C preprocessor register constants translated as Rust constants.
pub const CP0_CVMCTL_REG: (u32, u32) = (9, 7);
pub const CP0_CVMMEMCTL_REG: (u32, u32) = (11, 7);
pub const CP0_PRID_REG: (u32, u32) = (15, 0);
pub const CP0_DCACHE_ERR_REG: (u32, u32) = (27, 1);
pub const CP0_PRID_OCTEON_PASS1: u32 = 0x000d0000;
pub const CP0_PRID_OCTEON_CN30XX: u32 = 0x000d0200;

/// Original `kernel_entry_setup` assembly macro.
#[inline(always)]
pub unsafe fn kernel_entry_setup() {
    core::arch::asm!(r#"
        .set push
        .set arch=octeon
        dmfc0   v0, $11, 7
        dins    v0, $0, 0, 6
        ori     v0, {cvmseg_size}
        dmtc0   v0, $11, 7
        dmfc0   v0, $9, 7
        or      v0, v0, 0x5001
        xor     v0, v0, 0x1001
        dli     v1, ~(7 << 7)
        and     v0, v0, v1
        ori     v0, (6 << 7)
        mfc0    v1, $15, 0
        and     t1, v1, 0xfff8
        xor     t1, t1, 0x9000
        beqz    t1, 4f
        and     t1, v1, 0xfff8
        xor     t1, t1, 0x9008
        beqz    t1, 4f
        and     t1, v1, 0xfff8
        xor     t1, t1, 0x9100
        beqz    t1, 4f
        and     t1, v1, 0xff00
        xor     t1, t1, 0x9200
        bnez    t1, 5f
        and     t1, v1, 0x00ff
        slti    t1, t1, 2
        beqz    t1, 5f
4:
        or      v0, v0, 0x2000
5:
        dmtc0   v0, $9, 7
        sync
        cache   9, 0($0)
        dli     v0, {cvmseg_size}
        dsll    v0, 7
        beqz    v0, 2f
1:
        dsubu   v0, 8
        sd      $0, -32768(v0)
        bnez    v0, 1b
2:
        mfc0    v0, $15, 0
        bbit0   v0, 15, 1f
        and     t1, v0, 0xff00
        dli     v0, 0x9500
        bge     t1, v0, 1f
        dli     v0, 0x27
        dmtc0   v0, $27, 1
1:
        rdhwr   v0, $0
        bne     a2, zero, octeon_main_processor
        nop

        // CONFIG_SMP / CONFIG_RELOCATABLE branches are retained below.
        .ifdef CONFIG_SMP
octeon_spin_wait_boot:
        .ifdef CONFIG_RELOCATABLE
        PTR_LA  t0, octeon_processor_relocated_kernel_entry
        LONG_L  t0, (t0)
        beq     zero, t0, 1f
        nop
        jr      t0
        nop
1:
        .endif
        PTR_LA  t0, octeon_processor_boot
        LONG_L  t1, (t0)
        bne     t1, v0, octeon_spin_wait_boot
        nop
        PTR_LA  t0, octeon_processor_gp
        LONG_L  gp, (t0)
        PTR_LA  t0, octeon_processor_sp
        LONG_L  sp, (t0)
        LONG_S  zero, (t0)
        .ifdef __OCTEON__
        syncw
        syncw
        .else
        sync
        .endif
        j       smp_bootstrap
        nop
        .else
octeon_wait_forever:
        wait
        b       octeon_wait_forever
        nop
        .endif
octeon_main_processor:
        .set pop
    "#,
        cvmseg_size = const CONFIG_CAVIUM_OCTEON_CVMSEG_SIZE,
    );
}

/// Original empty `smp_slave_setup` assembly macro.
#[inline(always)]
pub unsafe fn smp_slave_setup() {}

/// Original `kexec_smp_wait_final` assembly macro.
#[inline(always)]
pub unsafe fn kexec_smp_wait_final() {
    core::arch::asm!(r#"
        .set push
        .set noreorder
        synci  0($0)
        .set pop
    "#);
}

// #define USE_KEXEC_SMP_WAIT_FINAL
pub const USE_KEXEC_SMP_WAIT_FINAL: bool = true;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
