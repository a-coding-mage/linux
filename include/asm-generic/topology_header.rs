/*
 * linux/include/asm-generic/topology.h
 *
 * Written by: Matthew Dobson, IBM Corporation
 *
 * Copyright (C) 2002, IBM Corp.
 *
 * All rights reserved.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE, GOOD TITLE or
 * NON INFRINGEMENT.  See the GNU General Public License for more
 * details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.
 *
 * Send feedback to <colpatch@us.ibm.com>
 */

/* The CONFIG_NUMA conditional is supplied by the build configuration. */
#[cfg(not(CONFIG_NUMA))]
mod non_numa_topology {
    /* Other architectures wishing to use this simple topology API should fill
       in the below functions as appropriate in their own <asm/topology.h> file. */

    #[macro_export]
    macro_rules! cpu_to_node {
        ($cpu:expr) => {{ let _ = $cpu; 0 }};
    }

    #[macro_export]
    macro_rules! set_numa_node {
        ($node:expr) => {{ let _ = $node; }};
    }

    #[macro_export]
    macro_rules! set_cpu_numa_node {
        ($cpu:expr, $node:expr) => {{ let _ = $cpu; let _ = $node; }};
    }

    #[macro_export]
    macro_rules! cpu_to_mem {
        ($cpu:expr) => {{ let _ = $cpu; 0 }};
    }

    #[macro_export]
    macro_rules! cpumask_of_node {
        ($node:expr) => {{ let _ = $node; cpu_online_mask }};
    }

    #[macro_export]
    macro_rules! pcibus_to_node {
        ($bus:expr) => {{ let _ = $bus; -1 }};
    }

    #[macro_export]
    macro_rules! cpumask_of_pcibus {
        ($bus:expr) => {{
            if pcibus_to_node!($bus) == -1 {
                cpu_all_mask
            } else {
                cpumask_of_node!(pcibus_to_node!($bus))
            }
        }};
    }
}

/* CONFIG_NUMA or CONFIG_HAVE_MEMORYLESS_NODES may be supplied by the build. */
#[cfg(any(not(CONFIG_NUMA), not(CONFIG_HAVE_MEMORYLESS_NODES)))]
mod memoryless_topology {
    #[macro_export]
    macro_rules! set_numa_mem {
        ($node:expr) => {{ let _ = $node; }};
    }

    #[macro_export]
    macro_rules! set_cpu_numa_mem {
        ($cpu:expr, $node:expr) => {{ let _ = $cpu; let _ = $node; }};
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
