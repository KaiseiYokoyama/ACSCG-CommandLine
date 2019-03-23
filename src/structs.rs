pub mod elements {
    pub mod element {
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

        impl ToString for Element {
            fn to_string(&self) -> String {
                // 子要素のhtml
                let mut children_html = String::new();
                for child in &self.children {
                    children_html = format!("{}{}", children_html, child.to_string());
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
                               if self.text.len() == 0 { String::new() } else { self.text.clone() },
                               // 子要素のhtml
                               &children_html,
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