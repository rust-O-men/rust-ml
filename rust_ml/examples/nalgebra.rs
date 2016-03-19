extern crate nalgebra;

use nalgebra::{ DVec, DMat };

fn main() {
	println!("Hello, nalgebra!");
	let vec1 = DVec::from_slice(10, &vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 0.0]);
	println!("{:?}", vec1);
	let vec2 = DVec::from_slice(10, &vec![0.1, 9.1, 8.1, 7.1, 6.1, 5.1, 4.1, 3.1, 2.1, 1.1]);
	println!("{:?}", vec2);
	let dot = nalgebra::dot(&vec1, &vec2);
	println!("{}", dot);
	let vec3 = vec2 * 10.0;
	println!("{:?}", vec3);
	let mat1 = DMat::from_row_vec(3, 3, &vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
	println!("{:?}", mat1);
	let mat2 = DMat::from_row_vec(3, 3, &vec![9, 8, 7, 6, 5, 4, 3, 2, 1]);
	println!("{:?}", mat2);
	let mat3 = mat1 * mat2;
	println!("{:?}", mat3);
	let mat4 = nalgebra::transpose(&mat3);
	println!("{:?}", mat4);
	let mat5 = mat3 * 10;
	println!("{:?}", mat5);
}