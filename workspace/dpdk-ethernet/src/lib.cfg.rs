// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[macro_use] extern crate bitflags;
extern crate dpdk_bus;
extern crate dpdk_core;
extern crate dpdk_sys;
extern crate either;
extern crate libc;
#[macro_use] extern crate likely;
extern crate network_collections;
extern crate network_ethernet;
extern crate serde;
#[macro_use] extern crate serde_derive;


use self::link_status::*;
use self::number_of_queues::*;
use self::queue_identifiers::*;
use self::queue_ring_sizes::*;
use self::receive_side_scaling::*;
use ::dpdk_bus::pci::*;
use ::dpdk_core::*;
use ::dpdk_sys::*;
pub use ::either::*;
use ::libc::*;
use ::network_collections::Array40;
use ::network_collections::Array52;
use ::network_collections::NonNullUnifiedArrayVecAndVec;
use ::network_ethernet::MediaAccessControlAddress;
use ::std::borrow::Cow;
use ::std::cmp::min;
use ::std::convert::TryFrom;
use ::std::fmt;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::iter::Step;
use ::std::mem::replace;
use ::std::mem::uninitialized;
use ::std::ops::Add;
use ::std::ops::AddAssign;
use ::std::ops::Sub;
use ::std::ops::SubAssign;
use ::std::ptr::NonNull;
use ::std::sync::Arc;


/// Packet receive or send bursts.
pub mod bursts;


/// Link status.
pub mod link_status;


/// Number of queues.
pub mod number_of_queues;


/// Queue identifiers.
pub mod queue_identifiers;


/// Queue ring sizes.
pub mod queue_ring_sizes;


/// Receive side scaling.
pub mod receive_side_scaling;


include!("EthernetDeviceCapabilities.rs");
include!("EthernetPortIdentifier.rs");
include!("ReceiveHardwareOffloadingFlags.rs");
include!("TransmitHardwareOffloadingFlags.rs");
