pub fn main() {
  // 基本型
  let _u32: u32 = 1;
  let _u64: u64 = 1;
  let _i32: i32 = -1;
  let _i64: i64 = -1;
  let _f32: f32 = 1.1;
  let _i64: f64 = 1.1111;
  let _bool: bool = true;
  // 文字列リテラルは &str 型
  let _str: &str = "hello";
  // 変形できる String 型
  let _string: String = "hello".to_string();
  // ライフタイムを管理する Box 型
  let _box: Box<i32> = Box::new(1);

  // 再代入可能な let mut
  let mut _mx = 1;
  _mx = 3;

  // 代入
  // const a = 1;
  let a: i32 = 1;

  // if 式。if 文ではないのに注意。JS に当てはめるなら三項演算子
  // let b = a > 0 ? 2 : 3;
  let b = if a > 0 {
    2
  } else {
    3
  };

  // パターンマッチ
  // if や 関数ブロックと同じく、最後に評価したセミコロンレスな式を返す
  /*
  switch (b) {
    case 2:
      console.log("2");
      break;
    case 3:
      console.log("3");
      break;
    default:
      console.log("default");
      break;
  }
  */
  match b {
    2 => println!("2"),
    3 => println!("3"),
    _ => println!("other"),
  };

  // 変数の宣言。let だけなら書き換えられないので let mut になる (Rust で const は別の意味になる)
  // let c = 0;
  let mut c = 0;

  // while ループ
  /*
    while (c < 10) {
      console.log(c);
      c++;
    }
   */
  while c < 10 {
    println!("{}", c);
    c += 1;
  };

  // 配列
  // const list: number[] = [1, 2, 3] as const;
  let list = vec![1, 2, 3];

  // 配列のイテレーション
  /*
  for (const i of list) {
    console.log(list[i]);
  }
  */
  for i in list.iter() {
    println!("{}", i);
  };

  // 構造体の宣言
  // TS の type と同じ意味ではないが、 object の宣言と合わせると同じような意味になる。
  /*
  type Struct = {
    a: number;
    b: number;
  };
  */
  struct Point {
    x: i32,
    y: i32,
  }

  // 構造体のインスタンスを作成
  // const p: Point = { x: 1, y: 2 };
  let p = Point { x: 1, y: 2 };
  // console.log(`${p.x}:${p.y}`);
  println!("{}:{}", p.x, p.y);

  // impl による構造体のメソッド定義
  /*
  impl で Point に対して関数を実装する。
  厳密に透過なわけではないが、先の Point が IPoint だったとして、
  それを実装した class がある、と解釈してもよい。 Rust に class はない。

  class Point implements IPoint {
    constructor(public x: number, public y: number) {}
    distance(other: Point) {
      return Math.sqrt(
        (this.x * other.y) ** 2 +
        (this.y * other.y) ** 2
      );
    }
  }
  */
  impl Point {
    fn new(x: i32, y: i32) -> Self {
      Self { x, y }
    }
    // 他の Point 型とのユークリッド距離を返す
    // &self は .distance を呼び出すときの this 相当
    fn distance(&self, other: &Self) -> f64 {
      ((
        (self.x - other.x).pow(2) +
        (self.y - other.y).pow(2)
      ) as f64).sqrt()
    }
  }

  // const p1 = new Point(1, 2);
  let p1 = Point::new(1, 2);
  // const p2 = new Point(3, 4);
  let p2 = Point::new(3, 4);
  // console.log(`distance p1-p2: ${p1.distance(p2)}`);
  // 詳細は後述するが & は参照型で、構造体は参照で渡すように実装することが多い
  println!("distance p1-p2: {}", p1.distance(&p2));

  // 関数宣言
  /*
  function incr(i: number): number {
    return i + 1;
  }
  */
  fn incr(i: i32) -> i32 {
    i + 1
  }
  // 関数呼び出し
  // console.assert(incr(1) === 2);
  assert!(incr(1) == 2);

  // 匿名関数
  // const handler = (i: number): number => 1 + 1;
  let handler = |i: i32| -> i32 {
    i + 1
  };
  let _ = handler(1);

  // 文字列
  // 基本的に不変な str 型と 可変な String 型がある
  // const s: string = "my-string";
  let my_str = "my-string";

  // let my_string = my_str.slice();
  let mut my_string = my_str.to_string();
  // my_string += " - appended";
  my_string += " - appended";

  // console.log(my_string);
  println!("{}", my_string); // my-string - appended

  // HashMap 型
  // const map: Map<string, number> = new Map();
  let mut map: std::collections::HashMap<&str, u32> = std::collections::HashMap::new();
  // map.set("foo", 1);
  map.insert("foo", 1);  
  // map.set("bar", 2);
  map.insert("bar", 2);
  // [...map.entries()].forEach(([key, value]) => console.log(`${key}:${value}`));
  map.into_iter().for_each(|(k, v)| println!("{}:{}", k, v));

  // HashSet
  // const set = new Set();
  let mut set: std::collections::HashSet<i32> = std::collections::HashSet::new();
  // set.add(1);
  set.insert(1);
  // set.add(2);
  set.insert(2);
  // [...set].forEach(i => console.log(i));
  set.into_iter().for_each(|v| println!("{}", v));

  // Optional
  /*
  const s1: string | number = 5;
  if (s1) {
    console.log(s1);
  } else {
    throw new Error();
  }
  */
  let s1: Option<i32> = Some(5);
  match s1 {
    Some(s) => println!("{}", s),
    None => unreachable!(),
  }
  // const s1_unwrap = 1;
  let s1_unwrap = s1.unwrap();
  println!("{}", s1_unwrap.to_string());

  // None
  /*
  const s2 = null;
  if (s2 != null) {
    throw new Error();
  } else {
    console.log("None");
  }
  */
  let s2: Option<i32> = None;
  match s2 {
    Some(_) => unreachable!(),
    None => println!("None"),
  }

  // const tmp = 1;
  // const m = tmp ?? tmp * 2;
  // console.log(m);
  let m = Some(1).map(|x| x * 2);
  println!("{:?}", m); // Some(2)

  // unwrap 構文
  fn _trying() -> Result<i32, ()> {
    let x: Result<i32, ()> = Ok(1 as i32);
    // Result 型を返す関数の中だけで使える x? オペレータ
    // unwrap に失敗すると Err(()) が return される
    let y: i32 = x?;
    Ok(y)
  }
}