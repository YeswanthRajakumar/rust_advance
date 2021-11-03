use std::collections::HashMap;

fn sum(v: &[i32]) -> i32 {
    let mut s = 0;
    for num in v {
        s += num;
    }
    s
}

fn mean(v: &[i32]) -> f32 {
    let total_no_of_values = v.len();
    let sum_of_all_numbers = sum(v);
    (sum_of_all_numbers) as f32 / (total_no_of_values) as f32
}

fn median(v: &mut [i32]) -> f32 {
    v.sort_unstable();
    let middle_element = v.len() / 2;
    if (v.len() as i32) % 2 == 0 {
        // println!("{:?}",&v);
        // println!("{:?}",&[v[middle_element],v[middle_element-1]]);
        mean(&[v[middle_element],v[middle_element-1]])
    } else {
        v[middle_element] as f32
    }
}

fn mode(v: &[i32]) -> i32 {
    let mut num_count: HashMap<i32, i32> = HashMap::new();
    let mut max = 0;
    for num in v {
        num_count.insert(*num, 0);
    }

    for num in v {
        num_count.insert(*num, *num_count.get(num).unwrap() + 1);
    }

    for (key, value) in num_count {
        if value >= max {
            max = key
        }
    }

    max
}


pub fn get_result(v: Vec<i32>) {
    // let mut v = vec![1,7,3,5,8,4,1,3,5,6,5,3,4,3,1,4];
    let mut v = v;
    let mean_1: f32 = mean(&v);
    println!("MEAN : {}", mean_1);

    let median:f32 = median(&mut v);
    println!("MEDIAN : {}", median);

    let mode: i32 = mode(&v);
    println!("MODE : {}", mode);


}