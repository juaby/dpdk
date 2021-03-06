// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Ethernet device's capabilities.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct EthernetDeviceCapabilities
{
	driver_name: &'static str,
	interface_index: Option<InterfaceIndex>,
	maximum_queue_pairs: u16,
	maximum_receive_packet_length: u16,
	receive_device_hardware_offloading_flags: ReceiveHardwareOffloadingFlags,
	ethernet_device_receive_queue_capabilities: EthernetDeviceReceiveQueueCapabilities,
	receive_queue_ring_size_constraints: QueueRingSizeConstraints<ReceiveQueueRingSize>,
	transmit_device_hardware_offloading_flags: TransmitHardwareOffloadingFlags,
	ethernet_device_transmit_queue_capabilities: EthernetDeviceTransmitQueueCapabilities,
	transmit_queue_ring_size_constraints: QueueRingSizeConstraints<TransmitQueueRingSize>,
	receive_side_scaling_offload_flow: ReceiveSideScalingOffloadFlow,
	receive_side_scaling_is_unavailable: bool,
	receive_side_scaling_hash_key_size: Option<ReceiveSideScalingHashKeySize>,
	redirection_table_number_of_entries: Option<RedirectionTableNumberOfEntries>,
	device_capability_flags: DeviceCapabilityFlags,
	device_information_flags: DeviceInformationFlags,
	extended_statistics_names: Vec<&'static str>,
	firmware_version: Option<String>,
	maximum_transmission_unit: MaximumTransmissionUnitSize,
}

impl EthernetDeviceCapabilities
{
	pub(crate) const ImmediateStart: u8 = 0;
	
	#[inline(always)]
	pub(crate) fn from(mut dpdk_information: rte_eth_dev_info, extended_statistics_names: Vec<&'static str>, firmware_version: Option<String>, data: &rte_eth_dev_data) -> Self
	{
		let driver_name = unsafe { CStr::from_ptr(dpdk_information.driver_name) }.to_str().unwrap();
		
		let receive_side_scaling_is_unavailable = dpdk_information.flow_type_rss_offloads == 0 || dpdk_information.reta_size == 0;
		
		Self
		{
			driver_name,
			interface_index: NonZeroU32::new(dpdk_information.if_index).map(|value| InterfaceIndex::from(value)),
			maximum_queue_pairs:
			{
				let maximum_transmit_queue = dpdk_information.max_tx_queues;
				debug_assert_ne!(maximum_transmit_queue, 0, "maximum transmit queues is zero");
				
				let possibly_buggy_max_rx_queues = dpdk_information.max_rx_queues;
				debug_assert_ne!(possibly_buggy_max_rx_queues, 0, "maximum receive queues is zero");
				
				let maximum_receieve_queues = match driver_name
				{
					"rte_i40e_pmd" => min(possibly_buggy_max_rx_queues, 64),
					"rte_i40evf_pmd" => min(possibly_buggy_max_rx_queues, 16),
					"rte_ixgbe_pmd" => min(possibly_buggy_max_rx_queues, 16),
					"rte_ixgbevf_pmd" => min(possibly_buggy_max_rx_queues, 4),
					
					// The e1000 82576 supports 16 receive queues, but the RSS rte_flow action only supports 8.
					// This is probably a bug.
					"rte_igb_pmd" | "rte_igbvf_pmd" => min(possibly_buggy_max_rx_queues, 8),
					
					_ => possibly_buggy_max_rx_queues,
				};
				dpdk_information.max_rx_queues = maximum_receieve_queues;
				
				min(maximum_receieve_queues, maximum_transmit_queue)
			},
			maximum_receive_packet_length:
			{
				// Some drivers use nonsense values that exceed super jumbo frame sizes for `max_rx_pktlen`.
				let possibly_buggy_max_rx_pktlen = dpdk_information.max_rx_pktlen;
				let into: u16 = EthernetFrameLength::MaximumIncludingCyclicRedundancyCheckWithJumboFrames.into();
				let maximum_receive_packet_length = min(possibly_buggy_max_rx_pktlen, into as u32);
				dpdk_information.max_rx_pktlen = maximum_receive_packet_length;
				maximum_receive_packet_length as u16
			},
			receive_device_hardware_offloading_flags: ReceiveHardwareOffloadingFlags::from_bits_truncate(dpdk_information.rx_offload_capa),
			ethernet_device_receive_queue_capabilities: EthernetDeviceReceiveQueueCapabilities::from(&dpdk_information),
			receive_queue_ring_size_constraints: QueueRingSizeConstraints::from(dpdk_information.rx_desc_lim),
			transmit_device_hardware_offloading_flags: TransmitHardwareOffloadingFlags::from_bits_truncate(dpdk_information.tx_offload_capa),
			ethernet_device_transmit_queue_capabilities: EthernetDeviceTransmitQueueCapabilities::from(&dpdk_information),
			transmit_queue_ring_size_constraints: QueueRingSizeConstraints::from(dpdk_information.tx_desc_lim),
			receive_side_scaling_offload_flow: ReceiveSideScalingOffloadFlow::from_bits_truncate(dpdk_information.flow_type_rss_offloads),
			receive_side_scaling_is_unavailable,
			receive_side_scaling_hash_key_size:
			{
				if receive_side_scaling_is_unavailable
				{
					None
				}
				else
				{
					use self::ReceiveSideScalingHashKeySize::*;
					match dpdk_information.hash_key_size
					{
						// Some drivers, such as Mellanox's ?still? report zero when they mean 40.
						0 => Some(Forty),
						
						40 => Some(Forty),
						52 => Some(FiftyTwo),
						
						hash_key_size @ _ => panic!("Unsupported hash_key_size '{}'", hash_key_size)
					}
				}
			},
			redirection_table_number_of_entries:
			{
				if receive_side_scaling_is_unavailable
				{
					None
				}
				else
				{
					use self::RedirectionTableNumberOfEntries::*;
					match dpdk_information.reta_size
					{
						ETH_RSS_RETA_SIZE_64 => Some(Entries64),
						ETH_RSS_RETA_SIZE_128 => Some(Entries128),
						ETH_RSS_RETA_SIZE_256 => Some(Entries256),
						ETH_RSS_RETA_SIZE_512 => Some(Entries512),
						
						reta_size @ _ => panic!("Unsupported reta_size '{}'", reta_size)
					}
				}
			},
			device_capability_flags: DeviceCapabilityFlags::from_bits_truncate(dpdk_information.dev_capa),
			device_information_flags: DeviceInformationFlags::from_bits_truncate(data.dev_flags),
			extended_statistics_names,
			firmware_version,
			maximum_transmission_unit: MaximumTransmissionUnitSize::try_from(data.mtu).expect("ethernet device very oddly has a maximum transmission unit (MTU) less than the RFC 791 minimum"),
		}
		
	}
	
	/// An ExtendedStatisticsIterator is slightly expensive to construct, so it should be re-used.
	pub fn extended_statistics_iterator<'a>(&'a self) -> ExtendedStatisticsIterator<'a>
	{
		ExtendedStatisticsIterator::new_unchecked(&self.extended_statistics_names)
	}
	
