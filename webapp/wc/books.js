/* Display CASE books */
"use strict"
webui.define("app-books", {
    connected: (t) => {
        t.innerHTML = webui.applyAppDataToContent(`
<webui-side-by-side>

<webui-flex column justify="center">

<webui-quote theme="success">

Get your copy of the book "CASE: Continous Agile Software Engineering" for more information about Continuous Agile Software Engineering and how to integrate it into your teams and workflows.

Note: These links use an affiliate tracker that allow us to earn an extra commission when you make a purchase.

</webui-quote>

</webui-flex>

<webui-flex column align="center" class="ma-2">

<webui-flex gap="2" justify="center" wrap>

<webui-paper class="pa-2" elevation="10">

<webui-link target="_blank" href="https://a.co/d/0reKAjn" class="theme-secondary pa-3">

Kindle

</webui-link>

</webui-paper>

<webui-paper class="pa-2" elevation="10">

<webui-link target="_blank" href="https://a.co/d/9Kiyse8" class="theme-tertiary pa-3">

Paperback

</webui-link>

</webui-paper>

<webui-paper class="pa-2" elevation="10">

<webui-link target="_blank" href="https://a.co/d/3da8fQc" class="theme-primary pa-3">

Hardcover

</webui-link>

</webui-paper>

</webui-flex>

</webui-flex>

</webui-side-by-side>
`);
    }
});
