use std::fmt;
use chrono::{DateTime, Utc};

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub(crate) enum Value{
    NULL(),
    I32(i32),
    F32(f32, usize),
    STRING(String),
    DATETIME(DateTime<Utc>, String)
}

//Implementation for default Values of each datatype. Mainly used for comparisons between Enum variants
impl Value{
    fn null_default() -> Self {Value::NULL()}
    fn i32_default() -> Self {Value::I32(0)}
    fn f32_default() -> Self {Value::F32(0.0, 0)}
    fn string_default() -> Self {Value::STRING("".to_owned())}
    fn datetime_default() -> Self {Value::DATETIME(DateTime::<Utc>::MIN_UTC, "%Y%m%d".to_owned())}
}

//Formatting for Value Enum
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::NULL() => write!(f, ""),
            Value::I32(val) => write!(f, "{}", val.to_string()),
            Value::F32(val, precision) => write!(f, "{}", format!("{val:.prec$}", val=val, prec=precision)),
            Value::STRING(val) => write!(f, "{}", val),
            Value::DATETIME(val, format) => write!(f, "{}", val.format(format))
        }
    }
}

// Function to transform the input values into the same type, using the type with the most precision as the target datatype
pub(crate) fn get_same_form(input_1: Value, input_2: Value) -> Result<(Value, Value), String>{

    if matches!(input_1, Value::F32(_, _)) || matches!(input_2, Value::F32(_, _)){
        Ok((
            transform(input_1, Value::f32_default()).unwrap(),
            transform(input_2, Value::f32_default()).unwrap()
            ))
    }else{
        Ok((
            transform(input_1, Value::i32_default()).unwrap(),
            transform(input_2, Value::i32_default()).unwrap()
        ))
    }

}

fn transform(input: Value, target: Value) -> Result<Value, String>{
    //!For the given input attempt to convert to the target value.
    //!Only required to list the applicable conversions for each input. Otherwise allow the Err.
    //!Used to convert data-types to values so functions can be applied to them.

    //Const for Err to return
    const NOT_VALID_CONVERSION:&str = "Not a valid conversion";

    //Check if targets it F32 and attempt to convert input
    if matches!(target, Value::F32(_,_)){
        return match input {
            Value::I32(val) => Ok(Value::F32(val as f32, 0)),
            Value::F32(_, _) => Ok(input),
            _ => Err(NOT_VALID_CONVERSION.to_owned())
        }
    }

    //Check if targets it I32 and attempt to convert input
    if matches!(target, Value::I32(_)) {
        return match input {
            Value::I32(_) => Ok(input),
            Value::F32(val, _) => Ok(Value::I32(val as i32)),
            _ => Err(NOT_VALID_CONVERSION.to_owned())
        }
    }

    Err("Not a valid target".to_string())
}

#[cfg(test)]
mod tests {
    use crate::value::{get_same_form, Value};

    #[test]
    fn test_f32_and_i32() {
        // assert the an f32 and a i32 input is transformed to 2 f32 values
        let (test_1_val_1, _test_1_val_2) = get_same_form(Value::F32(1.2, 2), Value::I32(4)).unwrap();
        assert!(
            match (test_1_val_1, _test_1_val_2) {
                (Value::F32(_,_), Value::F32(_,_)) => true,
                _ => false
            }
        );

        // assert the an i32 and a f32 input is transformed to 2 f32 values
        let (test_2_val_1, _test_2_val_2) = get_same_form(Value::I32(4), Value::F32(1.2, 2)).unwrap();
        assert!(
            match (test_2_val_1, _test_2_val_2) {
                (Value::F32(_,_), Value::F32(_,_)) => true,
                _ => false
            }
        );
    }

    #[test]
    fn test_f32_formatting(){
        //Input values
        let input_f32:f32 = 2.041;
        let input_prec:usize = 2;

        //
        let input = Value::F32(input_f32, input_prec);
        let expected = "2.04".to_owned();

        //Check outputs match
        assert_eq!(expected.to_string(), input.to_string())
    }

    #[test]
    fn test_datetime_formatting(){
        // Input values
        let current_datetime = chrono::Utc::now();
        let input_format = "%Y %m %d %H:%M";

        //Create Value datatype and Expected outcome
        let input = Value::DATETIME(current_datetime, input_format.to_owned());
        let expected = current_datetime.format(input_format);

        //Check outputs match
        assert_eq!(expected.to_string(), input.to_string())
    }
}
