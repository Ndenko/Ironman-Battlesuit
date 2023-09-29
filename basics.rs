/**
    Returns the sum 1 + 2 + ... + n
    If n is less than 0, return -1
**/

pub fn gauss(n: i32) -> i32 {
    if n < 0
    {
        return -1;
    } 
    else
    {
        let mut sum = 0;
        let mut i = n.clone();
        while i > 0 
        {
            sum += i;
            i -= 1;
        }
        return sum;
    }
    
    

}

/**
    Returns the number of elements in the list that 
    are in the range [s,e]
**/
pub fn in_range(lst: &[i32], s: i32, e: i32) -> i32 {
    
    let mut num_in_range = 0;
    let mut i = 0;
    let lst_len = lst.len();

    while i < lst_len 
    {
        if (lst[i] <= e) && (lst[i] >= s)
        {
            num_in_range += 1;
            i += 1;
        }
        else 
        {
            i += 1;
        }
       
    }  
    

    return num_in_range;
}

/**
    Returns true if target is a subset of set, false otherwise

    Ex: [1,3,2] is a subset of [1,2,3,4,5]
**/
pub fn subset<T: PartialEq>(set: &[T], target: &[T]) -> bool {

    let mut in_target = 0;
    let mut i = 0;
    let mut j = 0;

    let target_len = target.len();
    let set_len = set.len();

    while i < set_len 
    {
        
        while j < target_len
        {
            /* println!("inner loop i is {}",i);
            println!("inner loop. j is {}",j); 
            println!("{}",set[i]);
            println!("{}",target[j]); */
            if set[i] == target[j]
            {
                /* println!("Added {}", target[j]); */
                in_target += 1;
                j+=1;
            }
            else
            {
                j+=1;
            }
        }
        i+=1;
        j=0;
  /*       println!("outer loop i is {}",i);
        println!("outer loop. j is {}",j); */

    }

    if in_target == target_len 
    {
        return true;
    }
    else
    {
        return false;
    }
}

/**
    Returns the mean of elements in ls. If the list is empty, return None
    It might be helpful to use the fold method of the Iterator trait
**/
pub fn mean(lst: &[f64]) -> Option<f64> {

    let mut sum: f64 = 0.0; 
    let lst_len: f64 = lst.len() as f64;

    if lst_len == 0.0 
    {
        return None;
    }
    else
    {
        for num in lst 
        {
            sum += num;
        }

    }

    

    return Some (sum/lst_len);
}

/**
    Converts a binary number to decimal, where each bit is stored in order in the array
    
    Ex: to_decimal of [1,0,1,0] returns 10
**/
pub fn to_decimal(lst: &[i32]) -> i32 {
    let mut sum:i32 = 0;
    let lst_len: usize= lst.len() as usize;
    let mut i:usize = 0;
    let mut j:usize = 1;
    while i < lst_len
    {
        sum += lst[i] * 2_i32.pow((lst_len - j) as u32);
        j+=1;
        i+=1;
    }
    return sum;
}

/**
    Decomposes an integer into its prime factors and returns them in a vector
    You can assume factorize will never be passed anything less than 2

    Ex: factorize of 36 should return [2,2,3,3] since 36 = 2 * 2 * 3 * 3
**/
pub fn factorize(n: u32) -> Vec<u32> {
    let mut i:u32= 2;
    let mut j:u32 = 0;
    let mut k:usize = 0;
    let mut factors:Vec<u32>= Vec::new();
    let mut repeated_factors: Vec<u32>= Vec::new();
    while i < n 
    {
        if n % i == 0 
        {
            let mut results:Vec<u32> = repeated_division(i, n, &mut factors);
            while j < results.len() as u32
            {
                repeated_factors.push(results[j as usize]);
                j += 1;
            }
            j = 0;
        }
        i += 1;
       /*  print!("{}",i); */
    }
    if repeated_factors.is_empty()
    {
        repeated_factors.push(n);
        return repeated_factors;
    }
    else
    {
        while k < repeated_factors.len() 
        {
            if is_prime(repeated_factors[k]) == false
            {
                repeated_factors.remove(k);
                /* we need to start over because the indexes are shifting while we remove */
                k = 0;
                 
            }
           
            k += 1;
        }
        return repeated_factors;
    }
}

