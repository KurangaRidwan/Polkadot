// SPDX-License-Identifier: Apache-2.0
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod todo {
    use ink::storage::Mapping;
    use ink::prelude::string::String;

    #[derive(scale::Encode, scale::Decode, Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct TodoItem {
        pub id: u32,
        pub description: String,
        pub done: bool,
    }

    #[ink(storage)]
    pub struct Todo {
        todos: Mapping<u32, TodoItem>,
        next_id: u32,
    }

    /// Events
    #[ink(event)]
    pub struct TodoCreated {
        #[ink(topic)]
        id: u32,
        description: String,
    }

    #[ink(event)]
    pub struct TodoStatusUpdated {
        #[ink(topic)]
        id: u32,
        #[ink(topic)]
        done: bool,
    }

    #[ink(event)]
    pub struct TodoDescriptionUpdated {
        #[ink(topic)]
        id: u32,
        description: String,
    }

    #[ink(event)]
    pub struct TodoDeleted {
        #[ink(topic)]
        id: u32,
    }

    /// Add a Default impl to silence clippy
    impl Default for Todo {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Todo {
        /// Constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                todos: Mapping::default(),
                next_id: 0,
            }
        }

        /// Create a new Todo
        #[ink(message)]
        pub fn create(&mut self, description: String) -> u32 {
            let id = self.next_id;
            let item = TodoItem {
                id,
                description: description.clone(),
                done: false,
            };
            self.todos.insert(id, &item);
            self.next_id = self.next_id.saturating_add(1);

            self.env().emit_event(TodoCreated { id, description });

            id
        }

        #[ink(message)]
        pub fn read(&self, id: u32) -> Option<TodoItem> {
            self.todos.get(id)
        }

        #[ink(message)]
        pub fn update_status(&mut self, id: u32, done: bool) -> bool {
            if let Some(mut item) = self.todos.get(id) {
                item.done = done;
                self.todos.insert(id, &item);

                self.env().emit_event(TodoStatusUpdated { id, done });

                true
            } else {
                false
            }
        }

        #[ink(message)]
        pub fn update_description(&mut self, id: u32, description: String) -> bool {
            if let Some(mut item) = self.todos.get(id) {
                item.description = description.clone();
                self.todos.insert(id, &item);

                self.env().emit_event(TodoDescriptionUpdated { id, description });

                true
            } else {
                false
            }
        }

        #[ink(message)]
        pub fn delete(&mut self, id: u32) -> bool {
            if self.todos.contains(id) {
                self.todos.remove(id);

                self.env().emit_event(TodoDeleted { id });

                true
            } else {
                false
            }
        }
    }
}
