/***********************license start***************
 * Author: Cavium Networks
 *
 * Contact: support@caviumnetworks.com
 * This file is part of the OCTEON SDK
 *
 * Copyright (C) 2003-2018 Cavium, Inc.
 *
 * This file is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License, Version 2, as
 * published by the Free Software Foundation.
 *
 * This file is distributed in the hope that it will be useful, but
 * AS-IS and WITHOUT ANY WARRANTY; without even the implied warranty
 * of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE, TITLE, or
 * NONINFRINGEMENT.  See the GNU General Public License for more
 * details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this file; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA
 * or visit http://www.gnu.org/licenses/.
 *
 * This file may also be available under a different license from Cavium.
 * Contact Cavium Networks for more information
 ***********************license end**************************************/


pub unsafe u64 CVMX_GMXX_HG2_CONTROL(u64 block_id)
{
	// family-specific dispatch preserved from C source
	// case OCTEON_CN68XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000550u64) + (block_id) * 0x1000000u64;
	}
	return CVMX_ADD_IO_SEG(0x0001180008000550u64) + (block_id) * 0x8000000u64;
}

pub unsafe u64 CVMX_GMXX_INF_MODE(u64 block_id)
{
	// family-specific dispatch preserved from C source
	// case OCTEON_CN68XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x00011800080007F8u64) + (block_id) * 0x1000000u64;
	}
	return CVMX_ADD_IO_SEG(0x00011800080007F8u64) + (block_id) * 0x8000000u64;
}

pub unsafe u64 CVMX_GMXX_PRTX_CFG(u64 offset, u64 block_id)
{
	// family-specific dispatch preserved from C source
	// case OCTEON_CN31XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000010u64) + ((offset) + (block_id) * 0x0u64) * 2048;
	// case OCTEON_CN68XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000010u64) + ((offset) + (block_id) * 0x2000u64) * 2048;
	}
	return CVMX_ADD_IO_SEG(0x0001180008000010u64) + ((offset) + (block_id) * 0x10000u64) * 2048;
}

pub unsafe u64 CVMX_GMXX_RXX_ADR_CAM0(u64 offset, u64 block_id)
{
	// family-specific dispatch preserved from C source
	// case OCTEON_CN31XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000180u64) + ((offset) + (block_id) * 0x0u64) * 2048;
	// case OCTEON_CN68XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000180u64) + ((offset) + (block_id) * 0x2000u64) * 2048;
	}
	return CVMX_ADD_IO_SEG(0x0001180008000180u64) + ((offset) + (block_id) * 0x10000u64) * 2048;
}

pub unsafe u64 CVMX_GMXX_RXX_ADR_CAM1(u64 offset, u64 block_id)
{
	// family-specific dispatch preserved from C source
	// case OCTEON_CN31XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000188u64) + ((offset) + (block_id) * 0x0u64) * 2048;
	// case OCTEON_CN68XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000188u64) + ((offset) + (block_id) * 0x2000u64) * 2048;
	}
	return CVMX_ADD_IO_SEG(0x0001180008000188u64) + ((offset) + (block_id) * 0x10000u64) * 2048;
}

pub unsafe u64 CVMX_GMXX_RXX_ADR_CAM2(u64 offset, u64 block_id)
{
	// family-specific dispatch preserved from C source
	// case OCTEON_CN31XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000190u64) + ((offset) + (block_id) * 0x0u64) * 2048;
	// case OCTEON_CN68XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000190u64) + ((offset) + (block_id) * 0x2000u64) * 2048;
	}
	return CVMX_ADD_IO_SEG(0x0001180008000190u64) + ((offset) + (block_id) * 0x10000u64) * 2048;
}

pub unsafe u64 CVMX_GMXX_RXX_ADR_CAM3(u64 offset, u64 block_id)
{
	// family-specific dispatch preserved from C source
	// case OCTEON_CN31XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000198u64) + ((offset) + (block_id) * 0x0u64) * 2048;
	// case OCTEON_CN68XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000198u64) + ((offset) + (block_id) * 0x2000u64) * 2048;
	}
	return CVMX_ADD_IO_SEG(0x0001180008000198u64) + ((offset) + (block_id) * 0x10000u64) * 2048;
}

pub unsafe u64 CVMX_GMXX_RXX_ADR_CAM4(u64 offset, u64 block_id)
{
	// family-specific dispatch preserved from C source
	// case OCTEON_CN31XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x00011800080001A0u64) + ((offset) + (block_id) * 0x0u64) * 2048;
	// case OCTEON_CN68XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x00011800080001A0u64) + ((offset) + (block_id) * 0x2000u64) * 2048;
	}
	return CVMX_ADD_IO_SEG(0x00011800080001A0u64) + ((offset) + (block_id) * 0x10000u64) * 2048;
}

pub unsafe u64 CVMX_GMXX_RXX_ADR_CAM5(u64 offset, u64 block_id)
{
	// family-specific dispatch preserved from C source
	// case OCTEON_CN31XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x00011800080001A8u64) + ((offset) + (block_id) * 0x0u64) * 2048;
	// case OCTEON_CN68XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x00011800080001A8u64) + ((offset) + (block_id) * 0x2000u64) * 2048;
	}
	return CVMX_ADD_IO_SEG(0x00011800080001A8u64) + ((offset) + (block_id) * 0x10000u64) * 2048;
}

pub unsafe u64 CVMX_GMXX_RXX_ADR_CAM_EN(u64 offset, u64 block_id)
{
	// family-specific dispatch preserved from C source
	// case OCTEON_CN31XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000108u64) + ((offset) + (block_id) * 0x0u64) * 2048;
	// case OCTEON_CN68XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000108u64) + ((offset) + (block_id) * 0x2000u64) * 2048;
	}
	return CVMX_ADD_IO_SEG(0x0001180008000108u64) + ((offset) + (block_id) * 0x10000u64) * 2048;
}

pub unsafe u64 CVMX_GMXX_RXX_ADR_CTL(u64 offset, u64 block_id)
{
	// family-specific dispatch preserved from C source
	// case OCTEON_CN31XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000100u64) + ((offset) + (block_id) * 0x0u64) * 2048;
	// case OCTEON_CN68XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000100u64) + ((offset) + (block_id) * 0x2000u64) * 2048;
	}
	return CVMX_ADD_IO_SEG(0x0001180008000100u64) + ((offset) + (block_id) * 0x10000u64) * 2048;
}

pub unsafe u64 CVMX_GMXX_RXX_FRM_CTL(u64 offset, u64 block_id)
{
	// family-specific dispatch preserved from C source
	// case OCTEON_CN31XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000018u64) + ((offset) + (block_id) * 0x0u64) * 2048;
	// case OCTEON_CN68XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000018u64) + ((offset) + (block_id) * 0x2000u64) * 2048;
	}
	return CVMX_ADD_IO_SEG(0x0001180008000018u64) + ((offset) + (block_id) * 0x10000u64) * 2048;
}

pub const fn CVMX_GMXX_RXX_FRM_MAX(offset: u64, block_id: u64) -> u64 { unsafe { CVMX_ADD_IO_SEG(0x0001180008000030u64) + (((offset) & 3) + ((block_id) & 1) * 0x10000u64) * 2048 }
pub const fn CVMX_GMXX_RXX_FRM_MIN(offset: u64, block_id: u64) -> u64 { unsafe { CVMX_ADD_IO_SEG(0x0001180008000028u64) + (((offset) & 3) + ((block_id) & 1) * 0x10000u64) * 2048 }

pub unsafe u64 CVMX_GMXX_RXX_INT_EN(u64 offset, u64 block_id)
{
	// family-specific dispatch preserved from C source
	// case OCTEON_CN31XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000008u64) + ((offset) + (block_id) * 0x0u64) * 2048;
	// case OCTEON_CN68XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000008u64) + ((offset) + (block_id) * 0x2000u64) * 2048;
	}
	return CVMX_ADD_IO_SEG(0x0001180008000008u64) + ((offset) + (block_id) * 0x10000u64) * 2048;
}

pub unsafe u64 CVMX_GMXX_RXX_INT_REG(u64 offset, u64 block_id)
{
	// family-specific dispatch preserved from C source
	// case OCTEON_CN31XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000000u64) + ((offset) + (block_id) * 0x0u64) * 2048;
	// case OCTEON_CN68XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000000u64) + ((offset) + (block_id) * 0x2000u64) * 2048;
	}
	return CVMX_ADD_IO_SEG(0x0001180008000000u64) + ((offset) + (block_id) * 0x10000u64) * 2048;
}

pub unsafe u64 CVMX_GMXX_RXX_JABBER(u64 offset, u64 block_id)
{
	// family-specific dispatch preserved from C source
	// case OCTEON_CN31XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000038u64) + ((offset) + (block_id) * 0x0u64) * 2048;
	// case OCTEON_CN68XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000038u64) + ((offset) + (block_id) * 0x2000u64) * 2048;
	}
	return CVMX_ADD_IO_SEG(0x0001180008000038u64) + ((offset) + (block_id) * 0x10000u64) * 2048;
}

