//SPDX-FileCopyrightText: 2022 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

#[derive(PartialEq, Clone, Debug)]
pub enum ControlOperator {
    Or, /* || */
    And, /* && */
    BgAnd, /* & */
    Semicolon, /* ; */
    DoubleSemicolon, /* ;; */
    SemiAnd, /* ;& */
    SemiSemiAnd, /* ;;& */
    Pipe, /* | */
    PipeAnd, /* |& */
    NewLine, /* \n */
    LeftParen, /* ( */
    RightParen, /* ) */
    NoChar,
}

#[derive(PartialEq, Debug)]
pub enum RedirectOp {
    Output, /* > */ 
    Input, /* < */
    InOut, /* <> */
    AndOutput, /* &> */ 
    OutputAnd, /* >& */ 
    Append, /* >> */ 
    HereDoc, /* << */ 
    AndAppend, /* &>> */ 
    HereStr, /* <<< */ 
    NoRedirect, 
}

/*
pub enum Reserved {
    Not, /* ! */
    Case,
    Do,
    Done,
    Elif,
    Else,
    Esac,
    Fi,
    For, 
    Function,
    If,
    In,
    Select,
    Then,
    Until,
    While,
    LeftBrace, /* { */
    RightBrace, /* } */
    Time,
    LeftDoubleBracket, /* [[ */
    RightDoubleBracket, /* ]] */
}
*/

