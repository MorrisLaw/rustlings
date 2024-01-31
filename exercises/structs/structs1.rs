// structs1.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a
// hint.
struct ColorClassicStruct {
    red: i32,
    green: i32,
    blue: i32,
}

fn newColorClassicStruct(red: i32, green: i32, blue: i32) -> ColorClassicStruct {
    return ColorClassicStruct{red, green, blue};
}

struct ColorTupleStruct(i32, i32, i32);

fn newColorTupleStruct(red: i32, green: i32, blue: i32) -> ColorTupleStruct {
    return ColorTupleStruct(red, green, blue);
}

#[derive(Debug)]
struct UnitLikeStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        let green = &newColorClassicStruct(0, 255, 0);
                        
        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        let green = &newColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit-like struct!
        let unit_like_struct = UnitLikeStruct;
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
