#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod complex_storage {
    use ink_storage::collections::{HashMap as StorageHashMap, Vec as StorageVec};
    use ink_storage::traits::{SpreadAllocate, PackedLayout, SpreadLayout};

    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub struct User {
        id: AccountId,
        name: Vec<u8>,
        data: StorageVec<Vec<u
    }

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct ComplexStorage {
        owner: AccountId,
        users: StorageHashMap<AccountId, User>,
        data: StorageHashMap<AccountId, StorageVec<Vec<u8>>>,
    }

    #[ink(event)]
    pub struct UserAdded {
        #[ink(topic)]
        user_id: AccountId,
    }

    #[ink(event)]
    pub struct UserRemoved {
        #[ink(topic)]
        user_id: AccountId,
    }

    impl ComplexStorage {
        #[ink(constructor)]
        pub fn new(owner: AccountId) -> Self {
            ink_lang::utils::initialize_contract(|contract| {
                contract.owner = owner;
                contract.users = StorageHashMap::new();
                contract.data = StorageHashMap::new();
            })
        }

        #[ink(message)]
        pub fn add_user(&mut self, id: AccountId, name: Vec<u8>) {
            let user = User {
                id,
                name,
                data: StorageVec::new(),
                is_active: true,
            };
            self.users.insert(id, user);
            self.env().emit_event(UserAdded { user_id: id });
        }

        #[ink(message)]
        pub fn remove_user(&mut self, id: AccountId) {
            self.users.take(&id);
            self.data.take(&id);
            self.env().emit_event(UserRemoved { user_id: id });
        }

        #[ink(message)]
        pub fn add_data(&mut self, id: AccountId, data: Vec<u8>) -> Result<(), &'static str> {
            let user = self.users.get_mut(&id).ok_or("User does not exist")?;
            if !user.is_active {
                return Err("User is not active");
            }
            user.data.push(data.clone());
            let user_data = self.data.entry(id).or_insert(StorageVec::new());
            user_data.push(data);
            Ok(())
        }

        #[ink(message)]
        pub fn get_data(&self, id: AccountId) -> Result<Vec<Vec<u8>>, &'static str> {
            let user = self.users.get(&id).ok_or("User does not exist")?;
            if !user.is_active {
                return Err("User is not active");
            }
            let user_data = self.data.get(&id).ok_or("No data for user")?;
            Ok(user_data.to_vec())
        }

        #[ink(message)]
        pub fn deactivate_user(&mut self, id: AccountId) -> Result<(), &'static str> {
            let user = self.users.get_mut(&id).ok_or("User does not exist")?;
            user.is_active = false;
            Ok(())
        }

        #[ink(message)]
        pub fn activate_user(&mut self, id: AccountId) -> Result<(), &'static str> {
            let user = self.users.get_mut(&id).ok_or("User does not exist")?;
            user.is_active = true;
            Ok(())
        }

        #[ink(message)]
        pub fn get_user(&self, id: AccountId) -> Result<User, &'static str> {
            let user = self.users.get(&id).ok_or("User does not exist")?;
            Ok(user.clone())
        }
    }
}
