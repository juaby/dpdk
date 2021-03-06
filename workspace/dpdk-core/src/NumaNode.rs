// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// NUMA node numbers start at zero.
///
/// It is assumed by DPDK code that there is always at least one NUMA node, and, if there is one NUMA node, it is number zero.
///
/// Some DPDK APIs (eg `rte_eth_dev_socket_id`) treat zero as also meaning 'undetermined'.
///
/// NUMA node numbers are not necessarily contiguous but usually are.
///
/// NUMA nodes are also, confusingly, known as sockets. In this sense they represent the socket where a modern CPU with multiple cores resides.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct NumaNode(u8);

impl Display for NumaNode
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}", self.0)
	}
}

impl Into<u8> for NumaNode
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self.0
	}
}

impl Into<u16> for NumaNode
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0 as u16
	}
}

impl Into<u32> for NumaNode
{
	#[inline(always)]
	fn into(self) -> u32
	{
		self.0 as u32
	}
}

impl Into<u64> for NumaNode
{
	#[inline(always)]
	fn into(self) -> u64
	{
		self.0 as u64
	}
}

impl Into<usize> for NumaNode
{
	#[inline(always)]
	fn into(self) -> usize
	{
		self.0 as usize
	}
}

impl Into<i32> for NumaNode
{
	#[inline(always)]
	fn into(self) -> i32
	{
		self.0 as i32
	}
}

impl NumaNode
{
	/// Maximum number of `NumaNode`s.
	pub const Maximum: usize = RTE_MAX_NUMA_NODES;
	
	/// Reads the hyper thread and NUMA node of the currently executing CPU from the `IA32_TSC_AUX` model state register, which Linux populates.
	///
	/// Currently uses the `RDTSCP` instruction, but, once Ice Lake is widely available, could be changed to use the `RDPID` instruction.
	#[cfg(target_os = "linux")]
	#[inline(always)]
	pub fn numa_node_and_hyper_thread() -> (NumaNode, HyperThread)
	{
		// The value of the timestamp register is stored into the `RDX` and `RAX` registers.
		// The value of the hyper thread and NUMA node is stored into the `RCX` register.
		// The top 32-bits of `RDX`, `RAX` and `RCX` are cleared (zero).
		#[inline(always)]
		unsafe fn rdtscp() -> u64
		{
			let _rax: u64;
			let _rdx: u64;
			let rcx: u64;
			
			asm!
			(
				"rdtscp"
				:
					"={rax}"(_rax), "={rdx}"(_rdx), "={rcx}"(rcx)
				:
				:
				:
					"volatile"
			);
			
			rcx
		}
		let rcx = unsafe { rdtscp() };
		
		let numa_node = (rcx & 0x00000000_0FFFF000) >> 12;
		let hyper_thread = rcx & 0x00000000_00000FFF;
		
		(NumaNode(numa_node as u8), HyperThread::from(hyper_thread as u16))
	}
	
	/// Constructs from an `u16` value.
	///
	/// Panics if the value is out-of-range greater than or equal to `RTE_MAX_NUMA_NODES`).
	#[inline(always)]
	pub fn from_u16(value: u16) -> Self
	{
		debug_assert!((Self::Maximum as u16) <= (::std::u8::MAX as u16), "Self::Maximum '{}' exceeds ::std::u8::MAX '{}'; the DPDK API is broken", Self::Maximum, ::std::u8::MAX);
		
		assert!(value < (Self::Maximum as u16), "value '{}' equals or exceeds Self::Maximum '{}'", value, Self::Maximum);
		
		NumaNode(value as u8)
	}
	
	/// Constructs from an `u32` value.
	///
	/// Panics if the value is out-of-range greater than or equal to `RTE_MAX_NUMA_NODES`).
	#[inline(always)]
	pub fn from_u32(value: u32) -> Self
	{
		debug_assert!((Self::Maximum as u32) <= (::std::u8::MAX as u32), "Self::Maximum '{}' exceeds ::std::u8::MAX '{}'; the DPDK API is broken", Self::Maximum, ::std::u8::MAX);
		
		assert!(value < (Self::Maximum as u32), "value '{}' equals or exceeds Self::Maximum '{}'", value, Self::Maximum);
		
		NumaNode(value as u8)
	}
	