pub const fn CVMX_GMXX_RXX_RX_INBND(offset: u64, block_id: u64) -> u64 { unsafe { CVMX_ADD_IO_SEG(0x0001180008000060u64) + (((offset) & 3) + ((block_id) & 1) * 0x10000u64) * 2048 }

pub unsafe u64 CVMX_GMXX_RX_PRTS(u64 block_id)
{
	// family-specific dispatch preserved from C source
	// case OCTEON_CN68XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000410u64) + (block_id) * 0x1000000u64;
	}
	return CVMX_ADD_IO_SEG(0x0001180008000410u64) + (block_id) * 0x8000000u64;
}

pub unsafe u64 CVMX_GMXX_RX_XAUI_CTL(u64 block_id)
{
	// family-specific dispatch preserved from C source
	// case OCTEON_CN68XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000530u64) + (block_id) * 0x1000000u64;
	}
	return CVMX_ADD_IO_SEG(0x0001180008000530u64) + (block_id) * 0x8000000u64;
}

pub unsafe u64 CVMX_GMXX_SMACX(u64 offset, u64 block_id)
{
	// family-specific dispatch preserved from C source
	// case OCTEON_CN31XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000230u64) + ((offset) + (block_id) * 0x0u64) * 2048;
	// case OCTEON_CN68XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000230u64) + ((offset) + (block_id) * 0x2000u64) * 2048;
	}
	return CVMX_ADD_IO_SEG(0x0001180008000230u64) + ((offset) + (block_id) * 0x10000u64) * 2048;
}

pub unsafe u64 CVMX_GMXX_TXX_BURST(u64 offset, u64 block_id)
{
	// family-specific dispatch preserved from C source
	// case OCTEON_CN31XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000228u64) + ((offset) + (block_id) * 0x0u64) * 2048;
	// case OCTEON_CN68XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000228u64) + ((offset) + (block_id) * 0x2000u64) * 2048;
	}
	return CVMX_ADD_IO_SEG(0x0001180008000228u64) + ((offset) + (block_id) * 0x10000u64) * 2048;
}

pub const fn CVMX_GMXX_TXX_CLK(offset: u64, block_id: u64) -> u64 { unsafe { CVMX_ADD_IO_SEG(0x0001180008000208u64) + (((offset) & 3) + ((block_id) & 1) * 0x10000u64) * 2048 }
pub unsafe u64 CVMX_GMXX_TXX_CTL(u64 offset, u64 block_id)
{
	// family-specific dispatch preserved from C source
	// case OCTEON_CN31XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000270u64) + ((offset) + (block_id) * 0x0u64) * 2048;
	// case OCTEON_CN68XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000270u64) + ((offset) + (block_id) * 0x2000u64) * 2048;
	}
	return CVMX_ADD_IO_SEG(0x0001180008000270u64) + ((offset) + (block_id) * 0x10000u64) * 2048;
}

pub unsafe u64 CVMX_GMXX_TXX_PAUSE_PKT_INTERVAL(u64 offset, u64 block_id)
{
	// family-specific dispatch preserved from C source
	// case OCTEON_CN31XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000248u64) + ((offset) + (block_id) * 0x0u64) * 2048;
	// case OCTEON_CN68XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000248u64) + ((offset) + (block_id) * 0x2000u64) * 2048;
	}
	return CVMX_ADD_IO_SEG(0x0001180008000248u64) + ((offset) + (block_id) * 0x10000u64) * 2048;
}

pub unsafe u64 CVMX_GMXX_TXX_PAUSE_PKT_TIME(u64 offset, u64 block_id)
{
	// family-specific dispatch preserved from C source
	// case OCTEON_CN31XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000238u64) + ((offset) + (block_id) * 0x0u64) * 2048;
	// case OCTEON_CN68XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000238u64) + ((offset) + (block_id) * 0x2000u64) * 2048;
	}
	return CVMX_ADD_IO_SEG(0x0001180008000238u64) + ((offset) + (block_id) * 0x10000u64) * 2048;
}

pub unsafe u64 CVMX_GMXX_TXX_SLOT(u64 offset, u64 block_id)
{
	// family-specific dispatch preserved from C source
	// case OCTEON_CN31XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000220u64) + ((offset) + (block_id) * 0x0u64) * 2048;
	// case OCTEON_CN68XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000220u64) + ((offset) + (block_id) * 0x2000u64) * 2048;
	}
	return CVMX_ADD_IO_SEG(0x0001180008000220u64) + ((offset) + (block_id) * 0x10000u64) * 2048;
}

pub unsafe u64 CVMX_GMXX_TXX_THRESH(u64 offset, u64 block_id)
{
	// family-specific dispatch preserved from C source
	// case OCTEON_CN31XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000210u64) + ((offset) + (block_id) * 0x0u64) * 2048;
	// case OCTEON_CN68XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000210u64) + ((offset) + (block_id) * 0x2000u64) * 2048;
	}
	return CVMX_ADD_IO_SEG(0x0001180008000210u64) + ((offset) + (block_id) * 0x10000u64) * 2048;
}

pub unsafe u64 CVMX_GMXX_TX_INT_EN(u64 block_id)
{
	// family-specific dispatch preserved from C source
	// case OCTEON_CN68XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000508u64) + (block_id) * 0x1000000u64;
	}
	return CVMX_ADD_IO_SEG(0x0001180008000508u64) + (block_id) * 0x8000000u64;
}

pub unsafe u64 CVMX_GMXX_TX_INT_REG(u64 block_id)
{
	// family-specific dispatch preserved from C source
	// case OCTEON_CN68XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000500u64) + (block_id) * 0x1000000u64;
	}
	return CVMX_ADD_IO_SEG(0x0001180008000500u64) + (block_id) * 0x8000000u64;
}

pub unsafe u64 CVMX_GMXX_TX_OVR_BP(u64 block_id)
{
	// family-specific dispatch preserved from C source
	// case OCTEON_CN68XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x00011800080004C8u64) + (block_id) * 0x1000000u64;
	}
	return CVMX_ADD_IO_SEG(0x00011800080004C8u64) + (block_id) * 0x8000000u64;
}

pub unsafe u64 CVMX_GMXX_TX_PRTS(u64 block_id)
{
	// family-specific dispatch preserved from C source
	// case OCTEON_CN68XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000480u64) + (block_id) * 0x1000000u64;
	}
	return CVMX_ADD_IO_SEG(0x0001180008000480u64) + (block_id) * 0x8000000u64;
}

pub const fn CVMX_GMXX_TX_SPI_CTL(block_id: u64) -> u64 { unsafe { CVMX_ADD_IO_SEG(0x00011800080004C0u64) + ((block_id) & 1) * 0x8000000u64 }
pub const fn CVMX_GMXX_TX_SPI_MAX(block_id: u64) -> u64 { unsafe { CVMX_ADD_IO_SEG(0x00011800080004B0u64) + ((block_id) & 1) * 0x8000000u64 }
pub const fn CVMX_GMXX_TX_SPI_THRESH(block_id: u64) -> u64 { unsafe { CVMX_ADD_IO_SEG(0x00011800080004B8u64) + ((block_id) & 1) * 0x8000000u64 }
pub unsafe u64 CVMX_GMXX_TX_XAUI_CTL(u64 block_id)
{
	// family-specific dispatch preserved from C source
	// case OCTEON_CN68XX & OCTEON_FAMILY_MASK:
		return CVMX_ADD_IO_SEG(0x0001180008000528u64) + (block_id) * 0x1000000u64;
	}
	return CVMX_ADD_IO_SEG(0x0001180008000528u64) + (block_id) * 0x8000000u64;
}

unsafe extern "C" { pub fn __cvmx_interrupt_gmxx_enable(interface: i32); }

#[repr(C)] pub union cvmx_gmxx_hg2_control {
	pub u64: u64,
	#[repr(C)] pub struct cvmx_gmxx_hg2_control_s {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_19_63: u64,
		pub hg2tx_en: u64,
		pub hg2rx_en: u64,
		pub phys_en: u64,
		pub logl_en: u64,
// #else
		pub logl_en: u64,
		pub phys_en: u64,
		pub hg2rx_en: u64,
		pub hg2tx_en: u64,
		pub reserved_19_63: u64,
	pub s: u64,
};

#[repr(C)] pub union cvmx_gmxx_inf_mode {
	pub u64: u64,
	#[repr(C)] pub struct cvmx_gmxx_inf_mode_s {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_20_63: u64,
		pub rate: u64,
		pub reserved_12_15: u64,
		pub speed: u64,
		pub reserved_7_7: u64,
		pub mode: u64,
		pub reserved_3_3: u64,
		pub p0mii: u64,
		pub en: u64,
		pub type: u64,
// #else
		pub type: u64,
		pub en: u64,
		pub p0mii: u64,
		pub reserved_3_3: u64,
		pub mode: u64,
		pub reserved_7_7: u64,
		pub speed: u64,
		pub reserved_12_15: u64,
		pub rate: u64,
		pub reserved_20_63: u64,
	pub s: u64,
	#[repr(C)] pub struct cvmx_gmxx_inf_mode_cn30xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_3_63: u64,
		pub p0mii: u64,
		pub en: u64,
		pub type: u64,
// #else
		pub type: u64,
		pub en: u64,
		pub p0mii: u64,
		pub reserved_3_63: u64,
	pub cn30xx: u64,
	#[repr(C)] pub struct cvmx_gmxx_inf_mode_cn31xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_2_63: u64,
		pub en: u64,
		pub type: u64,
// #else
		pub type: u64,
		pub en: u64,
		pub reserved_2_63: u64,
	pub cn31xx: u64,
	#[repr(C)] pub struct cvmx_gmxx_inf_mode_cn52xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_10_63: u64,
		pub speed: u64,
		pub reserved_6_7: u64,
		pub mode: u64,
		pub reserved_2_3: u64,
		pub en: u64,
		pub type: u64,
// #else
		pub type: u64,
		pub en: u64,
		pub reserved_2_3: u64,
		pub mode: u64,
		pub reserved_6_7: u64,
		pub speed: u64,
		pub reserved_10_63: u64,
	pub cn52xx: u64,
	#[repr(C)] pub struct cvmx_gmxx_inf_mode_cn61xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_12_63: u64,
		pub speed: u64,
		pub reserved_5_7: u64,
		pub mode: u64,
		pub reserved_2_3: u64,
		pub en: u64,
		pub type: u64,
// #else
		pub type: u64,
		pub en: u64,
		pub reserved_2_3: u64,
		pub mode: u64,
		pub reserved_5_7: u64,
		pub speed: u64,
		pub reserved_12_63: u64,
	pub cn61xx: u64,
	#[repr(C)] pub struct cvmx_gmxx_inf_mode_cn66xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_20_63: u64,
		pub rate: u64,
		pub reserved_12_15: u64,
		pub speed: u64,
		pub reserved_5_7: u64,
		pub mode: u64,
		pub reserved_2_3: u64,
		pub en: u64,
		pub type: u64,
// #else
		pub type: u64,
		pub en: u64,
		pub reserved_2_3: u64,
		pub mode: u64,
		pub reserved_5_7: u64,
		pub speed: u64,
		pub reserved_12_15: u64,
		pub rate: u64,
		pub reserved_20_63: u64,
	pub cn66xx: u64,
	#[repr(C)] pub struct cvmx_gmxx_inf_mode_cn68xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_12_63: u64,
		pub speed: u64,
		pub reserved_7_7: u64,
		pub mode: u64,
		pub reserved_2_3: u64,
		pub en: u64,
		pub type: u64,
// #else
		pub type: u64,
		pub en: u64,
		pub reserved_2_3: u64,
		pub mode: u64,
		pub reserved_7_7: u64,
		pub speed: u64,
		pub reserved_12_63: u64,
	pub cn68xx: u64,
};

