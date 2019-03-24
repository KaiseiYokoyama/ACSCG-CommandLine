pub mod web {
    /// htmlにstructを変換する実装を提供させるtrait
    pub trait ToHtml {
        /// structをhtmlに変換する
        fn to_html(&self) -> String;

        fn to_html_index_noted(&self, i: i32) -> String;
    }

    pub mod element {
        use super::ToHtml;

        pub struct Element {
            tag: String,
            id: String,
            class_list: Vec<String>,
            children: Vec<Element>,
            text: String,
            /// attribute (key,value)
            attributes: Vec<(String, String)>,
        }

        impl Element {
            /// tag名からElementを作る
            /// js -> Document.createElement(tag)
            pub fn create(tag: &str) -> Self {
                return Self { tag: tag.to_string(), id: String::new(), class_list: Vec::new(), children: Vec::new(), text: String::new(), attributes: Vec::new() };
            }

            /// Elementの子要素の最後に追加する
            /// js -> Node.appendchild(elem)
            pub fn append(&mut self, elem: Element) {
                self.children.push(elem);
            }

            /// Elementにclassを追加する
            /// js -> Element.classList.add()
            /// "class1 class2"のような追加も可能
            pub fn add_class(&mut self, class_name: &str) {
                let classes: Vec<&str> = class_name.split(' ').collect();
                for class in classes {
                    self.class_list.push(class.to_string());
                }
            }

            /// Elementのidを変更する
            pub fn set_id(&mut self, id: String) {
                self.id = id;
            }

            /// Elementにattributeを与える
            pub fn set_attribute(&mut self, key: &str, val: &str) {
                let attribute = (key.to_string(), val.to_string());
                self.attributes.push(attribute);
            }

            /// ElementにinnerTextを設定する
            pub fn set_text(&mut self, text: &String) {
                self.text = text.to_string();
            }
        }

        impl ToHtml for Element {
            fn to_html(&self) -> String {
                // 子要素のhtml
                let mut children_html = String::new();
                for child in &self.children {
                    children_html = format!("{}{}", children_html, child.to_html());
                }

                // <{tag}{id}{class}>{children}</{tag}>
                return format!("<{}{}{}{}>{}{}</{}>",
                               &self.tag,
                               // idがあれば出力
                               if self.id.len() == 0 { String::new() } else { format!(" id=\"{}\"", self.id) },
                               // classがあれば出力
                               if self.class_list.len() == 0 { String::new() } else { format!(" class=\"{}\"", &self.class_list.join(" ")) },
                               // attributeがあれば出力
                               if self.attributes.len() == 0 { String::new() } else { attributes_to_html(&self.attributes) },
                               // textがあれば出力
                               if self.text.len() == 0 { String::new() } else { format!("\n    {}", self.text.clone()) },
                               // 子要素があれば出力
                               if self.children.len() == 0 { String::new() } else { format!("{}", children_html) },
                               &self.tag);
            }

            fn to_html_index_noted(&self, i: i32) -> String {
                // インデント(1tab=4space)
                let mut indent = String::new();
                for j in 0..i {
                    indent = format!("{}{}", indent, "    ");
                }

                // 子要素なし
                if self.children.len() == 0 {
                    // {indent}<{tag}{id}{class}{attribute}>{text}</{tag}>
                    return format!("{}<{}{}{}{}>{}</{}>\n",
                                   &indent,
                                   &self.tag,
                                   // idがあれば出力
                                   if self.id.len() == 0 { String::new() } else { format!(" id=\"{}\"", self.id) },
                                   // classがあれば出力
                                   if self.class_list.len() == 0 { String::new() } else { format!(" class=\"{}\"", &self.class_list.join(" ")) },
                                   // attributeがあれば出力
                                   if self.attributes.len() == 0 { String::new() } else { attributes_to_html(&self.attributes) },
                                   // textがあれば出力
                                   if self.text.len() == 0 { String::new() } else { format!("{}", self.text.clone()) },
                                   &self.tag
                    );
                }

                // 子要素のhtml
                let mut children_html = String::new();
                for child in &self.children {
                    children_html = format!("{}{}", children_html, child.to_html_index_noted(i + 1));
                }

                // {indent}<{tag}{id}{class}{attributes}>
                // {children}{text}
                // {indent}</{tag}>
                return format!("{}<{}{}{}{}>\n{}{}{}</{}>\n",
                               &indent,
                               &self.tag,
                               // idがあれば出力
                               if self.id.len() == 0 { String::new() } else { format!(" id=\"{}\"", self.id) },
                               // classがあれば出力
                               if self.class_list.len() == 0 { String::new() } else { format!(" class=\"{}\"", &self.class_list.join(" ")) },
                               // attributeがあれば出力
                               if self.attributes.len() == 0 { String::new() } else { attributes_to_html(&self.attributes) },
                               // textがあれば出力
                               if self.text.len() == 0 { String::new() } else { format!("\n    {}", self.text.clone()) },
                               // 子要素があれば出力
                               &children_html,
                               &indent,
                               &self.tag);
            }
        }

        // attribute: Vec<(String, String)>をhtmlに用いられる形に変換
        fn attributes_to_html(attributes: &Vec<(String, String)>) -> String {
            let mut attributes_html = String::new();
            for attribute in attributes {
                let (key, val) = attribute;
                attributes_html = format!("{} {}=\"{}\"", attributes_html, key, val);
            }

            return attributes_html;
        }
    }

    pub mod css {
        use super::ToHtml;
        use num_derive::FromPrimitive;

        /// cssのセレクター(複数可)と宣言ブロックのセット
        ///
        /// elem.class {
        ///     selector: value;
        /// }
        ///
        /// -> CSS { selector: [elem.class], declaration: [(selector,value)] }
        ///
        #[derive(Debug)]
        pub struct CSS {
            /// セレクター
            selectors: Vec<String>,
            /// スタイル宣言
            declarations: Vec<(String, String)>,
        }

        impl ToHtml for CSS {
            fn to_html(&self) -> String {
                let mut selectors = self.selectors.join(" ");
                let mut declarations = String::new();
                for declaration in &self.declarations {
                    let (key, val) = declaration;
                    declarations = format!("{}  {}: {}; \n", declarations, key, val);
                }

                return format!("{}{}{}{}", selectors, " {\n", declarations, "}\n");
            }

            fn to_html_index_noted(&self, i: i32) -> String {
                return self.to_html();
            }
        }

        impl CSS {
            pub fn create(selector: &str) -> CSS {
                let mut css = CSS { selectors: Vec::new(), declarations: Vec::new() };
                let selectors: Vec<&str> = selector.split(' ').collect();
                for selector in selectors {
                    css.selectors.push(selector.to_string());
                }

                return css;
            }

            pub fn push_declaration(&mut self, key: &str, val: &str) {
                let declaration = (key.to_string(), val.to_string());
                self.declarations.push(declaration);
            }
        }

        /// markerに適用するcssのパターン列挙
        #[derive(FromPrimitive)]
        pub enum MakerCSSs {
            Pattern0 = 0,
            Pattern1,
            Pattern2,
            Pattern3,
            Pattern4,
        }

        impl MakerCSSs {
            pub fn to_csss(&self) -> Vec<CSS> {
                match self {
                    MakerCSSs::Pattern0 => {
                        // blue
                        return MakerCSSs::csss_from_colorcode(0, "#2196F3", true);
                    }
                    MakerCSSs::Pattern1 => {
                        // red
                        return MakerCSSs::csss_from_colorcode(1, "#F44336", true);
                    }
                    MakerCSSs::Pattern2 => {
                        // teal
                        return MakerCSSs::csss_from_colorcode(2, "#009688", true);
                    }
                    MakerCSSs::Pattern3 => {
                        // deep purple
                        return MakerCSSs::csss_from_colorcode(3, "#5e35b1", true);
                    }
                    MakerCSSs::Pattern4 => {
                        // orange
                        return MakerCSSs::csss_from_colorcode(4, "#fb8c00", true);
                    }
                }
            }

            pub fn csss_from_colorcode(index: u32, colorcode: &str, whitetext: bool) -> Vec<CSS> {
                let mut css = CSS::create(&format!("[event_index=\"{}\"]", index));
                css.push_declaration("background-color", &format!("{} !important", colorcode));
                if whitetext { css.push_declaration("color", "white"); }

                let mut css_marker_sample = CSS::create(&format!(".event-description [event_index=\"{}\"]", index));
                css_marker_sample.push_declaration("background-color", &format!("{} !important", colorcode));
                css_marker_sample.push_declaration("color", &format!("{} !important", colorcode));

                return vec![css, css_marker_sample];
            }
        }
    }
}

