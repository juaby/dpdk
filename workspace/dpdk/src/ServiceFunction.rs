// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A function to run repeatedly on a service core.
pub trait ServiceFunction
{
	/// Return true if multi-thread safe.
	///
	/// Defaults to false.
	#[inline(always)]
	fn is_multi_thread_safe(&self) -> bool
	{
		false
	}
	
	/// Called repeatedly by a service core.
	#[inline(always)]
	fn execute(&mut self);
	
	unsafe extern "C" fn callback(args: *mut c_void) -> i32
	{
		debug_assert!(args.is_not_null(), "args is null");
		
		let mut this = unsafe { &mut * (args as *mut Self) };
		
		this.execute();
		
		0
	}
}
