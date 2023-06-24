use std::cmp::max;
use crate::value::{get_same_form, Value};

pub(crate) fn add(input_1: Value, input_2: Value) -> Result<Value, String>{
    // Convert to the same datatype and the one with the most precision
    let (val_1, val_2) = get_same_form(input_1, input_2).unwrap();

    // Sum the actual values
    return match (val_1, val_2) {
        (Value::F32(val_1_val,val_1_prec), Value::F32(val_2_val,val_2_prec)) => Ok(Value::F32(val_1_val+val_2_val, max(val_1_prec, val_2_prec))),
        (Value::I32(val_1), Value::I32(val_2)) => Ok(Value::I32(val_1 + val_2)),
        _ => Err("Not valid datatypes for addition".to_string())
    }

}

#[cfg(test)]
mod tests {
    use crate::functions::add::add;
    use crate::value::Value;

    #[test]
    fn test_i32() {
        let val_1 = Value::I32(12);
        let val_2 = Value::I32(23);
        assert_eq!(Value::I32(35), add(val_1, val_2).unwrap())
    }

    #[test]
    fn test_f32() {
        let val_1 = Value::F32(326.11, 2);
        let val_2 = Value::F32(2.0031, 3);

        let (val, prec) = match add(val_1, val_2).unwrap() {
            Value::F32(val, prec) => (val, prec),
            _ => (0.0, 0)
        };
        assert_eq!(328.113.to_string(), format!("{val:.prec$}", val=val, prec=prec))
    }

    #[test]
    fn test_f32_and_i32() {
        let val_1 = Value::F32(1.03, 4);
        let val_2 = Value::I32(3);

        let (val, prec) = match add(val_1, val_2).unwrap() {
            Value::F32(val, prec) => (val, prec),
            _ => (0.0, 0)
        };
        assert_eq!("4.0300".to_string(), format!("{val:.prec$}", val=val, prec=prec))
    }

    #[test]
    fn test_i32_and_f32() {
        let val_2 = Value::I32(5);
        let val_1 = Value::F32(6.9, 2);


        let (val, prec) = match add(val_1, val_2).unwrap() {
            Value::F32(val, prec) => (val, prec),
            _ => (0.0, 0)
        };
        assert_eq!("11.90".to_string(), format!("{val:.prec$}", val=val, prec=prec))
    }
}
