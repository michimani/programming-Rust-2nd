16 章 コレクション
===

- 他言語のコレクションとの違い
  - ディープコピーを避けるためにあらゆる場所で移動と借用が起こる
  - 無効化エラー (コレクションないのデータを指すポインタを持った状態で、コレクションのサイズ変更などが発生した際の未定義動作) が起きない
    - 借用チェックによってコンパイル時に検出できる
  - `null` がない。代わりに `Option` を使う
- そのほかは大体同じなので 16.5.1 以外は飛ばしてもよい (一応見る)

## 概要

- `Vec<T>`
  - サイズの可変な配列
- `VecDeque<T>`
  - `Vec<T>` に似ているが FIFO キューとしてはこっちが適している
  - 末尾だけでなく先頭への要素の追加、削除が可能
  - そのほかの動作は少し遅い
- `BinaryHeap<T>`
  - 順序付きのキュー
  - 最大値 (=最優先) の値を取り出すことに最適化されている
- `HashMap<K, V>`
  - キーと値のペアからなるテーブル
  - キーによる検索は高速
  - エントリの順序は保証されない
- `BTreeMap<K, V>`
  - キーによるエントリの順序が保証される
  - 順序を気にする必要がなければ `HashMap<K, V>` の方が処理は高速
- `HashSet<T>`
  - 集合
  - 値の取得・削除・検索が高速
  - 順序は保証されない
- `BTreeSet<T>`
  - 順序付きの集合
  - 値の取得・削除・検索が高速
  - 順序を気にする必要がなければ `HashSet<T>` の方が処理は高速

## Vec

- 要素へのアクセスは参照を返す
- 各メソッドはとりあえず触ってみる
- ベクタおよびスライスが既にソートされている場合、 `binary_sort`, `binary_search_by` および `binary_search_by_key` によって効率的に値を検索できる。