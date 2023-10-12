

#[derive(Debug, Default, Clone)]
struct Vector(Vec<f64>);

#[derive(Debug, Default, Clone)]
pub struct MatrixFlex {
    inner: Vec<Vector>,
}