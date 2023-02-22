fn merge_sort(items: Vec<i32>) -> Vec<i32>{
    let mut v = items.clone();
    if v.len() <= 1 {
        v
    } else {
        let v_size: usize = v.len();
        let (left, right) = v.split_at_mut(v_size/2);
        let (sorted_left, sorted_right) = (merge_sort(left.to_vec()), merge_sort(right.to_vec()));
        let mut left_inx: usize = 0;
        let mut right_inx: usize = 0;
        let mut output = Vec::new();
        while left_inx < sorted_left.len() || right_inx < sorted_right.len(){
            let take_left = (left_inx < sorted_left.len(), right_inx < sorted_right.len());
            let result = match take_left {
                (true, false) => true,
                (false, true) => false,
                (true, true) => sorted_left[left_inx] < sorted_right[right_inx],
                _ => false,
            };
            if result {
                output.push(sorted_left[left_inx]);
                left_inx += 1;
            } else {
                output.push(sorted_right[right_inx]);
                right_inx += 1;
            }
        }
        output
    }

}
fn main() {

    let v = vec![1, 0, 5, 7, 4, 9, 8, 11, 2, 82];
    let new_vec = merge_sort(v);
    println!("{:?}", new_vec);
}
