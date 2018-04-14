// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum PciDeviceListColour
{
	Blacklist,
	Whitelist,
}

impl PciDeviceListColour
{
	const_cstr!
	{
		__pci_blacklist = "--pci-blacklist";    // aka -b
		__pci_whitelist = "--pci-whitelist";    // aka -w
	}
	
	#[inline(always)]
	fn as_initialisation_argument(self) -> ConstCStr
	{
		use self::PciDeviceListColour::*;
		
		match self
		{
			Blacklist => Self::__pci_blacklist,
			Whitelist => Self::__pci_whitelist,
		}
	}
}