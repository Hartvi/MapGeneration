// use rand::Rng;
use std::vec::Vec;
// use rand::rngs::ThreadRng;
use std::iter::repeat_with;
extern crate ndarray;
use ndarray::{Array, Array2, Axis, arr2};
use ndarray::s;
use ndarray::prelude::*;
use std::time::Instant;
use rayon::prelude::*;

pub struct ArrayUtility;

impl ArrayUtility {
    pub fn generate_noise(h: usize, w: usize) -> Vec<u8> {
        // let mut rng = rand::thread_rng();
        // let mut image = vec![0u8; w * h * 3];
        // rng.fill(image.as_mut_slice());

        let mut rng = fastrand::Rng::new();
        let image: Vec<u8> = repeat_with(|| {if rng.bool() {255} else {0}}).take(h*w).collect();

        image
    }
    fn flatten<T: Clone>(data: &Vec<Vec<T>>) -> Vec<T> {
        data.iter().flat_map(|row: &Vec<T>| row.iter().cloned()).collect()
    }
    pub fn normalize_rows(mut arr: ndarray::Array2<f32>) -> ndarray::Array2<f32> {
        let row_sums = arr.sum_axis(Axis(1));
        for (mut row, &row_sum) in arr.axis_iter_mut(Axis(0)).zip(&row_sums) {
            row /= row_sum;
        }
        arr
    }

    pub fn create_array(
        size: (usize, usize), 
        conditional_probabilities: ndarray::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::Dim<[usize; 2]>>,
    ) -> Array2<i32>{
        
        // Create a new array with size incremented by 1 in each dimension
        let mut padded_conditional_probabilities: Array2<f32> = Array2::zeros((
            conditional_probabilities.shape()[0] + 1,
            conditional_probabilities.shape()[1] + 1,
        ));
        // Copy the original array into the new one, starting from the 1st row and column
        padded_conditional_probabilities.slice_mut(s![1.., 1..]).assign(&conditional_probabilities);

        // Creating an array filled with 0
        let mut landscape: Array2<i32> = Array2::zeros(size);
        let mut mask: Array2<bool> = Array2::from_elem(size, false);
    
        let mut rng = fastrand::Rng::new();
        let arr: Array2<f32> = Array2::from_shape_fn(size, |_| rng.f32());
        /*
        let noise: Vec<f32> = repeat_with(|| rng.f32()).take(size.0*size.1).collect();
        let arr2: ndarray::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::Dim<[usize; 2]>> = Array::from_shape_vec(size, noise).unwrap();
        // */
        // Set top and bottom borders to true
        mask.slice_mut(s![0, ..]).fill(true);
        mask.slice_mut(s![size.0 - 1, ..]).fill(true);
        padded_conditional_probabilities.slice_mut(s![0, 1..]).fill(1./(conditional_probabilities.shape()[0] as f32));

        // Set left and right borders to true
        mask.slice_mut(s![.., 0]).fill(true);
        mask.slice_mut(s![.., size.1 - 1]).fill(true);
        let number_of_conditions: usize = padded_conditional_probabilities.shape()[0];
        let start_time = Instant::now();
        for i in 1..size.0-1 {
            for k in 1..size.1-1 {
                // (1..size.1-1).into_par_iter().for_each(|k| {
                    let mut prob_arr = Array::<f32, _>::zeros(number_of_conditions);
                    // (0, -1), left neighbour, , (0, 1) right neighbour
                    for &(j, l) in &[(-1, -1), (-1, 0), (-1, 1)] {
                        let neighbor_i: usize = (i as isize + j) as usize;
                        let neighbor_k: usize = (k as isize + l) as usize;
                        // has to have dim=1 specified, otherwise the compiler complains
                        let row: ArrayBase<ndarray::OwnedRepr<f32>, Dim<[usize; 1]>> = padded_conditional_probabilities.slice(s![landscape[[neighbor_i, neighbor_k]], ..]).to_owned();
                        if mask[[neighbor_i, neighbor_k]] {
                            for m in 1..number_of_conditions {
                                prob_arr[m] += row[m];
                            }
                        }
                    }
                    // Calculate cumulative sum
                    let mut cum_sum: ArrayBase<ndarray::OwnedRepr<f32>, Dim<[usize; 1]>> = prob_arr.clone();
                    let mut acc: f32 = 0.0;
                    for elem in cum_sum.iter_mut() {
                        acc += *elem;
                        *elem = acc;
                    }

                    // Normalize the array
                    let total_sum: f32 = *cum_sum.last().unwrap();
                    let prob_val: f32 = arr[[i,k]]*total_sum;  // instead of division in vector normalization
                    let index = cum_sum.iter().position(|&x| prob_val <= x).unwrap_or(0);
                    landscape[[i,k]] = index as i32;
                    mask[[i,k]] = true;
                // });
            }
        }
        println!("Time elapsed: {:?} seconds", start_time.elapsed());
        // landscape.mapv_inplace(|x| x*4);
        landscape
    }

    pub fn land_to_colours(landscape: Array2<i32>, colours: Array2<u8>) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::new();
    
        // Flatten the landscape array
        let flattened_landscape: Vec<i32> = landscape.iter().cloned().collect::<Vec<i32>>();
    
        // Map the landscape values to colours
        for &land in &flattened_landscape {
            // Get the corresponding colour for this landscape value
            let colour: Array1<u8> = colours.row(land as usize).to_owned();
    
            // Flatten the colour and append it to the output
            output.extend_from_slice(colour.as_slice().unwrap());
        }
    
        output
    }
    
    
    pub fn rgb_arr_to_u32(arr: Array2<i32>) -> Vec<u32> {
        arr.map_axis(ndarray::Axis(1), |rgb| {
            let r = (rgb[0] as u32) << 16;
            let g = (rgb[1] as u32) << 8;
            let b = rgb[2] as u32;
            r | g | b
        }).to_vec()
    }
    pub fn i_to_colour(num: i32, colour_arr: &[u32]) -> u32 {
        let col: u32 = colour_arr[(num-1) as usize];
        col
    }
    pub fn map_to_bytes(window_size: (usize, usize), landscape: Array2<i32>) -> Vec<u8> {
        let landscape_flat = landscape.into_shape((window_size.0 * window_size.1,)).unwrap();
        let flat_image_data: Vec<u8> = landscape_flat
            .iter()
            .flat_map(|&rgb| {
                let r = (rgb >> 16 & 0xFF) as u8;
                let g = (rgb >> 8 & 0xFF) as u8;
                let b = (rgb & 0xFF) as u8;
                vec![r, g, b]
            })
            .collect();
        flat_image_data
    }

    // pub fn generate_image(w: usize, h: usize) -> Vec<Vec<Vec<u8>>> {
    //     let mut rng = rand::thread_rng();
        

    //     let mut image = Vec::with_capacity(w);
    //     for _ in 0..w {
    //         let mut row = Vec::with_capacity(h);
    //         for _ in 0..h {
    //             row.push(vec![rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255)]);
    //         }
    //         image.push(row);
    //     }

    //     image
    // }
}
