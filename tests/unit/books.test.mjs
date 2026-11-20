import { test, expect, openPage } from './support/site.mjs';

test('app-books links to each edition of the book on Amazon', async ({ page }) => {
    await openPage(page, '/');
    const books = page.locator('app-books');
    await expect(books).toHaveCount(2);
    const first = books.first();
    const editions = { Kindle: 'https://www.amazon.com/dp/B0C4FX1F1S', Paperback: 'https://www.amazon.com/dp/B0C47LV1Y8', Hardcover: 'https://www.amazon.com/dp/B0C47YGGYH' };
    for (const [label, href] of Object.entries(editions)) {
        const link = first.locator('webui-link', { hasText: label });
        await expect(link).toHaveAttribute('href', href);
        await expect(link).toHaveAttribute('target', '_blank');
    }
    await expect(first).toContainText('CASE: Continuous Agile Software Engineering');
});
