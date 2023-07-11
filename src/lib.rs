extern crate console_error_panic_hook;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use strum_macros::EnumString;
use std::str::FromStr;
use std::collections::HashMap;
use core::fmt;
use ndarray::AssignElem;

mod value;
mod coordinate;
mod functions;


use ndarray::prelude::*;
use wasm_bindgen::describe::FUNCTION;
use crate::value::{solve_function, Value};
use crate::coordinate::Coordinate;
use crate::functions::concat::concat;
use crate::functions::add::add;


#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(EnumString)]
pub enum FUNCTION{
    ADD,
    MOD,
    CONCAT
}
impl FUNCTION{
    fn starts_with(value: &str) -> Result<FUNCTION, &str>{
        /* Attempts to convert any given string to a Function ENUM, tries to split on first "("
        then parse that into a enum */

        //Get String before first occurance of '('
        let mut splitter = value.splitn(2, '(');
        let first = splitter.next().unwrap().trim();

        //Parse into Enum
        if let Ok(res) = FUNCTION::from_str(first){
            return Ok(res)
        }

        return Err("Error, not a valid Function")
    }
}
impl fmt::Display for FUNCTION {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[wasm_bindgen]
#[derive(Debug)]
pub struct DataFrame{
    data: Array2<Value>,
    references: HashMap<Coordinate, Vec<Coordinate>>
}

impl DataFrame {
    pub fn new(data: Vec<Vec<String>>) -> Self {
        //Try parse each value in array2 to their corresponding value enum.
        //Trys to assume from string, need a way to parse inputs as a certain type.

        let mut data_transformed = Array2::<Value>::default((data.len(), data[0].len()));
        for (i, mut row) in data_transformed.axis_iter_mut(Axis(0)).enumerate() {
            for (j, col) in row.iter_mut().enumerate() {
                *col = Value::create_from_str(data[i][j].to_string());
            }
        }

        Self {
            data: data_transformed,
            references: HashMap::new()
        }
    }

    fn simplify(&self, coords1: Coordinate, coords2: Coordinate) -> Array2<Value> {
        /*
        For the given coordinate ranges simplify any functions or Cell references to simple Values.
        Simple values  in this case being anything but cell references and function
         */

        // Get the slice of data we care about
        let mut binding = self.data.clone();
        let mut printing_slice = binding.slice_mut(s!(coords1.row..coords2.row+1, coords1.column..coords2.column+1));

        //Solve all the values in the slice to non complex data

        return Array2::from_shape_vec( printing_slice.dim(),
                                    printing_slice.into_iter()
                                    .map(|v| self.solve_cell(v))
                                    .collect::<Vec<Value>>()
            ).unwrap();
    }

    fn solve_cell(&self, cell: &Value) -> Value {
        /*
        Recursive function which solves cell references and functions until a simple datatype is returned.
        In this context a simple datatype is any value that is not a cell reference or function
         */

        //This clones the values sot ehy can be inserted into DF if required without de-referencing
        return match cell {
            Value::CELL_REFERENCE(_) => self.solve_cell(cell.solve_reference(&self.data).unwrap()).clone(),
            Value::FUNCTION(function, values) => solve_function(function, &(values.into_iter().map(|v| self.solve_cell(v)).collect::<Vec<Value>>())).unwrap(),
            _ => cell.clone()
        };
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::value::Value;
    use ndarray::prelude::*;
    use crate::{DataFrame, FUNCTION};
    use crate::coordinate::Coordinate;

    #[test]
    fn test_df_print() {
        assert!(false)
    }

    #[test]
    fn test_simplify(){

        /*
        Create the input values
         */
        let mut input = DataFrame::new(
          vec![
            vec!["3".to_string()],   //A1
            vec!["ADD([A0],[A0])".to_string()], //A2
            vec!["ADD([A1],[A0])".to_string()]  //A3
          ]
        );

        // //Add the cell references to the struct for the input
        // input.references.insert(
        //     Coordinate {row:0 ,column:0},
        //     vec![Coordinate {row:1,column:0}]
        // );
        // input.references.insert(
        //     Coordinate {row:0 ,column:0},
        //     vec![Coordinate {row:2,column:0}]
        // );
        // input.references.insert(
        //     Coordinate {row:1 ,column:0},
        //     vec![Coordinate {row:2,column:0}]
        // );

        /*
        Create the expected result
         */
        let expected = DataFrame {
            data: arr2(&[[Value::I32(3)],   //A1
                        [Value::I32(6)],    //A2
                        [Value::I32(9)]]),   //A3
            references: HashMap::new()
        };

        assert_eq!(expected.data, input.simplify(Coordinate{row:0,column:0}, Coordinate{row:2,column:0}));
    }
}

