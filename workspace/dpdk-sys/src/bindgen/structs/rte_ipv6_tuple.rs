// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_ipv6_tuple
{
	pub src_addr: [u8; 16usize],
	pub dst_addr: [u8; 16usize],
	pub _1: rte_ipv6_tuple_1,
}

impl Default for rte_ipv6_tuple
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_ipv6_tuple
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_ipv6_tuple {{ src_addr: {:?}, dst_addr: {:?}, _1: {:?} }}", self.src_addr, self.dst_addr, self._1)
	}
}
