fn main() {
    let org_arr = [1, 2, 3, 4, 70];
    let sub_arr = [1, 2, 3];

    let mut is_sub = false;

    for i in 0..org_arr.len() {
        let mut eq_number = 0;

        if org_arr[i] == sub_arr[0] {
            for j in 0..sub_arr.len() {
                if org_arr[i + j] == sub_arr[j] {
                    eq_number += 1;
                }
            }
            if eq_number == sub_arr.len() {
                is_sub = true;
            }
        }
    }

    if is_sub {
        println!("Is sub array");
    } else {
        println!("Is not sub array");
    }
}
