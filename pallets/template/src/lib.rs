#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_module, decl_storage, decl_event, decl_error, ensure, StorageMap, traits::Get};
use frame_system::ensure_signed;
///use frame_system::Module;

use sp_std::vec::Vec;

/// The pallet's configuration trait.
pub trait Trait: frame_system::Trait {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_event! {
    pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
        // Events
        ClaimCreated(AccountId,Vec<u8>),
        ClaimRevoked(AccountId,Vec<u8>),
    }
}

decl_storage! {
    trait Store for Module<T: Trait> as TemplateModule {
        // Storage
        Proofs: map hasher(opaque_blake2_256) Vec<u8> => (T::AccountId,T::BlockNumber);
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Functions
        type Error = Error<T>;
        fn deposit_event() = default;

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        fn create_claim(origin,proof: Vec<u8>){
			let sender = ensure_signed(origin)?;

			ensure!(!Proofs::<T>::contains_key(&proof),Error::<T>::ProofAlreadyClaimed);

			let current_block = <frame_system::Module<T>>::block_number();

			Proofs::<T>::insert(&proof,(&sender,current_block));
			Self::deposit_event(RawEvent::ClaimCreated(sender,proof));
        }
        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        fn revoke_claim(origin,proof: Vec<u8>){
			let sender = ensure_signed(origin)?;
			ensure!(Proofs::<T>::contains_key(&proof),Error::<T>::NoSuchProof);
			let (owner,_) = Proofs::<T>::get(&proof);
			ensure!(sender==owner,Error::<T>::NotProofOwner);
			Proofs::<T>::remove(&proof);
			Self::deposit_event(RawEvent::ClaimRevoked(sender,proof));

        }
    }
}

// This pallet's errors.
decl_error! {
    pub enum Error for Module<T: Trait> {
        /// This claim is already taken!
        ProofAlreadyClaimed,
        /// This claim does not exists!
        NoSuchProof,
        /// Wrong owner
        NotProofOwner,

    }
}
