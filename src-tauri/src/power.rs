use ndarray::parallel::prelude::*;
use ndarray::prelude::*;
use ndarray::ViewRepr;
use ndarray_linalg::Norm;
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;

fn par_mat_vec_mul(array_a: &ArrayView<f64, Ix2>, vector_x: &Array<f64, Ix1>) -> Array<f64, Ix1> {
    let mut vector_b = Array::zeros(vector_x.len());
    let a_iter = array_a.axis_chunks_iter(Axis(0), 32);
    let b_iter = vector_b.axis_chunks_iter_mut(Axis(0), 32);
    let zipped = a_iter.into_par_iter().zip(b_iter);

    zipped.for_each(|mut x| x.1.assign(&x.0.dot(vector_x)));

    return vector_b;
}

fn rs_deflate(
    array: &mut ArrayViewMut<f64, Ix2>,
    eigenvector: &ArrayView<f64, Ix1>,
    eigenvalue: f64,
) {
    let scaled_eigenvector = eigenvalue * (eigenvector.clone().to_owned());
    array
        .axis_iter_mut(Axis(0))
        .zip(eigenvector.into_iter())
        .for_each(|(mut a, b)| a.assign(&(&a - *b * &scaled_eigenvector)));
}

fn rs_argmax(vector: ArrayBase<ViewRepr<&f64>, Ix1>) -> usize {
    let mut argmax: usize = 0;
    for i in 0..vector.len() {
        if vector[i].abs() > vector[argmax].abs() {
            argmax = i;
        };
    }
    return argmax;
}

fn power_sml_mat(array_A: ArrayView<f64, Ix2>, tolerance: f64) -> (Array<f64, Ix1>, f64) {
    let mut eigenvector = Array::random(array_A.nrows(), Uniform::new(0.0, 1.0));
    let mut eigenvalue = 0.0;
    let mut old_eigenvalue = 0.0;
    let mut is_converged = false;
    let mut argmax: usize = 0;

    while !is_converged {
        /* store t vector in eignenvector */
        eigenvector = array_A.dot(&eigenvector);
        /* find argmax */
        /* with t vector == eigenvector */
        argmax = rs_argmax(ArrayView::from(&eigenvector));
        eigenvalue = eigenvector.norm_l2();
        /* divide in place to convert t vector to eigenvector */
        eigenvector /= eigenvalue;
        if (old_eigenvalue - eigenvalue).abs() / eigenvalue.abs() > tolerance {
            old_eigenvalue = eigenvalue;
        } else {
            is_converged = true;
        }
    }

    if array_A.dot(&eigenvector)[argmax] / eigenvector[argmax] < 0.0 {
        eigenvalue = eigenvalue * -1.0;
    }

    return (eigenvector, eigenvalue);
}

fn power_lrg_mat(array_A: ArrayView<f64, Ix2>, tolerance: f64) -> (Array<f64, Ix1>, f64) {
    let mut eigenvector = Array::random(array_A.nrows(), Uniform::new(0.0, 1.0));
    let mut eigenvalue = 0.0;
    let mut old_eigenvalue = 0.0;
    let mut is_converged = false;
    let mut argmax: usize = 0;

    while !is_converged {
        /* store t vector in eignenvector */
        eigenvector = par_mat_vec_mul(&array_A, &eigenvector);
        /* find argmax */
        /* with t vector == eigenvector */
        argmax = rs_argmax(ArrayView::from(&eigenvector));
        eigenvalue = eigenvector.norm_l2();
        /* divide in place to convert t vector to eigenvector */
        eigenvector /= eigenvalue;
        if (old_eigenvalue - eigenvalue).abs() / eigenvalue.abs() > tolerance {
            old_eigenvalue = eigenvalue;
        } else {
            is_converged = true;
        }
    }

    if par_mat_vec_mul(&array_A, &eigenvector)[argmax] / eigenvector[argmax] < 0.0 {
        eigenvalue = eigenvalue * -1.0;
    }

    return (eigenvector, eigenvalue);
}

pub(crate) fn rs_power(array_A: ArrayView<f64, Ix2>, tolerance: f64) -> (Array<f64, Ix1>, f64) {
    let mut eigenvalue: f64 = 0.0;
    let mut eigenvector: Array<f64, Ix1> = Array::zeros(array_A.nrows());

    if eigenvector.len() < 255 {
        (eigenvector, eigenvalue) = power_sml_mat(array_A, tolerance);
    } else {
        (eigenvector, eigenvalue) = power_lrg_mat(array_A, tolerance);
    }

    return (eigenvector, eigenvalue);
}
