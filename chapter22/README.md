22 章 unsafe なコード
===

- unsafe ブロック内でできること
  - unsafe 関数を呼び出すことができる
  - raw ポインタを参照解決できる
  - 共用体 (union) のフィールドにアクセスできる
  - 可変 static 変数にアクセスできる
  - 外部インターフェースを通じて関数や変数にアクセスできる
- unsafe 関数は unsafe ブロック内からしか実行できない
  - コンパイルは通るが未定義動作が発生する可能性がある関数は、 unsafe 関数にしなければならない
  - 関数内で unsafe なブロックを使っているかどうかは関係なく、その関数を使う上での契約があるかどうかで unsafe にするかどうかを決める
- unsafe ブロックが未定義動作を起こすかどうかは、ブロック内だけでなくブロックに対して値を提供するコードにも依存する
- unsafe を使う上での契約を破った場合、その結果は unsafe ブロック内で起きないことが多い

## raw ポインタ

- raw ポインタの型は下記の二種類
  - `*mut T` : 参照先の変更を許す
  - `*const T` : 参照先の読み出しのみを許す
- raw ポインタは、他の参照や `Box` と違って `null` になり得る
- ポインタ自体の演算には `offset` や `wrapping_offset` を使う