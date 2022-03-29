#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::dispatch::DispatchResult;
pub use pallet::*;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use sp_std::vec::Vec;
#[frame_support::pallet]
pub mod pallet {
	pub use super::*;
	use frame_support::{
		sp_runtime::traits::Hash,
		traits::{ Randomness, Currency, tokens::ExistenceRequirement, Time},
		transactional
	};
	use sp_io::hashing::blake2_128;
	use scale_info::TypeInfo;
	
	#[cfg(feature = "std")]
	use frame_support::serde::{Deserialize, Serialize};

	type AccountOf<T> = <T as frame_system::Config>::AccountId;
	type TypeTime<T> = <<T as Config>::TimeNew as Time>::Moment;

	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	impl<T: Config> MaxEncodedLen for Domain<T> {
		fn max_encoded_len() -> usize {
			let len: usize = 4;
			len
		}
	}
	// Struct for holding domain information.
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]

	pub struct Domain<T: Config> {
		pub domain: Vec<u8>,
		pub price: Option<BalanceOf<T>>,
		pub wallet: Option<Vec<u8>>,
		pub owner: AccountOf<T>,
		pub date_created: Option<TypeTime<T>>
	}

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types it depends on.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The Currency handler for the domains pallet.
		type Currency: Currency<Self::AccountId>;

		/// The maximum amount of domains a single account can own.
		#[pallet::constant]
		type MaxDomainOwned: Get<u32>;
		type TimeNew: Time;
	}

	// Errors.
	#[pallet::error]
	pub enum Error<T> {
		/// Handles arithmetic overflow when incrementing the domain counter.
		domainCntOverflow,
		/// An account cannot own more domains than `MaxdomainCount`.
		ExceedMaxdomainOwned,
		/// Buyer cannot be the owner.
		BuyerIsdomainOwner,
		/// Cannot transfer a domain to its owner.
		TransferToSelf,
		/// Handles checking whether the domain exists.
		domainNotExist,
		/// Handles checking that the domain is owned by the account transferring, buying or setting a price for it.
		NotdomainOwner,
		/// Ensures the domain is for sale.
		domainNotForSale,
		/// Ensures that the buying price is greater than the asking price.
		domainBidPriceTooLow,
		/// Ensures that an account has enough funds to purchase a domain.
		NotEnoughBalance,
		DomainIsset
	}

	// Events.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new domain was successfully created. \[sender, domain_id\]
		Created(T::AccountId, T::Hash),
		/// domain price was successfully set. \[sender, domain_id, new_price\]
		PriceSet(T::AccountId, T::Hash, Option<BalanceOf<T>>),
		WalletSet(T::AccountId, T::Hash, Option<Vec<u8>>),
		/// A domain was successfully transferred. \[from, to, domain_id\]
		Transferred(T::AccountId, T::AccountId, T::Hash),
		/// A domain was successfully bought. \[buyer, seller, domain_id, bid_price\]
		Bought(T::AccountId, T::AccountId, T::Hash, BalanceOf<T>),
	}

	// Storage items.

	#[pallet::storage]
	#[pallet::getter(fn domain_cnt)]
	/// Keeps track of the number of domains in existence.
	pub(super) type DomainCnt<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn domains)]
	/// Stores a domain's unique traits, owner and price.
	pub(super) type Domains<T: Config> = StorageMap<_, Twox64Concat, T::Hash, Domain<T>>;

	#[pallet::storage]
	#[pallet::getter(fn domains_owned)]
	/// Keeps track of what accounts own what domain.
	pub(super) type DomainsOwned<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, BoundedVec<T::Hash, T::MaxDomainOwned>, ValueQuery>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new unique domain.
		///
		/// The actual domain creation is done in the `mint()` function.
		#[pallet::weight(42_192_000 + T::DbWeight::get().writes(1))]
		pub fn create_domain(origin: OriginFor<T>, domain: Vec<u8>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let domain_id = T::Hashing::hash_of(&domain);
			ensure!(Self::is_domain_isset(&domain_id)?, <Error<T>>::DomainIsset);

			let domain_id = Self::mint(&sender, &domain)?;

			// Logging to the console
			log::info!("A domain is born with ID: {:?}.", domain_id);
			// Deposit our "Created" event.
			Self::deposit_event(Event::Created(sender, domain_id));
			Ok(())
		}

		/// Set the price for a domain.
		///
		/// Updates domain price and updates storage.
		#[pallet::weight(100)]
		pub fn set_price(
			origin: OriginFor<T>,
			domain_id: T::Hash,
			new_price: Option<BalanceOf<T>>
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// Ensure the domain exists and is called by the domain owner
			ensure!(Self::is_domain_owner(&domain_id, &sender)?, <Error<T>>::NotdomainOwner);

			let mut domain = Self::domains(&domain_id).ok_or(<Error<T>>::domainNotExist)?;

			domain.price = new_price.clone();
			<Domains<T>>::insert(&domain_id, domain);

			// Deposit a "PriceSet" event.
			Self::deposit_event(Event::PriceSet(sender, domain_id, new_price));

			Ok(())
		}

		#[pallet::weight(100)]
		pub fn set_wallet(
			origin: OriginFor<T>,
			domain_id: T::Hash,
			new_wallet: Option<Vec<u8>>
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// Ensure the domain exists and is called by the domain owner
			ensure!(Self::is_domain_owner(&domain_id, &sender)?, <Error<T>>::NotdomainOwner);

			let mut domain = Self::domains(&domain_id).ok_or(<Error<T>>::domainNotExist)?;

			domain.wallet = new_wallet.clone();
			<Domains<T>>::insert(&domain_id, domain);
			Self::deposit_event(Event::WalletSet(sender, domain_id, new_wallet));

			Ok(())
		}

		/// Directly transfer a domain to another recipient.
		///
		/// Any account that holds a domain can send it to another Account. This will reset the asking
		/// price of the domain, marking it not for sale.
		#[pallet::weight(100)]
		pub fn transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			domain_id: T::Hash
		) -> DispatchResult {
			let from = ensure_signed(origin)?;

			// Ensure the domain exists and is called by the domain owner
			ensure!(Self::is_domain_owner(&domain_id, &from)?, <Error<T>>::NotdomainOwner);

			// Verify the domain is not transferring back to its owner.
			ensure!(from != to, <Error<T>>::TransferToSelf);

			// Verify the recipient has the capacity to receive one more domain
			let to_owned = <DomainsOwned<T>>::get(&to);
			ensure!((to_owned.len() as u32) < T::MaxDomainOwned::get(), <Error<T>>::ExceedMaxdomainOwned);

			Self::transfer_domain_to(&domain_id, &to)?;

			Self::deposit_event(Event::Transferred(from, to, domain_id));

			Ok(())
		}

		/// Buy a saleable domain. The bid price provided from the buyer has to be equal or higher
		/// than the ask price from the seller.
		///
		/// This will reset the asking price of the domain, marking it not for sale.
		/// Marking this method `transactional` so when an error is returned, we ensure no storage is changed.
		#[transactional]
		#[pallet::weight(100)]
		pub fn buy_domain(
			origin: OriginFor<T>,
			domain_id: T::Hash,
		) -> DispatchResult {
			let buyer = ensure_signed(origin)?;

			// Check the domain exists and buyer is not the current domain owner
			let domain = Self::domains(&domain_id).ok_or(<Error<T>>::domainNotExist)?;
			ensure!(domain.owner != buyer, <Error<T>>::BuyerIsdomainOwner);

			// Check the domain is for sale and the domain ask price <= bid_price
			if let Some(ask_price) = domain.price {
				ensure!(T::Currency::free_balance(&buyer) >= ask_price, <Error<T>>::NotEnoughBalance);

				// Verify the buyer has the capacity to receive one more domain
				let to_owned = <DomainsOwned<T>>::get(&buyer);
				ensure!((to_owned.len() as u32) < T::MaxDomainOwned::get(), <Error<T>>::ExceedMaxdomainOwned);

				let seller = domain.owner.clone();

				// Transfer the amount from buyer to seller
				T::Currency::transfer(&buyer, &seller, ask_price, ExistenceRequirement::KeepAlive)?;

				// Transfer the domain from seller to buyer
				Self::transfer_domain_to(&domain_id, &buyer)?;

				Self::deposit_event(Event::Bought(buyer, seller, domain_id, ask_price));

			} else {
				Err(<Error<T>>::domainNotForSale)?;
			}

			Ok(())
		}
	}

	//** Our helper functions.**//

	impl<T: Config> Pallet<T> {
		// Helper to mint a domain.
		pub fn mint(
			owner: &T::AccountId,
			domain: &Vec<u8>,
		) -> Result<T::Hash, Error<T>> {
			let domainStruct = Domain::<T> {
				price: None,
				domain: domain.clone(),
				wallet: None,
				owner: owner.clone(),
				date_created: Some(T::TimeNew::now())
			};

			let domain_id = T::Hashing::hash_of(&domain);

			// Performs this operation first as it may fail
			let new_cnt = Self::domain_cnt().checked_add(1)
				.ok_or(<Error<T>>::domainCntOverflow)?;

			// Performs this operation first because as it may fail
			<DomainsOwned<T>>::try_mutate(&owner, |domain_vec| {
				domain_vec.try_push(domain_id)
			}).map_err(|_| <Error<T>>::ExceedMaxdomainOwned)?;

			<Domains<T>>::insert(domain_id, domainStruct);
			<DomainCnt<T>>::put(new_cnt);
			Ok(domain_id)
		}

		pub fn is_domain_owner(domain_id: &T::Hash, acct: &T::AccountId) -> Result<bool, Error<T>> {
			match Self::domains(domain_id) {
				Some(domain) => Ok(domain.owner == *acct),
				None => Err(<Error<T>>::domainNotExist)
			}
		}

		pub fn is_domain_isset(domain_id: &T::Hash) -> Result<bool, Error<T>> {
			match Self::domains(domain_id) {
				Some(domain) => Err(<Error<T>>::DomainIsset),
				None => Ok(true)
			}
		}

		#[transactional]
		pub fn transfer_domain_to(
			domain_id: &T::Hash,
			to: &T::AccountId,
		) -> Result<(), Error<T>> {
			let mut domain = Self::domains(&domain_id).ok_or(<Error<T>>::domainNotExist)?;

			let prev_owner = domain.owner.clone();

			// Remove `domain_id` from the domainOwned vector of `prev_domain_owner`
			<DomainsOwned<T>>::try_mutate(&prev_owner, |owned| {
				if let Some(ind) = owned.iter().position(|&id| id == *domain_id) {
					owned.swap_remove(ind);
					return Ok(());
				}
				Err(())
			}).map_err(|_| <Error<T>>::domainNotExist)?;

			// Update the domain owner
			domain.owner = to.clone();
			// Reset the ask price so the domain is not for sale until `set_price()` is called
			// by the current owner.
			domain.price = None;

			<Domains<T>>::insert(domain_id, domain);

			<DomainsOwned<T>>::try_mutate(to, |vec| {
				vec.try_push(*domain_id)
			}).map_err(|_| <Error<T>>::ExceedMaxdomainOwned)?;

			Ok(())
		}
	}
}