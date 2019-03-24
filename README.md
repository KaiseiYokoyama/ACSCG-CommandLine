# ACSCG - AmCr新歓カレンダージェネレーター

json形式のデータを書き込んだファイルから、新歓カレンダーのHTMLを生成します。

![_Users_Literature_IdeaProjects_ACSCG_calendar html](https://user-images.githubusercontent.com/8509057/54881369-6482db80-4e92-11e9-8091-75bd6d6caff2.png)

## usage

すでにRustの開発環境が整っている(cargoが使用できる)ものとします。Rustの導入については公式リファレンスを御覧ください。

```
$ git clone https://github.com/crome110/ACSCG-CommandLine
$ cd ACSCG-CommandLine/
$ cargo build
$ cd target/debug/
$ ./ACSCG [inputするファイル名] [outputするファイル名(オプション)]
```

## format of input
[input_sample.json](input_sample.json)を御覧ください。