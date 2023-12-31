#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{pallet_prelude::*, Blake2_128Concat};
	use frame_system::pallet_prelude::*;
	use frame_support::dispatch::Vec;

	#[pallet::pallet]
	// #[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;
	}


	#[derive(Encode, Decode, Clone, PartialEq, Default, TypeInfo)]
	pub struct TeamsData {
		//Team Id
		pub team_id: i64,
		//Team Name
		pub team_name: Vec<u8>,
		//Team Size
		pub team_size: i64,
		//Team Description
		pub description: Vec<u8>
	}

	#[derive(Encode, Decode, Clone, PartialEq, Default, TypeInfo)]
	pub struct UserData {
		pub user_id: i64,
		pub user_name: Vec<u8>
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn team_data)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type AccountTeamData<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, TeamsData, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn user_data)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type AccountUserData<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, UserData, OptionQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		TeamsDataStored { team: T::AccountId },
		UserDataStored { user: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(0)]
		// #[pallet::weight(T::WeightInfo::add_teams_data())]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn add_teams_data(origin: OriginFor<T>, 
		team_id: i64, team_name: Vec<u8>, team_size: i64, description: Vec<u8>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			// Update storage.
			//<Something<T>>::put(something);
			let new_team = TeamsData { team_id, team_name, team_size, description };
			<AccountTeamData<T>>::insert(&who, new_team);

			// Emit an event.
			Self::deposit_event(Event::<T>::TeamsDataStored { team: who } );
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn add_user_data(origin: OriginFor<T>, 
		user_id: i64, user_name: Vec<u8>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			// Update storage.
			//<Something<T>>::put(something);
			let new_user = UserData { user_id, user_name };
			<AccountUserData<T>>::insert(&who, new_user);

			// Emit an event.
			Self::deposit_event(Event::<T>::UserDataStored { user: who } );
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		// An example dispatchable that may throw a custom error.
		// #[pallet::call_index(1)]
		// #[pallet::weight(T::WeightInfo::cause_error())]
		// pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
		// 	let _who = ensure_signed(origin)?;

		// 	// Read a value from storage.
		// 	match <Something<T>>::get() {
		// 		// Return an error if the value has not been set.
		// 		None => return Err(Error::<T>::NoneValue.into()),
		// 		Some(old) => {
		// 			// Increment the value read from storage; will error in the event of overflow.
		// 			let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
		// 			// Update the value in storage with the incremented result.
		// 			<Something<T>>::put(new);
		// 			Ok(())
		// 		},
		// 	}
		// }
	}
}
