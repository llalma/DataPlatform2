use std::fmt;
use regex::Regex;
use chrono::{NaiveDateTime,};
use ndarray::Array2;
use qndr::{get_alphabets, get_numbers};
use crate::coordinate::Coordinate;
use crate::FUNCTION;
use crate::functions::add::add;

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum Value{
    NULL(),
    I32(i32),
    F32(f32, usize),
    STRING(String),
    DATETIME(NaiveDateTime, String),
    FUNCTION(FUNCTION, Vec<Value>),
    CELL_REFERENCE(Coordinate)
}

//Implementation for default Values of each datatype. Mainly used for comparisons between Enum variants
impl Value{
    fn null_default() -> Self {Value::NULL()}
    fn i32_default() -> Self {Value::I32(0)}
    fn f32_default() -> Self {Value::F32(0.0, 0)}
    fn string_default() -> Self {Value::STRING("".to_owned())}
    fn function_default() -> Self{Value::FUNCTION(FUNCTION::ADD, vec![Value::I32(1), Value::I32(1)])}
    fn reference_default() -> Self{Value::CELL_REFERENCE(Coordinate{row:0,column:0})}

    pub fn create_from_str(value: String) -> Value {

        //If input is ""
        if value == "".to_owned(){
            return Value::NULL()
        }

        //If input is datetime
        //Currently only parses a single format
        const DATETIME_FORMAT: &str = "%Y%m%d %H%M%S";
        if let Ok(res) = NaiveDateTime::parse_from_str(&value, DATETIME_FORMAT){
            return Value::DATETIME(res, DATETIME_FORMAT.to_owned());
        }

        //If input is i32
        if let Ok(res) = value.parse::<i32>() {
            return Value::I32(res)
        }

        //If input is f32
        if let Ok(res) = value.parse::<f32>() {
            let re = Regex::new(r"\.(\d*)").unwrap();
            let precision = re.find(&value).map(|x| x.len()-1).unwrap_or(1);   //Minus 1 since capture includes the decimal point
            return Value::F32(res, precision)
        }

        //Check if is a Function string - String can represent other functions as this solves for recursive methods
        if let Ok(res) = FUNCTION::starts_with(&value){

            //Split on the opening (
            let mut splitter_opening = value.splitn(2, '(');
            let mut values = splitter_opening.nth(1).unwrap().trim();

            //Split on the closing )
            let mut splitter_closing = values.splitn(2, ')');
            values = splitter_closing.next().unwrap().trim();

            // Split on all , not inside (). This is done manually as rust regex does not support negative lookahead
            let mut brackets_count = 0;
            let mut end_last_string_index = 0;
            let mut split_values: Vec<Value> = vec![];
            for (i, c) in values.chars().enumerate(){

                //Add a count if a bracket was opened
                if c == '(' {brackets_count+=1}
                //Minus a count if a bracket was closed
                if c == ')' {brackets_count-=1}

                //Add any values to vec if can split as not inside ()
                if c == ',' && brackets_count == 0{
                    split_values.push(Value::create_from_str(values[end_last_string_index..i].to_owned()));
                    end_last_string_index = i+1 //Increment by 1 so next split does not contain the comma just split on
                }

                //If the last char then add anything else to the vec
                //Can do this as the string should never end in a comma so should not duplicate by accident
                if i == values.len()-1{
                    split_values.push(Value::create_from_str(values[end_last_string_index..].to_owned()));
                }
            }

            return Value::FUNCTION(res, split_values)
        }

        //If input is a cell reference
        let re = Regex::new(r"^\[{1}\w*\]{1}$").unwrap();
        if let Some(res) = re.find(&value.to_uppercase()) {

            //Find the letters to convert to column number
            let column: i32 = get_alphabets(res.as_str()).chars()
                .rev()
                .enumerate()
                .map(|(i,c)| i32::pow(26,i as u32)*(c as i32 - 64))
                .sum::<i32>()-1; //Minus 1 as 0 indexing

            //Find the digits to convert to row number - can unwrap as we only got numeric chars
            let row = get_numbers(res.as_str()).parse::<i32>().unwrap();

            return Value::CELL_REFERENCE(Coordinate{row:row as usize, column:column as usize})
        }

        //If all checks are done assume its an actual string
        return Value::STRING(value)
    }

