#include "gtest/gtest.h"

using std::vector;

class Solution {
public:
    void moveZeroes(vector<int> &nums) {
        int left = 0, right = 0;
        while (right < nums.size()) {
            if (nums[right] != 0) {
                std::swap(nums[left], nums[right]);
                ++left;
            }
            ++right;
        }
    }
};

TEST(no283, Case1) {
    vector<int> nums = {0, 1, 0, 3, 12};
    vector<int> result = {1, 3, 12, 0, 0};
    Solution s;
    s.moveZeroes(nums);
    EXPECT_EQ(nums, result);
}