#[repr(C)] pub union cvmx_gmxx_prtx_cfg {
	pub u64: u64,
	#[repr(C)] pub struct cvmx_gmxx_prtx_cfg_s {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_22_63: u64,
		pub pknd: u64,
		pub reserved_14_15: u64,
		pub tx_idle: u64,
		pub rx_idle: u64,
		pub reserved_9_11: u64,
		pub speed_msb: u64,
		pub reserved_4_7: u64,
		pub slottime: u64,
		pub duplex: u64,
		pub speed: u64,
		pub en: u64,
// #else
		pub en: u64,
		pub speed: u64,
		pub duplex: u64,
		pub slottime: u64,
		pub reserved_4_7: u64,
		pub speed_msb: u64,
		pub reserved_9_11: u64,
		pub rx_idle: u64,
		pub tx_idle: u64,
		pub reserved_14_15: u64,
		pub pknd: u64,
		pub reserved_22_63: u64,
	pub s: u64,
	#[repr(C)] pub struct cvmx_gmxx_prtx_cfg_cn30xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_4_63: u64,
		pub slottime: u64,
		pub duplex: u64,
		pub speed: u64,
		pub en: u64,
// #else
		pub en: u64,
		pub speed: u64,
		pub duplex: u64,
		pub slottime: u64,
		pub reserved_4_63: u64,
	pub cn30xx: u64,
	#[repr(C)] pub struct cvmx_gmxx_prtx_cfg_cn52xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_14_63: u64,
		pub tx_idle: u64,
		pub rx_idle: u64,
		pub reserved_9_11: u64,
		pub speed_msb: u64,
		pub reserved_4_7: u64,
		pub slottime: u64,
		pub duplex: u64,
		pub speed: u64,
		pub en: u64,
// #else
		pub en: u64,
		pub speed: u64,
		pub duplex: u64,
		pub slottime: u64,
		pub reserved_4_7: u64,
		pub speed_msb: u64,
		pub reserved_9_11: u64,
		pub rx_idle: u64,
		pub tx_idle: u64,
		pub reserved_14_63: u64,
	pub cn52xx: u64,
};

#[repr(C)] pub union cvmx_gmxx_rxx_adr_ctl {
	pub u64: u64,
	#[repr(C)] pub struct cvmx_gmxx_rxx_adr_ctl_s {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_4_63: u64,
		pub cam_mode: u64,
		pub mcst: u64,
		pub bcst: u64,
// #else
		pub bcst: u64,
		pub mcst: u64,
		pub cam_mode: u64,
		pub reserved_4_63: u64,
	pub s: u64,
};

#[repr(C)] pub union cvmx_gmxx_rxx_frm_ctl {
	pub u64: u64,
	#[repr(C)] pub struct cvmx_gmxx_rxx_frm_ctl_s {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_13_63: u64,
		pub ptp_mode: u64,
		pub reserved_11_11: u64,
		pub nu64_dis: u64,
		pub pre_align: u64,
		pub pad_len: u64,
		pub vlan_len: u64,
		pub pre_free: u64,
		pub ctl_smac: u64,
		pub ctl_mcst: u64,
		pub ctl_bck: u64,
		pub ctl_drp: u64,
		pub pre_strp: u64,
		pub pre_chk: u64,
// #else
		pub pre_chk: u64,
		pub pre_strp: u64,
		pub ctl_drp: u64,
		pub ctl_bck: u64,
		pub ctl_mcst: u64,
		pub ctl_smac: u64,
		pub pre_free: u64,
		pub vlan_len: u64,
		pub pad_len: u64,
		pub pre_align: u64,
		pub nu64_dis: u64,
		pub reserved_11_11: u64,
		pub ptp_mode: u64,
		pub reserved_13_63: u64,
	pub s: u64,
	#[repr(C)] pub struct cvmx_gmxx_rxx_frm_ctl_cn30xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_9_63: u64,
		pub pad_len: u64,
		pub vlan_len: u64,
		pub pre_free: u64,
		pub ctl_smac: u64,
		pub ctl_mcst: u64,
		pub ctl_bck: u64,
		pub ctl_drp: u64,
		pub pre_strp: u64,
		pub pre_chk: u64,
// #else
		pub pre_chk: u64,
		pub pre_strp: u64,
		pub ctl_drp: u64,
		pub ctl_bck: u64,
		pub ctl_mcst: u64,
		pub ctl_smac: u64,
		pub pre_free: u64,
		pub vlan_len: u64,
		pub pad_len: u64,
		pub reserved_9_63: u64,
	pub cn30xx: u64,
	#[repr(C)] pub struct cvmx_gmxx_rxx_frm_ctl_cn31xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_8_63: u64,
		pub vlan_len: u64,
		pub pre_free: u64,
		pub ctl_smac: u64,
		pub ctl_mcst: u64,
		pub ctl_bck: u64,
		pub ctl_drp: u64,
		pub pre_strp: u64,
		pub pre_chk: u64,
// #else
		pub pre_chk: u64,
		pub pre_strp: u64,
		pub ctl_drp: u64,
		pub ctl_bck: u64,
		pub ctl_mcst: u64,
		pub ctl_smac: u64,
		pub pre_free: u64,
		pub vlan_len: u64,
		pub reserved_8_63: u64,
	pub cn31xx: u64,
	#[repr(C)] pub struct cvmx_gmxx_rxx_frm_ctl_cn50xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_11_63: u64,
		pub nu64_dis: u64,
		pub pre_align: u64,
		pub reserved_7_8: u64,
		pub pre_free: u64,
		pub ctl_smac: u64,
		pub ctl_mcst: u64,
		pub ctl_bck: u64,
		pub ctl_drp: u64,
		pub pre_strp: u64,
		pub pre_chk: u64,
// #else
		pub pre_chk: u64,
		pub pre_strp: u64,
		pub ctl_drp: u64,
		pub ctl_bck: u64,
		pub ctl_mcst: u64,
		pub ctl_smac: u64,
		pub pre_free: u64,
		pub reserved_7_8: u64,
		pub pre_align: u64,
		pub nu64_dis: u64,
		pub reserved_11_63: u64,
	pub cn50xx: u64,
	#[repr(C)] pub struct cvmx_gmxx_rxx_frm_ctl_cn56xxp1 {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_10_63: u64,
		pub pre_align: u64,
		pub reserved_7_8: u64,
		pub pre_free: u64,
		pub ctl_smac: u64,
		pub ctl_mcst: u64,
		pub ctl_bck: u64,
		pub ctl_drp: u64,
		pub pre_strp: u64,
		pub pre_chk: u64,
