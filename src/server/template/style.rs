pub(in super::super) fn style_css(buf: &mut String) {
    *buf = r":root {
        background-color:beige;
    }
    a {
        text-decoration: none;
        color: inherit;
    }

    #app {
        margin: auto 20%;
    }

    .memo-preview-block {
        border: 1px solid #777;
        border-radius: 3px;
        display: grid;
        grid-template-columns: 20% 1fr;
        margin: 10px auto;
        padding: 5px 12px;
    }
    .memo-preview-block:hover {
        background-color: rgba(0,0,0,0.3);
        backdrop-filter: blur(10px);
    }"
    .to_string();
}
