# What is Agile Development?

```sideimage "left" "https://cdn.myfi.ws/v/Vecteezy/intellectual-capital-economic-growth-and-development-stock.svg"
Agile is a term that gets thrown around and misused a lot, without much consistency in practice to what it means.

If you take a look at the actual [Agile Alliance documentation](https://www.agilealliance.org/agile-essentials), you will find that the Agile framework is a mental structure on how to solve problems faced when engineering software.

Like any standard, there is room for interpretation and adaptation. And while concepts like Scrum and Sprints may have sprouted by Agile teams, this does not mean that because you have Scrum meetings or develop in Sprints you are doing Agile development.

For us, the Agile framework is nothing more than a mindset. It is being open to constantly evaluate and update how we develop software to assure we are developing as efficiently as possible while adhering to high-quality development standards.

But we do believe the Agile Manifesto has room for improvement, and for this reason the agile part of Continuous Agile Software Engineering, as well as when we say agile in other contexts without the capital A, is not referring to Agile from the Agile Manifesto, but instead referring to the word agile.

We believe in progressing development through small, iterative updates, with various feedback loops from users and insights to help guide future progression.

We believe in adapting to various projects we work on, adapting to various size teams, and even adapting to the various types of people we work with.

We believe that truly agile workflows do not require nor demand detailed pre-planning or documenting of any specific technical details, nor does it involve any feature-specific deadlines or estimating. Instead, planning and documentation are worked on continuously throughout development, evolving as the project and requirements evolve. And instead of delivering features we deliver iterations, with a focus only on the highest priorities.

The only pre-planning and documentation we want before development is the Statement of Work, or SOW, defining the who, what, where, when, why, and how's of the project.

These are high-level questions and answers that any user, developer, or executive should be expected to be able to understand when participating in any work in the project.

In most cases, this SOW documentation should be created and managed by a project owner or manager, and needs to focus on business goals and user experiences. There absolutely should be no specifics within a SOW document that specifies any technologies, workflows, or any other technical details about the project.

Technical details should be documented during development as part of the an agile workflow loop.
```

## Agile Workflow Loop

```sideimage "right" "https://cdn.myfi.ws/v/Vecteezy/business-teamwork-with-pieces-of-puzzle-in-office.svg"
Agile workflows should loop through specific stages of development, often several times in an average workday. A single feature may require several, even hundreds of loop cycles to implement, depending on the complexity of the feature.

That loop is Research-Prototype-Evaluate-Integrate-Test-Deploy-Validate.
```

````cards
```card "Research" "580"
The first stage for any development loop needs to be research. Sometimes you don't need much, other times you need a lot. This can be anything from researching the code you're planning to update, to as much as researching new technologies or APIs that you might want to integrate into your project to support the feature.

This could also involve asking peers if they have any suggestions or ideas on how to implement a feature, or relevant technologies that might be available.
```
```card "Prototype" "580"
From your research, you likely came up with multiple solutions. Before implementing, prototype each solution or idea so you can evaluate the pros and cons of each.

Prototypes should include using test-driven development practices so each prototype can showcase proofs that it works as expected, as well as make it easier to pinpoint performance bottlenecks for the evaluation stage.
```quote "info"
Note: Each prototype at this point might also include its own Agile Workflow Loop.
```

```card "Evaluate" "580"
You've created your prototypes, now it's time to evaluate the pros and cons of each.

It is likely important to consider final performance, but you may also want to consider other factors such as code complexity, use of third-party dependencies, and ease of maintenance.

Even though we generally prefer whatever is best performing, there have been a lot of times when we have chosen a solution simply because it was a much simpler and easier to maintain solution, rather than a solution that was slightly faster but much more complex.
```
```card "Integrate" "580"
Once you analyzed the results from your prototype evaluation and chosen a final solution, this step is where you'll integrate that solution into your code.
```
```card "Test" "580"
When you have completed integration and feel it is ready for deployment, run the full suite of tests for your project to assure everything is working as expected.

Any tests that need to be run manually, take the time to try to setup a way to automate them. Ideally you should just have a button somewhere that you can press and have all local tests run automatically from there, and end with some kind of report that details all the tests run and highlights any failures.
```
```card "Deploy" "580"
When your work is completed and all testing is good, this is the step where you commit and push your work to the Master | Main branch.

This should kick off build and release pipelines, each with their own steps to process any respective tests.

If any build or release pipelines need any updates to account for new tests or configurations, make sure you update them accordingly.
```
```card "Validate" "580"
Once deployed, take some time to validate everything tested and deployed correctly. Run some extra sanity checks, watch logs and insights to make sure nothing spikes.

Be ready to roll back your changes if necessary.
```
````
