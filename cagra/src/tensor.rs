use crate::error::*;
use cauchy::Scalar;
use ndarray::*;

pub type Tensor<A> = ArcArray<A, IxDyn>;

pub trait IntoTensor<A> {
    fn into_tensor(self) -> Tensor<A>;
}

impl<A: Scalar> IntoTensor<A> for A {
    fn into_tensor(self) -> Tensor<A> {
        arr0(self).into_dyn().into_shared()
    }
}

impl<'a, A: Scalar> IntoTensor<A> for &'a [A] {
    fn into_tensor(self) -> Tensor<A> {
        arr1(self).into_dyn().into_shared()
    }
}

pub trait TensorCast<A> {
    fn as_scalar(&self) -> Result<A>;
}

impl<A: Scalar> TensorCast<A> for Tensor<A> {
    fn as_scalar(&self) -> Result<A> {
        if self.ndim() != 0 {
            return Err(Error::TensorRankMismatch {
                actual: self.ndim(),
                desired: 0,
            });
        }
        let t = self.to_owned().into_shape(()).unwrap();
        Ok(t.into_scalar())
    }
}
