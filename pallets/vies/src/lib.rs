#![cfg_attr(not(feature = "std"), no_std)]
use sp_std::{vec::Vec};
use sp_std::convert::{Into};
use frame_support::{
	codec::{Encode, Decode,FullCodec,WrapperTypeEncode},
	traits::{Randomness,ReservableCurrency,Currency,BalanceStatus,ExistenceRequirement,Time},
	RuntimeDebug
};
use sp_io::hashing::blake2_128;

use sp_runtime::{
	traits::{SaturatedConversion}
};

use orml_nft::Module as NftModule;
use orml_utilities::with_transaction_result;
// use frame_support::pallet_prelude::*;
pub mod types;
use types::*;

// #[cfg(feature = "std")]
// use serde::{Serialize, Deserialize};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;


pub type MomentOf<T> = <<T as Config>::Time as Time>::Moment;
pub type VieRequestOf<T> = VieReq<<T as frame_system::Config>::AccountId,BalanceReservableOf<T>>;
pub type BalanceOf<T> = < <T as Config>::Currency as Currency< <T as frame_system::Config>::AccountId> > ::Balance;
pub type VieOf<T> = Vie<<T as frame_system::Config>::AccountId,BalanceReservableOf<T>, MomentOf<T>>;
pub type BalanceReservableOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type PodiumReqOf<T> = PodiumReq<AccountIdOf<T>>;

pub use pallet::*;

// pub use self::currency::DOLLARS;

/// Money matters.
// pub mod currency {
	// use BalanceOf;
/// The existential deposit.
// pub const EXISTENTIAL_DEPOSIT:Balance= 100 * CENTS;
// pub const UNITS: Balance = 1_000_000_000_000;
// pub const DOLLARS: Balance = UNITS; // 10_000_000_000
// pub const CENTS: Balance = DOLLARS / 100; // 100_000_000
// pub const MILLICENTS: Balance = CENTS / 1_000; // 100_000
// pub const fn deposit(items: u32, bytes: u32) -> Balance {
// 	items as Balance * 20 * DOLLARS + (bytes as Balance) * 100 * MILLICENTS
// 	}
// }


