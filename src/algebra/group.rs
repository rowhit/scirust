// std imports

// local imports
use algebra::monoid::{MonoidAddPartial, MonoidAdd, 
    MonoidMulPartial, MonoidMul};
use algebra::loop_::{LoopAddPartial, LoopAdd, 
    LoopMulPartial, LoopMul};


///////////////////////////////////////////////////////////

pub trait GroupAddPartial 
: MonoidAddPartial  + LoopAddPartial
{

}

impl<T> GroupAddPartial for T where
    T: MonoidAddPartial + LoopAddPartial
{}



///////////////////////////////////////////////////////////


pub trait GroupAdd 
: GroupAddPartial + MonoidAdd  + LoopAdd
{

}


impl<T> GroupAdd for T where
    T: MonoidAdd + LoopAdd + GroupAddPartial
{}


///////////////////////////////////////////////////////////


pub trait GroupMulPartial 
: MonoidMulPartial  + LoopMulPartial
{

}

impl<T> GroupMulPartial for T where
    T: MonoidMulPartial + LoopMulPartial
{}




///////////////////////////////////////////////////////////


pub trait GroupMul 
: GroupMulPartial + MonoidMul  + LoopMul
{

}


impl<T> GroupMul for T where
    T: MonoidMul + LoopMul + GroupMulPartial
{}


///////////////////////////////////////////////////////////

pub trait CommutativeGroupAddPartial 
: GroupAddPartial
{

    /// Returns `true` if the addition operator is approximately commutative for
    /// the given argument tuple.
    fn prop_add_is_commutative_partial(a : Self, b : Self) -> bool {
        let ab = a.clone() + b.clone();
        let ba = b.clone() + a.clone();
        ab == ba
    }

}

impl CommutativeGroupAddPartial for i8   {}
impl CommutativeGroupAddPartial for i16  {}
impl CommutativeGroupAddPartial for i32  {}
impl CommutativeGroupAddPartial for i64  {}
impl CommutativeGroupAddPartial for f32  {}
impl CommutativeGroupAddPartial for f64  {}

///////////////////////////////////////////////////////////

pub trait CommutativeGroupAdd
: CommutativeGroupAddPartial + GroupAdd
{

    /// Returns `true` if the addition operator is approximately commutative for
    /// the given argument tuple.
    fn prop_add_is_commutative_partial(a : Self, b : Self) -> bool {
        let ab = a.clone() + b.clone();
        let ba = b.clone() + a.clone();
        ab == ba
    }

}

impl CommutativeGroupAdd for i8   {}
impl CommutativeGroupAdd for i16  {}
impl CommutativeGroupAdd for i32  {}
impl CommutativeGroupAdd for i64  {}




///////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use algebra::zero::Zero;
    use algebra::one::One;

    fn check_group_add_partial<T: GroupAddPartial>(a: T, b: T, c : T)->T{
        let d = a + b;
        let e = d - c;
        -e + Zero::zero()
    }


    #[test]
    fn test_group_add_partial() {
        assert_eq!(check_group_add_partial(2i8, 3i8, 1i8), -4);
        assert_eq!(check_group_add_partial(2i16, 3i16, 1i16), -4);
        assert_eq!(check_group_add_partial(2i32, 3i32, 1i32), -4);
        assert_eq!(check_group_add_partial(2i64, 3i64, 1i64), -4);
        assert_eq!(check_group_add_partial(2f32, 3f32, 1f32), -4f32);
        assert_eq!(check_group_add_partial(2f64, 3f64, 1f64), -4f64);
        // The following will not compile
        // assert_eq!(check_group_add_partial(2u8, 3u8, 1u8), -4);
        // assert_eq!(check_group_add_partial(2u16, 3u16, 1u16), -4);
        // assert_eq!(check_group_add_partial(2u32, 3u32, 1u32), -4);
        // assert_eq!(check_group_add_partial(2u64, 3u64, 1u64), -4);
    }


    fn check_group_add<T: GroupAdd>(a: T, b: T, c : T)->T{
        let d = a + b;
        let e = d - c;
        -e
    }


    #[test]
    fn test_group_add() {
        assert_eq!(check_group_add(2i8, 3i8, 1i8), -4);
        assert_eq!(check_group_add(2i16, 3i16, 1i16), -4);
        assert_eq!(check_group_add(2i32, 3i32, 1i32), -4);
        assert_eq!(check_group_add(2i64, 3i64, 1i64), -4);
        // The following will not compile
        // assert_eq!(check_group_add(2u8, 3u8, 1u8), -4);
        // assert_eq!(check_group_add(2u16, 3u16, 1u16), -4);
        // assert_eq!(check_group_add(2u32, 3u32, 1u32), -4);
        // assert_eq!(check_group_add(2u64, 3u64, 1u64), -4);
        //assert_eq!(check_group_add(2f32, 3f32, 1f32), -4f32);
        //assert_eq!(check_group_add(2f64, 3f64, 1f64), -4f64);
    }



    fn check_group_mul_partial<T: GroupMulPartial>(a: T, b: T, c : T)->T{
        let d = a * b;
        let e = d / c;
        e.recip() * One::one()
    }


    #[test]
    fn test_group_mul_partial() {
        assert_eq!(check_group_mul_partial(2f32, 3f32, 3f32), 0.5f32);
        assert_eq!(check_group_mul_partial(2f64, 3f64, 3f64), 0.5f64);
        // The following will not compile
        // assert_eq!(check_group_mul_partial(2i8, 3i8, 1i8), -4);
        // assert_eq!(check_group_mul_partial(2i16, 3i16, 1i16), -4);
        // assert_eq!(check_group_mul_partial(2i32, 3i32, 1i32), -4);
        // assert_eq!(check_group_mul_partial(2i64, 3i64, 1i64), -4);
        // assert_eq!(check_group_add_partial(2u8, 3u8, 1u8), -4);
        // assert_eq!(check_group_add_partial(2u16, 3u16, 1u16), -4);
        // assert_eq!(check_group_add_partial(2u32, 3u32, 1u32), -4);
        // assert_eq!(check_group_add_partial(2u64, 3u64, 1u64), -4);
    }



}

