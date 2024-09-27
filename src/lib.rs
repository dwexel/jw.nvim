use nvim_oxi::{Dictionary, Function, Object};
use nvim_oxi::api::{self, opts::*, types::*, Window};
use nvim_oxi::lua::Poppable;

use std::io::BufWriter;
use std::io::Write;

use std::fs::File;
use anyhow::Result;


// 58年前、静岡県でみその会社の家族4人が殺されました。袴田巌さんはこの事件で捕まって、死刑が決まっていました。しかし、袴田さんは「殺していない」とずっと言っていました。このため、去年から裁判をやり直していました。
//
// 東京の上野動物園にいるパンダの雄の「リーリー」と雌の「シンシン」は、今年で19歳になりました。
// 年をとったため、今月29日に中国に帰ります。
// 動物園には毎日、たくさんの人たちが2頭に会いに来ています。27日も多くの人が朝から並んでいました。

const DIC: &str = r".\ipadic-mecab-2_7_0\system.dic.zst";


fn init() -> anyhow::Result<vibrato::Tokenizer> {
    let reader = zstd::Decoder::new(File::open(DIC)?)?;
    let dict = vibrato::Dictionary::read(reader)?;
    let tokenizer = vibrato::Tokenizer::new(dict);
    Ok(tokenizer)
}

#[nvim_oxi::plugin]
fn jw_nvim() -> Result<Dictionary> {

    let mut log = BufWriter::new(File::create("./log").expect("cannot create log file"));

    let result = init();
    if let Err(e) = result {
        log.write_all(format!("error opening or processing dictionary file, {}", e).as_bytes()).unwrap();
        return 
            anyhow::Result::Err(e);
    }

    //init succeeded
    log.write_all(b"initialization ok").unwrap();

    // have to tell nvim-oxi that referenced vars have a static lifetime
    use vibrato::tokenizer::Tokenizer;
    use vibrato::tokenizer::worker::Worker;

    let tokenizer = Box::new(result.unwrap());
    let tokenizer: &'static mut Tokenizer = Box::leak(tokenizer);

    let worker = Box::new(tokenizer.new_worker());
    let worker: &'static mut Worker = Box::leak(worker);

    use nvim_oxi::Array;


    // nvim-oxi 
    //
    // Object = any valid lua type
    // Array, Dictionary = helpers to make containers for Objects

    let send = Function::from_fn_mut(
        |s: String| {
            worker.reset_sentence(s.clone());
            worker.tokenize();
            // what's the actual upper bound for token indices?
            let ends = worker.token_iter().map(|tok| Object::from(tok.range_byte().end as i32));
            Array::from_iter(ends)
        }
    );


    let worker = Box::new(tokenizer.new_worker());
    let worker: &'static mut Worker = Box::leak(worker);

    let ssend = Function::from_fn_mut(
        |os: Option<String>| {
            if let Some(s) = os {

                worker.reset_sentence(s);
                worker.tokenize();
                let ends = worker.token_iter().map(|tok| Object::from(tok.range_byte().end as i32));
                Array::from_iter(ends)
            } else if let Ok(s) = nvim_oxi::api::get_current_line() {

                worker.reset_sentence(s);
                worker.tokenize();
                let ends = worker.token_iter().map(|tok| Object::from(tok.range_byte().end as i32));
                Array::from_iter(ends)
            } else {

                Array::new()
            } 
        }
    );
    //
    //use nvim_oxi::api::opts::*;
    //use nvim_oxi::api::get_current_win;
    //use nvim_oxi as nvim;
    //
    //let opts = CreateAutocmdOpts::builder()
    //    .callback(
    //            Function::from_fn(|args: AutocmdCallbackArgs| {
    //                let n = get_current_win().get_cursor().unwrap();
    //                nvim::print!("{n:?}");
    //                true 
    //            })
    //        )
    //    // desc 
    //    .build();
    //
    //if nvim_oxi::api::create_autocmd(["CursorMoved"], &opts).is_err() {
    //    log.write_all(b"failed to create autocmd");
    //}
    //

    let plugin = Dictionary::from_iter([
        ("ssend", Object::from(ssend)),
        ("send", Object::from(send)),
    ]);

    Ok(plugin)
}
