extern crate chrono;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod structs;

use std::env;
use crate::structs::input::Input;

fn main() {
    // 実行ファイルのpath
    let program: String = env::args().next().unwrap();
    // 実行時引数
    let args: Vec<String> = env::args().skip(1).collect();
    // 入力ファイルへのpath
    let path = &args[0];
    // 入力ファイルの内容
    let input = std::fs::read_to_string(path).expect("ファイルを正しく読み込めませんでした");
    // struct Input化した入力ファイル
    let input: Input = serde_json::from_str(&input).expect("ファイルの内容が不正です");

    // html生成
    create_html::create(input);
}

pub mod create_html {
    use crate::structs::input::Input;
    use crate::structs::elements::element::Element;

    /// Input構造体(インプットされたファイルの中身)を受け取って、
    /// それに応じたカレンダーのhtmlを出力する
    pub fn create(input: Input) {
        // js -> document
        let mut document = Element::create("html");

        // head領域を追加
        document.append(create_head(&input));

        println!("{}", &document.to_string());
    }

    /// html:head領域を作成する
    fn create_head(input: &Input) -> Element {
        let mut head = Element::create("head");

        // title要素
        let mut title = Element::create("title");
        title.set_text(input.title.clone());

        // materialize css :css
        let mut materializecss_css = Element::create("link");
        materializecss_css.set_attribute("rel", "stylesheet");
        materializecss_css.set_attribute("href", "https://cdnjs.cloudflare.com/ajax/libs/materialize/1.0.0/css/materialize.min.css");

        // materialize css :js
        let mut materializecss_js = Element::create("script");
        materializecss_js.set_attribute("src", "https://cdnjs.cloudflare.com/ajax/libs/materialize/1.0.0/js/materialize.min.js");

        // Material icons
        let mut materialicons = Element::create("link");
        materialicons.set_attribute("href", "https://fonts.googleapis.com/icon?family=Material+Icons");
        materialicons.set_attribute("rel", "stylesheet");

        // append to head
        head.append(title);
        head.append(materializecss_css);
        head.append(materializecss_js);
        head.append(materialicons);

        return head;
    }
}
