/*leetcode account

https://leetcode.com/TonyCoc/

 */

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        
        if x < 0 {
            return false
        }
        else if x == 0 {
            return true
        }
        else{
            let mut rev_of_num = Vec::with_capacity(x.clone().to_string().chars().count());


            for (_,item) in x.to_string().chars().enumerate(){
                rev_of_num.push(item);
            };

            let len = x.clone().to_string().chars().count();

            if rev_of_num[len-1] == '0' {
                return false
            }else{
                rev_of_num.reverse();

                let mut rev_str = String::new();

                for i in rev_of_num{
                    rev_str.push(i)
                };

                if rev_str == x.to_string() {
                    return true
                }else{
                    return false
                };
            }

        }
    }   
}