// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for dex_swap_router
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-23, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `manjaro`, CPU: `Intel(R) Core(TM) i7-10875H CPU @ 2.30GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ../target/release/dex-template-node
// benchmark
// pallet
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// dex_swap_router
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for dex_swap_router.
pub trait WeightInfo {
	fn swap_exact_token_for_tokens_through_stable_pool() -> Weight;
}

/// Weights for dex_swap_router using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: DexGeneral ForeignLedger (r:4 w:4)
	// Storage: DexGeneral PairStatuses (r:1 w:0)
	// Storage: DexStable Pools (r:1 w:1)
	// Storage: Tokens Accounts (r:4 w:4)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:1 w:0)
	fn swap_exact_token_for_tokens_through_stable_pool() -> Weight {
		// Minimum execution time: 181_671 nanoseconds.
		Weight::from_ref_time(183_419_000 as u64)
			.saturating_add(T::DbWeight::get().reads(13 as u64))
			.saturating_add(T::DbWeight::get().writes(9 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: DexGeneral ForeignLedger (r:4 w:4)
	// Storage: DexGeneral PairStatuses (r:1 w:0)
	// Storage: DexStable Pools (r:1 w:1)
	// Storage: Tokens Accounts (r:4 w:4)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:1 w:0)
	fn swap_exact_token_for_tokens_through_stable_pool() -> Weight {
		// Minimum execution time: 181_671 nanoseconds.
		Weight::from_ref_time(183_419_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(13 as u64))
			.saturating_add(RocksDbWeight::get().writes(9 as u64))
	}
}
