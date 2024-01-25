# rusty-ppm
A crate to provide writing and reading functionality for `.ppm` images.

The library provides:

- The `canvas` module: Contains the `Canvas` struct which holds the pixel data of the image you are working on, as well as the functions necessary to access and mutate its data.
- The `ppm_writer` module: Contains functions to write your canvas into a `.ppm` image.
- The `ppm_reader` module: Contains functions to read `.ppm` images and convert it into a `Canvas`.
- The `utils` module: Contains functions to create sample `.ppm` images. Good for testing.

## Inner workings
The `Canvas` struct is implemented as a single array as opposed to a 2D array. After running some tests, this method did allow for a small performance improvement. The `ppm_writer` module provides the `write_ppm` function, which takes a `Canvas` and writes it to a binary `.ppm` file. The `ppm_reader` module provides the `read_ppm` function, which can take either a plain text `.ppm` image, or a binary `.ppm` image and convert it into a `Canvas` to be used by the user.

The pixel data inside the `Canvas` is made up of `Vector3<u8>`, using the `cgmath` crate `Vector3` struct. In order to interact with the image, you will most likely need to import the `cgmath` crate.

The `Canvas` can give access to individual pixels through the `get` and `get_mut` functions, using rows and columns as indexes. The `iter` and `iter_mut` functions will give an iterator over the whole image, starting from the top left corner, and scanning from left to right, up to down.

## Example Benchmarks
Machine specs:
```
OS:   Pop!_OS 22.04 LTS x86_64
Host: 20QNS00Q00 ThinkPad P53
CPU:  Intel i5-9400H (8) @ 4.300GHz
GPU1: Intel CoffeLake-H GT2 [UHD Graphics 630]
GPU2: NVIDIA Quadro T1000 Mobile
```
Testing methodology: Each function was timed and ran for a total of 10 times each. Results were averaged.

#### Writing/Reading a 4k(3840 × 2160) image (8,294,400 Pixels):
- Average `read_ppm` speed (plain text `.ppm` image): **277.071ms**
- Average `read_ppm` speed (binary `.ppm` image): **35.603ms**
- Average `write_ppm` speed: **48.378ms**

#### Writing/Reading a 10,000 x 10,000 image (100,000,000 Pixels):
- Average `read_ppm` speed (plain text `.ppm` image): **3.928s** 
- Average `read_ppm` speed (binary `.ppm` image): **570.408ms**
- Average `write_ppm` speed: **655.232ms**

---

## Motivation
First encountered the `.ppm` format while reading "Raytracing in one weekend" ebook and created this crate to help me deal with it. But otherwise, I still think there's something nice about the simplicity of the format.

It also turns out that this is my first crate and I thought it would be a great way to initiate myself to publishing process.

## Notes
I haven't tested the performance of using `Vector3<u8>` as opposed to other integer type, but from my understanding, it is possible that certain CPU architecture might perform better with different types of intger, such as `u32`.

I haven't created a function to write a plain text `.ppm` image as I didn't see the need to yet. Writing and reading plain text images is much slower than binary images.

Currently, version is `0.1.0`. Once I have used it more extensively and tested it further, I will promote it to version `1.0.0`.