#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_std::vec;
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config + orml_nft::Config<TokenData = Trophy<AccountIdOf<Self>,MomentOf<Self>,BalanceReservableOf<Self>>, ClassData= () > {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
		type Randomness: Randomness<Self::Hash>;
		type One: Get<BalanceOf<Self>>;
		type Zero: Get<BalanceOf<Self>>;	
		type Time: Time;
		
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);
	
	#[pallet::storage]
	#[pallet::getter(fn class_id)]
	pub type ClassId<T: Config> = StorageValue<
		_,
		T::ClassId,
		ValueQuery
	>;
	
	#[pallet::storage]
	#[pallet::getter(fn vies)]
	pub type Vies<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		[u8;16],
		VieOf<T>,
		ValueQuery
	>;
	
	#[pallet::storage]
	#[pallet::getter(fn competitors)]
	pub type Competitors<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Competitor,
		ValueQuery,
	>;
	
	#[pallet::storage]
	#[pallet::getter(fn operators)]
	pub type Operators<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		[u8;16],
		ValueQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn one)]
	pub type One<T: Config> = StorageValue<
		_,
		BalanceOf<T>,
		ValueQuery
	>;
	
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub sudo: T::AccountId,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { sudo: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			let class_id = <orml_nft::Module<T>>::create_class(&self.sudo,Vec::new(),()).expect("Cannot fail or invalid chain spec");
			ClassId::<T>::put(class_id);
		}
	}

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
		/// VieCreated [competitor]
		VieCreated(Vec<T::AccountId>),
		/// CompetitorJoinedVie [vie_id, who]
		CompetitorJoinedVie([u8;16],T::AccountId),
		/// VieFinished [vie_id]
		VieFinished([u8;16]),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		/// Overflow
		Overflow,
		/// NoneValue
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// PayoutRatioNotEqualTo100
		PayoutRatioNotEqualTo100,
		/// PodiumSpotOutOfRange
		PodiumSpotsOutOfRange,
		/// VieRequiresAtLeast2Participants
		VieRequiresAtLeast2Participants,
		/// OperatorCanOnlyBeInOneVieAtATime
		OperatorCanOnlyBeInOneVieAtATime,
		/// CompetitorCanOnlyBeInOneVieAtATime
		CompetitorCanOnlyBeInOneVieAtATime,
		/// CompetitorNotInvitedToJoinVie
		CompetitorNotInvitedToJoinVie,
		/// VieDoesNotExist
		VieDoesNotExist,
		/// OperatorDoesNotExist
		OperatorDoesNotExist,
		///PodiumSpotsMustMatchPayoutStructure
		PodiumSpotsMustMatchPodiumStructure,
		/// TotalPodiumStakeMustEqualAllCompetitorsStake
		PodiumRewardSumMustEqualAllCompetitorsStake,
		/// DuplicateCompetitors
		DuplicateCompetitorsAreNotAllowed,
		///PodiumMustOnlyHaveCompetitorsInVie
		PodiumMustOnlyHaveCompetitorsInVie,
		///CompetitorAlreadyStaked
		CompetitorAlreadyStaked,
		///CompetitorsMustAllJoinToFinishTheVie
		CompetitorsMustAllJoinToFinishTheVie,
		///MemoTooBig
		MemoTooBig,
		///RemoveOperatorFromCompetitors 
		RemoveOperatorFromCompetitors
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T:Config> Pallet<T> {

		// #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		#[pallet::weight(1_000_000_000_000)]
		pub fn vie(origin: OriginFor<T>, vie_request: VieRequestOf<T> ) -> DispatchResultWithPostInfo {
			let operator = ensure_signed(origin)?;
			ensure!(vie_request.memo.len() <= 256, Error::<T>::MemoTooBig);
			ensure!(!<Operators<T>>::contains_key(&operator),Error::<T>::OperatorCanOnlyBeInOneVieAtATime);
			ensure!(vie_request.competitors.len()>=1,Error::<T>::VieRequiresAtLeast2Participants);
			ensure!(Self::validate_spots(&vie_request.places), Error::<T>::PodiumSpotsOutOfRange);
			ensure!(Self::validate_total_stake(&vie_request), Error::<T>::PodiumRewardSumMustEqualAllCompetitorsStake);
			ensure!(Self::validate_competitors(&vie_request.competitors), Error::<T>::DuplicateCompetitorsAreNotAllowed);
			ensure!(!vie_request.competitors.contains(&operator), Error::<T>::RemoveOperatorFromCompetitors);
			//Operator cannot be a player in the current game 
			//Friend
			for p in vie_request.competitors.iter() {
				ensure!(!<Competitors<T>>::contains_key(&p),Error::<T>::CompetitorCanOnlyBeInOneVieAtATime);
			}
			match with_transaction_result(|| {
				let stake = vie_request.stake.clone();
				T::Currency::reserve(&operator,stake)?;
				let vie_id = Self::random_value(&operator);
				let vie = Self::new_vie(&vie_request,operator.clone());
				<Vies<T>>::insert(vie_id.clone(),vie.clone());
				<Operators<T>>::insert(operator.clone(),vie_id.clone());
				Self::new_competitors(vie_id,&vie_request.competitors);
				<Competitors<T>>::insert(&operator,Competitor{
					vie_id: vie_id.clone(),
					staked: true,
					submitted_winner: false,
				});
				Self::deposit_event(Event::VieCreated(vie_request.competitors));
				Ok(())
			}){
				Ok(_)=>Ok(().into()),
				Err(e)=>Err(e.into()),
			}
		}

		// #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		#[pallet::weight(1_000_000_000_000)]
		pub fn join(origin: OriginFor<T>) ->DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			ensure!(<Competitors<T>>::contains_key(&who),Error::<T>::CompetitorNotInvitedToJoinVie);
			let c: Competitor = <Competitors<T>>::get(&who);
			ensure!(!c.staked,Error::<T>::CompetitorAlreadyStaked);
			ensure!(<Vies<T>>::contains_key(c.vie_id),Error::<T>::VieDoesNotExist);
			let vie: VieOf<T> =<Vies<T>>::get(c.vie_id);
			match with_transaction_result(|| {
				T::Currency::transfer(&who,&vie.operator,vie.stake.clone(), ExistenceRequirement::KeepAlive)?;
				T::Currency::reserve(&vie.operator,vie.stake.clone())?;
				<Competitors<T>>::mutate(&who,|this| {
					this.staked = true;
				});
				Self::deposit_event(Event::CompetitorJoinedVie(c.vie_id,who));
				Ok(())
			}){
				Ok(_)=>Ok(().into()),
				Err(e)=>Err(e.into()),
			}
		}

		// #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		#[pallet::weight(1_000_000_000_000)]
		pub fn finish(origin: OriginFor<T>,podium: PodiumReqOf<T>) ->DispatchResultWithPostInfo {
			let operator = ensure_signed(origin)?;
			ensure!(<Operators<T>>::contains_key(&operator),Error::<T>::OperatorDoesNotExist);
			let vie_id = <Operators<T>>::get(&operator);
			ensure!(<Vies<T>>::contains_key(vie_id),Error::<T>::VieDoesNotExist);
			let vie: VieOf<T> =<Vies<T>>::get(vie_id);
			ensure!(Self::validate_podium_spots_request(&podium.podium,vie.clone()),Error::<T>::PodiumSpotsMustMatchPodiumStructure);
			//Check all competitors are valid 
			//Check that the operator is valid
			//Current error when submitting operator in podium from the validate_competitors_in_vie
			ensure!(Self::validate_competitors_in_vie(podium.clone(),vie.clone()),Error::<T>::PodiumMustOnlyHaveCompetitorsInVie);
			ensure!(Self::validate_competitors_staked(vie.clone()),Error::<T>::CompetitorsMustAllJoinToFinishTheVie);
			
			match with_transaction_result(|| {
				// ensure!(Self::payout(podium.podium.clone(),vie.clone()),Error::<T>::PodiumSpotsMustMatchPodiumStructure);
				// let trophy = Self::random_value(&podium.champion);
				let mut vie_copy = vie.clone(); 
				let mut winners = podium.podium.clone();
				vie_copy.places.sort_by(|a,b|a.spot.cmp(&b.spot));
				winners.sort_by(|a,b|a.spot.cmp(&b.spot));
				for i in 0..vie.places.len() {
					let winnings = vie.places[i].payout.clone();
					let competitor = winners[i].competitor.clone();
					T::Currency::repatriate_reserved(&vie.operator,&competitor,winnings,BalanceStatus::Free)?;
				}
				let trophy = Trophy {
					trophy: vie_id.clone(),
					competitors: vie.competitors.clone(),
					stake: vie.stake.clone(),
					memo: vie.memo.clone(),
					time: vie.time.clone(),
					podium: winners.clone(),
				};
				let _nft_id = NftModule::<T>::mint(&podium.champion,Self::class_id(),Default::default(),trophy.clone())?;	
				for p in vie.competitors.iter() {
					<Competitors<T>>::remove(p);
				}
				<Competitors<T>>::remove(&operator);
				<Vies<T>>::remove(vie_id);
				<Operators<T>>::remove(&operator);
				Self::deposit_event(Event::VieFinished(vie_id));
				Ok(())
				}){
					Ok(_)=>Ok(().into()),
					Err(e)=>Err(e.into()),
				}
		}

		// #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		#[pallet::weight(1_000_000_000_000)]
		pub fn force_finish(origin: OriginFor<T>) ->DispatchResultWithPostInfo {
			let operator = ensure_signed(origin)?;
			ensure!(<Operators<T>>::contains_key(&operator),Error::<T>::OperatorDoesNotExist);
			let vie_id = <Operators<T>>::get(&operator);
			ensure!(<Vies<T>>::contains_key(vie_id),Error::<T>::VieDoesNotExist);
			let vie: VieOf<T> =<Vies<T>>::get(vie_id);
			match with_transaction_result(|| {
				//Unstake all funds 
				for competitor_account_id in vie.competitors.iter() {
					let competitor =  <Competitors<T>>::get(&competitor_account_id);
					if competitor.staked {
						T::Currency::repatriate_reserved(&vie.operator,&competitor_account_id,vie.stake,BalanceStatus::Free)?;
					}
					<Competitors<T>>::remove(&competitor_account_id);
				}
				T::Currency::unreserve(&vie.operator,vie.stake);
				<Competitors<T>>::remove(&operator);
				<Vies<T>>::remove(vie_id);
				<Operators<T>>::remove(&operator);
				Self::deposit_event(Event::VieFinished(vie_id));
				Ok(())
			}){
				Ok(_)=>Ok(().into()),
				Err(e)=>Err(e.into()),
			}
		}
	}
}


