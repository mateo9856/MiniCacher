fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.len() < 3 {
        return vec![]; //if less than 3 return empty vector
    }
    let mut res = Vec::new();
    let mut nums = nums; //enable param to mutable
    nums.sort();

    for i in 0..nums.len() - 2 {
        if i > 0 && nums[i] == nums[i-1] {
            continue; //skip duplicates
        }
    }

    let mut left = i + 1;
    let mut right = nums.len() - 1;

    while left < right {
        let sum = nums[i] + nums[left] + nums[right];

        if sum == 0 {
            res.push(vec![nums[i], nums[left], nums[right]]);
            
            //skip duplicate elements
            while left < right && nums[left] == nums[left+1] {
               left += 1; 
            }

            while left < right && nums[right] == nums[right-1] {
                right -= 1;
            }

        } else if sum < 0 {
            left += 1; //if sum is less move left 1
        } else {
            right -= 1; //otherwise move right -1
        }
    }

    res //return res
}