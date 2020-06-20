// the high value is 1 index above what we search.
pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut low: usize = 0;
    let mut high: usize = array.len();
    while low < high {
        let middle = (low + high) / 2;
        if array[middle] == key {
            return Some(middle);
        } else if array[middle] < key {
            low = middle + 1;
        } else {
            high = middle; // note
        }
    }
    None
}
