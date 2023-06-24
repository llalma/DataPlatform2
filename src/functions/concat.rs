use crate::value::Value;

pub(crate) fn concat(input_1: Value, input_2: Value) -> Result<Value, String>{
    return Ok(Value::STRING(format!("{0}{1}", input_1.to_string(), input_2.to_string())))
}

#[cfg(test)]
mod tests {
    use crate::functions::add::add;
    use crate::functions::concat::concat;
    use crate::value::Value;

    #[test]
    fn test_strimg() {
        let val_1 = Value::STRING("test1".to_string());
        let val_2 = Value::STRING("test2".to_string());
        assert_eq!(Value::STRING("test1test2".to_string()), concat(val_1, val_2).unwrap())
    }

    #[test]
    fn test_i32() {
        let val_1 = Value::I32(12);
        let val_2 = Value::I32(23);
        assert_eq!(Value::STRING("1223".to_string()), concat(val_1, val_2).unwrap())
    }

    #[test]
    fn test_f32() {
        let val_1 = Value::F32(326.11, 2);
        let val_2 = Value::F32(2.0031, 3);

        assert_eq!(Value::STRING("326.112.003".to_string()), concat(val_1, val_2).unwrap())
    }

    #[test]
    fn test_f32_and_i32() {
        let val_1 = Value::F32(1.03, 4);
        let val_2 = Value::I32(3);

        assert_eq!(Value::STRING("1.03003".to_string()), concat(val_1, val_2).unwrap())
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
