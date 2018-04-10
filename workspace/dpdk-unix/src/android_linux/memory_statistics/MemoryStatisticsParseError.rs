// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


quick_error!
{
	/// Errors possible when parsing memory statistics.
	#[derive(Debug)]
	pub enum MemoryStatisticsParseError
	{
		/// Could not open a file of memory statistics.
		CouldNotOpenFile(cause: Error)
		{
			cause(cause)
			from()
		}
		
		/// Could not parse a memory statistic.
		CouldNotParseMemoryStatisticValue(zero_based_line_number: usize, memory_statistic_name: MemoryStatisticName)
		{
			display("Zero-based line number  '{}' statistic '{:?}' did not contain a memory statistic value", zero_based_line_number, memory_statistic_name)
		}
		
		/// Could not parse a memory statistic as a u64 value.
		CouldNotParseMemoryStatisticValueAsU64(zero_based_line_number: usize, memory_statistic_name: MemoryStatisticName, bad_balue: String, cause: ParseIntError)
		{
			display("Zero-based line number  '{}' statistic '{:?}' did not contain a valid memory statistic value as u64 '{}' because '{}'", zero_based_line_number, memory_statistic_name, bad_balue, cause)
		}
		
		/// Could not parse a memory statistic because it was a duplicate.
		DuplicateMemoryStatistic(zero_based_line_number: usize, memory_statistic_name: MemoryStatisticName, new_value: u64)
		{
			display("Zero-based line number  '{}' statistic '{:?}' was duplicated (this value is '{}')", zero_based_line_number, memory_statistic_name, new_value)
		}
	}
}
