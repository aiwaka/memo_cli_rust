pub(in super::super) fn index_html(buf: &mut String) {
    *buf = r"<!DOCTYPE html>
    <html lang='ja'>
      <head>
        <style>/*% style %*/</style>
        <meta charset='utf-8'>
        <title>memo cli rust</title>
      </head>
      <body>
        <div id='app'>
          <h1>メモ一覧</h1>
          <div class='preview-container'>
            <!-- preview -->
          </div>
        </div>
      </body>
    </html>"
        .to_string();
}
