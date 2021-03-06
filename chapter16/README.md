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

## VecDeque

- Vec は追加・削除は末尾の要素に対してのみだが、 `VecDeque` では先頭に対しても要素の追加・削除が可能
- 要素の追加: `push_front`, `push_back`
- 要素の取り出し: `pop_front`, `pop_back`
- 先頭、末尾の要素へのアクセス: `front`, `back` (`first`, `last` と同じ動き)
- VecDeque は バッファ、サイズ、先頭の位置、末尾の位置 を持っているので、先頭・末尾へのアクセスは早い
- 各要素は連続したメモリ上に格納されていないため、スライスのメソッドをすべて継承できない
- ただし、下記メソッドを使用することで各要素を連続したメモリ上に配置し直すことができる (各要素をずらすコストが発生する)
  - `make_contiguous()`
- `Vec:from()` で VecDeque を Vec に変換できる (要素の移動のコストが発生する)
- 逆に `VecDeque::from()` で Vec を VecDeque に変換する

## BinaryHeap

- 最大値がキューの先頭に来るコレクション
- `peek` で最大値の参照を、 `pop` で最大値の Item を取得する
- イテレート可能だが、イテレータが生成する値は予測できない
- 順番に値を取り出したい場合、 None が返るまで `pop` で値を取得し続ける while ループにする

## HashMap と BTreeMap

- 検索用のテーブル
- HashMap のキーとなる型は Hash と Eq を実装している必要がある
- BTreeMap のキーとなる型は Ord を実装している必要がある
- 平衡二分探索木ではなく B-tree を使っている理由
  - 近代的なハードウェアでは B-tree のほうが高速
  - 探索回数は多くなるが、出たの局所性が高い = メモリアクセスが一部に集中し、キャッシュミスが少なくなる
- 作成方法いくつか
  - `HashMap::new()`, `BTreeMap::new()`
  - `iter.collect()`
  - `HashMap::with_capacity()`
- `contains_key()` : 特定のキーが存在するかどうか
- その他、 `insert`, `append`, `get`, `len` などのメソッドが用意されている

### エントリ

- `Entry`: 冗長なマップ検索を削除する役割
- `map.entry(key)`
  - key に該当する `Entry` を返す
  - 存在しない場合は空の `Entry` を返す
- `map.entry(key).or_insert(v)`
  - key に該当する `Entry` が存在しなければ、挿入
  - 挿入した、または既に存在していた値への参照を返す
- `map.entry(key).or_default(v)`
  - Default トレイトを実装している型のみ使用可能
- `map.entry(key).or_insert_with(f)`
  - デフォルト値を生成するための `f` を渡す
- `map.entry(key).and_modify(f)`
  - 既に `Entry` が存在していた場合のみ、その値をクロージャ `f` で変更する

### map に対するイテレート

- `iter`: `(K, V)` のペアを生成する
- `keys`: `K` だけを生成する
- `values`: `V` だけを生成する
- ↑それぞれ `into_XXX` とすると map を消費するイテレータを生成する
- HashMap のイテレータは順序を保証しない
- BTreeMap のイテレータは順序を保証する (キーの順番通り)

## HashSet と BTreeSet

- ある値が含まれているかどうかを高速に判定できるコレクション
- 同じ値を複数入れることはできない
- 値を取り出して使う という使い方はしない
- `insert`: 値を追加する。追加したら `true` を、既に存在している場合は `false` を返す
- `remove`: 値を削除する。削除したら `true` を、値が存在していなかった場合は `false` を返す

## ハッシュ

- HashMap のキーおよび HashSet の値とする型は `Hash` および `Eq` を実装している必要がある
- 独自の型にをそれらに対して使いたい場合は、下記メソッドを実装する
  - `eq`
  - `hash`
