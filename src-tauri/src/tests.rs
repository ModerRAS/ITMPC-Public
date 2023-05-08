// 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
use super::*;

#[test]
fn test_excel_reader() {
    // assert_eq!(add(1, 2), 3);
    let test = read_excel_lines("test.xls");
}

#[test]
fn test_excel_merger() {
    // 这个断言会导致测试失败。注意私有的函数也可以被测试！
    // assert_eq!(bad_add(1, 2), 3);
}