	/// Valid NUMA nodes.
	///
	/// Cheap.
	#[inline(always)]
	pub fn valid_numa_nodes() -> &'static BTreeSet<Self>
	{
		#[inline(always)]
		fn lazy_initialize() -> BTreeSet<NumaNode>
		{
			NumaNode::initialize_libnuma();
			
			let numa_nodes_bitmask = unsafe { numa_allocate_nodemask() };
			
			let number_of_numa_nodes_in_numa_nodes_bitmask = unsafe { numa_num_possible_nodes() } as usize;
			let maximum = min(number_of_numa_nodes_in_numa_nodes_bitmask, NumaNode::Maximum);
			
			let mut valid_numa_nodes = BTreeSet::new();
			for numa_node_index in 0 .. maximum
			{
				let is_unset = unsafe { numa_bitmask_isbitset(numa_nodes_bitmask as *const _, numa_node_index as u32) } == 0;
				if is_unset
				{
					continue
				}
				
				valid_numa_nodes.insert(NumaNode(numa_node_index as u8));
			}
			valid_numa_nodes
		}
		
		lazy_static!
		{
			static ref ValidNumaNodes: BTreeSet<NumaNode> = lazy_initialize();
		}
		
		&ValidNumaNodes
	}
	
	/// Valid NUMA nodes (as a map).
	///
	/// Cheap.
	#[inline(always)]
	pub fn valid_numa_nodes_map() -> &'static BTreeMap<Self, ()>
	{
		#[inline(always)]
		fn lazy_initialize() -> BTreeMap<NumaNode, ()>
		{
			let mut valid_numa_nodes_map = BTreeMap::new();
			
			for numa_node in NumaNode::valid_numa_nodes().iter()
			{
				valid_numa_nodes_map.insert(*numa_node, ());
			}
			
			valid_numa_nodes_map
		}
		
		lazy_static!
		{
			static ref ValidNumaNodesMap: BTreeMap<NumaNode, ()> = lazy_initialize();
		}
		
		&ValidNumaNodesMap
	}
	
	/// Neighbours to this NUMA node ordered in increasing distance order.
	///
	/// The first entry is `self`.
	pub fn neighbouring_numa_nodes_in_increasing_distance_order_with_first_as_self(self) -> IndexSet<Self>
	{
		let mut distances = BTreeSet::new();
		for neighbouring_numa_node in Self::valid_numa_nodes().iter()
		{
			let raw_distance = unsafe { numa_distance(self.0 as i32, neighbouring_numa_node.0 as i32) };
			if raw_distance > 0
			{
				let smaller_is_closer_and_zero_is_self = ((raw_distance as usize) / 10) - 1;
				distances.insert((smaller_is_closer_and_zero_is_self, *neighbouring_numa_node));
			}
		}
		
		let mut neighbouring_numa_nodes_in_increasing_distance_order_with_first_as_self = IndexSet::with_capacity(distances.len());
		for (_, neighbouring_numa_node) in distances.iter()
		{
			neighbouring_numa_nodes_in_increasing_distance_order_with_first_as_self.insert(*neighbouring_numa_node);
		}
		neighbouring_numa_nodes_in_increasing_distance_order_with_first_as_self
	}
	
	/// DPDK memory allocatio statistics on this NUMA node.
	#[inline(always)]
	pub fn dpdk_memory_allocation_statistics(self) -> rte_malloc_socket_stats
	{
		let mut statistics = unsafe { uninitialized() };
		assert_eq!(unsafe { rte_malloc_get_socket_stats(self.into(), &mut statistics) }, 0, "rte_malloc_get_socket_stats() failed");
		statistics
	}
	
	/// NUMA nodes that could possibly be online at some point.
	///
	/// Not reliable, as includes NUMA nodes that can never be brought online; simply reports the number that could be used by the Linux kernel upto the `CONFIG_` number of CPUs
	///
	/// Consider using libnuma-backed `valid_numa_nodes()` instead of this call.
	#[inline(always)]
	pub fn possible(sys_path: &SysPath) -> Option<BTreeSet<Self>>
	{
		Self::parse_list_mask(sys_path, "possible")
	}
	
	/// NUMA nodes that are online at some point.
	///
	/// Consider using libnuma-backed `valid_numa_nodes()` instead of this call.
	#[inline(always)]
	pub fn online(sys_path: &SysPath) -> Option<BTreeSet<Self>>
	{
		Self::parse_list_mask(sys_path, "online")
	}
	
	/// NUMA nodes that have normal memory (as opposed to what was high memory; I suspect this is a bit useless).
	///
	/// Consider using libnuma-backed `valid_numa_nodes()` instead of this call.
	#[inline(always)]
	pub fn have_normal_memory(sys_path: &SysPath) -> Option<BTreeSet<Self>>
	{
		Self::parse_list_mask(sys_path, "has_normal_memory")
	}
	
