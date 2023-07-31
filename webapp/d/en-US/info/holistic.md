# Holistic practices for Agile Development

```sideimage "left" "https://cdn.myfi.ws/v/Vecteezy/people-are-building-a-spaceship-rocket-cohesive-teamwork-in.svg"
This is a concept we want to explicitly point out because this is a major concept that we see commonly overlooked when a developer, team, or management is trying to integrate some framework or methodologies into their workflows or projects.

Too often, a framework like Agile or Scrum is proclaimed "the silver bullet" that will somehow miraculously improve the workflow or project, but instead of implementing the entire framework as originally intended, companies or teams only implement parts of it. This often leads to a lack of success, as the parts taken out are essential for the entire framework to work properly and should be considered to benefit from it.

To ensure that Agile or Scrum is successfully implemented in an organization, all components must be understood and effectively applied — not just cherry-picked pieces. A successful implementation requires that the framework is understood in its entirety and not just as bits and pieces.

When using Agile or Scrum, it's important to remember that the framework is meant to be flexible and iterative, so teams must not become too attached to specific processes or techniques. Instead, they should be open to changing certain elements to meet their specific needs. The adaptability and flexibility of the framework are what make it so effective, and that's why teams must be careful not to take too much away from it.

Finally, it’s important to remember that Agile or Scrum isn’t a one-time implementation — it requires ongoing maintenance and supervision. It’s essential to review the framework regularly and make sure all components are being utilized correctly. That way, teams can continue to benefit from Agile or Scrum in the long run.  By understanding the principles of Agile or Scrum and implementing them correctly, organizations can achieve successful results for their projects.
```

```sideimage "right" "https://cdn.myfi.ws/v/Vecteezy/team-metaphor-people-connecting-puzzle-elements-vector.svg"
With Continuous Agile Software Engineering, we wanted to be much more direct about our principles, in part to hopefully address many of the problems and failures we see from teams trying to implement Agile or Scrum practices into their software development. All of the separate components and concepts of CASE, when put together, help developers and teams to create high-quality software, in the shortest development cycles.

But a major reason CASE works so effectively is that its various tenets and principles work and fit together to make up the larger whole. It is how the smaller pieces work together as a larger whole that makes CASE such an effective way for developing software, not any individual piece by itself.

However, we've observed time and again that developers, managers, and executives often don't understand this concept. They view all of a frameworks components separately and conclude they can just choose whichever ones they prefer while discarding any that conflict with their current ideologies. Worse yet, when their implementation of said framework fails, they conclude that the framework was just bad, because their ego won't let them see that it was their decisions to alter or not use certain parts of the framework that caused it to fail.
```

## Long-term thinking

```sideimage "left" "https://cdn.myfi.ws/v/Vecteezy/tangle-tangled-and-unraveled-abstract-metaphor-business2.svg"
More often than not, we see opposition to change originating from a short-term perspective. Someone is unwilling to adopt something that would require more time and effort in the beginning, with no realization that it would save masses of time in the long term.

Test-Driven development is a great example of a practice that may seem like it would only add time to your project but can save you significant time and effort in the long run. With Test-Driven development, you are constantly testing your code for bugs which means that you catch them immediately instead of later down the line when they could cause more damage and consume a lot more time and resources.

This practice also leads to improved adoption of writing testable and maintainable code, which saves time in future updates. This is because the code is easier to follow and developers can make changes with confidence that they are not introducing new bugs.
```

### A real life horror story

