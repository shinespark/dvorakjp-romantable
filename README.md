# dvorakjp-romantable
Google 日本語入力のデフォルトローマ字テーブルをベースに、[DvorakJP](http://www7.plala.or.jp/dvorakjp/)のマッピングを行ったローマ字テーブルです。
DvorakJP([v1.0正式版](http://www7.plala.or.jp/dvorakjp/kaitei.htm))のローマ字テーブルとしてご利用ください。


## How to use

1. [dvorakjp-romantable/dvorakjp_prime.txt](https://github.com/shinespark/dvorakjp-romantable/blob/master/dvorakjp_prime.txt) をローカルにダウンロードします。
1. Google 日本語入力のPreferences > General > Romaji table > Customize... ボタンをクリックします。
1. EditボタンからImport from file... を クリックし、dvorakjp_prime.txtを取り込み、OKをクリックします。

通常は上記手順で適用されますが、時折正常に適用されない場合があります。  
その際はGoogle 日本語入力を再起動してください。

## Note

### Google 日本語入力のデフォルトローマ字テーブル

一般的なローマ字テーブルに加え、以下の特徴がありました。

1. `z*`キーによる記号入力
	- `zh`: `←`, `zj`: `↓`などの記号入力をサポート
1. 訓令式以外のローマ字入力
	- `f*`, `ch*`, `ts*`など、一般的なローマ字テーブルにも存在するヘボン式ローマ字入力
	- `twa`: `とぁ`などの`w`拗音の追加
1. `t'u`: `とぅ`などの`'`を利用した拗音入力
	- ただし、判り難い上、 `twu`: `とぅ`など他の入力でも代替可能な拗音も存在

Google 日本語入力のデフォルトローマ字テーブル(2015-10-11現在): [romantable_original.txt](https://github.com/shinespark/dvorak-romantable/blob/master/romantable_original.txt)

### DvorakJP(v1.0正式版)のローマ字テーブル

左手ホームポジションに母音がすべて揃っているDvorak配列に、より日本語入力でも打鍵しやすいよう拡張されたローマ字テーブルです。

1. か行が右手側の`c*`でも入力可能
	- 多様する子音をほぼ右手のみで入力可能に
1. 拗音入力時に利用する`y`キーの代わりに、`h`, または`n`キーが入力可能
	- 多様する子音をほぼ右手のみで入力可能に
1. 二重母音拡張と撥音拡張を追加
	- 連続する母音入力(左手での連続打鍵回数)を低減

詳細(公式): [DvorakJP - 日本語入力用拡張Dvorak](http://www7.plala.or.jp/dvorakjp/dvorakjp.htm) 

### DvorakJP Prime(dvorakjp_prime.txt)

Google 日本語入力とDvorakJPのローマ字テーブルをマージしたファイルです。  
マージにあたって、以下の変更を行っています。  

1. Google 日本語入力の `z*`キーによる記号の入力を、`;*`に変更
	- ざ行の入力と競合する為、Qwerty時の`z`キーに位置する`;`に置換
1. Google 日本語入力の `t'u`: `とぅ`などの`'`を利用した拗音入力の削除
	- 判り難い為。`twu`: `とぅ`など他拗音で入力可能
1. Google 日本語入力の `ch*`, `tw*` などの拗音にもDvorakJPの二重母音拡張を追加
	- `ch;neru`: `チャンネル`, `ch'ro`: `ちゃいろ` など、拗音に対しても二重母音拡張の入力が可能
	- 元のローマ字テーブルに4種の拗音がある為、`t*`は種類が多いので注意
		- `tya`: `ちゃ`, `tha`: `てゃ`, `tsa`: `つぁ`, `twa`:`とぁ`, `tna`: `ちゃ`, etc.
1. DvorakJPの`k`によるか行の二重母音拡張と拗音拡張の有効可
	- [DvorakJP](http://www7.plala.or.jp/dvorakjp/)  ではか行の入力を`c`だけに限定しているようですが、`k`でも入力可能にし、コンビネーションキーを利用した二重母音拡張と拗音拡張が利用可能
1. DvorakJP 0.2β時の`p`キーに二重母音拡張`uu`を復活
	- Google 日本語入力では連続同文字打鍵での「っ」が入れられない問題は発生しない(はず)
	- 参照: [DvorakJP - 前バージョンからの改定について](http://www7.plala.or.jp/dvorakjp/kaitei.htm)
