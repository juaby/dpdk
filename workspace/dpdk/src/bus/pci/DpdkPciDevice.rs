// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DpdkPciDevice(NonNull<rte_pci_device>);

impl DpdkPciDevice
{
	/// `/sys/fs` path used by DPDK.
	#[inline(always)]
	pub fn sys_fs_path() -> PathBuf
	{
		let from_c = unsafe { pci_get_sysfs_path() };
		let slice = unsafe { from_raw_parts(from_c as *mut u8, strnlen(from_c, PATH_MAX as usize)) };
		let os_str = OsStr::from_bytes(slice);
		let mut path = PathBuf::new();
		path.push(os_str);
		path
	}

	/// Scan.
	#[inline(always)]
	pub fn scan() -> Vec<Self>
	{
		match unsafe { rte_eal_pci_scan() }
		{
			0 => (),
			negative if negative < 0 => panic!("Could not scan PCI bus, error code was '{}'", negative),

			illegal @ _ => panic!("Invalid result code '{}' from rte_eal_pci_scan()", illegal),
		};

		let pci_device_list = unsafe { pci_device_list };

		let first_element = pci_device_list.tqh_first;
		let is_empty = first_element.is_null();
		let capacity = if is_empty
		{
			0
		}
		else
		{
			256
		};

		let mut devices = Vec::with_capacity(capacity);

		let mut element = first_element;
		while element.is_not_null()
		{
			devices.push(DpdkPciDevice(unsafe { NonNull::new_unchecked(element) }));
			let element_value = unsafe { (*element) };
			element = element_value.next.tqe_next;
		}
		devices.shrink_to_fit();

		devices
	}
	
	/// Returns the DpdkPciDevice for an ethernet port identifier, if the port identifier is for a valid ethernet port which is based on a PCI device.
	#[inline(always)]
	pub fn for_ethernet_port(port_identifier: u5) -> Option<Self>
	{
		match EthernetPort::new(port_identifier)
		{
			None => None,
			Some(ethernet_port) =>
			{
				let underlying_rte_eth_dev = ethernet_port.underlying_ethernet_device();
				if underlying_rte_eth_dev.device.is_null()
				{
					None
				}
				
				Ok(DpdkPciDevice(unsafe { NonNull::new_unchecked(rust_RTE_DEV_TO_PCI(underlying_rte_eth_dev.device)) }))
			}
		}
	}
	
	/// Next known DpdkPciDevice.
	#[inline(always)]
	pub fn next(&self) -> Option<Self>
	{
		let next = self.deref().tqe_next;
		if next.is_null()
		{
			None
		}
		else
		{
			Some(DpdkPciDevice(unsafe { NonNull::new_unchecked(next) }))
		}
	}
	
