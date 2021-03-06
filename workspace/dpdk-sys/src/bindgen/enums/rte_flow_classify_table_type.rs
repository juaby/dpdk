// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum rte_flow_classify_table_type
{
	RTE_FLOW_CLASSIFY_TABLE_TYPE_NONE = 1,
	RTE_FLOW_CLASSIFY_TABLE_ACL_IP4_5TUPLE = 2,
	RTE_FLOW_CLASSIFY_TABLE_ACL_VLAN_IP4_5TUPLE = 4,
	RTE_FLOW_CLASSIFY_TABLE_ACL_QINQ_IP4_5TUPLE = 8,
}
