// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub type cryptodev_queue_pair_setup_t = Option<unsafe extern "C" fn(dev: *mut rte_cryptodev, qp_id: u16, qp_conf: *const rte_cryptodev_qp_conf, socket_id: c_int, session_pool: *mut rte_mempool) -> c_int>;
