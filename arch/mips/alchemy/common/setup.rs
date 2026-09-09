/*
 * Copyright 2000, 2007-2008 MontaVista Software Inc.
 * Author: MontaVista Software, Inc. <source@mvista.com
 *
 * Updates to 2.6, Pete Popov, Embedded Alley Solutions, Inc.
 *
 *  This program is free software; you can redistribute  it and/or modify it
 *  under  the terms of  the GNU General  Public License as published by the
 *  Free Software Foundation;  either version 2 of the  License, or (at your
 *  option) any later version.
 *
 *  THIS  SOFTWARE  IS PROVIDED   ``AS  IS'' AND   ANY  EXPRESS OR IMPLIED
 *  WARRANTIES,   INCLUDING, BUT NOT  LIMITED  TO, THE IMPLIED WARRANTIES OF
 *  MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.  IN
 *  NO  EVENT  SHALL   THE AUTHOR  BE    LIABLE FOR ANY   DIRECT, INDIRECT,
 *  INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 *  NOT LIMITED   TO, PROCUREMENT OF  SUBSTITUTE GOODS  OR SERVICES; LOSS OF
 *  USE, DATA,  OR PROFITS; OR  BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON
 *  ANY THEORY OF LIABILITY, WHETHER IN  CONTRACT, STRICT LIABILITY, OR TORT
 *  (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 *  THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 *  You should have received a copy of the  GNU General Public License along
 *  with this program; if not, write  to the Free Software Foundation, Inc.,
 *  675 Mass Ave, Cambridge, MA 02139, USA.
 */

// External declarations supplied by the kernel and platform headers.
unsafe extern "C" {
    fn alchemy_get_cputype() -> i32;
    fn read_c0_prid() -> u32;
    fn au1xxx_cpu_needs_config_od() -> bool;
    fn set_c0_config(value: u32);
    fn clear_c0_config(value: u32);
    fn alchemy_set_lpj();
    fn board_setup();
    fn set_io_port_base(base: usize);

    static mut dma_default_coherent: bool;
    static mut ioport_resource: Resource;
    static mut iomem_resource: Resource;
}

#[repr(C)]
pub struct Resource {
    pub start: usize,
    pub end: usize,
}

fn alchemy_dma_coherent() -> bool {
    unsafe {
        match alchemy_get_cputype() {
            ALCHEMY_CPU_AU1000 | ALCHEMY_CPU_AU1500 | ALCHEMY_CPU_AU1100 => false,
            ALCHEMY_CPU_AU1200 => {
                /* Au1200 AB USB does not support coherent memory */
                if (read_c0_prid() & PRID_REV_MASK) == 0 {
                    return false;
                }
                true
            }
            _ => true,
        }
    }
}

pub unsafe fn plat_mem_setup() {
    alchemy_set_lpj();

    if au1xxx_cpu_needs_config_od() {
        /* Various early Au1xx0 errata corrected by this */
        set_c0_config(1 << 19); /* Set Config[OD] */
    } else {
        /* Clear to obtain best system bus performance */
        clear_c0_config(1 << 19); /* Clear Config[OD] */
    }

    dma_default_coherent = alchemy_dma_coherent();

    board_setup(); /* board specific setup */

    /* IO/MEM resources. */
    set_io_port_base(0);
    ioport_resource.start = IOPORT_RESOURCE_START;
    ioport_resource.end = IOPORT_RESOURCE_END;
    iomem_resource.start = IOMEM_RESOURCE_START;
    iomem_resource.end = IOMEM_RESOURCE_END;
}

#[cfg(CONFIG_MIPS_FIXUP_BIGPHYS_ADDR)]
pub unsafe fn fixup_bigphys_addr(phys_addr: u64, size: u64) -> u64 {
    let start: usize = ALCHEMY_PCI_MEMWIN_START;
    let end: usize = ALCHEMY_PCI_MEMWIN_END;

    /* Don't fixup 36-bit addresses */
    if (phys_addr >> 32) != 0 {
        return phys_addr;
    }

    /* Check for PCI memory window */
    if phys_addr >= start as u64 && (phys_addr + size - 1) <= end as u64 {
        return AU1500_PCI_MEM_PHYS_ADDR + phys_addr;
    }

    /* default nop */
    phys_addr
}

#[cfg(CONFIG_MIPS_FIXUP_BIGPHYS_ADDR)]
pub unsafe fn io_remap_pfn_range_pfn(pfn: usize, size: usize) -> usize {
    let phys_addr = fixup_bigphys_addr((pfn << PAGE_SHIFT) as u64, size as u64);

    (phys_addr >> PAGE_SHIFT) as usize
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
