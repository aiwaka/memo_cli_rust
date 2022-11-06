pub(in super::super) fn notfound_html(buf: &mut String) {
    *buf = r"<html>
    Not found.
    </html>"
        .to_string();
}
