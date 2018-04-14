// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::const_cstr_fork::ConstCStr;
use ::dpdk_sys::*;
use ::libc::*;
use ::libc_extra::stderr;
use ::rust_extra::unlikely;
use ::syscall_alt::constants::E;
use ::syscall_alt::constants::NegativeE;
use ::E_RTE;
use ::logicalCores::NumaSocketId;
use ::logicalCores::AnyNumaSocketId;
use ::rust_extra::powersOfTwo::PowerOfTwoThirtyTwoBit;


include!("MemoryZone.rs");
include!("MemoryZoneReservationPageChoice.rs");
