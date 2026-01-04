```rust
fn main() {
    // 定义一个函数来计算两个数的和
    fn sum(a: i32, b: i32) -> i32 {
        a + b
}

    // 定义一个函数来计算两个数的差
    fn difference(a: i32, b: i32) -> i32 {
              a - b
    }

    // 定义一个函数来计算两个数的乘积
    fn product(a: i32, b: i32) -> i32 {
              a * b
    }

    // 定义一个函数来计算两个数的商
    fn quotient(a: i32, b: i32) -> i32 {
              a / b
    }

    // 定义一个函数来计算两个数的余数
    fn remainder(a: i32, b: i32) -> i32 {
              a % b
    }

    // 使用定义的函数来计算并打印结果
    println!("Sum: {}", sum(10, 5));
    println!("Difference: {}", difference(10, 5));
    println!("Product: {}", product(10, 5));
    println!("Quotient: {}", quotient(10, 5));
    println!("Remainder: {}", remainder(10, 5));
}
```
