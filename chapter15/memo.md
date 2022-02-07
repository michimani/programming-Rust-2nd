15 章 イテレータ
===

- `fold(初期値, FnMut)`
- イテレータとは `std::iter::Iterator` トレイトを実装する値

  ```rust
  trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item> {
      // ... 大量のデフォルトメソッドがある。らしい
    }
  }
  ```

- `std::iter::IntoIterator` を実装する値は **イテレート可能** であるという

  ```rust
  trait IntoIterator where Self::IntoIter: Iterator<Item=Self::Item> {
    type Item;
    type IntoIter: Iterator
    fn into_iter(self) -> Self::IntoIter;
  }
  ```

- for 式は Iterator と IntoIterator の記述を短くしたもの

  ```rust
  #[test]
  fn test_for_iter() {
      let v = vec!["a", "b", "c", "d", "e"];

      for c in &v {
          print!("{}", c)
      }

      println!();

      let mut iter = (&v).into_iter();
      while let Some(c) = iter.next() {
          print!("{}", c)
      }

      println!();
  }
  ```
  
## イテレータの作成

### iter メソッドと iter_mut メソッド

- 多くのコレクションが下記メソッドを提供している
  - `iter`: 共有参照
  - `iter_mut`: 可変参照
- ある型に対して繰り返し実行を行う方法が複数考えれらる場合は、 `iter` メソッドだけでは用途が曖昧になる
- それぞれの方法に対して個別のメソッドを実装するのが一般的
  - 文字列スライス型の `&str` には `iter` メソッドは存在せず、 `&str` 型の `s` に対して `s.bytes()` は 1 バイトずつ値を取り出し、 `s.chars()` は 1 文字ずつ値を取り出す (文字列を UTF-8 と解釈して)

### IntoIterator の実装

- for ループは `into_iter` を使っている
- **ほとんどの** コレクションは複数の `IntoIterator` を実装している
  - 以下、`Vec<String>` 型の `vector` に対しての例とする
  - 共有参照 (`&T`)
    - `(&vector).into_iter()` が返す Item は `&String` 型
  - 可変参照 (`&mut T`)
    - `(&mut vector).into_iter()` が返す Item は `&mut String` 型
  - 値そのもの (`T`)
    - `vector.into_iter()` が返す Item は `String` 型
- HashSet, BTreeSet, BinaryHeap は可変参照に対する IntoIterator を実装していない (それぞれの型が要求している不変条件に違反するため)
- HashMap, BTreeMap は、値に対する可変参照はサポートするが、キーに対しては共有参照のみサポート

### from_fn と successors

- `std::iter::from_fn` は `Option<T>` を返す関数を引数として受け取り実行し、値を生成するだけのイテレータを返す
- `std::iter::successors` は値の生成に直前の値を使いたい場合に使う
  - 例: フィボナッチ数列を作りたい場合
  
    ```rust
    #[test]
    fn test_fibonacci() {
        // successors を使う場合
        use std::iter::successors;
        let limit: usize = 10;

        let init = (0, 1);
        for s in successors(Some(init), |&prev| Some((prev.1, prev.0 + prev.1)))
            .take(limit)
            .map(|(_, n)| n)
        {
            println!("{}", &s)
        }

        // from_fn を使う場合
        fn fibonacci() -> impl Iterator<Item = usize> {
            let mut state = (0, 1);
            std::iter::from_fn(move || {
                state = (state.1, state.0 + state.1);
                Some(state.0)
            })
        }

        for n in fibonacci().take(limit).collect::<Vec<_>>() {
            println!("{}", &n)
        }
    }
    ```
    
- `from_fn` および `successors` は与える関数の記述さえ頑張れば複数のイテレータを使って書くようなものを表現できる
- ただし、計算フローが不明瞭になったり、一般的な要件をみたすためのイテレータが用意されている場合に、それらを使わないことにより目的が分かりづらくなったりする
- 一般的なパターンについては、それ用のイテレータを使った方がよい

### drain メソッド

