# imgsf

An image similarity finding utility.

## Usage

Running the utility with two image paths will use a dhash algorithm to output their hamming distance (number of differing bits) and similarity expressed as a percentage.

A hamming distance of 0 would indicate two identical images.
```sh
imgsf <PATH_TO_IMAGE> <PATH_TO_IMAGE> 

```

## References

[Dhash algorithm](https://www.hackerfactor.com/blog/?/archives/529-Kind-of-Like-That.html)
