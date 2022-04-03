use mockall::*;
use mockall::predicate::*;

use crate::users::storage::get::{get};

// Interface -> list of signed methods for dynamodb
#[automock]
pub trait DynamoDBInterface {
    // list users
    // create user
    // update user
    // delete user
}

#[automock]
pub trait MathInterface {
    fn add(&self, x: u32, y: u32) -> u32;
}

// Storage functions
pub fn get_users() {
    println!("Service layer called!");
    get();
}

fn do_the_math(x: &dyn MathInterface) -> u32 {
    x.add(4, 3)
}

// Tests
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let mut mock = MockMathInterface::default();
        mock.expect_add()
            .returning(|x, y| x + y);
        assert_eq!(do_the_math(&mock), 7);
    }
}