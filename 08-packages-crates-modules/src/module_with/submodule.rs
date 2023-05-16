pub fn pr() {
    println!("submodule!");
}

mod another_submodule {
    pub mod new_submodule {
        pub fn function() {}
    }

    fn call_super() {
        // to reference an item in the parent module use `super::`
        super::pr();
    }

    // we can create public structs with just a few public keys, but enums are always full pub or not
    pub struct public_struct {
        pub public_key: String,
        private_key: i32,
    }
    // if we don't implement a way to create a new public_struct, then we won't be able to as private_key is private and can't be accessed or asigned
    impl public_struct {
        pub fn add(public: &str) -> public_struct {
            public_struct {
                public_key: String::from(public),
                private_key: 1,
            }
        }
    }
}

pub fn call_mod() {
    //absolute path
    crate::module_with::submodule::another_submodule::new_submodule::function;
    //relative path
    another_submodule::new_submodule::function;
    //"Our preference in general is to specify absolute paths because it’s more likely we’ll want to move code definitions and item calls independently of each other"
}