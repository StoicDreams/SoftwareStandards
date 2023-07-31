# Test Strategies for Continuous Development

```quote "info"
Good testing standards are an essential part of software development to produce high-quality and maintainable products.

Using quality testing standards with test-driven development across all testing types will help assure you can continuously produce updates and add features promptly while greatly reducing the chance of your projects falling into the technical debt trap where the larger your projects become the harder it is and longer it takes to fix issues, make changes, or add new features.
```

## Test-Driven Development

```sideimage "right" "https://cdn.myfi.ws/v/Vecteezy/the-scientist-conducts-laboratory-studies-and-studies-the2.svg"
Commit to updating your mindset and take the time to learn good test-driven development standards.

Test-driven development will likely require a fundamental shift in how you currently write, organize, and develop your code. This shift in thinking and organization is generally the hardest part for most developers and will take some extra time in the short term to get adjusted to. But the long-term results will be that your code is much easier to manage, debug, and update, which will greatly improve your development speed while also improving the quality of your code when compared to non-TDD practices.
```

## Automated Testing

```sideimage "left" "https://cdn.myfi.ws/v/Vecteezy/industrial-cybersecurity-illustration-exclusive-design.svg"
Automating your testing needs to be a top priority early on in the development of your project. The sooner this is done the more time you will save in the long run.

For local development, look for and utilize tools that will automate your testing as you develop, so you can see in real-time when changes you are making are breaking your tests, and you can immediately address this breakage by fixing your changed code or updating the tests to apply new assertions.

Automate unit and internal integration tests when pushing commits to help assure commits aren't pushed up that contain breaking changes.

Include code-coverage detection to assure all projects are meeting minimum requirements in test-coverage. Unit tests should be expected to have at least 90% code coverage for most types of projects, while internal-integration tests should be able to cover at least 95%.

Build pipelines should also include automated testing of the unit and internal integration tests, failing the pipeline if any tests fail or code-coverage requirements are not met.

Release pipelines should also include automated testing of all other types of testing.
```

## Testing in CI/CD Pipeline Workflows

````sideimage "right" "https://cdn.myfi.ws/v/Vecteezy/project-tracking-task-completion-or-checklist-to-remind.svg"
Depending on various factors including project/language types, and personal/team preferences you may choose to run your Build and Release pipelines in a single workflow (i.e. a single YAML file), or you may choose to have them split into their workflows, where successful completion of a build pipeline can then trigger the associated release pipeline.

Generally, we recommend a single build/release workflow for very small projects where all of the build/testing/deployment tasks are expected to never take much time (e.g. no more than 10-15 minutes.) And for larger projects separate the build and release pipelines.

Deployments should occur first in a User-Acceptance-Testing (UAT) environment where post-release tests can be run to verify before deploying to production. If you also require manual tests to be done as part of deployment you should do these in the UAT environment.

Ideally, your Prod and UAT environments will be set up to be swappable, so that once you have verified UAT is working as expected you can simply point production users to UAT, making it the new Prod. This way if any issues are discovered that demand reverting you can simply redirect users back to the old Prod.

Build Pipeline
```list
Restore dependencies
Build projects
Run unit/internal-integration tests
Publish test results
Save files to artifact
```
Release Pipeline
```list
Load artifact
Apply system updates to UAT
Run system tests to assure system is setup/configured correctly
Cleanup old/deprecated files
Deploy new files
Run external-integration tests
Run end-to-end tests
Run performance tests
Run load tests
```
````

## Build Tests / Unit Tests

```sideimage "left" "https://cdn.myfi.ws/v/Vecteezy/people-connect-the-elements-of-the-pyramid-symbol-of.svg"
Unit tests are the bare minimum that every developer should be producing when writing new code. This not only validates your code as your developing but also proves your code is doing what it's intended to be doing to others.

A Unit in unit testing is any grouping of code that you want to test. This could be as small as a function, a single file or class, or span across several files or classes.

The main key though is that build/unit tests should never depend on any dependencies external to the app, not even environment variables or always-expected OS libraries. Any dependency external to the codebase should be mocked.

Additionally, within the codebase, any internal dependencies external to what you are directly testing should also be mocked.
```

## System Tests

````sideimage "right" "https://cdn.myfi.ws/v/Vecteezy/online-big-data-courses-illustration-exclusive-design.svg"
System tests are tests that validate an environment meets the requirements needed for an application to be deployed and run correctly.

These tests can include such verifications as:
```list
Assure expected environment variables exist and are valid
Assure external APIs are accessible.
Assure any required system resources/sdks/etc are installed and accessible
```
````

## Service-Integration Tests

```sideimage "left" "https://cdn.myfi.ws/v/Vecteezy/information-architecture-illustration-exclusive-design.svg"
Service-integration tests are tests that validate that units and workflows run as expected between deployed services. These tests generally will not mock any resources, internal or external, having full usage of any associated databases, APIs, etc.

These tests need to be safe to run in a production environment, using accounts and data that are set up explicitly for these tests.

Developers should be able to run these tests in a local environment, mixing services that are deployed to their local machine and services hosted in a dev/uat/prod environment as needed.
```

## End-to-End Tests

```sideimage "right" "https://cdn.myfi.ws/v/Vecteezy/site-design-illustration-exclusive-design-inspiration.svg"
End-to-end testing refers to tests that you would create to cover anticipated user behavior.

Note that you want to cover both behavior that you want your users to take, as well as behavior that might be from malicious actors who are wanting to break or hack your application.
```

## Peformance Tests

```sideimage "left" "https://cdn.myfi.ws/v/Vecteezy/tiny-people-and-huge-sand-glass-flat-vector-illustration.svg"
Performance testing should be done after all previous testing has verified your app works as expected. These tests will validate that your application and its various workflows, endpoints, and tasks meet minimal acceptance criteria.

This testing should include publishing or logging metrics where they can be easily consumed and viewed by team members to gauge performance bottlenecks and prioritize where further optimizations are needed.

Minimum performance metrics should be defined and if tests prove a failure to meet expectations then deployment to Prod should also fail.

It is also good practice to have warning metrics for preferred performance that exceeds the minimum requirements and that will not trigger a failure to deploy but will trigger notifications and/or action items to address performance issues.
```

## Load Tests

```sideimage "right" "https://cdn.myfi.ws/v/Vecteezy/benchmarking-idea-of-business-development-and-improvement.svg"
Load testing should be the last of the automated tests you run after deployment. After you have assured your app works as expected and it meets performance requirements under minimal load and sets the baseline for your performance expectations in the current version.

In load testing, you want to verify if and how much performance degrades when placed under various loads.

Try to think of various scenarios to test against, from gradual increases to sudden spikes.

Generally, this type of testing might just be a matter of running a specified N number of end-to-end tests in parallel alongside specified performance tests, making it easy to compare the results of the performance tests under the specified load in comparison to the baseline.

While performance tests should assert a deployment failure when minimum performance metrics are not met, load tests may not need to require a deployment failure. Instead, load testing should be a way of measuring current load capacity to compare with load expectations and help drive business decisions to prioritize work to expand load capacity and assure your application will stay stable and usable up to and beyond the expected load. As well as assure that when the load is exceeded any expected alarms or safety actions are triggered so anyone who needs to be notified is properly notified and the app can be returned to a stable state as quickly as possible.
```
