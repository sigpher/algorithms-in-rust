pub fn nums_sum(nums: &[i32]) -> i32 {
    let mut sum = 0;
    for num in nums {
        sum += num;
    }
    sum
}

pub fn nums_sum1(nums: &[i32]) -> i32 {
    if nums.len() == 1 {
        nums[0]
    } else {
        let first = nums[0];
        first + nums_sum1(&nums[1..])
    }
}

pub fn nums_sum2(nums: &[i32]) -> i32 {
    if nums.len() == 1 {
        nums[0]
    } else {
        let last = nums[nums.len() - 1];
        last + nums_sum2(&nums[..nums.len() - 1])
    }
}

const BASTER: [&str; 16] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F",
];

pub fn num2str_rec(num: i32, base: i32) -> String {
    if num < base {
        BASTER[num as usize].to_string()
    } else {
        num2str_rec(num / base, base) + BASTER[(num % base) as usize]
    }
}

pub fn hanoi(height: u32, src_p: &str, des_p: &str, mid_p: &str) {
    if height >= 1 {
        hanoi(height - 1, src_p, mid_p, des_p);
        println!("move disk[{height}] from {src_p} to {des_p}");
        hanoi(height - 1, mid_p, des_p, src_p);
    }
}
