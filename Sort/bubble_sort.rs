fn bubble_sort(mut arr : Vec<i32>) -> Vec<i32>{
    for i in 0..arr.len() {
        for j in 0..arr.len()-1-i {
            if(arr[j] > arr[j+1]){
                arr.swap(j,j+1);
            }
        }
    }
    arr
}

fn main() {
    let mut arr = vec![5,4,3,2,1];
    arr = bubble_sort(arr);
    println!("{:?}",arr);
}