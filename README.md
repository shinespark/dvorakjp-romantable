# dvorakjp-romantable
Google 日本語入力のデフォルトローマ字テーブルをベースに、[DvorakJP](http://www7.plala.or.jp/dvorakjp/)のマッピングを行ったローマ字テーブルです。

一部カスタマイズしていますが、DvorakJP([v1.0正式版](http://www7.plala.or.jp/dvorakjp/kaitei.htm))のローマ字テーブルとして利用する分には差し支えありません。

## How to use

1. [dvorakjp-romantable/dvorakjp_rx.txt](https://github.com/shinespark/dvorakjp-romantable/blob/master/dvorakjp_rx.txt) をローカルにダウンロードします。
1. Google 日本語入力のPreferences > General > Romaji table > Customize... ボタンをクリックします。
1. EditボタンからImport from file... を クリックし、dvorakjp_rx.txtを取り込み、OKをクリックします。

正常に適用されていない場合には、Google 日本語入力を再起動してください。

## 補足情報

### Google 日本語入力のデフォルトローマ字テーブルについて

一般的なローマ字テーブルに加え、以下の特徴がありました。

1. `z*`キーによる記号入力サポート
  - `zh`: `←`, `zj`: `↓`などの記号入力が可能
1. 訓令式以外のローマ字入力サポート
  - `f*`, `ch*`, `ts*`などの一般的なヘボン式ローマ字
  - `kya`: `きゃ`などの`y`を利用した拗音の他に、`twa`: `とぁ`などの`w`拗音の追加
1. `t'u`: `とぅ`などの`'`を利用した拗音が追加
  - ただし、分かりづらい上、 `twu`: `とぅ`など他の入力でも代替可能な拗音もある

Google 日本語入力のデフォルトローマ字テーブル(2015-10-11現在): [romantable_original.txt](https://github.com/shinespark/dvorak-romantable/blob/master/romantable_original.txt)

### DvorakJPのローマ字テーブルについて

左手ホームポジションに母音がすべて揃っているDvorak配列に、より日本語入力でも使いやすく拡張したローマ字テーブルです。  

1. `k*`キーを`c*`でも入力可能
  - 多様する子音をほぼ右手のみで入力可能に
1. 拗音入力時に利用する`y`キーの代わりに、`h`, または`n`キーが入力可能
  - 多様する子音をほぼ右手のみで入力可能に
1. 二重母音拡張と撥音拡張を追加
  - 連続する母音入力(左手での連続打鍵回数)を低減

詳細: [DvorakJP - 日本語入力用拡張Dvorak](http://www7.plala.or.jp/dvorakjp/dvorakjp.htm) 

### DvorakJP Prime(dvorakjp_prime.txt)について

Google 日本語入力とDvorakJPのローマ字テーブルのマージにあたって、以下の一部変更を行っています。

1. Google 日本語入力の `z*`キーによる記号の入力を、`;*`に変更
  - ざ行の入力と競合する為、Qwerty時の`z`のキーに位置する`;`に置換
1. Google 日本語入力の `ch*`, `tw*` などにも二重母音拡張を追加
  - 元のローマ字テーブルに4種の拗音がある為、`t*`のテーブルは若干多め
    - `tya`: `ちゃ`, `tha`: `てゃ`, `tsa`: `つぁ`, `twa`:`とぁ`, `tna`: `ちゃ`, etc.
1. Google 日本語入力の `t'u`: `とぅ`などの`'`を利用した拗音入力の削除
  - 分かりづらいので。入力の際は`two`: `とぅ`などを利用推奨
1. DvorakJPの`k`によるか行の二重母音拡張と拗音拡張の有効可
  - [DvorakJP](http://www7.plala.or.jp/dvorakjp/)  ではか行の入力を`c`だけに限定しているようですが、`k`でも入力可能にし、コンビネーションキーを利用した二重母音拡張と拗音拡張が利用可能
1. DvorakJP 0.2β時の`p`キーに二重母音拡張`uu`を復活
  - Google 日本語入力では連続同文字打鍵での「っ」が入れられない問題は発生しない(はず)
  - 参照: [DvorakJP - 前バージョンからの改定について](http://www7.plala.or.jp/dvorakjp/)