- コレクションへの可変参照を引数として受け取り、値の所有権を消費者に引き渡すイテレータを返す
- `into_iter()` は値を消費するが、 `drain()` は可変参照を借用するのみで消費はしない
- `drain()` が返したイテレータがドロップされると、元のコレクションも空になる

## イテレータアダプタ

- １つのイテレータに対して特定の操作を行い、別のイテレータを生成するメソッド
- よく使われるのは `map()` と `filter()`
- 他にも、打ち切り/スキップ/組み合わせ/反転/結合/繰り返し などのアダプタメソッドがある
- ほとんどのアダプタは `self` を値で受け取るので、 `Self` は `Sized` である必要がある

### map と filter

- `map`: 個々の Item に対してクロージャを適用する
- 個々の Item をクロージャに値として渡す

  ```rust
  #[test]
  fn test_map() {
      let text = "alpha  \n bravo    \ncharlie".to_string();
      let v: Vec<&str> = text
          .lines() // 改行コードで区切った文字列のイテレータを生成
          .map(str::trim) // 各行に対して `str::trim` を実行するイテレータを生成
          .collect(); // 生成された値をベクタに集める

      assert_eq!(v, ["alpha", "bravo", "charlie"])
  }
  ```

- `filter`: 個々の Item のうち一部を取り除く
- 個々の Item をクロージャに共有参照で渡す

  ```rust
  #[test]
  fn test_filter() {
      let v: Vec<usize> = (0..=10)
          .filter(|n| *n % 2 == 0) // 偶数のみをフィルタするイテレータを生成
          .collect(); // 生成された値をベクタに集める

      assert_eq!(v, [0, 2, 4, 6, 8, 10])
  }
  ```

### filter_map と flat_map

- `filter_map`: クロージャに Item を渡し、変換または繰り返し処理の中からドロップする。filter と map を組み合わせたようなもの
  - `map` との違いは、返り値が `T` ではなく `Option<T>` であること
  - `Some<T>` なら `T` がイテレータの生成する値となり、`None` なら何も生成しない

    ```rust
    #[test]
    fn test_filter_map() {
        use std::str::FromStr;

        let text = "one 2 3 four five six 7 8 nine 10";
        let mut v = vec![];
        let iter = text
            .split_whitespace() // 空白で分割
            .filter_map(|n| i64::from_str(n).ok()); // i64 に変換できるものだけを値として生成する

        for num in iter {
            v.push(num)
        }

        assert_eq!(v, [2, 3, 4, 7, 8, 10])
    }
    ```

- `flat_map`: クロージャは任意の個数の Item の列を返す (`filter_map` は 1 or 0 個の Item を返す)
  - やっていることは二重ループ

    ```rust
    #[test]
    fn test_flat_map() {
        use std::collections::HashMap;

        let nums = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut divisor_map = HashMap::new();
        divisor_map.insert(1, vec![1]);
        divisor_map.insert(2, vec![1, 2]);
        divisor_map.insert(3, vec![1, 3]);
        divisor_map.insert(4, vec![1, 2, 4]);
        divisor_map.insert(5, vec![1, 5]);
        divisor_map.insert(6, vec![1, 2, 3, 6]);
        divisor_map.insert(7, vec![1, 7]);
        divisor_map.insert(8, vec![1, 2, 4, 8]);
        divisor_map.insert(9, vec![1, 3, 9]);
        divisor_map.insert(10, vec![1, 2, 5, 10]);

        let iter = nums
            .iter() // 1~10 のベクタのイテレータ
            .flat_map(|n| &divisor_map[n]); // 約数のベクタ内の値をひとつずつ生成する

        let mut v = vec![];
        for d in iter {
            v.push(*d)
        }

        assert_eq!(
            v,
            [1, 1, 2, 1, 3, 1, 2, 4, 1, 5, 1, 2, 3, 6, 1, 7, 1, 2, 4, 8, 1, 3, 9, 1, 2, 5, 10]
        )
    }
    ```