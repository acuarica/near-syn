type AType = i32;

/// Doc-comments for a type def
type BType = i32;

/// Doc-comment line 1 for A
/// Doc-comment line 2 for A
/// Doc-comment line 3 for A
#[derive(Serialize)]
struct A {
    // No doc-comment for this field
    a1_field: U64,
    a2_field: U64,

    /// Line for a3
    /// Line for a2, then blank line
    ///
    /// Some markdown
    /// ```
    /// const a = [];
    /// const b = "";
    /// ```
    a3_field: U128,
}

// No doc-comment for this struct
#[derive(Serialize)]
struct B {
    b: U64,
}

/// non-serde enums are not exported
enum E0 {
    V1,
    V2,
}

/// doc-comment for enum
#[derive(Serialize)]
enum E {
    V1,
    V2,
}

#[near_bindgen]
struct C {
    f128: U128,
}

#[near_bindgen]
impl C {
    /// init func
    #[init]
    pub fn init_here(f128: U128) -> Self {
        Self { f128 }
    }

    /// Line 1 for get_f128 first
    /// Line 2 for get_f128 second
    pub fn get_f128(&self) -> U128 {
        self.f128
    }

    // Regular comments are not transpiled
    /// Set f128.
    pub fn set_f128(&mut self, value: U128) {
        self.f128 = value;
    }

    pub fn get_f128_other_way(&self, key: U128) -> U128 {
        self.f128 + key
    }

    pub fn more_types(&mut self, key: U128, tuple: (String, BTreeSet<i32>)) -> () {
        self.f128 = key;
    }

    /// Pay to set f128.
    #[payable]
    pub fn set_f128_with_sum(&mut self, a_value: U128, other_value: U128) {
        self.f128 = a_value + other_value;
    }

    #[private]
    pub fn marked_as_private(&mut self) {}

    fn private_method_not_exported(&self, value: U128) -> U128 {
        self.f128
    }

    fn private_mut_method_not_exported(&mut self, value: U128) {
        self.f128 = value;
    }
}

#[near_bindgen]
impl C {
    /// another impl
    pub fn another_impl(&self, f128: U128) -> U128 {
        f128
    }
}

// All methods for traits are public, and thus exported
#[near_bindgen]
impl I for C {
    /// Single-line comment for get
    fn get(&self) -> U128 {
        self.f128
    }
}

// Omitted since near-bindgen is not present, methods not exported
impl J for C {
    fn m() {}
}

// Omitted since even near-bindgen is present, methods are private
#[near_bindgen]
impl K for C {
    #[private]
    fn p() {}
}

mod inner_mod {
    type A_in_mod = u32;
}
