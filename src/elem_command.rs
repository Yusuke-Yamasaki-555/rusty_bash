//SPDX-FileCopyrightText: 2022 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

use std::process;
use crate::{ShellCore,Feeder};
use nix::unistd::execvp;  // func
use std::ffi::CString;  // type

pub struct Command {
    pub text: String,
}

impl Command {
    pub fn exec(&mut self, _core: &mut ShellCore) { //引数_coreはまだ使いません
        if self.text == "exit\n" {
            process::exit(0);
        }
        // print!("{}", self.text);  // 改行なし出力 ここで入力文字列をコマンドとして解釈すれば、shellになる
        let mut words = vec![];  // ベクタを作る
        for w in self.text.trim_end().split(' ') {  // 空白で分割。trim_end():
                                                    // 文字列の末尾の改行を除去
            words.push(CString::new(w.to_string()).unwrap());  // 型変換
                                                               // self.text: String
                                                               // w:
                                                               // "文字列のスライスの参照"の&strという型
                                                               // w.to_string(): String
                                                               // CString::new(w.to_string()).unwrap(): CString(C言語の文字列型、つまり最後がnull文字。execvp()が要求)
                                                               // words: Vec<CString>                                                               
        };

        println!("{:?}", words);  // 改行付き出力 {:?}/ ブレースホルダ{}の中に:?
                                  // (デバック用にデータを文字列で出力するための指定)
                                  // というフォーマットの指定を記述
        if words.len() > 0 {  // 要素が１個以上あるか確認
            println!("{:?}", execvp(&words[0], &words));
        }
    }

    pub fn parse(feeder: &mut Feeder, _core: &mut ShellCore) -> Option<Command> {
        let line = feeder.consume(feeder.remaining.len());
        Some( Command {text: line} )
    }
}
