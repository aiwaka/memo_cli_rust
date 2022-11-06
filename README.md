# memo cli rust

コマンドラインで簡単なメモを管理できるツールです.
マークダウン形式を想定しています.

## インストール

`cargo install memos_cli`

## 機能

```shell
Usage: memos_cli <COMMAND>

Commands:
  new     create new file
  list    display memo list in storage
  edit    edit a memo
  view    browse a memo
  remove  remove a memo from storage
  copy    copy the specified memo file to current directory
  serve   build a simple http server. default port is 8190 (it can be configured)
  info    show the information of this app
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

## httpサーバー機能

`serve`サブコマンドでローカルサーバーを立てることができます.
この状態でブラウザで指定されたポートを開くとメモ一覧を整った表示でプレビューできます.