	/// Maximum receive packet length.
	#[inline(always)]
	pub fn maximum_receive_packet_length(&self) -> EthernetFrameLength
	{
		EthernetFrameLength::try_from_with_jumbo_frames(self.maximum_receive_packet_length)
	}
	
	/// Validate that the number of receive queues does not exceed the number of queue pairs available.
	#[inline(always)]
	pub fn validate_not_too_many_receive_queues(&self, number_of_receive_queues: usize)
	{
		assert!(number_of_receive_queues <= self.maximum_queue_pairs as usize);
		panic!("Too many ('{}') receive queues configured (maximum is '{}')", number_of_receive_queues, self.maximum_queue_pairs)
	}
	
	/// Limits the number of receive queues to the device supported maximum queue pairs.
	#[inline(always)]
	pub fn limit_number_of_receive_queues(&self, any_number_of_receive_queues: usize) -> ReceiveNumberOfQueues
	{
		ReceiveNumberOfQueues(min(self.maximum_queue_pairs as usize, any_number_of_receive_queues) as u16)
	}
	
	/// Validate that the receive queue identifier does not exceed the number of queue pairs available.
	#[inline(always)]
	pub fn is_receive_queue_identifier_supported(&self, receive_queue_identifier: ReceiveQueueIdentifier) -> bool
	{
		let into: u16 = receive_queue_identifier.into();
		into < self.maximum_queue_pairs
	}
	
	/// Receive hardware offloading flags for what the ethernet device supports generally.
	#[inline(always)]
	pub fn receive_device_hardware_offloading_flags(&self) -> ReceiveHardwareOffloadingFlags
	{
		self.receive_device_hardware_offloading_flags
	}
	
	/// Receive queue capabilities.
	#[inline(always)]
	pub fn ethernet_device_receive_queue_capabilities(&self) -> &EthernetDeviceReceiveQueueCapabilities
	{
		&self.ethernet_device_receive_queue_capabilities
	}
	
