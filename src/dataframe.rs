use std::fmt;
use ndarray::prelude::*;
use crate::value::Value;
use crate::functions::concat::concat;
use crate::functions::add::add;
enum FUNCTION{
    ADD,
    MOD,
    CONCAT
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct DataFrame{
    data: Array2<Value>
}

impl DataFrame{
    fn apply(&mut self,
             row_start: usize,
             column_start: usize,
             function: FUNCTION,
             apply_df: Array2<Value>
    ){


        let slice_index = s![row_start..row_start+apply_df.dim().0, column_start..column_start+apply_df.dim().1];

        let rows_slice:ArrayView2<Value> = self.data.slice(slice_index);

        let updated_slice = arr1(&rows_slice.into_iter()
            .zip(apply_df)
            .map(|(v1, v2)|
                match function {
                    FUNCTION::ADD => add(v1.clone(), v2.clone()),
                    FUNCTION::MOD => add(v1.clone(), v2.clone()),
                    FUNCTION::CONCAT => concat(v1.clone(), v2.clone())
                }
                .unwrap()
            )
            .collect::<Vec<Value>>()).into_shape(rows_slice.dim()).unwrap();

        self.data.slice_mut(slice_index).assign(
            &updated_slice
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::dataframe::{DataFrame, FUNCTION};
    use crate::value::Value;
    use ndarray::prelude::*;


    #[test]
    fn test_df_print() {
        assert!(false)
    }

    #[test]
    fn test_df_apply_add(){
        //Testing apply on all values in column for a simple addition of 2
        let mut input = DataFrame {
            data: arr2(&[[Value::I32(1), Value::I32(4)],
                        [Value::I32(3), Value::I32(5)],
                        [Value::I32(2), Value::I32(6)]])
        };

        let expected = DataFrame{
            data: arr2(&[
                        [Value::I32(3), Value::I32(4)],
                        [Value::I32(5), Value::I32(5)],
                        [Value::I32(4), Value::I32(6)]
                    ])
        };

        //Apply the addition of 2 to all values in column
        input.apply(
            0,0,
            FUNCTION::ADD,
            arr2(&[
                        [Value::I32(2)],
                        [Value::I32(2)],
                        [Value::I32(2)]
                    ])
        );

        assert_eq!(expected, input);
    }
}

