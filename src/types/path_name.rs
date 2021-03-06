//! Path name.
//!
//! Example: `/foo/bar.txt`

use crate::types;
use crate::types::{text as supertype, text::Text as Supertype};

pub type PathName = Supertype;
type T = PathName;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

pub fn fab() -> T {
    T::from(format!(
        "/{}/{}",
        types::path_dir_name::fab(),
        types::path_base_name::fab(),
    ))
}

#[cfg(test)]
mod tests {
    use crate::types::{path_name as t, path_name::PathName as T};

    #[test]
    fn test_from_str() {
        let s = "alpha";
        let x: T = T::from(s);
        assert_eq!(x, s);
    }

    #[test]
    fn test_from_option_str() {
        let s = "alpha";
        let x: T = t::from_option_str(Some(s)).unwrap();
        assert_eq!(x, s);
    }

    #[test]
    fn test_fab() {
        let x: T = t::fab();
        assert!(!x.is_empty());
    }

}
