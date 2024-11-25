fn selection_sort(mut arr :Vec<i32> ) -> Vec<i32>{
    for i in 0..arr.len(){
        let mut small = i;
        for j in i..arr.len(){
            if(arr[small] > arr[j]){
                small = j;
            }
        }
        arr.swap(small,i);
    }
    arr
}

