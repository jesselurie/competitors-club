//! Benchmarking setup for pallet-template

use super::*;

use frame_system::RawOrigin;
use frame_benchmarking::{benchmarks, whitelisted_caller, impl_benchmark_test_suite};
#[allow(unused)]
use crate::Module as Template;

benchmarks! {
	vie {
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller), s)
	join {
		assert_eq!(Something::<T>::get(), Some(s));
	}
}

impl_benchmark_test_suite!(
	Template,
	crate::mock::new_test_ext(),
	crate::mock::Test,
);
