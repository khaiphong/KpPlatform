//! # KpPlatform
//!
//! A collection of crates to enable scientifically sultivable #Prajna
//! to know more about oneself toward What Count.

pub mod platform {   // platform business
}
pub mod container {  // containrer platform business
}
pub mod mu {         // mu business
}
pub mod db {         // db business
}

pub mod ai {
 /// ai business
 pub enum Traits {
  Truth(String), 
  Care(String), 
  Honesty(String), // "Honesty / TRUTH / Chân"
  Budh(String),    // "Budh / Morality / Thiện"
  Patience(String), // "Patience / Nhẩn"
  Trust(String),
  Qi(String),
  Art(String),
  Empathy(String), 
  Kindness(String), 
  
  Balanced(String),
  
  Influenced(String),
  Veiled(String),
  BindingWord(String),
  BindingImage(String),
  ClingingThought(String)
  }
}
pub mod eip {        // eip business
}
pub mod hub {        // hub business
}
pub mod plan {       // plan business
}



pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}

