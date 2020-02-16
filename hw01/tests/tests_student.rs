#![cfg(test)]

use hw01::problem2::mat_mult;

//
// Problem 2
//

#[test]
fn test_mat_mult() {
    let mat1 = vec![vec![12., 7., 3.], vec![4., 5., 6.], vec![7., 8., 9.]];
    let mat2 = vec![
        vec![5., 8., 1., 2.],
        vec![6., 7., 3., 0.],
        vec![4., 5., 9., 1.],
    ];

    let result = mat_mult(&mat1, &mat2);
    assert_eq!(
        result,
        vec![
            vec![114., 160., 60., 27.],
            vec![74., 97., 73., 14.],
            vec![119., 157., 112., 23.]
        ]
    );
}