// #else
		pub pre_chk: u64,
		pub pre_strp: u64,
		pub ctl_drp: u64,
		pub ctl_bck: u64,
		pub ctl_mcst: u64,
		pub ctl_smac: u64,
		pub pre_free: u64,
		pub reserved_7_8: u64,
		pub pre_align: u64,
		pub reserved_10_63: u64,
	pub cn56xxp1: u64,
	#[repr(C)] pub struct cvmx_gmxx_rxx_frm_ctl_cn58xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_11_63: u64,
		pub nu64_dis: u64,
		pub pre_align: u64,
		pub pad_len: u64,
		pub vlan_len: u64,
		pub pre_free: u64,
		pub ctl_smac: u64,
		pub ctl_mcst: u64,
		pub ctl_bck: u64,
		pub ctl_drp: u64,
		pub pre_strp: u64,
		pub pre_chk: u64,
// #else
		pub pre_chk: u64,
		pub pre_strp: u64,
		pub ctl_drp: u64,
		pub ctl_bck: u64,
		pub ctl_mcst: u64,
		pub ctl_smac: u64,
		pub pre_free: u64,
		pub vlan_len: u64,
		pub pad_len: u64,
		pub pre_align: u64,
		pub nu64_dis: u64,
		pub reserved_11_63: u64,
	pub cn58xx: u64,
	#[repr(C)] pub struct cvmx_gmxx_rxx_frm_ctl_cn61xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_13_63: u64,
		pub ptp_mode: u64,
		pub reserved_11_11: u64,
		pub nu64_dis: u64,
		pub pre_align: u64,
		pub reserved_7_8: u64,
		pub pre_free: u64,
		pub ctl_smac: u64,
		pub ctl_mcst: u64,
		pub ctl_bck: u64,
		pub ctl_drp: u64,
		pub pre_strp: u64,
		pub pre_chk: u64,
// #else
		pub pre_chk: u64,
		pub pre_strp: u64,
		pub ctl_drp: u64,
		pub ctl_bck: u64,
		pub ctl_mcst: u64,
		pub ctl_smac: u64,
		pub pre_free: u64,
		pub reserved_7_8: u64,
		pub pre_align: u64,
		pub nu64_dis: u64,
		pub reserved_11_11: u64,
		pub ptp_mode: u64,
		pub reserved_13_63: u64,
	pub cn61xx: u64,
};

#[repr(C)] pub union cvmx_gmxx_rxx_frm_max {
	pub u64: u64,
	#[repr(C)] pub struct cvmx_gmxx_rxx_frm_max_s {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_16_63: u64,
		pub len: u64,
// #else
		pub len: u64,
		pub reserved_16_63: u64,
	pub s: u64,
};

#[repr(C)] pub union cvmx_gmxx_rxx_frm_min {
	pub u64: u64,
	#[repr(C)] pub struct cvmx_gmxx_rxx_frm_min_s {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_16_63: u64,
		pub len: u64,
// #else
		pub len: u64,
		pub reserved_16_63: u64,
	pub s: u64,
};

#[repr(C)] pub union cvmx_gmxx_rxx_int_en {
	pub u64: u64,
	#[repr(C)] pub struct cvmx_gmxx_rxx_int_en_s {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_29_63: u64,
		pub hg2cc: u64,
		pub hg2fld: u64,
		pub undat: u64,
		pub uneop: u64,
		pub unsop: u64,
		pub bad_term: u64,
		pub bad_seq: u64,
		pub rem_fault: u64,
		pub loc_fault: u64,
		pub pause_drp: u64,
		pub phy_dupx: u64,
		pub phy_spd: u64,
		pub phy_link: u64,
		pub ifgerr: u64,
		pub coldet: u64,
		pub falerr: u64,
		pub rsverr: u64,
		pub pcterr: u64,
		pub ovrerr: u64,
		pub niberr: u64,
		pub skperr: u64,
		pub rcverr: u64,
		pub lenerr: u64,
		pub alnerr: u64,
		pub fcserr: u64,
		pub jabber: u64,
		pub maxerr: u64,
		pub carext: u64,
		pub minerr: u64,
// #else
		pub minerr: u64,
		pub carext: u64,
		pub maxerr: u64,
		pub jabber: u64,
		pub fcserr: u64,
		pub alnerr: u64,
		pub lenerr: u64,
		pub rcverr: u64,
		pub skperr: u64,
		pub niberr: u64,
		pub ovrerr: u64,
		pub pcterr: u64,
		pub rsverr: u64,
		pub falerr: u64,
		pub coldet: u64,
		pub ifgerr: u64,
		pub phy_link: u64,
		pub phy_spd: u64,
		pub phy_dupx: u64,
		pub pause_drp: u64,
		pub loc_fault: u64,
		pub rem_fault: u64,
		pub bad_seq: u64,
		pub bad_term: u64,
		pub unsop: u64,
		pub uneop: u64,
		pub undat: u64,
		pub hg2fld: u64,
		pub hg2cc: u64,
		pub reserved_29_63: u64,
	pub s: u64,
	#[repr(C)] pub struct cvmx_gmxx_rxx_int_en_cn30xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_19_63: u64,
		pub phy_dupx: u64,
		pub phy_spd: u64,
		pub phy_link: u64,
		pub ifgerr: u64,
		pub coldet: u64,
		pub falerr: u64,
		pub rsverr: u64,
		pub pcterr: u64,
		pub ovrerr: u64,
		pub niberr: u64,
		pub skperr: u64,
		pub rcverr: u64,
		pub lenerr: u64,
		pub alnerr: u64,
		pub fcserr: u64,
		pub jabber: u64,
		pub maxerr: u64,
		pub carext: u64,
		pub minerr: u64,
// #else
		pub minerr: u64,
		pub carext: u64,
		pub maxerr: u64,
		pub jabber: u64,
		pub fcserr: u64,
		pub alnerr: u64,
		pub lenerr: u64,
		pub rcverr: u64,
		pub skperr: u64,
		pub niberr: u64,
		pub ovrerr: u64,
		pub pcterr: u64,
		pub rsverr: u64,
		pub falerr: u64,
		pub coldet: u64,
		pub ifgerr: u64,
		pub phy_link: u64,
		pub phy_spd: u64,
		pub phy_dupx: u64,
		pub reserved_19_63: u64,
	pub cn30xx: u64,
	#[repr(C)] pub struct cvmx_gmxx_rxx_int_en_cn50xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_20_63: u64,
		pub pause_drp: u64,
		pub phy_dupx: u64,
		pub phy_spd: u64,
		pub phy_link: u64,
		pub ifgerr: u64,
		pub coldet: u64,
		pub falerr: u64,
		pub rsverr: u64,
		pub pcterr: u64,
		pub ovrerr: u64,
		pub niberr: u64,
		pub skperr: u64,
		pub rcverr: u64,
		pub reserved_6_6: u64,
		pub alnerr: u64,
		pub fcserr: u64,
		pub jabber: u64,
		pub reserved_2_2: u64,
		pub carext: u64,
		pub reserved_0_0: u64,
// #else
		pub reserved_0_0: u64,
		pub carext: u64,
		pub reserved_2_2: u64,
		pub jabber: u64,
		pub fcserr: u64,
		pub alnerr: u64,
		pub reserved_6_6: u64,
		pub rcverr: u64,
		pub skperr: u64,
		pub niberr: u64,
		pub ovrerr: u64,
		pub pcterr: u64,
		pub rsverr: u64,
		pub falerr: u64,
		pub coldet: u64,
		pub ifgerr: u64,
		pub phy_link: u64,
		pub phy_spd: u64,
		pub phy_dupx: u64,
		pub pause_drp: u64,
		pub reserved_20_63: u64,
	pub cn50xx: u64,
	#[repr(C)] pub struct cvmx_gmxx_rxx_int_en_cn52xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_29_63: u64,
		pub hg2cc: u64,
		pub hg2fld: u64,
		pub undat: u64,
		pub uneop: u64,
		pub unsop: u64,
		pub bad_term: u64,
		pub bad_seq: u64,
		pub rem_fault: u64,
		pub loc_fault: u64,
		pub pause_drp: u64,
		pub reserved_16_18: u64,
		pub ifgerr: u64,
		pub coldet: u64,
		pub falerr: u64,
		pub rsverr: u64,
		pub pcterr: u64,
		pub ovrerr: u64,
		pub reserved_9_9: u64,
		pub skperr: u64,
		pub rcverr: u64,
		pub reserved_5_6: u64,
		pub fcserr: u64,
		pub jabber: u64,
		pub reserved_2_2: u64,
		pub carext: u64,
		pub reserved_0_0: u64,
