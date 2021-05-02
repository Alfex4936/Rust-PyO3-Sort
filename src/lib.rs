extern crate pyo3;

use pyo3::prelude::*;
use pyo3::types::{PyInt, PyList};

#[pymodule]
fn rust_sort(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "r_bubble")]
    fn bubble(_py: Python, array: &PyList) {
        // sort the array *inplace*
        let mut is_sorted = false;
        let mut count = 0;
        let length = array.len() - 1;
        while !is_sorted {
            is_sorted = true;
            for i in 0..length - count {
                let current = array.get_item(i as isize).extract::<i64>().unwrap();
                let next = array.get_item((i + 1) as isize).extract::<i64>().unwrap();
                if current > next {
                    array.set_item(i as isize, next).unwrap();
                    array.set_item((i + 1) as isize, current).unwrap();
                    is_sorted = false;
                }
            }
            count += 1;
        }
    }

    #[pyfn(m, "r_select")]
    fn selection_sort(_py: Python, array: &PyList) {
        let n = array.len();
        let mut changed = false;
        for i in 0..n - 1 {
            changed = true;
            let mut smallest = i;
            for j in i + 1..n {
                let current = array.get_item(smallest as isize).extract::<i64>().unwrap();
                let next = array.get_item((j) as isize).extract::<i64>().unwrap();
                if next < current {
                    smallest = j;
                    changed = false;
                }
            }
            if !changed {
                let current = array.get_item(smallest as isize).extract::<i64>().unwrap();
                let next = array.get_item(i as isize).extract::<i64>().unwrap();
                array.set_item(smallest as isize, next).unwrap();
                array.set_item(i as isize, current).unwrap();
            }
        }
    }

    #[pyfn(m, "r_sum")]
    fn rust_sum(_py: Python, array: &PyList) -> PyResult<i64> {
        let mut sum = 0_i64;
        for d in array.iter() {
            // sum += array.get_item(i).extract::<i64>().unwrap();
            sum += d.extract::<i64>().unwrap();
        }

        Ok(sum)
    }

    Ok(())
}
