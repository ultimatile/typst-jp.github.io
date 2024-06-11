# Typstドキュメント

非公式 Typst 日本語ドキュメント。[中国語版](https://github.com/typst-doc-cn/typst-doc-cn.github.io)よりフォーク。

## コントリ
1. repositoryをフォークします
2. `./docs/src` 内の Markdown ファイルを変更します
3. `./docs/i18n` の Yaml ファイルを変更します
4. 「サードパーティ パッケージ」ページのパッケージに対応する翻訳を追加する場合は、`./static/assets/index2cn.json` ファイルを変更できます
5. 翻訳ガイドラインを遵守します
 1. 和文と欧文の間にスペースを追加します
 2. 和文の句読点を使用するようにしてください
 3. 不明な用語については、用語集または他のページの翻訳を参照してください
 4. 例の英語を日本語に翻訳する必要はありません
6. プルリクエストを開始します
7. 必要に応じて、文書の最後に翻訳者の名前を残すこともできます

ご質問などがある場合は、[「くみはんクラブ」のDiscordサーバー](https://discord.gg/9xF7k4aAuH)に参加してご連絡ください。

もちろん、直接翻訳してプルリクエストを開始することもできます。


## 技術的な詳細
[中国語版](https://github.com/typst-doc-cn/typst-doc-cn.github.io?tab=readme-ov-file#%E6%8A%80%E6%9C%AF%E7%BB%86%E8%8A%82)参照

## ローカル生成

ローカルでの生成は必須ではありませんが、生成された Web ページが正しいかどうかをローカルで確認するのに役立ちます。

まず、このリポジトリのクローンを作成し、`cargo` ツールチェーン、Python および Python パッケージの `jinja2` と `pyyaml` をインストールする必要があります。
```
# `./docs/src` ディレクトリを変更した場合は、次の 2 行のコマンドを実行する必要があります
cargo test --package typst-docs --lib -- tests::test_docs --exact --nocapture
# `./docs/i18n` ディレクトリのみを変更した場合は、このコマンド行を実行するだけで済みます
python ./gen.py
```

最終的にコンパイルされたファイルは `./dist` にあります。

nodejs がインストールされている場合は、Web 静的サーバーをローカルですばやく起動し、`npxserve ./dist` を通じて結果をプレビューできます。
