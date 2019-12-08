use amethyst::core::math::{
    MatrixMN,
    DimName,
    DefaultAllocator,
    allocator::Allocator,
    RealField,
    Scalar,
};

pub fn limit<S, M, N>(matrix: &mut MatrixMN<S, M, N>, lim: S)
where
    S: RealField,
    M: DimName,
    N: DimName,
    DefaultAllocator: Allocator<S, M, N>
{
    let mag_squared = matrix.magnitude_squared();
    if mag_squared != S::zero()  {
        let lim_squared = lim.powi(2);
        if mag_squared > lim_squared {
            *matrix *= lim / mag_squared.sqrt();
        }
    }
}

pub fn set_magnitude<S, M, N>(matrix: &mut MatrixMN<S, M, N>, mag: S)
where
    S: RealField,
    M: DimName,
    N: DimName,
    DefaultAllocator: Allocator<S, M, N>
{
    let magnitude = matrix.magnitude();
    if magnitude != S::zero() { 
        *matrix *= mag / magnitude;
    }
}

pub fn all<S, M, N>(matrix: &mut MatrixMN<S, M, N>, v: S)
where
    S: Scalar,
    M: DimName,
    N: DimName,
    DefaultAllocator: Allocator<S, M, N>
{
    for m in 0..M::dim() {
        for n in 0..N::dim() {
            matrix[(m, n)] = v;
        }
    }
}