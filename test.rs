pub fn sum(a: i64, b: i64) -> i64
{
    const C: i64 = 5;
    return a + b + C;
}

fn main() {
    let (a, b) = (5, 6);
    let res = sum(a,b); 
    println!("Hello, world!");
    println!();
    println!("The sum between {} and {} + 5 is {}",a, b, sum(5,6));
    println!("The sum between {} and {} + 5 is {}",a, b, sum(a,b));
    println!("The sum between {} and {} + 5 is {}",a, b, res);
    println!();
    
    let mut owned_string: String = "Olá ".to_owned();
    let borrowed_string: String = "Mário".to_owned();
    
    owned_string.push_str(&borrowed_string);
    println!("{}", owned_string);
    println!();
    
    let c = TwoNumbers{first: 5, second: 6};
    
    println!("The sum between {} and {} + 5 is {}",c.first, c.second, sum(c.first, c.second));
    println!();
    
    for n in 0..30 {
        println!("Inside index {} of the loop", n);
    }
    
    if a > b {
        println!();
        println!("{} is bigger than {}", a, b);
    }
    else
    {
        println!();
        println!("{} is bigger than {}", b, a);  
    }
    
    let array = [1,2,3];
    let array2 = [0; 3];
    
    for n in 0..array.len() {
        println!("Array 1 = {}    Array 2 = {}", array[n], array2[n]);
    }
    
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);

    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], 1);

    assert_eq!(vec.pop(), Some(2));
    assert_eq!(vec.len(), 1);

    vec[0] = 7;
    assert_eq!(vec[0], 7);

    vec.extend([1, 2, 3].iter().copied());

    for x in &vec {
        println!("{x}");
    }
    assert_eq!(vec, [7, 1, 2, 3]);
    
    /*
    The vec! macro is provided for convenient initialization:

    let mut vec1 = vec![1, 2, 3];
    vec1.push(4);
    let vec2 = Vec::from([1, 2, 3, 4]);
    assert_eq!(vec1, vec2);

    It can also initialize each element of a Vec<T> with a given value. This may be more 
    efficient than performing allocation and initialization in separate steps, especially when initializing a vector of zeros:

    let vec = vec![0; 5];
    assert_eq!(vec, [0, 0, 0, 0, 0]);

    // The following is equivalent, but potentially slower:
    let mut vec = Vec::with_capacity(5);
    vec.resize(5, 0);
    assert_eq!(vec, [0, 0, 0, 0, 0]);
    */
    
}

pub struct TwoNumbers {
    pub first: i64,
    pub second: i64
}