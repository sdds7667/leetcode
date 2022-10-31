

#include <iostream>
#include <vector>
using namespace std;

class Solution
{
public:
    Solution()
    {
        ios_base::sync_with_stdio(0);
        cin.tie(0);
        cout.tie(0);
    }
    double findMedianSortedArrays(vector<int> &nums1, vector<int> &nums2)
    {
        int ll = nums1.size();
        int rl = nums2.size();
        if (rl < ll)
            return findMedianSortedArrays(nums2, nums1);

        int l = 0, r = ll - 1, half = (ll + rl) / 2;
        while (true)
        {
            // cout << l << " " << r << " " << endl;
            int ml = (r >= 0) ? ((l + r) / 2) : -1;
            int mr = half - ml - 2;

            int al = (ml < 0) ? INT32_MIN : nums1[ml];
            int ar = (ml + 1 >= ll) ? INT32_MAX : nums1[ml + 1];

            int bl = (mr < 0) ? INT32_MIN : nums2[mr];
            int br = (mr + 1 >= rl) ? INT32_MAX : nums2[mr + 1];
            if (al <= br && bl <= ar)
            {
                if ((ll + rl) % 2)
                    return min(ar, br);
                else {
                    // cout << al << " " << bl << " " << ar << " " << br << endl;
                    return ( ((double)max(al, bl) + min(ar, br)) / 2.0 );
                }
            }
            else if (al > br)
            {
                r = ml - 1;
            }
            else
            {
                l = ml + 1;
            }
        }
    }
};