// #else
		pub reserved_0_0: u64,
		pub carext: u64,
		pub reserved_2_2: u64,
		pub jabber: u64,
		pub fcserr: u64,
		pub reserved_5_6: u64,
		pub rcverr: u64,
		pub skperr: u64,
		pub reserved_9_9: u64,
		pub ovrerr: u64,
		pub pcterr: u64,
		pub rsverr: u64,
		pub falerr: u64,
		pub coldet: u64,
		pub ifgerr: u64,
		pub reserved_16_18: u64,
		pub pause_drp: u64,
		pub loc_fault: u64,
		pub rem_fault: u64,
		pub bad_seq: u64,
		pub bad_term: u64,
		pub unsop: u64,
		pub uneop: u64,
		pub undat: u64,
		pub hg2fld: u64,
		pub hg2cc: u64,
		pub reserved_29_63: u64,
	pub cn52xx: u64,
	#[repr(C)] pub struct cvmx_gmxx_rxx_int_en_cn56xxp1 {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_27_63: u64,
		pub undat: u64,
		pub uneop: u64,
		pub unsop: u64,
		pub bad_term: u64,
		pub bad_seq: u64,
		pub rem_fault: u64,
		pub loc_fault: u64,
		pub pause_drp: u64,
		pub reserved_16_18: u64,
		pub ifgerr: u64,
		pub coldet: u64,
		pub falerr: u64,
		pub rsverr: u64,
		pub pcterr: u64,
		pub ovrerr: u64,
		pub reserved_9_9: u64,
		pub skperr: u64,
		pub rcverr: u64,
		pub reserved_5_6: u64,
		pub fcserr: u64,
		pub jabber: u64,
		pub reserved_2_2: u64,
		pub carext: u64,
		pub reserved_0_0: u64,
// #else
		pub reserved_0_0: u64,
		pub carext: u64,
		pub reserved_2_2: u64,
		pub jabber: u64,
		pub fcserr: u64,
		pub reserved_5_6: u64,
		pub rcverr: u64,
		pub skperr: u64,
		pub reserved_9_9: u64,
		pub ovrerr: u64,
		pub pcterr: u64,
		pub rsverr: u64,
		pub falerr: u64,
		pub coldet: u64,
		pub ifgerr: u64,
		pub reserved_16_18: u64,
		pub pause_drp: u64,
		pub loc_fault: u64,
		pub rem_fault: u64,
		pub bad_seq: u64,
		pub bad_term: u64,
		pub unsop: u64,
		pub uneop: u64,
		pub undat: u64,
		pub reserved_27_63: u64,
	pub cn56xxp1: u64,
	#[repr(C)] pub struct cvmx_gmxx_rxx_int_en_cn58xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_20_63: u64,
		pub pause_drp: u64,
		pub phy_dupx: u64,
		pub phy_spd: u64,
		pub phy_link: u64,
		pub ifgerr: u64,
		pub coldet: u64,
		pub falerr: u64,
		pub rsverr: u64,
		pub pcterr: u64,
		pub ovrerr: u64,
		pub niberr: u64,
		pub skperr: u64,
		pub rcverr: u64,
		pub lenerr: u64,
		pub alnerr: u64,
		pub fcserr: u64,
		pub jabber: u64,
		pub maxerr: u64,
		pub carext: u64,
		pub minerr: u64,
// #else
		pub minerr: u64,
		pub carext: u64,
		pub maxerr: u64,
		pub jabber: u64,
		pub fcserr: u64,
		pub alnerr: u64,
		pub lenerr: u64,
		pub rcverr: u64,
		pub skperr: u64,
		pub niberr: u64,
		pub ovrerr: u64,
		pub pcterr: u64,
		pub rsverr: u64,
		pub falerr: u64,
		pub coldet: u64,
		pub ifgerr: u64,
		pub phy_link: u64,
		pub phy_spd: u64,
		pub phy_dupx: u64,
		pub pause_drp: u64,
		pub reserved_20_63: u64,
	pub cn58xx: u64,
	#[repr(C)] pub struct cvmx_gmxx_rxx_int_en_cn61xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_29_63: u64,
		pub hg2cc: u64,
		pub hg2fld: u64,
		pub undat: u64,
		pub uneop: u64,
		pub unsop: u64,
		pub bad_term: u64,
		pub bad_seq: u64,
		pub rem_fault: u64,
		pub loc_fault: u64,
		pub pause_drp: u64,
		pub reserved_16_18: u64,
		pub ifgerr: u64,
		pub coldet: u64,
		pub falerr: u64,
		pub rsverr: u64,
		pub pcterr: u64,
		pub ovrerr: u64,
		pub reserved_9_9: u64,
		pub skperr: u64,
		pub rcverr: u64,
		pub reserved_5_6: u64,
		pub fcserr: u64,
		pub jabber: u64,
		pub reserved_2_2: u64,
		pub carext: u64,
		pub minerr: u64,
// #else
		pub minerr: u64,
		pub carext: u64,
		pub reserved_2_2: u64,
		pub jabber: u64,
		pub fcserr: u64,
		pub reserved_5_6: u64,
		pub rcverr: u64,
		pub skperr: u64,
		pub reserved_9_9: u64,
		pub ovrerr: u64,
		pub pcterr: u64,
		pub rsverr: u64,
		pub falerr: u64,
		pub coldet: u64,
		pub ifgerr: u64,
		pub reserved_16_18: u64,
		pub pause_drp: u64,
		pub loc_fault: u64,
		pub rem_fault: u64,
		pub bad_seq: u64,
		pub bad_term: u64,
		pub unsop: u64,
		pub uneop: u64,
		pub undat: u64,
		pub hg2fld: u64,
		pub hg2cc: u64,
		pub reserved_29_63: u64,
	pub cn61xx: u64,
};

#[repr(C)] pub union cvmx_gmxx_rxx_int_reg {
	pub u64: u64,
	#[repr(C)] pub struct cvmx_gmxx_rxx_int_reg_s {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_29_63: u64,
		pub hg2cc: u64,
		pub hg2fld: u64,
		pub undat: u64,
		pub uneop: u64,
		pub unsop: u64,
		pub bad_term: u64,
		pub bad_seq: u64,
		pub rem_fault: u64,
		pub loc_fault: u64,
		pub pause_drp: u64,
		pub phy_dupx: u64,
		pub phy_spd: u64,
		pub phy_link: u64,
		pub ifgerr: u64,
		pub coldet: u64,
		pub falerr: u64,
		pub rsverr: u64,
		pub pcterr: u64,
		pub ovrerr: u64,
		pub niberr: u64,
		pub skperr: u64,
		pub rcverr: u64,
		pub lenerr: u64,
		pub alnerr: u64,
		pub fcserr: u64,
		pub jabber: u64,
		pub maxerr: u64,
		pub carext: u64,
		pub minerr: u64,
// #else
		pub minerr: u64,
		pub carext: u64,
		pub maxerr: u64,
		pub jabber: u64,
		pub fcserr: u64,
		pub alnerr: u64,
		pub lenerr: u64,
		pub rcverr: u64,
		pub skperr: u64,
		pub niberr: u64,
		pub ovrerr: u64,
		pub pcterr: u64,
		pub rsverr: u64,
		pub falerr: u64,
		pub coldet: u64,
		pub ifgerr: u64,
		pub phy_link: u64,
		pub phy_spd: u64,
		pub phy_dupx: u64,
		pub pause_drp: u64,
		pub loc_fault: u64,
		pub rem_fault: u64,
		pub bad_seq: u64,
		pub bad_term: u64,
		pub unsop: u64,
		pub uneop: u64,
		pub undat: u64,
		pub hg2fld: u64,
		pub hg2cc: u64,
		pub reserved_29_63: u64,
	pub s: u64,
	#[repr(C)] pub struct cvmx_gmxx_rxx_int_reg_cn30xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_19_63: u64,
		pub phy_dupx: u64,
		pub phy_spd: u64,
		pub phy_link: u64,
		pub ifgerr: u64,
		pub coldet: u64,
		pub falerr: u64,
		pub rsverr: u64,
		pub pcterr: u64,
		pub ovrerr: u64,
		pub niberr: u64,
		pub skperr: u64,
		pub rcverr: u64,
		pub lenerr: u64,
		pub alnerr: u64,
		pub fcserr: u64,
		pub jabber: u64,
		pub maxerr: u64,
		pub carext: u64,
		pub minerr: u64,
// #else
		pub minerr: u64,
		pub carext: u64,
		pub maxerr: u64,
		pub jabber: u64,
		pub fcserr: u64,
		pub alnerr: u64,
		pub lenerr: u64,
		pub rcverr: u64,
		pub skperr: u64,
		pub niberr: u64,
		pub ovrerr: u64,
		pub pcterr: u64,
		pub rsverr: u64,
		pub falerr: u64,
		pub coldet: u64,
		pub ifgerr: u64,
		pub phy_link: u64,
		pub phy_spd: u64,
		pub phy_dupx: u64,
		pub reserved_19_63: u64,
	pub cn30xx: u64,
	#[repr(C)] pub struct cvmx_gmxx_rxx_int_reg_cn50xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_20_63: u64,
		pub pause_drp: u64,
		pub phy_dupx: u64,
		pub phy_spd: u64,
		pub phy_link: u64,
		pub ifgerr: u64,
		pub coldet: u64,
		pub falerr: u64,
		pub rsverr: u64,
		pub pcterr: u64,
		pub ovrerr: u64,
		pub niberr: u64,
		pub skperr: u64,
		pub rcverr: u64,
		pub reserved_6_6: u64,
		pub alnerr: u64,
		pub fcserr: u64,
		pub jabber: u64,
		pub reserved_2_2: u64,
		pub carext: u64,
		pub reserved_0_0: u64,
