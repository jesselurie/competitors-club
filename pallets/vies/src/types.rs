
#[cfg(feature = "std")]
use serde::{Serialize, Deserialize};
use frame_support::{codec::{Encode, Decode},RuntimeDebug};
use sp_std::{vec::Vec};

#[derive(Encode, Decode, Default, Clone, PartialEq,RuntimeDebug)]
pub struct Place<Balance> {
	pub spot: u32, 
	pub payout: Balance,
}

#[derive(Encode, Decode, Default, Clone, PartialEq,RuntimeDebug)]
pub struct Competitor {
	pub vie_id: [u8;16],
	pub staked: bool,
	pub submitted_winner: bool,
}

#[derive(Encode, Decode, Default, Clone, PartialEq,RuntimeDebug)]
pub struct Vie<AccountId, Balance, Moment> {
    pub operator: AccountId,
	pub stake: Balance,
	pub places: Vec<Place<Balance>>,
	pub time: Moment,
	pub competitors: Vec<AccountId>,
	pub memo: Vec<u8>,
	// pub links: Vec<u8>,
}

#[derive(Encode, Decode, Default, Clone, PartialEq,RuntimeDebug)]
pub struct VieReq<AccountId, Balance> {
	pub stake: Balance,
	pub places: Vec<Place<Balance>>,
	pub competitors: Vec<AccountId>,
	pub memo: Vec<u8>,
	// pub links: Vec<u8>,
}

#[derive(Encode, Decode, Default, Clone, PartialEq,Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct StandingReq<AccountId> {
	pub competitor: AccountId,
	pub spot: u32,
}

#[derive(Encode, Decode, Default, Clone, PartialEq,RuntimeDebug)]
pub struct PodiumReq<AccountId>{
	pub champion: AccountId,
	pub podium: Vec<StandingReq<AccountId>>,
}

#[derive( Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Trophy<AccountId ,Moment, Amount> {
	pub trophy:  [u8; 16],
	pub competitors: Vec<AccountId>,
	pub stake: Amount,
	pub memo: Vec<u8>,
	pub time: Moment,
	pub podium: Vec<StandingReq<AccountId>>,
}

// {
// 	memo: 'Chess Tournament',
// 	podium:{
// 	  1:30,
// 	  2:7,
// 	  3:3
// 	},
// 	stake: 10,
// 	players: [
// 	  "ACDGSSDFSDF",
// 	  "ACCOPUNT_ID_2",
// 	  "ACCOPUNT_ID_3",
// 	  "ACCOPUNT_ID_4",
// 	],
// 	totalStake: 40, 
// 	finalPodium: {
// 	  1:  "ACDGSSDFSDF",
// 	  2: "ACCOPUNT_ID_2",
// 	  3: "ACCOPUNT_ID_3",
// 	},
// 	timestamp: 123123412231,
//   }