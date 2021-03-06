// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


bitflags!
{
	/// Unmount flags.
	#[allow(missing_docs)]
	#[derive(Serialize, Deserialize)]
	pub struct UnmountFlags: i32
	{
		/// Force.
		const Force = ::libc::MNT_FORCE;
		
		/// Detach.
		const Detach = ::libc::MNT_DETACH;
		
		/// Expire.
		const Expire = ::libc::MNT_EXPIRE;
		
		// Not in libc crate
		// const NoFollow = ::libc::UMOUNT_NOFOLLOW,
	}
}
