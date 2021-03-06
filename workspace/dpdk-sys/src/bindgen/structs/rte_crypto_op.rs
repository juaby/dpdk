// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_crypto_op
{
	pub type_: u8,
	pub status: u8,
	pub sess_type: u8,
	pub private_data_offset: u16,
	pub reserved: [u8; 3usize],
	pub mempool: *mut rte_mempool,
	pub phys_addr: rte_iova_t,
	pub _1: rte_crypto_op_1,
}

impl Default for rte_crypto_op
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_crypto_op
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_crypto_op {{ reserved: {:?}, mempool: {:?}, _1: {:?} }}", self.reserved, self.mempool, self._1)
	}
}
