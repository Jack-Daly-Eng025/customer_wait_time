pub struct Solution;

impl Solution {
    pub fn average_waiting_time(a: Vec<Vec<i32>>) -> f64 {
        let (mut tot, mut t_p, a_l) = (0,0, a.len());
        for i in 0..a_l{
            t_p = (a[i][0] as i64).max(t_p) + (a[i][1] as i64);
            tot += t_p - a[i][0] as i64;
        }
        (tot as f64)/(a_l as f64)
    }
}




fn main() {
    println!("Hello, world!");

    let wait_time_avg = Solution::average_waiting_time(vec![vec![5,2],vec![10,8],vec![6,3]]);

    println!("Average customer wait time is {}s.", wait_time_avg);

}
