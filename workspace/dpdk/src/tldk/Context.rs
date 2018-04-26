// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait Context<IpV4: AddressLookUpForSendCallback<in_addr>, IpV6: AddressLookUpForSendCallback<in6_addr>> : Sized + Drop
{
	const Protocol: Layer4Protocol;

	type Device: Device;

	#[doc(hidden)]
	#[inline(always)]
	fn handle(&mut self) -> *mut tle_ctx;

	#[doc(hidden)]
	#[inline(always)]
	fn _newContext(opaqueFfiHandle: *mut tle_ctx, addressLookUpForSendToIpV4: Rc<RefCell<IpV4>>, addressLookUpForSendToIpV6: Rc<RefCell<IpV6>>) -> Self;

	#[doc(hidden)]
	#[inline(always)]
	fn _drop(&mut self)
	{
		unsafe { tle_ctx_destroy(self.handle()) };
	}

	#[inline(always)]
	fn create
	(
		numa_node_choice: NumaNodeChoice,
		maximumNumberOfStreams: u32,
		maximumNumberOfReceiveBuffers: u32,
		maximumNumberOfSendBuffers: u32,
		sendMaximumBulkNumber: u32,
		addressLookUpForSendToIpV4: Rc<RefCell<IpV4>>,
		addressLookUpForSendToIpV6: Rc<RefCell<IpV6>>,
		mut deviceConfigurations: Vec<DeviceConfiguration>,
	) -> Option<(Self, Vec<(DeviceConfiguration, Self::Device)>)>
	{
		let parameters = tle_ctx_param
		{
			socket_id: numa_node_choice.into(),
			proto: Self::Protocol.tldk_value(),
			max_streams: maximumNumberOfStreams,
			max_stream_rbufs: maximumNumberOfReceiveBuffers,
			max_stream_sbufs: maximumNumberOfSendBuffers,
			send_bulk_size: sendMaximumBulkNumber,

			flags: TLE_CTX_FLAG_ST,

			lookup4: IpV4::asFunctionPointer(),
			lookup4_data: addressLookUpForSendToIpV4.as_ptr() as *mut c_void,
			lookup6: IpV6::asFunctionPointer(),
			lookup6_data: addressLookUpForSendToIpV4.as_ptr() as *mut c_void,

//			// hash algorithm to be used to generate sequence number
//			hash_alg: ,
//
//			// secret key to be used to calculate the hash.
//			secret_key: ,
//
//			// initial congestion window, default is 2*MSS if 0.
//			icw: ,
//
//			// TCP TIME_WAIT state timeout duration in milliseconds, default 2MSL, if UINT32_MAX; TLE_TCP_TIMEWAIT_DEFAULT (u32::MAX)
//			timewait: ,
		};


		let result = unsafe { tle_ctx_create(&parameters) };
		if unlikely(result.is_null())
		{
			match LogicalCore::current_logical_core_error_number()
			{
				E::ENOMEM => None,

				E::ENODEV => None,

				E::EINVAL => panic!("Supplied an invalid value"),

				illegal @ _ => panic!("Unexpected errno '{}' from tle_ctx_create()", illegal),
			}
		}
		else
		{
			let mut context = Self::_newContext(result, addressLookUpForSendToIpV4, addressLookUpForSendToIpV6);

			let mut devices = Vec::with_capacity(deviceConfigurations.len());
			for deviceConfiguration in deviceConfigurations.drain(..)
			{
				let device = context._createNewDevice(&deviceConfiguration).expect("Out of memory creating Device");
				devices.push((deviceConfiguration, device))
			}

			Some((context, devices))
		}
	}

	#[inline(always)]
	fn invalidateBecauseDestinationInformationHasChanged(&mut self)
	{
		unsafe { tle_ctx_invalidate(self.handle()) };
	}

	#[inline(always)]
	fn progress(&mut self, number: u32);

	#[doc(hidden)]
	#[inline(always)]
	fn _createNewDevice(&mut self, deviceConfiguration: &DeviceConfiguration) -> Option<Self::Device>
	{
		let (receiveOffload, transmitOffload) = deviceConfiguration.offloading(Self::Protocol);

		// NOTE: the value pointed to by port in tle_bl_port is NOT CLONED by tle_add_dev(). However, the reference (port) is never used again outside of tle_add_dev

		// NOTE: These 4 statement force borrows of ipV4BlockedPorts and ipV6BlockedPorts which live longer than the code before and after tle_add_dev()
		let ipV4BlockedPorts = deviceConfiguration.ipV4BlockedPortsForTldk();
		let ipV6BlockedPorts = deviceConfiguration.ipV6BlockedPortsForTldk();

		{
			let ipV4BlockedPorts = &ipV4BlockedPorts;
			let ipV6BlockedPorts = &ipV6BlockedPorts;

			let dev_prm = tle_dev_param
			{
				rx_offload: receiveOffload.bits(),
				tx_offload: transmitOffload.bits(),
				local_addr4: deviceConfiguration.in_addr(),
				local_addr6: deviceConfiguration.in6_addr(),
				bl4: tle_bl_port
				{
					nb_port: ipV4BlockedPorts.len() as u32,
					port: if ipV4BlockedPorts.is_empty()
					{
						null()
					}
					else
					{
						ipV4BlockedPorts.as_ptr()
					},
				},
				bl6: tle_bl_port
				{
					nb_port: ipV6BlockedPorts.len() as u32,
					port: if ipV6BlockedPorts.is_empty()
					{
						null()
					}
					else
					{
						ipV6BlockedPorts.as_ptr()
					}
				},
			};

			let result = unsafe { tle_add_dev(self.handle(), &dev_prm) };
			if unlikely(result.is_null())
			{
				match LogicalCore::current_logical_core_error_number()
				{
					E::ENOMEM => None,

					E::EINVAL => panic!("Supplied an invalid value"),

					illegal @ _ => panic!("Unexpected errno '{}' from tle_add_dev()", illegal),
				}
			}
			else
			{
				Some(Self::Device::_new(result))
			}
		}
	}
}
