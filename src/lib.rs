pub mod ast;
pub mod error;
pub mod lambda;
pub mod sequence;
pub mod stack;
pub mod term;
pub mod token;

/*
    s, k, iはwell-formed/validである
    x, yが`well-formed/validならば, `xyもwell-formed/validである
    以上によってwell-formed/validであるものだけが`SKIである

    atom   :=  "s", "k", "i"
    apply       :=  "`"
    term        :=  atom
    term        :=  apply term term
*/

/*

    やることとしては
    * 入力(str) -> 出力(str)ができるランタイムが欲しい
    * そのようなランタイムが作れるフレームワークが欲しい

    入力 -> 抽象機械
    抽象機械 -> 出力
    なので
    必要なのは

    抽象機械
    (入力 -> 抽象機械)フロントエンド
    (抽象機械 -> ランタイム)  バックエンド

    抽象機械:   Seqence, Ast, Combinator
    ランタイム: str -> str
    フロントエンド: str -> combinator
    バックエンド: Combinator -> str
*/
