//use std::{array, convert::TryInto, result};
use image::{ DynamicImage, GenericImageView, ImageBuffer, ImageFormat, ImageReader, Rgb, RgbImage };

///degrees * `RADIANS` -> degrees in radians
const RADIANS: f64 = core::f64::consts::PI / 180.0;

///.unwrap() but does not need `Debug` trait implmented for Err
fn get_ok_val<T, E>(result: Result<T, E>) -> T {
    match result {
        Ok(value) => value,
        Err(_) => {
            panic!("Result is an Err in get_ok_val()")
        }
    }
}

struct Matrix {
    values: Vec<Vec<f64>>,
}

impl Matrix {
    fn _get_width(&self) -> u8 {
        u8::try_from(self.values[0].len()).unwrap()
    }

    fn _get_height(&self) -> u8 {
        u8::try_from(self.values.len()).unwrap()
    }

    ///returns (width, height) of matrix
    fn get_dimensions(&self) -> (u8, u8) {
        (u8::try_from(self.values[0].len()).unwrap(), u8::try_from(self.values.len()).unwrap())
    }

    ///returns value of the matrix at (x, y) in f64
    fn get_value<T: Into<usize>>(&self, x: T, y: T) -> f64 {
        self.values[usize::try_from(y).unwrap()][usize::try_from(x).unwrap()]
    }

    ///matrix.multiply_by_num(int) = matrix * int
    fn multply_by_num<T: Into<f64>>(&self, num: T) -> Matrix {
        let f64num = f64::try_from(num).unwrap();
        let dim = self.get_dimensions();

        let mut products = vec![Vec::with_capacity(usize::from(dim.0)); usize::from(dim.1)];

        for i in 0..dim.1 {
            for j in 0..dim.0 {
                products[usize::from(i)].push(self.get_value(j, i) * f64num);
            }
        }

        Matrix { values: products }
    }
}

fn add_matrices(a: &Matrix, b: &Matrix) -> Matrix {
    let a_dim = a.get_dimensions();
    let b_dim = b.get_dimensions();

    //panics if a_dim != b_dim
    assert_eq!(a_dim, b_dim, "Incompatable Matrix Sizes For Adding");

    let mut c_values = vec![Vec::with_capacity(usize::from(a_dim.0)); usize::from(a_dim.1)];

    for i in 0..a_dim.0 {
        for j in 0..a_dim.1 {
            c_values[usize::from(j)].push(a.get_value(i, j) + b.get_value(i, j));
        }
    }

    Matrix { values: c_values }
}

fn multiply_matrices(a: &Matrix, b: &Matrix) -> Matrix {
    let a_dim = a.get_dimensions();
    let b_dim = b.get_dimensions();

    //panics if a_dim.0 != b_dim.1
    assert_eq!(a_dim.0, b_dim.1, "Incompatable Matrix Sizes For Multipling");

    let mut c_values = vec![Vec::with_capacity(usize::from(b_dim.0)); usize::from(a_dim.1)];

    for i in 0..a_dim.1 {
        for j in 0..b_dim.0 {
            let mut c_value = 0.0;
            for k in 0..a_dim.0 {
                c_value += a.get_value(k, i) * b.get_value(j, k);
            }
            c_values[usize::from(i)].push(c_value);
        }
    }

    Matrix { values: { c_values } }
}

struct PixelData {
    position: (f64, f64),
    color: [u8; 3]
}
struct ImagePixels {
    values: Vec<Vec<PixelData>>
}

fn bytes_into_image_pixels<T>(img: &DynamicImage, img_dim_t: (T, T)) -> ImagePixels
    where u32: TryFrom<T>
{
    let image_bytes = img.as_bytes();
    let img_dim_u32 = (get_ok_val(u32::try_from(img_dim_t.0)), get_ok_val(u32::try_from(img_dim_t.1)));
    let img_dim_usize = (get_ok_val(usize::try_from(img_dim_u32.0)), get_ok_val(usize::try_from(img_dim_u32.1)));

    let mut img_pixels = ImagePixels {values: Vec::with_capacity(img_dim_usize.1)};
    for i in 0..img_dim_u32.0 {
        let mut row = Vec::with_capacity(img_dim_usize.0);
        for j in 0..img_dim_u32.1 {
            let test: [u8; 3] = (0..3).map(|addon| image_bytes[get_ok_val(usize::try_from((j * img_dim_u32.0 + i) * 3 + addon))]).collect::<Vec<u8>>().try_into().unwrap();
            
            row.push(PixelData {
                position: (f64::from(i), f64::from(j)),
                color: (0..3).map(|addon| image_bytes[get_ok_val(usize::try_from((j * img_dim_u32.0 + i) * 3 + addon))]).collect::<Vec<u8>>().try_into().unwrap()
            });
        }
        img_pixels.values.push(row);
    }
    img_pixels
}

fn main() {
    
    let matrix1 = Matrix {
        values: vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0]]
    };

    let matrix2 = Matrix {
        values: vec![
            vec![1.0, 2.0],
            vec![3.0, 4.0],
            vec![5.0, 6.0]]
    };

    let matrix3 = Matrix {
        values: vec![
            vec![1.0],
            vec![2.0]
        ]
    };

    println!("{:?}", multiply_matrices(&matrix1, &matrix2).values);

    
    let test_matrix = Matrix {
        values: vec![
            vec![1.0, 1.0]]
    };

    let theta: f64 = 180.0 * RADIANS;
    let rotational_matrix = Matrix {
        values: vec![
            vec![theta.cos(), -theta.sin()],
            vec![theta.sin(), theta.cos()]],
    };

    //println!("{:?}", multiply_matrices(&test_matrix, &rotational_matrix).values);
    println!("{:?}", test_matrix.multply_by_num(2.0).values);

    /*
    println!("{:?}", f64::sin(theta));
    println!("{:?}", multiply_matrices(&test_matrix, &rotational_matrix).values);

    println!("{:?}", matrix2.values);
    println!("{:?}", matrix2.multply_by_num(3).values);
    */

    //let image = ImageReader::open("test.png").unwrap().decode().unwrap();

    //bytes_into_image_pixels(&image, image.dimensions());
    
    //image.save_with_format("test2.png", ImageFormat::Png).unwrap();
}
