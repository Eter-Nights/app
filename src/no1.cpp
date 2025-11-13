#include "gtest/gtest.h"

using std::vector;
using std::map;

class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
        map<int, int> res;

        for(int i = 0; i < nums.size(); i++) {
            auto it = res.find(target - nums[i]);
            if (it != res.end()) {
                return {it->second, i};
            }
            res[nums[i]] = i;
        }
        return {};
    }
};

TEST(no1, Case1) {
    vector<int> nums = {2, 7, 11, 15};
    int target = 9;
    vector<int> result = {0, 1};
    Solution s;
    EXPECT_EQ(s.twoSum(nums, target), result);
}