	/// NUMA nodes that have a CPU.
	///
	/// NUMA nodes without a CPU are effectively fake NUMA nodes.
	///
	/// Consider using libnuma-backed `valid_numa_nodes()` instead of this call.
	#[inline(always)]
	pub fn have_at_least_one_cpu(sys_path: &SysPath) -> Option<BTreeSet<Self>>
	{
		Self::parse_list_mask(sys_path, "has_cpu")
	}
	
	/// Try to unreserve (clear reservations of) huge pages.
	///
	/// Will only work as root.
	#[inline(always)]
	pub fn unreserve_huge_pages(self, sys_path: &SysPath, huge_page_size: HugePageSize) -> Result<(), io::Error>
	{
		huge_page_size.unreserve_numa_huge_pages(sys_path, self.into())
	}
	
	/// Try to reserve huge pages.
	///
	/// Will only work as root.
	#[inline(always)]
	pub fn reserve_huge_pages(self, sys_path: &SysPath, huge_page_size: HugePageSize, number_to_try_to_reserve: u64) -> io::Result<()>
	{
		huge_page_size.reserve_numa_huge_pages(sys_path, self.into(), number_to_try_to_reserve)
	}
	
	/// Read number of huge pages of `huge_page_size` size.
	#[inline(always)]
	pub fn number_of_huge_pages(self, sys_path: &SysPath, huge_page_size: HugePageSize) -> io::Result<u64>
	{
		huge_page_size.number_of_numa_huge_pages(sys_path, self.into())
	}
	
	/// Read number of free huge pages of `huge_page_size` size.
	#[inline(always)]
	pub fn number_of_free_global_huge_pages(self, sys_path: &SysPath, huge_page_size: HugePageSize) -> io::Result<u64>
	{
		huge_page_size.number_of_free_numa_huge_pages(sys_path, self.into())
	}
	
	/// Read number of surplus huge pages of `huge_page_size` size.
	#[inline(always)]
	pub fn number_of_surplus_huge_pages(self, sys_path: &SysPath, huge_page_size: HugePageSize) -> io::Result<u64>
	{
		huge_page_size.number_of_surplus_numa_huge_pages(sys_path, self.into())
	}
	
	/// Tries to compact pages for this NUMA node.
	///
	/// Will only work as root.
	#[inline(always)]
	pub fn compact_pages(self, sys_path: &SysPath)
	{
		assert_effective_user_id_is_root(&format!("Compact pages in NUMA node '{}'", self.0));
		
		if sys_path.is_a_numa_machine()
		{
			sys_path.numa_node_path(self.into(), "compact").write_value(1).unwrap();
		}
	}
	
	/// Tries to evict pages for this NUMA node.
	///
	/// Will only work as root.
	#[inline(always)]
	pub fn evict_pages(self, sys_path: &SysPath)
	{
		assert_effective_user_id_is_root(&format!("Evict pages from NUMA node '{}'", self.0));
		
		if sys_path.is_a_numa_machine()
		{
			sys_path.numa_node_path(self.into(), "scan_unevictable_pages").write_value(1).unwrap();
		}
	}
	
	/// This is a subset of `self.zoned_virtual_memory_statistics()`.
	///
	/// Interpret this by multiplying counts by page size.
	#[deprecated]
	#[inline(always)]
	pub fn numa_memory_statistics(self, sys_path: &SysPath) -> io::Result<HashMap<VirtualMemoryStatisticName, u64>>
	{
		sys_path.numa_node_path(self.into(), "numastat").parse_virtual_memory_statistics_file()
	}
	
	/// Memory statistics.
	///
	/// Interpret this by multiplying counts by page size.
	#[inline(always)]
	pub fn zoned_virtual_memory_statistics(self, sys_path: &SysPath) -> io::Result<HashMap<VirtualMemoryStatisticName, u64>>
	{
		sys_path.numa_node_path(self.into(), "vmstat").parse_virtual_memory_statistics_file()
	}
	
	/// Memory information.
	#[inline(always)]
	pub fn memory_information(self, sys_path: &SysPath, memory_information_name_prefix: &[u8]) -> Result<MemoryInformation, MemoryInformationParseError>
	{
		sys_path.numa_node_path(self.into(), "meminfo").parse_memory_information_file(memory_information_name_prefix)
	}
	
	const CacheLineSize: u32 = 64;
	
