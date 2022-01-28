# 5 章 参照

- `'a` ... tick A: 生存期間パラメータ
  - 任意の生存期間 'a に対して...
  - 生存期間を気にする必要があるのは、関数および型を定義するときのみ
  - 実行時には、コンパイラが生存期間として可能な限り短い期間を想定する

  ```rust
  fn g<'a>(p: &'a i32) {
      print!("{}", p)
  }

  fn main() {
      let x = 10;
      g(&x);
  }
  ```
- グローバル変数はプログラム実行全体を生存期間とする
  - 「'static 生存期間」と呼ぶ

  ```rust
  static mut STASH: &i32 = &128;
  fn f(p: &'static i32) {
      unsafe {
          STASH = p;
      }
  }

  fn main() {
      f(&32);
  }
  ```
- 返り値としての参照

  ```rust
  fn smallest(v: &[i32]) -> &i32 {
      let mut s = &v[0];

      for r in &v[1..] {
          if *r < *s {
              s = r;
          }
      }
      s
  }

  fn main() {
      let s;
      {
          let parabola = [9, 4, 1, 0, 1, 4, 9];
          s = smallest(&parabola);
          assert_eq!(*s, 0)
      }
      // assert_eq!(*s, 0) // ここでは parabola の生存期間外なので s の参照先が死んでいる
  }
  ```
- 構造体の生存期間についても同じ

  ```rust
  struct S<'a> {
      // r: &i32, // コンパイルエラー: expected named lifetime parameter
      // r: &'static i32, // 制約が強すぎる
      r: &'a i32,
  }

  // 入れ子
  struct D<'a> {
      // s: S, // expected named lifetime parameter
      // s: S<'static>, // 制約が強すぎる
      s: S<'a>,
  }
  ```

- 個別の生存期間

  ```rust
  struct XY<'a, 'b> {
      x: &'a i32,
      // y: &'a i32, // x と同じ生存期間はとれない
      y: &'b i32,
  }


  fn main {
      let x = 10;
      let r;
      {
          let y = 20;
          {
              let xy = XY { x: &x, y: &y };
              r = xy.x;
          }
      }
      println!("{}", r)
  }
  ```
  
  - 生存期間を加えていくとシグネチャが読みにくくなる
  - 問題はコンパイラが見つけてくれるので、見つかるまで待つという態度でも問題ない

- 生存期間はだいたい省略できる (コンパイラがよしなに推論してくれる)
- 明示的に指定したい場面でのみ記述すればいい
- 共有アクセスは読み出しのみのアクセス
- 可変アクセスは排他アクセス