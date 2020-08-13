#[macro_use]
extern crate rand_derive2;

#[cfg(test)]
mod test {
    #[derive(RandGen)]
    #[allow(dead_code)]
    struct SomeFields {
        age: i32,
        byte: u8
    }

    #[derive(RandGen)]
    pub struct UnitStruct;

    #[derive(RandGen)]
    #[allow(dead_code)]
    struct Recursive {
        field0: SomeFields,
        field1: std::string::String,
        field2: uuid::Uuid,
        field3: UnitStruct,
        field4: Vec<u8>,
        field5: std::vec::Vec<u8>,
    }

    #[derive(RandGen)]
    #[allow(dead_code)]
    struct CustomRand {
        #[custom_rand]
        field0: &'static str,
        #[custom_rand]
        field1: Vec<i32>,
        #[custom_rand]
        field2: UnitStruct
    }

    impl CustomRand {
        const STRING: &'static str = "string";

        fn vec() -> Vec<i32> {
            vec![1, 2]
        }
    }

    impl TestDataProviderForCustomRand for CustomRand {
        fn generate_field0() -> &'static str {
            CustomRand::STRING
        }

        fn generate_field1() -> Vec<i32> {
            CustomRand::vec()
        }

        fn generate_field2() -> UnitStruct {
            UnitStruct
        }
    }

    #[derive(RandGen)]
    struct UnnamedBoi(SomeFields, String, i32);

    #[derive(RandGen)]
    pub enum SomeEnum {
        Empty,
        Named { some_field: i32, another_field: i32 },
        Unnamed(i32, i32)
    }

    #[derive(RandGen, PartialEq, Debug)]
    pub enum SomeEnumSkipVariants {
        #[skip_variant]
        SkipMe,
        DontSkipMe,
        DontSkipMeAlso
    }

    #[derive(RandGen)]
    #[allow(dead_code)]
    struct Options {
        #[always_some]
        field0: Option<i32>,
        field1: Option<i32>
    }

    #[test]
    fn test_random_types() {
        let _: SomeFields = rand::random();
        let _: Recursive = rand::random();
        let _: UnnamedBoi = rand::random();
        let _: SomeEnum = rand::random();
    }

    #[test]
    fn test_loop() {
        for _ in 0..100 {
            // Generate a SomeEnumSkipVariants variants and verify it never generates a SkipMe variant
            let variant: SomeEnumSkipVariants = rand::random();

            assert_ne!(variant, SomeEnumSkipVariants::SkipMe);

            // Generate a Options and verify field0 is always filled
            let options: Options = rand::random();

            assert!(options.field0.is_some(), options.field0);
        }
    }

    #[test]
    fn test_custom_rand() {
        let custom_rand: CustomRand = rand::random();

        assert_eq!(CustomRand::STRING, custom_rand.field0);
        assert_eq!(CustomRand::vec(), custom_rand.field1);
    }
}