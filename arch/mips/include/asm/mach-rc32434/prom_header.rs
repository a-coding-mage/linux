/*
 *  Definitions for the PROM
 *
 *  Copyright 2002 Ryan Holm <ryan.holmQVist@idt.com>
 *  Copyright 2008 Florian Fainelli <florian@openwrt.org>
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

/// Equivalent to the C `PROM_ENTRY(x)` macro.
pub const fn PROM_ENTRY(x: u32) -> u32 {
    0xbfc0_0000u32.wrapping_add(x.wrapping_mul(8))
}

pub const SR_NMI: u32 = 0x0018_0000;
pub const SERIAL_SPEED_ENTRY: u32 = 0x0000_0001;

pub const FREQ_TAG: &str = "HZ=";
pub const KMAC_TAG: &str = "kmac=";
pub const MEM_TAG: &str = "mem=";
pub const BOARD_TAG: &str = "board=";

pub const BOARD_RB532: &str = "500";
pub const BOARD_RB532A: &str = "500r5";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
