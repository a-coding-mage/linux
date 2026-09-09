/*
 *  DaVinci Power & Sleep Controller (PSC) defines
 *
 *  Copyright (C) 2006 Texas Instruments.
 *
 *  This program is free software; you can redistribute  it and/or modify it
 *  under the terms of  the GNU General  Public License as published by the
 *  Free Software Foundation; either version 2 of the License, or (at your
 *  option) any later version.
 *
 *  THIS SOFTWARE IS PROVIDED ``AS IS'' AND ANY EXPRESS OR IMPLIED
 *  WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 *  MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN
 *  NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT,
 *  INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 *  NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF
 *  USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON
 *  ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 *  (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 *  THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 *  You should have received a copy of the GNU General Public License along
 *  with this program; if not, write to the Free Software Foundation, Inc.,
 *  675 Mass Ave, Cambridge, MA 02139, USA.
 */

/* Power and Sleep Controller (PSC) Domains */
pub const DAVINCI_GPSC_ARMDOMAIN: u32 = 0;
pub const DAVINCI_GPSC_DSPDOMAIN: u32 = 1;

pub const DAVINCI_LPSC_VPSSMSTR: u32 = 0;
pub const DAVINCI_LPSC_VPSSSLV: u32 = 1;
pub const DAVINCI_LPSC_TPCC: u32 = 2;
pub const DAVINCI_LPSC_TPTC0: u32 = 3;
pub const DAVINCI_LPSC_TPTC1: u32 = 4;
pub const DAVINCI_LPSC_EMAC: u32 = 5;
pub const DAVINCI_LPSC_EMAC_WRAPPER: u32 = 6;
pub const DAVINCI_LPSC_USB: u32 = 9;
pub const DAVINCI_LPSC_ATA: u32 = 10;
pub const DAVINCI_LPSC_VLYNQ: u32 = 11;
pub const DAVINCI_LPSC_UHPI: u32 = 12;
pub const DAVINCI_LPSC_DDR_EMIF: u32 = 13;
pub const DAVINCI_LPSC_AEMIF: u32 = 14;
pub const DAVINCI_LPSC_MMC_SD: u32 = 15;
pub const DAVINCI_LPSC_McBSP: u32 = 17;
pub const DAVINCI_LPSC_I2C: u32 = 18;
pub const DAVINCI_LPSC_UART0: u32 = 19;
pub const DAVINCI_LPSC_UART1: u32 = 20;
pub const DAVINCI_LPSC_UART2: u32 = 21;
pub const DAVINCI_LPSC_SPI: u32 = 22;
pub const DAVINCI_LPSC_PWM0: u32 = 23;
pub const DAVINCI_LPSC_PWM1: u32 = 24;
pub const DAVINCI_LPSC_PWM2: u32 = 25;
pub const DAVINCI_LPSC_GPIO: u32 = 26;
pub const DAVINCI_LPSC_TIMER0: u32 = 27;
pub const DAVINCI_LPSC_TIMER1: u32 = 28;
pub const DAVINCI_LPSC_TIMER2: u32 = 29;
pub const DAVINCI_LPSC_SYSTEM_SUBSYS: u32 = 30;
pub const DAVINCI_LPSC_ARM: u32 = 31;
pub const DAVINCI_LPSC_SCR2: u32 = 32;
pub const DAVINCI_LPSC_SCR3: u32 = 33;
pub const DAVINCI_LPSC_SCR4: u32 = 34;
pub const DAVINCI_LPSC_CROSSBAR: u32 = 35;
pub const DAVINCI_LPSC_CFG27: u32 = 36;
pub const DAVINCI_LPSC_CFG3: u32 = 37;
pub const DAVINCI_LPSC_CFG5: u32 = 38;
pub const DAVINCI_LPSC_GEM: u32 = 39;
pub const DAVINCI_LPSC_IMCOP: u32 = 40;

