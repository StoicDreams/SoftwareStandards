/* Display CASE books */
"use strict"
{
    function toCamel(property) {
        return property.replace(/(-[A-Za-z0-9]{1})/g, a => { return a[1].toUpperCase(); });
    }
    class Books extends HTMLElement {
        constructor() {
            super();
            const t = this;
            if (t.parentNode && t.parentNode.nodeName === 'P') {
                let p = t.parentNode;
                t.parentNode.parentNode.insertBefore(t, t.parentNode);
                if (p.innerHTML.trim() === '') {
                    p.remove();
                }
            }
        }
        static get observedAttributes() {
            return [];
        }
        attributeChangedCallback(property, oldValue, newValue) {
            property = toCamel(property);
            if (oldValue === newValue) return;
            if (newValue === null || newValue === undefined) {
                delete this[property];
            } else {
                this[property] = newValue;
            }
        }
        connectedCallback() {
            this.innerHTML = webuiApplyAppData(`
<webui-side-by-side>

<webui-flex column justify="center">

<webui-quote theme="success">

Get your copy of my book "CASE: Continous Agile Software Engineering" for more information about Continuous Agile Software Engineering and how to integrate it into your teams and workflows.

Note: These links use an affiliate tracker that allow us to earn an extra commission when you make a purchase.

</webui-quote>

</webui-flex>

<webui-flex column align="center" class="ma-2">

<webui-flex gap="2" justify="center" wrap>

<webui-paper class="pa-2" elevation="10">

<webui-link target="_blank" href="https://amzn.to/48S30VU" class="theme-secondary pa-3">

Kindle

</webui-link>

</webui-paper>

<webui-paper class="pa-2" elevation="10">

<webui-link target="_blank" href="https://amzn.to/3Sgpyu3" class="theme-tertiary pa-3">

Paperback

</webui-link>

</webui-paper>

<webui-paper class="pa-2" elevation="10">

<webui-link target="_blank" href="https://amzn.to/3vue8db" class="theme-primary pa-3">

Hardcover

</webui-link>

</webui-paper>

</webui-flex>

*Paid Links*

</webui-flex>

</webui-side-by-side>
`);

        }
        disconnectedCallback() { }
    }
    customElements.define('app-books', Books);
}