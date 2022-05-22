fn main() {
  // console.log("${add(1, 2)}")
  println!("{}", add(1, 2));
}

/*
function add(a: number, b: number): number {
  return a + b;
}
*/
fn add(a: i32, b: i32) -> i32 {
  a + b // 最後の行のセミコロン省略で return a + b; 相当
}

/*
vitest のインラインテスト https://vitest.dev/guide/in-source.html#setup
if (import.meta.vitest) {
  test("test_add", () => {
    expect(add(1, 2)).toBe(3);
  });
}
*/
#[test]
fn test_add() {
  assert_eq!(add(1, 2), 3);
}