    pub fn solve_reference<'a>(&'a self, data: &'a Array2<Value>) -> Option<&Value>{
        /*
        Returns the value in a given cell for the referenced cell.
         */

        //Dont need the if, but cant find how to resolve without if let or match statement
        if let Self::CELL_REFERENCE(coord) = self {
            return data.get((coord.row, coord.column))
        }

        return None
    }
}

impl Default for Value {
    fn default() -> Self {Value::NULL()}
}

//Formatting for Value Enum
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::NULL() => write!(f, ""),
            Value::I32(val) => write!(f, "{}", val.to_string()),
            Value::F32(val, precision) => write!(f, "{}", format!("{val:.prec$}", val=val, prec=precision)),
            Value::STRING(val) => write!(f, "{}", val),
            Value::DATETIME(val, format) => write!(f, "{}", val.format(format)),
            Value::FUNCTION(function, vals) => write!(f, "{}", "Not Implemented"),
            Value::CELL_REFERENCE(coords) => write!(f, "{}", coords)
        }
    }
}

pub fn solve_function(
    function: &FUNCTION,
    values: &Vec<Value>) -> Result<Value, String> {
    /*
    Implements the matching logic for determining which function to execute dependent on the given ENUM
     */


    //I feel like I should be borrowing values here instead of cloning but then have to deal with lifetimes in return component
    return match function {
        FUNCTION::ADD => add(values[0].clone(), values[1].clone()),

        _ => Ok(Value::I32(1))
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
    use chrono::{NaiveDateTime};
    use crate::coordinate::Coordinate;
    use crate::FUNCTION;
    use crate::value::{get_same_form, Value};

    #[test]
    fn test_from_string_null(){
        let input = Value::create_from_str("".to_string());
        let expected = Value::NULL();
        assert_eq!(input, expected)
    }

    #[test]
    fn test_from_string_i32(){
        let input = Value::create_from_str("3".to_string());
        let expected = Value::I32(3);
        assert_eq!(input, expected)
    }

    #[test]
    fn test_from_string_datetime(){
        let input = Value::create_from_str("20210418 210328".to_string());
        let expected = Value::DATETIME(NaiveDateTime::parse_from_str("20210418 210328", "%Y%m%d %H%M%S").unwrap(), "%Y%m%d %H%M%S".to_owned());
        assert_eq!(input, expected)
    }

    #[test]
    fn test_from_string_f32(){
        let mut input = Value::create_from_str("3.4".to_string());
        let mut expected = Value::F32(3.4,1);
        assert_eq!(input, expected);
        input = Value::create_from_str("5.143".to_string());
        expected = Value::F32(5.143,3);
        assert_eq!(input, expected);
        input = Value::create_from_str("5.".to_string());
        expected = Value::F32(5.0,0);
        assert_eq!(input, expected);
    }

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
        let current_datetime = chrono::Utc::now().naive_utc();
        let input_format = "%Y %m %d %H:%M";

        //Create Value datatype and Expected outcome
        let input = Value::DATETIME(current_datetime, input_format.to_owned());
        let expected = current_datetime.format(input_format);

        //Check outputs match
        assert_eq!(expected.to_string(), input.to_string())
    }

    #[test]
    fn test_from_string_string(){
        let input = Value::create_from_str("this is a test string".to_owned());
        let expected = Value::STRING("this is a test string".to_owned());
        assert_eq!(input, expected)
    }

    #[test]
    fn test_from_string_cell_reference(){
        let input = Value::create_from_str("[ZA62]".to_owned());
        let expected = Value::CELL_REFERENCE(Coordinate {row:62, column:677});
        assert_eq!(input, expected)
    }

    #[test]
    fn test_from_string_function(){
        let input = Value::create_from_str("ADD(2,ADD(1,2))".to_owned());
        let expected = Value::FUNCTION(FUNCTION::ADD,
                                       vec![Value::I32(2), Value::FUNCTION(FUNCTION::ADD,
                                                            vec![Value::I32(1), Value::I32(2)])]);

        assert_eq!(input, expected)
    }
}
