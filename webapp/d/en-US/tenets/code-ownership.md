# Code Ownership: Tenet 2 of Continuous Agile Software Engineering

````sidebyside
## Introduction

```paper
Code Ownership is about creating a sense of responsibility and collaboration among team members when it comes to the codebase. This not only leads to higher quality code but also fosters a culture of teamwork, knowledge sharing, and continuous improvement. In this chapter, we will explore the concept of Code Ownership and delve into its three core principles: Assign Ownership of Code, Foster Collaboration Among Owners, and Rotate Ownership Over Time.

And like the first tenet, Code Ownership is the 2nd tenet for a specific reason. This tenet directly addresses one of the biggest issues in software development, over saturation of developers in a project.

Bad management, workflows, and practices produce a growing list of issues that result in missed deadlines and deliveries. Managers misdiagnose the problem, rationalizing the project has just grown too big and they need to throw more resources in to account for the size, and they fail to see that every time they do this productivity slows down even further.

From my experience in all the teams that I have worked on in my career, every project that had five to ten developers working on it, should never had more than three, and often should have had only one. Even the projects that I think three would have been the optimal number of developers, I’m confident a single developer would have been more productive on those projects than having the five to ten that they had.

Let us go over now the three core principles of Code Ownership that are Assign Ownership of Code, Foster Collaboration Among Owners, and Rotate Ownership Over Time.
```
````

````sidebyside
## Core Principle 1: Assign Ownership of Code

```paper
Assigning ownership of code involves designating a single specific software engineer as the "owner" over a specific module of code, where a module is typically a single repository that may be for a library, a domain, or even an entire solution of multiple projects. The owner of a code module is also the primary developer working in that module, and in most cases should be the only developer working in that module.

As the owner of one or more modules, the developer has full rights to dictate their module’s formatting, organization, and adoption of their best-practices. This allows the developer to utilize the best practices that they are the most comfortable with and most productive using and helps give the developer more confidence in their work, a better feeling of ownership and responsibility, and overall, more satisfaction from their contributions.

Code ownership also helps to establish accountability by giving individuals responsibility for delivering features, performance, and bugs within their modules. With this responsibility developers are more motivated to take ownership of its quality and maintenance. This leads to cleaner, more reliable code and fewer issues down the line.

This also helps to encourage expertise where team members can develop deeper expertise in specific areas of the codebase, leading to more informed decision-making and a better understanding of potential risks and dependencies.

To effectively assign code ownership, consider following micro-service architecture to break up larger projects into smaller services. It is often beneficial as well to break up larger projects into libraries. You might for example break up a website project into 4 services and thus repositories, each with their own ownership role. The first being the front end, then the website API, then the database middleware, and finally a core library that contains shared functionality and structures used by the other 3 projects.

If a module grows to be too big for a single developer, prefer breaking it up into smaller modules over adding additional developers. This helps to follow the first tenet of keeping things simple, as well as helps to maximize the efficiency of developers by avoiding the common and often inevitable issues that come from multiple developers working in a single codebase.

When assigning ownership, take into consideration team members' strengths, interests, and areas of expertise, ensuring that they are well-suited to the code they will own, either by experience, interest, or both.

Finally, assure every developer has at least 1 module that they own. Developers should only work on code owned by other developers under rare and extreme circumstances. It is also okay and common that a single developer will own multiple modules, especially as modules mature and require less maintenance.
```
````

````sidebyside
## Core Principle 2: Foster Collaboration Among Owners

```paper
While assigning ownership helps to establish responsibility and expertise, it is also essential to encourage collaboration among code owners. This ensures that knowledge is shared among the team, and decisions that affect multiple services and owners are made collectively.

Create a culture of open communication among team members by encouraging cross-team communication. Ensure that everyone feels comfortable sharing ideas, asking questions, and providing input on other people’s code. This is often best when executed with a mindset of offering ideas and suggestions to consider, not telling others how they need to do something. For example, suggesting a different language feature to solve a problem because you find it is more readable, or it could help improve performance.

But of course, at the same time everyone needs to understand and be comfortable knowing that there are times when things need to be done a certain way for a specific reason. When communicating the need for someone to correct their code or project, always communicate why the correction is needed, and how they can correct it. Never leave them guessing at either and remember that “because this is how we do it” or “this is industry standard” is not a valid reason for why something should be done.

Implement a process for regular team code reviews, recommending sessions lasting 2 to 3 hours, 1 to 2 times per month, where code owners can do walkthroughs of architecture, discuss changes, provide insights and feedback, and identify potential issues or areas for improvement. 

These code reviews are not the same as you may have done from other workflows such as Scrum, Agile, or Waterfall, where the primary purpose is to find issues in the code and only merge code that passes a manual approval process. The primary purpose for code reviews in CASE is to share concepts being used, solutions used to solve specific problems, and provide some familiarity to others in case they need to take temporary ownership while someone is on vacation.

Make use of collaboration tools, such as version control systems, project management tools, and communication platforms, to facilitate open knowledge sharing and cooperation among team members.
```
````

````sidebyside
## Core Principle 3: Rotate Ownership Over Time

```paper
To further promote collaboration and knowledge sharing, it is important to rotate code ownership over time.

This can help to prevent knowledge silos by giving team members exposure to different projects and services. Over time this will also help ensure that the team is well-rounded and adaptable.

Rotating ownership also helps promote continuous learning for team members by introducing new opportunities to gain experience learning new skills, technologies, and approaches, fostering a culture of continuous learning and improvement.

As the codebase evolves, maintenance responsibilities can become a burden for a single owner. Rotating ownership ensures that these responsibilities are shared among the team.

You could implement a rotation system that establishes a rotation schedule where you create a schedule that outlines when and how ownership will be transferred between team members and done at regular intervals or based on project milestones. But in my experience, this seems to have worked better when it was done in a more organic and agile way, where some services might end up being rotated every 2 or 3 months, while other services are closer to 2 to 3 years. The key is that ownership should be rotated based on the needs to rotate, versus following some arbitrary schedule.

During transitions, ensure that team members have access to the necessary resources and support, such as documentation, training, or mentorship from the previous owner.

Also, when onboarding new developers to a seasoned team with mature modules, it can be good practice to transfer ownership of 1 or 2 modules to the new team member, giving them the freedom to get their feet wet by focusing on maintenance tasks such as performance improvements.
```
````

````sidebyside
## Conclusion

```paper
By embracing the tenet of Code Ownership and implementing its core principles, software development teams can foster a sense of responsibility, collaboration, and continuous learning. This can lead to higher quality code, increased efficiency and productivity, better job satisfaction for software engineers, and a more cohesive team that is better equipped to adapt to new challenges and technologies.
```
````