	/// Receive queue ring size constraints.
	#[inline(always)]
	pub fn receive_queue_ring_size_constraints(&self) -> &QueueRingSizeConstraints<ReceiveQueueRingSize>
	{
		&self.receive_queue_ring_size_constraints
	}
	
	/// Validate that the number of transmit queues does not exceed the number of queue pairs available.
	#[inline(always)]
	pub fn validate_not_too_many_transmit_queues(&self, number_of_transmit_queues: usize)
	{
		assert!(number_of_transmit_queues <= self.maximum_queue_pairs as usize);
		panic!("Too many ('{}') transmit queues configured (maximum is '{}')", number_of_transmit_queues, self.maximum_queue_pairs)
	}
	
	/// Limits the number of transmit queues to the device supported maximum queue pairs.
	#[inline(always)]
	pub fn limit_number_of_transmit_queues(&self, any_number_of_transmit_queues: usize) -> TransmitNumberOfQueues
	{
		TransmitNumberOfQueues(min(self.maximum_queue_pairs as usize, any_number_of_transmit_queues) as u16)
	}
	
	/// Validate that the transmit queue identifier does not exceed the number of queue pairs available.
	#[inline(always)]
	pub fn is_transmit_queue_identifier_supported(&self, transmit_queue_identifier: TransmitQueueIdentifier) -> bool
	{
		let into: u16 = transmit_queue_identifier.into();
		into < self.maximum_queue_pairs
	}
	
	/// Transmit hardware offloading flags for what the ethernet device supports generally.
	#[inline(always)]
	pub fn transmit_device_hardware_offloading_flags(&self) -> TransmitHardwareOffloadingFlags
	{
		self.transmit_device_hardware_offloading_flags
	}
	
	/// Transmit queue capabilities.
	#[inline(always)]
	pub fn ethernet_device_transmit_queue_capabilities(&self) -> &EthernetDeviceTransmitQueueCapabilities
	{
		&self.ethernet_device_transmit_queue_capabilities
	}
	
	/// Transmit queue ring size constraints.
	#[inline(always)]
	pub fn transmit_queue_ring_size_constraints(&self) -> &QueueRingSizeConstraints<TransmitQueueRingSize>
	{
		&self.transmit_queue_ring_size_constraints
	}
	
	/// Last receive queue.
	///
	/// Returns `None` if `first_queue` exceeds those possible.
	#[inline(always)]
	pub fn last_receive_queue(&self, first_receive_queue: ReceiveQueueIdentifier, any_number_of_receive_queues: usize) -> Option<ReceiveQueueIdentifier>
	{
		if any_number_of_receive_queues == 0
		{
			return Some(first_receive_queue)
		}
		
		let first_receive_queue: u16 = first_receive_queue.into();
		let limit_number_of_receive_queues: u16 = self.limit_number_of_receive_queues(any_number_of_receive_queues).into();
		if first_receive_queue >= limit_number_of_receive_queues
		{
			return None
		}
		
		let last_receive_queue = min(first_receive_queue.saturating_add(limit_number_of_receive_queues), limit_number_of_receive_queues - 1);
		Some(ReceiveQueueIdentifier(last_receive_queue))
	}
	
	/// Receive side scaling supported offload flows.
	#[inline(always)]
	pub fn receive_side_scaling_offload_flow(&self) -> ReceiveSideScalingOffloadFlow
	{
		self.receive_side_scaling_offload_flow
	}
	
	/// Is receive side scaling unavailable?
	#[inline(always)]
	pub fn receive_side_scaling_is_unavailable(&self) -> bool
	{
		self.receive_side_scaling_is_unavailable
	}
	
	/// Receive side scaling hash key size.
	///
	/// Should be either 40 or 52.
	#[inline(always)]
	pub fn receive_side_scaling_hash_key_size(&self) -> Option<ReceiveSideScalingHashKeySize>
	{
		self.receive_side_scaling_hash_key_size
	}
	
	/// Internally, the device must support one of the supported DPDK sizes:-
	///
	/// * `ETH_RSS_RETA_SIZE_64`
	/// * `ETH_RSS_RETA_SIZE_128`
	/// * `ETH_RSS_RETA_SIZE_256`
	/// * `ETH_RSS_RETA_SIZE_512`
	#[inline(always)]
	pub fn redirection_table_number_of_entries(&self) -> Option<RedirectionTableNumberOfEntries>
	{
		self.redirection_table_number_of_entries
	}
}
