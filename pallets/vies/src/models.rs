
#[cfg(feature = "std")]
use serde::{Serialize, Deserialize};
use frame_support::{codec::{Encode, Decode},RuntimeDebug};
use sp_std::{vec::Vec};

pub type Weight = Balance;
pub type Time = Time;
 //map of account id to topic. 
 //account id is the operator (first contributor) then the decided. 
pub type Id = [u8;32];
pub type Members = Vec<Id>; //Array of member ids 
pub type Assets = Vec<Id>;
pub type Topics = Vec<Id>;
pub type Clubs =  Vec<Id>; //
pub type Weighed = bool;

//Clubs = Array of club names 
//Club = map club name to club
//Vec<Club<Id,Weight,Assets,Topics,Members>>
#[derive( Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Club<Id,Weight,Assets,Topics,Members> {
	pub id:  Id,
	pub created: Time,
	pub members: Members,
	pub weight: Weight, //contribution to start club and for the members to join
	pub topics: Topics, //Topic ids 
}

// Memberships = Map of account id, club name to Member;
// Gets dust collect if below ext deposit and clubs is nul
#[derive( Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Member<Id,Weight,Assets,Topics,Members> {
	pub id:  Id,
	pub created: Time,
	pub club_ids: Vec<Id>,//belongs to club or member.//can be map of club name to club id
	// pub weight: Weight, //contribution to become a member
	// pub clubs: Clubs, //array of club ids
	// pub assets: Assets, //asset/s content identifiers
}

/* 
	Topics = map of (club name, topic name) to an array of Topic

	On club creation a topic is made "Genesis decision. Create or Distruct."
	to create the club and become part of the genesis of the club.
	A topic can have a assets that a memeber is contriubuting too.
	If the exp is less then now the topic is closed.
	Member operator can re open with new assets, weight and exp.
	
	Topic name like a mnemonic so its easy to remember
	Member operator can modify the member_operator
	i.e to self destruct
 */
#[derive( Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Topic<Weight, Members, Time, Assets,Decisions,Name> {
	pub id:  Id,
	pub member_operator: Id,
	pub club_id:  Id,
	pub created: Time,
	pub weight: Weight,
	pub exp: Time, 
	pub name:  Name,
	pub decisions: Decisions, //decisions match assets by index then topic id gets added to club. Member becomes MemberOperator if decided.
	// pub assets: Assets, //asset/s content identifiers from member
}

//Decisions = Map of (account_id,club name) to Decision
//On Join create a Decision option (remove after contributing) for the Member of a Club on a Topic 
//A Memeber joins a Topic and wants to widthdrawl 
#[derive( Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Decision<Id,Index,Assets,Topics,Members> {
	pub id: Id,
	pub club_id: Id,
	pub member_id: Id,
	pub index:  Index, 
	pub latest: Time,
	pub decided: bool,
}

//Assets = Map of (topic name) to an array of Asset
//Cant remove asset if in topic
#[derive( Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Asset<Id,Weight,Assets,Topics,Members,Cid> {
	pub id:  Id, 
	// pub member_operator: Id,//owner.
	pub created: Time,
	pub latest: Time,
	pub cid: Cid, //
	pub weight: Weight, //contribution to start(member&operator)/join(member) club
	// Can be cid, hash of asset.
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