pub mod input {
    use self::event::Event;

    /// 入力ファイルをそのままstruct化したもの
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Input {
        // 該当年度
        pub year: i32,
        // カレンダーのタイトル
        pub title: String,
        // イベント一覧
        pub events: Vec<Event>,
    }

    pub mod event {
        /// イベントの名前と日程
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Event {
            // イベント名
            pub name: String,
            // 日程
            pub dates: Vec<Date>,
        }

        /// イベントの開催日程
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Date {
            pub month: u32,
            pub days: Vec<u32>,
        }
    }
}

pub mod date {
    #[derive(Debug)]
    pub enum MonthNames {
        January,
        February,
        March,
        April,
        May,
        June,
        July,
        August,
        September,
        October,
        November,
        December,
    }

    impl MonthNames {
        pub fn from_u32(i: u32) -> MonthNames {
            match i {
                0 => {
                    MonthNames::January
                }
                1 => {
                    MonthNames::February
                }
                2 => {
                    MonthNames::March
                }
                3 => {
                    MonthNames::April
                }
                4 => {
                    MonthNames::May
                }
                5 => {
                    MonthNames::June
                }
                6 => {
                    MonthNames::July
                }
                7 => {
                    MonthNames::August
                }
                8 => {
                    MonthNames::September
                }
                9 => {
                    MonthNames::October
                }
                10 => {
                    MonthNames::November
                }
                11 => {
                    MonthNames::December
                }
                _ => {
                    panic!(format!("There is no {}th month.", i))
                }
            }
        }
    }
}