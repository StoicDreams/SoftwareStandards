<webui-data data-page-title="Code Reviews" data-page-subtitle=""></webui-data>

> [Erik Gassler] Published August 22, 2025

## Code Reviews Don't Belong in Your CI/CD Pipeline

<webui-page-segment elevation="10">
One of the biggest mistakes I see in teams practicing CI/CD is the idea that code reviews should be a gate in the pipeline. It seems logical: before code merges, someone should approve it. The problem is, that mindset completely undermines what CI/CD is supposed to achieve.

If humans are sitting in the middle of your automation, you don't have a continuous pipeline anymore. You have a stoplight.
</webui-page-segment>

### CI/CD Requires Flow

<webui-page-segment elevation="10">
The whole point of CI/CD is that every commit can move from source to production without waiting on anything other than automation. Every commit is integrated, tested, and deployed continuously. No bottlenecks, no approvals, no handoffs.

Insert a code review into that path, and you've broken it. Suddenly, "continuous" becomes "continuous… until Jim gets back from lunch and approves the PR." Your system is only as fast as your slowest reviewer. That's not continuous integration or delivery - it's a manual release process dressed up with some automation.
</webui-page-segment>

## Reviews as Events, Not Steps

<webui-page-segment elevation="10">
Now, to be clear: I'm not saying code reviews don't matter. In fact, they're vital. But they work best as deliberate events with the team, not as a mechanical step in a pipeline.

When reviews are bolted onto the merge process, they become about approval. The reviewer's job is to unblock. That's not real collaboration. That's bureaucracy.

In my experience, reviews are most effective when held in **bi-weekly or monthly review sessions**. This isn't a meeting where you nitpick line by line. Instead, it's a space for the team to step back, look at recent changes, and talk through them together. What patterns are emerging? What improvements can we make? What trade-offs did we take, and what can we learn from them?

The value of review isn't in catching typos - it's in building shared understanding. That works far better in group sessions where discussion flows, not in isolated comment threads attached to a PR.
</webui-page-segment>

### Trust and Safety in CI/CD

<webui-page-segment elevation="10">
Of course, the question always comes up: "But what if bad code gets merged?" The answer is simple: CI/CD only works if you trust your safety nets. Automated tests, observability, feature flags, and fast fixes are what keep bad code from hurting users.

If your process relies on a human reviewer as the last line of defense, you don't have CI/CD. You have manual QA with extra steps. Continuous delivery demands that we design for failure and recovery, not for preventing all mistakes up front.

That's why separating code reviews from the pipeline actually strengthens your process. Reviews become about **improving engineers**. Pipelines stay about **moving code**.
</webui-page-segment>

## Reviews as Part of CASE

<webui-page-segment elevation="10">
In Continuous Agile Software Engineering (CASE), this separation is critical. Teams deliver continuously, with automation as the backbone. Reviews then become recurring team events, used to strengthen code quality and team alignment without blocking flow.

By holding bi-weekly or monthly review sessions, you create a cadence that supports reflection without slowing delivery. Developers don't wait on approvals to ship. Instead, they ship continuously, then come together as a group to learn and improve based on real code that's already in production.

This rhythm balances the two forces that make CASE work: **flow** and **feedback**.
</webui-page-segment>

### Conclusion

<webui-page-segment elevation="10">
If your pipeline pauses for review, it's not continuous. You're gating automation with bureaucracy.

But when reviews are elevated to deliberate, recurring team events, they actually do what they're supposed to: raise the bar, spread knowledge, and grow the team.

CI/CD is about trust in automation. Code reviews are about trust in each other. Mixing the two only weakens both.

Keep your pipeline continuous. Keep your reviews collaborative. That's how you get real flow - and real quality.
</webui-page-segment>