// #else
		pub reserved_0_0: u64,
		pub carext: u64,
		pub reserved_2_2: u64,
		pub jabber: u64,
		pub fcserr: u64,
		pub alnerr: u64,
		pub reserved_6_6: u64,
		pub rcverr: u64,
		pub skperr: u64,
		pub niberr: u64,
		pub ovrerr: u64,
		pub pcterr: u64,
		pub rsverr: u64,
		pub falerr: u64,
		pub coldet: u64,
		pub ifgerr: u64,
		pub phy_link: u64,
		pub phy_spd: u64,
		pub phy_dupx: u64,
		pub pause_drp: u64,
		pub reserved_20_63: u64,
	pub cn50xx: u64,
	#[repr(C)] pub struct cvmx_gmxx_rxx_int_reg_cn52xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_29_63: u64,
		pub hg2cc: u64,
		pub hg2fld: u64,
		pub undat: u64,
		pub uneop: u64,
		pub unsop: u64,
		pub bad_term: u64,
		pub bad_seq: u64,
		pub rem_fault: u64,
		pub loc_fault: u64,
		pub pause_drp: u64,
		pub reserved_16_18: u64,
		pub ifgerr: u64,
		pub coldet: u64,
		pub falerr: u64,
		pub rsverr: u64,
		pub pcterr: u64,
		pub ovrerr: u64,
		pub reserved_9_9: u64,
		pub skperr: u64,
		pub rcverr: u64,
		pub reserved_5_6: u64,
		pub fcserr: u64,
		pub jabber: u64,
		pub reserved_2_2: u64,
		pub carext: u64,
		pub reserved_0_0: u64,
// #else
		pub reserved_0_0: u64,
		pub carext: u64,
		pub reserved_2_2: u64,
		pub jabber: u64,
		pub fcserr: u64,
		pub reserved_5_6: u64,
		pub rcverr: u64,
		pub skperr: u64,
		pub reserved_9_9: u64,
		pub ovrerr: u64,
		pub pcterr: u64,
		pub rsverr: u64,
		pub falerr: u64,
		pub coldet: u64,
		pub ifgerr: u64,
		pub reserved_16_18: u64,
		pub pause_drp: u64,
		pub loc_fault: u64,
		pub rem_fault: u64,
		pub bad_seq: u64,
		pub bad_term: u64,
		pub unsop: u64,
		pub uneop: u64,
		pub undat: u64,
		pub hg2fld: u64,
		pub hg2cc: u64,
		pub reserved_29_63: u64,
	pub cn52xx: u64,
	#[repr(C)] pub struct cvmx_gmxx_rxx_int_reg_cn56xxp1 {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_27_63: u64,
		pub undat: u64,
		pub uneop: u64,
		pub unsop: u64,
		pub bad_term: u64,
		pub bad_seq: u64,
		pub rem_fault: u64,
		pub loc_fault: u64,
		pub pause_drp: u64,
		pub reserved_16_18: u64,
		pub ifgerr: u64,
		pub coldet: u64,
		pub falerr: u64,
		pub rsverr: u64,
		pub pcterr: u64,
		pub ovrerr: u64,
		pub reserved_9_9: u64,
		pub skperr: u64,
		pub rcverr: u64,
		pub reserved_5_6: u64,
		pub fcserr: u64,
		pub jabber: u64,
		pub reserved_2_2: u64,
		pub carext: u64,
		pub reserved_0_0: u64,
// #else
		pub reserved_0_0: u64,
		pub carext: u64,
		pub reserved_2_2: u64,
		pub jabber: u64,
		pub fcserr: u64,
		pub reserved_5_6: u64,
		pub rcverr: u64,
		pub skperr: u64,
		pub reserved_9_9: u64,
		pub ovrerr: u64,
		pub pcterr: u64,
		pub rsverr: u64,
		pub falerr: u64,
		pub coldet: u64,
		pub ifgerr: u64,
		pub reserved_16_18: u64,
		pub pause_drp: u64,
		pub loc_fault: u64,
		pub rem_fault: u64,
		pub bad_seq: u64,
		pub bad_term: u64,
		pub unsop: u64,
		pub uneop: u64,
		pub undat: u64,
		pub reserved_27_63: u64,
	pub cn56xxp1: u64,
	#[repr(C)] pub struct cvmx_gmxx_rxx_int_reg_cn58xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_20_63: u64,
		pub pause_drp: u64,
		pub phy_dupx: u64,
		pub phy_spd: u64,
		pub phy_link: u64,
		pub ifgerr: u64,
		pub coldet: u64,
		pub falerr: u64,
		pub rsverr: u64,
		pub pcterr: u64,
		pub ovrerr: u64,
		pub niberr: u64,
		pub skperr: u64,
		pub rcverr: u64,
		pub lenerr: u64,
		pub alnerr: u64,
		pub fcserr: u64,
		pub jabber: u64,
		pub maxerr: u64,
		pub carext: u64,
		pub minerr: u64,
// #else
		pub minerr: u64,
		pub carext: u64,
		pub maxerr: u64,
		pub jabber: u64,
		pub fcserr: u64,
		pub alnerr: u64,
		pub lenerr: u64,
		pub rcverr: u64,
		pub skperr: u64,
		pub niberr: u64,
		pub ovrerr: u64,
		pub pcterr: u64,
		pub rsverr: u64,
		pub falerr: u64,
		pub coldet: u64,
		pub ifgerr: u64,
		pub phy_link: u64,
		pub phy_spd: u64,
		pub phy_dupx: u64,
		pub pause_drp: u64,
		pub reserved_20_63: u64,
	pub cn58xx: u64,
	#[repr(C)] pub struct cvmx_gmxx_rxx_int_reg_cn61xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_29_63: u64,
		pub hg2cc: u64,
		pub hg2fld: u64,
		pub undat: u64,
		pub uneop: u64,
		pub unsop: u64,
		pub bad_term: u64,
		pub bad_seq: u64,
		pub rem_fault: u64,
		pub loc_fault: u64,
		pub pause_drp: u64,
		pub reserved_16_18: u64,
		pub ifgerr: u64,
		pub coldet: u64,
		pub falerr: u64,
		pub rsverr: u64,
		pub pcterr: u64,
		pub ovrerr: u64,
		pub reserved_9_9: u64,
		pub skperr: u64,
		pub rcverr: u64,
		pub reserved_5_6: u64,
		pub fcserr: u64,
		pub jabber: u64,
		pub reserved_2_2: u64,
		pub carext: u64,
		pub minerr: u64,
// #else
		pub minerr: u64,
		pub carext: u64,
		pub reserved_2_2: u64,
		pub jabber: u64,
		pub fcserr: u64,
		pub reserved_5_6: u64,
		pub rcverr: u64,
		pub skperr: u64,
		pub reserved_9_9: u64,
		pub ovrerr: u64,
		pub pcterr: u64,
		pub rsverr: u64,
		pub falerr: u64,
		pub coldet: u64,
		pub ifgerr: u64,
		pub reserved_16_18: u64,
		pub pause_drp: u64,
		pub loc_fault: u64,
		pub rem_fault: u64,
		pub bad_seq: u64,
		pub bad_term: u64,
		pub unsop: u64,
		pub uneop: u64,
		pub undat: u64,
		pub hg2fld: u64,
		pub hg2cc: u64,
		pub reserved_29_63: u64,
	pub cn61xx: u64,
};

#[repr(C)] pub union cvmx_gmxx_rxx_jabber {
	pub u64: u64,
	#[repr(C)] pub struct cvmx_gmxx_rxx_jabber_s {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_16_63: u64,
		pub cnt: u64,
// #else
		pub cnt: u64,
		pub reserved_16_63: u64,
	pub s: u64,
};

#[repr(C)] pub union cvmx_gmxx_rxx_rx_inbnd {
	pub u64: u64,
	#[repr(C)] pub struct cvmx_gmxx_rxx_rx_inbnd_s {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_4_63: u64,
		pub duplex: u64,
		pub speed: u64,
		pub status: u64,
// #else
		pub status: u64,
		pub speed: u64,
		pub duplex: u64,
		pub reserved_4_63: u64,
	pub s: u64,
};

#[repr(C)] pub union cvmx_gmxx_rx_prts {
	pub u64: u64,
	#[repr(C)] pub struct cvmx_gmxx_rx_prts_s {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_3_63: u64,
		pub prts: u64,
// #else
		pub prts: u64,
		pub reserved_3_63: u64,
	pub s: u64,
};

#[repr(C)] pub union cvmx_gmxx_rx_xaui_ctl {
	pub u64: u64,
	#[repr(C)] pub struct cvmx_gmxx_rx_xaui_ctl_s {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_2_63: u64,
		pub status: u64,
// #else
		pub status: u64,
		pub reserved_2_63: u64,
	pub s: u64,
};

#[repr(C)] pub union cvmx_gmxx_txx_thresh {
	pub u64: u64,
	#[repr(C)] pub struct cvmx_gmxx_txx_thresh_s {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_10_63: u64,
		pub cnt: u64,
// #else
		pub cnt: u64,
		pub reserved_10_63: u64,
	pub s: u64,
	#[repr(C)] pub struct cvmx_gmxx_txx_thresh_cn30xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_7_63: u64,
		pub cnt: u64,
