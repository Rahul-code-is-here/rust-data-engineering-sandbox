MAP vs FILTER (Rust / General)

1. map
- Purpose: Transform each element
- Input size == Output size
- Creates NEW values
- Works well when you want to change data

Rust example:
numbers.iter().map(|x| x * x).collect::<Vec<i32>>();

Explanation:
- iter() gives &i32
- x * x produces a new i32
- No cloning needed

--------------------------------------------------

2. filter
- Purpose: Select elements based on condition
- Output size <= Input size
- Does NOT create new values
- Keeps original elements (references)

Rust example:
numbers.iter().filter(|x| *x % 2 == 0).cloned().collect::<Vec<i32>>();

Explanation:
- iter() gives &i32
- filter keeps &i32
- Vec<i32> needs owned values
- cloned() converts &i32 → i32

--------------------------------------------------

Key Difference:
- map = change data
- filter = remove data

Rule of thumb:
- If iterator returns references AND you collect owned values → use cloned() or copied()



This project demonstrates **parallel programming in Rust** using the **Rayon library**. Rayon simplifies parallelism by providing **parallel iterators** and handling **thread pool management automatically**.