fn solution(a: i32, b: i32, c: i32) -> bool {
    a+b==c || a-b==c || a*b==c || a as f32/b as f32 == c as f32
}