// #else
		pub cnt: u64,
		pub reserved_7_63: u64,
	pub cn30xx: u64,
	#[repr(C)] pub struct cvmx_gmxx_txx_thresh_cn38xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_9_63: u64,
		pub cnt: u64,
// #else
		pub cnt: u64,
		pub reserved_9_63: u64,
	pub cn38xx: u64,
};

#[repr(C)] pub union cvmx_gmxx_tx_int_en {
	pub u64: u64,
	#[repr(C)] pub struct cvmx_gmxx_tx_int_en_s {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_25_63: u64,
		pub xchange: u64,
		pub ptp_lost: u64,
		pub late_col: u64,
		pub xsdef: u64,
		pub xscol: u64,
		pub reserved_6_7: u64,
		pub undflw: u64,
		pub reserved_1_1: u64,
		pub pko_nxa: u64,
// #else
		pub pko_nxa: u64,
		pub reserved_1_1: u64,
		pub undflw: u64,
		pub reserved_6_7: u64,
		pub xscol: u64,
		pub xsdef: u64,
		pub late_col: u64,
		pub ptp_lost: u64,
		pub xchange: u64,
		pub reserved_25_63: u64,
	pub s: u64,
	#[repr(C)] pub struct cvmx_gmxx_tx_int_en_cn30xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_19_63: u64,
		pub late_col: u64,
		pub reserved_15_15: u64,
		pub xsdef: u64,
		pub reserved_11_11: u64,
		pub xscol: u64,
		pub reserved_5_7: u64,
		pub undflw: u64,
		pub reserved_1_1: u64,
		pub pko_nxa: u64,
// #else
		pub pko_nxa: u64,
		pub reserved_1_1: u64,
		pub undflw: u64,
		pub reserved_5_7: u64,
		pub xscol: u64,
		pub reserved_11_11: u64,
		pub xsdef: u64,
		pub reserved_15_15: u64,
		pub late_col: u64,
		pub reserved_19_63: u64,
	pub cn30xx: u64,
	#[repr(C)] pub struct cvmx_gmxx_tx_int_en_cn31xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_15_63: u64,
		pub xsdef: u64,
		pub reserved_11_11: u64,
		pub xscol: u64,
		pub reserved_5_7: u64,
		pub undflw: u64,
		pub reserved_1_1: u64,
		pub pko_nxa: u64,
// #else
		pub pko_nxa: u64,
		pub reserved_1_1: u64,
		pub undflw: u64,
		pub reserved_5_7: u64,
		pub xscol: u64,
		pub reserved_11_11: u64,
		pub xsdef: u64,
		pub reserved_15_63: u64,
	pub cn31xx: u64,
	#[repr(C)] pub struct cvmx_gmxx_tx_int_en_cn38xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_20_63: u64,
		pub late_col: u64,
		pub xsdef: u64,
		pub xscol: u64,
		pub reserved_6_7: u64,
		pub undflw: u64,
		pub ncb_nxa: u64,
		pub pko_nxa: u64,
// #else
		pub pko_nxa: u64,
		pub ncb_nxa: u64,
		pub undflw: u64,
		pub reserved_6_7: u64,
		pub xscol: u64,
		pub xsdef: u64,
		pub late_col: u64,
		pub reserved_20_63: u64,
	pub cn38xx: u64,
	#[repr(C)] pub struct cvmx_gmxx_tx_int_en_cn38xxp2 {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_16_63: u64,
		pub xsdef: u64,
		pub xscol: u64,
		pub reserved_6_7: u64,
		pub undflw: u64,
		pub ncb_nxa: u64,
		pub pko_nxa: u64,
// #else
		pub pko_nxa: u64,
		pub ncb_nxa: u64,
		pub undflw: u64,
		pub reserved_6_7: u64,
		pub xscol: u64,
		pub xsdef: u64,
		pub reserved_16_63: u64,
	pub cn38xxp2: u64,
	#[repr(C)] pub struct cvmx_gmxx_tx_int_en_cn52xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_20_63: u64,
		pub late_col: u64,
		pub xsdef: u64,
		pub xscol: u64,
		pub reserved_6_7: u64,
		pub undflw: u64,
		pub reserved_1_1: u64,
		pub pko_nxa: u64,
// #else
		pub pko_nxa: u64,
		pub reserved_1_1: u64,
		pub undflw: u64,
		pub reserved_6_7: u64,
		pub xscol: u64,
		pub xsdef: u64,
		pub late_col: u64,
		pub reserved_20_63: u64,
	pub cn52xx: u64,
	#[repr(C)] pub struct cvmx_gmxx_tx_int_en_cn63xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_24_63: u64,
		pub ptp_lost: u64,
		pub late_col: u64,
		pub xsdef: u64,
		pub xscol: u64,
		pub reserved_6_7: u64,
		pub undflw: u64,
		pub reserved_1_1: u64,
		pub pko_nxa: u64,
// #else
		pub pko_nxa: u64,
		pub reserved_1_1: u64,
		pub undflw: u64,
		pub reserved_6_7: u64,
		pub xscol: u64,
		pub xsdef: u64,
		pub late_col: u64,
		pub ptp_lost: u64,
		pub reserved_24_63: u64,
	pub cn63xx: u64,
	#[repr(C)] pub struct cvmx_gmxx_tx_int_en_cn68xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_25_63: u64,
		pub xchange: u64,
		pub ptp_lost: u64,
		pub late_col: u64,
		pub xsdef: u64,
		pub xscol: u64,
		pub reserved_6_7: u64,
		pub undflw: u64,
		pub pko_nxp: u64,
		pub pko_nxa: u64,
// #else
		pub pko_nxa: u64,
		pub pko_nxp: u64,
		pub undflw: u64,
		pub reserved_6_7: u64,
		pub xscol: u64,
		pub xsdef: u64,
		pub late_col: u64,
		pub ptp_lost: u64,
		pub xchange: u64,
		pub reserved_25_63: u64,
	pub cn68xx: u64,
	#[repr(C)] pub struct cvmx_gmxx_tx_int_en_cnf71xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_25_63: u64,
		pub xchange: u64,
		pub reserved_22_23: u64,
		pub ptp_lost: u64,
		pub reserved_18_19: u64,
		pub late_col: u64,
		pub reserved_14_15: u64,
		pub xsdef: u64,
		pub reserved_10_11: u64,
		pub xscol: u64,
		pub reserved_4_7: u64,
		pub undflw: u64,
		pub reserved_1_1: u64,
		pub pko_nxa: u64,
// #else
		pub pko_nxa: u64,
		pub reserved_1_1: u64,
		pub undflw: u64,
		pub reserved_4_7: u64,
		pub xscol: u64,
		pub reserved_10_11: u64,
		pub xsdef: u64,
		pub reserved_14_15: u64,
		pub late_col: u64,
		pub reserved_18_19: u64,
		pub ptp_lost: u64,
		pub reserved_22_23: u64,
		pub xchange: u64,
		pub reserved_25_63: u64,
	pub cnf71xx: u64,
};

#[repr(C)] pub union cvmx_gmxx_tx_int_reg {
	pub u64: u64,
	#[repr(C)] pub struct cvmx_gmxx_tx_int_reg_s {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_25_63: u64,
		pub xchange: u64,
		pub ptp_lost: u64,
		pub late_col: u64,
		pub xsdef: u64,
		pub xscol: u64,
		pub reserved_6_7: u64,
		pub undflw: u64,
		pub reserved_1_1: u64,
		pub pko_nxa: u64,
// #else
		pub pko_nxa: u64,
		pub reserved_1_1: u64,
		pub undflw: u64,
		pub reserved_6_7: u64,
		pub xscol: u64,
		pub xsdef: u64,
		pub late_col: u64,
		pub ptp_lost: u64,
		pub xchange: u64,
		pub reserved_25_63: u64,
	pub s: u64,
	#[repr(C)] pub struct cvmx_gmxx_tx_int_reg_cn30xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_19_63: u64,
		pub late_col: u64,
		pub reserved_15_15: u64,
		pub xsdef: u64,
		pub reserved_11_11: u64,
		pub xscol: u64,
		pub reserved_5_7: u64,
		pub undflw: u64,
		pub reserved_1_1: u64,
		pub pko_nxa: u64,
// #else
		pub pko_nxa: u64,
		pub reserved_1_1: u64,
		pub undflw: u64,
		pub reserved_5_7: u64,
		pub xscol: u64,
		pub reserved_11_11: u64,
		pub xsdef: u64,
		pub reserved_15_15: u64,
		pub late_col: u64,
		pub reserved_19_63: u64,
	pub cn30xx: u64,
	#[repr(C)] pub struct cvmx_gmxx_tx_int_reg_cn31xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_15_63: u64,
		pub xsdef: u64,
		pub reserved_11_11: u64,
		pub xscol: u64,
		pub reserved_5_7: u64,
		pub undflw: u64,
		pub reserved_1_1: u64,
		pub pko_nxa: u64,
// #else
		pub pko_nxa: u64,
		pub reserved_1_1: u64,
		pub undflw: u64,
		pub reserved_5_7: u64,
		pub xscol: u64,
		pub reserved_11_11: u64,
		pub xsdef: u64,
		pub reserved_15_63: u64,
	pub cn31xx: u64,
	#[repr(C)] pub struct cvmx_gmxx_tx_int_reg_cn38xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_20_63: u64,
		pub late_col: u64,
		pub xsdef: u64,
		pub xscol: u64,
		pub reserved_6_7: u64,
		pub undflw: u64,
		pub ncb_nxa: u64,
		pub pko_nxa: u64,
