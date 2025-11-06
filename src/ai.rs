use burn_tensor::backend::Backend;
use burn_tensor::Tensor;
use burn_tensor::Int;

fn example<B: Backend>() {
    let device = Default::default();

    let tensor = Tensor::<B, 2>::from_data(
        [
            [3.0, 4.9, 2.0],
            [2.0, 1.9, 3.0],
            [6.0, 1.5, 7.0],
            [3.0, 4.9, 9.0],
        ],
        &device,
    );

    // Slice the tensor to get the second and third rows:
    // [[2.0, 1.9, 3.0], [6.0, 1.5, 7.0]]
    // The resulting tensor will have dimensions [2, 3].
    let slice = tensor.clone().slice([1..3]);
    println!("{slice}");

    // Slice the tensor to get the first two rows and the first 2 columns:
    // [[3.0, 4.9], [2.0, 1.9]]
    // The resulting tensor will have dimensions [2, 2].
    let slice = tensor.clone().slice([0..2, 0..2]);
    println!("{slice}");

    // Index the tensor along the dimension 1 to get the elements 0 and 2:
    // [[3.0, 2.0], [2.0, 3.0], [6.0, 7.0], [3.0, 9.0]]
    // The resulting tensor will have dimensions [4, 2]
    let indices = Tensor::<B, 1, Int>::from_data([0, 2], &device);
    let indexed = tensor.select(1, indices);
    println!("{indexed}");
}