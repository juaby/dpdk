// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy, Clone)]
pub struct rte_cryptodev_sym_capability_idx
{
	pub type_: rte_crypto_sym_xform_type,
	pub algo: rte_cryptodev_sym_capability_idx_1,
}

impl Default for rte_cryptodev_sym_capability_idx
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_cryptodev_sym_capability_idx
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_cryptodev_sym_capability_idx {{ type: {:?}, algo: {:?} }}", self.type_, self.algo)
	}
}
