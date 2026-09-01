/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Landlock variants for three processes with various domains.
 *
 * Copyright © 2024 Tahera Fahimi <fahimitahera@gmail.com>
 */

#[repr(C)]
pub enum sandbox_type {
	NO_SANDBOX,
	SCOPE_SANDBOX,
	/* Any other type of sandboxing domain */
	OTHER_SANDBOX,
}

pub struct scoped_vs_unscoped {
	pub domain_all: i32,
	pub domain_parent: i32,
	pub domain_children: i32,
	pub domain_child: i32,
	pub domain_grand_child: i32,
}

/*
 * .-----------------.
 * |         ####### |  P3 -> P2 : allow
 * |   P1----# P2  # |  P3 -> P1 : deny
 * |         #  |  # |
 * |         # P3  # |
 * |         ####### |
 * '-----------------'
 */
pub const deny_scoped: scoped_vs_unscoped = scoped_vs_unscoped {
	domain_all: sandbox_type::OTHER_SANDBOX as i32,
	domain_parent: sandbox_type::NO_SANDBOX as i32,
	domain_children: sandbox_type::SCOPE_SANDBOX as i32,
	domain_child: sandbox_type::NO_SANDBOX as i32,
	domain_grand_child: sandbox_type::NO_SANDBOX as i32,
};

/*
 * ###################
 * #         ####### #  P3 -> P2 : allow
 * #   P1----# P2  # #  P3 -> P1 : deny
 * #         #  |  # #
 * #         # P3  # #
 * #         ####### #
 * ###################
 */
pub const all_scoped: scoped_vs_unscoped = scoped_vs_unscoped {
	domain_all: sandbox_type::SCOPE_SANDBOX as i32,
	domain_parent: sandbox_type::NO_SANDBOX as i32,
	domain_children: sandbox_type::SCOPE_SANDBOX as i32,
	domain_child: sandbox_type::NO_SANDBOX as i32,
	domain_grand_child: sandbox_type::NO_SANDBOX as i32,
};

/*
 * .-----------------.
 * |         .-----. |  P3 -> P2 : allow
 * |   P1----| P2  | |  P3 -> P1 : allow
 * |         |     | |
 * |         | P3  | |
 * |         '-----' |
 * '-----------------'
 */
pub const allow_with_other_domain: scoped_vs_unscoped = scoped_vs_unscoped {
	domain_all: sandbox_type::OTHER_SANDBOX as i32,
	domain_parent: sandbox_type::NO_SANDBOX as i32,
	domain_children: sandbox_type::OTHER_SANDBOX as i32,
	domain_child: sandbox_type::NO_SANDBOX as i32,
	domain_grand_child: sandbox_type::NO_SANDBOX as i32,
};

/*
 *  .----.    ######   P3 -> P2 : allow
 *  | P1 |----# P2 #   P3 -> P1 : allow
 *  '----'    ######
 *              |
 *              P3
 */
pub const allow_with_one_domain: scoped_vs_unscoped = scoped_vs_unscoped {
	domain_all: sandbox_type::NO_SANDBOX as i32,
	domain_parent: sandbox_type::OTHER_SANDBOX as i32,
	domain_children: sandbox_type::NO_SANDBOX as i32,
	domain_child: sandbox_type::SCOPE_SANDBOX as i32,
	domain_grand_child: sandbox_type::NO_SANDBOX as i32,
};

/*
 *  ######    .-----.   P3 -> P2 : allow
 *  # P1 #----| P2  |   P3 -> P1 : allow
 *  ######    '-----'
 *              |
 *              P3
 */
pub const allow_with_grand_parent_scoped: scoped_vs_unscoped = scoped_vs_unscoped {
	domain_all: sandbox_type::NO_SANDBOX as i32,
	domain_parent: sandbox_type::SCOPE_SANDBOX as i32,
	domain_children: sandbox_type::NO_SANDBOX as i32,
	domain_child: sandbox_type::OTHER_SANDBOX as i32,
	domain_grand_child: sandbox_type::NO_SANDBOX as i32,
};

/*
 *  ######    ######   P3 -> P2 : allow
 *  # P1 #----# P2 #   P3 -> P1 : allow
 *  ######    ######
 *               |
 *             .----.
 *             | P3 |
 *             '----'
 */
pub const allow_with_parents_domain: scoped_vs_unscoped = scoped_vs_unscoped {
	domain_all: sandbox_type::NO_SANDBOX as i32,
	domain_parent: sandbox_type::SCOPE_SANDBOX as i32,
	domain_children: sandbox_type::NO_SANDBOX as i32,
	domain_child: sandbox_type::SCOPE_SANDBOX as i32,
	domain_grand_child: sandbox_type::NO_SANDBOX as i32,
};

/*
 *  ######		P3 -> P2 : deny
 *  # P1 #----P2	P3 -> P1 : deny
 *  ######     |
 *	       |
 *	     ######
 *           # P3 #
 *           ######
 */
pub const deny_with_self_and_grandparent_domain: scoped_vs_unscoped = scoped_vs_unscoped {
	domain_all: sandbox_type::NO_SANDBOX as i32,
	domain_parent: sandbox_type::SCOPE_SANDBOX as i32,
	domain_children: sandbox_type::NO_SANDBOX as i32,
	domain_child: sandbox_type::NO_SANDBOX as i32,
	domain_grand_child: sandbox_type::SCOPE_SANDBOX as i32,
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
