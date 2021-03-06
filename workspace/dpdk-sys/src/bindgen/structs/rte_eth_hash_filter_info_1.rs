// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_eth_hash_filter_info_1
{
	pub enable: BindgenUnionField<u8>,
	pub global_conf: BindgenUnionField<rte_eth_hash_global_conf>,
	pub input_set_conf: BindgenUnionField<rte_eth_input_set_conf>,
	pub bindgen_union_field: [u64; 65usize],
}

impl Default for rte_eth_hash_filter_info_1
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_eth_hash_filter_info_1
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_eth_hash_filter_info_1 {{ union }}")
	}
}
