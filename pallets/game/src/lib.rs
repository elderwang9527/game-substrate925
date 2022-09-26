#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	// zz 16
	use frame_support::traits::Randomness;
	// zz 18
	// use sp_io::hashing::blake2_128;

	// zz1, index for fighter
	type FighterIndex = u32;

	// zz3, l4
	#[pallet::type_value]
	pub fn GetDefaultValue() -> FighterIndex {
		0_u32
	}

	// zz4, l2, a Fighter with array [attack, defend, hp]
	#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
	pub struct Fighter(pub [u16; 3]);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		// zz15, l2
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// zz2, l3
	#[pallet::storage]
	#[pallet::getter(fn next_fighter_id)]
	pub type NextFighterId<T> = StorageValue<_, FighterIndex, ValueQuery, GetDefaultValue>;

	// zz5, l3
	#[pallet::storage]
	#[pallet::getter(fn fighters)]
	pub type Fighters<T> = StorageMap<_, Blake2_128Concat, FighterIndex, Fighter>;

	// zz6, l3
	#[pallet::storage]
	#[pallet::getter(fn fighter_owner)]
	pub type FighterOwner<T: Config> = StorageMap<_, Blake2_128Concat, FighterIndex, T::AccountId>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		//zz10
		FighterCreated(T::AccountId, FighterIndex, Fighter),
		FighterTransferred(T::AccountId, T::AccountId, FighterIndex),
	}

	#[pallet::error]
	pub enum Error<T> {
		// zz13
		InvalidFighterId,
		NotOwner,
		TotalOverflow,
		DefOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// zz11
		#[pallet::weight(10_000)]
		pub fn create(origin: OriginFor<T>, hp: u16, atk: u16, def: u16) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(hp + atk * 10 + def * 10 <= 1000, Error::<T>::TotalOverflow);
			ensure!(def <= 50, Error::<T>::DefOverflow);
			let fighter_id = Self::get_next_id().map_err(|_| Error::<T>::InvalidFighterId)?;
			let dna = [hp, atk, def];
			let fighter = Fighter(dna);
			Fighters::<T>::insert(fighter_id, &fighter);
			FighterOwner::<T>::insert(fighter_id, &who);
			NextFighterId::<T>::set(fighter_id + 1);
			Self::deposit_event(Event::FighterCreated(who, fighter_id, fighter));
			Ok(())
		}

		// zz12
		#[pallet::weight(10_000)]
		pub fn transfer(
			origin: OriginFor<T>,
			fighter_id: u32,
			new_owner: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::get_fighter(fighter_id).map_err(|_| Error::<T>::InvalidFighterId)?;

			ensure!(Self::fighter_owner(fighter_id) == Some(who.clone()), Error::<T>::NotOwner);

			<FighterOwner<T>>::insert(fighter_id, new_owner);

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		// zz7
		// fn random_value(sender: &T::AccountId) -> [u8; 16] {
		// 	let payload = (
		// 		T::Randomness::random_seed(),
		// 		&sender,
		// 		<frame_system::Pallet<T>>::extrinsic_index(),
		// 	);
		// 	payload.using_encoded(blake2_128)
		// }

		// zz8
		fn get_next_id() -> Result<FighterIndex, ()> {
			match Self::next_fighter_id() {
				FighterIndex::MAX => Err(()),
				val => Ok(val),
			}
		}

		// zz9
		fn get_fighter(fighter_id: FighterIndex) -> Result<Fighter, ()> {
			match Self::fighters(fighter_id) {
				Some(fighter) => Ok(fighter),
				None => Err(()),
			}
		}
	}
}
