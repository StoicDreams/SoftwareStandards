import { test, expect, openPage } from './support/site.mjs';

test('app-books links to each edition of the book, and says the links are affiliate links', async ({ page }) => {
    await openPage(page, '/');
    const books = page.locator('app-books');
    await expect(books).toHaveCount(2);
    const first = books.first();
    const editions = { Kindle: 'https://a.co/d/0reKAjn', Paperback: 'https://a.co/d/9Kiyse8', Hardcover: 'https://a.co/d/3da8fQc' };
    for (const [label, href] of Object.entries(editions)) {
        const link = first.locator('webui-link', { hasText: label });
        await expect(link).toHaveAttribute('href', href);
        await expect(link).toHaveAttribute('target', '_blank');
    }
    await expect(first).toContainText('affiliate');
});
