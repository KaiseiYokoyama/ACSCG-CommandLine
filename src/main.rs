extern crate chrono;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate num_traits;

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
    use crate::structs::date::MonthNames;
    use chrono::{NaiveDate, Duration, Weekday, Datelike};
    use num_traits::cast::FromPrimitive;

    /// Input構造体(インプットされたファイルの中身)を受け取って、
    /// それに応じたカレンダーのhtmlを出力する
    pub fn create(input: Input) {
        // js -> document
        let mut document = Element::create("html");

        // head領域を追加
        let head = create_head(&input);
        document.append(head);

        // body領域を追加
        let mut body = create_body(&input);
        document.append(body);

        println!("{}", &document.to_string());
    }

    /// html::head領域を作成する
    fn create_head(input: &Input) -> Element {
        let mut head = Element::create("head");

        // title要素
        let mut title = Element::create("title");
        title.set_text(&input.title);

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

        // todo remove
        let mut custom = Element::create("link");
        custom.set_attribute("rel", "stylesheet");
        custom.set_attribute("href", "custom.css");

        // append to head
        head.append(title);
        head.append(materializecss_css);
        head.append(materializecss_js);
        head.append(materialicons);
        head.append(custom);

        return head;
    }

    /// html::body領域を作成する
    fn create_body(input: &Input) -> Element {
        let mut body = Element::create("body");

        // nav領域を追加
        let nav = create_nav(input);
        body.append(nav);

        // main領域を追加
        let mut main = create_main(input);
        body.append(main);

        return body;
    }

    /// html::body::nav領域を作成する
    fn create_nav(input: &Input) -> Element {
        // navigation barのタイトル(ページ最上中央)
        let mut a = Element::create("a");
        a.add_class("brand-logo center");
        a.set_text(&input.title);

        // タイトルのwrapper
        let mut div = Element::create("div");
        div.add_class("nav-wrapper");
        div.append(a);

        // nav本体
        let mut nav = Element::create("nav");
        nav.append(div);

        return nav;
    }

    /// html::body::main領域を作成する
    fn create_main(input: &Input) -> Element {
        let mut main = Element::create("main");

        // header領域を追加
        let header = create_header(input);
        main.append(header);

        // calendar領域を追加
        let calendar = create_calendar(input);
        main.append(calendar);

        return main;
    }

    /// html::body::main::header領域を作成する
    fn create_header(input: &Input) -> Element {
        let mut header = Element::create("header");
        let mut div = Element::create("div");
        div.add_class("event-description");

        // イベントの説明一覧
        let mut ul = Element::create("ul");
        ul.add_class("collection");

        for i in 0..input.events.len() {
            // イベントの説明
            let mut li = Element::create("li");
            // カレンダー中でイベントを示すマーカーのサンプル
            let mut span_marker = Element::create("span");
            span_marker.set_text(&"  ".to_string());
            span_marker.set_attribute("event_index", &format!("{}", i));
            // イベントの名前
            let mut span_description = Element::create("span");
            span_description.set_text(&input.events[i].name);
            span_description.add_class("description");
            // liにspanをそれぞれ追加
            li.append(span_marker);
            li.append(span_description);
            // ulにliを追加
            ul.append(li);
        }

        // divに追加
        div.append(ul);
        // headerに追加
        header.append(div);

        return header;
    }

    /// html::body::main::calendars領域を作成する
    fn create_calendar(input: &Input) -> Element {
        let mut calendars = Element::create("div");
        calendars.add_class("calendars");

        let schedule = calc_calendar(input);

        println!("Schedule: {:?}", schedule);

        // scheduleを月ごとに分ける
        let mut schedules_monthly: Vec<Vec<(NaiveDate, Option<i32>)>> = Vec::new();
        let mut m = 0;
        let mut index = 0;
        for sch in schedule {
            let (day, event) = sch;
            if m == 0 {
                m = day.month();
                schedules_monthly.push(Vec::new());
            } else if m != day.month() {
                index += 1;
                m = day.month();
                schedules_monthly.push(Vec::new());
            }
            schedules_monthly[index].push((day, event));
        }

        // scheduleを月ごとに処理する
        for schedule_monthly in schedules_monthly {
            let mut calendar = Element::create("div");
            calendar.add_class("calendar");

            // 月の名前を取得
            let (ref first_day, _) = schedule_monthly[0];
            let month_name = format!("{:?}", MonthNames::from_u32(first_day.month0()));

            // calendar-title領域を追加
            let title = create_calendar_title(month_name, input.year);
//            println!("{}", &title.to_string());
            calendar.append(title);

            let table = create_calendar_table(&schedule_monthly);
            calendar.append(table);

            // 格納
            calendars.append(calendar);
        }

        return calendars;
    }

    /// html::body::main::calendars::calendar::calendar-title領域を作成する
    fn create_calendar_title(month: String, year: i32) -> Element {
        let mut title = Element::create("div");
        title.add_class("calendar-title row");

        // calendarをhtmlに変換
        {
            // icon left
            {
                // wrapper
                let mut div = Element::create("div");
                div.add_class("col s2");

                // icon
                let mut i = Element::create("i");
                i.add_class("material-icons");
                i.set_text(&"navigate_before".to_string());

                // wrapperにiconを追加
                div.append(i);

                // iconのwrapperをtitleに追加
                title.append(div);
            }

            // title本体
            {
                // wrapper
                let mut div = Element::create("div");
                div.add_class("col s8 center-align date");

                let mut span_month = Element::create("span");
                span_month.add_class("month");
                span_month.set_text(&month);

                let br = Element::create("br");

                let mut span_year = Element::create("span");
                span_year.add_class("year");
                span_year.set_text(&format!("{}", year));

                div.append(span_month);
                div.append(br);
                div.append(span_year);

                title.append(div);
            }

            // icon right
            {
                // wrapper
                let mut div = Element::create("div");
                div.add_class("col s2");

                // icon
                let mut i = Element::create("i");
                i.add_class("material-icons");
                i.set_text(&"navigate_next".to_string());

                // wrapperにiconを追加
                div.append(i);

                // iconのwrapperをtitleに追加
                title.append(div);
            }
        }

        return title;
    }

    /// html::body::main::calendars::calendar::table領域を作成する
    fn create_calendar_table(schedule_monthly: &Vec<(NaiveDate, Option<i32>)>) -> Element {
        let mut table = Element::create("table");
        table.add_class("calendar-body");

        // table headを整備する
        {
            let mut thead = Element::create("thead");
            let mut tr = Element::create("tr");
            // 曜日を日曜日からthに入れていく
            for i in 0..7 {
                let mut th = Element::create("th");

                th.set_text(&format!("{:?}.", Weekday::from_i32((i + 6) % 7).unwrap()));

                th.add_class("center-align");
                // 日曜は赤、土曜は青
                if i == 0 { th.add_class("red-text"); }
                if i == 6 { th.add_class("blue-text"); }

                // 列に追加
                tr.append(th);
            }
            // theadにtrを格納
            thead.append(tr);
            // tableにtheadを格納
            table.append(thead);
        }

        // table bodyを整備する
        // カレンダーのマス目の左上から埋めていく
        {
            let mut tbody = Element::create("tbody");
            // scheduleのindex
            let mut index = 0;
            // 行
            for i in 0..5 {
                let mut tr = Element::create("tr");
                // 列
                for j in 0..7 {
                    let mut td = Element::create("td");
                    td.add_class("center-align");
                    // 日曜は赤、土曜は青
                    if j == 0 { td.add_class("red-text"); }
                    if j == 6 { td.add_class("blue-text"); }

                    // 日付の出力を開始する?
                    if index == schedule_monthly.len() {
                        // schedule_monthly[index]がOutBoundsOfIndexになるのを防ぐ
                    } else {
                        let (ref day, _) = schedule_monthly[index];
                        let weekday = day.weekday();

                        if Weekday::from_i32((j + 6) % 7).unwrap() == weekday {
                            // 日付を出力する
                            td.set_text(&format!("{}", index + 1));
                            index += 1;
                        } else {
                            // 何もしない
                        }
                    }

                    // trにtdを格納
                    tr.append(td);
                }
                // tbodyにtrを格納
                tbody.append(tr);
            }
            table.append(tbody);
        }

        return table;
    }

    /// コンピュータ上にカレンダーを再現する
    fn calc_calendar(input: &Input) -> Vec<(NaiveDate, Option<i32>)> {
        // 何月から何月までのcalendarを作成する必要があるのかを探る
        let mut min_month = 12;
        let mut max_month = 1;

        for event in &input.events {
            for date in &event.dates {
                let month = date.month;
                if month > max_month { max_month = month; }
                if month < min_month { min_month = month; }
            }
        }

        // 必要な月を出力
        let mut day = NaiveDate::from_ymd(input.year, min_month, 1);
        let the_day_after_last_day = NaiveDate::from_ymd(input.year, max_month + 1, 1);
        // (日時,イベントid)
        let mut schedules: Vec<(NaiveDate, Option<i32>)> = Vec::new();

        // カレンダーに出力されるdayをvecにしまっておく
        while day != the_day_after_last_day {
            schedules.push((day.clone(), None));
            day = day.succ();
        }

        // イベントとcalendar_vec内のNativeDateを紐付ける
        // iはイベントindex
        for i in 0..input.events.len() {
            let event = &input.events[i];

            // イベント開催日の配列
            let mut event_dates: Vec<NaiveDate> = Vec::new();
            for j in 0..event.dates.len() {
                let date = &event.dates[j];
                for k in 0..date.days.len() {
                    let day = date.days[k];
                    event_dates.push(NaiveDate::from_ymd(input.year, date.month, day));
                }
            }

            // 総当たりでイベント開催日とカレンダーをマッチング
            for j in 0..schedules.len() {
                let (day, _) = schedules[j];
                for k in 0..event_dates.len() {
                    // match
                    if day == event_dates[k] {
                        schedules.remove(j);
                        schedules.insert(j, (event_dates[k], Some(i as i32)));
                    }
                }
            }
        }

        return schedules;
    }
}
