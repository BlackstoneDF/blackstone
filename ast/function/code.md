```rs
pub fn root_null(num: number): number | null<text> {
    // (not in ast) Expressions cost and are not worth it
    if num < 0 { 
        return null;
    } else {
        return number.sqrt(num);
    }
}
```