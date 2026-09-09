// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * QNAP TS-x09 Boards common functions
 *
 * Maintainers: Lennert Buytenhek <buytenh@marvell.com>
 *              Byron Bradley <byron.bbradley@gmail.com>
 */

// Dependencies supplied by the surrounding kernel translation.

/*****************************************************************************
 * QNAP TS-x09 specific power off method via UART1-attached PIC
 *****************************************************************************/

const UART1_REG = |x: u32| UART1_VIRT_BASE + ((x) << 2);

pub unsafe fn qnap_tsx09_power_off() {
    /* 19200 baud divisor */
    let divisor: u32 = (orion5x_tclk + (8 * 19200)) / (16 * 19200);

    pr_info!("%s: triggering power-off...\n", "qnap_tsx09_power_off");

    /* hijack uart1 and reset into sane state (19200,8n1) */
    writel(0x83, UART1_REG(UART_LCR));
    writel(divisor & 0xff, UART1_REG(UART_DLL));
    writel((divisor >> 8) & 0xff, UART1_REG(UART_DLM));
    writel(0x03, UART1_REG(UART_LCR));
    writel(0x00, UART1_REG(UART_IER));
    writel(0x00, UART1_REG(UART_FCR));
    writel(0x00, UART1_REG(UART_MCR));

    /* send the power-off command 'A' to PIC */
    writel('A' as u32, UART1_REG(UART_TX));
}

/*****************************************************************************
 * Ethernet
 *****************************************************************************/

pub static mut qnap_tsx09_eth_data: mv643xx_eth_platform_data =
    mv643xx_eth_platform_data {
        phy_addr: MV643XX_ETH_PHY_ADDR(8),
    };

unsafe fn qnap_tsx09_parse_hex_nibble(n: i8) -> i32 {
    if n >= b'0' as i8 && n <= b'9' as i8 {
        return (n - b'0' as i8) as i32;
    }

    if n >= b'A' as i8 && n <= b'F' as i8 {
        return (n - b'A' as i8 + 10) as i32;
    }

    if n >= b'a' as i8 && n <= b'f' as i8 {
        return (n - b'a' as i8 + 10) as i32;
    }

    -1
}

unsafe fn qnap_tsx09_parse_hex_byte(b: *const i8) -> i32 {
    let hi: i32;
    let lo: i32;

    hi = qnap_tsx09_parse_hex_nibble(*b.add(0));
    lo = qnap_tsx09_parse_hex_nibble(*b.add(1));

    if hi < 0 || lo < 0 {
        return -1;
    }

    (hi << 4) | lo
}

unsafe fn qnap_tsx09_check_mac_addr(addr_str: *const i8) -> i32 {
    let mut addr = [0u8; 6];

    for i in 0..6 {
        let byte: i32;

        /*
         * Enforce "xx:xx:xx:xx:xx:xx\n" format.
         */
        let expected = if i < 5 { b':' as i8 } else { b'\n' as i8 };
        if *addr_str.add((i * 3) + 2) != expected {
            return -1;
        }

        byte = qnap_tsx09_parse_hex_byte(addr_str.add(i * 3));
        if byte < 0 {
            return -1;
        }
        addr[i] = byte as u8;
    }

    printk!(KERN_INFO, "tsx09: found ethernet mac address %pM\n", addr.as_ptr());

    memcpy(qnap_tsx09_eth_data.mac_addr.as_mut_ptr(), addr.as_ptr(), 6);

    0
}

/*
 * The 'NAS Config' flash partition has an ext2 filesystem which
 * contains a file that has the ethernet MAC address in plain text
 * (format "xx:xx:xx:xx:xx:xx\n").
 */
pub unsafe fn qnap_tsx09_find_mac_addr(mem_base: u32, size: u32) {
    let mut addr: c_ulong = mem_base as c_ulong;

    while addr < (mem_base + size) as c_ulong {
        let mut nor_page: *mut i8;
        let mut ret: i32 = 0;

        nor_page = ioremap(addr, 1024);
        if !nor_page.is_null() {
            ret = qnap_tsx09_check_mac_addr(nor_page);
            iounmap(nor_page);
        }

        if ret == 0 {
            break;
        }
        addr += 1024;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
