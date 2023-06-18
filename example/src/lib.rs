#![warn(missing_docs)]
#![allow(dead_code)]
//! Example for using the `rand_derive2` crate.

use rand_derive2::RandGen;

#[derive(RandGen)]
struct SomeFields {
    age: i32,
    byte: u8,
}

/// Documented
#[derive(RandGen)]
pub struct UnitStruct;

/// Documented
#[derive(RandGen)]
pub struct Recursive {
    field0: SomeFields,
    field1: std::string::String,
    field2: uuid::Uuid,
    field3: UnitStruct,
    field4: Vec<u8>,
    field5: std::vec::Vec<u8>,
    #[rand_derive(default)]
    field6: std::string::String,
}

/// Documented
#[derive(RandGen)]
pub struct CustomRand {
    #[rand_derive(custom)]
    field0: &'static str,
    #[rand_derive(custom)]
    field1: Vec<i32>,
    #[rand_derive(custom)]
    field2: UnitStruct,
}

impl CustomRand {
    const STRING: &'static str = "string";

    fn vec() -> Vec<i32> {
        vec![1, 2]
    }
}

impl TestDataProviderForCustomRand for CustomRand {
    fn generate_field0<R: rand::Rng + ?Sized>(_: &mut R) -> &'static str {
        CustomRand::STRING
    }

    fn generate_field1<R: rand::Rng + ?Sized>(_: &mut R) -> Vec<i32> {
        CustomRand::vec()
    }

    fn generate_field2<R: rand::Rng + ?Sized>(_: &mut R) -> UnitStruct {
        UnitStruct
    }
}

/// Documented
#[derive(RandGen)]
struct DefaultVecIsEmptyVec {
    #[rand_derive(empty)]
    empty_vec: Vec<i32>,
}

#[derive(RandGen)]
struct UnnamedBoi(SomeFields, String, i32);

/// Documented
#[derive(RandGen)]
#[allow(missing_docs)]
pub enum SomeEnum {
    Empty,
    Named { some_field: i32, another_field: i32 },
    Unnamed(i32, i32),
}

/// Docuemnted
#[derive(RandGen, PartialEq, Debug)]
#[allow(missing_docs)]
pub enum SomeEnumSkipVariants {
    #[rand_derive(skip)]
    SkipMe,
    DontSkipMe,
    DontSkipMeAlso,
}

/// Documented
#[derive(RandGen)]
#[allow(dead_code)]
pub struct Options {
    #[rand_derive(some)]
    field0: Option<i32>,
    field1: Option<i32>,
}

/// Documented
#[derive(RandGen)]
#[allow(dead_code)]
pub struct PleasePanic {
    #[rand_derive(panic)]
    some_property: i32,
}

/// Documented
#[derive(RandGen)]
#[allow(dead_code)]
pub struct NoneOption {
    #[rand_derive(none)]
    some_property: Option<i32>,
}
#[derive(RandGen)]
struct Fixed {
    #[rand_derive(fixed = "static")]
    str: &'static str,
    #[rand_derive(fixed = "Some String")]
    string: String,
    #[rand_derive(fixed = "1")]
    i32: i32,
    #[rand_derive(fixed = "false")]
    bool_false: bool,
    #[rand_derive(fixed = "true")]
    bool_true: bool,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_customize() {
        let fixed = Fixed::generate_random_customize(|f| f.i32 = 2);

        assert_eq!(2, fixed.i32);
        assert_eq!("static", fixed.str);
    }

    #[test]
    fn test_random_types() {
        let _: SomeFields = rand::random();
        let recursive: Recursive = rand::random();

        assert!(!recursive.field1.is_empty());
        assert!(recursive.field6.is_empty());

        let _: UnnamedBoi = rand::random();
        let _: SomeEnum = rand::random();

        let _ = SomeFields::generate_random();
        let _ = Recursive::generate_random();
        let _ = UnnamedBoi::generate_random();
        let _ = SomeEnum::generate_random();
        let t = DefaultVecIsEmptyVec::generate_random().empty_vec;

        assert!(t.is_empty());
    }

    #[test]
    fn test_fixed() {
        let generated = Fixed::generate_random();

        assert_eq!("static", generated.str);
        assert_eq!("Some String", generated.string);
        assert_eq!(1, generated.i32);
        assert_eq!(false, generated.bool_false);
        assert_eq!(true, generated.bool_true);
    }

    #[test]
    fn test_loop() {
        for _ in 0..100 {
            // Generate a SomeEnumSkipVariants variants and verify it never generates a SkipMe variant
            let variant: SomeEnumSkipVariants = rand::random();

            assert_ne!(variant, SomeEnumSkipVariants::SkipMe);

            // Generate a Options and verify field0 is always filled
            let options: Options = rand::random();

            assert!(options.field0.is_some());

            // Generate a NoneOption and verify its property is always None
            let op: NoneOption = rand::random();

            assert_eq!(op.some_property, None);
        }
    }

    #[test]
    fn test_custom_rand() {
        let custom_rand: CustomRand = rand::random();

        assert_eq!(CustomRand::STRING, custom_rand.field0);
        assert_eq!(CustomRand::vec(), custom_rand.field1);
    }

    #[test]
    #[should_panic]
    fn test_should_panic() {
        let _: PleasePanic = rand::random();
    }
}