pub fn is_prime(n: u32) -> bool
{
    let mut i = 1;

    while i < (n-1)
    {
        if n % (n-i) == 0 
        {
            return false;
        }
        else 
        {
            i += 1;    
        }
    }
    return true;
}
pub fn repeated_division(factor: u32, target: u32, factors: &mut Vec<u32>) -> Vec<u32>
{
    let mut factors2 = factors.clone();
    
    if target % factor == 0 
    {
        
        let mut factors3 = repeated_division(factor, target/factor, &mut factors2);
        factors3.push(factor);
        return factors3.to_vec();
    }
    else
    {
        return factors2.to_vec();
    }
    
}
/** 
    Takes all of the elements of the given slice and creates a new vector.
    The new vector takes all the elements of the original and rotates them, 
    so the first becomes the last, the second becomes first, and so on.
    
    EX: rotate [1,2,3,4] returns [2,3,4,1]
**/
pub fn rotate(lst: &[i32]) -> Vec<i32> {
    let mut lst_vec:Vec<i32> = lst.to_vec();
    if lst_vec.is_empty()
    {
        return lst_vec;
    }
    else
    {
        let first_item: i32 = lst_vec[0]; 
        lst_vec.remove(0);
        lst_vec.push(first_item);
        return lst_vec;
    }
    

}

/**
    Returns true if target is a subtring of s, false otherwise
    You should not use the contains function of the string library in your implementation
    
    Ex: "ace" is a substring of "rustacean"
**/
 pub fn substr(s: &String, target: &str) -> bool {
    let mut in_target = 0;
    let mut i = 0;
    let mut j = 0;

    let target_len = target.len();
    let s_len = s.len();

    let mut is_substr = false;

    while i < s_len 
    {  
            /* println!("inner loop i is {}",i);
            println!("inner loop. j is {}",j); 
            println!("{}",set[i]);
            println!("{}",target[j]); */         
        while (s.chars().nth(i) == target.chars().nth(j)) && (j < target_len)
        {
            /* println!("Added {}", target[j]); */
            in_target += 1;
            i+=1;
            j+=1;
            if in_target == target_len
            {
                is_substr = true;
            }
        }      
        j = 0; 
        i += 1;
        in_target = 0; 
        
  /*       println!("outer loop i is {}",i);
        println!("outer loop. j is {}",j); */
    }
    if target == ""
    {
        return true;
    }
    else
    {
        return is_substr; 
    }
        
}
 
/**
    Takes a string and returns the first longest substring of consecutive equal characters

    EX: longest_sequence of "ababbba" is Some("bbb")
    EX: longest_sequence of "aaabbb" is Some("aaa")
    EX: longest_sequence of "xyz" is Some("x")
    EX: longest_sequence of "" is None
**/
/* 
 pub fn longest_sequence(s: &str) -> Option<&str> {

    if s == ""
    {
        return None;
    }
    else
    {
        let mut i: usize = 0;
        let mut j: usize = 0;
        let s_len = s.len();
        let mut curr_longest = "".to_string();
        let mut curr_count:usize = 0;
        let mut prev_count:usize = 0;

        let mut curr_char = s.chars().nth(0).unwrap();
        curr_longest.push(curr_char);

        while i < s_len
        {
            if s.chars().nth(i).unwrap() != curr_char
            {   
                prev_count = curr_count;
                curr_char = s.chars().nth(i).unwrap();
                curr_count = 0;
            }
            if s.chars().nth(i).unwrap() == curr_char
            {   
                curr_count += 1;
            }
            if curr_count > prev_count
            {
                curr_longest = "".to_string();
                while j < curr_count
                {
                    curr_longest.push(curr_char);
                    j += 1;
                }
                j = 0;
            }
            i += 1;
        }
        let curr_longest_str = curr_longest.as_str();
        return Some(curr_longest_str);
    }
     

} */

