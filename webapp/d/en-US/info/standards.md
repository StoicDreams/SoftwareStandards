# Team and Developer Standards

```sideimage "right" "https://cdn.myfi.ws/v/Vecteezy/business-concept-of-vector-illustration-little-people-links.svg"
As developers it is important to set out standards for development. The desired outcome for defining standards is to make our products better and our development easier.

To achieve these goals, we must use standards that make sense and are objectively helpful to reaching these goals.

For this reason, do not succumb to just using any standards because "that's what everyone else is doing".

And when asking why an existing standard exists, if the reason is "that's how we've always done it", then it's probably time to either drop it or review it, because that reasoning is equivalent to "I don't know". And if you don't know why you are doing something, than it may not be a good idea to be doing it.
```

```quote "info"
Even when you have defined standards, it's important to remember that standards should never be absolute. When you define a standard, all you should be saying is "this is the best standard we can think of for the situations we could think of". You may find a better standard in the future to replace it. Or you may find edge cases where your defined standard is not the best solution, but your defined standard is still best for your other scenarios you've used it for.
```

```quote "warning"
Be agile with your standards. Improve and update them as often as you can.
```

## Team Standards

```sideimage "left" "https://cdn.myfi.ws/v/Vecteezy/businessmen-discuss-social-network-news-social-networks4.svg"
When defining team standards, it is very important to evaluate the reasoning for a standard. Why is this standard better than another?

Team standards should be standards that objectively improve development, or the end products being produced.

If the reasons why for any standard are only subjective in nature, then exclude it from your team standards.
```

`````cards
````card "Examples of Good Team Standards" "580" "success"

- Only use minimized Javascript files in production because our testing has shown it improves load times by 30-80%.
- Use SQL procedures instead of raw queries because our testing has shown it improves performance by at least 10-30%.
- Place website files in a folder called wwwroot because that's what this framework we use requires.

````
````card "Examples of Bad Team Standards" "580" "danger"

- Use 2 spaces for indentation, because it's easier to read.
- Use Pascal Casing for all global variables, because that's the industry standard.
- Break any lines of code that exceed 80 characters so developers with smaller screens don't have code running off the screen.
- Put all components of your project in a root folder named `src` because that's how we do it for all of our projects.
````
`````

## Personal Developer Standards

```sideimage "right" "https://cdn.myfi.ws/v/Vecteezy/content-writer-blogger-bullet-journalist-or-publishing2.svg"
Developers should be given the freedom to define their own subjective standards. Standards that they find make development easier for them, even if other developers on the team disagree.

Personal standards should only include standards that are subjective. Standards with objective reasoning should be added or discussed for Team Standards.
```

````cards
```card "Examples of Good Developer Standards" "580" "success"
I use 3 spaces for indentation, because I find it easier to read.
I use Pascal Casing for all variables defined outside of a method because I find that easy to read and I can develop faster not having to remember complex naming conventions.
```
```card "Examples of Bad Developer Standards" "580" "danger"
I write SQL scripts as queries instead of procedures because that's easier and faster to develop.
I don't minimize my JavaScript files for production because I find it hard to debug production issues when I do that.
```
````

## Ownership of Standards

`````cards
````card "What is ownership?" "580" "info"
An **owner** of a codebase or project within this context of standards is a developer that will be in charge of dictating the standards for that codebase/project.

Typically this is more for standards that come from personal preferences, not standards that would be in place for performance or functional reasons. Though the owner should have authority and trust to deviate from performance|functional standards when they deem it necessary, but they should document in the code where and why they are deviated from these standards so future developers can easily understand why.
````
````card "Single Developer per Project | Codebase" "580" "info"
When you have organized your teams so that you end up with projects that only have a single developer working in them, then that developer should be considered the owner of that codebase.

When another developer takes over a codebase/project, they should have the freedom to refactor the code as the deem appropriate to conform to their personal standards so they can more easily manage and update the project.
````
````card "Multi-Developer Codebases | Projects" "580" "info"
When you have multiple developers working in the same codebase at the same time, then allow each developer ownership over whatever files they are needing to change for the work they are doing.

Developers should be given the freedom to change a file they are working in so it is easier to read and work in for them.

Most modern IDE's developers are using will allow them to quickly reformat a file to match the standards they've defined in their IDE, taking just a few seconds after opening the file.

This maximizes the readability for that developer so they can maximize their efficiency in doing their job.
````
````card "Org | Manager | Team Ownership" "580" "info"
In many cases these arbitrary, preference-based coding standards are dictated by a higher authority than the developer doing the work. These may come down from the organization/company level, or they may be dictated by some level of management above the team.

In some cases these coding standards are allowed to be decided by the team as a group, generally doing some form of voting to dictate the standards, that then must be applied to all projects worked on by that team.

```quote "danger"
This is the most damaging form of ownership, and we strongly recommended to break away from doing this.
```
The main arguments for doing this is code consistency and readability.
First, readability is not a valid reason, as it is a subjective reason. Most of the time this readability improvement is at best applicable to a lead or manager who needs to do code review, but that is bad justification to make code more readable for a single person that is only reading the code 1% of the time, and less readable for the developers that are reading and writing the code 99% of the time. Or worse the readability is dictated from people higher up the chain that may never look at the code at all.

As far as the code consistency argument, this is an arbitrary benefit. If your code has consistent standards throughout, that is literally the only benefit you can say comes from that. And that benefit has no tangible value.

Code consistency is not a requirement for making code that is easy to read and understand. It does not help with performance or keeping code bug free. It does not help with organization.
```quote "warning"
Code consistency standards really only serve as a form of micromanagement from wannabe-dictators to give them a sense of control and power over people. But it has the adverse affect of degrading developer moral and motivation, degrading product quality, and stifling creativity and innovation.
```
````
`````
