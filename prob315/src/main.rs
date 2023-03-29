struct Solution;

impl Solution {
    pub fn count_smaller(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut el = vec![0; n];
        let mut el_index: Vec<usize> = (0..n).collect();
        Solution::merge_sort_and_count(&mut nums, &mut el_index, 0, n, &mut el);
        return el;
    }


    pub fn merge_sort_and_count(source: &mut Vec<i32>, source_index: &mut Vec<usize>,
                                l: usize, r: usize,
                                el: &mut Vec<i32>) {
        if r - l <= 1 {
            return;
        }

        let m = (r - l) / 2;

        Solution::merge_sort_and_count(source, source_index, l, l + m, el);
        Solution::merge_sort_and_count(source, source_index, l + m, r, el);

        let mut lii = l;
        let mut rii = l+m;
        let mut linc = 0;
        let mut i = 0;
        let mut tmp = vec![0; r - l];
        let mut tmp_index = vec![0; r - l];
        while lii < l + m && rii < r {
            if source[lii] <= source[rii] {
                tmp[i] = source[lii];
                tmp_index[i] = source_index[lii];
                el[source_index[lii]] += linc;
                lii += 1;
            } else {
                tmp[i] = source[rii];
                tmp_index[i] = source_index[rii];
                rii += 1;
                linc += 1;
            }
            i += 1;
        }

        while lii < l + m {
            tmp[i] = source[lii];
            tmp_index[i] = source_index[lii];
            el[source_index[lii]] += linc;
            i += 1;
            lii += 1;
        }

        while rii < r {
            tmp[i] = source[rii];
            tmp_index[i] = source_index[rii];
            i += 1;
            rii += 1;
        }

        i = 0;
        for j in l..r {
            source[j] = tmp[i];
            source_index[j] = tmp_index[i];
            i += 1;
        }
    }
}

fn main() {
    println!("Hello, world!");
}
