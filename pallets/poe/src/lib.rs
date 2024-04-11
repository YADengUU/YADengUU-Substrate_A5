#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet{
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
    
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The maximum length of claim that can be added.
        #[pallet::constant]
        type MaxClaimLength: Get<u32>;
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn proofs)]
    pub type Proofs<T: Config> = StorageMap<
        _,
        Blake2_128Concat, //hash algorithm used to calculate the storage location of our storage item in the underlying database
        BoundedVec<u8, T::MaxClaimLength>,
        (T::AccountId, BlockNumberFor<T>),
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config>{
        ClaimCreated(T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
        ClaimRevoked(T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
        //ClaimTransferred(T::AccountId, T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
    }

    #[pallet::error]
    pub enum Error<T>{
        ProofAlreadyExist,
        ClaimTooLong,
        ClaimNotExist,
        NotClaimOwner,
        SameOwner,
    }

    // define reserved functions, but our PoE module doesn't use any reserved functions, we leave it empty
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T>{}

    // now define the dispatchable functions
    #[pallet::call]
    impl<T: Config> Pallet<T>{
        #[pallet::call_index(0)] //defines the order of the dispatchable functions within the module
        #[pallet::weight(0)]
        pub fn create_claim(origin: OriginFor<T>, claim: BoundedVec<u8, T::MaxClaimLength>) -> DispatchResultWithPostInfo{
            let sender = ensure_signed(origin)?;
            ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

            // to insert our key-value pair
            Proofs::<T>::insert(
                &claim, //  the key is a bounded vector which is claim here
                (sender.clone(), frame_system::Pallet::<T>::block_number()),// and its value is a tuple of two elements: the first is the sender of the transaction and the other is the curent block number
            );

            // trigger the ClaimCreated event to indicate that the claim has been successfully created
            Self::deposit_event(Event::ClaimCreated(sender, claim));

            Ok(().into())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(0)]
        pub fn revoke_claim(origin: OriginFor<T>, claim: BoundedVec<u8, T::MaxClaimLength>,) -> DispatchResultWithPostInfo{
            let sender = ensure_signed(origin)?;
            let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;
            ensure!(owner==sender, Error::<T>::NotClaimOwner);

            Proofs::<T>::remove(&claim);

            Self::deposit_event(Event::ClaimRevoked(sender,claim));

            Ok(().into())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(0)]
        pub fn transfer_claim(origin: OriginFor<T>,  claim: BoundedVec<u8, T::MaxClaimLength>, recipient: T::AccountId) -> DispatchResultWithPostInfo{
            let sender = ensure_signed(origin)?;
            let (owner, _block_number) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;
            ensure!(owner==sender, Error::<T>::NotClaimOwner);

            ensure!(sender!=recipient, Error::<T>::SameOwner); // ensure the snder is not the same as new owner

            // update the claim's owner
            Proofs::<T>::insert(&claim, (recipient,frame_system::Pallet::<T>::block_number()));

            // trigger the event for claim transfer (it seems this was not necessary)
            //Self::deposit_event(Event::ClaimTransferred(sender, recipient, claim));

            Ok(().into())

        }
    }

}