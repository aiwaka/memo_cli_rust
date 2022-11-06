pub(in super::super) fn memo_html(buf: &mut String) {
    *buf = r"<!DOCTYPE html>
    <html lang='ja'>
      <head>
        <style>/*% style %*/</style>
        <meta charset='utf-8'>
        <title>memo cli rust</title>
      </head>
      <body>
        <div id='app'>
          <!-- view -->
        </div>
      </body>
    </html>"
        .to_string();
}
