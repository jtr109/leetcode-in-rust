/*!

## 解决方案 1

直接实现一个结构体，包含 vector，在 vector 中管理 nums。

这样的方案存在的问题是每次都需要从各个索引重新计算 sum_range，会带来性能损失。

## 方案 2

缓存前 i 个数的和，这样的好处是 `sum_range` 的时候只需要将记录的和相减即可。

 */

pub struct NumArray {
    nums: Vec<i32>,
}

impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        Self { nums }
    }

    pub fn update(&mut self, index: i32, val: i32) {
        self.nums[index as usize] = val;
    }

    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.nums[left as usize..=right as usize].iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = [1, 3, 5];
        let mut obj = NumArray::new(nums.to_vec());
        assert_eq!(obj.sum_range(0, 2), 9);
        obj.update(1, 2);
        assert_eq!(obj.sum_range(0, 2), 8);
    }
}
