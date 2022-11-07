# memocr

コマンドラインで簡易的にメモを管理できるツールです.

## 概要

Vimの他, 好きなエディタを用いてメモを編集できます.

メモを作成するとyaml形式のfrontmatterブロックが自動で生成され, これを含んだマークダウンとして書くことを想定しています.
httpサーバーを起動してマークダウンのプレビューを行うこともできます.

設定ファイルはデフォルトで`${HOME}/.rustmemorc`に作成されます.
デフォルトで`${HOME}/rustmemostorage`ディレクトリが作成され, そこにメモがテキストファイルで保存されます.

## インストール

`cargo install memocr`

## コマンド

```shell
A simple note management tool.
It has the ability to create, edit (using Vim), delete, list, and view contents.
You can also set up a local server and preview markdowns in a browser.

Usage: memocr <COMMAND>

Commands:
  new     Create a new file
  list    List memo in storage
  edit    Edit a memo
  view    Browse a memo
  remove  Remove a memo from storage
  grep    Searching the contents of a file with the `grep` command
  copy    Copy the specified memo file to current directory
  serve   Set up a simple local http server. Default port is 8190 (configurable)
  info    Display information about this app
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

## httpサーバー機能

`serve`サブコマンドでローカルサーバーを立てることができます.
この状態でブラウザで指定されたポートを開くとメモ一覧を整った表示でプレビューできます.
現在は起動時に自動でブラウザが開かれます.

## お願い

初めて作ったソフトウェアを公開します.
不具合等多くあると思いますが, 見つけたらissueに記載してください.
基本的にMac環境で開発しています.