impl<T: Config>Pallet<T>{
	fn validate_competitors_staked(vie: VieOf<T> )->bool{
		for i in vie.competitors {
			let c = <Competitors<T>>::get(&i);
			if !c.staked {
				return false 
			}
		}
		true
	}

	fn validate_competitors_in_vie(podium: PodiumReqOf<T>, vie: VieOf<T> )->bool{
		for i in podium.podium {
			if !vie.competitors.contains(&i.competitor) {
				return false
			}
		}
		true
	}

	fn validate_competitors(competitors: &Vec<T::AccountId>)->bool{
		if (1..competitors.len()).any(|i| competitors[i..].contains(&competitors[i - 1])) {
			return false 
		}
		return true
	}

	fn validate_total_stake(vie_req: &VieRequestOf<T>)->bool{
		let stake = vie_req.stake.saturated_into::<u64>();
		let competitors = vie_req.competitors.len() as u64;
		let total_stake = stake * (competitors+1 as u64);
		let podium_stake_sum: u64 = vie_req.places.iter().fold(0,|sum,val| sum + val.payout.saturated_into::<u64>());
		if podium_stake_sum != total_stake {
			return false 
		}
		true
	}

	fn random_value(sender: &T::AccountId) -> [u8; 16] {
		let payload = (
			T::Randomness::random_seed(),
			&sender,
			<frame_system::Module<T>>::extrinsic_index(),
		);
		payload.using_encoded(blake2_128)
	}
	
