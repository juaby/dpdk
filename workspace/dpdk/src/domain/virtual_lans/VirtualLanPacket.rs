// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An IEEE 802.1Q virtual LAN tagged packet.
///
/// This is a specialized structure designed to represent a buffer of packet data.
#[repr(C, packed)]
pub struct VirtualLanPacket
{
	/// Header.
	pub header: VirtualLanPacketHeader,
	
	/// Layer 3 packet.
	pub layer_3_packet: Layer3Packet,
}

impl VirtualLanPacket
{
	#[inline(always)]
	pub(crate) fn potentially_invalid_ether_type(&self) -> EtherType
	{
		unsafe { self.header.ether_type_or_legacy_ethernet_frame_size.ether_type }
	}
}