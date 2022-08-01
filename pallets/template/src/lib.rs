#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	// The struct on which we build all of our Pallet logic.
    #[pallet::pallet]
    pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Token was initialized by user
		Initialized(T::AccountId),
		/// Tokens successfully transferred between users
		Transfer(T::AccountId, T::AccountId, u64), // (from, to, value)
	}

	#[pallet::storage]
	pub(super) type Balances<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, u64, ValueQuery>;

	#[pallet::type_value]
	pub(super) fn TotalSupplyDefaultValue<T: Config>() -> u64 {
		21000000
	}

	#[pallet::storage]
	pub(super) type TotalSupply<T: Config> =
		StorageValue<_, u64, ValueQuery, TotalSupplyDefaultValue<T>>;

	#[pallet::storage]
	pub(super) type Init<T: Config> = StorageValue<_, bool, ValueQuery>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::error]
	pub enum Error<T> {
		/// Attempted to initialize the token after it had already been initialized.
		AlreadyInitialized,
		/// Attempted to transfer more funds than were available
		InsufficientFunds,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Initialize the token
		/// transfers the total_supply amout to the caller
		#[pallet::weight(10_000)]
		pub fn init(_origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(_origin)?;
			ensure!(!<Init<T>>::get(), <Error<T>>::AlreadyInitialized);

			<Balances<T>>::insert(sender.clone(), <TotalSupply<T>>::get());

			Init::<T>::put(true);
			Self::deposit_event(Event::Initialized(sender));
			Ok(().into())
		}

		/// Transfer tokens from one account to another
		#[pallet::weight(10_000)]
		pub fn transfer(
			_origin: OriginFor<T>,
			to: T::AccountId,
			value: u64,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(_origin)?;
			let sender_balance = <Balances<T>>::get(&sender);
			let receiver_balance = <Balances<T>>::get(&to);

			// Calculate new balances
			let updated_from_balance = sender_balance
				.checked_sub(value)
				.ok_or(<Error<T>>::InsufficientFunds)?;
			let updated_to_balance = receiver_balance
				.checked_add(value)
				.expect("Entire supply fits in u64; qed");

			// Write new balances to storage
			<Balances<T>>::insert(&sender, updated_from_balance);
			<Balances<T>>::insert(&to, updated_to_balance);

			Self::deposit_event(Event::Transfer(sender, to, value));
			Ok(().into())
		}
	}
}