/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2008 Maxime Bizon <mbizon@freebox.fr>
 */

// Dependencies supplied by the corresponding Linux/BCM63xx headers are
// intentionally left external to this translation.

static mut BCM63XX_CS_LOCK: u8 = 0;

/*
 * check if given chip select exists
 */
#[inline]
unsafe fn is_valid_cs(cs: u32) -> bool {
    cs <= 6
}

/*
 * Configure chipselect base address and size (bytes).
 * Size must be a power of two between 8k and 256M.
 */
pub unsafe fn bcm63xx_set_cs_base(cs: u32, base: u32, size: u32) -> i32 {
    let mut flags: usize = 0;
    let mut val: u32;

    if !is_valid_cs(cs) {
        return -EINVAL;
    }

    /* sanity check on size */
    if size != size.next_power_of_two() {
        return -EINVAL;
    }

    if size < 8 * 1024 || size > 256 * 1024 * 1024 {
        return -EINVAL;
    }

    val = base & MPI_CSBASE_BASE_MASK;
    /* 8k => 0 - 256M => 15 */
    val |= (size.ilog2() - (8 * 1024u32).ilog2()) << MPI_CSBASE_SIZE_SHIFT;

    spin_lock_irqsave(&raw mut BCM63XX_CS_LOCK, &mut flags);
    bcm_mpi_writel(val, MPI_CSBASE_REG(cs));
    spin_unlock_irqrestore(&raw mut BCM63XX_CS_LOCK, flags);

    0
}

/*
 * configure chipselect timing (ns)
 */
pub unsafe fn bcm63xx_set_cs_timing(
    cs: u32,
    wait: u32,
    setup: u32,
    hold: u32,
) -> i32 {
    let mut flags: usize = 0;
    let mut val: u32;

    if !is_valid_cs(cs) {
        return -EINVAL;
    }

    spin_lock_irqsave(&raw mut BCM63XX_CS_LOCK, &mut flags);
    val = bcm_mpi_readl(MPI_CSCTL_REG(cs));
    val &= !MPI_CSCTL_WAIT_MASK;
    val &= !MPI_CSCTL_SETUP_MASK;
    val &= !MPI_CSCTL_HOLD_MASK;
    val |= wait << MPI_CSCTL_WAIT_SHIFT;
    val |= setup << MPI_CSCTL_SETUP_SHIFT;
    val |= hold << MPI_CSCTL_HOLD_SHIFT;
    bcm_mpi_writel(val, MPI_CSCTL_REG(cs));
    spin_unlock_irqrestore(&raw mut BCM63XX_CS_LOCK, flags);

    0
}

/*
 * configure other chipselect parameter (data bus size, ...)
 */
pub unsafe fn bcm63xx_set_cs_param(cs: u32, params: u32) -> i32 {
    let mut flags: usize = 0;
    let mut val: u32;

    if !is_valid_cs(cs) {
        return -EINVAL;
    }

    /* none of this fields apply to pcmcia */
    if cs == MPI_CS_PCMCIA_COMMON || cs == MPI_CS_PCMCIA_ATTR || cs == MPI_CS_PCMCIA_IO {
        return -EINVAL;
    }

    spin_lock_irqsave(&raw mut BCM63XX_CS_LOCK, &mut flags);
    val = bcm_mpi_readl(MPI_CSCTL_REG(cs));
    val &= !MPI_CSCTL_DATA16_MASK;
    val &= !MPI_CSCTL_SYNCMODE_MASK;
    val &= !MPI_CSCTL_TSIZE_MASK;
    val &= !MPI_CSCTL_ENDIANSWAP_MASK;
    val |= params;
    bcm_mpi_writel(val, MPI_CSCTL_REG(cs));
    spin_unlock_irqrestore(&raw mut BCM63XX_CS_LOCK, flags);

    0
}

/*
 * set cs status (enable/disable)
 */
pub unsafe fn bcm63xx_set_cs_status(cs: u32, enable: i32) -> i32 {
    let mut flags: usize = 0;
    let mut val: u32;

    if !is_valid_cs(cs) {
        return -EINVAL;
    }

    spin_lock_irqsave(&raw mut BCM63XX_CS_LOCK, &mut flags);
    val = bcm_mpi_readl(MPI_CSCTL_REG(cs));
    if enable != 0 {
        val |= MPI_CSCTL_ENABLE_MASK;
    } else {
        val &= !MPI_CSCTL_ENABLE_MASK;
    }
    bcm_mpi_writel(val, MPI_CSCTL_REG(cs));
    spin_unlock_irqrestore(&raw mut BCM63XX_CS_LOCK, flags);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
