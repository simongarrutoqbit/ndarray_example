use ndarray::{Array, Axis};
use polars::{
    df,
    prelude::{Float64Type, IndexOrder},
};

fn main() {
    let dataframe = df!(
        "data" => &[0.1,0.1,0.1,0.1,0.1,0.1,0.1,0.1,0.1,0.1,0.1,0.1,0.1]
    )
    .unwrap();
    println!("{:?}", dataframe);
    let df: Vec<Array<f64, ndarray::Ix1>> = dataframe
        .to_ndarray::<Float64Type>(IndexOrder::Fortran)
        .unwrap()
        .axis_iter(Axis(1))
        .map(|col| {
            col.to_owned()
                .into_dimensionality::<ndarray::Ix1>()
                .unwrap()
        })
        .collect();
    println!("{:?}", df);
}
