# Lossy Image Compressor
URI CSC411 Arith Assignment, [Leon Hartley](https://github.com/hartleyleo), [Jacob Duhaime](https://github.com/Jacob-Duhaime)

### Information
This program is built in rust, and took approximately 30 hours to troubleshoot, and get working correctly.

We received help from in class lectures, posts on edStem, and posts in the CSC 411 TA Discord server. We also frequently referenced the rust wiki as well as the documentation for the CSC 411 image, arith, and rpegio crates.

All required aspects should be functioning properly and the program can properly compress and decompress an image (lossy).

### Goals

Our goal was to keep the bulk of our code outside of the main.rs files and instead group functions and modules by their use and dependancies to improved the abstraction and readability of our program.

For bitpack, the architecture of the program was provided to us, all we did was implement the functions and unit tests.

# Compression Methodology

First for compression, we receive input from the user and trim the image to even dimensions. Afterwards, we convert the rgb float image to component video ( Y | Pb | Pr ). We can then look at 2x2 blocks of pixels compute the average of Pb and Pr values using the index_of_chroma method in the csc411_arith library and convert the y values to a, b, c, and d values. We can perform operations on these values using our bitpack module and get the appropriate binary values for our pixels.

# Decompression Methodology

For decompression, we start with a compressed image of binary words and uses the bitpack module to convert these words to their appropriate decimal values.
However, since averaging values is an irreversible operation, the original four Pb and Pr values that were averaged during compression will be unattainable. Therefore, there will be data loss at this step of the decompression process. Our methodology for tracing back from our custom data type to the component video type will attach both the averaged Pb and Pr values as the regular “original” Pb and Pr values for each of the pixels, as well as use the Y value that was associated with it from the corresponding a, b, c, and d values. We can convert the component video image back to an rgb image and store the data in our vector. From there, we can output the image


We spend a total of ~10 hours fully analyzing the problems posed in the assignment, mostly working on the design document.

We spend ~35 hours total working on the implementation for this assignment.
