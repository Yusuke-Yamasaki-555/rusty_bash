//SPDX-FileCopyrightText: 2022 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

use crate::{ShellCore,Feeder};
use nix::unistd;
use std::ffi::CString;
use std::process;
use nix::unistd::ForkResult;
use nix::sys::wait;
use std::env;
use std::path::Path;

pub struct Command {
    text: String,
    args: Vec<String>,
    cargs: Vec<CString>,
}

impl Command {
    pub fn exec(&mut self, _core: &mut ShellCore) {
        if self.text == "exit\n" {
            process::exit(0);
        }
        if self.args[0] == "cd" && self.args.len() > 1 {  // .len():
                                                          // ベクタのメソッド、要素数を返す。
            let path = Path::new(&self.args[1]);  // self.args: CStringではなく、String型で入力された単語を収めたベクタ。最初がcdで、引数があることを判定。Path型に変換
            if env::set_current_dir(&path).is_err() {  // ディレクトリの移動を試みる。set_current_dir:
                                                       // 返り値Result型（ここではmatchを使わず、Errに対してのみ対処）
                eprintln!("Cannnot change directory");
            }
            return;
        }

        match unsafe{unistd::fork()} {  // fork: プロセスのコピーを作って、片方でexecする.返り値: Result<ForkResult> 成功や失敗を返す
                                        // unsafe{<>}: 危険なメモリ操作をするときに必要.forkはそれに該当する.
            Ok(ForkResult::Child) => {  // Ok(): 成功（子プロセスの場合）（forkの返り値）
                let err = unistd::execvp(&self.cargs[0], &self.cargs);  // execしてコマンドの実行に処理を切り替え。成功なら、子プロセスはコマンドのものになる。出力先も親からの情報で判断。
                println!("Failed to execute. {:?}", err);  // コマンドの実行が失敗するとエラーが帰ってくる。
                process::exit(127);
            },
            Ok(ForkResult::Parent { child } ) => {  // Ok():
                                                    // 成功（親プロセスの場合）Parentは構造体で、childはフィールド。
                let _ = wait::waitpid(child, None);  // childフィールドの値をchild変数で受け取る。値は子のプロセスID（＝PID）。
            },
            Err(err) => panic!("Failed to fork. {}", err),  // Err(): 失敗
        }
    }

    pub fn parse(feeder: &mut Feeder, _core: &mut ShellCore) -> Option<Command> {
        let line = feeder.consume(feeder.remaining.len());
        let args: Vec<String> = line
            .trim_end()
            .split(' ')
            .map(|w| w.to_string())
            .collect();

        let cargs: Vec<CString> = args
            .iter()
            .map(|w| CString::new(w.clone()).unwrap())
            .collect();

        if args.len() > 0 { // 1個以上の単語があればCommandのインスタンスを作成して返す
            return Some( Command {text: line, args: args, cargs: cargs} );
        }else{
            return None; // そうでなければ何も返さない
        }
    }
}