// #else
		pub pko_nxa: u64,
		pub ncb_nxa: u64,
		pub undflw: u64,
		pub reserved_6_7: u64,
		pub xscol: u64,
		pub xsdef: u64,
		pub late_col: u64,
		pub reserved_20_63: u64,
	pub cn38xx: u64,
	#[repr(C)] pub struct cvmx_gmxx_tx_int_reg_cn38xxp2 {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_16_63: u64,
		pub xsdef: u64,
		pub xscol: u64,
		pub reserved_6_7: u64,
		pub undflw: u64,
		pub ncb_nxa: u64,
		pub pko_nxa: u64,
// #else
		pub pko_nxa: u64,
		pub ncb_nxa: u64,
		pub undflw: u64,
		pub reserved_6_7: u64,
		pub xscol: u64,
		pub xsdef: u64,
		pub reserved_16_63: u64,
	pub cn38xxp2: u64,
	#[repr(C)] pub struct cvmx_gmxx_tx_int_reg_cn52xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_20_63: u64,
		pub late_col: u64,
		pub xsdef: u64,
		pub xscol: u64,
		pub reserved_6_7: u64,
		pub undflw: u64,
		pub reserved_1_1: u64,
		pub pko_nxa: u64,
// #else
		pub pko_nxa: u64,
		pub reserved_1_1: u64,
		pub undflw: u64,
		pub reserved_6_7: u64,
		pub xscol: u64,
		pub xsdef: u64,
		pub late_col: u64,
		pub reserved_20_63: u64,
	pub cn52xx: u64,
	#[repr(C)] pub struct cvmx_gmxx_tx_int_reg_cn63xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_24_63: u64,
		pub ptp_lost: u64,
		pub late_col: u64,
		pub xsdef: u64,
		pub xscol: u64,
		pub reserved_6_7: u64,
		pub undflw: u64,
		pub reserved_1_1: u64,
		pub pko_nxa: u64,
// #else
		pub pko_nxa: u64,
		pub reserved_1_1: u64,
		pub undflw: u64,
		pub reserved_6_7: u64,
		pub xscol: u64,
		pub xsdef: u64,
		pub late_col: u64,
		pub ptp_lost: u64,
		pub reserved_24_63: u64,
	pub cn63xx: u64,
	#[repr(C)] pub struct cvmx_gmxx_tx_int_reg_cn68xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_25_63: u64,
		pub xchange: u64,
		pub ptp_lost: u64,
		pub late_col: u64,
		pub xsdef: u64,
		pub xscol: u64,
		pub reserved_6_7: u64,
		pub undflw: u64,
		pub pko_nxp: u64,
		pub pko_nxa: u64,
// #else
		pub pko_nxa: u64,
		pub pko_nxp: u64,
		pub undflw: u64,
		pub reserved_6_7: u64,
		pub xscol: u64,
		pub xsdef: u64,
		pub late_col: u64,
		pub ptp_lost: u64,
		pub xchange: u64,
		pub reserved_25_63: u64,
	pub cn68xx: u64,
	#[repr(C)] pub struct cvmx_gmxx_tx_int_reg_cnf71xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_25_63: u64,
		pub xchange: u64,
		pub reserved_22_23: u64,
		pub ptp_lost: u64,
		pub reserved_18_19: u64,
		pub late_col: u64,
		pub reserved_14_15: u64,
		pub xsdef: u64,
		pub reserved_10_11: u64,
		pub xscol: u64,
		pub reserved_4_7: u64,
		pub undflw: u64,
		pub reserved_1_1: u64,
		pub pko_nxa: u64,
// #else
		pub pko_nxa: u64,
		pub reserved_1_1: u64,
		pub undflw: u64,
		pub reserved_4_7: u64,
		pub xscol: u64,
		pub reserved_10_11: u64,
		pub xsdef: u64,
		pub reserved_14_15: u64,
		pub late_col: u64,
		pub reserved_18_19: u64,
		pub ptp_lost: u64,
		pub reserved_22_23: u64,
		pub xchange: u64,
		pub reserved_25_63: u64,
	pub cnf71xx: u64,
};

#[repr(C)] pub union cvmx_gmxx_tx_ovr_bp {
	pub u64: u64,
	#[repr(C)] pub struct cvmx_gmxx_tx_ovr_bp_s {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_48_63: u64,
		pub tx_prt_bp: u64,
		pub reserved_12_31: u64,
		pub en: u64,
		pub bp: u64,
		pub ign_fu64: u64,
// #else
		pub ign_fu64: u64,
		pub bp: u64,
		pub en: u64,
		pub reserved_12_31: u64,
		pub tx_prt_bp: u64,
		pub reserved_48_63: u64,
	pub s: u64,
	#[repr(C)] pub struct cvmx_gmxx_tx_ovr_bp_cn30xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_11_63: u64,
		pub en: u64,
		pub reserved_7_7: u64,
		pub bp: u64,
		pub reserved_3_3: u64,
		pub ign_fu64: u64,
// #else
		pub ign_fu64: u64,
		pub reserved_3_3: u64,
		pub bp: u64,
		pub reserved_7_7: u64,
		pub en: u64,
		pub reserved_11_63: u64,
	pub cn30xx: u64,
	#[repr(C)] pub struct cvmx_gmxx_tx_ovr_bp_cn38xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_12_63: u64,
		pub en: u64,
		pub bp: u64,
		pub ign_fu64: u64,
// #else
		pub ign_fu64: u64,
		pub bp: u64,
		pub en: u64,
		pub reserved_12_63: u64,
	pub cn38xx: u64,
	#[repr(C)] pub struct cvmx_gmxx_tx_ovr_bp_cnf71xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_48_63: u64,
		pub tx_prt_bp: u64,
		pub reserved_10_31: u64,
		pub en: u64,
		pub reserved_6_7: u64,
		pub bp: u64,
		pub reserved_2_3: u64,
		pub ign_fu64: u64,
// #else
		pub ign_fu64: u64,
		pub reserved_2_3: u64,
		pub bp: u64,
		pub reserved_6_7: u64,
		pub en: u64,
		pub reserved_10_31: u64,
		pub tx_prt_bp: u64,
		pub reserved_48_63: u64,
	pub cnf71xx: u64,
};

#[repr(C)] pub union cvmx_gmxx_tx_prts {
	pub u64: u64,
	#[repr(C)] pub struct cvmx_gmxx_tx_prts_s {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_5_63: u64,
		pub prts: u64,
// #else
		pub prts: u64,
		pub reserved_5_63: u64,
	pub s: u64,
};

#[repr(C)] pub union cvmx_gmxx_tx_spi_ctl {
	pub u64: u64,
	#[repr(C)] pub struct cvmx_gmxx_tx_spi_ctl_s {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_2_63: u64,
		pub tpa_clr: u64,
		pub cont_pkt: u64,
// #else
		pub cont_pkt: u64,
		pub tpa_clr: u64,
		pub reserved_2_63: u64,
	pub s: u64,
};

#[repr(C)] pub union cvmx_gmxx_tx_spi_max {
	pub u64: u64,
	#[repr(C)] pub struct cvmx_gmxx_tx_spi_max_s {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_23_63: u64,
		pub slice: u64,
		pub max2: u64,
		pub max1: u64,
// #else
		pub max1: u64,
		pub max2: u64,
		pub slice: u64,
		pub reserved_23_63: u64,
	pub s: u64,
	#[repr(C)] pub struct cvmx_gmxx_tx_spi_max_cn38xx {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_16_63: u64,
		pub max2: u64,
		pub max1: u64,
// #else
		pub max1: u64,
		pub max2: u64,
		pub reserved_16_63: u64,
	pub cn38xx: u64,
};

#[repr(C)] pub union cvmx_gmxx_tx_spi_thresh {
	pub u64: u64,
	#[repr(C)] pub struct cvmx_gmxx_tx_spi_thresh_s {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_6_63: u64,
		pub thresh: u64,
// #else
		pub thresh: u64,
		pub reserved_6_63: u64,
	pub s: u64,
};

#[repr(C)] pub union cvmx_gmxx_tx_xaui_ctl {
	pub u64: u64,
	#[repr(C)] pub struct cvmx_gmxx_tx_xaui_ctl_s {
// #ifdef __BIG_ENDIAN_BITFIELD
		pub reserved_11_63: u64,
		pub hg_pause_hgi: u64,
		pub hg_en: u64,
		pub reserved_7_7: u64,
		pub ls_byp: u64,
		pub ls: u64,
		pub reserved_2_3: u64,
		pub uni_en: u64,
		pub dic_en: u64,
// #else
		pub dic_en: u64,
		pub uni_en: u64,
		pub reserved_2_3: u64,
		pub ls: u64,
		pub ls_byp: u64,
		pub reserved_7_7: u64,
		pub hg_en: u64,
		pub hg_pause_hgi: u64,
		pub reserved_11_63: u64,
	pub s: u64,
};


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
