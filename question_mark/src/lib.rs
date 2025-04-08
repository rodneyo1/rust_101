// Define a public struct `One` with a single public field
// `first_layer` that contains an Option of `Two`
pub struct One {
    pub first_layer: Option<Two>,
}

// Define a public struct `Two` with a single public field
// `second_layer` that contains an Option of `Three`
pub struct Two {
    pub second_layer: Option<Three>,
}

// Define a public struct `Three` with a single public field
// `third_layer` that contains an Option of `Four`
pub struct Three {
    pub third_layer: Option<Four>,
}

// Define a public struct `Four` with a single public field
// `fourth_layer` that contains an Option of u16
pub struct Four {
    pub fourth_layer: Option<u16>,
}

impl One {
    // Method to get the u16 value from the deepest layer (Four)
    pub fn get_fourth_layer(self) -> Option<u16> {
        // Use the ? operator to safely unwrap each layer:
        // 1. Unwrap first_layer (Option<Two>), returns None if None
        // 2. Unwrap second_layer (Option<Three>), returns None if None
        // 3. Unwrap third_layer (Option<Four>), returns None if None
        // 4. Finally return the fourth_layer (Option<u16>)
        self.first_layer?
            .second_layer?
            .third_layer?
            .fourth_layer
    }
}