/* PSC0 defines */
pub const DA8XX_LPSC0_TPCC: u32 = 0;
pub const DA8XX_LPSC0_TPTC0: u32 = 1;
pub const DA8XX_LPSC0_TPTC1: u32 = 2;
pub const DA8XX_LPSC0_EMIF25: u32 = 3;
pub const DA8XX_LPSC0_SPI0: u32 = 4;
pub const DA8XX_LPSC0_MMC_SD: u32 = 5;
pub const DA8XX_LPSC0_AINTC: u32 = 6;
pub const DA8XX_LPSC0_ARM_RAM_ROM: u32 = 7;
pub const DA8XX_LPSC0_SECU_MGR: u32 = 8;
pub const DA8XX_LPSC0_UART0: u32 = 9;
pub const DA8XX_LPSC0_SCR0_SS: u32 = 10;
pub const DA8XX_LPSC0_SCR1_SS: u32 = 11;
pub const DA8XX_LPSC0_SCR2_SS: u32 = 12;
pub const DA8XX_LPSC0_PRUSS: u32 = 13;
pub const DA8XX_LPSC0_ARM: u32 = 14;
pub const DA8XX_LPSC0_GEM: u32 = 15;

/* PSC1 defines */
pub const DA850_LPSC1_TPCC1: u32 = 0;
pub const DA8XX_LPSC1_USB20: u32 = 1;
pub const DA8XX_LPSC1_USB11: u32 = 2;
pub const DA8XX_LPSC1_GPIO: u32 = 3;
pub const DA8XX_LPSC1_UHPI: u32 = 4;
pub const DA8XX_LPSC1_CPGMAC: u32 = 5;
pub const DA8XX_LPSC1_EMIF3C: u32 = 6;
pub const DA8XX_LPSC1_McASP0: u32 = 7;
pub const DA850_LPSC1_SATA: u32 = 8;
pub const DA850_LPSC1_VPIF: u32 = 9;
pub const DA8XX_LPSC1_SPI1: u32 = 10;
pub const DA8XX_LPSC1_I2C: u32 = 11;
pub const DA8XX_LPSC1_UART1: u32 = 12;
pub const DA8XX_LPSC1_UART2: u32 = 13;
pub const DA850_LPSC1_McBSP0: u32 = 14;
pub const DA850_LPSC1_McBSP1: u32 = 15;
pub const DA8XX_LPSC1_LCDC: u32 = 16;
pub const DA8XX_LPSC1_PWM: u32 = 17;
pub const DA850_LPSC1_MMC_SD1: u32 = 18;
pub const DA8XX_LPSC1_ECAP: u32 = 20;
pub const DA850_LPSC1_TPTC2: u32 = 21;
pub const DA850_LPSC1_SCR_P0_SS: u32 = 24;
pub const DA850_LPSC1_SCR_P1_SS: u32 = 25;
pub const DA850_LPSC1_CR_P3_SS: u32 = 26;
pub const DA8XX_LPSC1_L3_CBA_RAM: u32 = 31;

/* PSC register offsets */
pub const EPCPR: u32 = 0x070;
pub const PTCMD: u32 = 0x120;
pub const PTSTAT: u32 = 0x128;
pub const PDSTAT: u32 = 0x200;
pub const PDCTL: u32 = 0x300;
pub const MDSTAT: u32 = 0x800;
pub const MDCTL: u32 = 0xA00;

/* PSC module states */
pub const PSC_STATE_SWRSTDISABLE: u32 = 0;
pub const PSC_STATE_SYNCRST: u32 = 1;
pub const PSC_STATE_DISABLE: u32 = 2;
pub const PSC_STATE_ENABLE: u32 = 3;

pub const MDSTAT_STATE_MASK: u32 = 0x3f;
pub const PDSTAT_STATE_MASK: u32 = 0x1f;
pub const MDCTL_LRST: u32 = 1u32 << 8;
pub const MDCTL_FORCE: u32 = 1u32 << 31;
pub const PDCTL_NEXT: u32 = 1u32 << 0;
pub const PDCTL_EPCGOOD: u32 = 1u32 << 8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