pub fn longest_sequence(s: &str) -> Option<&str> {

    if s == ""
    {
        return None;
    }
    else
    {
        let mut i: usize = 0;
        let mut j: usize = 0;
        let s_len = s.len();
/*         let mut curr_longest = "".to_string(); */
        let mut curr_count:usize = 0;
        let mut prev_count:usize = 0;
        let mut start: usize = 0;
        let mut start_end: usize = 0;
/*         let mut end: usize = s.len(); */
        

        let mut curr_char = s.chars().nth(0).unwrap();
    /*     curr_longest.push(curr_char); */

        /* loop through to determine starting point of longest sequence */
        while i < s_len
        {
            if s.chars().nth(i).unwrap() != curr_char
            {   
                prev_count = curr_count;
                curr_char = s.chars().nth(i).unwrap();
                curr_count = 0;
            }
            if s.chars().nth(i).unwrap() == curr_char
            {   
                curr_count += 1;
            }
            if curr_count > prev_count
            {
                start = i - (curr_count-1);
                start_end = i+1;
            }
            i += 1;
        }
       
        /* loop through to determine ending point of longest sequence */
      /*   i = 0; */
       /*  while (i < start_end) 
        {
            if s.chars().nth(i).unwrap() != curr_char  
            {
                end = i + 1;
            }
            i += 1;

        } */

        return Some(&s[start..start_end]);
    }
     

} 
 
/* fn remove_ending_char<'a>(s: &'a str, p: &str) -> &'a str {
    if s.ends_with(p) {
        &s[..s.len() - p.len()]
    } else {
        s
    }
}
fn remove_starting_char<'a>(s: &'a str, p: &str) -> &'a str {
    if s.starts_with(p) {
        &s[1..s.len()]
    } else {
        s
    }
}
 */
/* fn main() 
{
    /* println!("{}",gauss(-17));
    println!("{}",in_range(&[5,6,7], 8, 9));  
    println!("{}", subset(&['a','b','c'], &['a']) ); 
    println!("{}", mean(&[2.0, 4.0, 9.0]));this wont work, do pub test
    println!("{}", to_decimal(&[1,1,1,1])); */
    /* let mut my_vec: Vec<u32> = Vec::new();
    my_vec = vec![1,2,3];
    println!("{}",my_vec[0]); */
   /*  println!("{:?}", repeated_division(3, 12, &mut my_vec)); 
    println!("{:?}", factorize(12));
     println!("{}",is_prime(6)); */
     /* println!("{:?}",rotate(&[6,7,8,5])); */
     /* let my_str = "yep";
     println!("{}",my_str.chars().nth(0).unwrap()); */
     /* println!("{}",substr(&"rustacean".to_string(), &"kcean")); */
   /*  let str = "abbbbabcc"; */
    /* let mut new_str = str.chars();
    new_str.next();
    println!("{}",new_str.as_str()); */
/*  println!("{}",remove_starting_char(str, "a"));
    println!("{}",remove_starting_char(str, "a")); */
   /*  println!("{}",&str[0..7]); */
  /*  println!("{:?}",longest_sequence(&"")); */
 /*  let mut x = 5;
  let y = x;
  println!("x:{} y:{}",x,y);
  x = 10;
  println!("x:{}, y:{}",x,y); */
/*   let mut s1 = String::from("hello");
  let   s2 = &s1;
   s1.push_str("hey"); 
  println!("{}",s2); */
  { let mut s1 = String::from("hello");
    { let s2 = &s1;
    println!("String is {} and {}",s1,s2); //ok
    s1.push_str(" world!"); //disallowed
    } 
    }

} */