```quote "tertiary" "Erik Gassler"
I once had a conversation with my skip manager while I was at Microsoft about my frustration with their static, waterfall-centric standards and workflows that they had mixed in with what they called Agile/Scrum development, instead of the truly agile practices and continuous delivery workflows I was accustomed to using. The major point of frustration for me was that it was taking a team of 8-plus developers weeks and months to develop features that my last role prior to Microsoft had proven that a single developer could accomplish in spans of hours and days.

I gave him context about how at my previous company I had architected and developed an application by myself that at the start was very similar in concept and scope to the project my then-current team was working on, which consisted of roughly 8 to 10 software engineers at that time. This project had been in development by this team for over 2 years, with its target customers being internal users within Microsoft. And during that time it had never been released to its target customers and had been completely rebooted twice because it was failing to meet expectations in both performance and features, and they kept determining that they needed to rethink the architecture of the concept.

A limited number of beta-users were currently using the second iteration which was severely broken, with several features not working as expected, while the team was frantically trying to build out the third iteration.

By comparison at my previous company, using my CASE practices and working by myself, I had my primary customers using the product within the first week of development, delivering a similarly scoped user experince. Within the first two months I had delivered at least as much technical features as the two year old MS project had developed. For context, the Microsoft product was affectively a 3-page website application - one page for defining report filters into groups, another page for applying filters and running a report, and the third page to view previously run reports. Versus my CASE project which launched with a few pages, each offering a separate report with filtering options, and after two months of development had grown to a few dozen pages, the majority of which were separate reporting pages with their own unique filtering options.

Over the next few months after the first two months on the CASE project, the scope of the project had grown dramatically and partners outside of the company had also been onboarded with access, and I had personally scaled up several features that included dozens if not hundreds of separate reporting pages with filterable data; Robust error and request time tracking and reporting for all of the company projects; As well as several other custom one-off features and services that had been desired to be added by both admin, other developers, and myself as time was going on. **This website had transformed from its initial concept as an internal reporting tool, to a company portal that managed many other features beyond simple reporting**.

On the rare occassion new bugs were introduced to the system (most often these were performance issues - i.e. a page taking more than 300 milliseconds to load), they were automatically detected and prioritized to be fixed quickly, most of the time without any user complaining about them.

When users did spot errors that weren't being automatically detected, they had available and were encouraged to use a simple integrated feedback system that allowed them to report the error so I would be notified and could prioritize resolving the issue.

When a new report page was needed to display some new type of metric, it was often usable within hours after being requested, including fully featured filter options, role-based limitations, and pagination.

This Microsoft manager's response to me explaining this context was to tell me that Agile doesn't work for large corporations. He told me how they had tried it once before, and it was a disaster because developers were just doing their own thing and never getting any work done.

He also reasoned that they can't do Agile development because without pre-planning features like security and accessibility, they couldn't be implemented properly.

This response blew me away. I was dumbfounded and speechless by such naive and stupid rationalization. He made no effort and had no interest to ask questions or probe for more details, to see if any part of what I was doing before could be incorporated to improve their standards and workflows. Instead, he seemed offended by the thought that I could have possibly been doing anything better in any way than what they were doing there.

First off, his statements and reasoning told me that he didn't have any understanding of what Agile development truly was. Agile development does not requuire that you have to not give developers any direction and letting them just do whatever they want. Though I have seen that concept work for some teams, it is not what Agile development is.

It also doesn't mean you can't have key concepts like security and accessibility in place at the start of development. In reality, it makes these easier and quicker to implement and adjust as needed, it's just a matter of prioritization.

Second, it also told me that whatever his interpretation of Agile development they tried, they missed a key part of Agile development. And that is to constantly analyze your processes, standards, and workflows, validate what's working and what's failing, and make adjustments to fix what's failing. Instead, based on his words, they just concluded that Agile development can't possibly work and abandoned the concept completely, going back to more Waterfall-centric practices and workflows.
```

## Agile development is not constrained by company size

```quote "info"
The only constraint to agile development is in the mindsets of those trying to adopt it.
```

```sideimage "left" "https://cdn.myfi.ws/v/Vecteezy/executive-manager-planning-and-monitoring-presentation.svg"
Agile development works, as many companies, including large enterprise companies, can attest to.

And the fact that these companies continue to use Agile practices attests to the fact that for them it worked better than whatever waterfall or other statically structured practices they were doing before.

But, in order for it to work correctly, the concept needs to be understood and used correctly. In the case of Agile development, you need to understand that at its root Agile is a mindset, not any specific collection of frameworks, workflows, or methodologies.

Scrum is a framework that some Agile developers use because they find that they benefit from that method of structuring and organizing tasks. This does not mean that if you use Scrum structures or practices that you are doing Agile development. Scrum itself is not Agile. It can be used in Agile workflows as much as it can also be used in Waterfall workflows.

The concepts of Sprints is another tool that some Agile developers use, again because they find that they benefit from that method of structuring tasks and prioritizing their work. But again, just because you develop in Sprints, does not mean you are doing Agile development.

In fact, more often than not from what we've seen in our experiences, companies are using Scrum meetings in name only when they're really just having daily status meetings, and they're doing work in Sprints, but they are spending great effort to estimate work and time to completion, planning out Sprints for several Sprints and months ahead, which means they are still developing using Waterfall centric practices. They do not have that Agile mindset.

Another major issue we see is that tools, practices, and workflows are being dictated by high level managers and administration, instead of allowing the developers to dictate the tools and workflows that work best for them and their current projects and tasks. Most companies seem to miss the point that Agile development needs to be developer lead and developers need to be allowed to actually be agile with their tasks, practices, and workflows in order to be successful.
```

```quote "info"
Managers should be feeding developers problems to solve while providing the developers the trust to use their expertise to solve those problems.
```
