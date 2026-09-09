/*
 * arch/arm/mach-omap1/board-ams-delta.h
 *
 * Copyright (C) 2006 Jonathan McDowell <noodles@earth.li>
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the
 * Free Software Foundation; either version 2 of the License, or (at your
 * option) any later version.
 *
 * THIS SOFTWARE IS PROVIDED ``AS IS'' AND ANY EXPRESS OR IMPLIED
 * WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN
 * NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT,
 * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF
 * USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON
 * ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 675 Mass Ave, Cambridge, MA 02139, USA.
 */

// The original declarations are enabled when CONFIG_MACH_AMS_DELTA is defined.
#[cfg(feature = "CONFIG_MACH_AMS_DELTA")]
pub const AMS_DELTA_GPIO_PIN_KEYBRD_DATA: i32 = 0;
#[cfg(feature = "CONFIG_MACH_AMS_DELTA")]
pub const AMS_DELTA_GPIO_PIN_KEYBRD_CLK: i32 = 1;
#[cfg(feature = "CONFIG_MACH_AMS_DELTA")]
pub const AMS_DELTA_GPIO_PIN_MODEM_IRQ: i32 = 2;
#[cfg(feature = "CONFIG_MACH_AMS_DELTA")]
pub const AMS_DELTA_GPIO_PIN_HOOK_SWITCH: i32 = 4;
#[cfg(feature = "CONFIG_MACH_AMS_DELTA")]
pub const AMS_DELTA_GPIO_PIN_SCARD_NOFF: i32 = 6;
#[cfg(feature = "CONFIG_MACH_AMS_DELTA")]
pub const AMS_DELTA_GPIO_PIN_SCARD_IO: i32 = 7;
#[cfg(feature = "CONFIG_MACH_AMS_DELTA")]
pub const AMS_DELTA_GPIO_PIN_CONFIG: i32 = 11;
#[cfg(feature = "CONFIG_MACH_AMS_DELTA")]
pub const AMS_DELTA_GPIO_PIN_NAND_RB: i32 = 12;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