	/// Underlying generic DPDK device, a sort of super class.
	///
	/// Use this to get to the NUMA node associated with this PCI device.
	#[inline(always)]
	pub fn device<'a>(&'a self) -> DpdkDevice<'a>
	{
		DpdkDevice(unsafe { NonNull::new_unchecked(&self.deref().device as *mut _) }, PhantomData)
	}
	
	/// DPDK driver.
	#[inline(always)]
	pub fn driver(&self) -> Option<DpdkPciDriver>
	{
		let driver = self.deref().driver;
		if unlikely(driver.is_null())
		{
			None
		}
		else
		{
			Some(DpdkPciDriver(driver))
		}
	}
	
	/// See also PciKernelDriver.
	#[inline(always)]
	pub fn kernel_driver(&self) -> rte_kernel_driver
	{
		self.deref().kdrv
	}
	
	/// Memory resources.
	#[inline(always)]
	pub fn memory_resources<'a>(&'a self) -> DpdkDeviceMemoryResources<'a>
	{
		DpdkDeviceMemoryResources(&self.deref().mem_resource, PhantomData, 0)
	}
	
	/// Interrupt handle.
	#[inline(always)]
	pub fn interrupt_handle<'a>(&'a self) -> &'a rte_intr_handle
	{
		&self.deref().intr_handle
	}
	
	/// Name (does not exceed 18 bytes).
	///
	/// Formatted PCI device address.
	#[inline(always)]
	pub fn name<'a>(&'a self) -> &'a CStr
	{
		unsafe { CStr::from_ptr(self.deref().name) }
	}
	
	/// Maximum virtul; functions supported.
	///
	/// A value of 0 (zero) implies SR-IOV is disabled.
	#[inline(always)]
	pub fn maximum_virtual_functions(&self) -> u16
	{
		self.deref().max_vfs
	}
	
	/// PCI device address.
	#[inline(always)]
	pub fn pci_device_address(&self) -> DpdkPciDeviceAddress
	{
		let address = self.deref().addr;
		DpdkPciDeviceAddress::from_rte_pci_addr(address)
	}
	
	/// PCI raw class and subclass identifiers.
	#[inline(always)]
	pub fn pci_vendor_raw_class_and_subclass_identifiers(&self) -> (u16, u16)
	{
		let class_id = self.deref().id.class_id;
		(((class_id >> 16) as u16), (class_id & 0x0000_FFFFF) as u16)
	}
	
	/// PCI vendor identifier.
	#[inline(always)]
	pub fn pci_vendor_identifier(&self) -> PciVendorIdentifier
	{
		PciVendorIdentifier(self.deref().id.vendor_id)
	}
	
	/// PCI device identifier.
	#[inline(always)]
	pub fn pci_device_identifier(&self) -> PciDeviceIdentifier
	{
		PciDeviceIdentifier(self.deref().id.device_id)
	}
	
	/// PCI subsystem vendor identifier.
	#[inline(always)]
	pub fn pci_subsystem_vendor_identifier(&self) -> Option<PciVendorIdentifier>
	{
		let value = self.deref().id.subsystem_vendor_id;
		if value = 0xFFFF
		{
			None
		}
		else
		{
			Some(PciVendorIdentifier(value))
		}
	}
	
	/// PCI subsystem device identifier.
	#[inline(always)]
	pub fn pci_subsystem_device_identifier(&self) -> Option<PciDeviceIdentifier>
	{
		let value = self.deref().id.subsystem_device_id;
		if value = 0xFFFF
		{
			None
		}
		else
		{
			Some(PciDeviceIdentifier(value))
		}
	}
	
	/// PCI device address.
	#[inline(always)]
	pub fn matches_vendor_and_device(&self, pci_device_type: &PciDeviceType) -> bool
	{
		pci_device_type.vendor == self.pci_device_identifier() && pci_device_type.device == self.deref().id.device_id
	}
	
	/// Map IO port.
	#[inline(always)]
	pub fn map_input_output_port(&mut self, base_address_register: i32) -> Option<DpdkPciInputOutputPort>
	{
		let mut data = unsafe { uninitialized() };
		let result = unsafe { rte_eal_pci_ioport_map(self.handle(), base_address_register, &mut data) };
		if likely(result == 0)
		{
			Some(DpdkPciInputOutputPort::new(UnsafeCell::new(data)))
		}
		else
		{
			match result
			{
				negative if negative < 0 => None,

				_ => panic!("rte_eal_pci_ioport_map() returned illegal result '{}'", result),
			}
		}
	}
	
	/// Map.
	#[inline(always)]
	pub fn map(&mut self) -> Option<bool>
	{
		match unsafe { rte_eal_pci_map_device(self.handle()) }
		{
			0 => Some(true),
			negative if negative < 0 => Some(false),
			_ => None,
		}
	}

	/// Returns None if no driver found.
	#[inline(always)]
	pub fn unmap(&mut self)
	{
		unsafe { rte_eal_pci_unmap_device(self.handle()) }
	}
	
	/// Read configuration space.
	#[inline(always)]
	pub fn read_configuration_space(&self, read_into: &mut [u8], offset_into_configuration_space: isize) -> Result<(), i32>
	{
		let result = unsafe { rte_eal_pci_read_config(self.handle(), read_into.as_mut_ptr() as *mut c_void, read_into.len(), offset_into_configuration_space as off_t) };
		if likely(result == 0)
		{
			Ok(())
		}
		else
		{
			Err(result)
		}
	}
	
	/// Write configuration space.
	#[inline(always)]
	pub fn write_configuration_space(&self, write_from: &[u8], offset_into_configuration_space: isize) -> Result<(), i32>
	{
		let result = unsafe { rte_eal_pci_write_config(self.handle(), write_from.as_ptr() as *mut c_void, write_from.len(), offset_into_configuration_space as off_t) };
		if likely(result == 0)
		{
			Ok(())
		}
		else
		{
			Err(result)
		}
	}
	
	#[inline(always)]
	fn deref(&self) -> &rte_pci_device
	{
		unsafe { & * self.handle() }
	}
	
	#[inline(always)]
	fn handle(&self) -> *mut rte_pci_device
	{
		self.0.as_ptr()
	}
}