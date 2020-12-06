mod android_sdk;

pub use android_sdk::*;

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::StdResult;

pub trait Dependency {
    fn check() -> StdResult<()>;
}

pub trait IntoChecks {
    fn into_checks(&self) -> Vec<Box<dyn Fn() -> StdResult<()>>>;
}

macro_rules! tuple_impls {
    ( $( $name:ident )+ ) => {
        impl<$($name: Dependency),+> IntoChecks for ($($name,)+)
        {
            fn into_checks(&self) -> Vec<Box<dyn Fn() -> StdResult<()>>> {
                vec![$(Box::new(|| $name::check()),)+]
            }
        }
    };
}

tuple_impls! { A }
tuple_impls! { A B }
tuple_impls! { A B C }
tuple_impls! { A B C D }
tuple_impls! { A B C D E }
tuple_impls! { A B C D E F }
tuple_impls! { A B C D E F G }
tuple_impls! { A B C D E F G H }
tuple_impls! { A B C D E F G H I }
tuple_impls! { A B C D E F G H I J }
tuple_impls! { A B C D E F G H I J K }
tuple_impls! { A B C D E F G H I J K L }

// #[derive(Clone, Default)]
// pub struct Checks {
//     items: Rc<RefCell<Vec<Box<dyn Fn() -> StdResult<()>>>>>,
// }

// impl Checks {
//     pub fn push(&mut self, item: impl Fn() -> StdResult<()> + 'static) {
//         self.items.borrow_mut().push(Box::new(item));
//     }

//     pub fn run(&self) -> StdResult<()> {
//         self.items.borrow().iter().try_for_each(|item| item())
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     fn check_1() -> StdResult<()> {
//         println!("check 1");
//         Ok(())
//     }

//     fn check_2() -> StdResult<()> {
//         println!("check 2");
//         Err("error check 2".into())
//     }

//     #[test]
//     #[should_panic(expected = "error check 2")]
//     fn test_checks() {
//         let mut checks = Checks::default();
//         checks.push(check_1);
//         checks.push(check_2);
//         checks.run().unwrap();
//     }
// }
