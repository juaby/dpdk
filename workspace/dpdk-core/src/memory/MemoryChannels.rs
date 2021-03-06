// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Restricted enumeration of number of memory channels.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub enum MemoryChannels
{
	#[allow(missing_docs)]
	One = 1,
	
	#[allow(missing_docs)]
	Two = 2,
	
	#[allow(missing_docs)]
	Three = 3,
	
	#[allow(missing_docs)]
	Four = 4,
}

impl MemoryChannels
{
	/// Configured number of memory channels.
	///
	/// Returns None if differs across memory segments or devices.
	#[inline(always)]
	pub fn configured_number_of_memory_channels() -> Option<MemoryChannels>
	{
		let channels = unsafe { rte_memory_get_nchannel() };
		if channels == 0
		{
			return None
		}
		if channels > 4
		{
			panic!("Invalid number of memory channels '{}'", channels)
		}
		Some(unsafe { transmute(channels) })
	}
	
	/// As an initialization argument.
	#[inline(always)]
	pub fn as_initialization_argument(self) -> ConstCStr
	{
		use self::MemoryChannels::*;
		
		match self
		{
			One => ConstCStr(b"1\0"),
			Two => ConstCStr(b"2\0"),
			Three => ConstCStr(b"3\0"),
			Four => ConstCStr(b"4\0"),
		}
	}
}