	/// Allocates memory on the heap on to a particular NUMA node.
	#[inline(always)]
	pub fn allocate_uninitialized<T>(self, alignment_power_of_two: u32) -> Option<DpdkAllocatedMemory<T>>
	{
		debug_assert!(alignment_power_of_two.is_power_of_two(), "alignment_power_of_two '{}' is not a power of two", alignment_power_of_two);
		debug_assert!(alignment_power_of_two >= Self::CacheLineSize, "alignment_power_of_two '{}' is less than CacheLineSize '{}'", alignment_power_of_two, Self::CacheLineSize);
		
		let result = unsafe { rte_malloc_socket(null(), size_of::<T>(), alignment_power_of_two, self.into()) };
		if unlikely!(result.is_null())
		{
			None
		}
		else
		{
			Some(DpdkAllocatedMemory(result as *mut T))
		}
	}
	
	/// Allocates memory on the heap on to a particular NUMA node.
	#[inline(always)]
	pub fn allocate_zeroed<T>(self, alignment_power_of_two: u32) -> Option<DpdkAllocatedMemory<T>>
	{
		debug_assert!(alignment_power_of_two.is_power_of_two(), "alignment_power_of_two '{}' is not a power of two", alignment_power_of_two);
		debug_assert!(alignment_power_of_two >= Self::CacheLineSize, "alignment_power_of_two '{}' is less than CacheLineSize '{}'", alignment_power_of_two, Self::CacheLineSize);
		
		let result = unsafe { rte_zmalloc_socket(null(), size_of::<T>(), alignment_power_of_two, self.into()) };
		if unlikely!(result.is_null())
		{
			None
		}
		else
		{
			Some(DpdkAllocatedMemory(result as *mut T))
		}
	}
	
	/// Allocates memory on the heap on to a particular NUMA node.
	#[inline(always)]
	pub fn allocate_uninitialized_for_array<T>(self, number_of_elements: usize, alignment_power_of_two: u32) -> Option<DpdkAllocatedMemory<T>>
	{
		debug_assert!(alignment_power_of_two.is_power_of_two(), "alignment_power_of_two '{}' is not a power of two", alignment_power_of_two);
		debug_assert!(alignment_power_of_two >= Self::CacheLineSize, "alignment_power_of_two '{}' is less than CacheLineSize '{}'", alignment_power_of_two, Self::CacheLineSize);
		
		let result = unsafe { rte_calloc_socket(null(), number_of_elements, size_of::<T>(), alignment_power_of_two, self.into()) };
		if unlikely!(result.is_null())
		{
			None
		}
		else
		{
			Some(DpdkAllocatedMemory(result as *mut T))
		}
	}
	
	/// Hyper threads are similar to `LogicalCore`s, but, since this code often runs before DPDK has been initialized (`rte_eal_init`), we can not use them as their global and thread local statics will not have been initialized.
	///
	/// TODO: ?assume master logical core is current process? (DPDK defaults to first logical core).
	pub fn hyper_threads(self) -> HashSet<HyperThread>
	{
		Self::initialize_libnuma();
		
		let hyper_thread_bitmask = unsafe { numa_allocate_cpumask() };
		
		let numa_node_index = self.0;
		
		assert_eq!(unsafe { numa_node_to_cpus(numa_node_index as i32, hyper_thread_bitmask) }, 0, "numa_node_to_cpus failed");
		
		let maximum_hyper_threads = min((unsafe { numa_num_possible_cpus() }) as usize, LogicalCore::Maximum);
		let mut set = HashSet::with_capacity(maximum_hyper_threads);
		for hyper_thread_index in 0 .. maximum_hyper_threads
		{
			let is_unset = unsafe { numa_bitmask_isbitset(hyper_thread_bitmask as *const _, hyper_thread_index as u32) } == 0;
			if is_unset
			{
				continue
			}
			
			set.insert(HyperThread::from(hyper_thread_index as u16));
		}
		
		unsafe { numa_bitmask_free(hyper_thread_bitmask) };
		
		set
	}
	
	#[inline(always)]
	fn parse_list_mask(sys_path: &SysPath, file_name: &str) -> Option<BTreeSet<Self>>
	{
		if sys_path.is_a_numa_machine()
		{
			Some(sys_path.numa_nodes_path(file_name).read_linux_core_or_numa_list(Self::from_u16).unwrap())
		}
		else
		{
			None
		}
	}
	
	#[inline(always)]
	fn initialize_libnuma()
	{
		static InitializeLibnuma: Once = ONCE_INIT;
		
		InitializeLibnuma.call_once(|| assert_eq!(unsafe { numa_available() }, 0, "numa_available failed"))
	}
}
