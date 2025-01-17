fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        // Note to self: Not sure why it cannot know a[1..4] is of type [i32; 3] :-/
        let nice_slice = a[1..4];

        assert_eq!([2, 3, 4], nice_slice);
    }
}
