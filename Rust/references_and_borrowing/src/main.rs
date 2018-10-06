fn main() {
    let v1 = vec![1,2,3];
    let v2 = vec![1,2,3];

    let (v1, v2, answer) = foo(&v1, &v2);
}

fn foo (v1: Vec<i32>, v2: Vec<i32>) -> i32 {
    //v1とv2についての作業を行う

    //所有権と関数の結果を返す
    42
}
