fn solution(f1: String, f2: String) -> bool {
    (f1.to_lowercase() < f2.to_lowercase()) != (f1 < f2)
}