	// fn payout(mut winners: Vec<StandingReq<T::AccountId>>, mut vie: VieOf<T>) -> bool{
	// 	vie.podium.sort_by(|a,b|a.spot.cmp(&b.spot));
	// 	winners.sort_by(|a,b|a.spot.cmp(&b.spot));
	// 	for i in 0..vie.podium.len() {
	// 		let winnings = vie.podium[i].payout.clone();
	// 		let competitor = winners[i].competitor.clone();
	// 		T::Currency::repatriate_reserved(&vie.operator,&competitor,winnings,BalanceStatus::Free);
	// 	}
	// 	true
	// }

	fn validate_podium_spots_request(podium: &Vec<StandingReq<T::AccountId>>, vie: VieOf<T>)-> bool{
		let spots_req: Vec<u32> = podium.iter().map(|p|p.spot).collect();
		let spots: Vec<u32> = vie.places.iter().map(|p|p.spot).collect();
		if spots_req.len() != vie.places.len(){
			return false
		}
		if &spots_req[..] != &spots[..] {
			return false
		} 
		true
	}

	fn validate_spots(podium: &Vec<Place<BalanceOf<T>>>) -> bool{
		let spots: Vec<u32> = podium.iter().map(|p|p.spot).collect();
		for i in 0..spots.len() {
			let index: &u32 = &(i as u32 + 1);
			if !spots.contains(index) {
				return false
			}
		}
		true
	}

	fn new_competitors(vie_id: [u8;16],competitors: &Vec<T::AccountId>){
		competitors.iter().for_each(|c|{
			<Competitors<T>>::insert(&c,Competitor{
				vie_id: vie_id.clone(),
				staked: false,
				submitted_winner: false,
			})		
		});
	}



	fn new_vie(vie: &VieRequestOf<T>, operator: T::AccountId)-> Vie<T::AccountId,BalanceReservableOf<T>,MomentOf<T>> {
		let mut competitors = vie.competitors.clone();
		competitors.push(operator.clone());
		Vie{
			operator: operator.clone(),
			stake: vie.stake.clone(),
			places: vie.places.clone(),
			time: T::Time::now(),
			// date: <pallet_timestamp::Module<T>>::get(),
			competitors: competitors,
			memo: vie.memo.clone(),
			// links: vie.links.clone(),
		}
	}
}